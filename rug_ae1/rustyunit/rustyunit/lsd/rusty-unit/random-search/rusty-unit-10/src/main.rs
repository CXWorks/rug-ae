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
fn rusty_test_5274() {
    rusty_monitor::set_test_id(5274);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 53usize;
    let mut bool_1: bool = true;
    let mut usize_1: usize = 43usize;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut u64_0: u64 = 41u64;
    let mut usize_2: usize = 58usize;
    let mut bool_5: bool = false;
    let mut usize_3: usize = 68usize;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut usize_4: usize = 31usize;
    let mut bool_11: bool = true;
    let mut u64_1: u64 = 64u64;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut usize_5: usize = 92usize;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut usize_6: usize = 94usize;
    let mut bool_18: bool = true;
    let mut bool_19: bool = false;
    let mut usize_7: usize = 15usize;
    let mut bool_20: bool = true;
    let mut usize_8: usize = 13usize;
    let mut bool_21: bool = true;
    let mut usize_9: usize = 17usize;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut usize_10: usize = 35usize;
    let mut bool_25: bool = true;
    let mut u64_2: u64 = 11u64;
    let mut bool_26: bool = false;
    let mut usize_11: usize = 9usize;
    let mut bool_27: bool = true;
    let mut bool_28: bool = true;
    let mut bool_29: bool = false;
    let mut bool_30: bool = true;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_31: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_31);
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_3: u64 = 28u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_12: usize = 2usize;
    let mut bool_32: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_32, depth: usize_12};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_10: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_1);
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut bool_33: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_33);
    let mut option_14: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_16, reverse: option_15, dir_grouping: option_14};
    let mut option_17: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_18: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut option_19: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_34: bool = false;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_34);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_25: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_2);
    let mut option_26: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_26, theme: option_25, separator: option_24};
    let mut option_27: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_28: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_35: bool = false;
    let mut option_29: std::option::Option<bool> = std::option::Option::Some(bool_35);
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_31: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_32: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_36: bool = true;
    let mut option_33: std::option::Option<bool> = std::option::Option::Some(bool_36);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_33, blocks: option_32, color: option_31, date: option_30, dereference: option_29, display: option_28, icons: option_27, ignore_globs: option_23, indicators: option_22, layout: option_21, recursion: option_20, size: option_19, permission: option_18, sorting: option_17, no_symlink: option_13, total_size: option_12, symlink_arrow: option_11, hyperlink: option_10};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_4: u64 = 3u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_4);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    panic!("From RustyUnit with love");
}
}