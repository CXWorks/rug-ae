///! This module provides methods to create theme from files and operations related to
///! this.
use crate::config_file;
use crate::print_error;

use crossterm::style::Color;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// A struct holding the theme configuration
/// Color table: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.avg
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Theme {
    pub user: Color,
    pub group: Color,
    pub permission: Permission,
    pub date: Date,
    pub size: Size,
    pub inode: INode,
    pub tree_edge: Color,
    pub links: Links,

    #[serde(skip)]
    pub file_type: FileType,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Permission {
    pub read: Color,
    pub write: Color,
    pub exec: Color,
    pub exec_sticky: Color,
    pub no_access: Color,
    pub octal: Color,
    pub acl: Color,
    pub context: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct FileType {
    pub file: File,
    pub dir: Dir,
    pub pipe: Color,
    pub symlink: Symlink,
    pub block_device: Color,
    pub char_device: Color,
    pub socket: Color,
    pub special: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct File {
    pub exec_uid: Color,
    pub uid_no_exec: Color,
    pub exec_no_uid: Color,
    pub no_exec_no_uid: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Dir {
    pub uid: Color,
    pub no_uid: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Symlink {
    pub default: Color,
    pub broken: Color,
    pub missing_target: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Date {
    pub hour_old: Color,
    pub day_old: Color,
    pub older: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Size {
    pub none: Color,
    pub small: Color,
    pub medium: Color,
    pub large: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct INode {
    pub valid: Color,
    pub invalid: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Links {
    pub valid: Color,
    pub invalid: Color,
}

impl Default for Permission {
    fn default() -> Self {
        Permission {
            read: Color::DarkGreen,
            write: Color::DarkYellow,
            exec: Color::DarkRed,
            exec_sticky: Color::AnsiValue(5),
            no_access: Color::AnsiValue(245), // Grey
            octal: Color::AnsiValue(6),
            acl: Color::DarkCyan,
            context: Color::Cyan,
        }
    }
}
impl Default for FileType {
    fn default() -> Self {
        FileType {
            file: File::default(),
            dir: Dir::default(),
            symlink: Symlink::default(),
            pipe: Color::AnsiValue(44),         // DarkTurquoise
            block_device: Color::AnsiValue(44), // DarkTurquoise
            char_device: Color::AnsiValue(172), // Orange3
            socket: Color::AnsiValue(44),       // DarkTurquoise
            special: Color::AnsiValue(44),      // DarkTurquoise
        }
    }
}
impl Default for File {
    fn default() -> Self {
        File {
            exec_uid: Color::AnsiValue(40),        // Green3
            uid_no_exec: Color::AnsiValue(184),    // Yellow3
            exec_no_uid: Color::AnsiValue(40),     // Green3
            no_exec_no_uid: Color::AnsiValue(184), // Yellow3
        }
    }
}
impl Default for Dir {
    fn default() -> Self {
        Dir {
            uid: Color::AnsiValue(33),    // DodgerBlue1
            no_uid: Color::AnsiValue(33), // DodgerBlue1
        }
    }
}
impl Default for Symlink {
    fn default() -> Self {
        Symlink {
            default: Color::AnsiValue(44),         // DarkTurquoise
            broken: Color::AnsiValue(124),         // Red3
            missing_target: Color::AnsiValue(124), // Red3
        }
    }
}
impl Default for Date {
    fn default() -> Self {
        Date {
            hour_old: Color::AnsiValue(40), // Green3
            day_old: Color::AnsiValue(42),  // SpringGreen2
            older: Color::AnsiValue(36),    // DarkCyan
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            none: Color::AnsiValue(245),   // Grey
            small: Color::AnsiValue(229),  // Wheat1
            medium: Color::AnsiValue(216), // LightSalmon1
            large: Color::AnsiValue(172),  // Orange3
        }
    }
}
impl Default for INode {
    fn default() -> Self {
        INode {
            valid: Color::AnsiValue(13),    // Pink
            invalid: Color::AnsiValue(245), // Grey
        }
    }
}
impl Default for Links {
    fn default() -> Self {
        Links {
            valid: Color::AnsiValue(13),    // Pink
            invalid: Color::AnsiValue(245), // Grey
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        // TODO(zwpaper): check terminal color and return light or dark
        Self::default_dark()
    }
}

impl Theme {
    /// This read theme from file,
    /// use the file path if it is absolute
    /// prefix the config_file dir to it if it is not
    pub fn from_path(file: &str) -> Option<Self> {
        let real = if let Some(path) = config_file::Config::expand_home(file) {
            path
        } else {
            print_error!("Not a valid theme file path: {}.", &file);
            return None;
        };
        let path = if Path::new(&real).is_absolute() {
            real
        } else {
            config_file::Config::config_file_path()?
                .join("themes")
                .join(real)
        };
        match fs::read(&path.with_extension("yaml")) {
            Ok(f) => match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                Ok(t) => Some(t),
                Err(e) => {
                    print_error!("Theme file {} format error: {}.", &file, e);
                    None
                }
            },
            Err(_) => {
                // try `yml` if `yaml` extension file not found
                match fs::read(&path.with_extension("yml")) {
                    Ok(f) => match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                        Ok(t) => Some(t),
                        Err(e) => {
                            print_error!("Theme file {} format error: {}.", &file, e);
                            None
                        }
                    },
                    Err(e) => {
                        print_error!("Not a valid theme: {}, {}.", path.to_string_lossy(), e);
                        None
                    }
                }
            }
        }
    }

    /// This constructs a Theme struct with a passed [Yaml] str.
    fn with_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(yaml)
    }

    pub fn default_dark() -> Self {
        Theme {
            user: Color::AnsiValue(230),  // Cornsilk1
            group: Color::AnsiValue(187), // LightYellow3
            permission: Permission::default(),
            file_type: FileType::default(),
            date: Date::default(),
            size: Size::default(),
            inode: INode::default(),
            links: Links::default(),
            tree_edge: Color::AnsiValue(245), // Grey
        }
    }

    #[cfg(test)]
    pub fn default_yaml() -> &'static str {
        r#"---
user: 230
group: 187
permission:
  read: dark_green
  write: dark_yellow
  exec: dark_red
  exec-sticky: 5
  no-access: 245
date:
  hour-old: 40
  day-old: 42
  older: 36
size:
  none: 245
  small: 229
  medium: 216
  large: 172
inode:
  valid: 13
  invalid: 245
links:
  valid: 13
  invalid: 245
tree-edge: 245
"#
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn test_default_theme() {
        assert_eq!(
            Theme::default_dark(),
            Theme::with_yaml(Theme::default_yaml()).unwrap()
        );
    }

    #[test]
    fn test_default_theme_file() {
        use std::fs::File;
        use std::io::Write;
        let dir = assert_fs::TempDir::new().unwrap();
        let theme = dir.path().join("theme.yaml");
        let mut file = File::create(&theme).unwrap();
        writeln!(file, "{}", Theme::default_yaml()).unwrap();

        assert_eq!(
            Theme::default_dark(),
            Theme::from_path(theme.to_str().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_empty_theme_return_default() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme = Theme::with_yaml("user: 230".into()).unwrap(); // 230 is the default value
        let default_theme = Theme::default_dark();
        assert_eq!(empty_theme, default_theme);
    }

    #[test]
    fn test_first_level_theme_return_default_but_changed() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme = Theme::with_yaml("user: 130".into()).unwrap();
        let mut theme = Theme::default_dark();
        use crossterm::style::Color;
        theme.user = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }

    #[test]
    fn test_second_level_theme_return_default_but_changed() {
        // Must contain one field at least
        // ref https://github.com/dtolnay/serde-yaml/issues/86
        let empty_theme = Theme::with_yaml(
            r#"---
permission:
  read: 130"#
                .into(),
        )
        .unwrap();
        let mut theme = Theme::default_dark();
        use crossterm::style::Color;
        theme.permission.read = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8153() {
    rusty_monitor::set_test_id(8153);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_2: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_4: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1041() {
    rusty_monitor::set_test_id(1041);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut usize_0: usize = 5813usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6305() {
    rusty_monitor::set_test_id(6305);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_2: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut bool_3: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5787() {
    rusty_monitor::set_test_id(5787);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut symlink_1: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut symlink_1_ref_0: &crate::color::theme::Symlink = &mut symlink_1;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_5, day_old: color_4, older: color_3};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Octal;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_3: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_8: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_7, no_uid: color_6};
    let mut elem_9: color::Elem = crate::color::Elem::HourOld;
    let mut bool_4: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut bool_5: bool = crate::color::theme::Symlink::ne(symlink_1_ref_0, symlink_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_550() {
    rusty_monitor::set_test_id(550);
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 9085usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 6998usize;
    let mut option_8: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut bool_1: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 398u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_0_ref_0: &crate::color::theme::INode = &mut inode_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_3: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut inode_1_ref_0: &crate::color::theme::INode = &mut inode_1;
    let mut option_23: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_23);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut bool_4: bool = crate::color::theme::INode::ne(inode_1_ref_0, inode_0_ref_0);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Date;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2737() {
    rusty_monitor::set_test_id(2737);
    let mut file_0: crate::color::theme::File = crate::color::theme::File::default();
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_2: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_2};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut file_1: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut file_1_ref_0: &crate::color::theme::File = &mut file_1;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_2: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_7: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_7, theme: option_6, separator: option_5};
    let mut option_8: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Byte;
    let mut bool_5: bool = crate::color::theme::File::ne(file_1_ref_0, file_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5387() {
    rusty_monitor::set_test_id(5387);
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut str_0: &str = "vCSZbo4Y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Context;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2258() {
    rusty_monitor::set_test_id(2258);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::NonFile;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut app_0: clap::App = crate::app::build();
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut bool_0: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_5, day_old: color_4, older: color_3};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut bool_1: bool = crate::color::theme::Date::ne(date_1_ref_0, date_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7341() {
    rusty_monitor::set_test_id(7341);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_1: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Context;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::SymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Acl;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_6, day_old: color_5, older: color_4};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::Octal;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut size_2: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut bool_4: bool = crate::color::theme::Size::eq(size_0_ref_0, size_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_9: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_8, no_uid: color_7};
    let mut elem_10: color::Elem = crate::color::Elem::HourOld;
    let mut bool_5: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6226() {
    rusty_monitor::set_test_id(6226);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Acl;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_0: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_4, no_uid: color_3};
    let mut elem_6: color::Elem = crate::color::Elem::HourOld;
    let mut bool_1: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1210() {
    rusty_monitor::set_test_id(1210);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut usize_0: usize = 5813usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_683() {
    rusty_monitor::set_test_id(683);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::INode;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut block_3: flags::blocks::Block = crate::flags::blocks::Block::Links;
    let mut block_2_ref_0: &flags::blocks::Block = &mut block_2;
    let mut block_4: flags::blocks::Block = crate::flags::blocks::Block::Size;
    let mut block_3_ref_0: &flags::blocks::Block = &mut block_3;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_1: bool = crate::color::theme::Theme::ne(theme_4_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1173() {
    rusty_monitor::set_test_id(1173);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut dir_0_ref_0: &crate::color::theme::Dir = &mut dir_0;
    let mut dir_1: crate::color::theme::Dir = crate::color::theme::Dir::default();
    let mut dir_1_ref_0: &crate::color::theme::Dir = &mut dir_1;
    let mut str_0: &str = "vCSZbo4Y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_8_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_7, day_old: color_6, older: color_5};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_8: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_9_ref_0);
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_10_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_2: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_10: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_2: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_9, no_uid: color_8};
    let mut elem_11: color::Elem = crate::color::Elem::HourOld;
    let mut bool_3: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_4, day_old: color_3, older: color_2};
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut elem_12: color::Elem = crate::color::Elem::Exec;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut elem_13: color::Elem = crate::color::Elem::Special;
    let mut bool_4: bool = crate::color::theme::Dir::eq(dir_1_ref_0, dir_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2069() {
    rusty_monitor::set_test_id(2069);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut u64_0: u64 = 3908u64;
    let mut usize_0: usize = 5813usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2476() {
    rusty_monitor::set_test_id(2476);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut dir_0_ref_0: &crate::color::theme::Dir = &mut dir_0;
    let mut dir_1: crate::color::theme::Dir = crate::color::theme::Dir::default();
    let mut dir_1_ref_0: &crate::color::theme::Dir = &mut dir_1;
    let mut bool_0: bool = false;
    let mut str_0: &str = "vCSZbo4Y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_6_ref_0);
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_7, day_old: color_6, older: color_5};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_8: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_9_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_3: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_10: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_2: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_9, no_uid: color_8};
    let mut elem_11: color::Elem = crate::color::Elem::DayOld;
    let mut bool_4: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_4, day_old: color_3, older: color_2};
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut elem_12: color::Elem = crate::color::Elem::Exec;
    let mut elem_13: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut bool_5: bool = crate::color::theme::Dir::ne(dir_1_ref_0, dir_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2243() {
    rusty_monitor::set_test_id(2243);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_0_ref_0: &crate::color::theme::INode = &mut inode_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut inode_1_ref_0: &crate::color::theme::INode = &mut inode_1;
    let mut str_0: &str = "vCSZbo4Y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_5: color::Elem = crate::color::Elem::SymLink;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_7: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_8_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_7, day_old: color_6, older: color_5};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_8: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_9_ref_0);
    let mut theme_10: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_10_ref_0: &crate::color::theme::Theme = &mut theme_10;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_10_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_2: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut elem_10: color::Elem = crate::color::Elem::Write;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_9, no_uid: color_8};
    let mut elem_11: color::Elem = crate::color::Elem::HourOld;
    let mut bool_3: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_4, day_old: color_3, older: color_2};
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut elem_12: color::Elem = crate::color::Elem::Exec;
    let mut bool_4: bool = crate::color::theme::INode::eq(inode_1_ref_0, inode_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4123() {
    rusty_monitor::set_test_id(4123);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_4_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_6_ref_0);
    let mut u64_0: u64 = 8102u64;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_2: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_4, no_uid: color_3};
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut bool_3: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
    let mut bool_4: bool = crate::color::theme::Theme::eq(theme_1_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}
}