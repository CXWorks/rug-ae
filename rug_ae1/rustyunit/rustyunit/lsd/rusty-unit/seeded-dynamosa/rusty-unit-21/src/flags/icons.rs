//! This module defines the [IconOption]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use serde::Deserialize;

/// A collection of flags on how to use icons.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Icons {
    /// When to use icons.
    pub when: IconOption,
    /// Which icon theme to use.
    pub theme: IconTheme,
    /// String between icon and name.
    pub separator: IconSeparator,
}

impl Icons {
    /// Get an `Icons` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [IconOption] and [IconTheme] are configured with their respective [Configurable]
    /// implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let when = IconOption::configure_from(matches, config);
        let theme = IconTheme::configure_from(matches, config);
        let separator = IconSeparator::configure_from(matches, config);
        Self {
            when,
            theme,
            separator,
        }
    }
}

/// The flag showing when to use icons in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconOption {
    Always,
    Auto,
    Never,
}

impl Configurable<Self> for IconOption {
    /// Get a potential `IconOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [IconOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("icon") > 0 {
            match matches.values_of("icon")?.last() {
                Some("always") => Some(Self::Always),
                Some("auto") => Some(Self::Auto),
                Some("never") => Some(Self::Never),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `IconOption` variant from a [Config].
    ///
    /// If the `Configs::classic` has value and is "true" then this returns Some(IconOption::Never).
    /// Otherwise if the `Config::icon::when` has value and is one of "always", "auto" or "never",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = &config.classic {
            return Some(Self::Never);
        }

        if let Some(icon) = &config.icons {
            icon.when
        } else {
            None
        }
    }
}

/// The default value for the `IconOption` is [IconOption::Auto].
impl Default for IconOption {
    fn default() -> Self {
        Self::Auto
    }
}

/// The flag showing which icon theme to use.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconTheme {
    Unicode,
    Fancy,
}

impl Configurable<Self> for IconTheme {
    /// Get a potential `IconTheme` variant from [ArgMatches].
    ///
    /// If the argument is passed, this returns the variant corresponding to its parameter in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.occurrences_of("icon-theme") > 0 {
            match matches.values_of("icon-theme")?.last() {
                Some("fancy") => Some(Self::Fancy),
                Some("unicode") => Some(Self::Unicode),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `IconTheme` variant from a [Config].
    ///
    /// If the `Config::icons::theme` has value and is one of "fancy" or "unicode",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(icon) = &config.icons {
            if let Some(theme) = icon.theme {
                return Some(theme);
            }
        }
        None
    }
}

/// The default value for `IconTheme` is [IconTheme::Fancy].
impl Default for IconTheme {
    fn default() -> Self {
        Self::Fancy
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IconSeparator(pub String);

impl Configurable<Self> for IconSeparator {
    /// Get a potential `IconSeparator` variant from [ArgMatches].
    ///
    /// If the argument is passed, this returns the variant corresponding to its parameter in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(_matches: &ArgMatches) -> Option<Self> {
        None
    }

    /// Get a potential `IconSeparator` variant from a [Config].
    ///
    /// This returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(icon) = &config.icons {
            if let Some(separator) = icon.separator.clone() {
                return Some(IconSeparator(separator));
            }
        }
        None
    }
}

/// The default value for `IconSeparator` is [" "].
impl Default for IconSeparator {
    fn default() -> Self {
        IconSeparator(" ".to_string())
    }
}

#[cfg(test)]
mod test_icon_option {
    use super::IconOption;

    use crate::app;
    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, IconOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--icon", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Always),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_autp() {
        let argv = vec!["lsd", "--icon", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Auto),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--icon", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--icon", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_icon_when_multi() {
        let argv = vec!["lsd", "--icon", "always", "--icon", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconOption::Never),
            IconOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Always), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Auto),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Auto), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: Some(IconOption::Never),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.classic = Some(true);
        c.icons = Some(Icons {
            when: Some(IconOption::Always),
            theme: None,
            separator: None,
        });
        assert_eq!(Some(IconOption::Never), IconOption::from_config(&c));
    }
}

