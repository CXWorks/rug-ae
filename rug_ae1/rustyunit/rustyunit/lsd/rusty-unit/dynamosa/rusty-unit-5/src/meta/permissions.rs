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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions::clone(permissions_0_ref_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    panic!("From RustyUnit with love");
}
}