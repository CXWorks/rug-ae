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
fn rusty_test_5817() {
    rusty_monitor::set_test_id(5817);
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::HourOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_0, exec: bool_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4265() {
    rusty_monitor::set_test_id(4265);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 50u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 86u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 25u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_2_ref_0, flags_2_ref_0);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_3: u64 = 64u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_3_ref_0, flags_3_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    crate::meta::size::Size::render(size_0_ref_0, colors_0_ref_0, flags_0_ref_0, option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5880() {
    rusty_monitor::set_test_id(5880);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 25u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_1_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 64u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_2_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_1_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2064() {
    rusty_monitor::set_test_id(2064);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 3u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_1_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 64u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_668() {
    rusty_monitor::set_test_id(668);
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 25u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_0_ref_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 64u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_1, string_0);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_0_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5700() {
    rusty_monitor::set_test_id(5700);
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_2: u64 = 25u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7094() {
    rusty_monitor::set_test_id(7094);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5623() {
    rusty_monitor::set_test_id(5623);
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_1: color::Elem = crate::color::Elem::Older;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut elem_2: color::Elem = crate::color::Elem::HourOld;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_5);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7687() {
    rusty_monitor::set_test_id(7687);
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 25u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 64u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_2_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_0_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7145() {
    rusty_monitor::set_test_id(7145);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_0: u64 = 10u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_1: u64 = 97u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_2: u64 = 25u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_2_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_3: u64 = 64u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_4_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_0_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5583() {
    rusty_monitor::set_test_id(5583);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 22u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 89usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_0: &str = "6Luv9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_1: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut str_1: &str = "2Wdw528Om3DvfQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut option_21: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_23, blocks: option_22, color: option_21, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    crate::meta::filetype::FileType::render(filetype_3, colors_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1116() {
    rusty_monitor::set_test_id(1116);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::NonFile;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_13, user_write: bool_12, user_execute: bool_11, group_read: bool_10, group_write: bool_9, group_execute: bool_8, other_read: bool_7, other_write: bool_6, other_execute: bool_5, sticky: bool_4, setgid: bool_3, setuid: bool_2};
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6824() {
    rusty_monitor::set_test_id(6824);
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_1: bool = true;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_328() {
    rusty_monitor::set_test_id(328);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut u64_0: u64 = 25u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 64u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5826() {
    rusty_monitor::set_test_id(5826);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_1_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_5: crate::color::Colors = crate::color::Colors::new(themeoption_5);
    let mut colors_5_ref_0: &crate::color::Colors = &mut colors_5;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_3_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2633() {
    rusty_monitor::set_test_id(2633);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 68u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 86u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_1_ref_0, flags_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 25u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_2_ref_0, flags_2_ref_0);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_3: u64 = 64u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_3_ref_0: &crate::meta::size::Size = &mut size_3;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_3_ref_0, flags_3_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    crate::meta::size::Size::render_unit(size_0_ref_0, colors_0_ref_0, flags_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2188() {
    rusty_monitor::set_test_id(2188);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_1: u64 = 25u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_1_ref_0);
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_2: u64 = 64u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_2_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_2_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4484() {
    rusty_monitor::set_test_id(4484);
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1337() {
    rusty_monitor::set_test_id(1337);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Special;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut date_0: crate::color::theme::Date = crate::color::theme::Date {hour_old: color_2, day_old: color_1, older: color_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_3: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 35u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut date_0_ref_0: &crate::color::theme::Date = &mut date_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4698() {
    rusty_monitor::set_test_id(4698);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_0_ref_0);
    let mut theme_0: icon::Theme = crate::icon::Theme::Fancy;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_1);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = true;
    let mut bool_17: bool = false;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = true;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_23, user_write: bool_22, user_execute: bool_21, group_read: bool_20, group_write: bool_19, group_execute: bool_18, other_read: bool_17, other_write: bool_16, other_execute: bool_15, sticky: bool_14, setgid: bool_13, setuid: bool_12};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut u64_1: u64 = 25u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut string_2: std::string::String = crate::meta::size::Size::value_string(size_1_ref_0, flags_2_ref_0);
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_2: u64 = 64u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut string_3: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_3_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_3, string_2);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_1_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icons_0: crate::icon::Icons = crate::icon::Icons::new(theme_0, string_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3562() {
    rusty_monitor::set_test_id(3562);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_0_ref_0: &crate::flags::Flags = &mut flags_0;
    let mut u64_0: u64 = 86u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_0: std::string::String = crate::meta::size::Size::unit_string(size_0_ref_0, flags_0_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5547() {
    rusty_monitor::set_test_id(5547);
    let mut u64_0: u64 = 98u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut flags_0: crate::flags::Flags = crate::flags::Flags::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut flags_1: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_1_ref_0: &crate::flags::Flags = &mut flags_1;
    let mut flags_2: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_2_ref_0: &crate::flags::Flags = &mut flags_2;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut string_0: std::string::String = crate::meta::date::Date::date_string(date_0_ref_0, flags_2_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Custom(string_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut flags_3: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_3_ref_0: &crate::flags::Flags = &mut flags_3;
    let mut u64_1: u64 = 10u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut flags_4: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_4_ref_0: &crate::flags::Flags = &mut flags_4;
    let mut u64_2: u64 = 97u64;
    let mut size_2: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_2_ref_0: &crate::meta::size::Size = &mut size_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_2: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_3: crate::color::Colors = crate::color::Colors::new(themeoption_3);
    let mut colors_3_ref_0: &crate::color::Colors = &mut colors_3;
    let mut flags_5: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_5_ref_0: &crate::flags::Flags = &mut flags_5;
    let mut u64_3: u64 = 25u64;
    let mut size_3: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut string_1: std::string::String = crate::meta::size::Size::value_string(size_0_ref_0, flags_5_ref_0);
    let mut flags_6: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_6_ref_0: &crate::flags::Flags = &mut flags_6;
    let mut u64_4: u64 = 64u64;
    let mut size_4: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_4_ref_0: &crate::meta::size::Size = &mut size_4;
    let mut string_2: std::string::String = crate::meta::size::Size::unit_string(size_2_ref_0, flags_4_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_4: crate::color::Colors = crate::color::Colors::new(themeoption_4);
    let mut colors_4_ref_0: &crate::color::Colors = &mut colors_4;
    let mut flags_7: crate::flags::Flags = crate::flags::Flags::default();
    let mut flags_7_ref_0: &crate::flags::Flags = &mut flags_7;
    let mut owner_0: crate::meta::owner::Owner = crate::meta::owner::Owner::new(string_2, string_1);
    let mut owner_0_ref_0: &crate::meta::owner::Owner = &mut owner_0;
    crate::meta::owner::Owner::render_group(owner_0_ref_0, colors_3_ref_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    crate::meta::filetype::FileType::render(filetype_1, colors_2_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_1_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Size::get_unit(size_1_ref_0, flags_3_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut core_0: crate::core::Core = crate::core::Core::new(flags_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    panic!("From RustyUnit with love");
}
}