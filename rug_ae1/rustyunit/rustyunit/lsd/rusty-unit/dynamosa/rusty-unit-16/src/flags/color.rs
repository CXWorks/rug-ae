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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6813() {
    rusty_monitor::set_test_id(6813);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::Pipe;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Context;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_2};
    let mut bool_3: bool = true;
    let mut bool_4: bool = false;
    let mut bool_5: bool = false;
    let mut bool_6: bool = true;
    let mut bool_7: bool = true;
    let mut bool_8: bool = true;
    let mut bool_9: bool = false;
    let mut bool_10: bool = false;
    let mut bool_11: bool = true;
    let mut bool_12: bool = false;
    let mut bool_13: bool = false;
    let mut bool_14: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_14, user_write: bool_13, user_execute: bool_12, group_read: bool_11, group_write: bool_10, group_execute: bool_9, other_read: bool_8, other_write: bool_7, other_execute: bool_6, sticky: bool_5, setgid: bool_4, setuid: bool_3};
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut elem_2: color::Elem = crate::color::Elem::File {exec: bool_1, uid: bool_0};
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut bool_15: bool = crate::flags::color::ThemeOption::eq(themeoption_2_ref_0, themeoption_0_ref_0);
    let mut links_0: crate::color::theme::Links = crate::color::theme::Links {valid: color_1, invalid: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5877() {
    rusty_monitor::set_test_id(5877);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut str_0: &str = "M9xNsLfJW5ro";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Acl;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut elem_2: color::Elem = crate::color::Elem::Pipe;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut elem_3: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut elem_4: color::Elem = crate::color::Elem::Special;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut elem_5: color::Elem = crate::color::Elem::Write;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut elem_6: color::Elem = crate::color::Elem::DayOld;
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut elem_7: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_7_ref_0: &color::Elem = &mut elem_7;
    let mut elem_8: color::Elem = crate::color::Elem::Special;
    let mut elem_8_ref_0: &color::Elem = &mut elem_8;
    let mut elem_9: color::Elem = crate::color::Elem::Octal;
    let mut elem_9_ref_0: &color::Elem = &mut elem_9;
    let mut elem_10: color::Elem = crate::color::Elem::User;
    let mut elem_10_ref_0: &color::Elem = &mut elem_10;
    let mut elem_11: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_11_ref_0: &color::Elem = &mut elem_11;
    let mut elem_12: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_12_ref_0: &color::Elem = &mut elem_12;
    let mut elem_13: color::Elem = crate::color::Elem::Context;
    let mut elem_13_ref_0: &color::Elem = &mut elem_13;
    let mut elem_14: color::Elem = crate::color::Elem::MissingSymLinkTarget;
    let mut elem_14_ref_0: &color::Elem = &mut elem_14;
    let mut elem_15: color::Elem = crate::color::Elem::HourOld;
    let mut elem_15_ref_0: &color::Elem = &mut elem_15;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_16: color::Elem = crate::color::Elem::Context;
    let mut elem_16_ref_0: &color::Elem = &mut elem_16;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_16_ref_0, theme_0_ref_0);
    let mut elem_17: color::Elem = crate::color::Elem::SymLink;
    let mut str_1: &str = "1gA7LK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut str_2: &str = "Yui";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_0: std::option::Option<crate::color::theme::Theme> = crate::color::theme::Theme::from_path(str_1_ref_0);
    let mut elem_18: color::Elem = crate::color::Elem::DayOld;
    let mut elem_17_ref_0: &color::Elem = &mut elem_17;
    let mut elem_19: color::Elem = crate::color::Elem::NonFile;
    let mut elem_18_ref_0: &color::Elem = &mut elem_18;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut option_1: std::option::Option<lscolors::LsColors> = std::option::Option::None;
    let mut option_2: std::option::Option<crate::color::theme::Theme> = std::option::Option::None;
    let mut result_0: std::result::Result<(), std::string::String> = crate::app::validate_time_format(str_0_ref_0);
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut bool_0: bool = crate::flags::color::ThemeOption::ne(themeoption_2_ref_0, themeoption_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1126() {
    rusty_monitor::set_test_id(1126);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::None;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::clone(themeoption_1_ref_0);
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7237() {
    rusty_monitor::set_test_id(7237);
    let mut str_0: &str = "qwdI";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "XJuK9OhQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 799usize;
    let mut tuple_0: (usize, &str) = (usize_0, str_1_ref_0);
    let mut str_2: &str = "YOM";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut displayoption_0: meta::name::DisplayOption = crate::meta::name::DisplayOption::FileName;
    let mut displayoption_0_ref_0: &meta::name::DisplayOption = &mut displayoption_0;
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Name;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_1: usize = 8519usize;
    let mut bool_0: bool = false;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_1};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Relative;
    let mut date_0: meta::date::Date = crate::meta::date::Date::Invalid;
    let mut date_0_ref_0: &meta::date::Date = &mut date_0;
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_1, order: sortorder_1, dir_grouping: dirgrouping_1};
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 9962usize;
    let mut bool_1: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_2};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut hyperlinkoption_2: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_2: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Short;
    let mut usize_3: usize = 5510usize;
    let mut bool_2: bool = false;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_2, depth: usize_3};
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut u64_0: u64 = 1253u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut hyperlinkoption_3: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut bool_3: bool = true;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_3};
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut option_0: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_str(str_3_ref_0);
    let mut option_1: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::Permission;
    let mut block_0_ref_0: &flags::blocks::Block = &mut block_0;
    let mut display_3: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut display_3_ref_0: &flags::display::Display = &mut display_3;
    let mut elem_0: color::Elem = crate::color::Elem::BrokenSymLink;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut elem_1: color::Elem = crate::color::Elem::Exec;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_2: color::Elem = crate::color::Elem::NoAccess;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_0_ref_0);
    let mut bool_4: bool = false;
    let mut elem_3: color::Elem = crate::color::Elem::Dir {uid: bool_4};
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut display_4: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut display_4_ref_0: &flags::display::Display = &mut display_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1739() {
    rusty_monitor::set_test_id(1739);
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_1);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_2_ref_0: &flags::color::ThemeOption = &mut themeoption_2;
    let mut bool_13: bool = crate::flags::color::ThemeOption::eq(themeoption_2_ref_0, themeoption_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5324() {
    rusty_monitor::set_test_id(5324);
    let mut bool_0: bool = true;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Directory {uid: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_2: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut themeoption_3: flags::color::ThemeOption = crate::flags::color::ThemeOption::clone(themeoption_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_517() {
    rusty_monitor::set_test_id(517);
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Never;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::None;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_0, order: sortorder_0, dir_grouping: dirgrouping_0};
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut usize_0: usize = 8588usize;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Date;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::Default;
    let mut coloroption_0: flags::color::ColorOption = crate::flags::color::ColorOption::Never;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color {when: coloroption_0, theme: themeoption_0};
    let mut config_1: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_1_ref_0: &crate::config_file::Config = &mut config_1;
    let mut u64_0: u64 = 9069u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut usize_1: usize = 342usize;
    let mut bool_1: bool = false;
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::CharDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut elem_1: color::Elem = crate::color::Elem::DayOld;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_1, depth: usize_1};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut layout_1_ref_0: &flags::layout::Layout = &mut layout_1;
    let mut color_0_ref_0: &crate::flags::color::Color = &mut color_0;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color::clone(color_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2400() {
    rusty_monitor::set_test_id(2400);
    let mut usize_0: usize = 1372usize;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut dirgrouping_0: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_0: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_0: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut permissionflag_0: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_0: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut bool_0: bool = true;
    let mut recursion_0: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_0, depth: usize_0};
    let mut layout_0: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_0: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_0: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_0: flags::display::Display = crate::flags::display::Display::AlmostAll;
    let mut dateflag_0: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_0: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::Special;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut filetype_1: meta::filetype::FileType = crate::meta::filetype::FileType::File {uid: bool_2, exec: bool_1};
    let mut dirgrouping_1: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::None;
    let mut sortorder_1: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Default;
    let mut sortcolumn_1: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Extension;
    let mut permissionflag_1: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_1: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_1: usize = 1924usize;
    let mut bool_3: bool = false;
    let mut recursion_1: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_3, depth: usize_1};
    let mut layout_1: flags::layout::Layout = crate::flags::layout::Layout::Grid;
    let mut icontheme_1: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_1: flags::icons::IconOption = crate::flags::icons::IconOption::Always;
    let mut display_1: flags::display::Display = crate::flags::display::Display::DirectoryOnly;
    let mut dateflag_1: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_1: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut hyperlinkoption_0: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Auto;
    let mut dirgrouping_2: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::First;
    let mut sortorder_2: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_2: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Time;
    let mut sorting_0: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_2, order: sortorder_2, dir_grouping: dirgrouping_2};
    let mut permissionflag_2: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Octal;
    let mut sizeflag_2: flags::size::SizeFlag = crate::flags::size::SizeFlag::Default;
    let mut layout_2: flags::layout::Layout = crate::flags::layout::Layout::OneLine;
    let mut icontheme_2: flags::icons::IconTheme = crate::flags::icons::IconTheme::Fancy;
    let mut iconoption_2: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_2: flags::display::Display = crate::flags::display::Display::All;
    let mut dateflag_2: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_2: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut hyperlinkoption_1: flags::hyperlink::HyperlinkOption = crate::flags::hyperlink::HyperlinkOption::Always;
    let mut dirgrouping_3: flags::sorting::DirGrouping = crate::flags::sorting::DirGrouping::Last;
    let mut sortorder_3: flags::sorting::SortOrder = crate::flags::sorting::SortOrder::Reverse;
    let mut sortcolumn_3: flags::sorting::SortColumn = crate::flags::sorting::SortColumn::Size;
    let mut sorting_1: crate::flags::sorting::Sorting = crate::flags::sorting::Sorting {column: sortcolumn_3, order: sortorder_3, dir_grouping: dirgrouping_3};
    let mut permissionflag_3: flags::permission::PermissionFlag = crate::flags::permission::PermissionFlag::Rwx;
    let mut sizeflag_3: flags::size::SizeFlag = crate::flags::size::SizeFlag::Bytes;
    let mut usize_2: usize = 5689usize;
    let mut bool_4: bool = true;
    let mut recursion_2: crate::flags::recursion::Recursion = crate::flags::recursion::Recursion {enabled: bool_4, depth: usize_2};
    let mut layout_3: flags::layout::Layout = crate::flags::layout::Layout::Tree;
    let mut icontheme_3: flags::icons::IconTheme = crate::flags::icons::IconTheme::Unicode;
    let mut iconoption_3: flags::icons::IconOption = crate::flags::icons::IconOption::Never;
    let mut display_3: flags::display::Display = crate::flags::display::Display::VisibleOnly;
    let mut dateflag_3: flags::date::DateFlag = crate::flags::date::DateFlag::Iso;
    let mut color_3: crate::flags::color::Color = crate::flags::color::Color::default();
    let mut u64_0: u64 = 2876u64;
    let mut size_0: crate::meta::size::Size = crate::meta::size::Size::new(u64_0);
    let mut size_0_ref_0: &crate::meta::size::Size = &mut size_0;
    let mut option_0: std::option::Option<flags::color::ThemeOption> = std::option::Option::Some(themeoption_0);
    let mut option_1: std::option::Option<flags::color::ColorOption> = crate::flags::color::ColorOption::from_environment();
    let mut color_4: crate::config_file::Color = crate::config_file::Color {when: option_1, theme: option_0};
    let mut option_2: std::option::Option<crate::config_file::Color> = std::option::Option::Some(color_4);
    let mut option_3: std::option::Option<std::vec::Vec<std::string::String>> = std::option::Option::None;
    let mut option_4: std::option::Option<bool> = std::option::Option::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut theme_0: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_0_ref_0: &crate::color::theme::Theme = &mut theme_0;
    let mut elem_0: color::Elem = crate::color::Elem::BlockDevice;
    let mut elem_0_ref_0: &color::Elem = &mut elem_0;
    let mut color_0: crossterm::style::Color = crate::color::Elem::get_color(elem_0_ref_0, theme_0_ref_0);
    let mut theme_1: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_1_ref_0: &crate::color::theme::Theme = &mut theme_1;
    let mut elem_1: color::Elem = crate::color::Elem::Socket;
    let mut elem_1_ref_0: &color::Elem = &mut elem_1;
    let mut color_1: crossterm::style::Color = crate::color::Elem::get_color(elem_1_ref_0, theme_1_ref_0);
    let mut theme_2: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_2_ref_0: &crate::color::theme::Theme = &mut theme_2;
    let mut elem_2: color::Elem = crate::color::Elem::Older;
    let mut elem_2_ref_0: &color::Elem = &mut elem_2;
    let mut color_2: crossterm::style::Color = crate::color::Elem::get_color(elem_2_ref_0, theme_2_ref_0);
    let mut elem_3: color::Elem = crate::color::Elem::Write;
    let mut elem_3_ref_0: &color::Elem = &mut elem_3;
    let mut theme_3: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_3_ref_0: &crate::color::theme::Theme = &mut theme_3;
    let mut elem_4: color::Elem = crate::color::Elem::DayOld;
    let mut elem_4_ref_0: &color::Elem = &mut elem_4;
    let mut color_3: crossterm::style::Color = crate::color::Elem::get_color(elem_4_ref_0, theme_3_ref_0);
    let mut theme_4: crate::color::theme::Theme = crate::color::theme::Theme::default_dark();
    let mut theme_4_ref_0: &crate::color::theme::Theme = &mut theme_4;
    let mut elem_5: color::Elem = crate::color::Elem::Group;
    let mut elem_5_ref_0: &color::Elem = &mut elem_5;
    let mut color_4: crossterm::style::Color = crate::color::Elem::get_color(elem_5_ref_0, theme_4_ref_0);
    let mut bool_0: bool = true;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_0_ref_0: &flags::color::ThemeOption = &mut themeoption_0;
    let mut tuple_0: () = crate::flags::color::ThemeOption::assert_receiver_is_total_eq(themeoption_0_ref_0);
    let mut elem_6: color::Elem = crate::color::Elem::INode {valid: bool_0};
    let mut block_0: flags::blocks::Block = crate::flags::blocks::Block::User;
    let mut inode_0: crate::color::theme::INode = crate::color::theme::INode {valid: color_4, invalid: color_3};
    let mut elem_6_ref_0: &color::Elem = &mut elem_6;
    let mut app_0: clap::App = crate::app::build();
    let mut symlink_0: crate::color::theme::Symlink = crate::color::theme::Symlink {default: color_2, broken: color_1, missing_target: color_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2258() {
    rusty_monitor::set_test_id(2258);
    let mut config_0: crate::config_file::Config = crate::config_file::Config::with_none();
    let mut config_0_ref_0: &crate::config_file::Config = &mut config_0;
    let mut themeoption_0: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoLscolors;
    let mut colors_0: crate::color::Colors = crate::color::Colors::new(themeoption_0);
    let mut colors_0_ref_0: &crate::color::Colors = &mut colors_0;
    let mut bool_0: bool = false;
    let mut filetype_0: meta::filetype::FileType = crate::meta::filetype::FileType::SymLink {is_dir: bool_0};
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut bool_5: bool = true;
    let mut bool_6: bool = true;
    let mut bool_7: bool = false;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut bool_10: bool = false;
    let mut bool_11: bool = false;
    let mut bool_12: bool = false;
    let mut permissions_0: crate::meta::permissions::Permissions = crate::meta::permissions::Permissions {user_read: bool_12, user_write: bool_11, user_execute: bool_10, group_read: bool_9, group_write: bool_8, group_execute: bool_7, other_read: bool_6, other_write: bool_5, other_execute: bool_4, sticky: bool_3, setgid: bool_2, setuid: bool_1};
    let mut option_0: std::option::Option<std::path::PathBuf> = crate::config_file::Config::config_file_path();
    let mut permissions_0_ref_0: &crate::meta::permissions::Permissions = &mut permissions_0;
    let mut pathbuf_0: std::path::PathBuf = std::option::Option::unwrap(option_0);
    crate::meta::filetype::FileType::render(filetype_0, colors_0_ref_0);
    let mut themeoption_1: flags::color::ThemeOption = crate::flags::color::ThemeOption::NoColor;
    let mut themeoption_1_ref_0: &flags::color::ThemeOption = &mut themeoption_1;
    let mut tuple_0: () = crate::flags::color::ThemeOption::assert_receiver_is_total_eq(themeoption_1_ref_0);
    panic!("From RustyUnit with love");
}
}