#[cfg(test)]
mod test_icon_theme {
    use super::IconTheme;

    use crate::app;
    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, IconTheme::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_fancy() {
        let argv = vec!["lsd", "--icon-theme", "fancy"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconTheme::Fancy),
            IconTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_unicode() {
        let argv = vec!["lsd", "--icon-theme", "unicode"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconTheme::Unicode),
            IconTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_icon_multi() {
        let argv = vec!["lsd", "--icon-theme", "fancy", "--icon-theme", "unicode"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(IconTheme::Unicode),
            IconTheme::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, IconTheme::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_fancy() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Fancy),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Fancy), IconTheme::from_config(&c));
    }

    #[test]
    fn test_from_config_unicode() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: Some(IconTheme::Unicode),
            separator: None,
        });
        assert_eq!(Some(IconTheme::Unicode), IconTheme::from_config(&c));
    }
}

#[cfg(test)]
mod test_icon_separator {
    use super::IconSeparator;

    use crate::config_file::{Config, Icons};
    use crate::flags::Configurable;

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" ".to_string()),
        });
        let expected = Some(IconSeparator(" ".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }

    #[test]
    fn test_from_config_custom() {
        let mut c = Config::with_none();
        c.icons = Some(Icons {
            when: None,
            theme: None,
            separator: Some(" |".to_string()),
        });
        let expected = Some(IconSeparator(" |".to_string()));
        assert_eq!(expected, IconSeparator::from_config(&c));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_409() {
//    rusty_monitor::set_test_id(409);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_3_ref_0: &flags::icons::IconTheme = &mut icontheme_3;
    let mut icontheme_4: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_4_ref_0: &flags::icons::IconTheme = &mut icontheme_4;
    let mut icontheme_5: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_5_ref_0: &flags::icons::IconTheme = &mut icontheme_5;
    let mut icontheme_6: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_6_ref_0: &flags::icons::IconTheme = &mut icontheme_6;
    let mut icontheme_7: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_7_ref_0: &flags::icons::IconTheme = &mut icontheme_7;
    let mut icontheme_8: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_8_ref_0: &flags::icons::IconTheme = &mut icontheme_8;
    let mut icontheme_9: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_9_ref_0: &flags::icons::IconTheme = &mut icontheme_9;
    let mut icontheme_10: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_10_ref_0: &flags::icons::IconTheme = &mut icontheme_10;
    let mut tuple_0: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_10_ref_0);
    let mut tuple_1: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_9_ref_0);
    let mut tuple_2: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_8_ref_0);
    let mut tuple_3: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_7_ref_0);
    let mut tuple_4: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_6_ref_0);
    let mut tuple_5: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_5_ref_0);
    let mut tuple_6: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_4_ref_0);
    let mut tuple_7: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_3_ref_0);
    let mut tuple_8: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_2_ref_0);
    let mut tuple_9: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_1_ref_0);
    let mut tuple_10: () = crate::flags::icons::IconTheme::assert_receiver_is_total_eq(icontheme_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_390() {
//    rusty_monitor::set_test_id(390);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_2_ref_0: &flags::icons::IconOption = &mut iconoption_2;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_3_ref_0: &flags::icons::IconOption = &mut iconoption_3;
    let mut iconoption_4: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_4_ref_0: &flags::icons::IconOption = &mut iconoption_4;
    let mut iconoption_5: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_5_ref_0: &flags::icons::IconOption = &mut iconoption_5;
    let mut iconoption_6: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_6_ref_0: &flags::icons::IconOption = &mut iconoption_6;
    let mut iconoption_7: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_7_ref_0: &flags::icons::IconOption = &mut iconoption_7;
    let mut iconoption_8: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_8_ref_0: &flags::icons::IconOption = &mut iconoption_8;
    let mut iconoption_9: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_9_ref_0: &flags::icons::IconOption = &mut iconoption_9;
    let mut iconoption_10: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_10_ref_0: &flags::icons::IconOption = &mut iconoption_10;
    let mut iconoption_11: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_11_ref_0: &flags::icons::IconOption = &mut iconoption_11;
    let mut iconoption_12: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_12_ref_0: &flags::icons::IconOption = &mut iconoption_12;
    let mut iconoption_13: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_13_ref_0: &flags::icons::IconOption = &mut iconoption_13;
    let mut bool_0: bool = crate::flags::icons::IconOption::eq(iconoption_13_ref_0, iconoption_12_ref_0);
    let mut bool_1: bool = crate::flags::icons::IconOption::eq(iconoption_11_ref_0, iconoption_10_ref_0);
    let mut bool_2: bool = crate::flags::icons::IconOption::eq(iconoption_9_ref_0, iconoption_8_ref_0);
    let mut bool_3: bool = crate::flags::icons::IconOption::eq(iconoption_7_ref_0, iconoption_6_ref_0);
    let mut bool_4: bool = crate::flags::icons::IconOption::eq(iconoption_5_ref_0, iconoption_4_ref_0);
    let mut bool_5: bool = crate::flags::icons::IconOption::eq(iconoption_3_ref_0, iconoption_2_ref_0);
    let mut bool_6: bool = crate::flags::icons::IconOption::eq(iconoption_1_ref_0, iconoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3612() {
//    rusty_monitor::set_test_id(3612);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_1_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_1;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = true;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut bool_12: bool = crate::flags::icons::IconSeparator::eq(iconseparator_1_ref_0, iconseparator_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_870() {
//    rusty_monitor::set_test_id(870);
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "vlc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_13: bool = false;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_13};
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut bool_18: bool = true;
    let mut bool_19: bool = true;
    let mut bool_20: bool = false;
    let mut bool_21: bool = true;
    let mut bool_22: bool = true;
    let mut bool_23: bool = false;
    let mut bool_24: bool = false;
    let mut bool_25: bool = false;
    let mut permissions_1: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_25, user_write: bool_24, user_execute: bool_23, group_read: bool_22, group_write: bool_21, group_execute: bool_20, other_read: bool_19, other_write: bool_18, other_execute: bool_17, sticky: bool_16, setgid: bool_15, setuid: bool_14};
    let mut permissions_1_ref_0: &crate::meta::permissions::Permissions = &mut permissions_1;
    let mut option_1: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_2: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut option_3: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_0);
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_26: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_26);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut icons_3: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_1, theme: icontheme_1, separator: iconseparator_0};
    let mut icons_3_ref_0: &crate::flags::icons::Icons = &mut icons_3;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut icons_4: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_2, theme: icontheme_2, separator: iconseparator_1};
    let mut icons_4_ref_0: &crate::flags::icons::Icons = &mut icons_4;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icons_5: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_5_ref_0: &crate::flags::icons::Icons = &mut icons_5;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut iconseparator_2: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_6: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_3, theme: icontheme_3, separator: iconseparator_2};
    let mut icons_6_ref_0: &crate::flags::icons::Icons = &mut icons_6;
    let mut bool_27: bool = crate::flags::icons::Icons::eq(icons_6_ref_0, icons_5_ref_0);
    let mut bool_28: bool = crate::flags::icons::Icons::eq(icons_4_ref_0, icons_3_ref_0);
    let mut bool_29: bool = crate::flags::icons::Icons::eq(icons_2_ref_0, icons_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_702() {
//    rusty_monitor::set_test_id(702);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_0_ref_0);
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_1: std::option::Option<flags::icons::IconOption> = crate::flags::icons::IconOption::from_config(config_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4925() {
//    rusty_monitor::set_test_id(4925);
    let mut str_0: &str = "sass";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut tuple_0: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_458() {
//    rusty_monitor::set_test_id(458);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_3_ref_0: &flags::icons::IconTheme = &mut icontheme_3;
    let mut icontheme_4: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_4_ref_0: &flags::icons::IconTheme = &mut icontheme_4;
    let mut icontheme_5: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_5_ref_0: &flags::icons::IconTheme = &mut icontheme_5;
    let mut icontheme_6: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_6_ref_0: &flags::icons::IconTheme = &mut icontheme_6;
    let mut icontheme_7: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_7_ref_0: &flags::icons::IconTheme = &mut icontheme_7;
    let mut icontheme_8: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_8_ref_0: &flags::icons::IconTheme = &mut icontheme_8;
    let mut icontheme_9: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_9_ref_0: &flags::icons::IconTheme = &mut icontheme_9;
    let mut icontheme_10: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_10_ref_0: &flags::icons::IconTheme = &mut icontheme_10;
    let mut icontheme_11: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_11_ref_0: &flags::icons::IconTheme = &mut icontheme_11;
    let mut icontheme_12: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_12_ref_0: &flags::icons::IconTheme = &mut icontheme_12;
    let mut icontheme_13: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_13_ref_0: &flags::icons::IconTheme = &mut icontheme_13;
    let mut bool_0: bool = crate::flags::icons::IconTheme::eq(icontheme_13_ref_0, icontheme_12_ref_0);
    let mut bool_1: bool = crate::flags::icons::IconTheme::eq(icontheme_11_ref_0, icontheme_10_ref_0);
    let mut bool_2: bool = crate::flags::icons::IconTheme::eq(icontheme_9_ref_0, icontheme_8_ref_0);
    let mut bool_3: bool = crate::flags::icons::IconTheme::eq(icontheme_7_ref_0, icontheme_6_ref_0);
    let mut bool_4: bool = crate::flags::icons::IconTheme::eq(icontheme_5_ref_0, icontheme_4_ref_0);
    let mut bool_5: bool = crate::flags::icons::IconTheme::eq(icontheme_3_ref_0, icontheme_2_ref_0);
    let mut bool_6: bool = crate::flags::icons::IconTheme::eq(icontheme_1_ref_0, icontheme_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_639() {
//    rusty_monitor::set_test_id(639);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_1_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_1;
    let mut iconseparator_2: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_2_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_2;
    let mut iconseparator_3: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_3_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_3;
    let mut iconseparator_4: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_4_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_4;
    let mut iconseparator_5: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_5_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_5;
    let mut iconseparator_6: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_6_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_6;
    let mut iconseparator_7: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_7_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_7;
    let mut iconseparator_8: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_8_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_8;
    let mut iconseparator_9: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_9_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_9;
    let mut iconseparator_10: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_10_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_10;
    let mut tuple_0: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_10_ref_0);
    let mut tuple_1: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_9_ref_0);
    let mut tuple_2: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_8_ref_0);
    let mut tuple_3: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_7_ref_0);
    let mut tuple_4: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_6_ref_0);
    let mut tuple_5: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_5_ref_0);
    let mut tuple_6: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_4_ref_0);
    let mut tuple_7: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_3_ref_0);
    let mut tuple_8: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_2_ref_0);
    let mut tuple_9: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_1_ref_0);
    let mut tuple_10: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_51() {
//    rusty_monitor::set_test_id(51);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut usize_0: usize = 1usize;
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_5: bool = false;
    let mut usize_1: usize = 52usize;
    let mut bool_6: bool = false;
    let mut usize_2: usize = 64usize;
    let mut u64_0: u64 = 1073741824u64;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut bool_12: bool = true;
    let mut usize_3: usize = 40usize;
    let mut bool_13: bool = false;
    let mut u64_1: u64 = 1099511627776u64;
    let mut bool_14: bool = false;
    let mut bool_15: bool = true;
    let mut bool_16: bool = false;
    let mut bool_17: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_17);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_4: usize = 79usize;
    let mut bool_18: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_18, depth: usize_4};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut option_1: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_19: bool = false;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_19);
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut usize_5: usize = 120usize;
    let mut option_8: std::option::Option<usize> = std::option::Option::Some(usize_5);
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_1: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_9, depth: option_8};
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_1);
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_20: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_20);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_23, blocks: option_22, color: option_21, date: option_20, dereference: option_19, display: option_18, icons: option_17, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_7, permission: option_6, sorting: option_5, no_symlink: option_4, total_size: option_3, symlink_arrow: option_2, hyperlink: option_1};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_2: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_2);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut bool_21: bool = true;
    let mut bool_22: bool = false;
    let mut bool_23: bool = false;
    let mut bool_24: bool = true;
    let mut bool_25: bool = false;
    let mut bool_26: bool = false;
    let mut bool_27: bool = true;
    let mut bool_28: bool = true;
    let mut bool_29: bool = true;
    let mut bool_30: bool = true;
    let mut bool_31: bool = true;
    let mut bool_32: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_32, user_write: bool_31, user_execute: bool_30, group_read: bool_29, group_write: bool_28, group_execute: bool_27, other_read: bool_26, other_write: bool_25, other_execute: bool_24, sticky: bool_23, setgid: bool_22, setuid: bool_21};
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut option_24: std::option::Option<usize> = std::option::Option::None;
    let mut option_25: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_26: std::option::Option<crate::flags::icons::IconSeparator> = crate::flags::icons::IconSeparator::from_config(config_1_ref_0);
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_25, depth: option_24};
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::clone(icontheme_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_668() {
//    rusty_monitor::set_test_id(668);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_1, theme: icontheme_1, separator: iconseparator_1};
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut iconseparator_2: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut icons_3: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_2, theme: icontheme_2, separator: iconseparator_2};
    let mut icons_3_ref_0: &crate::flags::icons::Icons = &mut icons_3;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icons_4: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_4_ref_0: &crate::flags::icons::Icons = &mut icons_4;
    let mut icons_5: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_5_ref_0: &crate::flags::icons::Icons = &mut icons_5;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut tuple_0: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_5_ref_0);
    let mut tuple_1: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_4_ref_0);
    let mut tuple_2: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_3_ref_0);
    let mut tuple_3: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_2_ref_0);
    let mut tuple_4: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_1_ref_0);
    let mut tuple_5: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_265() {
//    rusty_monitor::set_test_id(265);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_1_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_1;
    let mut iconseparator_2: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_2_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_2;
    let mut iconseparator_3: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_3_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_3;
    let mut iconseparator_4: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_4_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_4;
    let mut iconseparator_5: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_5_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_5;
    let mut iconseparator_6: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_6_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_6;
    let mut iconseparator_7: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_7_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_7;
    let mut iconseparator_8: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_8_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_8;
    let mut iconseparator_9: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_9_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_9;
    let mut iconseparator_10: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_10_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_10;
    let mut iconseparator_11: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_11_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_11;
    let mut iconseparator_12: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_12_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_12;
    let mut iconseparator_13: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_13_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_13;
    let mut bool_0: bool = crate::flags::icons::IconSeparator::ne(iconseparator_13_ref_0, iconseparator_12_ref_0);
    let mut bool_1: bool = crate::flags::icons::IconSeparator::ne(iconseparator_11_ref_0, iconseparator_10_ref_0);
    let mut bool_2: bool = crate::flags::icons::IconSeparator::ne(iconseparator_9_ref_0, iconseparator_8_ref_0);
    let mut bool_3: bool = crate::flags::icons::IconSeparator::ne(iconseparator_7_ref_0, iconseparator_6_ref_0);
    let mut bool_4: bool = crate::flags::icons::IconSeparator::ne(iconseparator_5_ref_0, iconseparator_4_ref_0);
    let mut bool_5: bool = crate::flags::icons::IconSeparator::ne(iconseparator_3_ref_0, iconseparator_2_ref_0);
    let mut bool_6: bool = crate::flags::icons::IconSeparator::ne(iconseparator_1_ref_0, iconseparator_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1467() {
//    rusty_monitor::set_test_id(1467);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::clone(icontheme_0_ref_0);
//    panic!("From RustyUnit with love");
}
}