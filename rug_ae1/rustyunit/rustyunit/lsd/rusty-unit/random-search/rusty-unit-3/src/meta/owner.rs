use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;

#[derive(Clone, Debug)]
pub struct Owner {
    user: String,
    group: String,
}

impl Owner {
    #[cfg_attr(unix, allow(dead_code))]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }
}

#[cfg(unix)]
impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        use users::{get_group_by_gid, get_user_by_uid};

        let user = match get_user_by_uid(meta.uid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        };

        let group = match get_group_by_gid(meta.gid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        };

        Self { user, group }
    }
}

impl Owner {
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.user.clone(), &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.group.clone(), &Elem::Group)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5401() {
    rusty_monitor::set_test_id(5401);
    let mut str_0: &str = "rBfuABEFy7vtJk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 8usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_0_ref_0);
    let mut usize_1: usize = 80usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut usize_2: usize = 2usize;
    let mut bool_3: bool = false;
    let mut usize_3: usize = 2usize;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut u64_0: u64 = 54u64;
    let mut str_1: &str = "9";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Ad";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut usize_4: usize = 80usize;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut usize_5: usize = 60usize;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut usize_6: usize = 53usize;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut usize_7: usize = 68usize;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut u64_1: u64 = 55u64;
    let mut usize_8: usize = 52usize;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut str_3: &str = "C0gUZmeLrIA";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_21: bool = false;
    let mut usize_9: usize = 20usize;
    let mut usize_10: usize = 22usize;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut usize_11: usize = 22usize;
    let mut bool_24: bool = true;
    let mut usize_12: usize = 18usize;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = true;
    let mut bool_33: bool = true;
    let mut bool_34: bool = true;
    let mut bool_35: bool = false;
    let mut bool_36: bool = true;
    let mut bool_37: bool = true;
    let mut bool_38: bool = true;
    let mut bool_39: bool = true;
    let mut bool_40: bool = true;
    let mut usize_13: usize = 68usize;
    let mut bool_41: bool = false;
    let mut bool_42: bool = true;
    let mut usize_14: usize = 25usize;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_15: usize = 65usize;
    let mut bool_43: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_43, depth: usize_15};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_2);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut bool_44: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_44);
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_8: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_45: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_45);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_46: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_46);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_47: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_47);
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_18, blocks: option_17, color: option_16, date: option_15, dereference: option_14, display: option_13, icons: option_12, ignore_globs: option_11, indicators: option_10, layout: option_9, recursion: option_8, size: option_7, permission: option_6, sorting: option_5, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_48: bool = false;
    let mut bool_49: bool = true;
    let mut bool_50: bool = false;
    let mut bool_51: bool = true;
    let mut bool_52: bool = true;
    let mut bool_53: bool = true;
    let mut bool_54: bool = true;
    let mut bool_55: bool = true;
    let mut bool_56: bool = true;
    let mut bool_57: bool = true;
    let mut bool_58: bool = false;
    let mut bool_59: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_59, user_write: bool_58, user_execute: bool_57, group_read: bool_56, group_write: bool_55, group_execute: bool_54, other_read: bool_53, other_write: bool_52, other_execute: bool_51, sticky: bool_50, setgid: bool_49, setuid: bool_48};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_19: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    panic!("From RustyUnit with love");
}
}