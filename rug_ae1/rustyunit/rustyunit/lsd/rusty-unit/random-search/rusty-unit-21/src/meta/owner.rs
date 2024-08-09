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
fn rusty_test_4987() {
    rusty_monitor::set_test_id(4987);
    let mut usize_0: usize = 10usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 20usize;
    let mut bool_1: bool = true;
    let mut u64_0: u64 = 64u64;
    let mut usize_2: usize = 2usize;
    let mut bool_2: bool = false;
    let mut usize_3: usize = 67usize;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut u64_1: u64 = 43u64;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut usize_4: usize = 69usize;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_15: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_15);
    let mut bool_16: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_5: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_17: bool = true;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_17);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_7: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_7, reverse: option_6, dir_grouping: option_5};
    let mut option_8: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_9: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_11: std::option::Option<usize> = std::option::Option::None;
    let mut bool_18: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_18);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_12, depth: option_11};
    let mut option_13: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_14: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_19: bool = false;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_19);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_18: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_19: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_19, theme: option_18, separator: option_17};
    let mut option_20: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_21: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_20: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_20);
    let mut option_23: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_24: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_21: bool = false;
    let mut option_26: std::option::Option<bool> = std::option::Option::Some(bool_21);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_26, blocks: option_25, color: option_24, date: option_23, dereference: option_22, display: option_21, icons: option_20, ignore_globs: option_16, indicators: option_15, layout: option_14, recursion: option_13, size: option_10, permission: option_9, sorting: option_8, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_2: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "1loU9PFwOwfs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_1};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_5: usize = 5usize;
    let mut bool_22: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_22, depth: usize_5};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Size;
    panic!("From RustyUnit with love");
}
}