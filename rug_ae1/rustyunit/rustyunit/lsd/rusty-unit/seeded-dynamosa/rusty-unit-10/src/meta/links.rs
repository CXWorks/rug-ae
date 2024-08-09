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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_592() {
//    rusty_monitor::set_test_id(592);
    let mut u64_0: u64 = 0u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_1: u64 = 1024u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_2: u64 = 1024u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
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
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut bool_0: bool = crate::meta::links::Links::ne(links_7_ref_0, links_6_ref_0);
    let mut bool_1: bool = crate::meta::links::Links::ne(links_5_ref_0, links_4_ref_0);
    let mut bool_2: bool = crate::meta::links::Links::ne(links_3_ref_0, links_2_ref_0);
    let mut bool_3: bool = crate::meta::links::Links::ne(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5294() {
//    rusty_monitor::set_test_id(5294);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 0usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 73u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 2usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut bool_3: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut u64_1: u64 = 1073741824u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_21: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_21};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_22: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_22};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    crate::meta::links::Links::render(links_1_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_541() {
//    rusty_monitor::set_test_id(541);
    let mut u64_0: u64 = 1024u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_1: u64 = 0u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut u64_3: u64 = 0u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_4: u64 = 56u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut bool_0: bool = crate::meta::links::Links::eq(links_7_ref_0, links_6_ref_0);
    let mut bool_1: bool = crate::meta::links::Links::eq(links_5_ref_0, links_4_ref_0);
    let mut bool_2: bool = crate::meta::links::Links::eq(links_3_ref_0, links_2_ref_0);
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_638() {
//    rusty_monitor::set_test_id(638);
    let mut u64_0: u64 = 91u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_1: u64 = 1099511627776u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_2: u64 = 1048576u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
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
#[timeout(30000)]fn rusty_test_524() {
//    rusty_monitor::set_test_id(524);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut u64_0: u64 = 40u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_1: u64 = 1024u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut tuple_0: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_7_ref_0);
    let mut tuple_1: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_6_ref_0);
    let mut tuple_2: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_5_ref_0);
    let mut tuple_3: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_4_ref_0);
    let mut tuple_4: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_3_ref_0);
    let mut tuple_5: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_2_ref_0);
    let mut tuple_6: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_1_ref_0);
    let mut tuple_7: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_0_ref_0);
//    panic!("From RustyUnit with love");
}
}