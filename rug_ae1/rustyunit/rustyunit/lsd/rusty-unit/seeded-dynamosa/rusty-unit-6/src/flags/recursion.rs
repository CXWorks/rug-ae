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
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5773() {
//    rusty_monitor::set_test_id(5773);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_100() {
//    rusty_monitor::set_test_id(100);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut usize_0: usize = 40usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut bool_2: bool = crate::flags::recursion::Recursion::ne(recursion_1_ref_0, recursion_0_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_414() {
//    rusty_monitor::set_test_id(414);
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut usize_1: usize = 2usize;
    let mut bool_1: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut recursion_2_ref_0: &crate::flags::recursion::Recursion = &mut recursion_2;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_3_ref_0: &crate::flags::recursion::Recursion = &mut recursion_3;
    let mut usize_2: usize = 360usize;
    let mut bool_2: bool = true;
    let mut recursion_4: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut recursion_4_ref_0: &crate::flags::recursion::Recursion = &mut recursion_4;
    let mut recursion_5: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_5_ref_0: &crate::flags::recursion::Recursion = &mut recursion_5;
    let mut usize_3: usize = 80usize;
    let mut bool_3: bool = true;
    let mut recursion_6: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut recursion_6_ref_0: &crate::flags::recursion::Recursion = &mut recursion_6;
    let mut recursion_7: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_7_ref_0: &crate::flags::recursion::Recursion = &mut recursion_7;
    let mut usize_4: usize = 2usize;
    let mut bool_4: bool = false;
    let mut recursion_8: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_4};
    let mut recursion_8_ref_0: &crate::flags::recursion::Recursion = &mut recursion_8;
    let mut recursion_9: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_9_ref_0: &crate::flags::recursion::Recursion = &mut recursion_9;
    let mut bool_5: bool = crate::flags::recursion::Recursion::ne(recursion_9_ref_0, recursion_8_ref_0);
    let mut bool_6: bool = crate::flags::recursion::Recursion::ne(recursion_7_ref_0, recursion_6_ref_0);
    let mut bool_7: bool = crate::flags::recursion::Recursion::ne(recursion_5_ref_0, recursion_4_ref_0);
    let mut bool_8: bool = crate::flags::recursion::Recursion::ne(recursion_3_ref_0, recursion_2_ref_0);
    let mut bool_9: bool = crate::flags::recursion::Recursion::ne(recursion_1_ref_0, recursion_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_305() {
//    rusty_monitor::set_test_id(305);
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut usize_0: usize = 0usize;
    let mut bool_0: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_2_ref_0: &crate::flags::recursion::Recursion = &mut recursion_2;
    let mut usize_1: usize = 360usize;
    let mut bool_1: bool = false;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut recursion_3_ref_0: &crate::flags::recursion::Recursion = &mut recursion_3;
    let mut recursion_4: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_4_ref_0: &crate::flags::recursion::Recursion = &mut recursion_4;
    let mut usize_2: usize = 2usize;
    let mut bool_2: bool = true;
    let mut recursion_5: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut recursion_5_ref_0: &crate::flags::recursion::Recursion = &mut recursion_5;
    let mut usize_3: usize = 0usize;
    let mut bool_3: bool = false;
    let mut recursion_6: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut recursion_6_ref_0: &crate::flags::recursion::Recursion = &mut recursion_6;
    let mut recursion_7: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_7_ref_0: &crate::flags::recursion::Recursion = &mut recursion_7;
    let mut tuple_0: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_7_ref_0);
    let mut tuple_1: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_6_ref_0);
    let mut tuple_2: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_5_ref_0);
    let mut tuple_3: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_4_ref_0);
    let mut tuple_4: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_3_ref_0);
    let mut tuple_5: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_2_ref_0);
    let mut tuple_6: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_1_ref_0);
    let mut tuple_7: () = crate::flags::recursion::Recursion::assert_receiver_is_total_eq(recursion_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_291() {
//    rusty_monitor::set_test_id(291);
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut recursion_1_ref_0: &crate::flags::recursion::Recursion = &mut recursion_1;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_2_ref_0: &crate::flags::recursion::Recursion = &mut recursion_2;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_3_ref_0: &crate::flags::recursion::Recursion = &mut recursion_3;
    let mut usize_1: usize = 0usize;
    let mut bool_1: bool = true;
    let mut recursion_4: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut recursion_4_ref_0: &crate::flags::recursion::Recursion = &mut recursion_4;
    let mut usize_2: usize = 2usize;
    let mut bool_2: bool = false;
    let mut recursion_5: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut recursion_5_ref_0: &crate::flags::recursion::Recursion = &mut recursion_5;
    let mut usize_3: usize = 1usize;
    let mut bool_3: bool = false;
    let mut recursion_6: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut recursion_6_ref_0: &crate::flags::recursion::Recursion = &mut recursion_6;
    let mut usize_4: usize = 91usize;
    let mut bool_4: bool = false;
    let mut recursion_7: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_4};
    let mut recursion_7_ref_0: &crate::flags::recursion::Recursion = &mut recursion_7;
    let mut usize_5: usize = 6usize;
    let mut bool_5: bool = false;
    let mut recursion_8: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_5};
    let mut recursion_8_ref_0: &crate::flags::recursion::Recursion = &mut recursion_8;
    let mut recursion_9: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion::default();
    let mut recursion_9_ref_0: &crate::flags::recursion::Recursion = &mut recursion_9;
    let mut bool_6: bool = crate::flags::recursion::Recursion::eq(recursion_9_ref_0, recursion_8_ref_0);
    let mut bool_7: bool = crate::flags::recursion::Recursion::eq(recursion_7_ref_0, recursion_6_ref_0);
    let mut bool_8: bool = crate::flags::recursion::Recursion::eq(recursion_5_ref_0, recursion_4_ref_0);
    let mut bool_9: bool = crate::flags::recursion::Recursion::eq(recursion_3_ref_0, recursion_2_ref_0);
    let mut bool_10: bool = crate::flags::recursion::Recursion::eq(recursion_1_ref_0, recursion_0_ref_0);
//    panic!("From RustyUnit with love");
}
}