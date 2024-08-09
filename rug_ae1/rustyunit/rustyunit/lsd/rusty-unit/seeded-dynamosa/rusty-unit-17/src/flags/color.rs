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
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use flags::Configurable;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1556() {
//    rusty_monitor::set_test_id(1556);
    let mut bool_0: bool = false;
    let mut option_0: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut option_1: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_2: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut option_3: std::option::Option<flags::size::SizeFlag> = std::option::Option::None;
    let mut usize_0: usize = 1usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut bool_1: bool = true;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut recursion_0: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_5, depth: option_4};
    let mut option_6: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_0);
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_7: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut bool_2: bool = false;
    let mut option_8: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_9: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_10: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_11: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_3: bool = true;
    let mut option_12: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut option_13: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_14: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_15: std::option::Option<bool> = std::option::Option::None;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 120usize;
    let mut bool_4: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut option_16: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::Some(hyperlinkoption_0);
    let mut option_17: std::option::Option<std::string::String> = std::option::Option::None;
    let mut bool_5: bool = false;
    let mut option_18: std::option::Option<bool> = std::option::Option::Some(bool_5);
    let mut bool_6: bool = true;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_6);
    let mut option_20: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_21: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_22: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_1);
    let mut option_23: std::option::Option<usize> = std::option::Option::None;
    let mut option_24: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_24, depth: option_23};
    let mut option_25: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut option_26: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut bool_7: bool = false;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_7);
    let mut option_28: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_29: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut option_30: std::option::Option<flags::display::Display> = std::option::Option::Some(display_1);
    let mut bool_8: bool = false;
    let mut option_31: std::option::Option<bool> = std::option::Option::Some(bool_8);
    let mut option_32: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_33: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_34: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_35: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_35, blocks: option_34, color: option_33, date: option_32, dereference: option_31, display: option_30, icons: option_29, ignore_globs: option_28, indicators: option_27, layout: option_26, recursion: option_25, size: option_22, permission: option_21, sorting: option_20, no_symlink: option_19, total_size: option_18, symlink_arrow: option_17, hyperlink: option_16};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut u64_0: u64 = 1048576u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut option_36: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut option_37: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_config(config_1_ref_0);
    let mut color_1: crate::config_file::Color = crate::config_file::Color {when: option_37, theme: option_36};
    let mut color_1_ref_0: &crate::config_file::Color = &mut color_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7383() {
//    rusty_monitor::set_test_id(7383);
    let mut usize_0: usize = 80usize;
    let mut bool_0: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_1: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_1};
    let mut str_0: &str = ":
