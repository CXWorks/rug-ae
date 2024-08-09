#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::match_same_arms,
    clippy::cast_possible_wrap
)]
#![feature(no_coverage)]
#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_humanize;
extern crate dirs;
extern crate libc;
extern crate lscolors;
#[cfg(test)]
extern crate tempfile;
extern crate term_grid;
extern crate terminal_size;
extern crate unicode_width;
extern crate url;
extern crate wild;
extern crate xdg;
extern crate yaml_rust;

#[cfg(unix)]
extern crate users;

#[cfg(windows)]
extern crate winapi;

pub mod rusty_monitor;
pub use ntest::timeout;
pub mod app;
pub mod color;
pub mod config_file;
pub mod core;
pub mod display;
pub mod flags;
pub mod icon;
pub mod meta;
pub mod sort;

use crate::config_file::Config;
use crate::core::Core;
use crate::flags::Flags;
use std::path::PathBuf;

/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stderr close before our program.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        {
            use std::io::Write;

            let stderr = std::io::stderr();

            {
                let mut handle = stderr.lock();
                // We can write on stderr, so we simply ignore the error and don't print
                // and stop with success.
                let res = handle.write_all(std::format!("lsd: {}\n\n",
                                                        std::format!($($arg)*)).as_bytes());
                if res.is_err() {
                    std::process::exit(0);
                }
            }
        }
    };
}

/// Macro used to avoid panicking when the lsd method is used with a pipe and
/// stdout close before our program.
#[macro_export]
macro_rules! print_output {
    ($($arg:tt)*) => {
        use std::io::Write;

        let stderr = std::io::stdout();


        {
            let mut handle = stderr.lock();
            // We can write on stdout, so we simply ignore the error and don't print
            // and stop with success.
            let res = handle.write_all(std::format!($($arg)*).as_bytes());
            if res.is_err() {
                std::process::exit(0);
            }
        }
    };
}

fn main() {
    let matches = app::build().get_matches_from(wild::args_os());

    // input translate glob FILE without single quote into real names
    // for example:
    // * to all files matched
    // '*' remain as '*'
    let inputs = matches
        .values_of("FILE")
        .expect("failed to retrieve cli value")
        .map(PathBuf::from)
        .collect();

    let config = if matches.is_present("ignore-config") {
        Config::with_none()
    } else if matches.is_present("config-file") {
        let path = matches
            .value_of("config-file")
            .expect("Invalid config file path")
            .into();

        Config::from_file(path).expect("Provided file path is invalid")
    } else {
        Config::default()
    };
    let flags = Flags::configure_from(&matches, &config).unwrap_or_else(|err| err.exit());
    let core = Core::new(flags);

    core.run(inputs);
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5044() {
    rusty_monitor::set_test_id(5044);
    let mut str_0: &str = "sWLytXk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 16usize;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut bool_7: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_6: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut bool_8: bool = false;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_8: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_8, reverse: option_7, dir_grouping: option_6};
    let mut option_9: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_11: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_14: std::option::Option<bool> = std::option::Option::None;
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_17: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_18: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_18, theme: option_17, separator: option_16};
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_9: bool = true;
    let mut option_21: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_2);
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_27: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_15, indicators: option_14, layout: option_13, recursion: option_12, size: option_11, permission: option_10, sorting: option_9, no_symlink: option_5, total_size: option_4, symlink_arrow: option_3, hyperlink: option_2};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 27usize;
    let mut bool_10: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_10, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    panic!("From RustyUnit with love");
}
}