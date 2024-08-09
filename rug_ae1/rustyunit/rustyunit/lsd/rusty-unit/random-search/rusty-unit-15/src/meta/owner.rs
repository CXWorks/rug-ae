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
fn rusty_test_5300() {
    rusty_monitor::set_test_id(5300);
    let mut usize_0: usize = 70usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 30usize;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut usize_2: usize = 52usize;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut usize_3: usize = 48usize;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut bool_27: bool = true;
    let mut bool_28: bool = false;
    let mut bool_29: bool = true;
    let mut bool_30: bool = false;
    let mut usize_4: usize = 0usize;
    let mut bool_31: bool = true;
    let mut u64_0: u64 = 53u64;
    let mut bool_32: bool = true;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut bool_36: bool = false;
    let mut bool_37: bool = true;
    let mut bool_38: bool = false;
    let mut bool_39: bool = false;
    let mut bool_40: bool = true;
    let mut bool_41: bool = true;
    let mut bool_42: bool = false;
    let mut bool_43: bool = true;
    let mut bool_44: bool = false;
    let mut bool_45: bool = true;
    let mut bool_46: bool = false;
    let mut bool_47: bool = true;
    let mut bool_48: bool = true;
    let mut bool_49: bool = true;
    let mut bool_50: bool = true;
    let mut bool_51: bool = true;
    let mut bool_52: bool = false;
    let mut bool_53: bool = false;
    let mut bool_54: bool = false;
    let mut bool_55: bool = true;
    let mut bool_56: bool = false;
    let mut bool_57: bool = true;
    let mut u64_1: u64 = 8u64;
    let mut str_0: &str = "dtEN8SU8su";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_58: bool = true;
    let mut bool_59: bool = false;
    let mut bool_60: bool = false;
    let mut bool_61: bool = false;
    let mut bool_62: bool = false;
    let mut bool_63: bool = true;
    let mut bool_64: bool = true;
    let mut bool_65: bool = false;
    let mut bool_66: bool = true;
    let mut bool_67: bool = true;
    let mut bool_68: bool = false;
    let mut bool_69: bool = true;
    let mut bool_70: bool = false;
    let mut bool_71: bool = false;
    let mut bool_72: bool = false;
    let mut bool_73: bool = false;
    let mut bool_74: bool = true;
    let mut bool_75: bool = false;
    let mut bool_76: bool = true;
    let mut bool_77: bool = true;
    let mut bool_78: bool = true;
    let mut bool_79: bool = false;
    let mut bool_80: bool = false;
    let mut bool_81: bool = false;
    let mut bool_82: bool = true;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_5: usize = 80usize;
    let mut bool_83: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_83, depth: usize_5};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_2: u64 = 69u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_1: &str = "hSsORK6zcYXhX";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_6: usize = 7usize;
    let mut bool_84: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_84, depth: usize_6};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_85: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_85);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_8, reverse: option_7, dir_grouping: option_6};
    let mut option_9: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_2);
    let mut option_11: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_86: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_86);
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_87: bool = false;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_87);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_11, permission: option_10, sorting: option_9, no_symlink: option_5, total_size: option_4, symlink_arrow: option_3, hyperlink: option_2};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    panic!("From RustyUnit with love");
}
}