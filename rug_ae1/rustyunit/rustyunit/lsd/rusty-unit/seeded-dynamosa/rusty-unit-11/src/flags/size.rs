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
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_469() {
//    rusty_monitor::set_test_id(469);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut sizeflag_4: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_4_ref_0: &flags::size::SizeFlag = &mut sizeflag_4;
    let mut sizeflag_5: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_5_ref_0: &flags::size::SizeFlag = &mut sizeflag_5;
    let mut sizeflag_6: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_6_ref_0: &flags::size::SizeFlag = &mut sizeflag_6;
    let mut sizeflag_7: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_7_ref_0: &flags::size::SizeFlag = &mut sizeflag_7;
    let mut sizeflag_8: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_8_ref_0: &flags::size::SizeFlag = &mut sizeflag_8;
    let mut sizeflag_9: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_9_ref_0: &flags::size::SizeFlag = &mut sizeflag_9;
    let mut sizeflag_10: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_10_ref_0: &flags::size::SizeFlag = &mut sizeflag_10;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_10_ref_0);
    let mut tuple_1: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_9_ref_0);
    let mut tuple_2: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_8_ref_0);
    let mut tuple_3: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_7_ref_0);
    let mut tuple_4: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_6_ref_0);
    let mut tuple_5: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_5_ref_0);
    let mut tuple_6: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_4_ref_0);
    let mut tuple_7: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_3_ref_0);
    let mut tuple_8: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_2_ref_0);
    let mut tuple_9: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_1_ref_0);
    let mut tuple_10: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_639() {
//    rusty_monitor::set_test_id(639);
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_25: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_1_ref_0);
    let mut option_26: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_514() {
//    rusty_monitor::set_test_id(514);
    let mut str_0: &str = "pptx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = ".github";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "t";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "GRR";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "SizeValue";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "base_path";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "bin";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "other_write";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = ".bashrc";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "ZxpI";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_10_ref_0);
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_9_ref_0);
    let mut option_2: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_8_ref_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_7_ref_0);
    let mut option_4: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_6_ref_0);
    let mut option_5: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_5_ref_0);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_4_ref_0);
    let mut option_7: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_3_ref_0);
    let mut option_8: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_2_ref_0);
    let mut option_9: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_10: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1264() {
//    rusty_monitor::set_test_id(1264);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "base_path";
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut str_1: &str = "d";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "cxx";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = ".vimrc";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "user";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "toml";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
//    panic!("From RustyUnit with love");
}
}