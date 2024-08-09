use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Links {
    nlink: Option<u64>,
}

impl<'a> From<&'a Metadata> for Links {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let nlink = meta.nlink();

        Self { nlink: Some(nlink) }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { nlink: None }
    }
}

impl Links {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.nlink {
            Some(i) => colors.colorize(i.to_string(), &Elem::Links { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::Links { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::Links;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_hardlinks_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let links = Links::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(links.nlink.is_some());
        #[cfg(windows)]
        assert!(links.nlink.is_none());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1824() {
//    rusty_monitor::set_test_id(1824);
    let mut bool_0: bool = true;
    let mut str_0: &str = "Pipe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "cshtml";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2kgrDKhL";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "short";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "ipynb";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Sort the directories then the files";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "Permissions";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "%F";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_8_ref_0);
    let mut result_1: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_7_ref_0);
    let mut result_2: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_6_ref_0);
    let mut result_3: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_5_ref_0);
    let mut result_4: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_4_ref_0);
    let mut result_5: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_3_ref_0);
    let mut result_6: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_2_ref_0);
    let mut result_7: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_1_ref_0);
    let mut result_8: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2612() {
//    rusty_monitor::set_test_id(2612);
    let mut bool_0: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_0, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut tuple_0: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_230() {
//    rusty_monitor::set_test_id(230);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_0: u64 = 1099511627776u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 61u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_2: u64 = 0u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut u64_3: u64 = 1099511627776u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_4: u64 = 72u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links::clone(links_6_ref_0);
    let mut links_8: crate::meta::links::Links = crate::meta::links::Links::clone(links_5_ref_0);
    let mut links_9: crate::meta::links::Links = crate::meta::links::Links::clone(links_4_ref_0);
    let mut links_10: crate::meta::links::Links = crate::meta::links::Links::clone(links_3_ref_0);
    let mut links_11: crate::meta::links::Links = crate::meta::links::Links::clone(links_2_ref_0);
    let mut links_12: crate::meta::links::Links = crate::meta::links::Links::clone(links_1_ref_0);
    let mut links_13: crate::meta::links::Links = crate::meta::links::Links::clone(links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8650() {
//    rusty_monitor::set_test_id(8650);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut tuple_0: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7875() {
//    rusty_monitor::set_test_id(7875);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_16: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_16};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_6: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_8: color::Elem = crate::color::Elem::CharDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    crate::meta::links::Links::render(links_0_ref_0, colors_0_ref_0);
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7296() {
//    rusty_monitor::set_test_id(7296);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::Read;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_498() {
//    rusty_monitor::set_test_id(498);
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 58u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_2: u64 = 1048576u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut option_8: std::option::Option<u64> = std::option::Option::None;
    let mut links_8: crate::meta::links::Links = crate::meta::links::Links {nlink: option_8};
    let mut links_8_ref_0: &crate::meta::links::Links = &mut links_8;
    let mut option_9: std::option::Option<u64> = std::option::Option::None;
    let mut links_9: crate::meta::links::Links = crate::meta::links::Links {nlink: option_9};
    let mut links_9_ref_0: &crate::meta::links::Links = &mut links_9;
    let mut bool_0: bool = crate::meta::links::Links::eq(links_9_ref_0, links_8_ref_0);
    let mut bool_1: bool = crate::meta::links::Links::eq(links_7_ref_0, links_6_ref_0);
    let mut bool_2: bool = crate::meta::links::Links::eq(links_5_ref_0, links_4_ref_0);
    let mut bool_3: bool = crate::meta::links::Links::eq(links_3_ref_0, links_2_ref_0);
    let mut bool_4: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_288() {
//    rusty_monitor::set_test_id(288);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 1099511627776u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut u64_2: u64 = 0u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_3: u64 = 0u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_4: u64 = 1024u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut bool_0: bool = crate::meta::links::Links::ne(links_7_ref_0, links_6_ref_0);
    let mut bool_1: bool = crate::meta::links::Links::ne(links_5_ref_0, links_4_ref_0);
    let mut bool_2: bool = crate::meta::links::Links::ne(links_3_ref_0, links_2_ref_0);
    let mut bool_3: bool = crate::meta::links::Links::ne(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}
}