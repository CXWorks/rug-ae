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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5369() {
    rusty_monitor::set_test_id(5369);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 70usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut ignoreglobs_0: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "bWvEakHNySWg9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut bool_2: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut bool_3: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut bool_4: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_1: &str = "t";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 75u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_2: &str = "fDVfDs0bkXtY2";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<globset::Glob, clap::Error> = crate::flags::ignore_globs::IgnoreGlobs::create_glob(str_2_ref_0);
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut glob_0: globset::Glob = std::result::Result::unwrap(result_0);
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4999() {
    rusty_monitor::set_test_id(4999);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut ignoreglobs_0: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "LgoRme5sUbaNdQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "zmQZVHCl2f";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "ur7pe4l";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_1: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut u64_0: u64 = 93u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = false;
    let mut u64_1: u64 = 70u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1230() {
    rusty_monitor::set_test_id(1230);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 20usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut ignoreglobs_0: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut str_0: &str = "1LyBR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut ignoreglobs_1: crate::flags::ignore_globs::IgnoreGlobs = crate::flags::ignore_globs::IgnoreGlobs::default();
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_0: std::option::Option<std::result::Result<globset::GlobSet, clap::Error>> = crate::flags::ignore_globs::IgnoreGlobs::from_config(config_3_ref_0);
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    panic!("From RustyUnit with love");
}
}