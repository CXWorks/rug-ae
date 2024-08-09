//! This module defines the [Sorting] options. To set it up from [ArgMatches], a [Config]
//! and its [Default] value, use the [configure_from](Sorting::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// A collection of flags on how to sort the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Sorting {
    pub column: SortColumn,
    pub order: SortOrder,
    pub dir_grouping: DirGrouping,
}

impl Sorting {
    /// Get a `Sorting` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [SortColumn], [SortOrder] and [DirGrouping] are configured with their respective
    /// [Configurable] implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let column = SortColumn::configure_from(matches, config);
        let order = SortOrder::configure_from(matches, config);
        let dir_grouping = DirGrouping::configure_from(matches, config);
        Self {
            column,
            order,
            dir_grouping,
        }
    }
}

/// The flag showing which column to use for sorting.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SortColumn {
    None,
    Extension,
    Name,
    Time,
    Size,
    Version,
}

impl Configurable<Self> for SortColumn {
    /// Get a potential `SortColumn` variant from [ArgMatches].
    ///
    /// If either the "timesort" or "sizesort" arguments are passed, this returns the corresponding
    /// `SortColumn` variant in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        let sort = match matches.values_of("sort") {
            Some(s) => s.last(),
            None => None,
        };
        if matches.is_present("timesort") || sort == Some("time") {
            Some(Self::Time)
        } else if matches.is_present("sizesort") || sort == Some("size") {
            Some(Self::Size)
        } else if matches.is_present("extensionsort") || sort == Some("extension") {
            Some(Self::Extension)
        } else if matches.is_present("versionsort") || sort == Some("version") {
            Some(Self::Version)
        } else if matches.is_present("no-sort") || sort == Some("none") {
            Some(Self::None)
        } else {
            None
        }
    }

    /// Get a potential `SortColumn` variant from a [Config].
    ///
    /// If the `Config::sorting::column` has value and is one of "time", "size" or "name",
    /// this returns the corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(sort) = &config.sorting {
            sort.column
        } else {
            None
        }
    }
}

/// The default value for `SortColumn` is [SortColumn::Name].
impl Default for SortColumn {
    fn default() -> Self {
        Self::Name
    }
}

/// The flag showing which sort order to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Default,
    Reverse,
}

impl Configurable<Self> for SortOrder {
    /// Get a potential `SortOrder` variant from [ArgMatches].
    ///
    /// If the "reverse" argument is passed, this returns [SortOrder::Reverse] in a [Some].
    /// Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("reverse") {
            Some(Self::Reverse)
        } else {
            None
        }
    }

    /// Get a potential `SortOrder` variant from a [Config].
    ///
    /// If the `Config::sorting::reverse` has value,
    /// this returns a mapped variant in a [Some].
    /// Otherwise [None] is returned.
    /// A `true` maps to [SortOrder::Reverse] while `false` maps to [SortOrder::Default].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(sort) = &config.sorting {
            if let Some(reverse) = sort.reverse {
                if reverse {
                    Some(Self::Reverse)
                } else {
                    Some(Self::Default)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// The default value for `SortOrder` is [SortOrder::Default].
impl Default for SortOrder {
    fn default() -> Self {
        Self::Default
    }
}

/// The flag showing where to place directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirGrouping {
    None,
    First,
    Last,
}

impl DirGrouping {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "first" => Some(Self::First),
            "last" => Some(Self::Last),
            "none" => Some(Self::None),
            _ => panic!(
                "Group Dir can only be one of first, last or none, but got {}.",
                value
            ),
        }
    }
}
impl Configurable<Self> for DirGrouping {
    /// Get a potential `DirGrouping` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [DirGrouping::None] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            return Some(Self::None);
        }

        if matches.is_present("group-directories-first") {
            return Some(Self::First);
        }

        if matches.occurrences_of("group-dirs") > 0 {
            if let Some(group_dirs) = matches.values_of("group-dirs")?.last() {
                return Self::from_str(group_dirs);
            }
        }
        None
    }

    /// Get a potential `DirGrouping` variant from a [Config].
    ///
    /// If the `Config::classic` has value and is `true`,
    /// then this returns the the [DirGrouping::None] variant in a [Some].
    /// Otherwise if `Config::sorting::dir-grouping` has value and
    /// is one of "first", "last" or "none", this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            return Some(Self::None);
        }
        if let Some(sort) = &config.sorting {
            return sort.dir_grouping;
        }
        None
    }
}

