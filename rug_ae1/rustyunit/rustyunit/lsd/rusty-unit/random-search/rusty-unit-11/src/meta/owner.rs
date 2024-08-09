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
fn rusty_test_5366() {
    rusty_monitor::set_test_id(5366);
    let mut str_0: &str = "xZUavyRHgY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "OL8oLB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 82usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut usize_1: usize = 38usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut usize_2: usize = 5usize;
    let mut bool_13: bool = true;
    let mut usize_3: usize = 80usize;
    let mut bool_14: bool = false;
    let mut u64_0: u64 = 26u64;
    let mut u64_1: u64 = 20u64;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = false;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = true;
    let mut bool_32: bool = false;
    let mut bool_33: bool = false;
    let mut bool_34: bool = true;
    let mut bool_35: bool = true;
    let mut bool_36: bool = false;
    let mut bool_37: bool = true;
    let mut bool_38: bool = true;
    let mut u64_2: u64 = 85u64;
    let mut bool_39: bool = true;
    let mut usize_4: usize = 17usize;
    let mut bool_40: bool = false;
    let mut u64_3: u64 = 64u64;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_5: usize = 38usize;
    let mut bool_41: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_41, depth: usize_5};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_4: u64 = 35u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_2: &str = "wIfxl0ZZ4Wht";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut usize_6: usize = 66usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_6);
    let mut usize_7: usize = 39usize;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_8: usize = 22usize;
    let mut bool_42: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_42, depth: usize_8};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_43: bool = false;
    let mut bool_44: bool = false;
    let mut bool_45: bool = true;
    let mut bool_46: bool = false;
    let mut bool_47: bool = false;
    let mut bool_48: bool = true;
    let mut bool_49: bool = false;
    let mut bool_50: bool = false;
    let mut bool_51: bool = false;
    let mut bool_52: bool = true;
    let mut bool_53: bool = true;
    let mut bool_54: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_54, user_write: bool_53, user_execute: bool_52, group_read: bool_51, group_write: bool_50, group_execute: bool_49, other_read: bool_48, other_write: bool_47, other_execute: bool_46, sticky: bool_45, setgid: bool_44, setuid: bool_43};
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_55: bool = true;
    let mut bool_56: bool = false;
    let mut bool_57: bool = false;
    let mut bool_58: bool = true;
    let mut bool_59: bool = true;
    let mut bool_60: bool = true;
    let mut bool_61: bool = true;
    let mut bool_62: bool = true;
    let mut bool_63: bool = true;
    let mut bool_64: bool = true;
    let mut bool_65: bool = false;
    let mut bool_66: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_66, user_write: bool_65, user_execute: bool_64, group_read: bool_63, group_write: bool_62, group_execute: bool_61, other_read: bool_60, other_write: bool_59, other_execute: bool_58, sticky: bool_57, setgid: bool_56, setuid: bool_55};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    panic!("From RustyUnit with love");
}
}