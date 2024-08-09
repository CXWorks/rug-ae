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
#[timeout(30000)]fn rusty_test_420() {
//    rusty_monitor::set_test_id(420);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_4_ref_0: &flags::sorting::SortOrder = &mut sortorder_4;
    let mut sortorder_5: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_5_ref_0: &flags::sorting::SortOrder = &mut sortorder_5;
    let mut sortorder_6: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_6_ref_0: &flags::sorting::SortOrder = &mut sortorder_6;
    let mut sortorder_7: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_7_ref_0: &flags::sorting::SortOrder = &mut sortorder_7;
    let mut sortorder_8: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_8_ref_0: &flags::sorting::SortOrder = &mut sortorder_8;
    let mut sortorder_9: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_9_ref_0: &flags::sorting::SortOrder = &mut sortorder_9;
    let mut sortorder_10: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_10_ref_0: &flags::sorting::SortOrder = &mut sortorder_10;
    let mut sortorder_11: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_11_ref_0: &flags::sorting::SortOrder = &mut sortorder_11;
    let mut sortorder_12: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_12_ref_0: &flags::sorting::SortOrder = &mut sortorder_12;
    let mut sortorder_13: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
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
#[timeout(30000)]fn rusty_test_9178() {
//    rusty_monitor::set_test_id(9178);
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_0_ref_0: &crate::config_file::Sorting = &mut sorting_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = crate::flags::sorting::SortColumn::from_config(config_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut option_4: std::option::Option<usize> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut str_0: &str = "SymlinkArrow";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_11: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_2: crate::config_file::Sorting = crate::config_file::Sorting {column: option_11, reverse: option_10, dir_grouping: option_9};
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_2);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_15: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_16: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_18, indicators: option_17, layout: option_16, recursion: option_15, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_8, total_size: option_7, symlink_arrow: option_6, hyperlink: option_5};
    let mut elem_1: color::Elem = crate::color::Elem::User;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_226() {
//    rusty_monitor::set_test_id(226);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_3_ref_0: &flags::sorting::SortOrder = &mut sortorder_3;
    let mut sortorder_4: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_4_ref_0: &flags::sorting::SortOrder = &mut sortorder_4;
    let mut sortorder_5: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_5_ref_0: &flags::sorting::SortOrder = &mut sortorder_5;
    let mut sortorder_6: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_6_ref_0: &flags::sorting::SortOrder = &mut sortorder_6;
    let mut sortorder_7: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_7_ref_0: &flags::sorting::SortOrder = &mut sortorder_7;
    let mut sortorder_8: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_8_ref_0: &flags::sorting::SortOrder = &mut sortorder_8;
    let mut sortorder_9: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2629() {
//    rusty_monitor::set_test_id(2629);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut str_0: &str = "NoLscolors";
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut str_1: &str = "ds_store";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "group_execute";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "hs";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = ".bashrc";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "gulpfile.js";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut bool_0: bool = crate::flags::sorting::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7860() {
//    rusty_monitor::set_test_id(7860);
    let mut bool_0: bool = false;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 2usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 360usize;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 80usize;
    let mut bool_2: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_3, theme: themeoption_3};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_config(config_1_ref_0);
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_330() {
//    rusty_monitor::set_test_id(330);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_1_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_3_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_3;
    let mut dirgrouping_4: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_4_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_4;
    let mut dirgrouping_5: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_5_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_5;
    let mut dirgrouping_6: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_6_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_6;
    let mut dirgrouping_7: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_7_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_7;
    let mut dirgrouping_8: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_8_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_8;
    let mut dirgrouping_9: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_9_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_9;
    let mut dirgrouping_10: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_10_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_10;
    let mut dirgrouping_11: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_11_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_11;
    let mut dirgrouping_12: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_12_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_12;
    let mut dirgrouping_13: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
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
#[timeout(30000)]fn rusty_test_789() {
//    rusty_monitor::set_test_id(789);
    let mut str_0: &str = "permission";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_2_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_2;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_3_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_3;
    let mut dirgrouping_4: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_4_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_4;
    let mut dirgrouping_5: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_5_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_5;
    let mut dirgrouping_6: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_6_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_6;
    let mut dirgrouping_7: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut dirgrouping_7_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_7;
    let mut dirgrouping_8: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut dirgrouping_8_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_8;
    let mut dirgrouping_9: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut dirgrouping_9_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_9;
    let mut dirgrouping_10: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_9_ref_0);
    let mut dirgrouping_11: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_8_ref_0);
    let mut dirgrouping_12: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_7_ref_0);
    let mut dirgrouping_13: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_6_ref_0);
    let mut dirgrouping_14: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_5_ref_0);
    let mut dirgrouping_15: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_4_ref_0);
    let mut dirgrouping_16: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_3_ref_0);
    let mut dirgrouping_17: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::clone(dirgrouping_2_ref_0);
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_389() {
//    rusty_monitor::set_test_id(389);
    let mut str_0: &str = "SymLink";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = crate::flags::sorting::DirGrouping::from_str(str_0_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = crate::flags::sorting::SortColumn::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_861() {
//    rusty_monitor::set_test_id(861);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_0_ref_0: &flags::sorting::SortOrder = &mut sortorder_0;
    let mut tuple_0: () = crate::flags::sorting::SortOrder::assert_receiver_is_total_eq(sortorder_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6928() {
//    rusty_monitor::set_test_id(6928);
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_0: usize = 40usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "CLRenS5uE";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut u64_1: u64 = 39u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_1: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 5usize;
    let mut bool_2: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut option_23: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_24: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut bool_3: bool = crate::flags::sorting::Sorting::eq(sorting_2_ref_0, sorting_1_ref_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_897() {
//    rusty_monitor::set_test_id(897);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 0usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 50u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut u64_1: u64 = 45u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::clone(sorting_0_ref_0);
    let mut bool_1: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_966() {
//    rusty_monitor::set_test_id(966);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "Do not ignore entries starting with .";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut bool_1: bool = crate::flags::sorting::Sorting::ne(sorting_1_ref_0, sorting_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_248() {
//    rusty_monitor::set_test_id(248);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_3_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_3;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_4_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_4;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_5_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_5;
    let mut sortcolumn_6: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_6_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_6;
    let mut sortcolumn_7: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_7_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_7;
    let mut sortcolumn_8: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_8_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_8;
    let mut sortcolumn_9: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_9_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_9;
    let mut sortcolumn_10: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
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
#[timeout(30000)]fn rusty_test_8247() {
//    rusty_monitor::set_test_id(8247);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut bool_0: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 6usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_1: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "gsheet";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut sorting_4: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_3_ref_0: &crate::flags::sorting::Sorting = &mut sorting_3;
    let mut sorting_5: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_4_ref_0: &crate::flags::sorting::Sorting = &mut sorting_4;
    let mut bool_3: bool = crate::flags::sorting::Sorting::ne(sorting_3_ref_0, sorting_2_ref_0);
    let mut bool_4: bool = crate::flags::sorting::Sorting::ne(sorting_0_ref_0, sorting_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1850() {
//    rusty_monitor::set_test_id(1850);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut str_0: &str = "vlc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 0usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut tuple_0: () = crate::flags::sorting::DirGrouping::assert_receiver_is_total_eq(dirgrouping_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_705() {
//    rusty_monitor::set_test_id(705);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_3_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_3;
    let mut sortcolumn_4: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_4_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_4;
    let mut sortcolumn_5: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_5_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_5;
    let mut sortcolumn_6: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_6_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_6;
    let mut sortcolumn_7: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_7_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_7;
    let mut sortcolumn_8: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::default();
    let mut sortcolumn_8_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_8;
    let mut sortcolumn_9: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_9_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_9;
    let mut sortcolumn_10: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
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
#[timeout(30000)]fn rusty_test_3069() {
//    rusty_monitor::set_test_id(3069);
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut str_0: &str = "target";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "MissingSymLinkTarget";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "ds_store";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "group_execute";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "hs";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = ".bashrc";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "gulpfile.js";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "UtT14M";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut bool_0: bool = crate::flags::sorting::SortColumn::eq(sortcolumn_1_ref_0, sortcolumn_0_ref_0);
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_292() {
//    rusty_monitor::set_test_id(292);
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_1_ref_0: &crate::flags::sorting::Sorting = &mut sorting_1;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_2_ref_0: &crate::flags::sorting::Sorting = &mut sorting_2;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_3_ref_0: &crate::flags::sorting::Sorting = &mut sorting_3;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_4: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut sorting_4_ref_0: &crate::flags::sorting::Sorting = &mut sorting_4;
    let mut sorting_5: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting::default();
    let mut sorting_5_ref_0: &crate::flags::sorting::Sorting = &mut sorting_5;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::default();
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::default();
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_6: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut sorting_6_ref_0: &crate::flags::sorting::Sorting = &mut sorting_6;
    let mut tuple_0: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_6_ref_0);
    let mut tuple_1: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_5_ref_0);
    let mut tuple_2: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_4_ref_0);
    let mut tuple_3: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_3_ref_0);
    let mut tuple_4: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_2_ref_0);
    let mut tuple_5: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_1_ref_0);
    let mut tuple_6: () = crate::flags::sorting::Sorting::assert_receiver_is_total_eq(sorting_0_ref_0);
//    panic!("From RustyUnit with love");
}
}