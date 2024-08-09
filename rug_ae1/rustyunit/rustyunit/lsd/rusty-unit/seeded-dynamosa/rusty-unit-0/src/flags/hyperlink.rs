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
#[timeout(30000)]fn rusty_test_4126() {
//    rusty_monitor::set_test_id(4126);
    let mut option_0: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_1: std::option::Option<bool> = std::option::Option::None;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut str_0: &str = "Sort by file extension";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut option_2: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_7: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_8: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_1: crate::config_file::Sorting = crate::config_file::Sorting {column: option_8, reverse: option_7, dir_grouping: option_6};
    let mut option_9: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_10: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_11: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_12: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_22: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2usize;
    let mut bool_5: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_23: std::option::Option<flags::hyperlink::HyperlinkOption> = crate::flags::hyperlink::HyperlinkOption::from_config(config_1_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::FileSmall;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Special;
    let mut elem_3: color::Elem = crate::color::Elem::File {exec: bool_7, uid: bool_6};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_726() {
//    rusty_monitor::set_test_id(726);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_2_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_2;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_3_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_3;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_4_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_4;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_5_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_5;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_6_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_6;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_7_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_7;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_8_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_8;
    let mut hyperlinkoption_9: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_9_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_9;
    let mut hyperlinkoption_10: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_10_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_10;
    let mut hyperlinkoption_11: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_11_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_11;
    let mut hyperlinkoption_12: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_12_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_12;
    let mut hyperlinkoption_13: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2893() {
//    rusty_monitor::set_test_id(2893);
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "c++";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut option_2: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut option_4: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_1: &str = "gz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_5: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut option_6: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_2: &str = "xq75FwUXeIf";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_7: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_2_ref_0);
    let mut bool_3: bool = true;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut tuple_0: () = crate::flags::hyperlink::HyperlinkOption::assert_receiver_is_total_eq(hyperlinkoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4486() {
//    rusty_monitor::set_test_id(4486);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Date;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut bool_0: bool = crate::flags::hyperlink::HyperlinkOption::eq(hyperlinkoption_1_ref_0, hyperlinkoption_0_ref_0);
//    panic!("From RustyUnit with love");
}
}