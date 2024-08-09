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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_368() {
//    rusty_monitor::set_test_id(368);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_2_ref_0: &crate::meta::indicator::Indicator = &mut indicator_2;
    let mut bool_2: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_3_ref_0: &crate::meta::indicator::Indicator = &mut indicator_3;
    let mut bool_3: bool = true;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut indicator_4: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut indicator_4_ref_0: &crate::meta::indicator::Indicator = &mut indicator_4;
    let mut bool_4: bool = true;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_4};
    let mut indicator_5: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_5_ref_0: &crate::meta::indicator::Indicator = &mut indicator_5;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut indicator_6: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_6_ref_0: &crate::meta::indicator::Indicator = &mut indicator_6;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_7: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_7);
    let mut indicator_7_ref_0: &crate::meta::indicator::Indicator = &mut indicator_7;
    let mut bool_5: bool = true;
    let mut filetype_8: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_5};
    let mut indicator_8: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_8);
    let mut indicator_8_ref_0: &crate::meta::indicator::Indicator = &mut indicator_8;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_340() {
//    rusty_monitor::set_test_id(340);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut str_0: &str = "Links";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_3: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_2: &str = "group-directories-first";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_4: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_5: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_22: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_22, theme: option_21};
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_8, exec: bool_7};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut filetype_7: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_7);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_505() {
//    rusty_monitor::set_test_id(505);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut indicator_0: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_0);
    let mut indicator_0_ref_0: &crate::meta::indicator::Indicator = &mut indicator_0;
    let mut bool_1: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut indicator_1: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_1);
    let mut indicator_1_ref_0: &crate::meta::indicator::Indicator = &mut indicator_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut indicator_2: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_2);
    let mut indicator_2_ref_0: &crate::meta::indicator::Indicator = &mut indicator_2;
    let mut bool_2: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut indicator_3: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_3);
    let mut indicator_3_ref_0: &crate::meta::indicator::Indicator = &mut indicator_3;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut indicator_4: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_4);
    let mut indicator_4_ref_0: &crate::meta::indicator::Indicator = &mut indicator_4;
    let mut bool_3: bool = false;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut indicator_5: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_5);
    let mut indicator_5_ref_0: &crate::meta::indicator::Indicator = &mut indicator_5;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut indicator_6: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::from(filetype_6);
    let mut indicator_6_ref_0: &crate::meta::indicator::Indicator = &mut indicator_6;
    let mut indicator_7: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_6_ref_0);
    let mut indicator_8: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_5_ref_0);
    let mut indicator_9: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_4_ref_0);
    let mut indicator_10: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_3_ref_0);
    let mut indicator_11: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_2_ref_0);
    let mut indicator_12: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_1_ref_0);
    let mut indicator_13: crate::meta::indicator::Indicator = crate::meta::indicator::Indicator::clone(indicator_0_ref_0);
//    panic!("From RustyUnit with love");
}
}