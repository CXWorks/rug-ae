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
#[timeout(30000)]fn rusty_test_650() {
//    rusty_monitor::set_test_id(650);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_10_ref_0: &flags::layout::Layout = &mut layout_10;
    let mut layout_11: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_11_ref_0: &flags::layout::Layout = &mut layout_11;
    let mut layout_12: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_12_ref_0: &flags::layout::Layout = &mut layout_12;
    let mut layout_13: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
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
#[timeout(30000)]fn rusty_test_307() {
//    rusty_monitor::set_test_id(307);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 1usize;
    let mut bool_3: bool = false;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 1usize;
    let mut bool_4: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_6, exec: bool_5};
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_18, user_write: bool_17, user_execute: bool_16, group_read: bool_15, group_write: bool_14, group_execute: bool_13, other_read: bool_12, other_write: bool_11, other_execute: bool_10, sticky: bool_9, setgid: bool_8, setuid: bool_7};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 120usize;
    let mut bool_19: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_19, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_1: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_2: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_3);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_2, theme: option_1};
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut option_6: std::option::Option<flags::layout::Layout> = crate::flags::layout::Layout::from_config(config_2_ref_0);
    let mut option_7: std::option::Option<flags::layout::Layout> = crate::flags::layout::Layout::from_config(config_1_ref_0);
    let mut option_8: std::option::Option<flags::layout::Layout> = crate::flags::layout::Layout::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_328() {
//    rusty_monitor::set_test_id(328);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut layout_5: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_5_ref_0: &flags::layout::Layout = &mut layout_5;
    let mut layout_6: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_6_ref_0: &flags::layout::Layout = &mut layout_6;
    let mut layout_7: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_7_ref_0: &flags::layout::Layout = &mut layout_7;
    let mut layout_8: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_8_ref_0: &flags::layout::Layout = &mut layout_8;
    let mut layout_9: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_9_ref_0: &flags::layout::Layout = &mut layout_9;
    let mut layout_10: flags::layout::Layout = crate::flags::layout::Layout::Grid;
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