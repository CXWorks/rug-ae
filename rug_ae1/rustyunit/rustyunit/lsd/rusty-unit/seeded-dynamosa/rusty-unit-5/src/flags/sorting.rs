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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_570() {
//    rusty_monitor::set_test_id(570);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_config(config_0_ref_0);
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = crate::flags::sorting::SortColumn::from_config(config_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7544() {
//    rusty_monitor::set_test_id(7544);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_4, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_5, order: sortorder_3, dir_grouping: dirgrouping_3};
    let mut sorting_3_ref_0: &crate::flags::sorting::Sorting = &mut sorting_3;
    let mut bool_3: bool = crate::flags::sorting::Sorting::eq(sorting_3_ref_0, sorting_2_ref_0);
    let mut bool_4: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    let mut bool_5: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_627() {
//    rusty_monitor::set_test_id(627);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_3_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_3;
    let mut dirgrouping_4: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_4_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_4;
    let mut dirgrouping_5: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_5_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_5;
    let mut dirgrouping_6: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_6_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_6;
    let mut dirgrouping_7: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_7_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_7;
    let mut dirgrouping_8: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_8_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_8;
    let mut dirgrouping_9: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_9_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_9;
    let mut dirgrouping_10: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_10_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_10;
    let mut dirgrouping_11: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_10_ref_0);
    let mut dirgrouping_12: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_9_ref_0);
    let mut dirgrouping_13: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_8_ref_0);
    let mut dirgrouping_14: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_7_ref_0);
    let mut dirgrouping_15: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_6_ref_0);
    let mut dirgrouping_16: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_5_ref_0);
    let mut dirgrouping_17: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_4_ref_0);
    let mut dirgrouping_18: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_3_ref_0);
    let mut dirgrouping_19: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_2_ref_0);
    let mut dirgrouping_20: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_1_ref_0);
    let mut dirgrouping_21: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_340() {
//    rusty_monitor::set_test_id(340);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_3_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_3;
    let mut dirgrouping_4: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_4_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_4;
    let mut dirgrouping_5: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_5_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_5;
    let mut dirgrouping_6: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_6_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_6;
    let mut dirgrouping_7: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_7_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_7;
    let mut dirgrouping_8: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_8_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_8;
    let mut dirgrouping_9: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_9_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_9;
    let mut dirgrouping_10: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_10_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_10;
    let mut dirgrouping_11: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_11_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_11;
    let mut dirgrouping_12: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_12_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_12;
    let mut dirgrouping_13: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_13_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_13;
    let mut bool_0: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_13_ref_0, dirgrouping_12_ref_0);
    let mut bool_1: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_11_ref_0, dirgrouping_10_ref_0);
    let mut bool_2: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_9_ref_0, dirgrouping_8_ref_0);
    let mut bool_3: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_7_ref_0, dirgrouping_6_ref_0);
    let mut bool_4: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_5_ref_0, dirgrouping_4_ref_0);
    let mut bool_5: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_3_ref_0, dirgrouping_2_ref_0);
    let mut bool_6: bool = crate::flags::sorting::DirGrouping::eq(dirgrouping_1_ref_0, dirgrouping_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_362() {
//    rusty_monitor::set_test_id(362);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_4_ref_0: &flags::sorting::SortOrder = &mut sortorder_4;
    let mut sortorder_5: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_5_ref_0: &flags::sorting::SortOrder = &mut sortorder_5;
    let mut sortorder_6: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_6_ref_0: &flags::sorting::SortOrder = &mut sortorder_6;
    let mut sortorder_7: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_7_ref_0: &flags::sorting::SortOrder = &mut sortorder_7;
    let mut sortorder_8: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_8_ref_0: &flags::sorting::SortOrder = &mut sortorder_8;
    let mut sortorder_9: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_9_ref_0: &flags::sorting::SortOrder = &mut sortorder_9;
    let mut sortorder_10: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_10_ref_0: &flags::sorting::SortOrder = &mut sortorder_10;
    let mut sortorder_11: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_11_ref_0: &flags::sorting::SortOrder = &mut sortorder_11;
    let mut sortorder_12: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_12_ref_0: &flags::sorting::SortOrder = &mut sortorder_12;
    let mut sortorder_13: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_13_ref_0: &flags::sorting::SortOrder = &mut sortorder_13;
    let mut bool_0: bool = crate::flags::sorting::SortOrder::eq(sortorder_13_ref_0, sortorder_12_ref_0);
    let mut bool_1: bool = crate::flags::sorting::SortOrder::eq(sortorder_11_ref_0, sortorder_10_ref_0);
    let mut bool_2: bool = crate::flags::sorting::SortOrder::eq(sortorder_9_ref_0, sortorder_8_ref_0);
    let mut bool_3: bool = crate::flags::sorting::SortOrder::eq(sortorder_7_ref_0, sortorder_6_ref_0);
    let mut bool_4: bool = crate::flags::sorting::SortOrder::eq(sortorder_5_ref_0, sortorder_4_ref_0);
    let mut bool_5: bool = crate::flags::sorting::SortOrder::eq(sortorder_3_ref_0, sortorder_2_ref_0);
    let mut bool_6: bool = crate::flags::sorting::SortOrder::eq(sortorder_1_ref_0, sortorder_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_479() {
//    rusty_monitor::set_test_id(479);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_4_ref_0: &flags::sorting::SortOrder = &mut sortorder_4;
    let mut sortorder_5: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_5_ref_0: &flags::sorting::SortOrder = &mut sortorder_5;
    let mut sortorder_6: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_6_ref_0: &flags::sorting::SortOrder = &mut sortorder_6;
    let mut sortorder_7: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_7_ref_0: &flags::sorting::SortOrder = &mut sortorder_7;
    let mut sortorder_8: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_8_ref_0: &flags::sorting::SortOrder = &mut sortorder_8;
    let mut sortorder_9: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_9_ref_0: &flags::sorting::SortOrder = &mut sortorder_9;
    let mut sortorder_10: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_10_ref_0: &flags::sorting::SortOrder = &mut sortorder_10;
    let mut tuple_0: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_10_ref_0);
    let mut tuple_1: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_9_ref_0);
    let mut tuple_2: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_8_ref_0);
    let mut tuple_3: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_7_ref_0);
    let mut tuple_4: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_6_ref_0);
    let mut tuple_5: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_5_ref_0);
    let mut tuple_6: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_4_ref_0);
    let mut tuple_7: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_3_ref_0);
    let mut tuple_8: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_2_ref_0);
    let mut tuple_9: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_1_ref_0);
    let mut tuple_10: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_458() {
//    rusty_monitor::set_test_id(458);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_3_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_3;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_4_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_4;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_5_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_5;
    let mut sortcolumn_6: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_6_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_6;
    let mut sortcolumn_7: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_7_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_7;
    let mut sortcolumn_8: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_8_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_8;
    let mut sortcolumn_9: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_9_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_9;
    let mut sortcolumn_10: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_10_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_10;
    let mut sortcolumn_11: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_10_ref_0);
    let mut sortcolumn_12: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_9_ref_0);
    let mut sortcolumn_13: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_8_ref_0);
    let mut sortcolumn_14: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_7_ref_0);
    let mut sortcolumn_15: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_6_ref_0);
    let mut sortcolumn_16: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_5_ref_0);
    let mut sortcolumn_17: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_4_ref_0);
    let mut sortcolumn_18: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_3_ref_0);
    let mut sortcolumn_19: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_2_ref_0);
    let mut sortcolumn_20: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_1_ref_0);
    let mut sortcolumn_21: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::clone(sortcolumn_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_722() {
//    rusty_monitor::set_test_id(722);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_3_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_3;
    let mut dirgrouping_4: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_4_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_4;
    let mut dirgrouping_5: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_5_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_5;
    let mut dirgrouping_6: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_6_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_6;
    let mut dirgrouping_7: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_7_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_7;
    let mut dirgrouping_8: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_8_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_8;
    let mut dirgrouping_9: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_9_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_9;
    let mut dirgrouping_10: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_10_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_10;
    let mut tuple_0: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_10_ref_0);
    let mut tuple_1: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_9_ref_0);
    let mut tuple_2: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_8_ref_0);
    let mut tuple_3: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_7_ref_0);
    let mut tuple_4: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_6_ref_0);
    let mut tuple_5: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_5_ref_0);
    let mut tuple_6: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_4_ref_0);
    let mut tuple_7: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_3_ref_0);
    let mut tuple_8: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_2_ref_0);
    let mut tuple_9: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_1_ref_0);
    let mut tuple_10: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8836() {
//    rusty_monitor::set_test_id(8836);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1_ref_0: &flags::date::DateFlag = &mut dateflag_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut str_0: &str = "Date";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 4u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut dateflag_2_ref_0: &flags::date::DateFlag = &mut dateflag_2;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut bool_1: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8186() {
//    rusty_monitor::set_test_id(8186);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_5: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_6: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_6, theme: option_5, separator: option_4};
    let mut option_7: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_11: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_0: u64 = 92u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 54u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_12: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_13: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_13, theme: option_12};
    let mut option_14: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_3_ref_0: &crate::flags::sorting::Sorting = &mut sorting_3;
    let mut sorting_4: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_4_ref_0: &crate::flags::sorting::Sorting = &mut sorting_4;
    let mut bool_16: bool = crate::flags::sorting::Sorting::ne(sorting_4_ref_0, sorting_3_ref_0);
    let mut bool_17: bool = crate::flags::sorting::Sorting::ne(sorting_2_ref_0, sorting_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_413() {
//    rusty_monitor::set_test_id(413);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_3_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_3;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_4_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_4;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_5_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_5;
    let mut sortcolumn_6: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_6_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_6;
    let mut sortcolumn_7: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_7_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_7;
    let mut sortcolumn_8: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_8_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_8;
    let mut sortcolumn_9: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_9_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_9;
    let mut sortcolumn_10: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_10_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_10;
    let mut tuple_0: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_10_ref_0);
    let mut tuple_1: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_9_ref_0);
    let mut tuple_2: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_8_ref_0);
    let mut tuple_3: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_7_ref_0);
    let mut tuple_4: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_6_ref_0);
    let mut tuple_5: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_5_ref_0);
    let mut tuple_6: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_4_ref_0);
    let mut tuple_7: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_3_ref_0);
    let mut tuple_8: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_2_ref_0);
    let mut tuple_9: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_1_ref_0);
    let mut tuple_10: () = crate::flags::sorting::SortColumn::assert_receiver_is_total_eq(sortcolumn_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_183() {
//    rusty_monitor::set_test_id(183);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_3_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_3;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_4_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_4;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_5_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_5;
    let mut sortcolumn_6: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_6_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_6;
    let mut sortcolumn_7: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_7_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_7;
    let mut sortcolumn_8: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_8_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_8;
    let mut sortcolumn_9: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_9_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_9;
    let mut sortcolumn_10: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_10_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_10;
    let mut sortcolumn_11: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_11_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_11;
    let mut sortcolumn_12: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_12_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_12;
    let mut sortcolumn_13: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_13_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_13;
    let mut bool_0: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_13_ref_0, sortcolumn_12_ref_0);
    let mut bool_1: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_11_ref_0, sortcolumn_10_ref_0);
    let mut bool_2: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_9_ref_0, sortcolumn_8_ref_0);
    let mut bool_3: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_7_ref_0, sortcolumn_6_ref_0);
    let mut bool_4: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_5_ref_0, sortcolumn_4_ref_0);
    let mut bool_5: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_3_ref_0, sortcolumn_2_ref_0);
    let mut bool_6: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_610() {
//    rusty_monitor::set_test_id(610);
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "Stop recursing into directories after reaching specified depth";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_str(str_0_ref_0);
    let mut str_1: &str = "c";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = ".gitignore";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut usize_0: usize = 98usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1345() {
//    rusty_monitor::set_test_id(1345);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_4: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_5: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_5, theme: option_4, separator: option_3};
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_10: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut str_0: &str = "pdf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_0: u64 = 92u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 54u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_11: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_12: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_12, theme: option_11};
    let mut option_13: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_4: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_5: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_5_ref_0: &crate::flags::sorting::Sorting = &mut sorting_5;
    let mut bool_15: bool = crate::flags::sorting::Sorting::ne(sorting_5_ref_0, sorting_2_ref_0);
    let mut bool_16: bool = crate::flags::sorting::Sorting::ne(sorting_1_ref_0, sorting_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4501() {
//    rusty_monitor::set_test_id(4501);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_17, theme: option_16};
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_3, dir_grouping: dirgrouping_3};
    let mut sorting_3_ref_0: &crate::flags::sorting::Sorting = &mut sorting_3;
    let mut bool_3: bool = crate::flags::sorting::Sorting::eq(sorting_3_ref_0, sorting_2_ref_0);
    let mut bool_4: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_625() {
//    rusty_monitor::set_test_id(625);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_4_ref_0: &flags::sorting::SortOrder = &mut sortorder_4;
    let mut sortorder_5: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_5_ref_0: &flags::sorting::SortOrder = &mut sortorder_5;
    let mut sortorder_6: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_6_ref_0: &flags::sorting::SortOrder = &mut sortorder_6;
    let mut sortorder_7: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_7_ref_0: &flags::sorting::SortOrder = &mut sortorder_7;
    let mut sortorder_8: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_8_ref_0: &flags::sorting::SortOrder = &mut sortorder_8;
    let mut sortorder_9: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_9_ref_0: &flags::sorting::SortOrder = &mut sortorder_9;
    let mut sortorder_10: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_10_ref_0: &flags::sorting::SortOrder = &mut sortorder_10;
    let mut sortorder_11: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_10_ref_0);
    let mut sortorder_12: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_9_ref_0);
    let mut sortorder_13: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_8_ref_0);
    let mut sortorder_14: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_7_ref_0);
    let mut sortorder_15: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_6_ref_0);
    let mut sortorder_16: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_5_ref_0);
    let mut sortorder_17: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_4_ref_0);
    let mut sortorder_18: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_3_ref_0);
    let mut sortorder_19: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_2_ref_0);
    let mut sortorder_20: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_1_ref_0);
    let mut sortorder_21: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::clone(sortorder_0_ref_0);
//    panic!("From RustyUnit with love");
}
}