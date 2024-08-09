//! This module defines the [DateFlag]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::app;
use crate::config_file::Config;
use crate::print_error;

use clap::ArgMatches;

/// The flag showing which kind of time stamps to display.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFlag {
    Date,
    Relative,
    Iso,
    Formatted(String),
}

impl DateFlag {
    /// Get a value from a date format string
    fn from_format_string(value: &str) -> Option<Self> {
        match app::validate_time_format(value) {
            Ok(()) => Some(Self::Formatted(value[1..].to_string())),
            _ => {
                print_error!("Not a valid date format: {}.", value);
                None
            }
        }
    }

    /// Get a value from a str.
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "date" => Some(Self::Date),
            "relative" => Some(Self::Relative),
            _ if value.starts_with('+') => Self::from_format_string(value),
            _ => {
                print_error!("Not a valid date value: {}.", value);
                None
            }
        }
    }
}

impl Configurable<Self> for DateFlag {
    /// Get a potential `DateFlag` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [DateFlag::Date] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Date)
        } else if matches.occurrences_of("date") > 0 {
            match matches.values_of("date")?.last() {
                Some("date") => Some(Self::Date),
                Some("relative") => Some(Self::Relative),
                Some(format) if format.starts_with('+') => {
                    Some(Self::Formatted(format[1..].to_owned()))
                }
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `DateFlag` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(DateFlag::Date),
    /// Otherwise if the `Config::date` has value and is one of "date" or "relative",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = &config.classic {
            return Some(Self::Date);
        }

        if let Some(date) = &config.date {
            Self::from_str(date)
        } else {
            None
        }
    }

    /// Get a potential `DateFlag` variant from the environment.
    fn from_environment() -> Option<Self> {
        if let Ok(value) = std::env::var("TIME_STYLE") {
            match value.as_str() {
                "full-iso" => Some(Self::Formatted("%F %T.%f %z".into())),
                "long-iso" => Some(Self::Formatted("%F %R".into())),
                "iso" => Some(Self::Iso),
                _ if value.starts_with('+') => Self::from_format_string(&value),
                _ => {
                    print_error!("Not a valid date value: {}.", value);
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `DateFlag` is [DateFlag::Date].
impl Default for DateFlag {
    fn default() -> Self {
        Self::Date
    }
}

#[cfg(test)]
mod test {
    use super::DateFlag;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_date() {
        let argv = vec!["lsd", "--date", "date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_relative() {
        let argv = vec!["lsd", "--date", "relative"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Relative),
            DateFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_format() {
        let argv = vec!["lsd", "--date", "+%F"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_arg_matches(&matches)
        );
    }

    #[test]
    #[should_panic(expected = "invalid format specifier: %J")]
    fn test_from_arg_matches_format_invalid() {
        let argv = vec!["lsd", "--date", "+%J"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        DateFlag::from_arg_matches(&matches);
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--date", "date", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_date_multi() {
        let argv = vec!["lsd", "--date", "relative", "--date", "date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, DateFlag::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_date() {
        let mut c = Config::with_none();
        c.date = Some("date".into());

        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_relative() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        assert_eq!(Some(DateFlag::Relative), DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_format() {
        let mut c = Config::with_none();
        c.date = Some("+%F".into());
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())),
            DateFlag::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_format_invalid() {
        let mut c = Config::with_none();
        c.date = Some("+%J".into());
        assert_eq!(None, DateFlag::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        c.classic = Some(true);
        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(&c));
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_none() {
        std::env::set_var("TIME_STYLE", "");
        assert_eq!(None, DateFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_full_iso() {
        std::env::set_var("TIME_STYLE", "full-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %T.%f %z".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_long_iso() {
        std::env::set_var("TIME_STYLE", "long-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %R".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_iso() {
        std::env::set_var("TIME_STYLE", "iso");
        assert_eq!(Some(DateFlag::Iso), DateFlag::from_environment());
    }

    #[test]
    #[serial_test::serial]
    fn test_from_environment_format() {
        std::env::set_var("TIME_STYLE", "+%F");
        assert_eq!(
            Some(DateFlag::Formatted("%F".into())),
            DateFlag::from_environment()
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_arg() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = vec!["lsd", "--date", "+%F"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%F".into()),
            DateFlag::configure_from(&matches, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_env() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%R".into()),
            DateFlag::configure_from(&matches, &config)
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_parsing_order_config() {
        std::env::set_var("TIME_STYLE", "");
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%c".into()),
            DateFlag::configure_from(&matches, &config)
        );
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
#[timeout(30000)]fn rusty_test_2823() {
//    rusty_monitor::set_test_id(2823);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut str_0: &str = "";
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut str_1: &str = "b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut bool_0: bool = crate::flags::date::DateFlag::ne(dateflag_8_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6212() {
//    rusty_monitor::set_test_id(6212);
    let mut str_0: &str = "TgbK5eQ4";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "is_dir";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "7OuQ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "+";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "rs";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "4";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "opus";
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut str_9: &str = "c";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_8_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_9_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_7_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_6_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_5_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_2_ref_0);
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_10: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2022() {
//    rusty_monitor::set_test_id(2022);
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_9_ref_0: &flags::date::DateFlag = &mut dateflag_9;
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_10_ref_0: &flags::date::DateFlag = &mut dateflag_10;
    let mut dateflag_11: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_11_ref_0: &flags::date::DateFlag = &mut dateflag_11;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut dateflag_12: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_12_ref_0: &flags::date::DateFlag = &mut dateflag_12;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_11_ref_0, dateflag_10_ref_0);
    let mut bool_1: bool = crate::flags::date::DateFlag::eq(dateflag_9_ref_0, dateflag_8_ref_0);
    let mut bool_2: bool = crate::flags::date::DateFlag::eq(dateflag_7_ref_0, dateflag_6_ref_0);
    let mut bool_3: bool = crate::flags::date::DateFlag::eq(dateflag_5_ref_0, dateflag_4_ref_0);
    let mut bool_4: bool = crate::flags::date::DateFlag::eq(dateflag_1_ref_0, dateflag_0_ref_0);
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut tuple_0: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_3_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3488() {
//    rusty_monitor::set_test_id(3488);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_0: &str = "config-file";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "iPino7a5TfgQ9p";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "d";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "group";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "scpt";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "depth";
    let mut str_6: &str = "Relative";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "aIRp7";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_8: &str = "c";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_7_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_8_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_6_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_5_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_2_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7679() {
//    rusty_monitor::set_test_id(7679);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut elem_1: color::Elem = crate::color::Elem::Context;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2149() {
//    rusty_monitor::set_test_id(2149);
    let mut str_0: &str = "date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "order";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "directory-only";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_3: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_3, theme: option_2};
    let mut option_4: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_2_ref_0);
    let mut option_10: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_11: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6989() {
//    rusty_monitor::set_test_id(6989);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut str_0: &str = "XplNNi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 80usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6096() {
//    rusty_monitor::set_test_id(6096);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 80usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 360usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_0: &str = "PYhgadLWq1p";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_2: u64 = 1073741824u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut vec_0: std::vec::Vec<std::path::PathBuf> = std::vec::Vec::new();
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut bool_3: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2794() {
//    rusty_monitor::set_test_id(2794);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_9_ref_0: &flags::date::DateFlag = &mut dateflag_9;
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_10_ref_0: &flags::date::DateFlag = &mut dateflag_10;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_9_ref_0, dateflag_8_ref_0);
    let mut bool_1: bool = crate::flags::date::DateFlag::eq(dateflag_5_ref_0, dateflag_4_ref_0);
    let mut bool_2: bool = crate::flags::date::DateFlag::eq(dateflag_3_ref_0, dateflag_2_ref_0);
    let mut bool_3: bool = crate::flags::date::DateFlag::eq(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5257() {
//    rusty_monitor::set_test_id(5257);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut str_0: &str = "🗋";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "stylus";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "GRTB7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "HourOld";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "depth";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "aIRp7";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "c";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "ai";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_8_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_7_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_6_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_5_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_2_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
    let mut dateflag_2: flags::date::DateFlag = std::option::Option::unwrap(option_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_218() {
//    rusty_monitor::set_test_id(218);
    let mut str_0: &str = "pls";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "directory-only";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "zg3pLW2MXdaEi";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "hxx";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "mustache";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "mjs";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "%F %R";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "css";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "BkuDa8H1IA";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_10_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_9_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_8_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_7_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_6_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_5_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_4_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_3_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_2_ref_0);
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_1_ref_0);
    let mut option_10: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_348() {
//    rusty_monitor::set_test_id(348);
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_10: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_11: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_12: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_13: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_14: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_15: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_16: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_17: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_18: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_19: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_20: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_21: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_22: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_23: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_24: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_25: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_26: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_27: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_28: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_29: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2076() {
//    rusty_monitor::set_test_id(2076);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "Meta";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7400() {
//    rusty_monitor::set_test_id(7400);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 18usize;
    let mut option_10: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_25: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut option_26: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_1_ref_0);
    let mut option_27: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_2_ref_0);
    let mut option_28: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}