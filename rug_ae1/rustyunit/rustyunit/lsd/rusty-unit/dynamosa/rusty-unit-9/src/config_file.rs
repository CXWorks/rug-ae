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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8324() {
    rusty_monitor::set_test_id(8324);
    let mut u64_0: u64 = 88u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_2: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 19u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut u64_2: u64 = 98u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_0: &str = "xeL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut option_14: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_15: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_15, theme: option_14};
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut u64_3: u64 = 5u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_4: u64 = 31u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut date_2: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_2_ref_0: &meta::date::Date = &mut date_2;
    let mut str_1: &str = "ayl64Ov";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_4};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_19: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7663() {
    rusty_monitor::set_test_id(7663);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_12: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    panic!("From RustyUnit with love");
}
}