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
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_471() {
//    rusty_monitor::set_test_id(471);
    let mut str_0: &str = "AuR1eVc1zXu3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "sort";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "jsx";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "recursion";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "group";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "hyperlink";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "rakefile";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "indicators";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "Custom";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "IconSeparator";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "pdf";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut option_0: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_10_ref_0);
    let mut option_1: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_9_ref_0);
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_8_ref_0);
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_7_ref_0);
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_6_ref_0);
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_5_ref_0);
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_4_ref_0);
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_3_ref_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_2_ref_0);
    let mut option_9: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_1_ref_0);
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3796() {
//    rusty_monitor::set_test_id(3796);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut bool_13: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_842() {
//    rusty_monitor::set_test_id(842);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::clone(permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1882() {
//    rusty_monitor::set_test_id(1882);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut tuple_0: () = crate::flags::permission::PermissionFlag::assert_receiver_is_total_eq(permissionflag_0_ref_0);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4653() {
//    rusty_monitor::set_test_id(4653);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut bool_13: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut bool_14: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut bool_15: bool = crate::flags::permission::PermissionFlag::eq(permissionflag_1_ref_0, permissionflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7474() {
//    rusty_monitor::set_test_id(7474);
    let mut str_0: &str = "short";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_0: u64 = 39u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_2: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_3, reverse: option_1, dir_grouping: option_2};
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = crate::flags::permission::PermissionFlag::from_config(config_3_ref_0);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::default();
    let mut permissionflag_2_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_2;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
//    panic!("From RustyUnit with love");
}
}