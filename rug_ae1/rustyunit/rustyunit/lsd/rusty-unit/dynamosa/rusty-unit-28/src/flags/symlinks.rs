//! This module defines the [NoSymlink] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to follow symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct NoSymlink(pub bool);

impl Configurable<Self> for NoSymlink {
    /// Get a potential `NoSymlink` value from [ArgMatches].
    ///
    /// If the "no-symlink" argument is passed, this returns a `NoSymlink` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("no-symlink") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `NoSymlink` value from a [Config].
    ///
    /// If the `Config::no-symlink` has value,
    /// this returns it as the value of the `NoSymlink`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.no_symlink.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::NoSymlink;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--no-symlink"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.no_symlink = Some(true);
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.no_symlink = Some(false);
        assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut usize_0: usize = 6993usize;
    let mut bool_14: bool = true;
    let mut usize_1: usize = 8145usize;
    let mut bool_15: bool = false;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_16: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut bool_17: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_17);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_2: usize = 4102usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_18: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_18);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_19: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_19);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_20: bool = false;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_20);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut bool_31: bool = false;
    let mut bool_32: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_32, user_write: bool_31, user_execute: bool_30, group_read: bool_29, group_write: bool_28, group_execute: bool_27, other_read: bool_26, other_write: bool_25, other_execute: bool_24, sticky: bool_23, setgid: bool_22, setuid: bool_21};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_34, exec: bool_33};
    let mut bool_35: bool = false;
    let mut bool_36: bool = false;
    let mut bool_37: bool = true;
    let mut bool_38: bool = true;
    let mut bool_39: bool = false;
    let mut bool_40: bool = false;
    let mut bool_41: bool = false;
    let mut bool_42: bool = false;
    let mut bool_43: bool = false;
    let mut bool_44: bool = true;
    let mut bool_45: bool = false;
    let mut bool_46: bool = true;
    let mut bool_47: bool = false;
    let mut bool_48: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_48, user_write: bool_47, user_execute: bool_46, group_read: bool_45, group_write: bool_44, group_execute: bool_43, other_read: bool_42, other_write: bool_41, other_execute: bool_40, sticky: bool_39, setgid: bool_38, setuid: bool_37};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::clone(nosymlink_0_ref_0);
    let mut bool_49: bool = crate::meta::permissions::Permissions::is_executable(permissions_1_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_36, uid: bool_35};
    panic!("From RustyUnit with love");
}
}