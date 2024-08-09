//! This module defines the [PermissionFlag]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file permissions units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionFlag {
    /// The variant to show file permissions in rwx format
    Rwx,
    /// The variant to show file permissions in octal format
    Octal,
}

impl PermissionFlag {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "rwx" => Some(Self::Rwx),
            "octal" => Some(Self::Octal),
            _ => {
                panic!(
                    "Permissions can only be one of rwx or octal, but got {}.",
                    value
                );
            }
        }
    }
}

impl Configurable<Self> for PermissionFlag {
    /// Get a potential `PermissionFlag` variant from [ArgMatches].
    ///
    /// If any of the "rwx" or "octal" arguments is passed, the corresponding
    /// `PermissionFlag` variant is returned in a [Some]. If neither of them is passed,
    /// this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            return Some(Self::Rwx);
        } else if matches.occurrences_of("permission") > 0 {
            if let Some(permissions) = matches.values_of("permission")?.last() {
                return Self::from_str(permissions);
            }
        }
        None
    }

    /// Get a potential `PermissionFlag` variant from a [Config].
    ///
    /// If the `Config::permissions` has value and is one of "rwx" or "octal",
    /// this returns the corresponding `PermissionFlag` variant in a [Some].
    /// Otherwise this returns [None].
    /// Sets permissions to rwx if classic flag is enabled.
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            Some(Self::Rwx)
        } else {
            config.permission
        }
    }
}

/// The default value for `PermissionFlag` is [PermissionFlag::Default].
impl Default for PermissionFlag {
    fn default() -> Self {
        Self::Rwx
    }
}

#[cfg(test)]
mod test {
    use super::PermissionFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_default() {
        assert_eq!(PermissionFlag::Rwx, PermissionFlag::default());
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, PermissionFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_default() {
        let argv = vec!["lsd", "--permission", "rwx"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_short() {
        let args = vec!["lsd", "--permission", "octal"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Octal),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    #[should_panic]
    fn test_from_arg_matches_unknown() {
        let args = vec!["lsd", "--permission", "unknown"];
        let _ = app::build().get_matches_from_safe(args).unwrap();
    }
    #[test]
    fn test_from_arg_matches_permissions_multi() {
        let args = vec!["lsd", "--permission", "octal", "--permission", "rwx"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_permissions_classic() {
        let args = vec!["lsd", "--permission", "rwx", "--classic"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(
            Some(PermissionFlag::Rwx),
            PermissionFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, PermissionFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_rwx() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Rwx);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_octal() {
        let mut c = Config::with_none();
        c.permission = Some(PermissionFlag::Octal);
        assert_eq!(Some(PermissionFlag::Octal), PermissionFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(Some(PermissionFlag::Rwx), PermissionFlag::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5062() {
//    rusty_monitor::set_test_id(5062);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6439() {
//    rusty_monitor::set_test_id(6439);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut bool_12: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_166() {
//    rusty_monitor::set_test_id(166);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_3_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_3;
    let mut permissionflag_4: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_4_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_4;
    let mut permissionflag_5: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_5_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_5;
    let mut permissionflag_6: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_6_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_6;
    let mut permissionflag_7: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_7_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_7;
    let mut permissionflag_8: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_8_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_8;
    let mut permissionflag_9: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_9_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_9;
    let mut permissionflag_10: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_10_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_10;
    let mut permissionflag_11: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_11_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_11;
    let mut permissionflag_12: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_12_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_12;
    let mut permissionflag_13: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_13_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_13;
    let mut bool_0: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_13_ref_0, permissionflag_12_ref_0);
    let mut bool_1: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_11_ref_0, permissionflag_10_ref_0);
    let mut bool_2: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_9_ref_0, permissionflag_8_ref_0);
    let mut bool_3: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_7_ref_0, permissionflag_6_ref_0);
    let mut bool_4: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_5_ref_0, permissionflag_4_ref_0);
    let mut bool_5: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_3_ref_0, permissionflag_2_ref_0);
    let mut bool_6: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9115() {
//    rusty_monitor::set_test_id(9115);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 96usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::clone(permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4648() {
//    rusty_monitor::set_test_id(4648);
    let mut u64_0: u64 = 59u64;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_0_ref_0);
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_601() {
//    rusty_monitor::set_test_id(601);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "Natural sort of (version) numbers within text";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "m4a";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_1_ref_0);
    let mut str_2: &str = "mkd";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_0: usize = 6usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_3: &str = "gitignore_global";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}
}