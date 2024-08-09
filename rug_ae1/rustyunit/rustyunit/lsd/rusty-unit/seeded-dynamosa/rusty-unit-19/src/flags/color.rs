//! This module defines the [Color]. To set it up from [ArgMatches], a [Config] and its [Default]
//! value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;
use crate::print_error;

use clap::ArgMatches;
use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::env;
use std::fmt;

/// A collection of flags on how to use colors.
#[derive(Clone, Debug, Default)]
pub struct Color {
    /// When to use color.
    pub when: ColorOption,
    pub theme: ThemeOption,
}

impl Color {
    /// Get a `Color` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [ColorOption] is configured with their respective [Configurable] implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let when = ColorOption::configure_from(matches, config);
        let theme = ThemeOption::from_config(config);
        Self { when, theme }
    }
}

/// ThemeOption could be one of the following:
/// Custom(*.yaml): use the YAML theme file as theme file
/// if error happened, use the default theme
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ThemeOption {
    NoColor,
    Default,
    #[allow(dead_code)]
    NoLscolors,
    Custom(String),
}

impl ThemeOption {
    fn from_config(config: &Config) -> ThemeOption {
        if let Some(classic) = config.classic {
            if classic {
                return ThemeOption::NoColor;
            }
        }
        if let Some(c) = &config.color {
            if let Some(t) = &c.theme {
                return t.clone();
            }
        }

        ThemeOption::default()
    }
}

impl Default for ThemeOption {
    fn default() -> Self {
        ThemeOption::Default
    }
}

impl<'de> de::Deserialize<'de> for ThemeOption {
    fn deserialize<D>(deserializer: D) -> Result<ThemeOption, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeOptionVisitor;

        impl<'de> Visitor<'de> for ThemeOptionVisitor {
            type Value = ThemeOption;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`default` or <theme-file-path>")
            }

            fn visit_str<E>(self, value: &str) -> Result<ThemeOption, E>
            where
                E: de::Error,
            {
                match value {
                    "default" => Ok(ThemeOption::Default),
                    str => Ok(ThemeOption::Custom(str.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ThemeOptionVisitor)
    }
}

/// The flag showing when to use colors in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorOption {
    Always,
    Auto,
    Never,
}

impl ColorOption {
    /// Get a Color value from a [String].
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "always" => Some(Self::Always),
            "auto" => Some(Self::Auto),
            "never" => Some(Self::Never),
            _ => {
                print_error!(
                    "Config color.when could only be one of auto, always and never, got {}.",
                    &value
                );
                None
            }
        }
    }
}

impl Configurable<Self> for ColorOption {
    /// Get a potential `ColorOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [ColorOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("color") > 0 {
            if let Some(color) = matches.values_of("color")?.last() {
                Self::from_str(color)
            } else {
                panic!("Bad color args. This should not be reachable!");
            }
        } else {
            None
        }
    }

    /// Get a potential `ColorOption` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(ColorOption::Never),
    /// Otherwise if the `Config::color::when` has value and is one of "always", "auto" or "never"
    /// this returns its corresponding variant in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = config.classic {
            return Some(Self::Never);
        }

        if let Some(c) = &config.color {
            c.when
        } else {
            None
        }
    }

    fn from_environment() -> Option<Self> {
        if env::var("NO_COLOR").is_ok() {
            Some(Self::Never)
        } else {
            None
        }
    }
}

/// The default value for `ColorOption` is [ColorOption::Auto].
impl Default for ColorOption {
    fn default() -> Self {
        Self::Auto
    }
}

#[cfg(test)]
mod test_color_option {
    use super::ColorOption;

    use crate::app;
    use crate::config_file::{self, Config};
    use crate::flags::Configurable;

    use std::env::set_var;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, ColorOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--color", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Always),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_auto() {
        let argv = vec!["lsd", "--color", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Auto),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_env_no_color() {
        set_var("NO_COLOR", "true");
        assert_eq!(Some(ColorOption::Never), ColorOption::from_environment());
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--color", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_color_multiple() {
        let argv = vec!["lsd", "--color", "always", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, ColorOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });

        assert_eq!(Some(ColorOption::Always), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Auto),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Never),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });
        c.classic = Some(true);
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(&c));
    }
}

