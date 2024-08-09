//! This module defines the [Display] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing which file system nodes to display.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Display {
    All,
    AlmostAll,
    DirectoryOnly,
    VisibleOnly,
}

impl Configurable<Self> for Display {
    /// Get a potential `Display` variant from [ArgMatches].
    ///
    /// If any of the "all", "almost-all" or "directory-only" arguments is passed, this returns the
    /// corresponding `Display` variant in a [Some]. If neither of them is passed, this returns
    /// [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("directory-only") {
            Some(Self::DirectoryOnly)
        } else if matches.is_present("almost-all") {
            Some(Self::AlmostAll)
        } else if matches.is_present("all") {
            Some(Self::All)
        } else {
            None
        }
    }

    /// Get a potential `Display` variant from a [Config].
    ///
    /// If the `Config::display` has value and is one of
    /// "all", "almost-all", "directory-only" or `visible-only`,
    /// this returns the corresponding `Display` variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.display
    }
}

/// The default value for `Display` is [Display::VisibleOnly].
impl Default for Display {
    fn default() -> Self {
        Display::VisibleOnly
    }
}

#[cfg(test)]
mod test {
    use super::Display;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_all() {
        let argv = vec!["lsd", "--all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Display::All), Display::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_almost_all() {
        let argv = vec!["lsd", "--almost-all"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::AlmostAll),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_directory_only() {
        let argv = vec!["lsd", "--directory-only"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Display::DirectoryOnly),
            Display::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Display::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::All);
        assert_eq!(Some(Display::All), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_almost_all() {
        let mut c = Config::with_none();
        c.display = Some(Display::AlmostAll);
        assert_eq!(Some(Display::AlmostAll), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_directory_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::DirectoryOnly);
        assert_eq!(Some(Display::DirectoryOnly), Display::from_config(&c));
    }

    #[test]
    fn test_from_config_visible_only() {
        let mut c = Config::with_none();
        c.display = Some(Display::VisibleOnly);
        assert_eq!(Some(Display::VisibleOnly), Display::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_716() {
//    rusty_monitor::set_test_id(716);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::default();
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::default();
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::clone(display_10_ref_0);
    let mut display_12: flags::display::Display = crate::flags::display::Display::clone(display_9_ref_0);
    let mut display_13: flags::display::Display = crate::flags::display::Display::clone(display_8_ref_0);
    let mut display_14: flags::display::Display = crate::flags::display::Display::clone(display_7_ref_0);
    let mut display_15: flags::display::Display = crate::flags::display::Display::clone(display_6_ref_0);
    let mut display_16: flags::display::Display = crate::flags::display::Display::clone(display_5_ref_0);
    let mut display_17: flags::display::Display = crate::flags::display::Display::clone(display_4_ref_0);
    let mut display_18: flags::display::Display = crate::flags::display::Display::clone(display_3_ref_0);
    let mut display_19: flags::display::Display = crate::flags::display::Display::clone(display_2_ref_0);
    let mut display_20: flags::display::Display = crate::flags::display::Display::clone(display_1_ref_0);
    let mut display_21: flags::display::Display = crate::flags::display::Display::clone(display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_283() {
//    rusty_monitor::set_test_id(283);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::All;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut tuple_0: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_10_ref_0);
    let mut tuple_1: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_9_ref_0);
    let mut tuple_2: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_8_ref_0);
    let mut tuple_3: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_7_ref_0);
    let mut tuple_4: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_6_ref_0);
    let mut tuple_5: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_5_ref_0);
    let mut tuple_6: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_4_ref_0);
    let mut tuple_7: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_3_ref_0);
    let mut tuple_8: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_2_ref_0);
    let mut tuple_9: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_1_ref_0);
    let mut tuple_10: () = crate::flags::display::Display::assert_receiver_is_total_eq(display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_73() {
//    rusty_monitor::set_test_id(73);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_2: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_3: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut bool_0: bool = crate::flags::display::Display::eq(display_1_ref_0, display_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::BlockDevice;
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Kilo;
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    let mut elem_4: color::Elem = crate::color::Elem::Octal;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_5, reverse: option_4, dir_grouping: option_3};
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_2, theme: option_1, separator: option_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_504() {
//    rusty_monitor::set_test_id(504);
    let mut display_0: flags::display::Display = crate::flags::display::Display::default();
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::All;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::default();
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::default();
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_11_ref_0: &flags::display::Display = &mut display_11;
    let mut display_12: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_12_ref_0: &flags::display::Display = &mut display_12;
    let mut display_13: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_13_ref_0: &flags::display::Display = &mut display_13;
    let mut bool_0: bool = crate::flags::display::Display::eq(display_13_ref_0, display_12_ref_0);
    let mut bool_1: bool = crate::flags::display::Display::eq(display_11_ref_0, display_10_ref_0);
    let mut bool_2: bool = crate::flags::display::Display::eq(display_9_ref_0, display_8_ref_0);
    let mut bool_3: bool = crate::flags::display::Display::eq(display_7_ref_0, display_6_ref_0);
    let mut bool_4: bool = crate::flags::display::Display::eq(display_5_ref_0, display_4_ref_0);
    let mut bool_5: bool = crate::flags::display::Display::eq(display_3_ref_0, display_2_ref_0);
    let mut bool_6: bool = crate::flags::display::Display::eq(display_1_ref_0, display_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_294() {
//    rusty_monitor::set_test_id(294);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = false;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut bool_13: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_13};
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_14: bool = true;
    let mut bool_15: bool = false;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_15, exec: bool_14};
    let mut bool_16: bool = crate::meta::filetype::FileType::is_dirlike(filetype_5);
    let mut bool_17: bool = crate::meta::filetype::FileType::is_dirlike(filetype_4);
    let mut bool_18: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    let mut bool_19: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut bool_20: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut bool_21: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}
}