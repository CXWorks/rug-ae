//! This module defines the [Recursion] options. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Recursion::configure_from) method.

use crate::config_file::Config;

use clap::{ArgMatches, Error, ErrorKind};

/// The options relating to recursion.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Recursion {
    /// Whether the recursion into directories is enabled.
    pub enabled: bool,
    /// The depth for how far to recurse into directories.
    pub depth: usize,
}

impl Recursion {
    /// Get the Recursion from either [ArgMatches], a [Config] or the [Default] value.
    ///
    /// The "enabled" value is determined by [enabled_from](Recursion::enabled_from) and the depth
    /// value is determined by [depth_from](Recursion::depth_from).
    ///
    /// # Errors
    ///
    /// If [depth_from](Recursion::depth_from) returns an [Error], this returns it.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        let enabled = Self::enabled_from(matches, config);
        let depth = Self::depth_from(matches, config)?;
        Ok(Self { enabled, depth })
    }

    /// Get the "enabled" boolean from [ArgMatches], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [enabled_from_arg_matches](Recursion::enabled_from_arg_matches)
    /// - [Config.recursion.enabled]
    /// - [Default::default]
    fn enabled_from(matches: &ArgMatches, config: &Config) -> bool {
        if let Some(value) = Self::enabled_from_arg_matches(matches) {
            return value;
        }
        if let Some(recursion) = &config.recursion {
            if let Some(enabled) = recursion.enabled {
                return enabled;
            }
        }

        Default::default()
    }

    /// Get a potential "enabled" boolean from [ArgMatches].
    ///
    /// If the "recursive" argument is passed, this returns `true` in a [Some]. Otherwise this
    /// returns [None].
    fn enabled_from_arg_matches(matches: &ArgMatches) -> Option<bool> {
        if matches.is_present("recursive") {
            Some(true)
        } else {
            None
        }
    }

    /// Get the "depth" integer from [ArgMatches], a [Config] or the [Default] value. The first
    /// value that is not [None] is used. The order of precedence for the value used is:
    /// - [depth_from_arg_matches](Recursion::depth_from_arg_matches)
    /// - [Config.recursion.depth]
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// If both configuration file and Args is error, this will return a Max-Uint value.
    ///
    /// # Errors
    ///
    /// If [depth_from_arg_matches](Recursion::depth_from_arg_matches) returns an [Error], this
    /// returns it.
    fn depth_from(matches: &ArgMatches, config: &Config) -> Result<usize, Error> {
        if let Some(value) = Self::depth_from_arg_matches(matches) {
            return value;
        }

        if let Some(recursion) = &config.recursion {
            if let Some(depth) = recursion.depth {
                return Ok(depth);
            }
        }

        Ok(usize::max_value())
    }

    /// Get a potential "depth" value from [ArgMatches].
    ///
    /// If the "depth" argument is passed, its parameter is evaluated. If it can be parsed into a
    /// [usize], the [Result] is returned in the [Some]. If it can not be parsed an [Error] is
    /// returned in the [Some]. If the argument has not been passed, a [None] is returned.
    ///
    /// # Errors
    ///
    /// If the parameter to the "depth" argument can not be parsed, this returns an [Error] in a
    /// [Some].
    fn depth_from_arg_matches(matches: &ArgMatches) -> Option<Result<usize, Error>> {
        let depth = match matches.values_of("depth") {
            Some(d) => d.last(),
            None => None,
        };
        if let Some(str) = depth {
            match str.parse::<usize>() {
                Ok(value) => return Some(Ok(value)),
                Err(_) => {
                    return Some(Err(Error::with_description(
                        "The argument '--depth' requires a valid positive number.",
                        ErrorKind::ValueValidation,
                    )))
                }
            }
        }
        None
    }
}

