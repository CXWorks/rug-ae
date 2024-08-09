use crate::color::{ColoredString, Colors, Elem};
use crate::flags::{Flags, PermissionFlag};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Permissions {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,

    pub sticky: bool,
    pub setgid: bool,
    pub setuid: bool,
}

impl<'a> From<&'a Metadata> for Permissions {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let bits = meta.permissions().mode();
        let has_bit = |bit| bits & bit == bit;

        Self {
            user_read: has_bit(modes::USER_READ),
            user_write: has_bit(modes::USER_WRITE),
            user_execute: has_bit(modes::USER_EXECUTE),

            group_read: has_bit(modes::GROUP_READ),
            group_write: has_bit(modes::GROUP_WRITE),
            group_execute: has_bit(modes::GROUP_EXECUTE),

            other_read: has_bit(modes::OTHER_READ),
            other_write: has_bit(modes::OTHER_WRITE),
            other_execute: has_bit(modes::OTHER_EXECUTE),

            sticky: has_bit(modes::STICKY),
            setgid: has_bit(modes::SETGID),
            setuid: has_bit(modes::SETUID),
        }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        panic!("Cannot get permissions from metadata on Windows")
    }
}

impl Permissions {
    fn bits_to_octal(r: bool, w: bool, x: bool) -> u8 {
        (r as u8) * 4 + (w as u8) * 2 + (x as u8)
    }

    pub fn render(&self, colors: &Colors, flags: &Flags) -> ColoredString {
        let bit = |bit, chr: &'static str, elem: &Elem| {
            if bit {
                colors.colorize(String::from(chr), elem)
            } else {
                colors.colorize(String::from("-"), &Elem::NoAccess)
            }
        };

        let strings = match flags.permission {
            PermissionFlag::Rwx => vec![
                // User permissions
                bit(self.user_read, "r", &Elem::Read),
                bit(self.user_write, "w", &Elem::Write),
                match (self.user_execute, self.setuid) {
                    (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                    (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                    (false, true) => colors.colorize(String::from("S"), &Elem::ExecSticky),
                    (true, true) => colors.colorize(String::from("s"), &Elem::ExecSticky),
                },
                // Group permissions
                bit(self.group_read, "r", &Elem::Read),
                bit(self.group_write, "w", &Elem::Write),
                match (self.group_execute, self.setgid) {
                    (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                    (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                    (false, true) => colors.colorize(String::from("S"), &Elem::ExecSticky),
                    (true, true) => colors.colorize(String::from("s"), &Elem::ExecSticky),
                },
                // Other permissions
                bit(self.other_read, "r", &Elem::Read),
                bit(self.other_write, "w", &Elem::Write),
                match (self.other_execute, self.sticky) {
                    (false, false) => colors.colorize(String::from("-"), &Elem::NoAccess),
                    (true, false) => colors.colorize(String::from("x"), &Elem::Exec),
                    (false, true) => colors.colorize(String::from("T"), &Elem::ExecSticky),
                    (true, true) => colors.colorize(String::from("t"), &Elem::ExecSticky),
                },
            ],
            PermissionFlag::Octal => {
                let octal_sticky = Self::bits_to_octal(self.setuid, self.setgid, self.sticky);
                let octal_user =
                    Self::bits_to_octal(self.user_read, self.user_write, self.user_execute);
                let octal_group =
                    Self::bits_to_octal(self.group_read, self.group_write, self.group_execute);
                let octal_other =
                    Self::bits_to_octal(self.other_read, self.other_write, self.other_execute);
                vec![colors.colorize(
                    format!(
                        "{}{}{}{}",
                        octal_sticky, octal_user, octal_group, octal_other
                    ),
                    &Elem::Octal,
                )]
            }
        };

        let res = strings
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        ColoredString::new(Colors::default_style(), res)
    }

    pub fn is_executable(&self) -> bool {
        self.user_execute || self.group_execute || self.other_execute
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
#[cfg(unix)]
mod modes {
    pub type Mode = u32;
    // The `libc::mode_t` type’s actual type varies, but the value returned
    // from `metadata.permissions().mode()` is always `u32`.

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const USER_WRITE: Mode = libc::S_IWUSR as Mode;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;

    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const GROUP_WRITE: Mode = libc::S_IWGRP as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;

    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
    pub const OTHER_WRITE: Mode = libc::S_IWOTH as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const STICKY: Mode = libc::S_ISVTX as Mode;
    pub const SETGID: Mode = libc::S_ISGID as Mode;
    pub const SETUID: Mode = libc::S_ISUID as Mode;
}

#[cfg(unix)]
#[cfg(test)]
mod test {
    use super::Flags;
    use super::{PermissionFlag, Permissions};
    use crate::color::{Colors, ThemeOption};
    use std::fs;
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    #[test]
    pub fn permission_rwx() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o655))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let perms = Permissions::from(&meta);

        assert_eq!(
            "rw-r-xr-x",
            perms.render(&colors, &Flags::default()).content()
        );
    }

    #[test]
    pub fn permission_rwx2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o777))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let perms = Permissions::from(&meta);

