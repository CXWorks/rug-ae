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
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_897() {
//    rusty_monitor::set_test_id(897);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut filetype_1_ref_0: &meta::filetype::FileType = &mut filetype_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut filetype_2_ref_0: &meta::filetype::FileType = &mut filetype_2;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_4: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_5: flags::display::Display = crate::flags::display::Display::All;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_6: flags::display::Display = crate::flags::display::Display::All;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_7: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_8: flags::display::Display = crate::flags::display::Display::default();
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_10: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_11: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut bool_2: bool = crate::flags::display::Display::eq(display_5_ref_0, display_3_ref_0);
    let mut bool_3: bool = crate::flags::display::Display::eq(display_10_ref_0, display_1_ref_0);
    let mut bool_4: bool = crate::flags::display::Display::eq(display_2_ref_0, display_6_ref_0);
    let mut bool_5: bool = crate::flags::display::Display::eq(display_4_ref_0, display_9_ref_0);
    let mut bool_6: bool = crate::flags::display::Display::eq(display_8_ref_0, display_7_ref_0);
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_1, depth: option_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_606() {
//    rusty_monitor::set_test_id(606);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::All;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::All;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::All;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::default();
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::All;
    let mut display_11_ref_0: &flags::display::Display = &mut display_11;
    let mut display_12: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_12_ref_0: &flags::display::Display = &mut display_12;
    let mut display_13: flags::display::Display = crate::flags::display::Display::VisibleOnly;
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
#[timeout(30000)]fn rusty_test_736() {
//    rusty_monitor::set_test_id(736);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_20, blocks: option_19, color: option_18, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_21: std::option::Option<flags::display::Display> = crate::flags::display::Display::from_config(config_1_ref_0);
    let mut option_22: std::option::Option<flags::display::Display> = crate::flags::display::Display::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_265() {
//    rusty_monitor::set_test_id(265);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::default();
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::All;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::All;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
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
#[timeout(30000)]fn rusty_test_327() {
//    rusty_monitor::set_test_id(327);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut bool_13: bool = false;
    let mut bool_14: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_14, exec: bool_13};
    let mut bool_15: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_15};
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut filetype_5: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut filetype_6: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_16: bool = crate::meta::filetype::FileType::is_dirlike(filetype_6);
    let mut bool_17: bool = crate::meta::filetype::FileType::is_dirlike(filetype_5);
    let mut bool_18: bool = crate::meta::filetype::FileType::is_dirlike(filetype_4);
    let mut bool_19: bool = crate::meta::filetype::FileType::is_dirlike(filetype_3);
    let mut bool_20: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut bool_21: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut bool_22: bool = crate::meta::filetype::FileType::is_dirlike(filetype_0);
//    panic!("From RustyUnit with love");
}
}