";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 120usize;
    let mut bool_2: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_2};
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut elem_0: color::Elem = crate::color::Elem::Read;
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_375() {
//    rusty_monitor::set_test_id(375);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_1_ref_0: &flags::color::ColorOption = &mut coloroption_1;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_2_ref_0: &flags::color::ColorOption = &mut coloroption_2;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_3_ref_0: &flags::color::ColorOption = &mut coloroption_3;
    let mut coloroption_4: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_4_ref_0: &flags::color::ColorOption = &mut coloroption_4;
    let mut coloroption_5: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_5_ref_0: &flags::color::ColorOption = &mut coloroption_5;
    let mut coloroption_6: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_6_ref_0: &flags::color::ColorOption = &mut coloroption_6;
    let mut coloroption_7: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_7_ref_0: &flags::color::ColorOption = &mut coloroption_7;
    let mut coloroption_8: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut coloroption_8_ref_0: &flags::color::ColorOption = &mut coloroption_8;
    let mut coloroption_9: flags::color::ColorOption = crate::flags::color::ColorOption::default();
    let mut coloroption_9_ref_0: &flags::color::ColorOption = &mut coloroption_9;
    let mut coloroption_10: flags::color::ColorOption = crate::flags::color::ColorOption::default();
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
#[timeout(30000)]fn rusty_test_82() {
//    rusty_monitor::set_test_id(82);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut coloroption_1_ref_0: &flags::color::ColorOption = &mut coloroption_1;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_0};
    let mut color_0_ref_0: &crate::flags::color::Color = &mut color_0;
    let mut elem_0: color::Elem = crate::color::Elem::Exec;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut bool_0: bool = false;
    let mut elem_1: color::Elem = crate::color::Elem::Dir {uid: bool_0};
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_2, uid: bool_1};
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut option_0: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut str_0: &str = "smack_context";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_0_ref_0);
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_1: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut date_1_ref_0: &meta::date::Date = &mut date_1;
    let mut coloroption_3_ref_0: &flags::color::ColorOption = &mut coloroption_3;
    let mut coloroption_4: flags::color::ColorOption = crate::flags::color::ColorOption::clone(coloroption_3_ref_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut bool_3: bool = crate::flags::color::ColorOption::eq(coloroption_1_ref_0, coloroption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6595() {
//    rusty_monitor::set_test_id(6595);
    let mut str_0: &str = "1";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "epub";
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut str_1_ref_0: &str = &mut str_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7654() {
//    rusty_monitor::set_test_id(7654);
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut coloroption_0_ref_0: &flags::color::ColorOption = &mut coloroption_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_0: usize = 360usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_2};
    let mut filetype_0_ref_0: &meta::filetype::FileType = &mut filetype_0;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut coloroption_1_ref_0: &flags::color::ColorOption = &mut coloroption_1;
    let mut bool_1: bool = crate::flags::color::ColorOption::eq(coloroption_1_ref_0, coloroption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_276() {
//    rusty_monitor::set_test_id(276);
    let mut option_0: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_1: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_2: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_3: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_4: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_5: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_6: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_7: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_8: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_9: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_10: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_11: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_12: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_13: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_14: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_15: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_16: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_17: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_18: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_19: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_20: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_21: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_22: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_23: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_24: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_25: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_26: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_27: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_28: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut option_29: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5615() {
//    rusty_monitor::set_test_id(5615);
    let mut option_0: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut bool_0: bool = true;
    let mut option_3: std::option::Option<bool> = std::option::Option::Some(bool_0);
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut option_4: std::option::Option<flags::sorting::DirGrouping> = std::option::Option::Some(dirgrouping_0);
    let mut bool_1: bool = false;
    let mut option_5: std::option::Option<bool> = std::option::Option::Some(bool_1);
    let mut option_6: std::option::Option<flags::sorting::SortColumn> = std::option::Option::None;
    let mut sorting_0: crate::config_file::Sorting = crate::config_file::Sorting {column: option_6, reverse: option_5, dir_grouping: option_4};
    let mut option_7: std::option::Option<crate::config_file::Sorting> = std::option::Option::Some(sorting_0);
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut option_8: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::Some(permissionflag_0);
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut option_9: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_0);
    let mut option_10: std::option::Option<usize> = std::option::Option::None;
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut option_11: std::option::Option<flags::layout::Layout> = std::option::Option::Some(layout_0);
    let mut option_12: std::option::Option<bool> = std::option::Option::None;
    let mut option_13: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_14: std::option::Option<std::string::String> = std::option::Option::None;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut option_15: std::option::Option<flags::icons::IconTheme> = std::option::Option::Some(icontheme_0);
    let mut option_16: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut icons_0: crate::config_file::Icons = crate::config_file::Icons {when: option_16, theme: option_15, separator: option_14};
    let mut option_17: std::option::Option<crate::config_file::Icons> = std::option::Option::Some(icons_0);
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_18: std::option::Option<flags::display::Display> = std::option::Option::Some(display_0);
    let mut bool_2: bool = false;
    let mut option_19: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut option_20: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_22: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_3: bool = true;
    let mut option_23: std::option::Option<bool> = std::option::Option::Some(bool_3);
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 6usize;
    let mut bool_4: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_0};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut config_2: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_2_ref_0: &crate::config_file::Config = &mut config_2;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_2_ref_0);
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_3_ref_0: &flags::color::ThemeOption = &mut themeoption_3;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_4_ref_0: &flags::color::ThemeOption = &mut themeoption_4;
    let mut themeoption_5: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_5_ref_0: &flags::color::ThemeOption = &mut themeoption_5;
    let mut themeoption_6: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut themeoption_6_ref_0: &flags::color::ThemeOption = &mut themeoption_6;
    let mut themeoption_7: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut bool_5: bool = crate::flags::color::ThemeOption::eq(themeoption_5_ref_0, themeoption_4_ref_0);
    let mut bool_6: bool = crate::flags::color::ThemeOption::eq(themeoption_3_ref_0, themeoption_2_ref_0);
    let mut bool_7: bool = crate::flags::color::ThemeOption::eq(themeoption_1_ref_0, themeoption_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2987() {
//    rusty_monitor::set_test_id(2987);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 120usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Always;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_1};
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_1: crate::color::Colors = crate::color::Colors::new(themeoption_2);
    let mut colors_1_ref_0: &crate::color::Colors = &mut colors_1;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 120usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_1: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut option_0: std::option::Option<flags::icons::IconTheme> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::icons::IconOption> = std::option::Option::None;
    let mut display_2: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut option_2: std::option::Option<flags::display::Display> = std::option::Option::Some(display_2);
    let mut option_3: std::option::Option<bool> = std::option::Option::None;
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_5: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_6: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_7: std::option::Option<bool> = std::option::Option::None;
    let mut option_8: std::option::Option<flags::hyperlink::HyperlinkOption> = std::option::Option::None;
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_10: std::option::Option<bool> = std::option::Option::None;
    let mut option_11: std::option::Option<bool> = std::option::Option::None;
    let mut option_12: std::option::Option<crate::config_file::Sorting> = std::option::Option::None;
    let mut option_13: std::option::Option<flags::permission::PermissionFlag> = std::option::Option::None;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut option_14: std::option::Option<flags::size::SizeFlag> = std::option::Option::Some(sizeflag_2);
    let mut option_15: std::option::Option<usize> = std::option::Option::None;
    let mut option_16: std::option::Option<bool> = std::option::Option::None;
    let mut recursion_2: crate::config_file::Recursion = crate::config_file::Recursion {enabled: option_16, depth: option_15};
    let mut option_17: std::option::Option<crate::config_file::Recursion> = std::option::Option::Some(recursion_2);
    let mut option_18: std::option::Option<flags::layout::Layout> = std::option::Option::None;
    let mut option_19: std::option::Option<bool> = std::option::Option::None;
    let mut option_20: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_21: std::option::Option<crate::config_file::Icons> = std::option::Option::None;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut option_22: std::option::Option<flags::display::Display> = std::option::Option::Some(display_3);
    let mut option_23: std::option::Option<bool> = std::option::Option::None;
    let mut option_24: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_25: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_26: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_2: bool = true;
    let mut option_27: std::option::Option<bool> = std::option::Option::Some(bool_2);
    let mut config_0: crate::config_file::Config = crate::config_file::Config {classic: option_27, blocks: option_26, color: option_25, date: option_24, dereference: option_23, display: option_22, icons: option_21, ignore_globs: option_20, indicators: option_19, layout: option_18, recursion: option_17, size: option_14, permission: option_13, sorting: option_12, no_symlink: option_11, total_size: option_10, symlink_arrow: option_9, hyperlink: option_8};
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_442() {
//    rusty_monitor::set_test_id(442);
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_0: &str = "security.SMACK64";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
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
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::from_config(config_0_ref_0);
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut u64_0: u64 = 1099511627776u64;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_885() {
//    rusty_monitor::set_test_id(885);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_290() {
//    rusty_monitor::set_test_id(290);
    let mut elem_0: color::Elem = crate::color::Elem::FileLarge;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4506() {
//    rusty_monitor::set_test_id(4506);
    let mut str_0: &str = "GDBjfxFk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut coloroption_1: flags::color::ColorOption = crate::flags::color::ColorOption::Auto;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_1, theme: themeoption_2};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_1: usize = 2usize;
    let mut bool_1: bool = true;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut coloroption_2: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_2, theme: themeoption_3};
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_2: usize = 360usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_2};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut themeoption_4: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_3: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_3: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_3, theme: themeoption_4};
    let mut u64_1: u64 = 1024u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Pipe;
    let mut option_0: std::option::Option<crate::config_file::Color> = std::option::Option::None;
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_2: std::option::Option<bool> = std::option::Option::None;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut color_1_ref_0: &crate::flags::color::Color = &mut color_1;
    let mut color_4: crate::flags::color::Color = crate::flags::color::Color::clone(color_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_315() {
//    rusty_monitor::set_test_id(315);
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut str_0: &str = "Special";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_str(str_0_ref_0);
    let mut color_0: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut color_0_ref_0: &crate::config_file::Color = &mut color_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_455() {
//    rusty_monitor::set_test_id(455);
    let mut u64_0: u64 = 1073741824u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_539() {
//    rusty_monitor::set_test_id(539);
    let mut usize_0: usize = 40usize;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut usize_1: usize = 80usize;
    let mut bool_3: bool = false;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_2: usize = 1usize;
    let mut bool_4: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_2};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Auto;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut u64_0: u64 = 1099511627776u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Version;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 6usize;
    let mut bool_5: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_5, depth: usize_3};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_1: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut u64_1: u64 = 0u64;
    let mut size_1: crate::meta::size::Size = crate::meta::size::Size::new(u64_1);
    let mut size_1_ref_0: &crate::meta::size::Size = &mut size_1;
    let mut bool_6: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_6};
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_4: usize = 6usize;
    let mut bool_7: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_7, depth: usize_4};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_5: usize = 1usize;
    let mut bool_8: bool = true;
    let mut recursion_3: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_8, depth: usize_5};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut color_3: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::None;
    let mut option_1: std::option::Option<flags::color::ColorOption> = std::option::Option::None;
    let mut color_4: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_4);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut bool_9: bool = false;
    let mut option_4: std::option::Option<bool> = std::option::Option::Some(bool_9);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut tuple_0: () = crate::flags::color::ThemeOption::assert_receiver_is_total_eq(themeoption_0_ref_0);
//    panic!("From RustyUnit with love");
}
}