pub mod blocks;
pub mod color;
pub mod date;
pub mod dereference;
pub mod display;
pub mod hyperlink;
pub mod icons;
pub mod ignore_globs;
pub mod indicators;
pub mod layout;
pub mod permission;
pub mod recursion;
pub mod size;
pub mod sorting;
pub mod symlink_arrow;
pub mod symlinks;
pub mod total_size;

pub use blocks::Block;
pub use blocks::Blocks;
pub use color::Color;
pub use color::{ColorOption, ThemeOption};
pub use date::DateFlag;
pub use dereference::Dereference;
pub use display::Display;
pub use hyperlink::HyperlinkOption;
pub use icons::IconOption;
pub use icons::IconSeparator;
pub use icons::IconTheme;
pub use icons::Icons;
pub use ignore_globs::IgnoreGlobs;
pub use indicators::Indicators;
pub use layout::Layout;
pub use permission::PermissionFlag;
pub use recursion::Recursion;
pub use size::SizeFlag;
pub use sorting::DirGrouping;
pub use sorting::SortColumn;
pub use sorting::SortOrder;
pub use sorting::Sorting;
pub use symlink_arrow::SymlinkArrow;
pub use symlinks::NoSymlink;
pub use total_size::TotalSize;

use crate::config_file::Config;

use clap::{ArgMatches, Error};

#[cfg(doc)]
use yaml_rust::Yaml;

/// A struct to hold all set configuration flags for the application.
#[derive(Clone, Debug, Default)]
pub struct Flags {
    pub blocks: Blocks,
    pub color: Color,
    pub date: DateFlag,
    pub dereference: Dereference,
    pub display: Display,
    pub display_indicators: Indicators,
    pub icons: Icons,
    pub ignore_globs: IgnoreGlobs,
    pub layout: Layout,
    pub no_symlink: NoSymlink,
    pub recursion: Recursion,
    pub size: SizeFlag,
    pub permission: PermissionFlag,
    pub sorting: Sorting,
    pub total_size: TotalSize,
    pub symlink_arrow: SymlinkArrow,
    pub hyperlink: HyperlinkOption,
}

impl Flags {
    /// Set up the `Flags` from either [ArgMatches], a [Config] or its [Default] value.
    ///
    /// # Errors
    ///
    /// This can return an [Error], when either the building of the ignore globs or the parsing of
    /// the recursion depth parameter fails.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        Ok(Self {
            blocks: Blocks::configure_from(matches, config)?,
            color: Color::configure_from(matches, config),
            date: DateFlag::configure_from(matches, config),
            dereference: Dereference::configure_from(matches, config),
            display: Display::configure_from(matches, config),
            layout: Layout::configure_from(matches, config),
            size: SizeFlag::configure_from(matches, config),
            permission: PermissionFlag::configure_from(matches, config),
            display_indicators: Indicators::configure_from(matches, config),
            icons: Icons::configure_from(matches, config),
            ignore_globs: IgnoreGlobs::configure_from(matches, config)?,
            no_symlink: NoSymlink::configure_from(matches, config),
            recursion: Recursion::configure_from(matches, config)?,
            sorting: Sorting::configure_from(matches, config),
            total_size: TotalSize::configure_from(matches, config),
            symlink_arrow: SymlinkArrow::configure_from(matches, config),
            hyperlink: HyperlinkOption::configure_from(matches, config),
        })
    }
}

