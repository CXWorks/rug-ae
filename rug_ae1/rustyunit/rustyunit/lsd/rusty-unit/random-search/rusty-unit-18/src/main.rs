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
fn rusty_test_5250() {
    rusty_monitor::set_test_id(5250);
    let mut bool_0: bool = true;
    let mut u64_0: u64 = 86u64;
    let mut bool_1: bool = true;
    let mut str_0: &str = "3mXtC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = true;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut usize_0: usize = 99usize;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut bool_13: bool = true;
    let mut bool_14: bool = true;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut u64_1: u64 = 32u64;
    let mut bool_21: bool = true;
    let mut u64_2: u64 = 55u64;
    let mut usize_1: usize = 35usize;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = false;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = true;
    let mut bool_35: bool = true;
    let mut bool_36: bool = true;
    let mut bool_37: bool = false;
    let mut bool_38: bool = false;
    let mut bool_39: bool = true;
    let mut bool_40: bool = false;
    let mut bool_41: bool = true;
    let mut bool_42: bool = false;
    let mut bool_43: bool = true;
    let mut bool_44: bool = false;
    let mut bool_45: bool = false;
    let mut bool_46: bool = false;
    let mut bool_47: bool = false;
    let mut bool_48: bool = true;
    let mut usize_2: usize = 31usize;
    let mut bool_49: bool = true;
    let mut bool_50: bool = true;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut bool_51: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_51);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_1: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_3: usize = 87usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_3);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_5, depth: option_4};
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_52: bool = true;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_52);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_15: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_15, theme: option_14};
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_53: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_53);
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_0};
    let mut option_19: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_23: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_1);
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut option_25: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_1);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_25, reverse: option_24, dir_grouping: option_23};
    let mut option_26: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_27: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_1);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_28: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_4: usize = 54usize;
    let mut option_29: std::option::Option<usize> = std::option::Option::Some(usize_4);
    let mut bool_54: bool = false;
    let mut option_30: std::option::Option<bool> = std::option::Option::Some(bool_54);
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_30, depth: option_29};
    let mut option_31: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut option_32: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_55: bool = true;
    let mut option_33: std::option::Option<bool> = std::option::Option::Some(bool_55);
    let mut option_34: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_35: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_36: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_37: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_37, theme: option_36, separator: option_35};
    let mut option_38: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_39: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_56: bool = false;
    let mut option_40: std::option::Option<bool> = std::option::Option::Some(bool_56);
    let mut option_41: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_42: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_43: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_44: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_44, blocks: option_43, color: option_42, date: option_41, dereference: option_40, display: option_39, icons: option_38, ignore_globs: option_34, indicators: option_33, layout: option_32, recursion: option_31, size: option_28, permission: option_27, sorting: option_26, no_symlink: option_22, total_size: option_21, symlink_arrow: option_20, hyperlink: option_19};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_3: u64 = 12u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_3);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_0: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    panic!("From RustyUnit with love");
}
}