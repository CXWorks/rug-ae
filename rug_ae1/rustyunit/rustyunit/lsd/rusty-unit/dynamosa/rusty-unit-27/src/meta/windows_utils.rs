use std::ffi::{OsStr, OsString};
use std::io;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::shared::winerror;
use winapi::um::accctrl::TRUSTEE_W;
use winapi::um::winnt;

use super::{Owner, Permissions};

const BUF_SIZE: u32 = 256;

pub fn get_file_data(path: &Path) -> Result<(Owner, Permissions), io::Error> {
    // Overall design:
    // This function allocates some data with GetNamedSecurityInfoW,
    // manipulates it only through WinAPI calls (treating the pointers as
    // opaque) and then frees it at the end with LocalFree.
    //
    // For memory safety, the critical things are:
    // - No pointer is valid before the return value of GetNamedSecurityInfoW
    //   is checked
    // - LocalFree must be called before returning
    // - No pointer is valid after the call to LocalFree

    let windows_path = buf_from_os(path.as_os_str());

    // These pointers will be populated by GetNamedSecurityInfoW
    // sd_ptr points at a new buffer that must be freed
    // The others point at (opaque) things inside that buffer
    let mut owner_sid_ptr = null_mut();
    let mut group_sid_ptr = null_mut();
    let mut dacl_ptr = null_mut();
    let mut sd_ptr = null_mut();

    // Assumptions:
    // - windows_path is a null-terminated WTF-16-encoded string
    // - The return value is checked against ERROR_SUCCESS before pointers are used
    // - All pointers are opaque and should only be used with WinAPI calls
    // - Pointers are only valid if their corresponding X_SECURITY_INFORMATION
    //   flags are set
    // - sd_ptr must be freed with LocalFree
    let error_code = unsafe {
        winapi::um::aclapi::GetNamedSecurityInfoW(
            windows_path.as_ptr(),
            winapi::um::accctrl::SE_FILE_OBJECT,
            winnt::OWNER_SECURITY_INFORMATION
                | winnt::GROUP_SECURITY_INFORMATION
                | winnt::DACL_SECURITY_INFORMATION,
            &mut owner_sid_ptr,
            &mut group_sid_ptr,
            &mut dacl_ptr,
            null_mut(),
            &mut sd_ptr,
        )
    };

    if error_code != winerror::ERROR_SUCCESS {
        return Err(std::io::Error::from_raw_os_error(error_code as i32));
    }

    // Assumptions:
    // - owner_sid_ptr is valid
    // - group_sid_ptr is valid
    // (both OK because GetNamedSecurityInfoW returned success)

    let owner = match unsafe { lookup_account_sid(owner_sid_ptr) } {
        Ok((n, d)) => {
            let owner_name = os_from_buf(&n);
            let owner_domain = os_from_buf(&d);

            format!(
                "{}\\{}",
                owner_domain.to_string_lossy(),
                &owner_name.to_string_lossy()
            )
        }
        Err(_) => String::from("-"),
    };

    let group = match unsafe { lookup_account_sid(group_sid_ptr) } {
        Ok((n, d)) => {
            let group_name = os_from_buf(&n);
            let group_domain = os_from_buf(&d);

            format!(
                "{}\\{}",
                group_domain.to_string_lossy(),
                &group_name.to_string_lossy()
            )
        }
        Err(_) => String::from("-"),
    };

    // This structure will be returned
    let owner = Owner::new(owner, group);

    // Get the size and allocate bytes for a 1-sub-authority SID
    // 1 sub-authority because the Windows World SID is always S-1-1-0, with
    // only a single sub-authority.
    //
    // Assumptions: None
    // "This function cannot fail"
    //     -- Windows Dev Center docs
    let mut world_sid_len: u32 = unsafe { winapi::um::securitybaseapi::GetSidLengthRequired(1) };
    let mut world_sid = vec![0u8; world_sid_len as usize];

    // Assumptions:
    // - world_sid_len is no larger than the number of bytes available at
    //   world_sid
    // - world_sid is appropriately aligned (if there are strange crashes this
    //   might be why)
    let result = unsafe {
        winapi::um::securitybaseapi::CreateWellKnownSid(
            winnt::WinWorldSid,
            null_mut(),
            world_sid.as_mut_ptr() as *mut _,
            &mut world_sid_len,
        )
    };

    if result == 0 {
        // Failed to create the SID
        // Assumptions: Same as the other identical calls
        unsafe {
            winapi::um::winbase::LocalFree(sd_ptr);
        }

        // Assumptions: None (GetLastError shouldn't ever fail)
        return Err(io::Error::from_raw_os_error(unsafe {
            winapi::um::errhandlingapi::GetLastError()
        } as i32));
    }

    // Assumptions:
    // - xxxxx_sid_ptr are valid pointers to SIDs
    // - xxxxx_trustee is only valid as long as its SID pointer is
    let mut owner_trustee = unsafe { trustee_from_sid(owner_sid_ptr) };
    let mut group_trustee = unsafe { trustee_from_sid(group_sid_ptr) };
    let mut world_trustee = unsafe { trustee_from_sid(world_sid.as_mut_ptr() as *mut _) };

    // Assumptions:
    // - xxxxx_trustee are still valid (including underlying SID)
    // - dacl_ptr is still valid
    let owner_access_mask = unsafe { get_acl_access_mask(dacl_ptr as *mut _, &mut owner_trustee) }?;

    let group_access_mask = unsafe { get_acl_access_mask(dacl_ptr as *mut _, &mut group_trustee) }?;

    let world_access_mask = unsafe { get_acl_access_mask(dacl_ptr as *mut _, &mut world_trustee) }?;

    let has_bit = |field: u32, bit: u32| field & bit != 0;

    let permissions = Permissions {
        user_read: has_bit(owner_access_mask, winnt::FILE_GENERIC_READ),
        user_write: has_bit(owner_access_mask, winnt::FILE_GENERIC_WRITE),
        user_execute: has_bit(owner_access_mask, winnt::FILE_GENERIC_EXECUTE),

        group_read: has_bit(group_access_mask, winnt::FILE_GENERIC_READ),
        group_write: has_bit(group_access_mask, winnt::FILE_GENERIC_WRITE),
        group_execute: has_bit(group_access_mask, winnt::FILE_GENERIC_EXECUTE),

        other_read: has_bit(world_access_mask, winnt::FILE_GENERIC_READ),
        other_write: has_bit(world_access_mask, winnt::FILE_GENERIC_WRITE),
        other_execute: has_bit(world_access_mask, winnt::FILE_GENERIC_EXECUTE),

        sticky: false,
        setuid: false,
        setgid: false,
    };

    // Assumptions:
    // - sd_ptr was previously allocated with WinAPI functions
    // - All pointers into the memory are now invalid
    // - The free succeeds (currently unchecked -- there's no real recovery
    //   options. It's not much memory, so leaking it on failure is
    //   *probably* fine)
    unsafe {
        winapi::um::winbase::LocalFree(sd_ptr);
    }

    Ok((owner, permissions))
}

