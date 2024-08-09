use crate::flags::display::Display;
use crate::flags::icons::{IconOption, IconTheme};
use crate::flags::layout::Layout;
use crate::flags::permission::PermissionFlag;
use crate::flags::size::SizeFlag;
use crate::flags::sorting::{DirGrouping, SortColumn};
use crate::flags::HyperlinkOption;
use crate::flags::{ColorOption, ThemeOption};
///! This module provides methods to handle the program's config files and operations related to
///! this.
use crate::print_error;

use std::path::{Path, PathBuf};

use serde::Deserialize;

use std::fs;

const CONF_DIR: &str = "lsd";
const CONF_FILE_NAME: &str = "config";
const YAML_LONG_EXT: &str = "yaml";

/// A struct to hold an optional configuration items, and provides methods
/// around error handling in a config file.
#[derive(Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub classic: Option<bool>,
    pub blocks: Option<Vec<String>>,
    pub color: Option<Color>,
    pub date: Option<String>,
    pub dereference: Option<bool>,
    pub display: Option<Display>,
    pub icons: Option<Icons>,
    pub ignore_globs: Option<Vec<String>>,
    pub indicators: Option<bool>,
    pub layout: Option<Layout>,
    pub recursion: Option<Recursion>,
    pub size: Option<SizeFlag>,
    pub permission: Option<PermissionFlag>,
    pub sorting: Option<Sorting>,
    pub no_symlink: Option<bool>,
    pub total_size: Option<bool>,
    pub symlink_arrow: Option<String>,
    pub hyperlink: Option<HyperlinkOption>,
}

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub struct Color {
    pub when: Option<ColorOption>,
    pub theme: Option<ThemeOption>,
}

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub struct Icons {
    pub when: Option<IconOption>,
    pub theme: Option<IconTheme>,
    pub separator: Option<String>,
}

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub struct Recursion {
    pub enabled: Option<bool>,
    pub depth: Option<usize>,
}

#[derive(Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Sorting {
    pub column: Option<SortColumn>,
    pub reverse: Option<bool>,
    pub dir_grouping: Option<DirGrouping>,
}

impl Config {
    /// This constructs a Config struct with all None
    pub fn with_none() -> Self {
        Self {
            classic: None,
            blocks: None,
            color: None,
            date: None,
            dereference: None,
            display: None,
            icons: None,
            ignore_globs: None,
            indicators: None,
            layout: None,
            recursion: None,
            size: None,
            permission: None,
            sorting: None,
            no_symlink: None,
            total_size: None,
            symlink_arrow: None,
            hyperlink: None,
        }
    }

    /// This constructs a Config struct with a passed file path [String].
    pub fn from_file(file: String) -> Option<Self> {
        match fs::read(&file) {
            Ok(f) => match Self::from_yaml(&String::from_utf8_lossy(&f)) {
                Ok(c) => Some(c),
                Err(e) => {
                    print_error!("Configuration file {} format error, {}.", &file, e);
                    None
                }
            },
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => print_error!("Can not open config file {}: {}.", &file, e),
                };
                None
            }
        }
    }

    /// This constructs a Config struct with a passed [Yaml] str.
    /// If error happened, return the [serde_yaml::Error].
    fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(yaml)
    }

    /// This provides the path for a configuration file, according to the XDG_BASE_DIRS specification.
    /// return None if error like PermissionDenied
    #[cfg(not(windows))]
    pub fn config_file_path() -> Option<PathBuf> {
        use xdg::BaseDirectories;
        match BaseDirectories::with_prefix(CONF_DIR) {
            Ok(p) => {
                return Some(p.get_config_home());
            }
            Err(e) => print_error!("Can not open config file: {}.", e),
        }
        None
    }

    /// This provides the path for a configuration file, inside the %APPDATA% directory.
    /// return None if error like PermissionDenied
    #[cfg(windows)]
    pub fn config_file_path() -> Option<PathBuf> {
        if let Some(p) = dirs::config_dir() {
            return Some(p.join(CONF_DIR));
        }
        None
    }

    /// This expand the `~` in path to HOME dir
    /// returns the origin one if no `~` found;
    /// returns None if error happened when getting home dir
    ///
    /// Implementing this to reuse the `dirs` dependency, avoid adding new one
    pub fn expand_home<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
        let p = path.as_ref();
        if !p.starts_with("~") {
            return Some(p.to_path_buf());
        }
        if p == Path::new("~") {
            return dirs::home_dir();
        }
        dirs::home_dir().map(|mut h| {
            if h == Path::new("/") {
                // Corner case: `h` root directory;
                // don't prepend extra `/`, just drop the tilde.
                p.strip_prefix("~").unwrap().to_path_buf()
            } else {
                h.push(p.strip_prefix("~/").unwrap());
                h
            }
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        if let Some(p) = Self::config_file_path() {
            if let Some(c) = Self::from_file(
                p.join([CONF_FILE_NAME, YAML_LONG_EXT].join("."))
                    .to_string_lossy()
                    .to_string(),
            ) {
                return c;
            }
        }
        Self::from_yaml(DEFAULT_CONFIG).unwrap()
    }
}

