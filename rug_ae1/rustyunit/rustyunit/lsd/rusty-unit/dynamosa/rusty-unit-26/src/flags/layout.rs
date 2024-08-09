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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2154usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 1601usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_3: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_16: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_16};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut str_0: &str = "z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut tuple_0: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_2_ref_0);
    let mut option_20: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2895() {
    rusty_monitor::set_test_id(2895);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_3, invalid: color_2};
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_5: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_1: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_322() {
    rusty_monitor::set_test_id(322);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_6: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_155() {
    rusty_monitor::set_test_id(155);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5772() {
    rusty_monitor::set_test_id(5772);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_4, invalid: color_3};
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_5: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_3_ref_0: &flags::layout::Layout = &mut layout_3;
    let mut layout_4: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_3_ref_0);
    let mut layout_4_ref_0: &flags::layout::Layout = &mut layout_4;
    let mut bool_2: bool = crate::flags::layout::Layout::eq(layout_4_ref_0, layout_2_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_3, colors_0_ref_0);
    let mut bool_3: bool = crate::flags::layout::Layout::eq(layout_1_ref_0, layout_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_351() {
    rusty_monitor::set_test_id(351);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_4, invalid: color_3};
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_5: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3449() {
    rusty_monitor::set_test_id(3449);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_5: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_2: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_811() {
    rusty_monitor::set_test_id(811);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_1: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_5: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_2: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_221() {
    rusty_monitor::set_test_id(221);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_0: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_6, invalid: color_5};
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut elem_7: color::Elem = crate::color::Elem::ExecSticky;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_1_ref_0);
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut bool_1: bool = crate::flags::layout::Layout::eq(layout_2_ref_0, layout_0_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_4, broken: color_3, missing_target: color_2};
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    panic!("From RustyUnit with love");
}
}