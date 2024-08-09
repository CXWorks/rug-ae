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
#[timeout(30000)]fn rusty_test_8621() {
//    rusty_monitor::set_test_id(8621);
    let mut str_0: &str = "none";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "rDuQCs";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "oneline";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Extension";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Append indicator (one of */=>@|) at the end of the file names";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "d9QtEKq";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "CEyBB8";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_8_ref_0);
    let mut result_1: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_7_ref_0);
    let mut result_2: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_1_ref_0);
    let mut result_3: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_6_ref_0);
    let mut result_4: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_5_ref_0);
    let mut result_5: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_4_ref_0);
    let mut result_6: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_3_ref_0);
    let mut result_7: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_2_ref_0);
    let mut result_8: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3584() {
//    rusty_monitor::set_test_id(3584);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_0: &str = "jpeg";
    let mut str_1: &str = "How to display size";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "opp";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "pi";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "How to display permissions";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "readme";
    let mut str_6: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "Extension";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_0_ref_0);
    let mut result_1: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_5_ref_0);
    let mut result_2: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_7_ref_0);
    let mut result_3: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_6_ref_0);
    let mut result_4: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_8_ref_0);
    let mut result_5: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_4_ref_0);
    let mut result_6: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_3_ref_0);
    let mut result_7: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_2_ref_0);
    let mut result_8: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_1_ref_0);
    let mut glob_0: globset::Glob = std::result::Result::unwrap(result_6);
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_345() {
//    rusty_monitor::set_test_id(345);
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
#[timeout(30000)]fn rusty_test_6830() {
//    rusty_monitor::set_test_id(6830);
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut ignoreglobs_0: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 8usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut ignoreglobs_1: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 360usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut ignoreglobs_2: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_3);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "display_indicators";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Indicator";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_3: usize = 77usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_3);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<std::result::Result<globset::GlobSet, clap::Error>> = crate::flags::ignore_globs::IgnoreGlobs::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}