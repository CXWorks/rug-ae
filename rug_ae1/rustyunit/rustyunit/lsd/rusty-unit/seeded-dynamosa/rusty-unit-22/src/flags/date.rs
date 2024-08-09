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
#[timeout(30000)]fn rusty_test_488() {
//    rusty_monitor::set_test_id(488);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_1: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_2: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_3: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_4: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_5: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_6: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_7: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_8: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_9: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_10: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_11: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_12: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_13: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_14: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_15: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_16: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_17: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_18: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_19: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_20: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_21: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_22: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_23: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_24: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_25: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_26: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_27: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_28: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut contentstyle_29: crossterm::style::ContentStyle = crate::color::Colors::default_style();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7562() {
//    rusty_monitor::set_test_id(7562);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 97u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut str_0: &str = "mkd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_2_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5929() {
//    rusty_monitor::set_test_id(5929);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "medium";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8292() {
//    rusty_monitor::set_test_id(8292);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "medium";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_12: bool = crate::flags::date::DateFlag::eq(dateflag_7_ref_0, dateflag_6_ref_0);
    let mut bool_13: bool = crate::flags::date::DateFlag::eq(dateflag_5_ref_0, dateflag_4_ref_0);
    let mut bool_14: bool = crate::flags::date::DateFlag::eq(dateflag_3_ref_0, dateflag_2_ref_0);
    let mut bool_15: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_521() {
//    rusty_monitor::set_test_id(521);
    let mut str_0: &str = "Iso";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "GP2b30wuTJS";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "gslides";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "ogg";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "user_read";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Default";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "no_uid";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "tar";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "scss";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_10_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_9_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_8_ref_0);
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_7_ref_0);
    let mut option_4: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_6_ref_0);
    let mut option_5: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_5_ref_0);
    let mut option_6: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
    let mut option_7: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_3_ref_0);
    let mut option_8: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_2_ref_0);
    let mut option_9: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_10: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1336() {
//    rusty_monitor::set_test_id(1336);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut tuple_0: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_0_ref_0);
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4115() {
//    rusty_monitor::set_test_id(4115);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "medium";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_12: bool = crate::flags::date::DateFlag::eq(dateflag_7_ref_0, dateflag_6_ref_0);
    let mut bool_13: bool = crate::flags::date::DateFlag::eq(dateflag_5_ref_0, dateflag_4_ref_0);
    let mut bool_14: bool = crate::flags::date::DateFlag::eq(dateflag_3_ref_0, dateflag_2_ref_0);
    let mut bool_15: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6655() {
//    rusty_monitor::set_test_id(6655);
    let mut str_0: &str = "fsi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 55u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_format_string(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1734() {
//    rusty_monitor::set_test_id(1734);
    let mut str_0: &str = "apk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "7afxcA1pWO";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "epub";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Symlink";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Recursion";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "erb";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Whether to use fancy or unicode icons";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "vunTduv";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "short";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "Permissions";
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
#[timeout(30000)]fn rusty_test_3494() {
//    rusty_monitor::set_test_id(3494);
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
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
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_705() {
//    rusty_monitor::set_test_id(705);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_7_ref_0);
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_6_ref_0);
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_5_ref_0);
    let mut dateflag_11: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_4_ref_0);
    let mut dateflag_12: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_3_ref_0);
    let mut dateflag_13: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_2_ref_0);
    let mut dateflag_14: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut dateflag_15: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8012() {
//    rusty_monitor::set_test_id(8012);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "medium";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1270() {
//    rusty_monitor::set_test_id(1270);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_0: bool = false;
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
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut option_30: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}