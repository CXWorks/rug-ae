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
#[timeout(30000)]fn rusty_test_6299() {
//    rusty_monitor::set_test_id(6299);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_0: &str = "DB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "rss";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "xls";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Groups the directories at the top before the files. Same as --group-dirs=first";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "*";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "iml";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "Blocks";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "Blocks";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "sort";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "";
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
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_658() {
//    rusty_monitor::set_test_id(658);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_3_ref_0: &flags::date::DateFlag = &mut dateflag_3;
    let mut dateflag_4: flags::date::DateFlag = crate::flags::date::DateFlag::default();
    let mut dateflag_4_ref_0: &flags::date::DateFlag = &mut dateflag_4;
    let mut dateflag_5: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_5_ref_0: &flags::date::DateFlag = &mut dateflag_5;
    let mut dateflag_6: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_6_ref_0: &flags::date::DateFlag = &mut dateflag_6;
    let mut dateflag_7: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_7_ref_0: &flags::date::DateFlag = &mut dateflag_7;
    let mut dateflag_8: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_8_ref_0: &flags::date::DateFlag = &mut dateflag_8;
    let mut dateflag_9: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_9_ref_0: &flags::date::DateFlag = &mut dateflag_9;
    let mut dateflag_10: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_10_ref_0: &flags::date::DateFlag = &mut dateflag_10;
    let mut tuple_0: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_10_ref_0);
    let mut tuple_1: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_9_ref_0);
    let mut tuple_2: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_8_ref_0);
    let mut tuple_3: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_7_ref_0);
    let mut tuple_4: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_6_ref_0);
    let mut tuple_5: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_5_ref_0);
    let mut tuple_6: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_4_ref_0);
    let mut tuple_7: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_3_ref_0);
    let mut tuple_8: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_2_ref_0);
    let mut tuple_9: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_1_ref_0);
    let mut tuple_10: () = crate::flags::date::DateFlag::assert_receiver_is_total_eq(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2198() {
//    rusty_monitor::set_test_id(2198);
    let mut str_0: &str = "Date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "symlink_arrow";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 6usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
    let mut option_3: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_environment();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_526() {
//    rusty_monitor::set_test_id(526);
    let mut str_0: &str = "sticky";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Bytes";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "MsP";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "r";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "bashrc";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "flac";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "sort by WORD instead of name";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "avro";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "iQbE5PeY0";
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
#[timeout(30000)]fn rusty_test_1999() {
//    rusty_monitor::set_test_id(1999);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut bool_0: bool = false;
    let mut str_0: &str = "QC8XgO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut str_1: &str = "Relative";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_2: bool = true;
    let mut str_2: &str = "tree_edge";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_3: bool = true;
    let mut str_3: &str = "setgid";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_4: bool = false;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_5: bool = true;
    let mut str_5: &str = "vlc";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_6: bool = true;
    let mut str_6: &str = "Unicode";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bool_7: bool = false;
    let mut str_7: &str = "mi";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut bool_8: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2511() {
//    rusty_monitor::set_test_id(2511);
    let mut str_0: &str = "NoAccess";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 5u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut str_1: &str = "DjDRdhCWvRGIRV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "ttf";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "no_exec_no_uid";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "l";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "cpp";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "mov";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut option_0: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_1_ref_0);
    let mut option_1: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_str(str_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4627() {
//    rusty_monitor::set_test_id(4627);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "BlockDevice";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut str_1: &str = "CharDevice";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut u64_2: u64 = 1048576u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 0u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut bool_24: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1765() {
//    rusty_monitor::set_test_id(1765);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "BlockDevice";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut str_1: &str = "CharDevice";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut u64_2: u64 = 1048576u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_3: u64 = 1073741824u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut u64_4: u64 = 0u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut bool_24: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
    let mut option_2: std::option::Option<flags::date::DateFlag> = crate::flags::date::DateFlag::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_584() {
//    rusty_monitor::set_test_id(584);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "FileSmall";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 16u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7743() {
//    rusty_monitor::set_test_id(7743);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "FHZy8jKb7YOBc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_15: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut bool_16: bool = crate::flags::date::DateFlag::ne(dateflag_1_ref_0, dateflag_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2289() {
//    rusty_monitor::set_test_id(2289);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Pipe;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut bool_0: bool = crate::flags::date::DateFlag::eq(dateflag_1_ref_0, dateflag_0_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
//    panic!("From RustyUnit with love");
}
}