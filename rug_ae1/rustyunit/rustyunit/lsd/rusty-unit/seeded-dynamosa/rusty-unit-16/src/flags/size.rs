//! This module defines the [SizeFlag]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file size units to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SizeFlag {
    /// The variant to show file size with SI unit prefix and a B for bytes.
    Default,
    /// The variant to show file size with only the SI unit prefix.
    Short,
    /// The variant to show file size in bytes.
    Bytes,
}

impl SizeFlag {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "default" => Some(Self::Default),
            "short" => Some(Self::Short),
            "bytes" => Some(Self::Bytes),
            _ => {
                panic!(
                    "Size can only be one of default, short or bytes, but got {}.",
                    value
                );
            }
        }
    }
}

impl Configurable<Self> for SizeFlag {
    /// Get a potential `SizeFlag` variant from [ArgMatches].
    ///
    /// If any of the "default", "short" or "bytes" arguments is passed, the corresponding
    /// `SizeFlag` variant is returned in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            return Some(Self::Bytes);
        } else if matches.occurrences_of("size") > 0 {
            if let Some(size) = matches.values_of("size")?.last() {
                return Self::from_str(size);
            }
        }
        None
    }

    /// Get a potential `SizeFlag` variant from a [Config].
    ///
    /// If the `Config::size` has value and is one of "default", "short" or "bytes",
    /// this returns the corresponding `SizeFlag` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            Some(Self::Bytes)
        } else {
            config.size
        }
    }
}

/// The default value for `SizeFlag` is [SizeFlag::Default].
impl Default for SizeFlag {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod test {
    use super::SizeFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_default() {
        assert_eq!(SizeFlag::Default, SizeFlag::default());
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_default() {
        let argv = vec!["lsd", "--size", "default"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SizeFlag::Default),
            SizeFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_short() {
        let args = vec!["lsd", "--size", "short"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_bytes() {
        let args = vec!["lsd", "--size", "bytes"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    #[should_panic]
    fn test_from_arg_matches_unknonwn() {
        let args = vec!["lsd", "--size", "unknown"];
        let _ = app::build().get_matches_from_safe(args).unwrap();
    }
    #[test]
    fn test_from_arg_matches_size_multi() {
        let args = vec!["lsd", "--size", "bytes", "--size", "short"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_size_classic() {
        let args = vec!["lsd", "--size", "short", "--classic"];
        let matches = app::build().get_matches_from_safe(args).unwrap();
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, SizeFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Default);
        assert_eq!(Some(SizeFlag::Default), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_short() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Short);
        assert_eq!(Some(SizeFlag::Short), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_bytes() {
        let mut c = Config::with_none();
        c.size = Some(SizeFlag::Bytes);
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        assert_eq!(Some(SizeFlag::Bytes), SizeFlag::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1032() {
//    rusty_monitor::set_test_id(1032);
    let mut bool_0: bool = false;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "iCspZah5thDOi7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "fpl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_0: usize = 8usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut str_2: &str = "Natural sort of (version) numbers within text";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut str_3: &str = "H9ow";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6755() {
//    rusty_monitor::set_test_id(6755);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6256() {
//    rusty_monitor::set_test_id(6256);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<usize> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_0, depth: option_4};
    let mut option_5: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_6: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_1: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut option_18: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = false;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_22: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_24: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_24, reverse: option_23, dir_grouping: option_22};
    let mut option_25: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_26: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_27: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_28: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_29: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_30: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_31: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_32: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_33: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_34: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut bool_6: bool = true;
    let mut option_35: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_36: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_37: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_38: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_38, theme: option_37};
    let mut option_39: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_40: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_41: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_42: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = std::option::Option::unwrap(option_42);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1378() {
//    rusty_monitor::set_test_id(1378);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1: color::Elem = crate::color::Elem::Context;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Read;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_7: color::Elem = crate::color::Elem::Group;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut bool_1: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
    let mut elem_8: color::Elem = crate::color::Elem::Socket;
//    panic!("From RustyUnit with love");
}
}