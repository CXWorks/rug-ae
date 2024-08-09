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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_244() {
    rusty_monitor::set_test_id(244);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 38usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut str_0: &str = "r3EIUah";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_13: bool = false;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::clone(sizeflag_1_ref_0);
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_2_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_13};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_464() {
    rusty_monitor::set_test_id(464);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut u64_0: u64 = 29u64;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 47usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_1: u64 = 58u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut bool_1: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4552() {
    rusty_monitor::set_test_id(4552);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 88usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "7tWWB2Fgt7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "03L";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_3: std::option::Option<usize> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2132() {
    rusty_monitor::set_test_id(2132);
    let mut usize_0: usize = 89usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_1: usize = 37usize;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_6: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_2: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_8: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_8, reverse: option_7, dir_grouping: option_6};
    let mut option_9: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut option_11: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_2: usize = 28usize;
    let mut option_12: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_13, depth: option_12};
    let mut option_14: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_15: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_20, theme: option_19, separator: option_18};
    let mut option_21: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_22: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_27: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_24, dereference: option_23, display: option_22, icons: option_21, ignore_globs: option_17, indicators: option_16, layout: option_15, recursion: option_14, size: option_11, permission: option_10, sorting: option_9, no_symlink: option_5, total_size: option_4, symlink_arrow: option_3, hyperlink: option_2};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 87u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_28: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "oOhemlzxxmqb";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut option_29: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_4_ref_0);
    let mut option_30: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_3: usize = 11usize;
    let mut option_31: std::option::Option<usize> = std::option::Option::Some(usize_3);
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut option_32: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_33: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut config_6: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_6_ref_0: &crate::config_file::Config = &mut config_6;
    let mut option_34: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    panic!("From RustyUnit with love");
}
}