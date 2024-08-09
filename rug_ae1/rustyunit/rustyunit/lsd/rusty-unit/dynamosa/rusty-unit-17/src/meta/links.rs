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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut usize_0: usize = 7631usize;
    let mut bool_0: bool = false;
    let mut str_0: &str = "oS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_1: usize = 87usize;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_7: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut option_10: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_8: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut bool_9: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut option_14: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_10: bool = false;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_10);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_16: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_16, reverse: option_15, dir_grouping: option_14};
    let mut option_17: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_18: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_19: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_2: usize = 2691usize;
    let mut option_20: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_21, depth: option_20};
    let mut option_22: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_23: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_11: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_11);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_27: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_28: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_28, theme: option_27, separator: option_26};
    let mut option_29: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_30: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_12: bool = false;
    let mut option_31: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut option_32: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_33: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_34: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_35: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_35, blocks: option_34, color: option_33, date: option_32, dereference: option_31, display: option_30, icons: option_29, ignore_globs: option_25, indicators: option_24, layout: option_23, recursion: option_22, size: option_19, permission: option_18, sorting: option_17, no_symlink: option_13, total_size: option_12, symlink_arrow: option_11, hyperlink: option_10};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 2977u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_1: &str = "jDIqfU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_13: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_13};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_14: bool = false;
    let mut option_36: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_36};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut tuple_0: () = crate::meta::links::Links::assert_receiver_is_total_eq(links_0_ref_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_14};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    panic!("From RustyUnit with love");
}
}