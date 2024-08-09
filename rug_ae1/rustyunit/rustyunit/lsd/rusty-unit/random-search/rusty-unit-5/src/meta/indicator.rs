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
fn rusty_test_5436() {
    rusty_monitor::set_test_id(5436);
    let mut usize_0: usize = 50usize;
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_2: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_14: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_15: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_15, theme: option_14};
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 15usize;
    let mut bool_4: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_19: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_23: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut option_25: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_25, reverse: option_24, dir_grouping: option_23};
    let mut option_26: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_27: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_28: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_29: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_30: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_31: std::option::Option<bool> = std::option::Option::None;
    let mut option_32: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_33: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_34: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_35: std::option::Option<bool> = std::option::Option::None;
    let mut option_36: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_37: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_38: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_38, theme: option_37};
    let mut option_39: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_40: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_41: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_41, blocks: option_40, color: option_39, date: option_36, dereference: option_35, display: option_34, icons: option_33, ignore_globs: option_32, indicators: option_31, layout: option_30, recursion: option_29, size: option_28, permission: option_27, sorting: option_26, no_symlink: option_22, total_size: option_21, symlink_arrow: option_20, hyperlink: option_19};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_0: u64 = 96u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_7: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_7};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4756() {
    rusty_monitor::set_test_id(4756);
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut option_2: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_13: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_15: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_3: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_4: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_4};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_1};
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_2, invalid: color_1};
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut theme_4: icon::Theme = crate::icon::Theme::Unicode;
    let mut elem_5: color::Elem = crate::color::Elem::TreeEdge;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_15, reverse: option_14, dir_grouping: option_13};
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::CharDevice;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4867() {
    rusty_monitor::set_test_id(4867);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_0: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_0};
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
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::Context;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut bool_1: bool = true;
    let mut elem_9: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_9_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_9, no_uid: color_8};
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_10: color::Elem = crate::color::Elem::Exec;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_10: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_10_ref_0);
    let mut theme_11: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_11_ref_0: &crate::color::theme::Theme = &mut theme_11;
    let mut elem_11: color::Elem = crate::color::Elem::Socket;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut color_11: crossterm::style::Color = crate::color::Elem::get_color(elem_11_ref_0, theme_11_ref_0);
    let mut theme_12: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_12_ref_0: &crate::color::theme::Theme = &mut theme_12;
    let mut elem_12: color::Elem = crate::color::Elem::Pipe;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut color_12: crossterm::style::Color = crate::color::Elem::get_color(elem_12_ref_0, theme_12_ref_0);
    let mut theme_13: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_13_ref_0: &crate::color::theme::Theme = &mut theme_13;
    let mut elem_13: color::Elem = crate::color::Elem::Octal;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut color_13: crossterm::style::Color = crate::color::Elem::get_color(elem_13_ref_0, theme_13_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_13, uid_no_exec: color_12, exec_no_uid: color_11, no_exec_no_uid: color_10};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut elem_14: color::Elem = crate::color::Elem::NoAccess;
    let mut filetype_1: crate::color::theme::FileType = crate::color::theme::FileType {file: file_0, dir: dir_0, pipe: color_7, symlink: symlink_0, block_device: color_3, char_device: color_2, socket: color_1, special: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_963() {
    rusty_monitor::set_test_id(963);
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 98usize;
    let mut option_10: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_2: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_672() {
    rusty_monitor::set_test_id(672);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_2: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_18, exec: bool_17};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_16, user_write: bool_15, user_execute: bool_14, group_read: bool_13, group_write: bool_12, group_execute: bool_11, other_read: bool_10, other_write: bool_9, other_execute: bool_8, sticky: bool_7, setgid: bool_6, setuid: bool_5};
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 87usize;
    let mut bool_13: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_13, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut str_0: &str = "w73Zo";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_4: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_15, exec: bool_14};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut recursion_0_ref_0: &crate::flags::recursion::Recursion = &mut recursion_0;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}
}