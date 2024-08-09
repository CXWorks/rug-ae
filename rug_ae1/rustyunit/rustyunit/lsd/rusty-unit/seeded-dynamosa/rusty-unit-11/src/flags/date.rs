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
#[timeout(30000)]fn rusty_test_1527() {
//    rusty_monitor::set_test_id(1527);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_9_ref_0: &flags::date::DateFlag = &mut dateflag_9;
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_10_ref_0: &flags::date::DateFlag = &mut dateflag_10;
    let mut dateflag_11: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_11_ref_0: &flags::date::DateFlag = &mut dateflag_11;
    let mut dateflag_12: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_12_ref_0: &flags::date::DateFlag = &mut dateflag_12;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_12_ref_0, dateflag_11_ref_0);
    let mut bool_1: bool = crate::flags::date::DateFlag::eq(dateflag_10_ref_0, dateflag_9_ref_0);
    let mut bool_2: bool = crate::flags::date::DateFlag::eq(dateflag_6_ref_0, dateflag_5_ref_0);
    let mut bool_3: bool = crate::flags::date::DateFlag::eq(dateflag_4_ref_0, dateflag_3_ref_0);
    let mut bool_4: bool = crate::flags::date::DateFlag::eq(dateflag_2_ref_0, dateflag_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2540() {
//    rusty_monitor::set_test_id(2540);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "ksh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8882() {
//    rusty_monitor::set_test_id(8882);
    let mut str_0: &str = "OA5AEY8m81emv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "markdown";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "0NlC";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "gulpfile.coffee";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "d";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "cxx";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = ".vimrc";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "toml";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "BDzWSTyL8BeDU";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_8_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_7_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_6_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_5_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1685() {
//    rusty_monitor::set_test_id(1685);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut bool_1: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5439() {
//    rusty_monitor::set_test_id(5439);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Enable classic mode (display output similar to ls)";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "JO4TGe";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "mustache";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "0x8t7qKMxP";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "gdoc";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "X7Qk0Hr";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "always";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "NoAccess";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "All";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_8_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_7_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_6_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_5_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_4_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_3_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_2_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_1_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2732() {
//    rusty_monitor::set_test_id(2732);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
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
    let mut option_30: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_358() {
//    rusty_monitor::set_test_id(358);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut str_0: &str = "csh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_4_ref_0);
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_3_ref_0);
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_2_ref_0);
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3001() {
//    rusty_monitor::set_test_id(3001);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_9_ref_0: &flags::date::DateFlag = &mut dateflag_9;
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_10_ref_0: &flags::date::DateFlag = &mut dateflag_10;
    let mut dateflag_11: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_11_ref_0: &flags::date::DateFlag = &mut dateflag_11;
    let mut dateflag_12: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_12_ref_0: &flags::date::DateFlag = &mut dateflag_12;
    let mut dateflag_13: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_13_ref_0: &flags::date::DateFlag = &mut dateflag_13;
    let mut dateflag_14: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_14_ref_0: &flags::date::DateFlag = &mut dateflag_14;
    let mut dateflag_15: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_15_ref_0: &flags::date::DateFlag = &mut dateflag_15;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_15_ref_0, dateflag_14_ref_0);
    let mut bool_1: bool = crate::flags::date::DateFlag::eq(dateflag_13_ref_0, dateflag_12_ref_0);
    let mut bool_2: bool = crate::flags::date::DateFlag::eq(dateflag_11_ref_0, dateflag_10_ref_0);
    let mut bool_3: bool = crate::flags::date::DateFlag::eq(dateflag_9_ref_0, dateflag_8_ref_0);
    let mut bool_4: bool = crate::flags::date::DateFlag::eq(dateflag_7_ref_0, dateflag_6_ref_0);
    let mut bool_5: bool = crate::flags::date::DateFlag::eq(dateflag_5_ref_0, dateflag_4_ref_0);
    let mut bool_6: bool = crate::flags::date::DateFlag::eq(dateflag_3_ref_0, dateflag_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut bool_7: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9128() {
//    rusty_monitor::set_test_id(9128);
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
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4412() {
//    rusty_monitor::set_test_id(4412);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut tuple_0: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}
}