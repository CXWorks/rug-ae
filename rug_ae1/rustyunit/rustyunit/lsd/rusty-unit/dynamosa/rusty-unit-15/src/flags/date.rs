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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7925() {
    rusty_monitor::set_test_id(7925);
    let mut str_0: &str = "M3dL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut usize_0: usize = 50usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_2: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_12, theme: option_11, separator: option_10};
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_3: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut option_22: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_23: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_26: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_27: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_28: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_29: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_30: std::option::Option<bool> = std::option::Option::None;
    let mut option_31: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_32: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut option_33: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut option_35: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_36: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_37: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_38: std::option::Option<bool> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6728() {
    rusty_monitor::set_test_id(6728);
    let mut str_0: &str = "M3dL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 50usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_2: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_3: bool = true;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut option_20: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_24: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_25: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_2);
    let mut option_26: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_27: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut option_29: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_30: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut option_31: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_32: std::option::Option<bool> = std::option::Option::None;
    let mut option_33: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_34: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_36: std::option::Option<bool> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_1_ref_0);
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_2_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4019() {
    rusty_monitor::set_test_id(4019);
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_0: &str = "XKSgJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "cRnclL3gMk";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 37usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_2_ref_0);
    let mut bool_0: bool = true;
    let mut usize_1: usize = 92usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 72usize;
    let mut bool_2: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut u64_0: u64 = 22u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7057() {
    rusty_monitor::set_test_id(7057);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 24usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut bool_2: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_21: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_23: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut bool_4: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_23, reverse: option_22, dir_grouping: option_21};
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::clone(dateflag_0_ref_0);
    panic!("From RustyUnit with love");
}
}