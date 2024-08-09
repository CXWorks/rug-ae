use crate::color::{ColoredString, Colors};
use crate::flags::Flags;
use crate::meta::FileType;

#[derive(Clone, Debug)]
pub struct Indicator(&'static str);

impl From<FileType> for Indicator {
    fn from(file_type: FileType) -> Self {
        let res = match file_type {
            FileType::Directory { .. } => "/",
            FileType::File { exec: true, .. } => "*",
            FileType::Pipe => "|",
            FileType::Socket => "=",
            FileType::SymLink { .. } => "@",
            _ => "",
        };

        Indicator(res)
    }
}

impl Indicator {
    pub fn render(&self, flags: &Flags) -> ColoredString {
        if flags.display_indicators.0 {
            ColoredString::new(Colors::default_style(), self.0.to_string())
        } else {
            ColoredString::new(Colors::default_style(), "".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Indicator;
    use crate::flags::{Flags, Indicators};
    use crate::meta::FileType;

    #[test]
    fn test_directory_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::Directory { uid: false });

        assert_eq!("/", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_executable_file_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::File {
            uid: false,
            exec: true,
        });

        assert_eq!("*", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_socket_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::Socket);

        assert_eq!("=", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_symlink_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        let file_type = Indicator::from(FileType::SymLink { is_dir: false });
        assert_eq!("@", file_type.render(&flags).to_string().as_str());

        let file_type = Indicator::from(FileType::SymLink { is_dir: true });
        assert_eq!("@", file_type.render(&flags).to_string().as_str());
    }

    #[test]
    fn test_not_represented_indicator() {
        let mut flags = Flags::default();
        flags.display_indicators = Indicators(true);

        // The File type doesn't have any indicator
        let file_type = Indicator::from(FileType::File {
            exec: false,
            uid: false,
        });

        assert_eq!("", file_type.render(&flags).to_string().as_str());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::convert::From;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2918() {
    rusty_monitor::set_test_id(2918);
    let mut usize_0: usize = 97usize;
    let mut bool_0: bool = true;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_3: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_22: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_22, theme: option_21};
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 53u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 72usize;
    let mut bool_5: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut u64_1: u64 = 39u64;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_7, exec: bool_6};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3334() {
    rusty_monitor::set_test_id(3334);
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_1: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_2: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::HourOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_4, no_uid: color_3};
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_3: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::Context;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_9_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_8, uid_no_exec: color_7, exec_no_uid: color_6, no_exec_no_uid: color_5};
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut u64_0: u64 = 46u64;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut bool_16: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3873() {
    rusty_monitor::set_test_id(3873);
    let mut usize_0: usize = 72usize;
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut str_0: &str = "Gp7RWSskanky81";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 18usize;
    let mut bool_3: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_1: &str = "aHG1lC9UJVk0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_4: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_4};
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_16, user_write: bool_15, user_execute: bool_14, group_read: bool_13, group_write: bool_12, group_execute: bool_11, other_read: bool_10, other_write: bool_9, other_execute: bool_8, sticky: bool_7, setgid: bool_6, setuid: bool_5};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_18: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 3u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_17: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_17};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3525() {
    rusty_monitor::set_test_id(3525);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut usize_0: usize = 23usize;
    let mut bool_2: bool = false;
    let mut usize_1: usize = 67usize;
    let mut bool_3: bool = false;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_5: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "bozKyn2l6f";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_7, exec: bool_6};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_1: &str = "TDf2XQjwbSdEFz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_21: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_23, blocks: option_22, color: option_21, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1333() {
    rusty_monitor::set_test_id(1333);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_24: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut dirgrouping_0_ref_0: &flags::sorting::DirGrouping = &mut dirgrouping_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_24};
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3030() {
    rusty_monitor::set_test_id(3030);
    let mut usize_0: usize = 53usize;
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_5: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_6: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_6, theme: option_5, separator: option_4};
    let mut option_7: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_12: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_12, theme: option_11};
    let mut option_13: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_16: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_20: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_22: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_23: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_24: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_27: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_28: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_29: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_31: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_32: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_33: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_33, blocks: option_32, color: option_31, date: option_30, dereference: option_29, display: option_28, icons: option_27, ignore_globs: option_26, indicators: option_25, layout: option_24, recursion: option_23, size: option_22, permission: option_21, sorting: option_20, no_symlink: option_19, total_size: option_18, symlink_arrow: option_17, hyperlink: option_16};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_6: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_6};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut bool_7: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_7};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_2_ref_0: &flags::icons::IconOption = &mut iconoption_2;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_4, small: color_3, medium: color_2, large: color_1};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    panic!("From RustyUnit with love");
}
}