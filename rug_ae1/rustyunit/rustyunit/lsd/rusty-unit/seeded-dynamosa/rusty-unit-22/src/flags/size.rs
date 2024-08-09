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
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_735() {
//    rusty_monitor::set_test_id(735);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut sizeflag_4: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_4_ref_0: &flags::size::SizeFlag = &mut sizeflag_4;
    let mut sizeflag_5: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_5_ref_0: &flags::size::SizeFlag = &mut sizeflag_5;
    let mut sizeflag_6: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_6_ref_0: &flags::size::SizeFlag = &mut sizeflag_6;
    let mut sizeflag_7: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_7_ref_0: &flags::size::SizeFlag = &mut sizeflag_7;
    let mut sizeflag_8: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_8_ref_0: &flags::size::SizeFlag = &mut sizeflag_8;
    let mut sizeflag_9: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_9_ref_0: &flags::size::SizeFlag = &mut sizeflag_9;
    let mut sizeflag_10: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_10_ref_0: &flags::size::SizeFlag = &mut sizeflag_10;
    let mut sizeflag_11: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_11_ref_0: &flags::size::SizeFlag = &mut sizeflag_11;
    let mut sizeflag_12: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_12_ref_0: &flags::size::SizeFlag = &mut sizeflag_12;
    let mut sizeflag_13: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_13_ref_0: &flags::size::SizeFlag = &mut sizeflag_13;
    let mut bool_0: bool = crate::flags::size::SizeFlag::eq(sizeflag_13_ref_0, sizeflag_12_ref_0);
    let mut bool_1: bool = crate::flags::size::SizeFlag::eq(sizeflag_11_ref_0, sizeflag_10_ref_0);
    let mut bool_2: bool = crate::flags::size::SizeFlag::eq(sizeflag_9_ref_0, sizeflag_8_ref_0);
    let mut bool_3: bool = crate::flags::size::SizeFlag::eq(sizeflag_7_ref_0, sizeflag_6_ref_0);
    let mut bool_4: bool = crate::flags::size::SizeFlag::eq(sizeflag_5_ref_0, sizeflag_4_ref_0);
    let mut bool_5: bool = crate::flags::size::SizeFlag::eq(sizeflag_3_ref_0, sizeflag_2_ref_0);
    let mut bool_6: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1577() {
//    rusty_monitor::set_test_id(1577);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::Pipe;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_1_ref_0);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut bool_0: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6805() {
//    rusty_monitor::set_test_id(6805);
    let mut str_0: &str = "x1gq7vREbtS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "owner";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "sql";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "css";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "ttc";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Short";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "When showing file information for a symbolic link, show information for the file the link references rather than for the link itself";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "uid";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "long";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "guardfile";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_9_ref_0);
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_8_ref_0);
    let mut option_2: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_7_ref_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_6_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2929() {
//    rusty_monitor::set_test_id(2929);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_6: color::Elem = crate::color::Elem::Octal;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_1_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3205() {
//    rusty_monitor::set_test_id(3205);
    let mut str_0: &str = "color";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 6usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_2);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_2: usize = 80usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_17: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_17, theme: option_16};
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_21: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}