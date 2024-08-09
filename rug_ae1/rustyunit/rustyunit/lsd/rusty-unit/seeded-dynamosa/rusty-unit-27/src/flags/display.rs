//! This module defines the [Display] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
    All,
    AlmostAll,
    DirectoryOnly,
    VisibleOnly,
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [ArgMatches].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("directory-only") {
            Some(Self::DirectoryOnly)
        } else if matches.is_present("almost-all") {
            Some(Self::AlmostAll)
        } else if matches.is_present("all") {
            Some(Self::All)
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the `Config::display` has value and is one of
    /// "all", "almost-all", "directory-only" or `visible-only`,
    /// this returns the corresponding `Display` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.display
    }
}

/// The default value for `Display` is [Display::VisibleOnly].
impl Default for Display {
    fn default() -> Self {
        Display::VisibleOnly
    }
}

#[cfg(test)]
mod test {
    use super::Display;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_all() {
        let argv = vec!["lsd", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Display::All), Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_almost_all() {
        let argv = vec!["lsd", "--almost-all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::AlmostAll),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_directory_only() {
        let argv = vec!["lsd", "--directory-only"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::DirectoryOnly),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Display::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::All);
        assert_eq!(Some(Display::All), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_almost_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::AlmostAll);
        assert_eq!(Some(Display::AlmostAll), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_directory_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::DirectoryOnly);
        assert_eq!(Some(Display::DirectoryOnly), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_visible_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::VisibleOnly);
        assert_eq!(Some(Display::VisibleOnly), Display::from_config(&c));
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
#[timeout(30000)]fn rusty_test_669() {
//    rusty_monitor::set_test_id(669);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::default();
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::default();
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::All;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::default();
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::default();
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::default();
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut tuple_0: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_10_ref_0);
    let mut tuple_1: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_9_ref_0);
    let mut tuple_2: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_8_ref_0);
    let mut tuple_3: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_7_ref_0);
    let mut tuple_4: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_6_ref_0);
    let mut tuple_5: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_5_ref_0);
    let mut tuple_6: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_4_ref_0);
    let mut tuple_7: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_3_ref_0);
    let mut tuple_8: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_2_ref_0);
    let mut tuple_9: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_1_ref_0);
    let mut tuple_10: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_219() {
//    rusty_monitor::set_test_id(219);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::All;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::All;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::All;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::default();
    let mut display_11_ref_0: &flags::display::Display = &mut display_11;
    let mut display_12: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_12_ref_0: &flags::display::Display = &mut display_12;
    let mut display_13: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_13_ref_0: &flags::display::Display = &mut display_13;
    let mut bool_0: bool = crate::flags::display::Display::eq(display_13_ref_0, display_12_ref_0);
    let mut bool_1: bool = crate::flags::display::Display::eq(display_11_ref_0, display_10_ref_0);
    let mut bool_2: bool = crate::flags::display::Display::eq(display_9_ref_0, display_8_ref_0);
    let mut bool_3: bool = crate::flags::display::Display::eq(display_7_ref_0, display_6_ref_0);
    let mut bool_4: bool = crate::flags::display::Display::eq(display_5_ref_0, display_4_ref_0);
    let mut bool_5: bool = crate::flags::display::Display::eq(display_3_ref_0, display_2_ref_0);
    let mut bool_6: bool = crate::flags::display::Display::eq(display_1_ref_0, display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_486() {
//    rusty_monitor::set_test_id(486);
    let mut usize_0: usize = 80usize;
    let mut bool_0: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 6usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_1: u64 = 93u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut str_0: &str = "ico";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 2usize;
    let mut bool_2: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_2);
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_2);
    let mut option_5: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_6: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_2);
    let mut bool_3: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_10: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut option_11: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_11, theme: option_10, separator: option_9};
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut bool_4: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_19: std::option::Option<flags::display::Display> = crate::flags::display::Display::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}