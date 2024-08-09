//! This module defines the [Dereference] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to dereference symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Dereference(pub bool);

impl Configurable<Self> for Dereference {
    /// Get a potential `Dereference` value from [ArgMatches].
    ///
    /// If the "dereference" argument is passed, this returns a `Dereference` with value `true` in
    /// a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("dereference") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `Dereference` value from a [Config].
    ///
    /// If the `Config::dereference` has value, this returns its value
    /// as the value of the `Dereference`, in a [Some], Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.dereference.as_ref().map(|deref| Self(*deref))
    }
}

#[cfg(test)]
mod test {
    use super::Dereference;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Dereference::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--dereference"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(Dereference(true)),
            Dereference::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Dereference::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.dereference = Some(true);
        assert_eq!(Some(Dereference(true)), Dereference::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.dereference = Some(false);
        assert_eq!(Some(Dereference(false)), Dereference::from_config(&c));
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
#[timeout(30000)]fn rusty_test_224() {
//    rusty_monitor::set_test_id(224);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_1: bool = false;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_2: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_19: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_23: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_1_ref_0);
    let mut option_24: std::option::Option<crate::flags::dereference::Dereference> = crate::flags::dereference::Dereference::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_699() {
//    rusty_monitor::set_test_id(699);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_5_ref_0: &crate::flags::dereference::Dereference = &mut dereference_5;
    let mut dereference_6: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_6_ref_0: &crate::flags::dereference::Dereference = &mut dereference_6;
    let mut dereference_7: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_7_ref_0: &crate::flags::dereference::Dereference = &mut dereference_7;
    let mut dereference_8: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_8_ref_0: &crate::flags::dereference::Dereference = &mut dereference_8;
    let mut dereference_9: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_9_ref_0: &crate::flags::dereference::Dereference = &mut dereference_9;
    let mut dereference_10: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_10_ref_0: &crate::flags::dereference::Dereference = &mut dereference_10;
    let mut dereference_11: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_11_ref_0: &crate::flags::dereference::Dereference = &mut dereference_11;
    let mut dereference_12: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_12_ref_0: &crate::flags::dereference::Dereference = &mut dereference_12;
    let mut dereference_13: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_13_ref_0: &crate::flags::dereference::Dereference = &mut dereference_13;
    let mut bool_0: bool = crate::flags::dereference::Dereference::eq(dereference_13_ref_0, dereference_12_ref_0);
    let mut bool_1: bool = crate::flags::dereference::Dereference::eq(dereference_11_ref_0, dereference_10_ref_0);
    let mut bool_2: bool = crate::flags::dereference::Dereference::eq(dereference_9_ref_0, dereference_8_ref_0);
    let mut bool_3: bool = crate::flags::dereference::Dereference::eq(dereference_7_ref_0, dereference_6_ref_0);
    let mut bool_4: bool = crate::flags::dereference::Dereference::eq(dereference_5_ref_0, dereference_4_ref_0);
    let mut bool_5: bool = crate::flags::dereference::Dereference::eq(dereference_3_ref_0, dereference_2_ref_0);
    let mut bool_6: bool = crate::flags::dereference::Dereference::eq(dereference_1_ref_0, dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_245() {
//    rusty_monitor::set_test_id(245);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_5_ref_0: &crate::flags::dereference::Dereference = &mut dereference_5;
    let mut dereference_6: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_6_ref_0: &crate::flags::dereference::Dereference = &mut dereference_6;
    let mut dereference_7: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_7_ref_0: &crate::flags::dereference::Dereference = &mut dereference_7;
    let mut dereference_8: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_8_ref_0: &crate::flags::dereference::Dereference = &mut dereference_8;
    let mut dereference_9: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_9_ref_0: &crate::flags::dereference::Dereference = &mut dereference_9;
    let mut dereference_10: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_10_ref_0: &crate::flags::dereference::Dereference = &mut dereference_10;
    let mut tuple_0: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_10_ref_0);
    let mut tuple_1: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_9_ref_0);
    let mut tuple_2: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_8_ref_0);
    let mut tuple_3: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_7_ref_0);
    let mut tuple_4: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_6_ref_0);
    let mut tuple_5: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_5_ref_0);
    let mut tuple_6: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_4_ref_0);
    let mut tuple_7: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_3_ref_0);
    let mut tuple_8: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_2_ref_0);
    let mut tuple_9: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_1_ref_0);
    let mut tuple_10: () = crate::flags::dereference::Dereference::assert_receiver_is_total_eq(dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_526() {
//    rusty_monitor::set_test_id(526);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_5_ref_0: &crate::flags::dereference::Dereference = &mut dereference_5;
    let mut dereference_6: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_6_ref_0: &crate::flags::dereference::Dereference = &mut dereference_6;
    let mut dereference_7: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_7_ref_0: &crate::flags::dereference::Dereference = &mut dereference_7;
    let mut dereference_8: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_8_ref_0: &crate::flags::dereference::Dereference = &mut dereference_8;
    let mut dereference_9: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_9_ref_0: &crate::flags::dereference::Dereference = &mut dereference_9;
    let mut dereference_10: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_10_ref_0: &crate::flags::dereference::Dereference = &mut dereference_10;
    let mut dereference_11: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_11_ref_0: &crate::flags::dereference::Dereference = &mut dereference_11;
    let mut dereference_12: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_12_ref_0: &crate::flags::dereference::Dereference = &mut dereference_12;
    let mut dereference_13: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_13_ref_0: &crate::flags::dereference::Dereference = &mut dereference_13;
    let mut bool_0: bool = crate::flags::dereference::Dereference::ne(dereference_13_ref_0, dereference_12_ref_0);
    let mut bool_1: bool = crate::flags::dereference::Dereference::ne(dereference_11_ref_0, dereference_10_ref_0);
    let mut bool_2: bool = crate::flags::dereference::Dereference::ne(dereference_9_ref_0, dereference_8_ref_0);
    let mut bool_3: bool = crate::flags::dereference::Dereference::ne(dereference_7_ref_0, dereference_6_ref_0);
    let mut bool_4: bool = crate::flags::dereference::Dereference::ne(dereference_5_ref_0, dereference_4_ref_0);
    let mut bool_5: bool = crate::flags::dereference::Dereference::ne(dereference_3_ref_0, dereference_2_ref_0);
    let mut bool_6: bool = crate::flags::dereference::Dereference::ne(dereference_1_ref_0, dereference_0_ref_0);
//    panic!("From RustyUnit with love");
}
}