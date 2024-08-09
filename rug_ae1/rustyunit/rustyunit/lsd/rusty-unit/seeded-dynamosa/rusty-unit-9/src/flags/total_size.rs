//! This module defines the [TotalSize] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to show the total size for directories.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TotalSize(pub bool);

impl Configurable<Self> for TotalSize {
    /// Get a potential `TotalSize` value from [ArgMatches].
    ///
    /// If the "total-size" argument is passed, this returns a `TotalSize` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("total-size") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `TotalSize` value from a [Config].
    ///
    /// If the `Config::total-size` has value,
    /// this returns it as the value of the `TotalSize`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.total_size.map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::TotalSize;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--total-size"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(TotalSize(true)), TotalSize::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, TotalSize::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.total_size = Some(true);
        assert_eq!(Some(TotalSize(true)), TotalSize::from_config(&c));
    }

    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.total_size = Some(false);
        assert_eq!(Some(TotalSize(false)), TotalSize::from_config(&c));
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
#[timeout(30000)]fn rusty_test_726() {
//    rusty_monitor::set_test_id(726);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut totalsize_2: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_2_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_2;
    let mut totalsize_3: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_3;
    let mut totalsize_4: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_4_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_4;
    let mut totalsize_5: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_5_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_5;
    let mut totalsize_6: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_6_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_6;
    let mut totalsize_7: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_7_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_7;
    let mut totalsize_8: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_8_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_8;
    let mut totalsize_9: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_9_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_9;
    let mut totalsize_10: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_10_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_10;
    let mut totalsize_11: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_11_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_11;
    let mut totalsize_12: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_12_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_12;
    let mut totalsize_13: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_13_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_13;
    let mut bool_0: bool = crate::flags::total_size::TotalSize::ne(totalsize_13_ref_0, totalsize_12_ref_0);
    let mut bool_1: bool = crate::flags::total_size::TotalSize::ne(totalsize_11_ref_0, totalsize_10_ref_0);
    let mut bool_2: bool = crate::flags::total_size::TotalSize::ne(totalsize_9_ref_0, totalsize_8_ref_0);
    let mut bool_3: bool = crate::flags::total_size::TotalSize::ne(totalsize_7_ref_0, totalsize_6_ref_0);
    let mut bool_4: bool = crate::flags::total_size::TotalSize::ne(totalsize_5_ref_0, totalsize_4_ref_0);
    let mut bool_5: bool = crate::flags::total_size::TotalSize::ne(totalsize_3_ref_0, totalsize_2_ref_0);
    let mut bool_6: bool = crate::flags::total_size::TotalSize::ne(totalsize_1_ref_0, totalsize_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7366() {
//    rusty_monitor::set_test_id(7366);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "mkd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut bool_1: bool = crate::flags::total_size::TotalSize::eq(totalsize_1_ref_0, totalsize_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_411() {
//    rusty_monitor::set_test_id(411);
    let mut totalsize_0: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_0_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_0;
    let mut totalsize_1: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_1_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_1;
    let mut totalsize_2: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_2_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_2;
    let mut totalsize_3: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_3_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_3;
    let mut totalsize_4: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_4_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_4;
    let mut totalsize_5: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_5_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_5;
    let mut totalsize_6: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_6_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_6;
    let mut totalsize_7: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_7_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_7;
    let mut totalsize_8: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_8_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_8;
    let mut totalsize_9: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_9_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_9;
    let mut totalsize_10: crate::flags::total_size::TotalSize = crate::flags::total_size::TotalSize::default();
    let mut totalsize_10_ref_0: &crate::flags::total_size::TotalSize = &mut totalsize_10;
    let mut tuple_0: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_10_ref_0);
    let mut tuple_1: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_9_ref_0);
    let mut tuple_2: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_8_ref_0);
    let mut tuple_3: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_7_ref_0);
    let mut tuple_4: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_6_ref_0);
    let mut tuple_5: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_5_ref_0);
    let mut tuple_6: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_4_ref_0);
    let mut tuple_7: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_3_ref_0);
    let mut tuple_8: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_2_ref_0);
    let mut tuple_9: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_1_ref_0);
    let mut tuple_10: () = crate::flags::total_size::TotalSize::assert_receiver_is_total_eq(totalsize_0_ref_0);
//    panic!("From RustyUnit with love");
}
}