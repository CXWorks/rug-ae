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
	use std::clone::Clone;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_339() {
    rusty_monitor::set_test_id(339);
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::SymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut bool_0: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Group;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_11: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3365() {
    rusty_monitor::set_test_id(3365);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2401usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 4989usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_2: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 8655u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_4: bool = true;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_5: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_5};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::SymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_20, theme: option_19, separator: option_18};
    let mut bool_6: bool = crate::flags::layout::Layout::eq(layout_0_ref_0, layout_2_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_2, invalid: color_1};
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1099() {
    rusty_monitor::set_test_id(1099);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut usize_0: usize = 8372usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut tuple_0: () = crate::flags::layout::Layout::assert_receiver_is_total_eq(layout_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_249() {
    rusty_monitor::set_test_id(249);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut bool_0: bool = crate::flags::layout::Layout::eq(layout_1_ref_0, layout_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::NonFile;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_4_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_230() {
    rusty_monitor::set_test_id(230);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::default();
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut u64_0: u64 = 7317u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_2_ref_0: &flags::layout::Layout = &mut layout_2;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut bool_1: bool = crate::color::Elem::has_suid(elem_1_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4479() {
    rusty_monitor::set_test_id(4479);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_0_ref_0: &flags::layout::Layout = &mut layout_0;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_0: usize = 848usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut str_0: &str = "G";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "24VzMtiXUyN";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut str_2: &str = "T9vI1O0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_3: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_4, theme: option_3, separator: option_2};
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_6: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::clone(layout_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    panic!("From RustyUnit with love");
}
}