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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_418() {
    rusty_monitor::set_test_id(418);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_1: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut u64_1: u64 = 21u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_3_ref_0, links_2_ref_0);
    crate::meta::links::Links::render(links_1_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2729() {
    rusty_monitor::set_test_id(2729);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 21u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut bool_0: bool = crate::meta::links::Links::ne(links_3_ref_0, links_2_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut bool_1: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_669() {
    rusty_monitor::set_test_id(669);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut u64_0: u64 = 65u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut bool_1: bool = false;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_4: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_1: u64 = 42u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 59usize;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5566() {
    rusty_monitor::set_test_id(5566);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_2: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_2: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1816() {
    rusty_monitor::set_test_id(1816);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut bool_0: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2304() {
    rusty_monitor::set_test_id(2304);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut option_3: std::option::Option<u64> = std::option::Option::None;
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_2: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1169() {
    rusty_monitor::set_test_id(1169);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3433() {
    rusty_monitor::set_test_id(3433);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7524() {
    rusty_monitor::set_test_id(7524);
    let mut bool_0: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut str_0: &str = "mpM9iCD6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 49u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_3: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_13: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_13);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut option_6: std::option::Option<u64> = std::option::Option::None;
    let mut option_7: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_7};
    let mut option_8: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_8};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_9: std::option::Option<u64> = std::option::Option::None;
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_9};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_2: u64 = 4u64;
    let mut option_10: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_6};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut u64_3: u64 = 21u64;
    let mut option_11: std::option::Option<u64> = std::option::Option::Some(u64_3);
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_11};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut option_12: std::option::Option<u64> = std::option::Option::None;
    let mut links_5: crate::meta::links::Links = crate::meta::links::Links {nlink: option_12};
    let mut links_5_ref_0: &crate::meta::links::Links = &mut links_5;
    let mut bool_16: bool = crate::meta::links::Links::ne(links_5_ref_0, links_4_ref_0);
    crate::meta::links::Links::render(links_3_ref_0, colors_2_ref_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_15, uid: bool_14};
    let mut bool_17: bool = crate::meta::links::Links::eq(links_2_ref_0, links_0_ref_0);
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7498() {
    rusty_monitor::set_test_id(7498);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2413() {
    rusty_monitor::set_test_id(2413);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7688() {
    rusty_monitor::set_test_id(7688);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut u64_0: u64 = 97u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut u64_1: u64 = 6u64;
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_2: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_2);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut u64_3: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4733() {
    rusty_monitor::set_test_id(4733);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut links_0: crate::meta::links::Links = crate::meta::links::Links {nlink: option_0};
    let mut links_0_ref_0: &crate::meta::links::Links = &mut links_0;
    let mut option_1: std::option::Option<u64> = std::option::Option::None;
    let mut links_1: crate::meta::links::Links = crate::meta::links::Links {nlink: option_1};
    let mut links_1_ref_0: &crate::meta::links::Links = &mut links_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 6u64;
    let mut option_2: std::option::Option<u64> = std::option::Option::Some(u64_0);
    let mut links_2: crate::meta::links::Links = crate::meta::links::Links {nlink: option_2};
    let mut links_2_ref_0: &crate::meta::links::Links = &mut links_2;
    let mut u64_1: u64 = 21u64;
    let mut option_3: std::option::Option<u64> = std::option::Option::Some(u64_1);
    let mut links_3: crate::meta::links::Links = crate::meta::links::Links {nlink: option_3};
    let mut links_3_ref_0: &crate::meta::links::Links = &mut links_3;
    let mut option_4: std::option::Option<u64> = std::option::Option::None;
    let mut links_4: crate::meta::links::Links = crate::meta::links::Links {nlink: option_4};
    let mut links_4_ref_0: &crate::meta::links::Links = &mut links_4;
    let mut bool_2: bool = crate::meta::links::Links::ne(links_4_ref_0, links_3_ref_0);
    crate::meta::links::Links::render(links_2_ref_0, colors_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut elem_0: color::Elem = crate::color::Elem::Older;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut bool_3: bool = crate::meta::links::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    panic!("From RustyUnit with love");
}
}