/// Evaluate an ACL for a particular trustee and get its access rights
///
/// Assumptions:
/// - acl_ptr points to a valid ACL data structure
/// - trustee_ptr points to a valid trustee data structure
/// - Both remain valid through the function call (no long-term requirement)
unsafe fn get_acl_access_mask(
    acl_ptr: *mut c_void,
    trustee_ptr: *mut TRUSTEE_W,
) -> Result<u32, io::Error> {
    let mut access_mask = 0;

    // Assumptions:
    // - All function assumptions
    // - Result is not valid until return value is checked
    let err_code = winapi::um::aclapi::GetEffectiveRightsFromAclW(
        acl_ptr as *mut _,
        trustee_ptr,
        &mut access_mask,
    );

    if err_code == winerror::ERROR_SUCCESS {
        Ok(access_mask)
    } else {
        Err(io::Error::from_raw_os_error(err_code as i32))
    }
}

/// Get a trustee buffer from a SID
///
/// Assumption: sid is valid, and the trustee is only valid as long as the SID
/// is
///
/// Note: winapi's TRUSTEE_W looks different from the one in the MS docs because
/// of some unusal pre-processor macros in the original .h file. The winapi
/// version is correct (MS's doc generator messed up)
unsafe fn trustee_from_sid(sid_ptr: *mut c_void) -> TRUSTEE_W {
    let mut trustee: TRUSTEE_W = std::mem::zeroed();

    winapi::um::aclapi::BuildTrusteeWithSidW(&mut trustee, sid_ptr);

    trustee
}

