//! This module defines the [NoSymlink] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to follow symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct NoSymlink(pub bool);

impl Configurable<Self> for NoSymlink {
    /// Get a potential `NoSymlink` value from [ArgMatches].
    ///
    /// If the "no-symlink" argument is passed, this returns a `NoSymlink` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("no-symlink") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `NoSymlink` value from a [Config].
    ///
    /// If the `Config::no-symlink` has value,
    /// this returns it as the value of the `NoSymlink`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.no_symlink.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::NoSymlink;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--no-symlink"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.no_symlink = Some(true);
        assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.no_symlink = Some(false);
        assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_765() {
//    rusty_monitor::set_test_id(765);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut tuple_0: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_10_ref_0);
    let mut tuple_1: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_9_ref_0);
    let mut tuple_2: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_8_ref_0);
    let mut tuple_3: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_7_ref_0);
    let mut tuple_4: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_6_ref_0);
    let mut tuple_5: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_5_ref_0);
    let mut tuple_6: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_4_ref_0);
    let mut tuple_7: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_3_ref_0);
    let mut tuple_8: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_2_ref_0);
    let mut tuple_9: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_1_ref_0);
    let mut tuple_10: () = crate::flags::symlinks::NoSymlink::assert_receiver_is_total_eq(nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_206() {
//    rusty_monitor::set_test_id(206);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut nosymlink_11: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_11_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_11;
    let mut nosymlink_12: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_12_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_12;
    let mut nosymlink_13: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_13_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_13;
    let mut bool_0: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_13_ref_0, nosymlink_12_ref_0);
    let mut bool_1: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_11_ref_0, nosymlink_10_ref_0);
    let mut bool_2: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_9_ref_0, nosymlink_8_ref_0);
    let mut bool_3: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_7_ref_0, nosymlink_6_ref_0);
    let mut bool_4: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_5_ref_0, nosymlink_4_ref_0);
    let mut bool_5: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_3_ref_0, nosymlink_2_ref_0);
    let mut bool_6: bool = crate::flags::symlinks::NoSymlink::eq(nosymlink_1_ref_0, nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5656() {
//    rusty_monitor::set_test_id(5656);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut option_2: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_0);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_2, reverse: option_1, dir_grouping: option_0};
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut elem_0: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_1: color::Elem = crate::color::Elem::Context;
    let mut bool_12: bool = crate::meta::permissions::Permissions::is_executable(permissions_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_458() {
//    rusty_monitor::set_test_id(458);
    let mut nosymlink_0: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_0_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_0;
    let mut nosymlink_1: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_1_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_1;
    let mut nosymlink_2: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_2_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_2;
    let mut nosymlink_3: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_3_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_3;
    let mut nosymlink_4: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_4_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_4;
    let mut nosymlink_5: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_5_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_5;
    let mut nosymlink_6: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_6_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_6;
    let mut nosymlink_7: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_7_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_7;
    let mut nosymlink_8: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_8_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_8;
    let mut nosymlink_9: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_9_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_9;
    let mut nosymlink_10: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_10_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_10;
    let mut nosymlink_11: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_11_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_11;
    let mut nosymlink_12: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_12_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_12;
    let mut nosymlink_13: crate::flags::symlinks::NoSymlink = crate::flags::symlinks::NoSymlink::default();
    let mut nosymlink_13_ref_0: &crate::flags::symlinks::NoSymlink = &mut nosymlink_13;
    let mut bool_0: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_13_ref_0, nosymlink_12_ref_0);
    let mut bool_1: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_11_ref_0, nosymlink_10_ref_0);
    let mut bool_2: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_9_ref_0, nosymlink_8_ref_0);
    let mut bool_3: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_7_ref_0, nosymlink_6_ref_0);
    let mut bool_4: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_5_ref_0, nosymlink_4_ref_0);
    let mut bool_5: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_3_ref_0, nosymlink_2_ref_0);
    let mut bool_6: bool = crate::flags::symlinks::NoSymlink::ne(nosymlink_1_ref_0, nosymlink_0_ref_0);
//    panic!("From RustyUnit with love");
}
}