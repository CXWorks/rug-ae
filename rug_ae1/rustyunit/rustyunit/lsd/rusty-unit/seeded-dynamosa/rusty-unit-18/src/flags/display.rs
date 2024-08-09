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
#[timeout(30000)]fn rusty_test_453() {
//    rusty_monitor::set_test_id(453);
    let mut display_0: flags::display::Display = crate::flags::display::Display::default();
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::default();
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::default();
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::All;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::default();
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::default();
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::default();
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
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
#[timeout(30000)]fn rusty_test_515() {
//    rusty_monitor::set_test_id(515);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::All;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::All;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::All;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::All;
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
#[timeout(30000)]fn rusty_test_288() {
//    rusty_monitor::set_test_id(288);
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_0_ref_0: &flags::display::Display = &mut display_0;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_1_ref_0: &flags::display::Display = &mut display_1;
    let mut display_2: flags::display::Display = crate::flags::display::Display::default();
    let mut display_2_ref_0: &flags::display::Display = &mut display_2;
    let mut display_3: flags::display::Display = crate::flags::display::Display::default();
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut display_4: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    let mut display_5: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_5_ref_0: &flags::display::Display = &mut display_5;
    let mut display_6: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_6_ref_0: &flags::display::Display = &mut display_6;
    let mut display_7: flags::display::Display = crate::flags::display::Display::All;
    let mut display_7_ref_0: &flags::display::Display = &mut display_7;
    let mut display_8: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_8_ref_0: &flags::display::Display = &mut display_8;
    let mut display_9: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut display_9_ref_0: &flags::display::Display = &mut display_9;
    let mut display_10: flags::display::Display = crate::flags::display::Display::All;
    let mut display_10_ref_0: &flags::display::Display = &mut display_10;
    let mut display_11: flags::display::Display = crate::flags::display::Display::default();
    let mut display_11_ref_0: &flags::display::Display = &mut display_11;
    let mut display_12: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
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
}