const DEFAULT_CONFIG: &str = r#"---
# == Classic ==
# This is a shorthand to override some of the options to be backwards compatible
# with `ls`. It affects the "color"->"when", "sorting"->"dir-grouping", "date"
# and "icons"->"when" options.
# Possible values: false, true
classic: false

# == Blocks ==
# This specifies the columns and their order when using the long and the tree
# layout.
# Possible values: permission, user, group, context, size, size_value, date, name, inode
blocks:
  - permission
  - user
  - group
  - size
  - date
  - name

# == Color ==
# This has various color options. (Will be expanded in the future.)
color:
  # When to colorize the output.
  # When "classic" is set, this is set to "never".
  # Possible values: never, auto, always
  when: auto
  # How to colorize the output.
  # When "classic" is set, this is set to "no-color".
  # Possible values: default, no-color, no-lscolors, <theme-file-name>
  # when specifying <theme-file-name>, lsd will look up theme file in
  # XDG Base Directory if relative
  # The file path if absolute
  theme: default

# == Date ==
# This specifies the date format for the date column. The freeform format
# accepts an strftime like string.
# When "classic" is set, this is set to "date".
# Possible values: date, relative, +<date_format>
# date: date

# == Dereference ==
# Whether to dereference symbolic links.
# Possible values: false, true
dereference: false

# == Display ==
# What items to display. Do not specify this for the default behavior.
# Possible values: all, almost-all, directory-only
# display: all

# == Icons ==
icons:
  # When to use icons.
  # When "classic" is set, this is set to "never".
  # Possible values: always, auto, never
  when: auto
  # Which icon theme to use.
  # Possible values: fancy, unicode
  theme: fancy
  # The string between the icons and the name.
  # Possible values: any string (eg: " |")
  separator: " "

# == Ignore Globs ==
# A list of globs to ignore when listing.
# ignore-globs:
#   - .git

# == Indicators ==
# Whether to add indicator characters to certain listed files.
# Possible values: false, true
indicators: false

# == Layout ==
# Which layout to use. "oneline" might be a bit confusing here and should be
# called "one-per-line". It might be changed in the future.
# Possible values: grid, tree, oneline
layout: grid

# == Recursion ==
recursion:
  # Whether to enable recursion.
  # Possible values: false, true
  enabled: false
  # How deep the recursion should go. This has to be a positive integer. Leave
  # it unspecified for (virtually) infinite.
  # depth: 3

# == Size ==
# Specifies the format of the size column.
# Possible values: default, short, bytes
size: default

# == Permission ==
# Specify the format of the permission column.
# Possible value: rwx, octal
permission: rwx

# == Sorting ==
sorting:
  # Specify what to sort by.
  # Possible values: extension, name, time, size, version
  column: name
  # Whether to reverse the sorting.
  # Possible values: false, true
  reverse: false
  # Whether to group directories together and where.
  # When "classic" is set, this is set to "none".
  # Possible values: first, last, none
  dir-grouping: none

# == No Symlink ==
# Whether to omit showing symlink targets
# Possible values: false, true
no-symlink: false

# == Total size ==
# Whether to display the total size of directories.
# Possible values: false, true
total-size: false

# == Hyperlink ==
# Whether to display the total size of directories.
# Possible values: always, auto, never
hyperlink: never

# == Symlink arrow ==
# Specifies how the symlink arrow display, chars in both ascii and utf8
symlink-arrow: ⇒
"#;

