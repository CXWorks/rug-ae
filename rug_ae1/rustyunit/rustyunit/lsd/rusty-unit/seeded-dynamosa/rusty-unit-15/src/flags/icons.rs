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
#[timeout(30000)]fn rusty_test_2453() {
//    rusty_monitor::set_test_id(2453);
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_3: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_4: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_5: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_14: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_15: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_15, theme: option_14};
    let mut option_16: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_17: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_3: bool = false;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_4: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut bool_5: bool = crate::flags::icons::Icons::eq(icons_1_ref_0, icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_683() {
//    rusty_monitor::set_test_id(683);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut usize_0: usize = 80usize;
    let mut option_7: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_14: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut option_16: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_16, dereference: option_15, display: option_14, icons: option_13, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_3: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_1, theme: icontheme_1, separator: iconseparator_1};
    let mut icons_3_ref_0: &crate::flags::icons::Icons = &mut icons_3;
    let mut bool_1: bool = crate::flags::icons::Icons::ne(icons_3_ref_0, icons_2_ref_0);
    let mut bool_2: bool = crate::flags::icons::Icons::ne(icons_1_ref_0, icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3065() {
//    rusty_monitor::set_test_id(3065);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_1);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_7, theme: option_6, separator: option_5};
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_8: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_15: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_16: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_20: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut option_22: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_23: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut option_24: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_24, theme: option_23};
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_22, dereference: option_21, display: option_20, icons: option_19, ignore_globs: option_18, indicators: option_17, layout: option_16, recursion: option_15, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_11, total_size: option_10, symlink_arrow: option_9, hyperlink: option_8};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_5: bool = false;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_1_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_1;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_6: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut option_28: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_1_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut tuple_0: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_1_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_5};
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut option_29: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_0_ref_0);
    let mut bool_7: bool = crate::flags::icons::Icons::eq(icons_2_ref_0, icons_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6176() {
//    rusty_monitor::set_test_id(6176);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = true;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 360usize;
    let mut bool_1: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::clone(iconoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_643() {
//    rusty_monitor::set_test_id(643);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_2_ref_0: &flags::icons::IconOption = &mut iconoption_2;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_3_ref_0: &flags::icons::IconOption = &mut iconoption_3;
    let mut iconoption_4: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_4_ref_0: &flags::icons::IconOption = &mut iconoption_4;
    let mut iconoption_5: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_5_ref_0: &flags::icons::IconOption = &mut iconoption_5;
    let mut iconoption_6: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_6_ref_0: &flags::icons::IconOption = &mut iconoption_6;
    let mut iconoption_7: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_7_ref_0: &flags::icons::IconOption = &mut iconoption_7;
    let mut iconoption_8: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_8_ref_0: &flags::icons::IconOption = &mut iconoption_8;
    let mut iconoption_9: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_9_ref_0: &flags::icons::IconOption = &mut iconoption_9;
    let mut iconoption_10: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_10_ref_0: &flags::icons::IconOption = &mut iconoption_10;
    let mut tuple_0: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_10_ref_0);
    let mut tuple_1: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_9_ref_0);
    let mut tuple_2: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_8_ref_0);
    let mut tuple_3: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_7_ref_0);
    let mut tuple_4: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_6_ref_0);
    let mut tuple_5: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_5_ref_0);
    let mut tuple_6: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_4_ref_0);
    let mut tuple_7: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_3_ref_0);
    let mut tuple_8: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_2_ref_0);
    let mut tuple_9: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_1_ref_0);
    let mut tuple_10: () = crate::flags::icons::IconOption::assert_receiver_is_total_eq(iconoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_372() {
//    rusty_monitor::set_test_id(372);
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_2_ref_0: &flags::icons::IconOption = &mut iconoption_2;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_3_ref_0: &flags::icons::IconOption = &mut iconoption_3;
    let mut iconoption_4: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_4_ref_0: &flags::icons::IconOption = &mut iconoption_4;
    let mut iconoption_5: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_5_ref_0: &flags::icons::IconOption = &mut iconoption_5;
    let mut iconoption_6: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_6_ref_0: &flags::icons::IconOption = &mut iconoption_6;
    let mut iconoption_7: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_7_ref_0: &flags::icons::IconOption = &mut iconoption_7;
    let mut iconoption_8: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_8_ref_0: &flags::icons::IconOption = &mut iconoption_8;
    let mut iconoption_9: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut iconoption_9_ref_0: &flags::icons::IconOption = &mut iconoption_9;
    let mut iconoption_10: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_10_ref_0: &flags::icons::IconOption = &mut iconoption_10;
    let mut iconoption_11: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_11_ref_0: &flags::icons::IconOption = &mut iconoption_11;
    let mut iconoption_12: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut iconoption_12_ref_0: &flags::icons::IconOption = &mut iconoption_12;
    let mut iconoption_13: flags::icons::IconOption = crate::flags::icons::IconOption::default();
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
#[timeout(30000)]fn rusty_test_426() {
//    rusty_monitor::set_test_id(426);
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
    let mut iconseparator_11: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_10_ref_0);
    let mut iconseparator_12: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_9_ref_0);
    let mut iconseparator_13: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_8_ref_0);
    let mut iconseparator_14: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_7_ref_0);
    let mut iconseparator_15: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_6_ref_0);
    let mut iconseparator_16: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_5_ref_0);
    let mut iconseparator_17: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_4_ref_0);
    let mut iconseparator_18: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_3_ref_0);
    let mut iconseparator_19: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_2_ref_0);
    let mut iconseparator_20: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_1_ref_0);
    let mut iconseparator_21: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::clone(iconseparator_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8455() {
//    rusty_monitor::set_test_id(8455);
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut bool_3: bool = crate::flags::icons::Icons::eq(icons_1_ref_0, icons_0_ref_0);
    let mut unit_0: meta::size::Unit = crate::meta::size::Unit::Mega;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2435() {
//    rusty_monitor::set_test_id(2435);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut bool_2: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_16: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_4: bool = false;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut option_20: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_2_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut tuple_0: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_4};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut option_21: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_1_ref_0);
    let mut bool_6: bool = crate::flags::icons::Icons::eq(icons_1_ref_0, icons_0_ref_0);
    let mut option_22: std::option::Option<crate::flags::icons::IconSeparator> = crate::flags::icons::IconSeparator::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_708() {
//    rusty_monitor::set_test_id(708);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_1, theme: icontheme_1, separator: iconseparator_1};
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut icons_3: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_3_ref_0: &crate::flags::icons::Icons = &mut icons_3;
    let mut iconseparator_2: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut icons_4: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_2, theme: icontheme_2, separator: iconseparator_2};
    let mut icons_4_ref_0: &crate::flags::icons::Icons = &mut icons_4;
    let mut icons_5: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_5_ref_0: &crate::flags::icons::Icons = &mut icons_5;
    let mut iconseparator_3: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut icons_6: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_3, theme: icontheme_3, separator: iconseparator_3};
    let mut icons_6_ref_0: &crate::flags::icons::Icons = &mut icons_6;
    let mut icons_7: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_7_ref_0: &crate::flags::icons::Icons = &mut icons_7;
    let mut tuple_0: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_7_ref_0);
    let mut tuple_1: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_6_ref_0);
    let mut tuple_2: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_5_ref_0);
    let mut tuple_3: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_4_ref_0);
    let mut tuple_4: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_3_ref_0);
    let mut tuple_5: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_2_ref_0);
    let mut tuple_6: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_1_ref_0);
    let mut tuple_7: () = crate::flags::icons::Icons::assert_receiver_is_total_eq(icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8942() {
//    rusty_monitor::set_test_id(8942);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 2usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 120usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::default();
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_3, theme: themeoption_3};
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut option_0: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_2_ref_0);
    let mut config_3: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_3_ref_0: &crate::config_file::Config = &mut config_3;
    let mut option_1: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_3_ref_0);
    let mut config_4: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_4_ref_0: &crate::config_file::Config = &mut config_4;
    let mut option_2: std::option::Option<flags::icons::IconOption> = crate::flags::icons::IconOption::from_config(config_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_342() {
//    rusty_monitor::set_test_id(342);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_3_ref_0: &flags::icons::IconTheme = &mut icontheme_3;
    let mut icontheme_4: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_4_ref_0: &flags::icons::IconTheme = &mut icontheme_4;
    let mut icontheme_5: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_5_ref_0: &flags::icons::IconTheme = &mut icontheme_5;
    let mut icontheme_6: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_6_ref_0: &flags::icons::IconTheme = &mut icontheme_6;
    let mut icontheme_7: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_7_ref_0: &flags::icons::IconTheme = &mut icontheme_7;
    let mut icontheme_8: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_8_ref_0: &flags::icons::IconTheme = &mut icontheme_8;
    let mut icontheme_9: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_9_ref_0: &flags::icons::IconTheme = &mut icontheme_9;
    let mut icontheme_10: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
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
#[timeout(30000)]fn rusty_test_64() {
//    rusty_monitor::set_test_id(64);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::CharDevice;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut bool_6: bool = false;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut bool_11: bool = true;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_11, user_write: bool_10, user_execute: bool_9, group_read: bool_8, group_write: bool_7, group_execute: bool_6, other_read: bool_5, other_write: bool_4, other_execute: bool_3, sticky: bool_2, setgid: bool_1, setuid: bool_0};
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut option_0: std::option::Option<&std::fs::Metadata> = std::option::Option::None;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut iconoption_0_ref_0: &flags::icons::IconOption = &mut iconoption_0;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut iconoption_1_ref_0: &flags::icons::IconOption = &mut iconoption_1;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut iconseparator_1: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_1_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_1;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut bool_12: bool = crate::flags::icons::IconSeparator::ne(iconseparator_1_ref_0, iconseparator_0_ref_0);
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut bool_13: bool = crate::flags::icons::IconOption::eq(iconoption_1_ref_0, iconoption_0_ref_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9021() {
//    rusty_monitor::set_test_id(9021);
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_11: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_12: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_13: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_14: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_1: crate::config_file::Icons = crate::config_file::Icons {when: option_14, theme: option_13, separator: option_12};
    let mut option_15: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_1);
    let mut option_16: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_17: std::option::Option<bool> = std::option::Option::None;
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_19: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<bool> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_3: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_3};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_2_ref_0: &crate::flags::icons::Icons = &mut icons_2;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_5, uid: bool_4};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut icons_3: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_3_ref_0: &crate::flags::icons::Icons = &mut icons_3;
    let mut bool_6: bool = crate::flags::icons::Icons::ne(icons_3_ref_0, icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_750() {
//    rusty_monitor::set_test_id(750);
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_1_ref_0: &flags::icons::IconTheme = &mut icontheme_1;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_2_ref_0: &flags::icons::IconTheme = &mut icontheme_2;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_3_ref_0: &flags::icons::IconTheme = &mut icontheme_3;
    let mut icontheme_4: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_4_ref_0: &flags::icons::IconTheme = &mut icontheme_4;
    let mut icontheme_5: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_5_ref_0: &flags::icons::IconTheme = &mut icontheme_5;
    let mut icontheme_6: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_6_ref_0: &flags::icons::IconTheme = &mut icontheme_6;
    let mut icontheme_7: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_7_ref_0: &flags::icons::IconTheme = &mut icontheme_7;
    let mut icontheme_8: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_8_ref_0: &flags::icons::IconTheme = &mut icontheme_8;
    let mut icontheme_9: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_9_ref_0: &flags::icons::IconTheme = &mut icontheme_9;
    let mut icontheme_10: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut icontheme_10_ref_0: &flags::icons::IconTheme = &mut icontheme_10;
    let mut icontheme_11: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut icontheme_11_ref_0: &flags::icons::IconTheme = &mut icontheme_11;
    let mut icontheme_12: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut icontheme_12_ref_0: &flags::icons::IconTheme = &mut icontheme_12;
    let mut icontheme_13: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
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
#[timeout(30000)]fn rusty_test_8642() {
//    rusty_monitor::set_test_id(8642);
    let mut str_0: &str = "sql";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5952() {
//    rusty_monitor::set_test_id(5952);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7341() {
//    rusty_monitor::set_test_id(7341);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6577() {
//    rusty_monitor::set_test_id(6577);
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons {when: iconoption_0, theme: icontheme_0, separator: iconseparator_0};
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut bool_0: bool = true;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Pipe;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_1: u64 = crate::meta::size::Size::get_bytes(size_0_ref_0);
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_1, invalid: color_0};
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut icons_2: crate::flags::icons::Icons = crate::flags::icons::Icons::clone(icons_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7836() {
//    rusty_monitor::set_test_id(7836);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut str_0: &str = "lua";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 1024u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut elem_0: color::Elem = crate::color::Elem::DayOld;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut block_1: flags::blocks::Block = crate::flags::blocks::Block::Name;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut block_1_ref_0: &flags::blocks::Block = &mut block_1;
    let mut icontheme_0_ref_0: &flags::icons::IconTheme = &mut icontheme_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::clone(icontheme_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_610() {
//    rusty_monitor::set_test_id(610);
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
#[timeout(30000)]fn rusty_test_2564() {
//    rusty_monitor::set_test_id(2564);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_13: std::option::Option<bool> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut option_16: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut bool_3: bool = false;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_4: bool = crate::meta::filetype::FileType::is_dirlike(filetype_1);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut option_18: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_0_ref_0);
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut tuple_0: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_3};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    crate::meta::filetype::FileType::render(filetype_3, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_315() {
//    rusty_monitor::set_test_id(315);
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
    let mut bool_0: bool = crate::flags::icons::IconSeparator::eq(iconseparator_13_ref_0, iconseparator_12_ref_0);
    let mut bool_1: bool = crate::flags::icons::IconSeparator::eq(iconseparator_11_ref_0, iconseparator_10_ref_0);
    let mut bool_2: bool = crate::flags::icons::IconSeparator::eq(iconseparator_9_ref_0, iconseparator_8_ref_0);
    let mut bool_3: bool = crate::flags::icons::IconSeparator::eq(iconseparator_7_ref_0, iconseparator_6_ref_0);
    let mut bool_4: bool = crate::flags::icons::IconSeparator::eq(iconseparator_5_ref_0, iconseparator_4_ref_0);
    let mut bool_5: bool = crate::flags::icons::IconSeparator::eq(iconseparator_3_ref_0, iconseparator_2_ref_0);
    let mut bool_6: bool = crate::flags::icons::IconSeparator::eq(iconseparator_1_ref_0, iconseparator_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9064() {
//    rusty_monitor::set_test_id(9064);
    let mut option_0: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_0: usize = 0usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_3, depth: option_2};
    let mut option_4: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_5: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_6: std::option::Option<bool> = std::option::Option::None;
    let mut option_7: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_8: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_9: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_13, theme: option_12};
    let mut option_14: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_15: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut icons_0: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_0_ref_0: &crate::flags::icons::Icons = &mut icons_0;
    let mut icons_1: crate::flags::icons::Icons = crate::flags::icons::Icons::default();
    let mut icons_1_ref_0: &crate::flags::icons::Icons = &mut icons_1;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_17: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_18: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut bool_3: bool = true;
    let mut option_20: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_21: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut option_22: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut option_23: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_24: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_25: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_26: std::option::Option<bool> = std::option::Option::None;
    let mut option_27: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_28: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_29: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut option_30: std::option::Option<bool> = std::option::Option::None;
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_32: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_33: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_33, theme: option_32};
    let mut option_34: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_1);
    let mut option_35: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_36: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_36, blocks: option_35, color: option_34, date: option_31, dereference: option_30, display: option_29, icons: option_28, ignore_globs: option_27, indicators: option_26, layout: option_25, recursion: option_24, size: option_23, permission: option_22, sorting: option_21, no_symlink: option_20, total_size: option_19, symlink_arrow: option_18, hyperlink: option_17};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut iconseparator_0: crate::flags::icons::IconSeparator = crate::flags::icons::IconSeparator::default();
    let mut iconseparator_0_ref_0: &crate::flags::icons::IconSeparator = &mut iconseparator_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut bool_5: bool = crate::meta::filetype::FileType::is_dirlike(filetype_2);
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut option_37: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_1_ref_0);
    let mut filetype_3: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut tuple_0: () = crate::flags::icons::IconSeparator::assert_receiver_is_total_eq(iconseparator_0_ref_0);
    let mut elem_0: color::Elem = crate::color::Elem::Special;
    let mut filetype_4: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_5, exec: bool_5};
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::default();
    crate::meta::filetype::FileType::render(filetype_4, colors_0_ref_0);
    let mut option_38: std::option::Option<flags::icons::IconTheme> = crate::flags::icons::IconTheme::from_config(config_0_ref_0);
    let mut bool_6: bool = crate::flags::icons::Icons::eq(icons_1_ref_0, icons_0_ref_0);
//    panic!("From RustyUnit with love");
}
}