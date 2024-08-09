//! This module defines the [IgnoreGlobs]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](IgnoreGlobs::configure_from) method.

use crate::config_file::Config;

use clap::{ArgMatches, Error, ErrorKind};
use globset::{Glob, GlobSet, GlobSetBuilder};

/// The struct holding a [GlobSet] and methods to build it.
#[derive(Clone, Debug)]
pub struct IgnoreGlobs(pub GlobSet);

impl IgnoreGlobs {
    /// Returns a value from either [ArgMatches], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_arg_matches](IgnoreGlobs::from_arg_matches)
    /// - [from_config](IgnoreGlobs::from_config)
    /// - [Default::default]
    ///
    /// # Errors
    ///
    /// If either of the [Glob::new] or [GlobSetBuilder.build] methods return an [Err].
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        let mut result: Result<Self, Error> = Ok(Default::default());

        if !matches.is_present("ignore-config") {
            if let Some(value) = Self::from_config(config) {
                match value {
                    Ok(glob_set) => result = Ok(Self(glob_set)),
                    Err(err) => result = Err(err),
                }
            }
        }

        if let Some(value) = Self::from_arg_matches(matches) {
            match value {
                Ok(glob_set) => result = Ok(Self(glob_set)),
                Err(err) => result = Err(err),
            }
        }

