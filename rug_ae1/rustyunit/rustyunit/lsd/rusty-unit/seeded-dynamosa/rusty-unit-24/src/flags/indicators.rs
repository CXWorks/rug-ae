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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6377() {
//    rusty_monitor::set_test_id(6377);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut str_0: &str = "so";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "fi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "wav";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 40usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_2_ref_0);
    let mut str_3: &str = "CPWggSJ7JD";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut bool_0: bool = crate::flags::indicators::Indicators::eq(indicators_1_ref_0, indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_207() {
//    rusty_monitor::set_test_id(207);
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
#[timeout(30000)]fn rusty_test_489() {
//    rusty_monitor::set_test_id(489);
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
#[timeout(30000)]fn rusty_test_453() {
//    rusty_monitor::set_test_id(453);
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
#[timeout(30000)]fn rusty_test_572() {
//    rusty_monitor::set_test_id(572);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 0usize;
    let mut bool_2: bool = true;
    let mut usize_1: usize = 120usize;
    let mut bool_3: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_2: usize = 80usize;
    let mut bool_4: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_2};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_1: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 98usize;
    let mut bool_5: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut str_0: &str = "file_type";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_4: usize = 30usize;
    let mut bool_6: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_4};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_3);
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_3);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_5: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}