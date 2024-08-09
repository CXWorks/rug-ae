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
fn rusty_test_5064() {
    rusty_monitor::set_test_id(5064);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut usize_0: usize = 29usize;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut usize_1: usize = 35usize;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut usize_2: usize = 76usize;
    let mut bool_10: bool = false;
    let mut usize_3: usize = 81usize;
    let mut bool_11: bool = true;
    let mut u64_0: u64 = 7u64;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut usize_4: usize = 80usize;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut u64_1: u64 = 75u64;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut usize_5: usize = 67usize;
    let mut bool_29: bool = false;
    let mut bool_30: bool = false;
    let mut bool_31: bool = true;
    let mut bool_32: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_32);
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_4: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_4, theme: option_3, separator: option_2};
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_6: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_33: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_33);
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_6: usize = 13usize;
    let mut bool_34: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_34, depth: usize_6};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_12: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_17: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_18: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_7: usize = 28usize;
    let mut option_19: std::option::Option<usize> = std::option::Option::Some(usize_7);
    let mut bool_35: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_35);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_20, depth: option_19};
    let mut option_21: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_22: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_36: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_36);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_25: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_26: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_27: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_27, theme: option_26, separator: option_25};
    let mut option_28: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut option_29: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_37: bool = false;
    let mut option_30: std::option::Option<bool> = std::option::Option::Some(bool_37);
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_32: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_33: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_38: bool = false;
    let mut option_34: std::option::Option<bool> = std::option::Option::Some(bool_38);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_34, blocks: option_33, color: option_32, date: option_31, dereference: option_30, display: option_29, icons: option_28, ignore_globs: option_24, indicators: option_23, layout: option_22, recursion: option_21, size: option_18, permission: option_17, sorting: option_16, no_symlink: option_15, total_size: option_14, symlink_arrow: option_13, hyperlink: option_12};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "50N7XOcqQlOmiU";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5077() {
    rusty_monitor::set_test_id(5077);
    let mut str_0: &str = "TX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 53usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_0_ref_0);
    let mut usize_1: usize = 84usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut usize_2: usize = 43usize;
    let mut bool_13: bool = false;
    let mut u64_0: u64 = 38u64;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut bool_36: bool = true;
    let mut bool_37: bool = true;
    let mut usize_3: usize = 91usize;
    let mut bool_38: bool = false;
    let mut u64_1: u64 = 70u64;
    let mut u64_2: u64 = 5u64;
    let mut str_1: &str = "TCsMkjMy";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_39: bool = false;
    let mut usize_4: usize = 71usize;
    let mut bool_40: bool = false;
    let mut u64_3: u64 = 29u64;
    let mut bool_41: bool = false;
    let mut bool_42: bool = true;
    let mut bool_43: bool = true;
    let mut bool_44: bool = false;
    let mut bool_45: bool = true;
    let mut bool_46: bool = true;
    let mut bool_47: bool = false;
    let mut bool_48: bool = false;
    let mut bool_49: bool = true;
    let mut bool_50: bool = false;
    let mut bool_51: bool = true;
    let mut bool_52: bool = true;
    let mut bool_53: bool = true;
    let mut bool_54: bool = true;
    let mut bool_55: bool = false;
    let mut bool_56: bool = true;
    let mut bool_57: bool = false;
    let mut bool_58: bool = true;
    let mut bool_59: bool = false;
    let mut bool_60: bool = true;
    let mut bool_61: bool = true;
    let mut bool_62: bool = true;
    let mut bool_63: bool = false;
    let mut bool_64: bool = false;
    let mut str_2: &str = "od9eT";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_5: usize = 89usize;
    let mut tuple_1: (usize, &str) = (usize_5, str_2_ref_0);
    let mut usize_6: usize = 52usize;
    let mut bool_65: bool = true;
    let mut usize_7: usize = 10usize;
    let mut bool_66: bool = true;
    let mut u64_4: u64 = 61u64;
    let mut bool_67: bool = true;
    let mut bool_68: bool = true;
    let mut bool_69: bool = true;
    let mut bool_70: bool = false;
    let mut bool_71: bool = true;
    let mut bool_72: bool = false;
    let mut bool_73: bool = true;
    let mut bool_74: bool = false;
    let mut bool_75: bool = true;
    let mut bool_76: bool = false;
    let mut bool_77: bool = true;
    let mut bool_78: bool = true;
    let mut usize_8: usize = 97usize;
    let mut bool_79: bool = true;
    let mut u64_5: u64 = 70u64;
    let mut u64_6: u64 = 10u64;
    let mut bool_80: bool = true;
    let mut usize_9: usize = 53usize;
    let mut bool_81: bool = true;
    let mut usize_10: usize = 1usize;
    let mut usize_11: usize = 84usize;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_12: usize = 81usize;
    let mut bool_82: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_82, depth: usize_12};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut bool_83: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_83};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_13: usize = 36usize;
    let mut bool_84: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_84, depth: usize_13};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_14: usize = 14usize;
    let mut bool_85: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_85, depth: usize_14};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_4};
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_86: bool = true;
    let mut bool_87: bool = true;
    let mut bool_88: bool = true;
    let mut bool_89: bool = false;
    let mut bool_90: bool = true;
    let mut bool_91: bool = true;
    let mut bool_92: bool = true;
    let mut bool_93: bool = false;
    let mut bool_94: bool = true;
    let mut bool_95: bool = false;
    let mut bool_96: bool = true;
    let mut bool_97: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_97, user_write: bool_96, user_execute: bool_95, group_read: bool_94, group_write: bool_93, group_execute: bool_92, other_read: bool_91, other_write: bool_90, other_execute: bool_89, sticky: bool_88, setgid: bool_87, setuid: bool_86};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    panic!("From RustyUnit with love");
}
}