#[cfg(test)]
mod test_theme_option {
    use super::ThemeOption;
    use crate::config_file::{self, Config};

    #[test]
    fn test_from_config_none_default() {
        assert_eq!(
            ThemeOption::Default,
            ThemeOption::from_config(&Config::with_none())
        );
    }

    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });

        assert_eq!(ThemeOption::Default, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_no_color() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoColor),
        });
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_no_lscolor() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoLscolors),
        });
        assert_eq!(ThemeOption::NoLscolors, ThemeOption::from_config(&c));
    }

    #[test]
    fn test_from_config_bad_file_flag() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Custom("not-existed".to_string())),
        });
        assert_eq!(
            ThemeOption::Custom("not-existed".to_string()),
            ThemeOption::from_config(&c)
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });
        c.classic = Some(true);
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(&c));
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
#[timeout(30000)]fn rusty_test_9117() {
//    rusty_monitor::set_test_id(9117);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut bool_1: bool = crate::flags::color::ThemeOption::ne(themeoption_4_ref_0, themeoption_3_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_5_ref_0: &flags::color::ThemeOption = &mut themeoption_5;
    let mut bool_2: bool = crate::flags::color::ThemeOption::eq(themeoption_5_ref_0, themeoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7778() {
//    rusty_monitor::set_test_id(7778);
    let mut str_0: &str = "setgid";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut bool_1: bool = crate::flags::color::ThemeOption::ne(themeoption_4_ref_0, themeoption_3_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut tuple_0: () = crate::flags::color::ThemeOption::assert_receiver_is_total_eq(themeoption_0_ref_0);
    let mut option_0: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2947() {
//    rusty_monitor::set_test_id(2947);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut bool_1: bool = crate::flags::color::ThemeOption::ne(themeoption_3_ref_0, themeoption_2_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_0: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4425() {
//    rusty_monitor::set_test_id(4425);
    let mut bool_0: bool = false;
    let mut str_0: &str = "";
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut str_1: &str = "Relative";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "1dg4M39O";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4_ref_0: &str = &mut str_4;
    let mut elem_0: color::Elem = crate::color::Elem::INode {valid: bool_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4568() {
//    rusty_monitor::set_test_id(4568);
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_1: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_15: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_2: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_18: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut str_0: &str = "fish";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_19: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_str(str_0_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_19, theme: option_18};
    let mut option_20: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_21: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_22, blocks: option_21, color: option_20, date: option_17, dereference: option_16, display: option_15, icons: option_14, ignore_globs: option_13, indicators: option_12, layout: option_11, recursion: option_10, size: option_9, permission: option_8, sorting: option_7, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_1_ref_0);
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_5_ref_0: &flags::color::ThemeOption = &mut themeoption_5;
    let mut themeoption_6: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_6_ref_0: &flags::color::ThemeOption = &mut themeoption_6;
    let mut themeoption_7: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_7_ref_0: &flags::color::ThemeOption = &mut themeoption_7;
    let mut themeoption_8: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_8_ref_0: &flags::color::ThemeOption = &mut themeoption_8;
    let mut themeoption_9: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_9_ref_0: &flags::color::ThemeOption = &mut themeoption_9;
    let mut themeoption_10: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_10_ref_0: &flags::color::ThemeOption = &mut themeoption_10;
    let mut themeoption_11: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_11_ref_0: &flags::color::ThemeOption = &mut themeoption_11;
    let mut themeoption_12: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_12_ref_0: &flags::color::ThemeOption = &mut themeoption_12;
    let mut themeoption_13: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_13_ref_0: &flags::color::ThemeOption = &mut themeoption_13;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_14: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
    let mut themeoption_14_ref_0: &flags::color::ThemeOption = &mut themeoption_14;
    let mut themeoption_15: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_15_ref_0: &flags::color::ThemeOption = &mut themeoption_15;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7729() {
//    rusty_monitor::set_test_id(7729);
    let mut bool_0: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_1: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut option_7: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_8: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_8, theme: option_7};
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2usize;
    let mut bool_3: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut option_12: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_4: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut bool_5: bool = true;
    let mut option_15: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_16: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_6: bool = false;
    let mut option_17: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_18: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_18, reverse: option_17, dir_grouping: option_16};
    let mut option_19: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut option_20: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_21: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_22: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_23: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_7: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_25: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_26: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_27: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_28: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_28, theme: option_27, separator: option_26};
    let mut option_29: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_30: std::option::Option<bool> = std::option::Option::None;
    let mut option_31: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_32: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_33: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_34: std::option::Option<bool> = std::option::Option::None;
    let mut config_1: crate::config_file::Config = crate::config_file::Config {classic: option_34, blocks: option_33, color: option_32, date: option_31, dereference: option_30, display: option_4, icons: option_29, ignore_globs: option_25, indicators: option_24, layout: option_23, recursion: option_22, size: option_21, permission: option_20, sorting: option_19, no_symlink: option_15, total_size: option_14, symlink_arrow: option_13, hyperlink: option_12};
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "torrent";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_8: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_8};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut bool_9: bool = false;
    let mut bool_10: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_10, exec: bool_9};
    let mut bool_11: bool = crate::color::Elem::has_suid(elem_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6965() {
//    rusty_monitor::set_test_id(6965);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::SymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_9: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_11: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_12: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut bool_3: bool = false;
    let mut option_13: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_15: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_1);
    let mut option_16: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_16, theme: option_15};
    let mut option_17: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_18: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_19, blocks: option_18, color: option_17, date: option_14, dereference: option_13, display: option_12, icons: option_11, ignore_globs: option_10, indicators: option_9, layout: option_8, recursion: option_7, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_3_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut bool_4: bool = false;
    let mut elem_4: color::Elem = crate::color::Elem::Links {valid: bool_4};
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_5: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_4_ref_0);
    let mut theme_5: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_5_ref_0: &crate::color::theme::Theme = &mut theme_5;
    let mut bool_5: bool = false;
    let mut elem_5: color::Elem = crate::color::Elem::Links {valid: bool_5};
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_6: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_5_ref_0);
    let mut theme_6: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_6_ref_0: &crate::color::theme::Theme = &mut theme_6;
    let mut elem_6: color::Elem = crate::color::Elem::NonFile;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut elem_7: color::Elem = crate::color::Elem::Write;
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_458() {
//    rusty_monitor::set_test_id(458);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_2: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_2_ref_0: &crate::color::Colors = &mut colors_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8903() {
//    rusty_monitor::set_test_id(8903);
    let mut elem_0: color::Elem = crate::color::Elem::FileMedium;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::ExecSticky;
    let mut str_0: &str = "dippXakpsnf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_1, exec: bool_0};
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::Socket;
    let mut elem_2: color::Elem = crate::color::Elem::TreeEdge;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0_ref_0: &crate::flags::color::Color = &mut color_0;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::clone(color_0_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::SymLink;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_719() {
//    rusty_monitor::set_test_id(719);
    let mut elem_0: color::Elem = crate::color::Elem::Acl;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6028() {
//    rusty_monitor::set_test_id(6028);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut bool_0: bool = crate::flags::color::ThemeOption::ne(themeoption_2_ref_0, themeoption_1_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut hyperlinkoption_0_ref_0: &flags::hyperlink::HyperlinkOption = &mut hyperlinkoption_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_416() {
//    rusty_monitor::set_test_id(416);
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_0_ref_0: &crate::config_file::Color = &mut color_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8649() {
//    rusty_monitor::set_test_id(8649);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut option_0: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut str_0: &str = "Provided file path is invalid";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "MissingSymLinkTarget";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut option_1: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut option_2: std::option::Option<usize> = std::option::Option::None;
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_2: &str = "Context";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_6: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_str(str_2_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_6, theme: option_5};
    let mut option_7: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_8: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut u64_0: u64 = 0u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut option_9: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut option_10: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_11: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8560() {
//    rusty_monitor::set_test_id(8560);
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut elem_0: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_2: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut bool_3: bool = crate::flags::color::ThemeOption::ne(themeoption_3_ref_0, themeoption_2_ref_0);
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut bool_4: bool = crate::color::Elem::has_suid(elem_0_ref_0);
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1721() {
//    rusty_monitor::set_test_id(1721);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_5_ref_0: &flags::color::ThemeOption = &mut themeoption_5;
    let mut themeoption_6: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_6_ref_0: &flags::color::ThemeOption = &mut themeoption_6;
    let mut themeoption_7: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_7_ref_0: &flags::color::ThemeOption = &mut themeoption_7;
    let mut themeoption_8: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_8_ref_0: &flags::color::ThemeOption = &mut themeoption_8;
    let mut themeoption_9: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_9_ref_0: &flags::color::ThemeOption = &mut themeoption_9;
    let mut themeoption_10: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_10_ref_0: &flags::color::ThemeOption = &mut themeoption_10;
    let mut themeoption_11: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_11_ref_0: &flags::color::ThemeOption = &mut themeoption_11;
    let mut themeoption_12: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_12_ref_0: &flags::color::ThemeOption = &mut themeoption_12;
    let mut themeoption_13: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_13_ref_0: &flags::color::ThemeOption = &mut themeoption_13;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_14: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_2_ref_0);
    let mut themeoption_14_ref_0: &flags::color::ThemeOption = &mut themeoption_14;
    let mut themeoption_15: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_15_ref_0: &flags::color::ThemeOption = &mut themeoption_15;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_16: flags::color::ThemeOption = crate::flags::color::ThemeOption::clone(themeoption_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3963() {
//    rusty_monitor::set_test_id(3963);
    let mut option_0: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_1: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_2: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_3: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut option_4: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_5: std::option::Option<bool> = std::option::Option::None;
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut option_7: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_8: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_8, theme: option_7};
    let mut option_9: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_0);
    let mut option_10: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_1: bool = false;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 2usize;
    let mut bool_2: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut option_12: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_14: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_15: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::None;
    let mut bool_4: bool = false;
    let mut option_16: std::option::Option<bool> = std::option::Option::Some(bool_4);
    let mut option_17: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_17, reverse: option_16, dir_grouping: option_15};
    let mut option_18: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_19: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_20: std::option::Option<crate::config_file::Recursion> = std::option::Option::None;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut option_21: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_1);
    let mut bool_5: bool = true;
    let mut option_22: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut option_25: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_1);
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut option_26: std::option::Option<flags::icons::IconOption> = std::option::Option::Some(iconoption_1);
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_26, theme: option_25, separator: option_24};
    let mut option_27: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_28: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_29: std::option::Option<bool> = std::option::Option::None;
    let mut option_30: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_31: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_32: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_33: std::option::Option<bool> = std::option::Option::None;
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut str_0: &str = "torrent";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_6: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_6};
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::BlockDevice;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_7: bool = false;
    let mut bool_8: bool = true;
    let mut filetype_2: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_8, exec: bool_7};
    crate::meta::filetype::FileType::render(filetype_2, colors_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2481() {
//    rusty_monitor::set_test_id(2481);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Context;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_2: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut bool_1: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_4: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_5: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_6: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut option_7: std::option::Option<usize> = std::option::Option::None;
    let mut option_8: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_8, depth: option_7};
    let mut option_9: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut option_10: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_11: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_12: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_15: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_15, theme: option_14, separator: option_13};
    let mut option_16: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut option_17: std::option::Option<flags::display::Display> = std::option::Option::None;
    let mut option_18: std::option::Option<bool> = std::option::Option::None;
    let mut option_19: std::option::Option<std::string::String> = std::option::Option::None;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_20: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut option_21: std::option::Option<flags::color::ColorOption> = std::option::Option::Some(coloroption_0);
    let mut color_2: crate::config_file::Color = crate::config_file::Color {when: option_21, theme: option_20};
    let mut option_22: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_2);
    let mut option_23: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_24: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_24, blocks: option_23, color: option_22, date: option_19, dereference: option_18, display: option_17, icons: option_16, ignore_globs: option_12, indicators: option_11, layout: option_10, recursion: option_9, size: option_6, permission: option_5, sorting: option_4, no_symlink: option_3, total_size: option_2, symlink_arrow: option_1, hyperlink: option_0};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_4: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_3: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut bool_5: bool = crate::flags::color::ThemeOption::ne(themeoption_4_ref_0, themeoption_3_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut option_25: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6217() {
//    rusty_monitor::set_test_id(6217);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 6usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut bool_1: bool = crate::flags::color::ThemeOption::ne(themeoption_0_ref_0, themeoption_3_ref_0);
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_383() {
//    rusty_monitor::set_test_id(383);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_1_ref_0: &flags::color::ColorOption = &mut coloroption_1;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_2_ref_0: &flags::color::ColorOption = &mut coloroption_2;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_3_ref_0: &flags::color::ColorOption = &mut coloroption_3;
    let mut coloroption_4: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_4_ref_0: &flags::color::ColorOption = &mut coloroption_4;
    let mut coloroption_5: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_5_ref_0: &flags::color::ColorOption = &mut coloroption_5;
    let mut coloroption_6: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_6_ref_0: &flags::color::ColorOption = &mut coloroption_6;
    let mut coloroption_7: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_7_ref_0: &flags::color::ColorOption = &mut coloroption_7;
    let mut coloroption_8: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_8_ref_0: &flags::color::ColorOption = &mut coloroption_8;
    let mut coloroption_9: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_9_ref_0: &flags::color::ColorOption = &mut coloroption_9;
    let mut coloroption_10: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_10_ref_0: &flags::color::ColorOption = &mut coloroption_10;
    let mut tuple_0: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_10_ref_0);
    let mut tuple_1: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_9_ref_0);
    let mut tuple_2: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_8_ref_0);
    let mut tuple_3: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_7_ref_0);
    let mut tuple_4: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_6_ref_0);
    let mut tuple_5: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_5_ref_0);
    let mut tuple_6: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_4_ref_0);
    let mut tuple_7: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_3_ref_0);
    let mut tuple_8: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_2_ref_0);
    let mut tuple_9: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_1_ref_0);
    let mut tuple_10: () = crate::flags::color::ColorOption::assert_receiver_is_total_eq(coloroption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_565() {
//    rusty_monitor::set_test_id(565);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_1_ref_0: &flags::color::ColorOption = &mut coloroption_1;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_2_ref_0: &flags::color::ColorOption = &mut coloroption_2;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_3_ref_0: &flags::color::ColorOption = &mut coloroption_3;
    let mut coloroption_4: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_4_ref_0: &flags::color::ColorOption = &mut coloroption_4;
    let mut coloroption_5: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_5_ref_0: &flags::color::ColorOption = &mut coloroption_5;
    let mut coloroption_6: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_6_ref_0: &flags::color::ColorOption = &mut coloroption_6;
    let mut coloroption_7: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_7_ref_0: &flags::color::ColorOption = &mut coloroption_7;
    let mut coloroption_8: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_8_ref_0: &flags::color::ColorOption = &mut coloroption_8;
    let mut coloroption_9: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_9_ref_0: &flags::color::ColorOption = &mut coloroption_9;
    let mut coloroption_10: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_10_ref_0: &flags::color::ColorOption = &mut coloroption_10;
    let mut coloroption_11: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_11_ref_0: &flags::color::ColorOption = &mut coloroption_11;
    let mut coloroption_12: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_12_ref_0: &flags::color::ColorOption = &mut coloroption_12;
    let mut coloroption_13: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_13_ref_0: &flags::color::ColorOption = &mut coloroption_13;
    let mut bool_0: bool = crate::flags::color::ColorOption::eq(coloroption_13_ref_0, coloroption_12_ref_0);
    let mut bool_1: bool = crate::flags::color::ColorOption::eq(coloroption_11_ref_0, coloroption_10_ref_0);
    let mut bool_2: bool = crate::flags::color::ColorOption::eq(coloroption_9_ref_0, coloroption_8_ref_0);
    let mut bool_3: bool = crate::flags::color::ColorOption::eq(coloroption_7_ref_0, coloroption_6_ref_0);
    let mut bool_4: bool = crate::flags::color::ColorOption::eq(coloroption_5_ref_0, coloroption_4_ref_0);
    let mut bool_5: bool = crate::flags::color::ColorOption::eq(coloroption_3_ref_0, coloroption_2_ref_0);
    let mut bool_6: bool = crate::flags::color::ColorOption::eq(coloroption_1_ref_0, coloroption_0_ref_0);
//    panic!("From RustyUnit with love");
}
}