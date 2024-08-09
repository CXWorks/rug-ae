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
#[timeout(30000)]fn rusty_test_4068() {
//    rusty_monitor::set_test_id(4068);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Read;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Group;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_8: color::Elem = crate::color::Elem::Socket;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_9_ref_0);
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_9: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_10_ref_0);
    let mut theme_11: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_11_ref_0: &crate::color::theme::Theme = &mut theme_11;
    let mut elem_10: color::Elem = crate::color::Elem::Acl;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_11_ref_0);
    let mut theme_12: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_12_ref_0: &crate::color::theme::Theme = &mut theme_12;
    let mut elem_11: color::Elem = crate::color::Elem::Group;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_10: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_12_ref_0);
    let mut theme_13: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_13_ref_0: &crate::color::theme::Theme = &mut theme_13;
    let mut elem_12: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut color_11: crossterm::style::Color = crate::color::Elem::get_color(elem_12_ref_0, theme_13_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_11, uid_no_exec: color_10, exec_no_uid: color_9, no_exec_no_uid: color_8};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_config(config_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = std::option::Option::unwrap(option_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7283() {
//    rusty_monitor::set_test_id(7283);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut str_0: &str = "py";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "wma";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 73usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5111() {
//    rusty_monitor::set_test_id(5111);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "Stop recursing into directories after reaching specified depth";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8246() {
//    rusty_monitor::set_test_id(8246);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut str_0: &str = "NonFile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut bool_0: bool = crate::flags::size::SizeFlag::eq(sizeflag_1_ref_0, sizeflag_0_ref_0);
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5881() {
//    rusty_monitor::set_test_id(5881);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut str_0: &str = "NonFile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Name;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4790() {
//    rusty_monitor::set_test_id(4790);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "reverse";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7989() {
//    rusty_monitor::set_test_id(7989);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut str_0: &str = "NonFile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut bool_0: bool = crate::flags::size::SizeFlag::eq(sizeflag_0_ref_0, sizeflag_2_ref_0);
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2948() {
//    rusty_monitor::set_test_id(2948);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "reverse";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_845() {
//    rusty_monitor::set_test_id(845);
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "gradle";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_1_ref_0);
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_0: usize = 2usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_700() {
//    rusty_monitor::set_test_id(700);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_2_ref_0: &flags::size::SizeFlag = &mut sizeflag_2;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_3_ref_0: &flags::size::SizeFlag = &mut sizeflag_3;
    let mut sizeflag_4: flags::size::SizeFlag = crate::flags::size::SizeFlag::default();
    let mut sizeflag_4_ref_0: &flags::size::SizeFlag = &mut sizeflag_4;
    let mut sizeflag_5: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
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
#[timeout(30000)]fn rusty_test_4526() {
//    rusty_monitor::set_test_id(4526);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut str_0: &str = "gslides";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5054() {
//    rusty_monitor::set_test_id(5054);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut str_0: &str = "NonFile";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4499() {
//    rusty_monitor::set_test_id(4499);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut str_0: &str = "wpl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "long";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "so";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Enable classic mode (display output similar to ls)";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "dump";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "How to display date [possible values: date, relative, +date-time-format]";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "rb";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut str_8: &str = "NonFile";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_3_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2100() {
//    rusty_monitor::set_test_id(2100);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut str_0: &str = "t";
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut sizeflag_1_ref_0: &flags::size::SizeFlag = &mut sizeflag_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = crate::flags::size::SizeFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}