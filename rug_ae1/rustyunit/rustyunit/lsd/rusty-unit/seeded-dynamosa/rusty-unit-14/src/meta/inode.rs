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
#[timeout(30000)]fn rusty_test_653() {
//    rusty_monitor::set_test_id(653);
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_1: u64 = 0u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut u64_2: u64 = 27u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut option_8: std::option::Option<u64> = std::option::Option::None;
    let mut inode_8: crate::meta::inode::INode = crate::meta::inode::INode {index: option_8};
    let mut inode_8_ref_0: &crate::meta::inode::INode = &mut inode_8;
    let mut option_9: std::option::Option<u64> = std::option::Option::None;
    let mut inode_9: crate::meta::inode::INode = crate::meta::inode::INode {index: option_9};
    let mut inode_9_ref_0: &crate::meta::inode::INode = &mut inode_9;
    let mut bool_0: bool = crate::meta::inode::INode::ne(inode_9_ref_0, inode_8_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::ne(inode_7_ref_0, inode_6_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::ne(inode_5_ref_0, inode_4_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::ne(inode_3_ref_0, inode_2_ref_0);
    let mut bool_4: bool = crate::meta::inode::INode::ne(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_596() {
//    rusty_monitor::set_test_id(596);
    let mut u64_0: u64 = 0u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_1: u64 = 0u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut u64_3: u64 = 1099511627776u64;
    let mut option_4: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_4: u64 = 1099511627776u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut u64_5: u64 = 1024u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_5);
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_6_ref_0);
    let mut inode_8: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_5_ref_0);
    let mut inode_9: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_4_ref_0);
    let mut inode_10: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_3_ref_0);
    let mut inode_11: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_2_ref_0);
    let mut inode_12: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_1_ref_0);
    let mut inode_13: crate::meta::inode::INode = crate::meta::inode::INode::clone(inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1722() {
//    rusty_monitor::set_test_id(1722);
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_6: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut str_0: &str = "gemspec";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 360usize;
    let mut bool_6: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 360usize;
    let mut bool_7: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_10: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_10};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 1073741824u64;
    let mut option_11: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_11};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    crate::meta::inode::INode::render(inode_1_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_469() {
//    rusty_monitor::set_test_id(469);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut u64_0: u64 = 0u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut u64_1: u64 = 1073741824u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
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
    let mut u64_2: u64 = 1024u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_2);
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
#[timeout(30000)]fn rusty_test_2212() {
//    rusty_monitor::set_test_id(2212);
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_6: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut str_0: &str = "gemspec";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_4};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 360usize;
    let mut bool_6: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 360usize;
    let mut bool_7: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_10: std::option::Option<u64> = std::option::Option::None;
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_10};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_11: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_11};
    crate::meta::inode::INode::render(inode_0_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_322() {
//    rusty_monitor::set_test_id(322);
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut inode_0: crate::meta::inode::INode = crate::meta::inode::INode {index: option_0};
    let mut inode_0_ref_0: &crate::meta::inode::INode = &mut inode_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut inode_1: crate::meta::inode::INode = crate::meta::inode::INode {index: option_1};
    let mut inode_1_ref_0: &crate::meta::inode::INode = &mut inode_1;
    let mut u64_1: u64 = 1024u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut inode_2: crate::meta::inode::INode = crate::meta::inode::INode {index: option_2};
    let mut inode_2_ref_0: &crate::meta::inode::INode = &mut inode_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut inode_3: crate::meta::inode::INode = crate::meta::inode::INode {index: option_3};
    let mut inode_3_ref_0: &crate::meta::inode::INode = &mut inode_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut inode_4: crate::meta::inode::INode = crate::meta::inode::INode {index: option_4};
    let mut inode_4_ref_0: &crate::meta::inode::INode = &mut inode_4;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut inode_5: crate::meta::inode::INode = crate::meta::inode::INode {index: option_5};
    let mut inode_5_ref_0: &crate::meta::inode::INode = &mut inode_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut inode_6: crate::meta::inode::INode = crate::meta::inode::INode {index: option_6};
    let mut inode_6_ref_0: &crate::meta::inode::INode = &mut inode_6;
    let mut u64_3: u64 = 1048576u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut inode_7: crate::meta::inode::INode = crate::meta::inode::INode {index: option_7};
    let mut inode_7_ref_0: &crate::meta::inode::INode = &mut inode_7;
    let mut bool_0: bool = crate::meta::inode::INode::eq(inode_7_ref_0, inode_6_ref_0);
    let mut bool_1: bool = crate::meta::inode::INode::eq(inode_5_ref_0, inode_4_ref_0);
    let mut bool_2: bool = crate::meta::inode::INode::eq(inode_3_ref_0, inode_2_ref_0);
    let mut bool_3: bool = crate::meta::inode::INode::eq(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}
}