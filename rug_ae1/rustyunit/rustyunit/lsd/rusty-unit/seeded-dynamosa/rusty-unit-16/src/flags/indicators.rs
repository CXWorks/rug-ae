//! This module defines the [Indicators] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to print file type indicators.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Indicators(pub bool);

impl Configurable<Self> for Indicators {
    /// Get a potential `Indicators` value from [ArgMatches].
    ///
    /// If the "indicators" argument is passed, this returns an `Indicators` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("indicators") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Indicators` value from a [Config].
    ///
    /// If the `Config::indicators` has value,
    /// this returns its value as the value of the `Indicators`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.indicators.as_ref().map(|ind| Self(*ind))
    }
}

#[cfg(test)]
mod test {
    use super::Indicators;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Indicators::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--classify"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Indicators(true)),
            Indicators::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Indicators::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.indicators = Some(true);
        assert_eq!(Some(Indicators(true)), Indicators::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.indicators = Some(false);
        assert_eq!(Some(Indicators(false)), Indicators::from_config(&c));
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
#[timeout(30000)]fn rusty_test_829() {
//    rusty_monitor::set_test_id(829);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 1usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut bool_1: bool = crate::flags::indicators::Indicators::ne(indicators_1_ref_0, indicators_3_ref_0);
    let mut bool_2: bool = crate::flags::indicators::Indicators::ne(indicators_6_ref_0, indicators_4_ref_0);
    let mut bool_3: bool = crate::flags::indicators::Indicators::ne(indicators_5_ref_0, indicators_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_586() {
//    rusty_monitor::set_test_id(586);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_7_ref_0: &crate::flags::indicators::Indicators = &mut indicators_7;
    let mut indicators_8: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_8_ref_0: &crate::flags::indicators::Indicators = &mut indicators_8;
    let mut indicators_9: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_9_ref_0: &crate::flags::indicators::Indicators = &mut indicators_9;
    let mut indicators_10: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_10_ref_0: &crate::flags::indicators::Indicators = &mut indicators_10;
    let mut tuple_0: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_10_ref_0);
    let mut tuple_1: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_9_ref_0);
    let mut tuple_2: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_8_ref_0);
    let mut tuple_3: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_7_ref_0);
    let mut tuple_4: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_6_ref_0);
    let mut tuple_5: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_5_ref_0);
    let mut tuple_6: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_4_ref_0);
    let mut tuple_7: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_3_ref_0);
    let mut tuple_8: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_2_ref_0);
    let mut tuple_9: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_1_ref_0);
    let mut tuple_10: () = crate::flags::indicators::Indicators::assert_receiver_is_total_eq(indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_666() {
//    rusty_monitor::set_test_id(666);
    let mut indicators_0: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_0_ref_0: &crate::flags::indicators::Indicators = &mut indicators_0;
    let mut indicators_1: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_1_ref_0: &crate::flags::indicators::Indicators = &mut indicators_1;
    let mut indicators_2: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_2_ref_0: &crate::flags::indicators::Indicators = &mut indicators_2;
    let mut indicators_3: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_3_ref_0: &crate::flags::indicators::Indicators = &mut indicators_3;
    let mut indicators_4: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_4_ref_0: &crate::flags::indicators::Indicators = &mut indicators_4;
    let mut indicators_5: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_5_ref_0: &crate::flags::indicators::Indicators = &mut indicators_5;
    let mut indicators_6: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_6_ref_0: &crate::flags::indicators::Indicators = &mut indicators_6;
    let mut indicators_7: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_7_ref_0: &crate::flags::indicators::Indicators = &mut indicators_7;
    let mut indicators_8: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_8_ref_0: &crate::flags::indicators::Indicators = &mut indicators_8;
    let mut indicators_9: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_9_ref_0: &crate::flags::indicators::Indicators = &mut indicators_9;
    let mut indicators_10: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_10_ref_0: &crate::flags::indicators::Indicators = &mut indicators_10;
    let mut indicators_11: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_11_ref_0: &crate::flags::indicators::Indicators = &mut indicators_11;
    let mut indicators_12: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_12_ref_0: &crate::flags::indicators::Indicators = &mut indicators_12;
    let mut indicators_13: crate::flags::indicators::Indicators = crate::flags::indicators::Indicators::default();
    let mut indicators_13_ref_0: &crate::flags::indicators::Indicators = &mut indicators_13;
    let mut bool_0: bool = crate::flags::indicators::Indicators::eq(indicators_13_ref_0, indicators_12_ref_0);
    let mut bool_1: bool = crate::flags::indicators::Indicators::eq(indicators_11_ref_0, indicators_10_ref_0);
    let mut bool_2: bool = crate::flags::indicators::Indicators::eq(indicators_9_ref_0, indicators_8_ref_0);
    let mut bool_3: bool = crate::flags::indicators::Indicators::eq(indicators_7_ref_0, indicators_6_ref_0);
    let mut bool_4: bool = crate::flags::indicators::Indicators::eq(indicators_5_ref_0, indicators_4_ref_0);
    let mut bool_5: bool = crate::flags::indicators::Indicators::eq(indicators_3_ref_0, indicators_2_ref_0);
    let mut bool_6: bool = crate::flags::indicators::Indicators::eq(indicators_1_ref_0, indicators_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_574() {
//    rusty_monitor::set_test_id(574);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut config_5: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_5_ref_0: &crate::config_file::Config = &mut config_5;
    let mut config_6: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_6_ref_0: &crate::config_file::Config = &mut config_6;
    let mut config_7: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_7_ref_0: &crate::config_file::Config = &mut config_7;
    let mut config_8: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_8_ref_0: &crate::config_file::Config = &mut config_8;
    let mut config_9: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_9_ref_0: &crate::config_file::Config = &mut config_9;
    let mut config_10: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_10_ref_0: &crate::config_file::Config = &mut config_10;
    let mut option_0: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_10_ref_0);
    let mut option_1: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_9_ref_0);
    let mut option_2: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_8_ref_0);
    let mut option_3: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_7_ref_0);
    let mut option_4: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_6_ref_0);
    let mut option_5: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_5_ref_0);
    let mut option_6: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_4_ref_0);
    let mut option_7: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_3_ref_0);
    let mut option_8: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_2_ref_0);
    let mut option_9: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_1_ref_0);
    let mut option_10: std::option::Option<crate::flags::indicators::Indicators> = crate::flags::indicators::Indicators::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}