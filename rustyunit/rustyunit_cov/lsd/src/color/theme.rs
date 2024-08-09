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
            no_access: Color::AnsiValue(245),
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
            pipe: Color::AnsiValue(44),
            block_device: Color::AnsiValue(44),
            char_device: Color::AnsiValue(172),
            socket: Color::AnsiValue(44),
            special: Color::AnsiValue(44),
        }
    }
}
impl Default for File {
    fn default() -> Self {
        File {
            exec_uid: Color::AnsiValue(40),
            uid_no_exec: Color::AnsiValue(184),
            exec_no_uid: Color::AnsiValue(40),
            no_exec_no_uid: Color::AnsiValue(184),
        }
    }
}
impl Default for Dir {
    fn default() -> Self {
        Dir {
            uid: Color::AnsiValue(33),
            no_uid: Color::AnsiValue(33),
        }
    }
}
impl Default for Symlink {
    fn default() -> Self {
        Symlink {
            default: Color::AnsiValue(44),
            broken: Color::AnsiValue(124),
            missing_target: Color::AnsiValue(124),
        }
    }
}
impl Default for Date {
    fn default() -> Self {
        Date {
            hour_old: Color::AnsiValue(40),
            day_old: Color::AnsiValue(42),
            older: Color::AnsiValue(36),
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            none: Color::AnsiValue(245),
            small: Color::AnsiValue(229),
            medium: Color::AnsiValue(216),
            large: Color::AnsiValue(172),
        }
    }
}
impl Default for INode {
    fn default() -> Self {
        INode {
            valid: Color::AnsiValue(13),
            invalid: Color::AnsiValue(245),
        }
    }
}
impl Default for Links {
    fn default() -> Self {
        Links {
            valid: Color::AnsiValue(13),
            invalid: Color::AnsiValue(245),
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
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
            print_error!("Not a valid theme file path: {}.", & file);
            return None;
        };
        let path = if Path::new(&real).is_absolute() {
            real
        } else {
            config_file::Config::config_file_path()?.join("themes").join(real)
        };
        match fs::read(&path.with_extension("yaml")) {
            Ok(f) => {
                match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                    Ok(t) => Some(t),
                    Err(e) => {
                        print_error!("Theme file {} format error: {}.", & file, e);
                        None
                    }
                }
            }
            Err(_) => {
                match fs::read(&path.with_extension("yml")) {
                    Ok(f) => {
                        match Self::with_yaml(&String::from_utf8_lossy(&f)) {
                            Ok(t) => Some(t),
                            Err(e) => {
                                print_error!("Theme file {} format error: {}.", & file, e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        print_error!(
                            "Not a valid theme: {}, {}.", path.to_string_lossy(), e
                        );
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
            user: Color::AnsiValue(230),
            group: Color::AnsiValue(187),
            permission: Permission::default(),
            file_type: FileType::default(),
            date: Date::default(),
            size: Size::default(),
            inode: INode::default(),
            links: Links::default(),
            tree_edge: Color::AnsiValue(245),
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
            Theme::default_dark(), Theme::with_yaml(Theme::default_yaml()).unwrap()
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
            Theme::default_dark(), Theme::from_path(theme.to_str().unwrap()).unwrap()
        );
    }
    #[test]
    fn test_empty_theme_return_default() {
        let empty_theme = Theme::with_yaml("user: 230".into()).unwrap();
        let default_theme = Theme::default_dark();
        assert_eq!(empty_theme, default_theme);
    }
    #[test]
    fn test_first_level_theme_return_default_but_changed() {
        let empty_theme = Theme::with_yaml("user: 130".into()).unwrap();
        let mut theme = Theme::default_dark();
        use crossterm::style::Color;
        theme.user = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }
    #[test]
    fn test_second_level_theme_return_default_but_changed() {
        let empty_theme = Theme::with_yaml(r#"---
permission:
  read: 130"#.into())
            .unwrap();
        let mut theme = Theme::default_dark();
        use crossterm::style::Color;
        theme.permission.read = Color::AnsiValue(130);
        assert_eq!(empty_theme, theme);
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use color::theme::Date;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 40;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 36;
        let expected = Date {
            hour_old: Color::AnsiValue(rug_fuzz_0),
            day_old: Color::AnsiValue(rug_fuzz_1),
            older: Color::AnsiValue(rug_fuzz_2),
        };
        let result = <Date as std::default::Default>::default();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 33;
        let rug_fuzz_1 = 33;
        let expected = Dir {
            uid: Color::AnsiValue(rug_fuzz_0),
            no_uid: Color::AnsiValue(rug_fuzz_1),
        };
        let result = <color::theme::Dir as std::default::Default>::default();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8_llm_16_7 {
    use super::*;
    use crate::*;
    use color::{Color, theme::File};
    use std::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_8_llm_16_7_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 40;
        let rug_fuzz_1 = 184;
        let rug_fuzz_2 = 40;
        let rug_fuzz_3 = 184;
        let expected = File {
            exec_uid: Color::AnsiValue(rug_fuzz_0),
            uid_no_exec: Color::AnsiValue(rug_fuzz_1),
            exec_no_uid: Color::AnsiValue(rug_fuzz_2),
            no_exec_no_uid: Color::AnsiValue(rug_fuzz_3),
        };
        let actual: File = Default::default();
        debug_assert_eq!(actual, expected);
        let _rug_ed_tests_llm_16_8_llm_16_7_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use color::theme::{Dir, File, FileType, Symlink, Color};
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 40;
        let rug_fuzz_1 = 184;
        let rug_fuzz_2 = 40;
        let rug_fuzz_3 = 184;
        let rug_fuzz_4 = 33;
        let rug_fuzz_5 = 33;
        let rug_fuzz_6 = 44;
        let rug_fuzz_7 = 124;
        let rug_fuzz_8 = 124;
        let rug_fuzz_9 = 44;
        let rug_fuzz_10 = 44;
        let rug_fuzz_11 = 172;
        let rug_fuzz_12 = 44;
        let rug_fuzz_13 = 44;
        let expected_result = FileType {
            file: File {
                exec_uid: Color::AnsiValue(rug_fuzz_0),
                uid_no_exec: Color::AnsiValue(rug_fuzz_1),
                exec_no_uid: Color::AnsiValue(rug_fuzz_2),
                no_exec_no_uid: Color::AnsiValue(rug_fuzz_3),
            },
            dir: Dir {
                uid: Color::AnsiValue(rug_fuzz_4),
                no_uid: Color::AnsiValue(rug_fuzz_5),
            },
            symlink: Symlink {
                default: Color::AnsiValue(rug_fuzz_6),
                broken: Color::AnsiValue(rug_fuzz_7),
                missing_target: Color::AnsiValue(rug_fuzz_8),
            },
            pipe: Color::AnsiValue(rug_fuzz_9),
            block_device: Color::AnsiValue(rug_fuzz_10),
            char_device: Color::AnsiValue(rug_fuzz_11),
            socket: Color::AnsiValue(rug_fuzz_12),
            special: Color::AnsiValue(rug_fuzz_13),
        };
        let result = <color::theme::FileType as std::default::Default>::default();
        debug_assert_eq!(result, expected_result);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11_llm_16_10 {
    use crate::color::theme::INode;
    use crate::color::theme::Color;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_11_llm_16_10_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 13;
        let rug_fuzz_1 = 245;
        let expected = INode {
            valid: Color::AnsiValue(rug_fuzz_0),
            invalid: Color::AnsiValue(rug_fuzz_1),
        };
        let actual = INode::default();
        debug_assert_eq!(actual.valid, expected.valid);
        debug_assert_eq!(actual.invalid, expected.invalid);
        let _rug_ed_tests_llm_16_11_llm_16_10_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_13_llm_16_12 {
    use crate::color::theme::Links;
    use crate::color::Color;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_13_llm_16_12_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 13;
        let rug_fuzz_1 = 245;
        let expected = Links {
            valid: Color::AnsiValue(rug_fuzz_0),
            invalid: Color::AnsiValue(rug_fuzz_1),
        };
        let result = <Links as std::default::Default>::default();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_13_llm_16_12_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_14_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 245;
        let rug_fuzz_2 = 6;
        let default_permission = color::theme::Permission {
            read: color::theme::Color::DarkGreen,
            write: color::theme::Color::DarkYellow,
            exec: color::theme::Color::DarkRed,
            exec_sticky: color::theme::Color::AnsiValue(rug_fuzz_0),
            no_access: color::theme::Color::AnsiValue(rug_fuzz_1),
            octal: color::theme::Color::AnsiValue(rug_fuzz_2),
            acl: color::theme::Color::DarkCyan,
            context: color::theme::Color::Cyan,
        };
        debug_assert_eq!(default_permission, color::theme::Permission::default());
        let _rug_ed_tests_llm_16_14_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_19 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default_theme() {
        let _rug_st_tests_llm_16_19_rrrruuuugggg_test_default_theme = 0;
        let theme = Theme::default();
        debug_assert_eq!(theme.user, Color::AnsiValue(230));
        debug_assert_eq!(theme.group, Color::AnsiValue(187));
        debug_assert_eq!(theme.permission.read, Color::DarkGreen);
        debug_assert_eq!(theme.permission.write, Color::DarkYellow);
        debug_assert_eq!(theme.permission.exec, Color::DarkRed);
        debug_assert_eq!(theme.permission.exec_sticky, Color::AnsiValue(5));
        debug_assert_eq!(theme.permission.no_access, Color::AnsiValue(245));
        debug_assert_eq!(theme.file_type.file.exec_uid, Color::AnsiValue(40));
        debug_assert_eq!(theme.file_type.file.uid_no_exec, Color::AnsiValue(184));
        debug_assert_eq!(theme.file_type.file.exec_no_uid, Color::AnsiValue(40));
        debug_assert_eq!(theme.file_type.file.no_exec_no_uid, Color::AnsiValue(184));
        debug_assert_eq!(theme.file_type.dir.uid, Color::AnsiValue(33));
        debug_assert_eq!(theme.file_type.dir.no_uid, Color::AnsiValue(33));
        debug_assert_eq!(theme.file_type.symlink.default, Color::AnsiValue(44));
        debug_assert_eq!(theme.file_type.symlink.broken, Color::AnsiValue(124));
        debug_assert_eq!(theme.file_type.symlink.missing_target, Color::AnsiValue(124));
        debug_assert_eq!(theme.file_type.pipe, Color::AnsiValue(44));
        debug_assert_eq!(theme.file_type.block_device, Color::AnsiValue(44));
        debug_assert_eq!(theme.file_type.char_device, Color::AnsiValue(172));
        debug_assert_eq!(theme.file_type.socket, Color::AnsiValue(44));
        debug_assert_eq!(theme.file_type.special, Color::AnsiValue(44));
        debug_assert_eq!(theme.date.hour_old, Color::AnsiValue(40));
        debug_assert_eq!(theme.date.day_old, Color::AnsiValue(42));
        debug_assert_eq!(theme.date.older, Color::AnsiValue(36));
        debug_assert_eq!(theme.size.none, Color::AnsiValue(245));
        debug_assert_eq!(theme.size.small, Color::AnsiValue(229));
        debug_assert_eq!(theme.size.medium, Color::AnsiValue(216));
        debug_assert_eq!(theme.size.large, Color::AnsiValue(172));
        debug_assert_eq!(theme.inode.valid, Color::AnsiValue(13));
        debug_assert_eq!(theme.inode.invalid, Color::AnsiValue(245));
        debug_assert_eq!(theme.links.valid, Color::AnsiValue(13));
        debug_assert_eq!(theme.links.invalid, Color::AnsiValue(245));
        debug_assert_eq!(theme.tree_edge, Color::AnsiValue(245));
        let _rug_ed_tests_llm_16_19_rrrruuuugggg_test_default_theme = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_158 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_path_with_absolute_path() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_from_path_with_absolute_path = 0;
        let rug_fuzz_0 = "/path/to/theme.yaml";
        let file = rug_fuzz_0;
        let theme = Theme::from_path(file);
        debug_assert!(theme.is_some());
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_from_path_with_absolute_path = 0;
    }
    #[test]
    fn test_from_path_with_relative_path() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_from_path_with_relative_path = 0;
        let rug_fuzz_0 = "theme.yaml";
        let file = rug_fuzz_0;
        let theme = Theme::from_path(file);
        debug_assert!(theme.is_some());
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_from_path_with_relative_path = 0;
    }
    #[test]
    fn test_from_path_with_invalid_path() {
        let _rug_st_tests_llm_16_158_rrrruuuugggg_test_from_path_with_invalid_path = 0;
        let rug_fuzz_0 = "invalid_path";
        let file = rug_fuzz_0;
        let theme = Theme::from_path(file);
        debug_assert!(theme.is_none());
        let _rug_ed_tests_llm_16_158_rrrruuuugggg_test_from_path_with_invalid_path = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_159 {
    use super::*;
    use crate::*;
    use serde_yaml::Error;
    #[test]
    fn test_with_yaml() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_with_yaml = 0;
        let rug_fuzz_0 = r#"
            name: Test Theme
            background: '#000000'
            foreground: '#ffffff'
            accent: '#ff0000'
        "#;
        let yaml = rug_fuzz_0;
        let theme = Theme::with_yaml(yaml);
        debug_assert!(theme.is_ok());
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_with_yaml = 0;
    }
    #[test]
    fn test_with_yaml_invalid_yaml() {
        let _rug_st_tests_llm_16_159_rrrruuuugggg_test_with_yaml_invalid_yaml = 0;
        let rug_fuzz_0 = "invalid yaml";
        let yaml = rug_fuzz_0;
        let theme = Theme::with_yaml(yaml);
        debug_assert!(theme.is_err());
        let _rug_ed_tests_llm_16_159_rrrruuuugggg_test_with_yaml_invalid_yaml = 0;
    }
}
#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::color::theme::Symlink;
    use crate::color::Color;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_30_rrrruuuugggg_test_rug = 0;
        <Symlink as Default>::default();
        let _rug_ed_tests_rug_30_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::color::theme::{
        Theme, Color, Permission, FileType, Date, Size, INode, Links,
    };
    #[test]
    fn test_default_dark() {
        let _rug_st_tests_rug_32_rrrruuuugggg_test_default_dark = 0;
        Theme::default_dark();
        let _rug_ed_tests_rug_32_rrrruuuugggg_test_default_dark = 0;
    }
}
