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
fn rusty_test_5329() {
    rusty_monitor::set_test_id(5329);
    let mut usize_0: usize = 40usize;
    let mut str_0: &str = "dDfp6Qe5R";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 55usize;
    let mut tuple_0: (usize, &str) = (usize_1, str_0_ref_0);
    let mut usize_2: usize = 54usize;
    let mut bool_0: bool = true;
    let mut usize_3: usize = 2usize;
    let mut bool_1: bool = true;
    let mut u64_0: u64 = 82u64;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut str_1: &str = "2tNKpBM";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_4: usize = 39usize;
    let mut bool_16: bool = false;
    let mut u64_1: u64 = 85u64;
    let mut u64_2: u64 = 8u64;
    let mut usize_5: usize = 19usize;
    let mut bool_17: bool = true;
    let mut u64_3: u64 = 19u64;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut bool_24: bool = false;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut str_2: &str = "2LZ9Ox6I";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_6: usize = 9usize;
    let mut tuple_1: (usize, &str) = (usize_6, str_2_ref_0);
    let mut usize_7: usize = 73usize;
    let mut bool_32: bool = true;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_8: usize = 76usize;
    let mut bool_33: bool = false;
    let mut usize_9: usize = 43usize;
    let mut bool_34: bool = true;
    let mut u64_4: u64 = 48u64;
    let mut u64_5: u64 = 19u64;
    let mut usize_10: usize = 30usize;
    let mut bool_35: bool = false;
    let mut usize_11: usize = 4usize;
    let mut bool_36: bool = false;
    let mut u64_6: u64 = 82u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_6);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_12: usize = 96usize;
    let mut bool_37: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_37, depth: usize_12};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_13: usize = 13usize;
    let mut bool_38: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_38, depth: usize_13};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut u64_7: u64 = 31u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_7);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_14: usize = 10usize;
    let mut bool_39: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_39, depth: usize_14};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_40: bool = false;
    let mut bool_41: bool = true;
    let mut bool_42: bool = false;
    let mut bool_43: bool = false;
    let mut bool_44: bool = true;
    let mut bool_45: bool = true;
    let mut bool_46: bool = false;
    let mut bool_47: bool = true;
    let mut bool_48: bool = false;
    let mut bool_49: bool = false;
    let mut bool_50: bool = true;
    let mut bool_51: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_51, user_write: bool_50, user_execute: bool_49, group_read: bool_48, group_write: bool_47, group_execute: bool_46, other_read: bool_45, other_write: bool_44, other_execute: bool_43, sticky: bool_42, setgid: bool_41, setuid: bool_40};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    panic!("From RustyUnit with love");
}
}