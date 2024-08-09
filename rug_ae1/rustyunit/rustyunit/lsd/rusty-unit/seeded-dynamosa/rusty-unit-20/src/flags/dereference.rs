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
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_683() {
//    rusty_monitor::set_test_id(683);
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
#[timeout(30000)]fn rusty_test_527() {
//    rusty_monitor::set_test_id(527);
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
#[timeout(30000)]fn rusty_test_856() {
//    rusty_monitor::set_test_id(856);
    let mut dereference_0: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_1: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut usize_0: usize = 11usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut dereference_1_ref_0: &crate::flags::dereference::Dereference = &mut dereference_1;
    let mut dereference_2: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_0_ref_0: &crate::flags::dereference::Dereference = &mut dereference_0;
    let mut dereference_3: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_2_ref_0: &crate::flags::dereference::Dereference = &mut dereference_2;
    let mut dereference_4: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_3_ref_0: &crate::flags::dereference::Dereference = &mut dereference_3;
    let mut dereference_5: crate::flags::dereference::Dereference = crate::flags::dereference::Dereference::default();
    let mut dereference_4_ref_0: &crate::flags::dereference::Dereference = &mut dereference_4;
    let mut bool_1: bool = crate::flags::dereference::Dereference::ne(dereference_2_ref_0, dereference_1_ref_0);
    let mut bool_2: bool = crate::flags::dereference::Dereference::ne(dereference_4_ref_0, dereference_0_ref_0);
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}
}