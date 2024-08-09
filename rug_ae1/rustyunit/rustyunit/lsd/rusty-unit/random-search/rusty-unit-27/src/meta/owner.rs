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
fn rusty_test_5166() {
    rusty_monitor::set_test_id(5166);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut usize_0: usize = 92usize;
    let mut bool_6: bool = false;
    let mut usize_1: usize = 74usize;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_2: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_3: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_3, theme: option_2, separator: option_1};
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_5: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_10: bool = false;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_10);
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_8: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_9: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_9, theme: option_8};
    let mut option_10: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_13: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_17: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_11: bool = true;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_11);
    let mut option_19: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_19, reverse: option_18, dir_grouping: option_17};
    let mut option_20: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_21: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_2: usize = 30usize;
    let mut option_23: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_24, depth: option_23};
    let mut option_25: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_26: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_12: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut option_28: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_29: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_30: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_31: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_2);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_31, theme: option_30, separator: option_29};
    let mut option_32: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_33: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut bool_13: bool = false;
    let mut option_34: std::option::Option<bool> = std::option::Option::Some(bool_13);
    let mut option_35: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_36: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_37: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_37, theme: option_36};
    let mut option_38: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_39: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_14: bool = true;
    let mut option_40: std::option::Option<bool> = std::option::Option::Some(bool_14);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_40, blocks: option_39, color: option_38, date: option_35, dereference: option_34, display: option_33, icons: option_32, ignore_globs: option_28, indicators: option_27, layout: option_26, recursion: option_25, size: option_22, permission: option_21, sorting: option_20, no_symlink: option_16, total_size: option_15, symlink_arrow: option_14, hyperlink: option_13};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 53u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    panic!("From RustyUnit with love");
}
}