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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_135() {
    rusty_monitor::set_test_id(135);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_4: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_8: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_10, reverse: option_9, dir_grouping: option_8};
    let mut option_11: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_12: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_15: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_19: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_1: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_22: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut option_23: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_23, theme: option_22};
    let mut option_24: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 15u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "LTUmcOc2IMdy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut option_27: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_28: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_29: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_1: &str = "bG0fiFhlyQA";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::clone(recursion_0_ref_0);
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::clone(recursion_1_ref_0);
    let mut option_30: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut option_31: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut bool_4: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_29, reverse: option_28, dir_grouping: option_27};
    panic!("From RustyUnit with love");
}
}