/// The default value for `DirGrouping` is [DirGrouping::None].
impl Default for DirGrouping {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod test_sort_column {
    use super::SortColumn;

    use crate::app;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SortColumn::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_extension() {
        let argv = vec!["lsd", "--extensionsort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Extension),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_time() {
        let argv = vec!["lsd", "--timesort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_size() {
        let argv = vec!["lsd", "--sizesort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Size),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_version() {
        let argv = vec!["lsd", "--versionsort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Version),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_no_sort() {
        let argv = vec!["lsd", "--no-sort"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::None),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_sort() {
        let argv = vec!["lsd", "--sort", "time"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Size),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "extension"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Extension),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "version"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Version),
            SortColumn::from_arg_matches(&matches)
        );

        let argv = vec!["lsd", "--sort", "none"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::None),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_multi_sort() {
        let argv = vec!["lsd", "--sort", "size", "--sort", "time"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_multi_sort_use_last() {
        let argv = vec!["lsd", "--sort", "size", "-t", "-S", "-X", "--sort", "time"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortColumn::Time),
            SortColumn::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortColumn::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty_column() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });

        assert_eq!(None, SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_extension() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Extension),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Extension), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_name() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Name),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Name), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_time() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Time),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Time), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_size() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Size),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Size), SortColumn::from_config(&c));
    }

    #[test]
    fn test_from_config_version() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: Some(SortColumn::Version),
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(Some(SortColumn::Version), SortColumn::from_config(&c));
    }
}

#[cfg(test)]
mod test_sort_order {
    use super::SortOrder;

    use crate::app;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, SortOrder::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_reverse() {
        let argv = vec!["lsd", "--reverse"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(SortOrder::Reverse),
            SortOrder::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, SortOrder::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_default_config() {
        assert_eq!(
            Some(SortOrder::default()),
            SortOrder::from_config(&Config::builtin())
        );
    }

    #[test]
    fn test_from_config_empty_reverse() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_true() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(true),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Reverse), SortOrder::from_config(&c));
    }

    #[test]
    fn test_from_config_reverse_false() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: Some(false),
            dir_grouping: None,
        });
        assert_eq!(Some(SortOrder::Default), SortOrder::from_config(&c));
    }
}

#[cfg(test)]
mod test_dir_grouping {
    use super::DirGrouping;

    use crate::app;
    use crate::config_file::{Config, Sorting};
    use crate::flags::Configurable;

