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
#[timeout(30000)]fn rusty_test_678() {
//    rusty_monitor::set_test_id(678);
    let mut u64_0: u64 = 1024u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
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
    let mut option_5: std::option::Option<u64> = std::option::Option::None;
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_5};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut u64_1: u64 = 1073741824u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_6: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_6_ref_0: &crate::meta::links::Links = &mut links_6;
    let mut u64_2: u64 = 1024u64;
    let mut option_7: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_7: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut links_7_ref_0: &crate::meta::links::Links = &mut links_7;
    let mut links_8: crate::meta::links::Links = crate::meta::links::Links::clone(links_7_ref_0);
    let mut links_9: crate::meta::links::Links = crate::meta::links::Links::clone(links_6_ref_0);
    let mut links_10: crate::meta::links::Links = crate::meta::links::Links::clone(links_5_ref_0);
    let mut links_11: crate::meta::links::Links = crate::meta::links::Links::clone(links_4_ref_0);
    let mut links_12: crate::meta::links::Links = crate::meta::links::Links::clone(links_3_ref_0);
    let mut links_13: crate::meta::links::Links = crate::meta::links::Links::clone(links_2_ref_0);
    let mut links_14: crate::meta::links::Links = crate::meta::links::Links::clone(links_1_ref_0);
    let mut links_15: crate::meta::links::Links = crate::meta::links::Links::clone(links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6818() {
//    rusty_monitor::set_test_id(6818);
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 360usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_3: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_6: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_7: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_8: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 40usize;
    let mut option_9: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_2, depth: option_9};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_3: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut option_23: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_23};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_2: u64 = 1099511627776u64;
    let mut option_24: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_24};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    crate::meta::links::Links::render(links_1_ref_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_170() {
//    rusty_monitor::set_test_id(170);
    let mut u64_0: u64 = 1048576u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 0u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
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
    let mut u64_2: u64 = 0u64;
    let mut option_6: std::option::Option<u64> = std::option::Option::Some(u64_2);
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
#[timeout(30000)]fn rusty_test_724() {
//    rusty_monitor::set_test_id(724);
    let mut u64_0: u64 = 1073741824u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 1073741824u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
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
    let mut option_8: std::option::Option<u64> = std::option::Option::None;
    let mut links_8: crate::meta::links::Links = crate::meta::links::Links {nlink: option_8};
    let mut links_8_ref_0: &crate::meta::links::Links = &mut links_8;
    let mut u64_2: u64 = 1073741824u64;
    let mut option_9: std::option::Option<u64> = std::option::Option::Some(u64_2);
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
#[timeout(30000)]fn rusty_test_7419() {
//    rusty_monitor::set_test_id(7419);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1048576u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut tuple_0: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3127() {
//    rusty_monitor::set_test_id(3127);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = false;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
//    panic!("From RustyUnit with love");
}
}