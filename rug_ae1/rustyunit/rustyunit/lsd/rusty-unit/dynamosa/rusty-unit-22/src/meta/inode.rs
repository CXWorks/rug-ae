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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut usize_0: usize = 9788usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut str_0: &str = "7Kwxs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_1: usize = 7834usize;
    let mut bool_4: bool = false;
    let mut str_1: &str = "eoPB66bHD";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_5: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_2: usize = 6781usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_5, depth: option_4};
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_6: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_2: &str = "Y0ln7j";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_7: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_7};
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_19, user_write: bool_18, user_execute: bool_17, group_read: bool_16, group_write: bool_15, group_execute: bool_14, other_read: bool_13, other_write: bool_12, other_execute: bool_11, sticky: bool_10, setgid: bool_9, setuid: bool_8};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_11: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_12: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_13: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_13, theme: option_12, separator: option_10};
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_20: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_20);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_21: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_21};
    let mut str_3: &str = "R40z5qKeEYU";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_22: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_22);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 131u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut u64_1: u64 = 1943u64;
    let mut option_21: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_21};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_22: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_22};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut u64_2: u64 = 1344u64;
    let mut option_23: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_23};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut bool_25: bool = crate::meta::inode::INode::eq(inode_2_ref_0, inode_1_ref_0);
    let mut tuple_0: () = crate::meta::inode::INode::assert_receiver_is_total_eq(inode_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_24, exec: bool_23};
    panic!("From RustyUnit with love");
}
}