    #[test]
    #[should_panic(
        expected = "Group Dir can only be one of first, last or none, but got bad value."
    )]
    fn test_from_str_bad_value() {
        assert_eq!(None, DirGrouping::from_str("bad value"));
    }

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, DirGrouping::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_first() {
        let argv = vec!["lsd", "--group-dirs", "first"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::First),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_last() {
        let argv = vec!["lsd", "--group-dirs", "last"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::Last),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_explicit_none() {
        let argv = vec!["lsd", "--group-dirs", "none"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--group-dirs", "first", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::None),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_group_dirs_multi() {
        let argv = vec!["lsd", "--group-dirs", "first", "--group-dirs", "last"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::Last),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_group_directories_first() {
        let argv = vec!["lsd", "--group-directories-first"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DirGrouping::First),
            DirGrouping::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_empty() {
        assert_eq!(None, DirGrouping::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_first() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::First),
        });
        assert_eq!(Some(DirGrouping::First), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_last() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        assert_eq!(Some(DirGrouping::Last), DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_explicit_empty() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: None,
        });
        assert_eq!(None, DirGrouping::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.sorting = Some(Sorting {
            column: None,
            reverse: None,
            dir_grouping: Some(DirGrouping::Last),
        });
        c.classic = Some(true);
        assert_eq!(Some(DirGrouping::None), DirGrouping::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2610() {
    rusty_monitor::set_test_id(2610);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 31usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_1_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut bool_14: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4186() {
    rusty_monitor::set_test_id(4186);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut theme_5: icon::Theme = crate::icon::Theme::Fancy;
    let mut theme_5_ref_0: &icon::Theme = &mut theme_5;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_5: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_6: color::Elem = crate::color::Elem::FileSmall;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_0_ref_0);
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4155() {
    rusty_monitor::set_test_id(4155);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "b7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_str(str_0_ref_0);
    let mut str_1: &str = "x";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "C";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_0: usize = 88usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_3: &str = "vwtnjG";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut tuple_0: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_0_ref_0);
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2165() {
    rusty_monitor::set_test_id(2165);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut tuple_0: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_0_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4747() {
    rusty_monitor::set_test_id(4747);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "o8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut u64_1: u64 = 11u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_2: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 90usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_4: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_5: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_21, blocks: option_20, color: option_19, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut theme_1: icon::Theme = crate::icon::Theme::Fancy;
    let mut tuple_0: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_2_ref_0);
    let mut bool_7: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4729() {
    rusty_monitor::set_test_id(4729);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_2, dir_grouping: dirgrouping_1};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut bool_2: bool = crate::flags::sorting::SortOrder::eq(sortorder_1_ref_0, sortorder_0_ref_0);
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut bool_3: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_2_ref_0, dirgrouping_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_613() {
    rusty_monitor::set_test_id(613);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut str_0: &str = "5dTpc8Yi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut u64_0: u64 = 20u64;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut bool_12: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut metadata_0: &std::fs::Metadata = std::option::Option::unwrap(option_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4327() {
    rusty_monitor::set_test_id(4327);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Socket;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut bool_0: bool = crate::flags::sorting::Sorting::ne(sorting_1_ref_0, sorting_0_ref_0);
    let mut bool_1: bool = crate::flags::sorting::SortOrder::eq(sortorder_1_ref_0, sortorder_0_ref_0);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1941() {
    rusty_monitor::set_test_id(1941);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 9usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 89u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_2, invalid: color_1};
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut bool_1: bool = crate::flags::sorting::SortOrder::eq(sortorder_2_ref_0, sortorder_1_ref_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = crate::flags::sorting::SortColumn::from_config(config_2_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4727() {
    rusty_monitor::set_test_id(4727);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "pT9tDTs2wrBwEf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 45u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 4u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_23: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_config(config_1_ref_0);
    let mut bool_3: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_226() {
    rusty_monitor::set_test_id(226);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_2};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_1, dir_grouping: dirgrouping_3};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut bool_14: bool = crate::flags::sorting::Sorting::ne(sorting_1_ref_0, sorting_0_ref_0);
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_15: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_1_ref_0, dirgrouping_0_ref_0);
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_13, exec: bool_12};
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut tuple_0: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_0_ref_0);
    let mut bool_16: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4227() {
    rusty_monitor::set_test_id(4227);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Context;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::SymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut tuple_0: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_0_ref_0);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1591() {
    rusty_monitor::set_test_id(1591);
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut sizeflag_0_ref_0: &flags::size::SizeFlag = &mut sizeflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut bool_12: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3721() {
    rusty_monitor::set_test_id(3721);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 34usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut tuple_0: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_3, no_uid: color_2};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2987() {
    rusty_monitor::set_test_id(2987);
    let mut usize_0: usize = 30usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 14usize;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_5: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_6: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_3: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_11, theme: option_10, separator: option_9};
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_16: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_17: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_17, theme: option_16};
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut option_21: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_25: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_26: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_27: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_2: usize = 77usize;
    let mut option_28: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_5: bool = false;
    let mut option_29: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_29, depth: option_28};
    let mut option_30: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_31: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_32: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_33: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_34: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_35: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_36: std::option::Option<bool> = std::option::Option::None;
    let mut option_37: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_38: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut option_39: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_39, theme: option_38};
    let mut option_40: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_41: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_42: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_42, blocks: option_41, color: option_40, date: option_37, dereference: option_36, display: option_35, icons: option_34, ignore_globs: option_33, indicators: option_32, layout: option_31, recursion: option_30, size: option_27, permission: option_26, sorting: option_25, no_symlink: option_24, total_size: option_23, symlink_arrow: option_22, hyperlink: option_21};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 26u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 26u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_43: std::option::Option<flags::sorting::SortOrder> = crate::flags::sorting::SortOrder::from_config(config_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    panic!("From RustyUnit with love");
}
}