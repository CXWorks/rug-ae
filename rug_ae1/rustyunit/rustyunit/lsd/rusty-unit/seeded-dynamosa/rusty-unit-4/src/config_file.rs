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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1174() {
//    rusty_monitor::set_test_id(1174);
    let mut u64_0: u64 = 1048576u64;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut sorting_0_ref_0: &crate::config_file::Sorting = &mut sorting_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 8usize;
    let mut bool_1: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut str_0: &str = "rds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_2};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_3: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_4, theme: option_3};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_3: bool = crate::config_file::Config::ne(config_3_ref_0, config_0_ref_0);
    let mut tuple_0: () = crate::config_file::Color::assert_receiver_is_total_eq(color_1_ref_0);
    let mut tuple_1: () = crate::config_file::Sorting::assert_receiver_is_total_eq(sorting_0_ref_0);
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5896() {
//    rusty_monitor::set_test_id(5896);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 46usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 45usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_1};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 8usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_3: usize = 8usize;
    let mut bool_3: bool = false;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_5: bool = crate::config_file::Config::eq(config_3_ref_0, config_2_ref_0);
    let mut bool_6: bool = crate::config_file::Config::eq(config_1_ref_0, config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8851() {
//    rusty_monitor::set_test_id(8851);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut bool_2: bool = crate::config_file::Config::eq(config_2_ref_0, config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9159() {
//    rusty_monitor::set_test_id(9159);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 46usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 45usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 8usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_3, dir_grouping: dirgrouping_3};
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_3: usize = 8usize;
    let mut bool_3: bool = false;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_3};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_5: bool = crate::config_file::Config::eq(config_1_ref_0, config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4076() {
//    rusty_monitor::set_test_id(4076);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut bool_1: bool = crate::config_file::Config::ne(config_1_ref_0, config_2_ref_0);
    let mut tuple_0: () = crate::config_file::Color::assert_receiver_is_total_eq(color_1_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2933() {
//    rusty_monitor::set_test_id(2933);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut elem_0: color::Elem = crate::color::Elem::Context;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut option_3: std::option::Option<usize> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_4, depth: option_3};
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut tuple_0: () = crate::config_file::Config::assert_receiver_is_total_eq(config_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2085() {
//    rusty_monitor::set_test_id(2085);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_0: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut sorting_1_ref_0: &crate::config_file::Sorting = &mut sorting_1;
    let mut option_6: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_8: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_2: crate::config_file::Sorting = crate::config_file::Sorting {column: option_8, reverse: option_7, dir_grouping: option_6};
    let mut sorting_2_ref_0: &crate::config_file::Sorting = &mut sorting_2;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_3: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_1, dir_grouping: dirgrouping_2};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 8usize;
    let mut bool_2: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut str_0: &str = "rds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_3: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_9: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_10, theme: option_9};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_4: bool = crate::config_file::Config::ne(config_3_ref_0, config_2_ref_0);
    let mut tuple_0: () = crate::config_file::Color::assert_receiver_is_total_eq(color_1_ref_0);
    let mut bool_5: bool = crate::config_file::Sorting::eq(sorting_2_ref_0, sorting_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_371() {
//    rusty_monitor::set_test_id(371);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8478() {
//    rusty_monitor::set_test_id(8478);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut usize_0: usize = 120usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_2: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut recursion_0_ref_0: &crate::config_file::Recursion = &mut recursion_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut elem_2: color::Elem = crate::color::Elem::CharDevice;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut recursion_1_ref_0: &crate::config_file::Recursion = &mut recursion_1;
    let mut bool_5: bool = crate::config_file::Recursion::ne(recursion_1_ref_0, recursion_0_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5589() {
//    rusty_monitor::set_test_id(5589);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 8usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut str_0: &str = "rds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::default();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut bool_2: bool = crate::config_file::Config::ne(config_3_ref_0, config_2_ref_0);
//    panic!("From RustyUnit with love");
}
}