        result
    }

    /// Get a potential [GlobSet] from [ArgMatches].
    ///
    /// If the "ignore-glob" argument has been passed, this returns a [Result] in a [Some] with
    /// either the built [GlobSet] or an [Error], if any error was encountered while creating the
    /// [GlobSet]. If the argument has not been passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Result<GlobSet, Error>> {
        if matches.occurrences_of("ignore-glob") > 0 {
            if let Some(values) = matches.values_of("ignore-glob") {
                let mut glob_set_builder = GlobSetBuilder::new();
                for value in values {
                    match Self::create_glob(value) {
                        Ok(glob) => {
                            glob_set_builder.add(glob);
                        }
                        Err(err) => return Some(Err(err)),
                    }
                }
                Some(Self::create_glob_set(&glob_set_builder))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get a potential [GlobSet] from a [Config].
    ///
    /// If the `Config::ignore-globs` contains an Array of Strings,
    /// each of its values is used to build the [GlobSet]. If the building
    /// succeeds, the [GlobSet] is returned in the [Result] in a [Some]. If any error is
    /// encountered while building, an [Error] is returned in the Result instead. If the Config does
    /// not contain such a key, this returns [None].
    fn from_config(config: &Config) -> Option<Result<GlobSet, Error>> {
        if let Some(globs) = &config.ignore_globs {
            let mut glob_set_builder = GlobSetBuilder::new();
            for glob in globs.iter() {
                match Self::create_glob(glob) {
                    Ok(glob) => {
                        glob_set_builder.add(glob);
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
            Some(Self::create_glob_set(&glob_set_builder))
        } else {
            None
        }
    }

    /// Create a [Glob] from a provided pattern.
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob(pattern: &str) -> Result<Glob, Error> {
        match Glob::new(pattern) {
            Ok(glob) => Ok(glob),
            Err(err) => Err(Error::with_description(
                &err.to_string(),
                ErrorKind::ValueValidation,
            )),
        }
    }

    /// Create a [GlobSet] from a provided [GlobSetBuilder].
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob_set(builder: &GlobSetBuilder) -> Result<GlobSet, Error> {
        match builder.build() {
            Ok(glob_set) => Ok(glob_set),
            Err(err) => Err(Error::with_description(
                &err.to_string(),
                ErrorKind::ValueValidation,
            )),
        }
    }
}

/// The default value of `IgnoreGlobs` is the empty [GlobSet], returned by [GlobSet::empty()].
impl Default for IgnoreGlobs {
    fn default() -> Self {
        Self(GlobSet::empty())
    }
}

#[cfg(test)]
mod test {
    use super::IgnoreGlobs;

    use crate::app;
    use crate::config_file::Config;

    // The following tests are implemented using match expressions instead of the assert_eq macro,
    // because clap::Error does not implement PartialEq.
    //
    // Further no tests for actually returned GlobSets are implemented, because GlobSet does not
    // even implement PartialEq and thus can not be easily compared.

    #[test]
    fn test_configuration_from_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(
            match IgnoreGlobs::configure_from(&matches, &Config::with_none()) {
                Ok(_) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = vec!["lsd", "--ignore-glob", ".git"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(
            match IgnoreGlobs::configure_from(&matches, &Config::with_none()) {
                Ok(_) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut c = Config::with_none();
        c.ignore_globs = Some(vec![".git".into()].into());
        assert!(match IgnoreGlobs::configure_from(&matches, &c) {
            Ok(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(match IgnoreGlobs::from_arg_matches(&matches) {
            None => true,
            _ => false,
        });
    }

    #[test]
    fn test_from_config_none() {
        assert!(match IgnoreGlobs::from_config(&Config::with_none()) {
            None => true,
            _ => false,
        });
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_421() {
//    rusty_monitor::set_test_id(421);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_14: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_14};
    let mut bool_15: bool = false;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_15};
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_16: bool = crate::meta::filetype::FileType::is_dirlike(filetype_5);
    let mut bool_17: bool = crate::meta::filetype::FileType::is_dirlike(filetype_4);
    let mut bool_18: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    let mut bool_19: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut bool_20: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut bool_21: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_433() {
//    rusty_monitor::set_test_id(433);
    let mut ignoreglobs_0: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_0_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_0;
    let mut ignoreglobs_1: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_1_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_1;
    let mut ignoreglobs_2: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_2_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_2;
    let mut ignoreglobs_3: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_3_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_3;
    let mut ignoreglobs_4: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_4_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_4;
    let mut ignoreglobs_5: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_5_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_5;
    let mut ignoreglobs_6: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_6_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_6;
    let mut ignoreglobs_7: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_7_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_7;
    let mut ignoreglobs_8: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_8_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_8;
    let mut ignoreglobs_9: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_9_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_9;
    let mut ignoreglobs_10: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut ignoreglobs_10_ref_0: &crate::flags::ignore_globs::IgnoreGlobs = &mut ignoreglobs_10;
    let mut ignoreglobs_11: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_10_ref_0);
    let mut ignoreglobs_12: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_9_ref_0);
    let mut ignoreglobs_13: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_8_ref_0);
    let mut ignoreglobs_14: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_7_ref_0);
    let mut ignoreglobs_15: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_6_ref_0);
    let mut ignoreglobs_16: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_5_ref_0);
    let mut ignoreglobs_17: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_4_ref_0);
    let mut ignoreglobs_18: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_3_ref_0);
    let mut ignoreglobs_19: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_2_ref_0);
    let mut ignoreglobs_20: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_1_ref_0);
    let mut ignoreglobs_21: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::clone(ignoreglobs_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5994() {
//    rusty_monitor::set_test_id(5994);
    let mut str_0: &str = "broken";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_1: &str = "gif";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "rspec_status";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "enabled";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "rE68588PYV4V";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "classify";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "T3t";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "include";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "gulpfile.coffee";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_8_ref_0);
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6142() {
//    rusty_monitor::set_test_id(6142);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<std::result::Result<globset::GlobSet, clap::Error>> = crate::flags::ignore_globs::IgnoreGlobs::from_config(config_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_7: color::Elem = crate::color::Elem::Socket;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6352() {
//    rusty_monitor::set_test_id(6352);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<std::result::Result<globset::GlobSet, clap::Error>> = crate::flags::ignore_globs::IgnoreGlobs::from_config(config_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_7: color::Elem = crate::color::Elem::Socket;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
//    panic!("From RustyUnit with love");
}
}