#[cfg(test)]
impl Config {
    pub fn builtin() -> Self {
        Self::from_yaml(DEFAULT_CONFIG).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::config_file;
    use crate::flags::color::{ColorOption, ThemeOption};
    use crate::flags::icons::{IconOption, IconTheme};
    use crate::flags::layout::Layout;
    use crate::flags::permission::PermissionFlag;
    use crate::flags::size::SizeFlag;
    use crate::flags::sorting::{DirGrouping, SortColumn};
    use crate::flags::HyperlinkOption;

    #[test]
    fn test_read_default() {
        let c = Config::from_yaml(config_file::DEFAULT_CONFIG).unwrap();
        assert_eq!(
            Config {
                classic: Some(false),
                blocks: Some(
                    vec![
                        "permission".into(),
                        "user".into(),
                        "group".into(),
                        "size".into(),
                        "date".into(),
                        "name".into(),
                    ]
                    .into()
                ),
                color: Some(config_file::Color {
                    when: Some(ColorOption::Auto),
                    theme: Some(ThemeOption::Default)
                }),
                date: None,
                dereference: Some(false),
                display: None,
                icons: Some(config_file::Icons {
                    when: Some(IconOption::Auto),
                    theme: Some(IconTheme::Fancy),
                    separator: Some(" ".to_string()),
                }),
                ignore_globs: None,
                indicators: Some(false),
                layout: Some(Layout::Grid),
                recursion: Some(config_file::Recursion {
                    enabled: Some(false),
                    depth: None,
                }),
                size: Some(SizeFlag::Default),
                permission: Some(PermissionFlag::Rwx),
                sorting: Some(config_file::Sorting {
                    column: Some(SortColumn::Name),
                    reverse: Some(false),
                    dir_grouping: Some(DirGrouping::None),
                }),
                no_symlink: Some(false),
                total_size: Some(false),
                symlink_arrow: Some("⇒".into()),
                hyperlink: Some(HyperlinkOption::Never),
            },
            c
        );
    }

    #[test]
    fn test_read_config_ok() {
        let c = Config::from_yaml("classic: true").unwrap();
        assert!(c.classic.unwrap())
    }

    #[test]
    fn test_read_config_bad_bool() {
        let c = Config::from_yaml("classic: notbool");
        assert!(c.is_err())
    }

    #[test]
    fn test_read_config_file_not_found() {
        let c = Config::from_file("not-existed".to_string());
        assert!(c.is_none())
    }

    #[test]
    fn test_read_bad_display() {
        assert!(Config::from_yaml("display: bad").is_err())
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2046() {
    rusty_monitor::set_test_id(2046);
    let mut usize_0: usize = 4usize;
    let mut bool_0: bool = true;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "0K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 52usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_2: usize = 44usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_3: bool = false;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_19: bool = crate::config_file::Config::ne(config_1_ref_0, config_0_ref_0);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_18, user_write: bool_17, user_execute: bool_16, group_read: bool_15, group_write: bool_14, group_execute: bool_13, other_read: bool_12, other_write: bool_11, other_execute: bool_10, sticky: bool_9, setgid: bool_8, setuid: bool_7};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_766() {
    rusty_monitor::set_test_id(766);
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::Special;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_0: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_6: color::Elem = crate::color::Elem::Read;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_5_ref_0);
    let mut dir_0: crate::color::theme::Dir = crate::color::theme::Dir {uid: color_5, no_uid: color_4};
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_8: color::Elem = crate::color::Elem::DayOld;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut elem_9: color::Elem = crate::color::Elem::Acl;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_9_ref_0, theme_8_ref_0);
    let mut theme_9: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_9_ref_0: &crate::color::theme::Theme = &mut theme_9;
    let mut elem_10: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_10_ref_0, theme_9_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_9, uid_no_exec: color_8, exec_no_uid: color_7, no_exec_no_uid: color_6};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut usize_0: usize = 15usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut usize_1: usize = 5usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut u64_0: u64 = 60u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut elem_11: color::Elem = crate::color::Elem::Special;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut bool_1: bool = crate::config_file::Recursion::eq(recursion_1_ref_0, recursion_0_ref_0);
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_2: bool = crate::config_file::Config::eq(config_2_ref_0, config_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4646() {
    rusty_monitor::set_test_id(4646);
    let mut usize_0: usize = 7usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 36usize;
    let mut u64_0: u64 = 37u64;
    let mut usize_2: usize = 31usize;
    let mut bool_1: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_3: usize = 33usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_3};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_1: u64 = 97u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_16, exec: bool_15};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_4: usize = 45usize;
    let mut bool_17: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_17, depth: usize_4};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut u64_2: u64 = 36u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_19, exec: bool_18};
    let mut config_6: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_6_ref_0: &crate::config_file::Config = &mut config_6;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_20: bool = crate::config_file::Config::eq(config_5_ref_0, config_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1135() {
    rusty_monitor::set_test_id(1135);
    let mut u64_0: u64 = 96u64;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_0_ref_0: &crate::config_file::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 99usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut tuple_0: () = crate::config_file::Sorting::assert_receiver_is_total_eq(sorting_0_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2082() {
    rusty_monitor::set_test_id(2082);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
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
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_3_ref_0: &crate::config_file::Color = &mut color_3;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut str_0: &str = "qj5bxL92HFzhW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::User;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_6, broken: color_5, missing_target: color_4};
    let mut symlink_0_ref_0: &crate::color::theme::Symlink = &mut symlink_0;
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_8: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut theme_8: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_8_ref_0: &crate::color::theme::Theme = &mut theme_8;
    let mut bool_1: bool = true;
    let mut elem_8: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut color_9: crossterm::style::Color = crate::color::Elem::get_color(elem_8_ref_0, theme_8_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sortcolumn_0_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_0;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut symlink_1: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_9, broken: color_8, missing_target: color_7};
    let mut symlink_1_ref_0: &crate::color::theme::Symlink = &mut symlink_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_2: bool = crate::config_file::Config::ne(config_1_ref_0, config_0_ref_0);
    let mut tuple_0: () = crate::config_file::Color::assert_receiver_is_total_eq(color_3_ref_0);
    let mut symlink_2: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1216() {
    rusty_monitor::set_test_id(1216);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut u64_0: u64 = 65u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Write;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut elem_5: color::Elem = crate::color::Elem::File {exec: bool_3, uid: bool_2};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_4: bool = false;
    let mut elem_6: color::Elem = crate::color::Elem::INode {valid: bool_4};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut theme_7: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_7_ref_0: &crate::color::theme::Theme = &mut theme_7;
    let mut elem_7: color::Elem = crate::color::Elem::Special;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut color_7: crossterm::style::Color = crate::color::Elem::get_color(elem_7_ref_0, theme_7_ref_0);
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_0_ref_0: &crate::config_file::Sorting = &mut sorting_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut sorting_1_ref_0: &crate::config_file::Sorting = &mut sorting_1;
    let mut str_0: &str = "A2uKaY8eVyz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut result_0: std::result::Result<crate::config_file::Config, serde_yaml::Error> = crate::config_file::Config::from_yaml(str_0_ref_0);
    let mut bool_5: bool = crate::config_file::Sorting::eq(sorting_1_ref_0, sorting_0_ref_0);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut file_1: crate::color::theme::File = crate::color::theme::File {exec_uid: color_7, uid_no_exec: color_6, exec_no_uid: color_5, no_exec_no_uid: color_4};
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut file_1_ref_0: &crate::color::theme::File = &mut file_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3042() {
    rusty_monitor::set_test_id(3042);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut str_0: &str = "lI5Bd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Older;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut elem_5: color::Elem = crate::color::Elem::Older;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut bool_4: bool = true;
    let mut elem_6: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_6_ref_0, theme_6_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut elem_7: color::Elem = crate::color::Elem::Special;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_5, invalid: color_4};
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut elem_8: color::Elem = crate::color::Elem::BlockDevice;
    let mut result_0: std::result::Result<crate::config_file::Config, serde_yaml::Error> = crate::config_file::Config::from_yaml(str_0_ref_0);
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Tera;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3994() {
    rusty_monitor::set_test_id(3994);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut usize_0: usize = 74usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "0lha0p7CnIfq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "SYuZuJ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_3: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_2: &str = "YUvin";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut tuple_0: () = crate::config_file::Recursion::assert_receiver_is_total_eq(recursion_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1633() {
    rusty_monitor::set_test_id(1633);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_0_ref_0: &crate::config_file::Color = &mut color_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_2: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_3: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_3, theme: option_2};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_6: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_6, theme: option_5, separator: option_4};
    let mut icons_0_ref_0: &crate::config_file::Icons = &mut icons_0;
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_9: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_9, theme: option_8, separator: option_7};
    let mut icons_1_ref_0: &crate::config_file::Icons = &mut icons_1;
    let mut bool_1: bool = true;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_3: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::Acl;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_5, uid_no_exec: color_4, exec_no_uid: color_3, no_exec_no_uid: color_2};
    let mut file_0_ref_0: &crate::color::theme::File = &mut file_0;
    let mut elem_5: color::Elem = crate::color::Elem::Dir {uid: bool_1};
    let mut bool_2: bool = crate::config_file::Icons::eq(icons_1_ref_0, icons_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::Older;
    let mut bool_3: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut elem_7: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut bool_4: bool = crate::config_file::Color::eq(color_1_ref_0, color_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2137() {
    rusty_monitor::set_test_id(2137);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 22usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 89usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_3: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_2);
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_2: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_2);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut str_0: &str = "B3xCdkB6tTxwQX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut color_2_ref_0: &crate::config_file::Color = &mut color_2;
    let mut option_7: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_8: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_8, theme: option_7};
    let mut color_3_ref_0: &crate::config_file::Color = &mut color_3;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut config_6: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_6_ref_0: &crate::config_file::Config = &mut config_6;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_3_ref_0: &flags::color::ColorOption = &mut coloroption_3;
    let mut bool_2: bool = crate::config_file::Color::ne(color_3_ref_0, color_2_ref_0);
    let mut config_7: crate::config_file::Config = crate::config_file::Config::default();
    let mut result_0: std::result::Result<crate::config_file::Config, serde_yaml::Error> = crate::config_file::Config::from_yaml(str_0_ref_0);
    let mut sorting_2: crate::config_file::Sorting = crate::config_file::Sorting {column: option_4, reverse: option_3, dir_grouping: option_2};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4443() {
    rusty_monitor::set_test_id(4443);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_0_ref_0: &crate::config_file::Sorting = &mut sorting_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut bool_0: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut sorting_1_ref_0: &crate::config_file::Sorting = &mut sorting_1;
    let mut usize_0: usize = 44usize;
    let mut bool_1: bool = true;
    let mut option_6: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_7, theme: option_6};
    let mut color_0_ref_0: &crate::config_file::Color = &mut color_0;
    let mut option_8: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_9: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_9, theme: option_8};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
    let mut str_0: &str = "fZ3ahlP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_10: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_11: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_11, theme: option_10};
    let mut color_2_ref_0: &crate::config_file::Color = &mut color_2;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_12: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_13: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_2);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut color_3: crate::config_file::Color = crate::config_file::Color {when: option_13, theme: option_12};
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut color_3_ref_0: &crate::config_file::Color = &mut color_3;
    let mut bool_14: bool = crate::config_file::Color::eq(color_3_ref_0, color_2_ref_0);
    let mut bool_15: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut bool_16: bool = crate::config_file::Color::eq(color_1_ref_0, color_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut bool_17: bool = crate::config_file::Sorting::ne(sorting_1_ref_0, sorting_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3986() {
    rusty_monitor::set_test_id(3986);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 40usize;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 88usize;
    let mut bool_3: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_2: usize = 10usize;
    let mut option_8: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut bool_4: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_7, permission: option_6, sorting: option_5, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_3: usize = 7usize;
    let mut bool_6: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_6, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_23: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_23, theme: option_22, separator: option_21};
    let mut icons_0_ref_0: &crate::config_file::Icons = &mut icons_0;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut tuple_0: () = crate::config_file::Icons::assert_receiver_is_total_eq(icons_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3702() {
    rusty_monitor::set_test_id(3702);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 17usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_2: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_17, blocks: option_16, color: option_15, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 99usize;
    let mut bool_4: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_1};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_0: u64 = 81u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_2: usize = 87usize;
    let mut option_18: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_19, depth: option_18};
    let mut recursion_2_ref_0: &crate::config_file::Recursion = &mut recursion_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut bool_5: bool = crate::config_file::Config::eq(config_4_ref_0, config_3_ref_0);
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4650() {
    rusty_monitor::set_test_id(4650);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_0: u64 = 93u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "AV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut u64_1: u64 = 69u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut tuple_0: () = crate::config_file::Config::assert_receiver_is_total_eq(config_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2224() {
    rusty_monitor::set_test_id(2224);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 63usize;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut option_0: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_2: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_6: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_8: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 37u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_12: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_9: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut bool_10: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_10);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_16: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_18, reverse: option_17, dir_grouping: option_16};
    let mut option_19: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_20: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_22: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_23: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_11: bool = false;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_11);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_27: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_28: std::option::Option<bool> = std::option::Option::None;
    let mut option_29: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_30: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_31: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_12: bool = true;
    let mut option_32: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_32, blocks: option_31, color: option_30, date: option_29, dereference: option_28, display: option_27, icons: option_26, ignore_globs: option_25, indicators: option_24, layout: option_23, recursion: option_22, size: option_21, permission: option_20, sorting: option_19, no_symlink: option_15, total_size: option_14, symlink_arrow: option_13, hyperlink: option_12};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_13: bool = crate::config_file::Config::ne(config_2_ref_0, config_1_ref_0);
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_3, broken: color_2, missing_target: color_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_242() {
    rusty_monitor::set_test_id(242);
    let mut usize_0: usize = 4usize;
    let mut bool_0: bool = false;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 78u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "NupMGwe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut theme_0: icon::Theme = crate::icon::Theme::NoIcon;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 33usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut str_1: &str = "miwc5Dow";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_3: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_1_ref_0: &crate::config_file::Sorting = &mut sorting_1;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut sortorder_1_ref_0: &flags::sorting::SortOrder = &mut sortorder_1;
    let mut sorting_2: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut bool_5: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sorting_2_ref_0: &crate::config_file::Sorting = &mut sorting_2;
    let mut bool_6: bool = crate::config_file::Sorting::eq(sorting_2_ref_0, sorting_1_ref_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut permissionflag_1_ref_0: &flags::permission::PermissionFlag = &mut permissionflag_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3555() {
    rusty_monitor::set_test_id(3555);
    let mut usize_0: usize = 95usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut option_0: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 10u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 64usize;
    let mut bool_3: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_3: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_4: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_6, total_size: option_5, symlink_arrow: option_4, hyperlink: option_3};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut option_21: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_22: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_23: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_23, theme: option_22, separator: option_21};
    let mut icons_0_ref_0: &crate::config_file::Icons = &mut icons_0;
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_26: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_2);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_26, theme: option_25, separator: option_24};
    let mut icons_1_ref_0: &crate::config_file::Icons = &mut icons_1;
    let mut option_27: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_0: &str = "GYWmSAwJvjT2j";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_28: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_3);
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_6: bool = crate::config_file::Icons::ne(icons_1_ref_0, icons_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4045() {
    rusty_monitor::set_test_id(4045);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::User;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut usize_0: usize = 80usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_4, depth: option_3};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut usize_1: usize = 52usize;
    let mut option_5: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_3: bool = false;
    let mut option_6: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_6, depth: option_5};
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut bool_4: bool = crate::config_file::Recursion::eq(recursion_1_ref_0, recursion_0_ref_0);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    let mut unit_1_ref_0: &meta::size::Unit = &mut unit_1;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_3, uid_no_exec: color_2, exec_no_uid: color_1, no_exec_no_uid: color_0};
    let mut elem_4: color::Elem = crate::color::Elem::User;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_202() {
    rusty_monitor::set_test_id(202);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::User;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut usize_0: usize = 46usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_2: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut str_0: &str = "FoNhO9jSLVxIXR";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_3: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut str_1: &str = "x9Xj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut result_0: std::result::Result<crate::config_file::Config, serde_yaml::Error> = crate::config_file::Config::from_yaml(str_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut option_4: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut bool_4: bool = crate::config_file::Recursion::ne(recursion_1_ref_0, recursion_0_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_894() {
    rusty_monitor::set_test_id(894);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_3: color::Elem = crate::color::Elem::Octal;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_2: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_2};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_1_ref_0);
    let mut elem_5: color::Elem = crate::color::Elem::NonFile;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::DayOld;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_3: bool = crate::config_file::Config::ne(config_1_ref_0, config_0_ref_0);
    panic!("From RustyUnit with love");
}
}