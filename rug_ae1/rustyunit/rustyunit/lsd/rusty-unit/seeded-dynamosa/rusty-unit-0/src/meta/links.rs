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
#[timeout(30000)]fn rusty_test_416() {
//    rusty_monitor::set_test_id(416);
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
    let mut u64_0: u64 = 1073741824u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_1: u64 = 5u64;
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_311() {
//    rusty_monitor::set_test_id(311);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 1048576u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut u64_2: u64 = 1024u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_3: u64 = 1073741824u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_4: u64 = 74u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_4);
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
#[timeout(30000)]fn rusty_test_6584() {
//    rusty_monitor::set_test_id(6584);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_0: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "k7UZdq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_6: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_10: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_11: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_12: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_14: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 8usize;
    let mut bool_2: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_24: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_24};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_25: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_25};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut option_26: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_27: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut option_28: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_28};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    crate::meta::links::Links::render(links_1_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_584() {
//    rusty_monitor::set_test_id(584);
    let mut u64_0: u64 = 0u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_1: u64 = 1024u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_2: u64 = 1099511627776u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_3: u64 = 1099511627776u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut u64_4: u64 = 1024u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_5: u64 = 1048576u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_5);
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
#[timeout(30000)]fn rusty_test_627() {
//    rusty_monitor::set_test_id(627);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut u64_0: u64 = 2u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 0u64;
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
    let mut u64_4: u64 = 1048576u64;
    let mut option_5: std::option::Option<u64> = std::option::Option::Some(u64_4);
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_5: u64 = 1099511627776u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_5);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_6: u64 = 0u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_6);
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut bool_0: bool = crate::meta::links::Links::eq(links_7_ref_0, links_6_ref_0);
    let mut bool_1: bool = crate::meta::links::Links::eq(links_5_ref_0, links_4_ref_0);
    let mut bool_2: bool = crate::meta::links::Links::eq(links_3_ref_0, links_2_ref_0);
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}
}