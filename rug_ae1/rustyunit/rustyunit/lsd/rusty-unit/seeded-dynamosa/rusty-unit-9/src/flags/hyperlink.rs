//! This module defines the [HyperlinkOption]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// The flag showing when to use hyperlink in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HyperlinkOption {
    Always,
    Auto,
    Never,
}

impl Configurable<Self> for HyperlinkOption {
    /// Get a potential `HyperlinkOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [HyperlinkOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("hyperlink") > 0 {
            match matches.values_of("hyperlink")?.last() {
                Some("always") => Some(Self::Always),
                Some("auto") => Some(Self::Auto),
                Some("never") => Some(Self::Never),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `HyperlinkOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(HyperlinkOption::Never).
    /// Otherwise if the `Config::hyperlink::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = &config.classic {
            return Some(Self::Never);
        }

        config.hyperlink
    }
}

/// The default value for the `HyperlinkOption` is [HyperlinkOption::Auto].
impl Default for HyperlinkOption {
    fn default() -> Self {
        Self::Never
    }
}

#[cfg(test)]
mod test_hyperlink_option {
    use super::HyperlinkOption;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, HyperlinkOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--hyperlink", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Always),
            HyperlinkOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_autp() {
        let argv = vec!["lsd", "--hyperlink", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Auto),
            HyperlinkOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--hyperlink", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--hyperlink", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_hyperlink_when_multi() {
        let argv = vec!["lsd", "--hyperlink", "always", "--hyperlink", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, HyperlinkOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Always);
        assert_eq!(
            Some(HyperlinkOption::Always),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Auto);
        assert_eq!(
            Some(HyperlinkOption::Auto),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.hyperlink = Some(HyperlinkOption::Never);
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.hyperlink = Some(HyperlinkOption::Always);
        assert_eq!(
            Some(HyperlinkOption::Never),
            HyperlinkOption::from_config(&c)
        );
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
#[timeout(30000)]fn rusty_test_420() {
//    rusty_monitor::set_test_id(420);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_2_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_2;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_3_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_3;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_4_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_4;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_5_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_5;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_6_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_6;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_7_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_7;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_8_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_8;
    let mut hyperlinkoption_9: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_9_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_9;
    let mut hyperlinkoption_10: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_10_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_10;
    let mut tuple_0: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_10_ref_0);
    let mut tuple_1: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_9_ref_0);
    let mut tuple_2: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_8_ref_0);
    let mut tuple_3: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_7_ref_0);
    let mut tuple_4: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_6_ref_0);
    let mut tuple_5: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_5_ref_0);
    let mut tuple_6: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_4_ref_0);
    let mut tuple_7: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_3_ref_0);
    let mut tuple_8: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_2_ref_0);
    let mut tuple_9: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_1_ref_0);
    let mut tuple_10: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8926() {
//    rusty_monitor::set_test_id(8926);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 6usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut bool_2: bool = true;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = false;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut bool_4: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut option_3: std::option::Option<flags::sorting::SortColumn> = std::option::Option::Some(sortcolumn_2);
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_2);
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<usize> = std::option::Option::None;
    let mut bool_5: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_7, depth: option_6};
    let mut option_8: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_6: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_15: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_17, theme: option_16};
    let mut option_18: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_19: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_20: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_21: std::option::Option<flags::hyperlink::HyperlinkOption> = crate::flags::hyperlink::HyperlinkOption::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_405() {
//    rusty_monitor::set_test_id(405);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_2_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_2;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_3_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_3;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_4_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_4;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_5_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_5;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_6_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_6;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_7_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_7;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_8_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_8;
    let mut hyperlinkoption_9: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_9_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_9;
    let mut hyperlinkoption_10: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_10_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_10;
    let mut hyperlinkoption_11: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_11_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_11;
    let mut hyperlinkoption_12: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_12_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_12;
    let mut hyperlinkoption_13: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_13_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_13;
    let mut bool_0: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_13_ref_0, hyperlinkoption_12_ref_0);
    let mut bool_1: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_11_ref_0, hyperlinkoption_10_ref_0);
    let mut bool_2: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_9_ref_0, hyperlinkoption_8_ref_0);
    let mut bool_3: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_7_ref_0, hyperlinkoption_6_ref_0);
    let mut bool_4: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_5_ref_0, hyperlinkoption_4_ref_0);
    let mut bool_5: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_3_ref_0, hyperlinkoption_2_ref_0);
    let mut bool_6: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_1_ref_0, hyperlinkoption_0_ref_0);
//    panic!("From RustyUnit with love");
}
}