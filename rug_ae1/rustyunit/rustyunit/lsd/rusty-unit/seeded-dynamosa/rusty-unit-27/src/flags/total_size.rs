//! This module defines the [TotalSize] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to show the total size for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TotalSize(pub bool);

impl Configurable<Self> for TotalSize {
    /// Get a potential `TotalSize` value from [ArgMatches].
    ///
    /// If the "total-size" argument is passed, this returns a `TotalSize` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("total-size") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `TotalSize` value from a [Config].
    ///
    /// If the `Config::total-size` has value,
    /// this returns it as the value of the `TotalSize`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.total_size.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::TotalSize;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--total-size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(TotalSize(true)), TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, TotalSize::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.total_size = Some(true);
        assert_eq!(Some(TotalSize(true)), TotalSize::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.total_size = Some(false);
        assert_eq!(Some(TotalSize(false)), TotalSize::from_config(&c));
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
#[timeout(30000)]fn rusty_test_563() {
//    rusty_monitor::set_test_id(563);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut totalsize_2: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_2_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_2;
    let mut totalsize_3: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_3;
    let mut totalsize_4: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_4_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_4;
    let mut totalsize_5: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_5_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_5;
    let mut totalsize_6: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_6_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_6;
    let mut totalsize_7: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_7_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_7;
    let mut totalsize_8: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_8_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_8;
    let mut totalsize_9: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_9_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_9;
    let mut totalsize_10: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_10_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_10;
    let mut totalsize_11: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_11_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_11;
    let mut totalsize_12: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_12_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_12;
    let mut totalsize_13: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_13_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_13;
    let mut bool_0: bool = crate::flags::total_size::TotalSize::eq(totalsize_13_ref_0, totalsize_12_ref_0);
    let mut bool_1: bool = crate::flags::total_size::TotalSize::eq(totalsize_11_ref_0, totalsize_10_ref_0);
    let mut bool_2: bool = crate::flags::total_size::TotalSize::eq(totalsize_9_ref_0, totalsize_8_ref_0);
    let mut bool_3: bool = crate::flags::total_size::TotalSize::eq(totalsize_7_ref_0, totalsize_6_ref_0);
    let mut bool_4: bool = crate::flags::total_size::TotalSize::eq(totalsize_5_ref_0, totalsize_4_ref_0);
    let mut bool_5: bool = crate::flags::total_size::TotalSize::eq(totalsize_3_ref_0, totalsize_2_ref_0);
    let mut bool_6: bool = crate::flags::total_size::TotalSize::eq(totalsize_1_ref_0, totalsize_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_468() {
//    rusty_monitor::set_test_id(468);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut totalsize_2: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_2_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_2;
    let mut totalsize_3: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_3;
    let mut totalsize_4: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_4_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_4;
    let mut totalsize_5: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_5_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_5;
    let mut totalsize_6: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_6_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_6;
    let mut totalsize_7: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_7_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_7;
    let mut totalsize_8: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_8_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_8;
    let mut totalsize_9: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_9_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_9;
    let mut totalsize_10: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_10_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_10;
    let mut tuple_0: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_10_ref_0);
    let mut tuple_1: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_9_ref_0);
    let mut tuple_2: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_8_ref_0);
    let mut tuple_3: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_7_ref_0);
    let mut tuple_4: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_6_ref_0);
    let mut tuple_5: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_5_ref_0);
    let mut tuple_6: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_4_ref_0);
    let mut tuple_7: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_3_ref_0);
    let mut tuple_8: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_2_ref_0);
    let mut tuple_9: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_1_ref_0);
    let mut tuple_10: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_398() {
//    rusty_monitor::set_test_id(398);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 2usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 84usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "hour_old";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_2: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_4, reverse: option_3, dir_grouping: option_2};
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_2);
    let mut option_8: std::option::Option<usize> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_2);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_23: std::option::Option<crate::flags::total_size::TotalSize> = crate::flags::total_size::TotalSize::from_config(config_1_ref_0);
    let mut option_24: std::option::Option<crate::flags::total_size::TotalSize> = crate::flags::total_size::TotalSize::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_836() {
//    rusty_monitor::set_test_id(836);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 7usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut totalsize_2: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_2_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_2;
    let mut totalsize_4: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_3;
    let mut totalsize_5: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_4_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_4;
    let mut totalsize_6: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_5_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_5;
    let mut bool_1: bool = crate::flags::total_size::TotalSize::ne(totalsize_2_ref_0, totalsize_0_ref_0);
    let mut bool_2: bool = crate::flags::total_size::TotalSize::ne(totalsize_1_ref_0, totalsize_4_ref_0);
    let mut bool_3: bool = crate::flags::total_size::TotalSize::ne(totalsize_3_ref_0, totalsize_5_ref_0);
//    panic!("From RustyUnit with love");
}
}