/// The default values for `Recursion` are the boolean default and [prim@usize::max_value()].
impl Default for Recursion {
    fn default() -> Self {
        Self {
            depth: usize::max_value(),
            enabled: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Recursion;

    use crate::app;
    use crate::config_file::{self, Config};

    use clap::ErrorKind;

    #[test]
    fn test_enabled_from_arg_matches_empty() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_arg_matches_true() {
        let argv = vec!["lsd", "--recursive"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(true), Recursion::enabled_from_arg_matches(&matches));
    }

    #[test]
    fn test_enabled_from_empty_matches_and_config() {
        let argv = vec!["lsd"];
        assert_eq!(
            false,
            Recursion::enabled_from(
                &app::build().get_matches_from_safe(argv).unwrap(),
                &Config::with_none()
            )
        );
    }

    #[test]
    fn test_enabled_from_matches_empty_and_config_true() {
        let argv = vec!["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: Some(true),
            depth: None,
        });
        assert_eq!(
            true,
            Recursion::enabled_from(&app::build().get_matches_from_safe(argv).unwrap(), &c)
        );
    }

    #[test]
    fn test_enabled_from_matches_empty_and_config_false() {
        let argv = vec!["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: Some(false),
            depth: None,
        });
        assert_eq!(
            false,
            Recursion::enabled_from(&app::build().get_matches_from_safe(argv).unwrap(), &c)
        );
    }

    // The following depth_from_arg_matches tests are implemented using match expressions instead
    // of the assert_eq macro, because clap::Error does not implement PartialEq.

    #[test]
    fn test_depth_from_arg_matches_empty() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_depth_from_arg_matches_integer() {
        let argv = vec!["lsd", "--depth", "42"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(value) => value == 42,
                    Err(_) => false,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_arg_matches_depth_multi() {
        let argv = vec!["lsd", "--depth", "4", "--depth", "2"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(value) => value == 2,
                    Err(_) => false,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_arg_matches_neg_int() {
        let argv = vec!["lsd", "--depth", "\\-42"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(_) => false,
                    Err(error) => error.kind == ErrorKind::ValueValidation,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_arg_matches_non_int() {
        let argv = vec!["lsd", "--depth", "foo"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match Recursion::depth_from_arg_matches(&matches) {
            None => false,
            Some(result) => {
                match result {
                    Ok(_) => false,
                    Err(error) => error.kind == ErrorKind::ValueValidation,
                }
            }
        });
    }

    #[test]
    fn test_depth_from_config_none_max() {
        let argv = vec!["lsd"];
        assert_eq!(
            usize::max_value(),
            Recursion::depth_from(
                &app::build().get_matches_from_safe(argv).unwrap(),
                &Config::with_none()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_depth_from_config_pos_integer() {
        let argv = vec!["lsd"];
        let mut c = Config::with_none();
        c.recursion = Some(config_file::Recursion {
            enabled: None,
            depth: Some(42),
        });
        assert_eq!(
            42,
            Recursion::depth_from(&app::build().get_matches_from_safe(argv).unwrap(), &c).unwrap()
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6109() {
    rusty_monitor::set_test_id(6109);
    let mut u64_0: u64 = 5849u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_0: usize = 6120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut str_0: &str = "JI";
    let mut u64_1: u64 = 9212u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut u64_2: u64 = 8974u64;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut str_1: &str = "y";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_1: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut str_2: &str = "1khP";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "EjMVv";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 5309usize;
    let mut tuple_0: (usize, &str) = (usize_1, str_3_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut u64_3: u64 = 8555u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut u64_4: u64 = 7118u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_2: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_2};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut u64_5: u64 = 1146u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut size_5: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<usize> = std::option::Option::None;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    crate::meta::filetype::FileType::render(filetype_2, colors_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut bool_3: bool = crate::flags::recursion::Recursion::ne(recursion_1_ref_0, recursion_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1268() {
    rusty_monitor::set_test_id(1268);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 937usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut usize_1: usize = 8231usize;
    let mut bool_2: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_2: usize = 5348usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_3: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::clone(recursion_0_ref_0);
    panic!("From RustyUnit with love");
}
}