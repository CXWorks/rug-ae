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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_304() {
//    rusty_monitor::set_test_id(304);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_2_ref_0: &crate::color::theme::Date = &mut date_2;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_0: bool = true;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut date_3: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_5, day_old: color_4, older: color_3};
    let mut date_3_ref_0: &crate::color::theme::Date = &mut date_3;
    let mut bool_1: bool = crate::color::theme::Date::eq(date_3_ref_0, date_2_ref_0);
    let mut bool_2: bool = crate::color::theme::Date::eq(date_1_ref_0, date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_401() {
//    rusty_monitor::set_test_id(401);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut dir_0_ref_0: &crate::color::theme::Dir = &mut dir_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::HourOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut dir_1: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_3, no_uid: color_2};
    let mut dir_1_ref_0: &crate::color::theme::Dir = &mut dir_1;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut dir_2: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_5, no_uid: color_4};
    let mut dir_2_ref_0: &crate::color::theme::Dir = &mut dir_2;
    let mut dir_3: crate::color::theme::Dir = crate::color::theme::Dir::default();
    let mut dir_3_ref_0: &crate::color::theme::Dir = &mut dir_3;
    let mut bool_1: bool = crate::color::theme::Dir::eq(dir_3_ref_0, dir_2_ref_0);
    let mut bool_2: bool = crate::color::theme::Dir::eq(dir_1_ref_0, dir_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_255() {
//    rusty_monitor::set_test_id(255);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut symlink_1: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_1_ref_0: &crate::color::theme::Symlink = &mut symlink_1;
    let mut symlink_2: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_2_ref_0: &crate::color::theme::Symlink = &mut symlink_2;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut symlink_3: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_5, broken: color_4, missing_target: color_3};
    let mut symlink_3_ref_0: &crate::color::theme::Symlink = &mut symlink_3;
    let mut symlink_4: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_4_ref_0: &crate::color::theme::Symlink = &mut symlink_4;
    let mut symlink_5: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_5_ref_0: &crate::color::theme::Symlink = &mut symlink_5;
    let mut bool_0: bool = crate::color::theme::Symlink::ne(symlink_5_ref_0, symlink_4_ref_0);
    let mut bool_1: bool = crate::color::theme::Symlink::ne(symlink_3_ref_0, symlink_2_ref_0);
    let mut bool_2: bool = crate::color::theme::Symlink::ne(symlink_1_ref_0, symlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2440() {
//    rusty_monitor::set_test_id(2440);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut filetype_0: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_1: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut filetype_1: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_2: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = crate::color::theme::Theme::ne(theme_0_ref_0, theme_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_385() {
//    rusty_monitor::set_test_id(385);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_1_ref_0: &crate::color::theme::Date = &mut date_1;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Context;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Socket;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_5, day_old: color_4, older: color_3};
    let mut date_2_ref_0: &crate::color::theme::Date = &mut date_2;
    let mut date_3: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut date_3_ref_0: &crate::color::theme::Date = &mut date_3;
    let mut bool_0: bool = crate::color::theme::Date::ne(date_3_ref_0, date_2_ref_0);
    let mut bool_1: bool = crate::color::theme::Date::ne(date_1_ref_0, date_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5664() {
//    rusty_monitor::set_test_id(5664);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut filetype_0: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_1: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut filetype_1: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_2: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_1: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut filetype_2: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_3: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_2: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_2: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_2: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_1: bool = crate::color::theme::Theme::ne(theme_6_ref_0, theme_5_ref_0);
    let mut bool_2: bool = crate::color::theme::Theme::ne(theme_4_ref_0, theme_3_ref_0);
    let mut links_1_ref_0: &crate::color::theme::Links = &mut links_1;
    let mut bool_3: bool = crate::color::theme::Links::eq(links_1_ref_0, links_0_ref_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5902() {
//    rusty_monitor::set_test_id(5902);
    let mut str_0: &str = "CkFDd6rIM";
    let mut str_1: &str = "";
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "tree";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_640() {
//    rusty_monitor::set_test_id(640);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut permission_0_ref_0: &crate::color::theme::Permission = &mut permission_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_1: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Context;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut permission_1: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    let mut permission_1_ref_0: &crate::color::theme::Permission = &mut permission_1;
    let mut bool_2: bool = crate::color::theme::Permission::eq(permission_1_ref_0, permission_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_476() {
//    rusty_monitor::set_test_id(476);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir::default();
    let mut dir_0_ref_0: &crate::color::theme::Dir = &mut dir_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut dir_1: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
    let mut dir_1_ref_0: &crate::color::theme::Dir = &mut dir_1;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Octal;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut dir_2: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_3, no_uid: color_2};
    let mut dir_2_ref_0: &crate::color::theme::Dir = &mut dir_2;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut dir_3: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_5, no_uid: color_4};
    let mut dir_3_ref_0: &crate::color::theme::Dir = &mut dir_3;
    let mut bool_0: bool = crate::color::theme::Dir::ne(dir_3_ref_0, dir_2_ref_0);
    let mut bool_1: bool = crate::color::theme::Dir::ne(dir_1_ref_0, dir_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5261() {
//    rusty_monitor::set_test_id(5261);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut str_0: &str = "tar";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_1: color::Elem = crate::color::Elem::Read;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_4: color::Elem = crate::color::Elem::HourOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_231() {
//    rusty_monitor::set_test_id(231);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut links_1: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_1_ref_0: &crate::color::theme::Links = &mut links_1;
    let mut links_2: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_2_ref_0: &crate::color::theme::Links = &mut links_2;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut links_3: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut links_3_ref_0: &crate::color::theme::Links = &mut links_3;
    let mut bool_2: bool = crate::color::theme::Links::ne(links_3_ref_0, links_2_ref_0);
    let mut bool_3: bool = crate::color::theme::Links::ne(links_1_ref_0, links_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_92() {
//    rusty_monitor::set_test_id(92);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Write;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut usize_0: usize = 0usize;
    let mut bool_1: bool = false;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink::default();
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5632() {
//    rusty_monitor::set_test_id(5632);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::HourOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_0: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut file_1: crate::color::theme::File = crate::color::theme::File {exec_uid: color_7, uid_no_exec: color_6, exec_no_uid: color_5, no_exec_no_uid: color_4};
    let mut file_1_ref_0: &crate::color::theme::File = &mut file_1;
    let mut bool_1: bool = crate::color::theme::File::ne(file_1_ref_0, file_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5138() {
//    rusty_monitor::set_test_id(5138);
    let mut str_0: &str = "mp4";
    let mut str_1: &str = ".vimrc";
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Reverse the order of the sort";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2_ref_0: &str = &mut str_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_468() {
//    rusty_monitor::set_test_id(468);
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut permission_0_ref_0: &crate::color::theme::Permission = &mut permission_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Read;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Octal;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Acl;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut permission_1: crate::color::theme::Permission = crate::color::theme::Permission {read: color_7, write: color_6, exec: color_5, exec_sticky: color_4, no_access: color_3, octal: color_2, acl: color_1, context: color_0};
    let mut permission_1_ref_0: &crate::color::theme::Permission = &mut permission_1;
    let mut bool_1: bool = crate::color::theme::Permission::ne(permission_1_ref_0, permission_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_371() {
//    rusty_monitor::set_test_id(371);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_0_ref_0: &crate::color::theme::INode = &mut inode_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut inode_1_ref_0: &crate::color::theme::INode = &mut inode_1;
    let mut inode_2: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_2_ref_0: &crate::color::theme::INode = &mut inode_2;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut inode_3: crate::color::theme::INode = crate::color::theme::INode {valid: color_3, invalid: color_2};
    let mut inode_3_ref_0: &crate::color::theme::INode = &mut inode_3;
    let mut inode_4: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_4_ref_0: &crate::color::theme::INode = &mut inode_4;
    let mut inode_5: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut inode_5_ref_0: &crate::color::theme::INode = &mut inode_5;
    let mut bool_0: bool = crate::color::theme::INode::eq(inode_5_ref_0, inode_4_ref_0);
    let mut bool_1: bool = crate::color::theme::INode::eq(inode_3_ref_0, inode_2_ref_0);
    let mut bool_2: bool = crate::color::theme::INode::eq(inode_1_ref_0, inode_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3793() {
//    rusty_monitor::set_test_id(3793);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::SizeValue;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5807() {
//    rusty_monitor::set_test_id(5807);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Context;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Socket;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Write;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut bool_0: bool = false;
    let mut elem_7: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut str_0: &str = "ai";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortorder_2_ref_0: &flags::sorting::SortOrder = &mut sortorder_2;
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut file_1: crate::color::theme::File = crate::color::theme::File {exec_uid: color_7, uid_no_exec: color_6, exec_no_uid: color_5, no_exec_no_uid: color_4};
    let mut file_1_ref_0: &crate::color::theme::File = &mut file_1;
    let mut bool_1: bool = crate::color::theme::File::eq(file_1_ref_0, file_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2063() {
//    rusty_monitor::set_test_id(2063);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 97u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut str_0: &str = "mkd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_596() {
//    rusty_monitor::set_test_id(596);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Group;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    let mut links_0_ref_0: &crate::color::theme::Links = &mut links_0;
    let mut links_1: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_1_ref_0: &crate::color::theme::Links = &mut links_1;
    let mut links_2: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_2_ref_0: &crate::color::theme::Links = &mut links_2;
    let mut links_3: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_3_ref_0: &crate::color::theme::Links = &mut links_3;
    let mut links_4: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut links_4_ref_0: &crate::color::theme::Links = &mut links_4;
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut links_5: crate::color::theme::Links = crate::color::theme::Links {valid: color_3, invalid: color_2};
    let mut links_5_ref_0: &crate::color::theme::Links = &mut links_5;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5066() {
//    rusty_monitor::set_test_id(5066);
    let mut filetype_0: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut filetype_0_ref_0: &crate::color::theme::FileType = &mut filetype_0;
    let mut filetype_1: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut filetype_1_ref_0: &crate::color::theme::FileType = &mut filetype_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut bool_0: bool = crate::color::theme::FileType::ne(filetype_1_ref_0, filetype_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_477() {
//    rusty_monitor::set_test_id(477);
    let mut str_0: &str = "scpt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "CQ4mr1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "CharDevice";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Fav3tvIG";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "htm";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "NoIcon";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "SnwvAboJsYth5p";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_10_ref_0);
    let mut result_1: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_9_ref_0);
    let mut result_2: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_8_ref_0);
    let mut result_3: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_7_ref_0);
    let mut result_4: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_6_ref_0);
    let mut result_5: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_5_ref_0);
    let mut result_6: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_4_ref_0);
    let mut result_7: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_3_ref_0);
    let mut result_8: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_2_ref_0);
    let mut result_9: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_1_ref_0);
    let mut result_10: std::result::Result<crate::color::theme::Theme, serde_yaml::Error> = crate::color::theme::Theme::with_yaml(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1383() {
//    rusty_monitor::set_test_id(1383);
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut str_0: &str = "FileLarge";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut elem_4: color::Elem = crate::color::Elem::Group;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_359() {
//    rusty_monitor::set_test_id(359);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut filetype_0: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_0: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut filetype_1: crate::color::theme::FileType = crate::color::theme::FileType::default();
    let mut links_1: crate::color::theme::Links = crate::color::theme::Links::default();
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode::default();
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date::default();
    let mut permission_1: crate::color::theme::Permission = crate::color::theme::Permission::default();
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut bool_0: bool = crate::color::theme::Theme::eq(theme_9_ref_0, theme_8_ref_0);
    let mut bool_1: bool = crate::color::theme::Theme::eq(theme_7_ref_0, theme_6_ref_0);
    let mut bool_2: bool = crate::color::theme::Theme::eq(theme_5_ref_0, theme_4_ref_0);
    let mut bool_3: bool = crate::color::theme::Theme::eq(theme_3_ref_0, theme_2_ref_0);
    let mut bool_4: bool = crate::color::theme::Theme::eq(theme_1_ref_0, theme_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_723() {
//    rusty_monitor::set_test_id(723);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Write;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::Exec;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Pipe;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut size_2: crate::color::theme::Size = crate::color::theme::Size {none: color_7, small: color_6, medium: color_5, large: color_4};
    let mut size_2_ref_0: &crate::color::theme::Size = &mut size_2;
    let mut size_3: crate::color::theme::Size = crate::color::theme::Size::default();
    let mut size_3_ref_0: &crate::color::theme::Size = &mut size_3;
    let mut bool_2: bool = crate::color::theme::Size::ne(size_3_ref_0, size_2_ref_0);
    let mut bool_3: bool = crate::color::theme::Size::ne(size_1_ref_0, size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_216() {
//    rusty_monitor::set_test_id(216);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::HourOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_5, invalid: color_4};
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode {valid: color_3, invalid: color_2};
    let mut inode_2: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_215() {
//    rusty_monitor::set_test_id(215);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::HourOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut symlink_1: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_5, broken: color_4, missing_target: color_3};
    let mut symlink_1_ref_0: &crate::color::theme::Symlink = &mut symlink_1;
    let mut bool_1: bool = crate::color::theme::Symlink::eq(symlink_1_ref_0, symlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_350() {
//    rusty_monitor::set_test_id(350);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Group;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_0: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_5, day_old: color_4, older: color_3};
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_282() {
//    rusty_monitor::set_test_id(282);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::User;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut size_0_ref_0: &crate::color::theme::Size = &mut size_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_0: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Group;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut size_1: crate::color::theme::Size = crate::color::theme::Size {none: color_7, small: color_6, medium: color_5, large: color_4};
    let mut size_1_ref_0: &crate::color::theme::Size = &mut size_1;
    let mut bool_1: bool = crate::color::theme::Size::eq(size_1_ref_0, size_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_119() {
//    rusty_monitor::set_test_id(119);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_2_ref_0);
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut inode_0_ref_0: &crate::color::theme::INode = &mut inode_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_3: color::Elem = crate::color::Elem::DayOld;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_4_ref_0);
    let mut inode_1: crate::color::theme::INode = crate::color::theme::INode {valid: color_3, invalid: color_2};
    let mut inode_1_ref_0: &crate::color::theme::INode = &mut inode_1;
    let mut bool_0: bool = crate::color::theme::INode::ne(inode_1_ref_0, inode_0_ref_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut permissionflag_0_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_0;
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_375() {
//    rusty_monitor::set_test_id(375);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::DayOld;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_5, no_uid: color_4};
    let mut dir_1: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_3, no_uid: color_2};
    let mut dir_2: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_1, no_uid: color_0};
//    panic!("From RustyUnit with love");
}
}