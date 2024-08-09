use crate::color::{ColoredString, Colors, Elem};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct AccessControl {
    has_acl: bool,
    selinux_context: String,
    smack_context: String,
}

impl AccessControl {
    #[cfg(not(unix))]
    pub fn for_path(_: &Path) -> Self {
        Self::from_data(false, &[], &[])
    }

    #[cfg(unix)]
    pub fn for_path(path: &Path) -> Self {
        let has_acl = !xattr::get(path, Method::Acl.name())
            .unwrap_or_default()
            .unwrap_or_default()
            .is_empty();
        let selinux_context = xattr::get(path, Method::Selinux.name())
            .unwrap_or_default()
            .unwrap_or_default();
        let smack_context = xattr::get(path, Method::Smack.name())
            .unwrap_or_default()
            .unwrap_or_default();

        Self::from_data(has_acl, &selinux_context, &smack_context)
    }

    fn from_data(has_acl: bool, selinux_context: &[u8], smack_context: &[u8]) -> Self {
        let selinux_context = String::from_utf8_lossy(selinux_context).to_string();
        let smack_context = String::from_utf8_lossy(smack_context).to_string();
        Self {
            has_acl,
            selinux_context,
            smack_context,
        }
    }

    pub fn render_method(&self, colors: &Colors) -> ColoredString {
        if self.has_acl {
            colors.colorize(String::from("+"), &Elem::Acl)
        } else if !self.selinux_context.is_empty() || !self.smack_context.is_empty() {
            colors.colorize(String::from("."), &Elem::Context)
        } else {
            colors.colorize(String::from(""), &Elem::Acl)
        }
    }

    pub fn render_context(&self, colors: &Colors) -> ColoredString {
        let mut context = self.selinux_context.clone();
        if !self.smack_context.is_empty() {
            if !context.is_empty() {
                context += "+";
            }
            context += &self.smack_context;
        }
        if context.is_empty() {
            context += "?";
        }
        colors.colorize(context, &Elem::Context)
    }
}

#[cfg(unix)]
enum Method {
    Acl,
    Selinux,
    Smack,
}

#[cfg(unix)]
impl Method {
    fn name(&self) -> &'static str {
        match self {
            Method::Acl => "system.posix_acl_access",
            Method::Selinux => "security.selinux",
            Method::Smack => "security.SMACK64",
        }
    }
}

#[cfg(test)]
mod test {
    use super::AccessControl;
    use crate::color::{Colors, ThemeOption};
    use crossterm::style::{Color, Stylize};

    #[test]
    fn test_acl_only_indicator() {
        // actual file would collide with proper AC data, no permission to scrub those
        let access_control = AccessControl::from_data(true, &[], &[]);

        assert_eq!(
            String::from("+").with(Color::DarkCyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_smack_only_indicator() {
        let access_control = AccessControl::from_data(false, &[], &[b'a']);

        assert_eq!(
            String::from(".").with(Color::Cyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_acl_and_selinux_indicator() {
        let access_control = AccessControl::from_data(true, &[b'a'], &[]);

        assert_eq!(
            String::from("+").with(Color::DarkCyan),
            access_control.render_method(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_selinux_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[]);

        assert_eq!(
            String::from("a").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_selinux_and_smack_context() {
        let access_control = AccessControl::from_data(false, &[b'a'], &[b'b']);

        assert_eq!(
            String::from("a+b").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }

    #[test]
    fn test_no_context() {
        let access_control = AccessControl::from_data(false, &[], &[]);

        assert_eq!(
            String::from("?").with(Color::Cyan),
            access_control.render_context(&Colors::new(ThemeOption::Default))
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4777() {
    rusty_monitor::set_test_id(4777);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 94usize;
    let mut bool_2: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_17, blocks: option_16, color: option_15, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 29usize;
    let mut bool_6: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_3};
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_7: bool = false;
    let mut method_0: meta::access_control::Method = crate::meta::access_control::Method::Selinux;
    let mut method_0_ref_0: &meta::access_control::Method = &mut method_0;
    let mut str_0: &str = crate::meta::access_control::Method::name(method_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2073() {
    rusty_monitor::set_test_id(2073);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 60usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_2: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 63usize;
    let mut option_10: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_11, depth: option_10};
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut bool_5: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_5};
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_27: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 73u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_1: &str = "DjmTr9gGdTJMqo";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_6: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_6};
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut method_0: meta::access_control::Method = crate::meta::access_control::Method::Acl;
    let mut method_0_ref_0: &meta::access_control::Method = &mut method_0;
    let mut str_2: &str = crate::meta::access_control::Method::name(method_0_ref_0);
    let mut bool_7: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1808() {
    rusty_monitor::set_test_id(1808);
    let mut usize_0: usize = 33usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut bool_2: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 70usize;
    let mut bool_4: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_9: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_11, reverse: option_10, dir_grouping: option_9};
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_15: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_18, indicators: option_17, layout: option_16, recursion: option_15, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_8, total_size: option_7, symlink_arrow: option_6, hyperlink: option_5};
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_0: u64 = 96u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut method_0: meta::access_control::Method = crate::meta::access_control::Method::Smack;
    let mut method_0_ref_0: &meta::access_control::Method = &mut method_0;
    let mut theme_1: icon::Theme = crate::icon::Theme::Fancy;
    let mut theme_1_ref_0: &icon::Theme = &mut theme_1;
    let mut str_0: &str = crate::meta::access_control::Method::name(method_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    panic!("From RustyUnit with love");
}
}