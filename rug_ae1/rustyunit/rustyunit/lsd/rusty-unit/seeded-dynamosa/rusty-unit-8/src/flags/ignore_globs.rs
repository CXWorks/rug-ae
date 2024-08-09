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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8096() {
//    rusty_monitor::set_test_id(8096);
    let mut str_0: &str = "Display the index number of each file";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 34u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_2: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_1: &str = "Iso";
    let mut str_2: &str = "heix";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "pipe";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_4: &str = "..";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "w";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "gsheet";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_7: &str = "Qv1lH9dPQ";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "x1";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "m3u";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "Formatted";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "iSU";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_11_ref_0);
    let mut result_1: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_10_ref_0);
    let mut result_2: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_9_ref_0);
    let mut result_3: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_8_ref_0);
    let mut result_4: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_7_ref_0);
    let mut result_5: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_3_ref_0);
    let mut result_6: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_6_ref_0);
    let mut result_7: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_5_ref_0);
    let mut result_8: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_4_ref_0);
    let mut result_9: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_1_ref_0);
    let mut result_10: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_2_ref_0);
    let mut option_25: std::option::Option<std::result::Result<globset::GlobSet, clap::Error>> = crate::flags::ignore_globs::IgnoreGlobs::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5975() {
//    rusty_monitor::set_test_id(5975);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "woff2";
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut str_1: &str = "pipe";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "..";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_3: &str = "gsheet";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "w";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "x1";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "ep3x";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "Formatted";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "iSU";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_8_ref_0);
    let mut glob_0: globset::Glob = std::result::Result::unwrap(result_0);
//    panic!("From RustyUnit with love");
}
}