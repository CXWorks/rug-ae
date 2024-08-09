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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3214() {
    rusty_monitor::set_test_id(3214);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_26, bool_25, bool_24);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_27: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3559() {
    rusty_monitor::set_test_id(3559);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Pipe;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Context;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = false;
    let mut bool_27: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_27, user_write: bool_26, user_execute: bool_25, group_read: bool_24, group_write: bool_23, group_execute: bool_22, other_read: bool_21, other_write: bool_20, other_execute: bool_19, sticky: bool_18, setgid: bool_17, setuid: bool_16};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_30, bool_29, bool_28);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_8: color::Elem = crate::color::Elem::HourOld;
    let mut bool_31: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3003() {
    rusty_monitor::set_test_id(3003);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = true;
    let mut bool_25: bool = true;
    let mut bool_26: bool = false;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = false;
    let mut bool_34: bool = true;
    let mut bool_35: bool = false;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = true;
    let mut bool_37: bool = true;
    let mut bool_38: bool = true;
    let mut bool_39: bool = true;
    let mut bool_40: bool = false;
    let mut bool_41: bool = true;
    let mut bool_42: bool = false;
    let mut bool_43: bool = true;
    let mut bool_44: bool = true;
    let mut bool_45: bool = false;
    let mut bool_46: bool = false;
    let mut bool_47: bool = false;
    let mut permissions_3: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_47, user_write: bool_46, user_execute: bool_45, group_read: bool_44, group_write: bool_43, group_execute: bool_42, other_read: bool_41, other_write: bool_40, other_execute: bool_39, sticky: bool_38, setgid: bool_37, setuid: bool_36};
    let mut permissions_3_ref_0: &crate::meta::permissions::Permissions = &mut permissions_3;
    let mut bool_48: bool = true;
    let mut bool_49: bool = false;
    let mut bool_50: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_50, bool_49, bool_48);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_51: bool = crate::meta::permissions::Permissions::eq(permissions_3_ref_0, permissions_2_ref_0);
    let mut bool_52: bool = crate::meta::permissions::Permissions::ne(permissions_1_ref_0, permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_142() {
    rusty_monitor::set_test_id(142);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
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
    let mut bool_24: bool = true;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = true;
    let mut bool_32: bool = true;
    let mut bool_33: bool = false;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut permissions_2: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_35, user_write: bool_34, user_execute: bool_33, group_read: bool_32, group_write: bool_31, group_execute: bool_30, other_read: bool_29, other_write: bool_28, other_execute: bool_27, sticky: bool_26, setgid: bool_25, setuid: bool_24};
    let mut permissions_2_ref_0: &crate::meta::permissions::Permissions = &mut permissions_2;
    let mut bool_36: bool = true;
    let mut bool_37: bool = false;
    let mut bool_38: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_38, bool_37, bool_36);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_39: bool = crate::meta::permissions::Permissions::eq(permissions_2_ref_0, permissions_1_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut permissions_3: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions::clone(permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7540() {
    rusty_monitor::set_test_id(7540);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_24: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7043() {
    rusty_monitor::set_test_id(7043);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_26, bool_25, bool_24);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_27: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7099() {
    rusty_monitor::set_test_id(7099);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_14, bool_13, bool_12);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7642() {
    rusty_monitor::set_test_id(7642);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut u8_0: u8 = crate::meta::permissions::Permissions::bits_to_octal(bool_26, bool_25, bool_24);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut bool_27: bool = crate::meta::permissions::Permissions::eq(permissions_1_ref_0, permissions_0_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    panic!("From RustyUnit with love");
}
}