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
fn rusty_test_5047() {
    rusty_monitor::set_test_id(5047);
    let mut str_0: &str = "udoYl";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 86usize;
    let mut str_1: &str = "1X";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 38usize;
    let mut tuple_0: (usize, &str) = (usize_1, str_1_ref_0);
    let mut usize_2: usize = 31usize;
    let mut bool_0: bool = false;
    let mut u64_0: u64 = 26u64;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut bool_13: bool = true;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut usize_3: usize = 48usize;
    let mut bool_16: bool = false;
    let mut str_2: &str = "6cKTjnLXe3Gh";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_17: bool = false;
    let mut bool_18: bool = false;
    let mut bool_19: bool = true;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = true;
    let mut bool_24: bool = true;
    let mut bool_25: bool = true;
    let mut bool_26: bool = true;
    let mut bool_27: bool = false;
    let mut bool_28: bool = false;
    let mut str_3: &str = "mMpPA1qT87U0";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_29: bool = false;
    let mut u64_1: u64 = 8u64;
    let mut usize_4: usize = 45usize;
    let mut bool_30: bool = false;
    let mut usize_5: usize = 24usize;
    let mut bool_31: bool = false;
    let mut u64_2: u64 = 62u64;
    let mut bool_32: bool = false;
    let mut bool_33: bool = true;
    let mut bool_34: bool = false;
    let mut bool_35: bool = false;
    let mut bool_36: bool = false;
    let mut bool_37: bool = false;
    let mut bool_38: bool = true;
    let mut bool_39: bool = false;
    let mut bool_40: bool = true;
    let mut bool_41: bool = true;
    let mut bool_42: bool = true;
    let mut bool_43: bool = true;
    let mut bool_44: bool = true;
    let mut bool_45: bool = false;
    let mut bool_46: bool = false;
    let mut bool_47: bool = false;
    let mut bool_48: bool = true;
    let mut bool_49: bool = true;
    let mut bool_50: bool = true;
    let mut bool_51: bool = false;
    let mut bool_52: bool = false;
    let mut bool_53: bool = false;
    let mut bool_54: bool = false;
    let mut bool_55: bool = false;
    let mut str_4: &str = "gEd9SkhzhUZ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Pz1oFRS1C";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_6: usize = 41usize;
    let mut tuple_1: (usize, &str) = (usize_6, str_5_ref_0);
    let mut usize_7: usize = 50usize;
    let mut bool_56: bool = true;
    let mut usize_8: usize = 34usize;
    let mut bool_57: bool = false;
    let mut u64_3: u64 = 40u64;
    let mut bool_58: bool = true;
    let mut bool_59: bool = true;
    let mut bool_60: bool = false;
    let mut bool_61: bool = true;
    let mut bool_62: bool = false;
    let mut bool_63: bool = false;
    let mut bool_64: bool = false;
    let mut bool_65: bool = true;
    let mut bool_66: bool = false;
    let mut bool_67: bool = true;
    let mut bool_68: bool = true;
    let mut bool_69: bool = true;
    let mut u64_4: u64 = 96u64;
    let mut usize_9: usize = 35usize;
    let mut bool_70: bool = false;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_10: usize = 27usize;
    let mut bool_71: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_71, depth: usize_10};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_11: usize = 3usize;
    let mut bool_72: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_72, depth: usize_11};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut u64_5: u64 = 81u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_5);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_12: usize = 94usize;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_13: usize = 83usize;
    let mut bool_73: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_73, depth: usize_13};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_74: bool = false;
    let mut bool_75: bool = false;
    let mut bool_76: bool = true;
    let mut bool_77: bool = true;
    let mut bool_78: bool = false;
    let mut bool_79: bool = true;
    let mut bool_80: bool = true;
    let mut bool_81: bool = true;
    let mut bool_82: bool = false;
    let mut bool_83: bool = false;
    let mut bool_84: bool = true;
    let mut bool_85: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_85, user_write: bool_84, user_execute: bool_83, group_read: bool_82, group_write: bool_81, group_execute: bool_80, other_read: bool_79, other_write: bool_78, other_execute: bool_77, sticky: bool_76, setgid: bool_75, setuid: bool_74};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    panic!("From RustyUnit with love");
}
}