/// Get a username and domain name from a SID
///
/// Assumption: sid is a valid pointer that remains valid through the entire
/// function execution
///
/// Returns null-terminated Vec's, one for the name and one for the domain.
unsafe fn lookup_account_sid(sid: *mut c_void) -> Result<(Vec<u16>, Vec<u16>), std::io::Error> {
    let mut name_size: u32 = BUF_SIZE;
    let mut domain_size: u32 = BUF_SIZE;

    loop {
        let mut name: Vec<u16> = vec![0; name_size as usize];
        let mut domain: Vec<u16> = vec![0; domain_size as usize];

        let old_name_size = name_size;
        let old_domain_size = domain_size;

        let mut sid_name_use = 0;

        // Assumptions:
        // - sid is a valid pointer to a SID data structure
        // - name_size and domain_size accurately reflect the sizes
        let result = winapi::um::winbase::LookupAccountSidW(
            null_mut(),
            sid,
            name.as_mut_ptr(),
            &mut name_size,
            domain.as_mut_ptr(),
            &mut domain_size,
            &mut sid_name_use,
        );

        if result != 0 {
            // Success!
            return Ok((name, domain));
        } else if name_size != old_name_size || domain_size != old_domain_size {
            // Need bigger buffers
            // name_size and domain_size are already set, just loop
            continue;
        } else {
            // Unknown account and or system domain identification
            // Possibly foreign item originating from another machine
            // TODO: Calculate permissions since it has to be possible if Explorer knows.
            return Err(io::Error::from_raw_os_error(
                winapi::um::errhandlingapi::GetLastError() as i32,
            ));
        }
    }
}

/// Create an `OsString` from a NUL-terminated buffer
///
/// Decodes the WTF-16 encoded buffer until it hits a NUL (code point 0).
/// Everything after and including that code point is not included.
fn os_from_buf(buf: &[u16]) -> OsString {
    OsString::from_wide(
        &buf.iter()
            .cloned()
            .take_while(|&n| n != 0)
            .collect::<Vec<u16>>(),
    )
}

/// Create a WTF-16-encoded NUL-terminated buffer from an `OsStr`.
///
/// Decodes the `OsStr`, then appends a NUL.
fn buf_from_os(os: &OsStr) -> Vec<u16> {
    let mut buf: Vec<u16> = os.encode_wide().collect();
    buf.push(0);
    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_wtf16_behavior() {
        let basic_os = OsString::from("TeSt");
        let basic_buf = vec![0x54, 0x65, 0x53, 0x74, 0x00];
        let basic_buf_nuls = vec![0x54, 0x65, 0x53, 0x74, 0x00, 0x00, 0x00, 0x00];

        assert_eq!(os_from_buf(&basic_buf), basic_os);
        assert_eq!(buf_from_os(&basic_os), basic_buf);
        assert_eq!(os_from_buf(&basic_buf_nuls), basic_os);

        let unicode_os = OsString::from("💩");
        let unicode_buf = vec![0xd83d, 0xdca9, 0x0];
        let unicode_buf_nuls = vec![0xd83d, 0xdca9, 0x0, 0x0, 0x0, 0x0, 0x0];

        assert_eq!(os_from_buf(&unicode_buf), unicode_os);
        assert_eq!(buf_from_os(&unicode_os), unicode_buf);
        assert_eq!(os_from_buf(&unicode_buf_nuls), unicode_os);
    }

    #[test]
    fn every_wtf16_codepair_roundtrip() {
        for lsb in 0..256u16 {
            let mut vec: Vec<u16> = Vec::with_capacity(257);

            for msb in 0..=256u16 {
                let val = msb << 8 | lsb;

                if val != 0 {
                    vec.push(val)
                }
            }

            vec.push(0);

            let os = os_from_buf(&vec);
            let new_vec = buf_from_os(&os);

            assert_eq!(&vec, &new_vec);
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6427() {
    rusty_monitor::set_test_id(6427);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut bool_0: bool = false;
    let mut str_0: &str = "pH0qsbV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 3687u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<usize> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_5, depth: option_4};
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 792u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 3373u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut str_1: &str = "YwB5iOmTZiTeRi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_11: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_10};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_3_ref_0: &meta::filetype::FileType = &mut filetype_3;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_15, uid: bool_0};
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_16};
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut filetype_6_ref_0: &meta::filetype::FileType = &mut filetype_6;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    crate::meta::filetype::FileType::render(filetype_2, colors_0_ref_0);
    panic!("From RustyUnit with love");
}
}