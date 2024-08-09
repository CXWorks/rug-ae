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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1856() {
    rusty_monitor::set_test_id(1856);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 1usize;
    let mut bool_1: bool = true;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "i";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 12usize;
    let mut bool_3: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_4: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_8: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_5: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_10: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_10, reverse: option_9, dir_grouping: option_8};
    let mut option_11: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_12: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_13: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_14: std::option::Option<usize> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_15, depth: option_14};
    let mut option_16: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_17: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_6: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_23: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_24: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_25: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_25, theme: option_24};
    let mut option_26: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_8: bool = false;
    let mut option_28: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_28, blocks: option_27, color: option_26, date: option_23, dereference: option_22, display: option_21, icons: option_20, ignore_globs: option_19, indicators: option_18, layout: option_17, recursion: option_16, size: option_13, permission: option_12, sorting: option_11, no_symlink: option_7, total_size: option_6, symlink_arrow: option_5, hyperlink: option_4};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_29: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_1_ref_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut indicators_1: crate::flags::indicators::Indicators = std::option::Option::unwrap(option_29);
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::clone(indicators_1_ref_0);
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_73() {
    rusty_monitor::set_test_id(73);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 71usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_4: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 27u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut bool_5: bool = crate::flags::indicators::Indicators::eq(indicators_2_ref_0, indicators_1_ref_0);
    let mut bool_6: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1024() {
    rusty_monitor::set_test_id(1024);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut bool_2: bool = crate::flags::indicators::Indicators::ne(indicators_1_ref_0, indicators_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_7, small: color_6, medium: color_5, large: color_4};
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2552() {
    rusty_monitor::set_test_id(2552);
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut usize_0: usize = 16usize;
    let mut bool_0: bool = true;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 11usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut tuple_0: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_1_ref_0);
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1859() {
    rusty_monitor::set_test_id(1859);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 57usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut u64_0: u64 = 41u64;
    panic!("From RustyUnit with love");
}
}