/// A trait to allow a type to be configured by either command line parameters, a configuration
/// file or a [Default] value.
pub trait Configurable<T>
where
    T: std::default::Default,
{
    /// Returns a value from either [ArgMatches], a [Config], a [Default] or the environment value.
    /// The first value that is not [None] is used. The order of precedence for the value used is:
    /// - [from_arg_matches](Configurable::from_arg_matches)
    /// - [from_environment](Configurable::from_environment)
    /// - [from_config](Configurable::from_config)
    /// - [Default::default]
    ///
    /// # Note
    ///
    /// The configuration file's Yaml is read in any case, to be able to check for errors and print
    /// out warnings.
    fn configure_from(matches: &ArgMatches, config: &Config) -> T {
        if let Some(value) = Self::from_arg_matches(matches) {
            return value;
        }

        if let Some(value) = Self::from_environment() {
            return value;
        }

        if let Some(value) = Self::from_config(config) {
            return value;
        }

        Default::default()
    }

    /// The method to implement the value fetching from command line parameters.
    fn from_arg_matches(matches: &ArgMatches) -> Option<T>;

    /// The method to implement the value fetching from a configuration file. This should return
    /// [None], if the [Config] does not have a [Yaml].
    fn from_config(config: &Config) -> Option<T>;

    /// The method to implement the value fetching from environment variables.
    fn from_environment() -> Option<T> {
        None
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
fn rusty_test_1149() {
    rusty_monitor::set_test_id(1149);
    let mut usize_0: usize = 58usize;
    let mut bool_0: bool = true;
    let mut u64_0: u64 = 91u64;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 8usize;
    let mut bool_3: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_5: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_11: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_25: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_11, permission: option_10, sorting: option_9, no_symlink: option_8, total_size: option_7, symlink_arrow: option_6, hyperlink: option_5};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "GWIMu29";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_17, user_write: bool_16, user_execute: bool_15, group_read: bool_14, group_write: bool_13, group_execute: bool_12, other_read: bool_11, other_write: bool_10, other_execute: bool_9, sticky: bool_8, setgid: bool_7, setuid: bool_6};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    crate::meta::permissions::Permissions::render(permissions_0_ref_0, colors_1_ref_0, flags_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2520() {
    rusty_monitor::set_test_id(2520);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 14u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 84u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_1_ref_0);
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 32usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut string_2: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_3_ref_0);
    crate::meta::owner::Owner::render_user(owner_0_ref_0, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_1: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2705() {
    rusty_monitor::set_test_id(2705);
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut usize_0: usize = 71usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_12: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 94u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    crate::meta::date::Date::render(date_0_ref_0, colors_1_ref_0, flags_1_ref_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    crate::meta::size::Size::render(size_0_ref_0, colors_0_ref_0, flags_0_ref_0, option_2);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut elem_1: color::Elem = crate::color::Elem::SymLink;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1696() {
    rusty_monitor::set_test_id(1696);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_12: bool = true;
    let mut option_1: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut bool_22: bool = true;
    let mut bool_23: bool = false;
    let mut bool_24: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_24, user_write: bool_23, user_execute: bool_22, group_read: bool_21, group_write: bool_20, group_execute: bool_19, other_read: bool_18, other_write: bool_17, other_execute: bool_16, sticky: bool_15, setgid: bool_14, setuid: bool_13};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut bool_25: bool = crate::meta::permissions::Permissions::is_executable(permissions_1_ref_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_3, reverse: option_2, dir_grouping: option_1};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_12};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4309() {
    rusty_monitor::set_test_id(4309);
    let mut usize_0: usize = 31usize;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 47usize;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "IP3NAqJJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_14: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_17: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut bool_5: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_21: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut option_23: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_23, reverse: option_22, dir_grouping: option_21};
    let mut option_24: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_25: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_26: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_27: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_28: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut option_30: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_31: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_32: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_33: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_34: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_35: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_36: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_37: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_37, blocks: option_36, color: option_35, date: option_34, dereference: option_33, display: option_32, icons: option_31, ignore_globs: option_30, indicators: option_29, layout: option_28, recursion: option_27, size: option_26, permission: option_25, sorting: option_24, no_symlink: option_20, total_size: option_19, symlink_arrow: option_18, hyperlink: option_17};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 30u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_2: usize = 76usize;
    let mut option_38: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_1: u64 = 91u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    crate::meta::size::Size::render(size_1_ref_0, colors_0_ref_0, flags_0_ref_0, option_38);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4599() {
    rusty_monitor::set_test_id(4599);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 62usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_0: u64 = 3u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_1_ref_0);
    let mut str_0: &str = "cbQ6ksj4Yv9t";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut str_1: &str = "Z6d9S";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_3);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut core_1: crate::core::Core = crate::core::Core::new(flags_2);
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut core_0_ref_0: &crate::core::Core = &mut core_0;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3758() {
    rusty_monitor::set_test_id(3758);
    let mut usize_0: usize = 17usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 29u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut option_2: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut option_4: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_1_ref_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_4, reverse: option_3, dir_grouping: option_2};
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_0_ref_0, flags_0_ref_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut elem_2: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2542() {
    rusty_monitor::set_test_id(2542);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut usize_0: usize = 97usize;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 24usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_7: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_5, depth: option_4};
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_8: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_11: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_12: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_12, theme: option_11, separator: option_10};
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_9: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_18, theme: option_17};
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_21, user_write: bool_20, user_execute: bool_19, group_read: bool_18, group_write: bool_17, group_execute: bool_16, other_read: bool_15, other_write: bool_14, other_execute: bool_13, sticky: bool_12, setgid: bool_11, setuid: bool_10};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_22: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_33, user_write: bool_32, user_execute: bool_31, group_read: bool_30, group_write: bool_29, group_execute: bool_28, other_read: bool_27, other_write: bool_26, other_execute: bool_25, sticky: bool_24, setgid: bool_23, setuid: bool_22};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    crate::meta::permissions::Permissions::render(permissions_1_ref_0, colors_0_ref_0, flags_1_ref_0);
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_429() {
    rusty_monitor::set_test_id(429);
    let mut usize_0: usize = 29usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_5: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_5, theme: option_4, separator: option_3};
    let mut option_6: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_7: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_5: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_13: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_17: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_19, reverse: option_18, dir_grouping: option_17};
    let mut option_20: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_21: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 22usize;
    let mut option_23: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_24, depth: option_23};
    let mut option_25: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_26: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_28: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_29: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_30: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_31: std::option::Option<bool> = std::option::Option::None;
    let mut option_32: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_33: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_34: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_34, theme: option_33};
    let mut option_35: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_36: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_37: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_37, blocks: option_36, color: option_35, date: option_32, dereference: option_31, display: option_30, icons: option_29, ignore_globs: option_28, indicators: option_27, layout: option_26, recursion: option_25, size: option_22, permission: option_21, sorting: option_20, no_symlink: option_16, total_size: option_15, symlink_arrow: option_14, hyperlink: option_13};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_19, user_write: bool_18, user_execute: bool_17, group_read: bool_16, group_write: bool_15, group_execute: bool_14, other_read: bool_13, other_write: bool_12, other_execute: bool_11, sticky: bool_10, setgid: bool_9, setuid: bool_8};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    crate::meta::permissions::Permissions::render(permissions_0_ref_0, colors_0_ref_0, flags_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5145() {
    rusty_monitor::set_test_id(5145);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut bool_0: bool = true;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 20u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut theme_3: icon::Theme = crate::icon::Theme::Fancy;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_3, string_0);
    let mut icons_0_ref_0: &crate::icon::Icons = &mut icons_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_1};
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_1: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_1);
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut option_1: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut date_1: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2241() {
    rusty_monitor::set_test_id(2241);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_0: u64 = 10u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_0_ref_0, flags_2_ref_0);
    let mut unit_0_ref_0: &meta::size::Unit = &mut unit_0;
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Giga;
    let mut elem_2: color::Elem = crate::color::Elem::Group;
    let mut elem_3: color::Elem = crate::color::Elem::Acl;
    let mut block_2: flags::blocks::Block = crate::flags::blocks::Block::Name;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1001() {
    rusty_monitor::set_test_id(1001);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 51usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "AEN062vVB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_3: bool = false;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_1: &str = "FsEyGr5CcHoYX";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_15, user_write: bool_14, user_execute: bool_13, group_read: bool_12, group_write: bool_11, group_execute: bool_10, other_read: bool_9, other_write: bool_8, other_execute: bool_7, sticky: bool_6, setgid: bool_5, setuid: bool_4};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_17: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_16: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut str_2: &str = "xEKN6qdKnvJVLw";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut vec_0: std::vec::Vec<std::path::PathBuf> = std::vec::Vec::new();
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    crate::core::Core::run(core_0, vec_0);
    let mut elem_0: color::Elem = crate::color::Elem::NonFile;
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1124() {
    rusty_monitor::set_test_id(1124);
    let mut usize_0: usize = 45usize;
    let mut bool_0: bool = false;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_1: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_2: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_22: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_22, theme: option_21};
    let mut option_23: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_24: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_25, blocks: option_24, color: option_23, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 69u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_16, user_write: bool_15, user_execute: bool_14, group_read: bool_13, group_write: bool_12, group_execute: bool_11, other_read: bool_10, other_write: bool_9, other_execute: bool_8, sticky: bool_7, setgid: bool_6, setuid: bool_5};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_26: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    panic!("From RustyUnit with love");
}
}