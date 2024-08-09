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
	use std::clone::Clone;
	use std::cmp::Eq;
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_767() {
    rusty_monitor::set_test_id(767);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut usize_0: usize = 57usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "DYuAxYpY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "a";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_2: &str = "Xpl0GKDqLIU";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut str_3: &str = "w";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut bool_1: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_824() {
    rusty_monitor::set_test_id(824);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut str_0: &str = "oBrcKNY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Ti6Dr5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::clone(sizeflag_0_ref_0);
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::clone(sizeflag_1_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_4, broken: color_3, missing_target: color_2};
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3729() {
    rusty_monitor::set_test_id(3729);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut str_0: &str = "E09i5KwsugsD87";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_5: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut usize_0: usize = 78usize;
    let mut bool_5: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_1: &str = "Gm9E2d";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_6: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_6};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 83usize;
    let mut bool_7: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut bool_8: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_8};
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_20, user_write: bool_19, user_execute: bool_18, group_read: bool_17, group_write: bool_16, group_execute: bool_15, other_read: bool_14, other_write: bool_13, other_execute: bool_12, sticky: bool_11, setgid: bool_10, setuid: bool_9};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_11: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut option_12: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_2_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2522() {
    rusty_monitor::set_test_id(2522);
    let mut u64_0: u64 = 58u64;
    let mut usize_0: usize = 95usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut u64_1: u64 = 74u64;
    let mut usize_1: usize = 61usize;
    let mut bool_7: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "dQg2dj0erLUBfd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_19, user_write: bool_18, user_execute: bool_17, group_read: bool_16, group_write: bool_15, group_execute: bool_14, other_read: bool_13, other_write: bool_12, other_execute: bool_11, sticky: bool_10, setgid: bool_9, setuid: bool_8};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_20: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_20);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut bool_21: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_21);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_22: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_22);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_23: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_23);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_24: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_24);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_25: bool = false;
    let mut bool_26: bool = false;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_28, exec: bool_27};
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut bool_29: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_26, uid: bool_25};
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_1_ref_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut block_5: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut option_20: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3502() {
    rusty_monitor::set_test_id(3502);
    let mut u64_0: u64 = 48u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut usize_0: usize = 37usize;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_13};
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_12, depth: usize_0};
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut bool_14: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3822() {
    rusty_monitor::set_test_id(3822);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_0: &str = "ejAxlZzOB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2579() {
    rusty_monitor::set_test_id(2579);
    let mut usize_0: usize = 96usize;
    let mut bool_0: bool = false;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "kjmlFrLPa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_8: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_7: bool = true;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_8, permission: option_7, sorting: option_6, no_symlink: option_5, total_size: option_4, symlink_arrow: option_3, hyperlink: option_2};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_8: bool = true;
    let mut usize_1: usize = 69usize;
    let mut option_25: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_9: bool = false;
    let mut option_26: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_1_ref_0);
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_26, depth: option_25};
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_8};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_10: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_11: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2791() {
    rusty_monitor::set_test_id(2791);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut usize_0: usize = 53usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 45u64;
    let mut bool_1: bool = false;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3627() {
    rusty_monitor::set_test_id(3627);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_4, uid: bool_3};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_5: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_6: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_6};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut str_0: &str = "r97zxyQEeQ9J";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Context;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_7: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_7};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::clone(sizeflag_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_6, invalid: color_5};
    let mut option_3: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_4, day_old: color_3, older: color_2};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut elem_7: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut elem_8: color::Elem = crate::color::Elem::NoAccess;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut elem_9: color::Elem = crate::color::Elem::Acl;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3631() {
    rusty_monitor::set_test_id(3631);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 33usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut str_0: &str = "5wzcU89";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut str_1: &str = "dT89rDtbcKMrh";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_2: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut bool_3: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut bool_4: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 39u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut bool_6: bool = crate::flags::size::SizeFlag::eq(sizeflag_3_ref_0, sizeflag_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    panic!("From RustyUnit with love");
}
}