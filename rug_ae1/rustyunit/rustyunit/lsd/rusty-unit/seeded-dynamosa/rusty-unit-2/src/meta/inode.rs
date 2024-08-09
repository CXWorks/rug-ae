use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct INode {
    index: Option<u64>,
}

impl<'a> From<&'a Metadata> for INode {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        let index = meta.ino();

        Self { index: Some(index) }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { index: None }
    }
}

impl INode {
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.index {
            Some(i) => colors.colorize(i.to_string(), &Elem::INode { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::INode { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::INode;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_inode_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("inode.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let inode = INode::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(inode.index.is_some());
        #[cfg(windows)]
        assert!(inode.index.is_none());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_774() {
//    rusty_monitor::set_test_id(774);
    let mut u64_0: u64 = 42u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_1: u64 = 1048576u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut u64_2: u64 = 1024u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut u64_3: u64 = 1099511627776u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut tuple_0: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_6_ref_0);
    let mut tuple_1: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_5_ref_0);
    let mut tuple_2: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_4_ref_0);
    let mut tuple_3: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_3_ref_0);
    let mut tuple_4: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_2_ref_0);
    let mut tuple_5: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_1_ref_0);
    let mut tuple_6: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6525() {
//    rusty_monitor::set_test_id(6525);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_11};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_1: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "FMtJ90S9bW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut u64_1: u64 = 7u64;
    let mut option_21: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_21};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_22: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_22};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_23: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_23};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_2: u64 = 39u64;
    let mut option_24: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_24};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    crate::meta::inode::INode::render(inode_3_ref_0, colors_2_ref_0);
    crate::meta::inode::INode::render(inode_2_ref_0, colors_1_ref_0);
    crate::meta::inode::INode::render(inode_1_ref_0, colors_0_ref_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_61() {
//    rusty_monitor::set_test_id(61);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut theme_0_ref_0: &icon::Theme = &mut theme_0;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut theme_1: icon::Theme = crate::icon::Theme::Unicode;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_13: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_207() {
//    rusty_monitor::set_test_id(207);
    let mut u64_0: u64 = 8u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_1: u64 = 1048576u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut u64_2: u64 = 75u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut bool_0: bool = crate::meta::inode::INode::ne(inode_7_ref_0, inode_6_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::ne(inode_5_ref_0, inode_4_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::ne(inode_3_ref_0, inode_2_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::ne(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_211() {
//    rusty_monitor::set_test_id(211);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_0: u64 = 1024u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut u64_1: u64 = 1099511627776u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut bool_0: bool = crate::meta::inode::INode::eq(inode_7_ref_0, inode_6_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::eq(inode_5_ref_0, inode_4_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::eq(inode_3_ref_0, inode_2_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::eq(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_515() {
//    rusty_monitor::set_test_id(515);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut u64_0: u64 = 1099511627776u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_1: u64 = 1048576u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut inode_8: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_7_ref_0);
    let mut inode_9: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_6_ref_0);
    let mut inode_10: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_5_ref_0);
    let mut inode_11: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_4_ref_0);
    let mut inode_12: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_3_ref_0);
    let mut inode_13: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_2_ref_0);
    let mut inode_14: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_1_ref_0);
    let mut inode_15: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_0_ref_0);
//    panic!("From RustyUnit with love");
}
}