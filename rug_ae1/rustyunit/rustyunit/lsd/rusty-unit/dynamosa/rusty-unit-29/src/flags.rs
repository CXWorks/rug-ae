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
fn rusty_test_316() {
    rusty_monitor::set_test_id(316);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut bool_0: bool = false;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5128() {
    rusty_monitor::set_test_id(5128);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_1_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut option_8: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_9: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7146() {
    rusty_monitor::set_test_id(7146);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 2850u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7585() {
    rusty_monitor::set_test_id(7585);
    let mut str_0: &str = "XB4mnwa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_0: u64 = 2475u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_0: bool = false;
    let mut u64_1: u64 = 2850u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_3_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_6: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_7: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_2: u64 = 3313u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_5_ref_0);
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut option_11: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut str_1: &str = "XvpVH3KTEucv8K";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_3: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::clone(flags_2_ref_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut elem_1: color::Elem = crate::color::Elem::FileLarge;
    let mut string_2: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_1_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4532() {
    rusty_monitor::set_test_id(4532);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 6413usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut str_0: &str = "UuHNmVX69";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut option_1: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_2: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_7: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_12: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut option_13: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_1: usize = 4038usize;
    let mut option_14: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8016() {
    rusty_monitor::set_test_id(8016);
    let mut u64_0: u64 = 3278u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_8: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_9: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_1: u64 = 3313u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_4_ref_0);
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_16: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_17: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut bool_0: bool = false;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_13: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_13};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut bool_14: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_18: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_17, theme: option_16, separator: option_15};
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::clone(flags_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_406() {
    rusty_monitor::set_test_id(406);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_0: u64 = 7159u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_1_ref_0);
    let mut str_0: &str = "xzIGoZY8fzmH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_388() {
    rusty_monitor::set_test_id(388);
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 7159u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut str_0: &str = "xzIGoZY8fzmH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Group;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_494() {
    rusty_monitor::set_test_id(494);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut bool_0: bool = true;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6999() {
    rusty_monitor::set_test_id(6999);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 118u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_1: u64 = 3313u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_4_ref_0);
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut option_12: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_16: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_17: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::clone(flags_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_207() {
    rusty_monitor::set_test_id(207);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut bool_0: bool = false;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = true;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_1};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5996() {
    rusty_monitor::set_test_id(5996);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut u64_0: u64 = 2850u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_8: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_9: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6000() {
    rusty_monitor::set_test_id(6000);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut sorting_0_ref_0: &crate::flags::sorting::Sorting = &mut sorting_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_0: u64 = 8273u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_1_ref_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sortcolumn_1_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_1;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut u64_1: u64 = 2850u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_1: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_5_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut option_9: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_10: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut sortcolumn_2_ref_0: &flags::sorting::SortColumn = &mut sortcolumn_2;
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2912() {
    rusty_monitor::set_test_id(2912);
    let mut u64_0: u64 = 3278u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::None;
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_8: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_9: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_1: u64 = 3313u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_4_ref_0);
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut option_13: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_14: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut bool_2: bool = false;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_3: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_3};
    let mut contentstyle_0: crossterm::style::ContentStyle = crate::color::Colors::default_style();
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_19: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_2};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::clone(flags_1_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::DayOld;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6102() {
    rusty_monitor::set_test_id(6102);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 4159u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 7427u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut str_0: &str = "cWghqsSD1";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_2: u64 = 2638u64;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_3: u64 = 6380u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_3_ref_0);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Formatted(string_2);
    let mut dateflag_0_ref_0: &flags::date::DateFlag = &mut dateflag_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_1: bool = false;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_3: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_6_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut option_7: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_8: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut flags_8: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_8_ref_0: &crate::flags::Flags = &mut flags_8;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8442() {
    rusty_monitor::set_test_id(8442);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_1: bool = false;
    let mut u64_0: u64 = 2850u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut option_6: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_7: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_8: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_1: u64 = 3313u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_4_ref_0);
    let mut option_10: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_3, exec: bool_2};
    let mut option_11: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut bool_4: bool = false;
    let mut str_0: &str = "XvpVH3KTEucv8K";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut bool_16: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_16, user_write: bool_15, user_execute: bool_14, group_read: bool_13, group_write: bool_12, group_execute: bool_11, other_read: bool_10, other_write: bool_9, other_execute: bool_8, sticky: bool_7, setgid: bool_6, setuid: bool_5};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut bool_17: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::Links {valid: bool_17};
    let mut bool_18: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_17: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    crate::meta::filetype::FileType::render(filetype_1, colors_1_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::clone(flags_1_ref_0);
    let mut bool_19: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
    panic!("From RustyUnit with love");
}
}