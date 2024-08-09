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
                    & value
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
        if let Some(c) = &config.color { c.when } else { None }
    }
    fn from_environment() -> Option<Self> {
        if env::var("NO_COLOR").is_ok() { Some(Self::Never) } else { None }
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
        assert_eq!(None, ColorOption::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--color", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(ColorOption::Always), ColorOption::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_auto() {
        let argv = vec!["lsd", "--color", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(ColorOption::Never), ColorOption::from_arg_matches(& matches));
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
        assert_eq!(Some(ColorOption::Never), ColorOption::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_color_multiple() {
        let argv = vec!["lsd", "--color", "always", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(ColorOption::Never), ColorOption::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_config_none() {
        assert_eq!(None, ColorOption::from_config(& Config::with_none()));
    }
    #[test]
    fn test_from_config_always() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Always), ColorOption::from_config(& c));
    }
    #[test]
    fn test_from_config_auto() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: Some(ColorOption::Auto),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Auto), ColorOption::from_config(& c));
    }
    #[test]
    fn test_from_config_never() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: Some(ColorOption::Never),
            theme: None,
        });
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(& c));
    }
    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: Some(ColorOption::Always),
            theme: None,
        });
        c.classic = Some(true);
        assert_eq!(Some(ColorOption::Never), ColorOption::from_config(& c));
    }
}
#[cfg(test)]
mod test_theme_option {
    use super::ThemeOption;
    use crate::config_file::{self, Config};
    #[test]
    fn test_from_config_none_default() {
        assert_eq!(
            ThemeOption::Default, ThemeOption::from_config(& Config::with_none())
        );
    }
    #[test]
    fn test_from_config_default() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });
        assert_eq!(ThemeOption::Default, ThemeOption::from_config(& c));
    }
    #[test]
    fn test_from_config_no_color() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoColor),
        });
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(& c));
    }
    #[test]
    fn test_from_config_no_lscolor() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::NoLscolors),
        });
        assert_eq!(ThemeOption::NoLscolors, ThemeOption::from_config(& c));
    }
    #[test]
    fn test_from_config_bad_file_flag() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Custom("not-existed".to_string())),
        });
        assert_eq!(
            ThemeOption::Custom("not-existed".to_string()), ThemeOption::from_config(& c)
        );
    }
    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c
            .color = Some(config_file::Color {
            when: None,
            theme: Some(ThemeOption::Default),
        });
        c.classic = Some(true);
        assert_eq!(ThemeOption::NoColor, ThemeOption::from_config(& c));
    }
}
#[cfg(test)]
mod tests_llm_16_28 {
    use super::*;
    use crate::*;
    use std::env;
    #[test]
    fn test_from_environment() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        env::remove_var(rug_fuzz_0);
        let result = <flags::color::ColorOption as flags::Configurable<
            flags::color::ColorOption,
        >>::from_environment();
        debug_assert_eq!(result, None);
        env::set_var(rug_fuzz_1, rug_fuzz_2);
        let result = <flags::color::ColorOption as flags::Configurable<
            flags::color::ColorOption,
        >>::from_environment();
        debug_assert_eq!(result, Some(ColorOption::Never));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_29_rrrruuuugggg_test_default = 0;
        let default_color = <ColorOption as Default>::default();
        debug_assert_eq!(default_color, ColorOption::Auto);
        let _rug_ed_tests_llm_16_29_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_default = 0;
        let result = <flags::color::ThemeOption as Default>::default();
        debug_assert_eq!(result, flags::color::ThemeOption::Default);
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_191 {
    use super::*;
    use crate::*;
    use clap::ArgMatches;
    #[test]
    fn test_configure_from() {
        let _rug_st_tests_llm_16_191_rrrruuuugggg_test_configure_from = 0;
        let matches = ArgMatches::new();
        let config = Config::default();
        let result = Color::configure_from(&matches, &config);
        debug_assert_eq!(result.when, ColorOption::Auto);
        debug_assert_eq!(result.theme, ThemeOption::Default);
        let _rug_ed_tests_llm_16_191_rrrruuuugggg_test_configure_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_192 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_str_always() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(ColorOption::from_str(rug_fuzz_0), Some(ColorOption::Always));
             }
});    }
    #[test]
    fn test_from_str_auto() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(ColorOption::from_str(rug_fuzz_0), Some(ColorOption::Auto));
             }
});    }
    #[test]
    fn test_from_str_never() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(ColorOption::from_str(rug_fuzz_0), Some(ColorOption::Never));
             }
});    }
    #[test]
    fn test_from_str_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(ColorOption::from_str(rug_fuzz_0).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_193 {
    use super::*;
    use crate::*;
    use config_file::Config;
    use flags::color::ThemeOption;
    #[test]
    fn test_from_config() {
        let _rug_st_tests_llm_16_193_rrrruuuugggg_test_from_config = 0;
        let config = Config::with_none();
        let result = ThemeOption::from_config(&config);
        debug_assert_eq!(result, ThemeOption::default());
        let _rug_ed_tests_llm_16_193_rrrruuuugggg_test_from_config = 0;
    }
}