        assert_eq!(
            "rwxrwxrwx",
            perms.render(&colors, &Flags::default()).content()
        );
    }

    #[test]
    pub fn permission_rwx_sticky() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o1777))
            .expect("unable to set permissions to file");

        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let mut flags = Flags::default();
        flags.permission = PermissionFlag::Rwx;
        let perms = Permissions::from(&meta);

        assert_eq!("rwxrwxrwt", perms.render(&colors, &flags).content());
    }

    #[test]
    pub fn permission_octal() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o655))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let mut flags = Flags::default();
        flags.permission = PermissionFlag::Octal;
        let perms = Permissions::from(&meta);

        assert_eq!("0655", perms.render(&colors, &flags).content());
    }

    #[test]
    pub fn permission_octal2() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o777))
            .expect("unable to set permissions to file");
        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let mut flags = Flags::default();
        flags.permission = PermissionFlag::Octal;
        let perms = Permissions::from(&meta);

        assert_eq!("0777", perms.render(&colors, &flags).content());
    }

    #[test]
    pub fn permission_octal_sticky() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        // Create the file;
        let file_path = tmp_dir.path().join("file.txt");
        File::create(&file_path).expect("failed to create file");
        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o1777))
            .expect("unable to set permissions to file");

        let meta = file_path.metadata().expect("failed to get meta");

        let colors = Colors::new(ThemeOption::NoColor);
        let mut flags = Flags::default();
        flags.permission = PermissionFlag::Octal;
        let perms = Permissions::from(&meta);

        assert_eq!("1777", perms.render(&colors, &flags).content());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8952() {
//    rusty_monitor::set_test_id(8952);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = true;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = crate::meta::permissions::Permissions::ne(permissions_1_ref_0, permissions_0_ref_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_898() {
//    rusty_monitor::set_test_id(898);
    let mut option_0: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_1: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_25, user_write: bool_24, user_execute: bool_23, group_read: bool_22, group_write: bool_21, group_execute: bool_20, other_read: bool_19, other_write: bool_18, other_execute: bool_17, sticky: bool_16, setgid: bool_15, setuid: bool_14};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = true;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut bool_36: bool = true;
    let mut bool_37: bool = true;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_37, user_write: bool_36, user_execute: bool_35, group_read: bool_34, group_write: bool_33, group_execute: bool_32, other_read: bool_31, other_write: bool_30, other_execute: bool_29, sticky: bool_28, setgid: bool_27, setuid: bool_26};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_38: bool = false;
    let mut bool_39: bool = false;
    let mut bool_40: bool = false;
    let mut bool_41: bool = false;
    let mut bool_42: bool = true;
    let mut bool_43: bool = false;
    let mut bool_44: bool = true;
    let mut bool_45: bool = true;
    let mut bool_46: bool = true;
    let mut bool_47: bool = true;
    let mut bool_48: bool = false;
    let mut bool_49: bool = true;
    let mut permissions_3: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_49, user_write: bool_48, user_execute: bool_47, group_read: bool_46, group_write: bool_45, group_execute: bool_44, other_read: bool_43, other_write: bool_42, other_execute: bool_41, sticky: bool_40, setgid: bool_39, setuid: bool_38};
    let mut permissions_3_ref_0: &crate::meta::permissions::Permissions = &mut permissions_3;
    let mut bool_50: bool = crate::meta::permissions::Permissions::eq(permissions_3_ref_0, permissions_2_ref_0);
    let mut bool_51: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4773() {
//    rusty_monitor::set_test_id(4773);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = true;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = crate::meta::permissions::Permissions::ne(permissions_2_ref_0, permissions_1_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_591() {
//    rusty_monitor::set_test_id(591);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = true;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut permissions_3: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions::clone(permissions_2_ref_0);
    let mut permissions_4: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions::clone(permissions_1_ref_0);
    let mut permissions_5: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions::clone(permissions_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_208() {
//    rusty_monitor::set_test_id(208);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = true;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = true;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut tuple_0: () = crate::meta::permissions::Permissions::assert_receiver_is_total_eq(permissions_2_ref_0);
    let mut tuple_1: () = crate::meta::permissions::Permissions::assert_receiver_is_total_eq(permissions_1_ref_0);
    let mut tuple_2: () = crate::meta::permissions::Permissions::assert_receiver_is_total_eq(permissions_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_145() {
//    rusty_monitor::set_test_id(145);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_0: &str = "tiff";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_2, bool_1, bool_0);
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4404() {
//    rusty_monitor::set_test_id(4404);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
//    panic!("From RustyUnit with love");
}
}