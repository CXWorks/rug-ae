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
#[timeout(30000)]fn rusty_test_3050() {
//    rusty_monitor::set_test_id(3050);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Octal;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::ExecSticky;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut bool_12: bool = true;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_13: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_13);
    let mut bool_14: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_12);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_5: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_5);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_14);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_20: std::option::Option<flags::hyperlink::HyperlinkOption> = crate::flags::hyperlink::HyperlinkOption::from_config(config_1_ref_0);
    let mut option_21: std::option::Option<flags::hyperlink::HyperlinkOption> = crate::flags::hyperlink::HyperlinkOption::from_config(config_0_ref_0);
    let mut size_0: crate::color::theme::Size = crate::color::theme::Size {none: color_4, small: color_3, medium: color_2, large: color_1};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_264() {
//    rusty_monitor::set_test_id(264);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_2_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_2;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_3_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_3;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_4_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_4;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_5_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_5;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_6_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_6;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_7_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_7;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
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
#[timeout(30000)]fn rusty_test_393() {
//    rusty_monitor::set_test_id(393);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_1_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_1;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_2_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_2;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_3_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_3;
    let mut hyperlinkoption_4: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_4_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_4;
    let mut hyperlinkoption_5: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_5_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_5;
    let mut hyperlinkoption_6: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut hyperlinkoption_6_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_6;
    let mut hyperlinkoption_7: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_7_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_7;
    let mut hyperlinkoption_8: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_8_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_8;
    let mut hyperlinkoption_9: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut hyperlinkoption_9_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_9;
    let mut hyperlinkoption_10: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut hyperlinkoption_10_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_10;
    let mut hyperlinkoption_11: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::default();
    let mut hyperlinkoption_11_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_11;
    let mut hyperlinkoption_12: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1332() {
//    rusty_monitor::set_test_id(1332);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_10: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut option_14: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_14, theme: option_13};
    let mut option_15: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_17: std::option::Option<flags::hyperlink::HyperlinkOption> = crate::flags::hyperlink::HyperlinkOption::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}
}