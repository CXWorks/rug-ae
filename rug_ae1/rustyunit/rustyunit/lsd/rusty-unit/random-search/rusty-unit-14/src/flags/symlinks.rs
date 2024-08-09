//! This module defines the [NoSymlink] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to follow symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct NoSymlink(pub bool);

impl Configurable<Self> for NoSymlink {
    /// Get a potential `NoSymlink` value from [ArgMatches].
    ///
    /// If the "no-symlink" argument is passed, this returns a `NoSymlink` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("no-symlink") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `NoSymlink` value from a [Config].
    ///
    /// If the `Config::no-symlink` has value,
    /// this returns it as the value of the `NoSymlink`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.no_symlink.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::NoSymlink;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--no-symlink"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.no_symlink = Some(true);
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.no_symlink = Some(false);
        assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2824() {
    rusty_monitor::set_test_id(2824);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut bool_2: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_1_ref_0, nosymlink_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut app_0: clap::App = crate::app::build();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2717() {
    rusty_monitor::set_test_id(2717);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 29usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_2: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut option_21: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_25: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_26: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_27: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_2);
    let mut option_28: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_29: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_30: std::option::Option<bool> = std::option::Option::None;
    let mut option_31: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_32: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_33: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut option_35: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_36: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_37: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_38: std::option::Option<bool> = std::option::Option::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_38, blocks: option_37, color: option_36, date: option_35, dereference: option_34, display: option_33, icons: option_32, ignore_globs: option_31, indicators: option_30, layout: option_29, recursion: option_28, size: option_27, permission: option_26, sorting: option_25, no_symlink: option_24, total_size: option_23, symlink_arrow: option_22, hyperlink: option_21};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_0: u64 = 3u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut tuple_0: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_1_ref_0);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_728() {
    rusty_monitor::set_test_id(728);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 55usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_22, theme: option_21};
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut bool_3: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut bool_4: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_3_ref_0, nosymlink_2_ref_0);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3239() {
    rusty_monitor::set_test_id(3239);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_6, broken: color_5, missing_target: color_4};
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_0: bool = false;
    let mut elem_7: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_9: color::Elem = crate::color::Elem::DayOld;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_9_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_9, no_uid: color_8};
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_10: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_10: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_10_ref_0);
    let mut theme_11: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_11_ref_0: &crate::color::theme::Theme = &mut theme_11;
    let mut elem_11: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_11: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_11_ref_0);
    let mut theme_12: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_12_ref_0: &crate::color::theme::Theme = &mut theme_12;
    let mut elem_12: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut color_12: crossterm::style::Color = crate::color::Elem::get_color(elem_12_ref_0, theme_12_ref_0);
    let mut theme_13: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_13_ref_0: &crate::color::theme::Theme = &mut theme_13;
    let mut elem_13: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut color_13: crossterm::style::Color = crate::color::Elem::get_color(elem_13_ref_0, theme_13_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_13, uid_no_exec: color_12, exec_no_uid: color_11, no_exec_no_uid: color_10};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<crate::flags::symlinks::NoSymlink> = crate::flags::symlinks::NoSymlink::from_config(config_0_ref_0);
    let mut filetype_0: crate::color::theme::FileType = crate::color::theme::FileType {file: file_0, dir: dir_0, pipe: color_7, symlink: symlink_0, block_device: color_3, char_device: color_2, socket: color_1, special: color_0};
    panic!("From RustyUnit with love");
}
}