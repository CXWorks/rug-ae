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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1370() {
//    rusty_monitor::set_test_id(1370);
    let mut usize_0: usize = 8usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_1: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_3, reverse: option_2, dir_grouping: option_1};
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut tuple_0: () = crate::flags::size::SizeFlag::assert_receiver_is_total_eq(sizeflag_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7209() {
//    rusty_monitor::set_test_id(7209);
    let mut usize_0: usize = 360usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::clone(sizeflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_348() {
//    rusty_monitor::set_test_id(348);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut sizeflag_4: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_4_ref_0: &flags::size::SizeFlag = &mut sizeflag_4;
    let mut sizeflag_5: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_5_ref_0: &flags::size::SizeFlag = &mut sizeflag_5;
    let mut sizeflag_6: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_6_ref_0: &flags::size::SizeFlag = &mut sizeflag_6;
    let mut sizeflag_7: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_7_ref_0: &flags::size::SizeFlag = &mut sizeflag_7;
    let mut sizeflag_8: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_8_ref_0: &flags::size::SizeFlag = &mut sizeflag_8;
    let mut sizeflag_9: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_9_ref_0: &flags::size::SizeFlag = &mut sizeflag_9;
    let mut sizeflag_10: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_10_ref_0: &flags::size::SizeFlag = &mut sizeflag_10;
    let mut sizeflag_11: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_11_ref_0: &flags::size::SizeFlag = &mut sizeflag_11;
    let mut sizeflag_12: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut sizeflag_12_ref_0: &flags::size::SizeFlag = &mut sizeflag_12;
    let mut sizeflag_13: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
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
#[timeout(30000)]fn rusty_test_499() {
//    rusty_monitor::set_test_id(499);
    let mut str_0: &str = "NptU";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Reverse the order of the sort";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "scss";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "pdf";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "jar";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Recursion";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "2w27D66mpK";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "rmd";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "tree_edge";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "TG";
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
}