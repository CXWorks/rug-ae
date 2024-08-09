//! This module defines the [Layout] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use crate::config_file::Config;

use super::Configurable;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which output layout to print.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Grid,
    Tree,
    OneLine,
}

impl Configurable<Layout> for Layout {
    /// Get a potential `Layout` variant from [ArgMatches].
    ///
    /// If any of the "tree", "long" or "oneline" arguments is passed, this returns the
    /// corresponding `Layout` variant in a [Some]. Otherwise if the number of passed "blocks"
    /// arguments is greater than 1, this also returns the [OneLine](Layout::OneLine) variant.
    /// Finally if neither of them is passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("tree") {
            Some(Self::Tree)
        } else if matches.is_present("long")
            || matches.is_present("oneline")
            || matches.is_present("inode")
            || matches.is_present("context")
            || matches!(matches.values_of("blocks"), Some(values) if values.len() > 1)
        // TODO: handle this differently
        {
            Some(Self::OneLine)
        } else {
            None
        }
    }

    /// Get a potential Layout variant from a [Config].
    ///
    /// If the `Config::layout` has value and is one of "tree", "oneline" or "grid",
    /// this returns the corresponding `Layout` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.layout
    }
}

/// The default value for `Layout` is [Layout::Grid].
impl Default for Layout {
    fn default() -> Self {
        Self::Grid
    }
}

#[cfg(test)]
mod test {
    use super::Layout;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_tree() {
        let argv = vec!["lsd", "--tree"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::Tree), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline() {
        let argv = vec!["lsd", "--oneline"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_long() {
        let argv = vec!["lsd", "--long"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_oneline_through_blocks() {
        let argv = vec!["lsd", "--blocks", "permission,name"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Layout::OneLine), Layout::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Layout::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_tree() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Tree);
        assert_eq!(Some(Layout::Tree), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_oneline() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::OneLine);
        assert_eq!(Some(Layout::OneLine), Layout::from_config(&c));
    }

    #[test]
    fn test_from_config_grid() {
        let mut c = Config::with_none();
        c.layout = Some(Layout::Grid);
        assert_eq!(Some(Layout::Grid), Layout::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_202() {
//    rusty_monitor::set_test_id(202);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_10_ref_0: &flags::layout::Layout = &mut layout_10;
    let mut layout_11: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_11_ref_0: &flags::layout::Layout = &mut layout_11;
    let mut layout_12: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_12_ref_0: &flags::layout::Layout = &mut layout_12;
    let mut layout_13: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_13_ref_0: &flags::layout::Layout = &mut layout_13;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_13_ref_0, layout_12_ref_0);
    let mut bool_1: bool = crate::flags::layout::Layout::eq(layout_11_ref_0, layout_10_ref_0);
    let mut bool_2: bool = crate::flags::layout::Layout::eq(layout_9_ref_0, layout_8_ref_0);
    let mut bool_3: bool = crate::flags::layout::Layout::eq(layout_7_ref_0, layout_6_ref_0);
    let mut bool_4: bool = crate::flags::layout::Layout::eq(layout_5_ref_0, layout_4_ref_0);
    let mut bool_5: bool = crate::flags::layout::Layout::eq(layout_3_ref_0, layout_2_ref_0);
    let mut bool_6: bool = crate::flags::layout::Layout::eq(layout_1_ref_0, layout_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_770() {
//    rusty_monitor::set_test_id(770);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_21: std::option::Option<flags::layout::Layout> = crate::flags::layout::Layout::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_225() {
//    rusty_monitor::set_test_id(225);
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut bool_2: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::Socket;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_9_ref_0);
    let mut bool_4: bool = crate::color::Elem::has_suid(elem_8_ref_0);
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_7_ref_0);
    let mut bool_6: bool = crate::color::Elem::has_suid(elem_6_ref_0);
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_5_ref_0);
    let mut bool_8: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut bool_9: bool = crate::color::Elem::has_suid(elem_3_ref_0);
    let mut bool_10: bool = crate::color::Elem::has_suid(elem_2_ref_0);
    let mut bool_11: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut bool_12: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_580() {
//    rusty_monitor::set_test_id(580);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_10_ref_0: &flags::layout::Layout = &mut layout_10;
    let mut tuple_0: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_10_ref_0);
    let mut tuple_1: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_9_ref_0);
    let mut tuple_2: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_8_ref_0);
    let mut tuple_3: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_7_ref_0);
    let mut tuple_4: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_6_ref_0);
    let mut tuple_5: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_5_ref_0);
    let mut tuple_6: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_4_ref_0);
    let mut tuple_7: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_3_ref_0);
    let mut tuple_8: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_2_ref_0);
    let mut tuple_9: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_1_ref_0);
    let mut tuple_10: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_0_ref_0);
//    panic!("From RustyUnit with love");
}
}