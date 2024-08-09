//! This module defines the [Indicators] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to print file type indicators.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Indicators(pub bool);

impl Configurable<Self> for Indicators {
    /// Get a potential `Indicators` value from [ArgMatches].
    ///
    /// If the "indicators" argument is passed, this returns an `Indicators` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("indicators") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Indicators` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Indicators`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.indicators.as_ref().map(|ind| Self(*ind))
    }
}

#[cfg(test)]
mod test {
    use super::Indicators;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Indicators::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--classify"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Indicators(true)),
            Indicators::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Indicators::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.indicators = Some(true);
        assert_eq!(Some(Indicators(true)), Indicators::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.indicators = Some(false);
        assert_eq!(Some(Indicators(false)), Indicators::from_config(&c));
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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_425() {
//    rusty_monitor::set_test_id(425);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_7_ref_0: &crate::flags::indicators::Indicators = &mut indicators_7;
    let mut indicators_8: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_8_ref_0: &crate::flags::indicators::Indicators = &mut indicators_8;
    let mut indicators_9: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_9_ref_0: &crate::flags::indicators::Indicators = &mut indicators_9;
    let mut indicators_10: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_10_ref_0: &crate::flags::indicators::Indicators = &mut indicators_10;
    let mut tuple_0: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_10_ref_0);
    let mut tuple_1: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_9_ref_0);
    let mut tuple_2: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_8_ref_0);
    let mut tuple_3: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_7_ref_0);
    let mut tuple_4: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_6_ref_0);
    let mut tuple_5: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_5_ref_0);
    let mut tuple_6: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_4_ref_0);
    let mut tuple_7: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_3_ref_0);
    let mut tuple_8: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_2_ref_0);
    let mut tuple_9: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_1_ref_0);
    let mut tuple_10: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_365() {
//    rusty_monitor::set_test_id(365);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_7_ref_0: &crate::flags::indicators::Indicators = &mut indicators_7;
    let mut indicators_8: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_8_ref_0: &crate::flags::indicators::Indicators = &mut indicators_8;
    let mut indicators_9: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_9_ref_0: &crate::flags::indicators::Indicators = &mut indicators_9;
    let mut indicators_10: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_10_ref_0: &crate::flags::indicators::Indicators = &mut indicators_10;
    let mut indicators_11: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_10_ref_0);
    let mut indicators_12: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_9_ref_0);
    let mut indicators_13: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_8_ref_0);
    let mut indicators_14: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_7_ref_0);
    let mut indicators_15: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_6_ref_0);
    let mut indicators_16: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_5_ref_0);
    let mut indicators_17: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_4_ref_0);
    let mut indicators_18: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_3_ref_0);
    let mut indicators_19: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_2_ref_0);
    let mut indicators_20: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_1_ref_0);
    let mut indicators_21: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_409() {
//    rusty_monitor::set_test_id(409);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_7_ref_0: &crate::flags::indicators::Indicators = &mut indicators_7;
    let mut indicators_8: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_8_ref_0: &crate::flags::indicators::Indicators = &mut indicators_8;
    let mut indicators_9: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_9_ref_0: &crate::flags::indicators::Indicators = &mut indicators_9;
    let mut indicators_10: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_10_ref_0: &crate::flags::indicators::Indicators = &mut indicators_10;
    let mut indicators_11: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_11_ref_0: &crate::flags::indicators::Indicators = &mut indicators_11;
    let mut indicators_12: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_12_ref_0: &crate::flags::indicators::Indicators = &mut indicators_12;
    let mut indicators_13: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_13_ref_0: &crate::flags::indicators::Indicators = &mut indicators_13;
    let mut bool_0: bool = crate::flags::indicators::Indicators::ne(indicators_13_ref_0, indicators_12_ref_0);
    let mut bool_1: bool = crate::flags::indicators::Indicators::ne(indicators_11_ref_0, indicators_10_ref_0);
    let mut bool_2: bool = crate::flags::indicators::Indicators::ne(indicators_9_ref_0, indicators_8_ref_0);
    let mut bool_3: bool = crate::flags::indicators::Indicators::ne(indicators_7_ref_0, indicators_6_ref_0);
    let mut bool_4: bool = crate::flags::indicators::Indicators::ne(indicators_5_ref_0, indicators_4_ref_0);
    let mut bool_5: bool = crate::flags::indicators::Indicators::ne(indicators_3_ref_0, indicators_2_ref_0);
    let mut bool_6: bool = crate::flags::indicators::Indicators::ne(indicators_1_ref_0, indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_548() {
//    rusty_monitor::set_test_id(548);
    let mut str_0: &str = "Icons";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 8usize;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut str_1: &str = "pls";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut str_2: &str = "Older";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 360usize;
    let mut bool_16: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_16, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_17: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_17};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 0usize;
    let mut bool_18: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_18, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_3: usize = 80usize;
    let mut bool_19: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_19, depth: usize_3};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_3, dir_grouping: dirgrouping_3};
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_4: usize = 35usize;
    let mut bool_20: bool = true;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_20, depth: usize_4};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_3: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_105() {
//    rusty_monitor::set_test_id(105);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut links_1: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut bool_0: bool = crate::flags::indicators::Indicators::eq(indicators_1_ref_0, indicators_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut links_1_ref_0: &crate::color::theme::Links = &mut links_1;
//    panic!("From RustyUnit with love");
}
}