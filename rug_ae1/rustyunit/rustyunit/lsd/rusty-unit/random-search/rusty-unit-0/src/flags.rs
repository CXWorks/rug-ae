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
	use std::clone::Clone;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2867() {
    rusty_monitor::set_test_id(2867);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 44usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 89u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 50u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3370() {
    rusty_monitor::set_test_id(3370);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 90u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut str_0: &str = "qhrYW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_0, string_0);
    let mut icons_0_ref_0: &crate::icon::Icons = &mut icons_0;
    let mut elem_0: color::Elem = crate::color::Elem::User;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_1: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_1_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_1: u64 = 81u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_2);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut string_3: std::string::String = crate::meta::date::Date::date_string(date_1_ref_0, flags_3_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_2: bool = false;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut elem_2: color::Elem = crate::color::Elem::Exec;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    crate::meta::filetype::FileType::render(filetype_3, colors_2_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_2};
    crate::color::Colors::colorize(colors_1_ref_0, string_3, elem_1_ref_0);
    let mut elem_4: color::Elem = crate::color::Elem::Context;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    crate::color::Colors::colorize(colors_0_ref_0, string_1, elem_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_265() {
    rusty_monitor::set_test_id(265);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 97u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut str_0: &str = "SVMzU1JMjeACbu";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut theme_0: icon::Theme = crate::icon::Theme::Unicode;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_0, string_0);
    let mut icons_0_ref_0: &crate::icon::Icons = &mut icons_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 29usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::clone(flags_2_ref_0);
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::clone(flags_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_675() {
    rusty_monitor::set_test_id(675);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut elem_0: color::Elem = crate::color::Elem::Write;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut usize_0: usize = 30usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_0: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_1: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut elem_2: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut elem_3: color::Elem = crate::color::Elem::Pipe;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_4: color::Elem = crate::color::Elem::NoAccess;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4481() {
    rusty_monitor::set_test_id(4481);
    let mut str_0: &str = "I32mL7ow5HH8Q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Links {valid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_1: bool = false;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 92u64;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
    let mut app_0: clap::App = crate::app::build();
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut elem_4: color::Elem = crate::color::Elem::INode {valid: bool_1};
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_3, small: color_2, medium: color_1, large: color_0};
    let mut unit_1: meta::size::Unit = crate::meta::size::Unit::Tera;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Context;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4353() {
    rusty_monitor::set_test_id(4353);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Socket;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Octal;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut bool_0: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_4: color::Elem = crate::color::Elem::Older;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut usize_0: usize = 14usize;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut file_0: crate::color::theme::File = crate::color::theme::File {exec_uid: color_5, uid_no_exec: color_4, exec_no_uid: color_3, no_exec_no_uid: color_2};
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4815() {
    rusty_monitor::set_test_id(4815);
    let mut usize_0: usize = 27usize;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 75u64;
    let mut str_0: &str = "Aav";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut u64_1: u64 = 21u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_13: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_13);
    let mut bool_14: bool = true;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_14);
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_1: usize = 79usize;
    let mut option_8: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut bool_15: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_15);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_16: bool = true;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_16);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_7, permission: option_6, sorting: option_5, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_2: u64 = 3u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_3: u64 = 33u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut bool_17: bool = true;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_4: u64 = 79u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_3_ref_0, flags_2_ref_0);
    crate::meta::size::Size::render_unit(size_2_ref_0, colors_0_ref_0, flags_0_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2742() {
    rusty_monitor::set_test_id(2742);
    let mut u64_0: u64 = 49u64;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 87usize;
    let mut bool_3: bool = true;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut bool_5: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_6: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_7: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_8: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_1: u64 = 95u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut bool_9: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_9};
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut bool_20: bool = true;
    let mut bool_21: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_21, user_write: bool_20, user_execute: bool_19, group_read: bool_18, group_write: bool_17, group_execute: bool_16, other_read: bool_15, other_write: bool_14, other_execute: bool_13, sticky: bool_12, setgid: bool_11, setuid: bool_10};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_22: bool = true;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_22};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    crate::meta::permissions::Permissions::render(permissions_0_ref_0, colors_0_ref_0, flags_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4231() {
    rusty_monitor::set_test_id(4231);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 64u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut f64_0: f64 = 17.474115f64;
    let mut u64_1: u64 = 98u64;
    let mut option_3: std::option::Option<std::vec::Vec<crate::meta::Meta>> = std::option::Option::None;
    let mut u64_2: u64 = 85u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "aTM8nIsNPM37";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    crate::meta::date::Date::render(date_0_ref_0, colors_1_ref_0, flags_1_ref_0);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1847() {
    rusty_monitor::set_test_id(1847);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 70usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "vkzA9hplKAjg6";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 57u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 67usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_2);
    let mut bool_2: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    crate::meta::size::Size::render_value(size_0_ref_0, colors_0_ref_0, flags_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4041() {
    rusty_monitor::set_test_id(4041);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 33usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut str_0: &str = "T8EzApRdRp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_13: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_13};
    let mut u64_0: u64 = 66u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut u64_1: u64 = 12u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_1_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut vec_0: std::vec::Vec<std::path::PathBuf> = std::vec::Vec::new();
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_2);
    crate::core::Core::run(core_0, vec_0);
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut elem_1: color::Elem = crate::color::Elem::NonFile;
    crate::meta::filetype::FileType::render(filetype_2, colors_0_ref_0);
    let mut core_1: crate::core::Core = crate::core::Core::new(flags_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2309() {
    rusty_monitor::set_test_id(2309);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 25u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 36usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_1_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_2);
    let mut core_0_ref_0: &crate::core::Core = &mut core_0;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1445() {
    rusty_monitor::set_test_id(1445);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 91usize;
    let mut bool_1: bool = true;
    let mut u64_0: u64 = 57u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 72usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut str_0: &str = "AuUefzPYh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_4, exec: bool_3};
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_16, user_write: bool_15, user_execute: bool_14, group_read: bool_13, group_write: bool_12, group_execute: bool_11, other_read: bool_10, other_write: bool_9, other_execute: bool_8, sticky: bool_7, setgid: bool_6, setuid: bool_5};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_17: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_17);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Read;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut option_4: std::option::Option<usize> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut u64_1: u64 = 43u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    crate::meta::size::Size::render(size_1_ref_0, colors_2_ref_0, flags_2_ref_0, option_4);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_2, invalid: color_1};
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    panic!("From RustyUnit with love");
}
}