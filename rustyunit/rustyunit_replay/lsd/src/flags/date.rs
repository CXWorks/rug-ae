//! This module defines the [DateFlag]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use its [configure_from](Configurable::configure_from) method.
use super::Configurable;
use crate::app;
use crate::config_file::Config;
use crate::print_error;
use clap::ArgMatches;
/// The flag showing which kind of time stamps to display.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFlag {
    Date,
    Relative,
    Iso,
    Formatted(String),
}
impl DateFlag {
    /// Get a value from a date format string
    fn from_format_string(value: &str) -> Option<Self> {
        match app::validate_time_format(value) {
            Ok(()) => Some(Self::Formatted(value[1..].to_string())),
            _ => {
                print_error!("Not a valid date format: {}.", value);
                None
            }
        }
    }
    /// Get a value from a str.
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "date" => Some(Self::Date),
            "relative" => Some(Self::Relative),
            _ if value.starts_with('+') => Self::from_format_string(value),
            _ => {
                print_error!("Not a valid date value: {}.", value);
                None
            }
        }
    }
}
impl Configurable<Self> for DateFlag {
    /// Get a potential `DateFlag` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [DateFlag::Date] variant in a
    /// [Some]. Otherwise if the argument is passed, this returns the variant corresponding to its
    /// parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Date)
        } else if matches.occurrences_of("date") > 0 {
            match matches.values_of("date")?.last() {
                Some("date") => Some(Self::Date),
                Some("relative") => Some(Self::Relative),
                Some(format) if format.starts_with('+') => {
                    Some(Self::Formatted(format[1..].to_owned()))
                }
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }
    /// Get a potential `DateFlag` variant from a [Config].
    ///
    /// If the `Config::classic` is `true` then this returns the Some(DateFlag::Date),
    /// Otherwise if the `Config::date` has value and is one of "date" or "relative",
    /// this returns its corresponding variant in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(true) = &config.classic {
            return Some(Self::Date);
        }
        if let Some(date) = &config.date { Self::from_str(date) } else { None }
    }
    /// Get a potential `DateFlag` variant from the environment.
    fn from_environment() -> Option<Self> {
        if let Ok(value) = std::env::var("TIME_STYLE") {
            match value.as_str() {
                "full-iso" => Some(Self::Formatted("%F %T.%f %z".into())),
                "long-iso" => Some(Self::Formatted("%F %R".into())),
                "iso" => Some(Self::Iso),
                _ if value.starts_with('+') => Self::from_format_string(&value),
                _ => {
                    print_error!("Not a valid date value: {}.", value);
                    None
                }
            }
        } else {
            None
        }
    }
}
/// The default value for `DateFlag` is [DateFlag::Date].
impl Default for DateFlag {
    fn default() -> Self {
        Self::Date
    }
}
#[cfg(test)]
mod test {
    use super::DateFlag;
    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;
    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, DateFlag::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_date() {
        let argv = vec!["lsd", "--date", "date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_relative() {
        let argv = vec!["lsd", "--date", "relative"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Relative), DateFlag::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_format() {
        let argv = vec!["lsd", "--date", "+%F"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())), DateFlag::from_arg_matches(&
            matches)
        );
    }
    #[test]
    #[should_panic(expected = "invalid format specifier: %J")]
    fn test_from_arg_matches_format_invalid() {
        let argv = vec!["lsd", "--date", "+%J"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        DateFlag::from_arg_matches(&matches);
    }
    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--date", "date", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_date_multi() {
        let argv = vec!["lsd", "--date", "relative", "--date", "date"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(DateFlag::Date), DateFlag::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_config_none() {
        assert_eq!(None, DateFlag::from_config(& Config::with_none()));
    }
    #[test]
    fn test_from_config_date() {
        let mut c = Config::with_none();
        c.date = Some("date".into());
        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(& c));
    }
    #[test]
    fn test_from_config_relative() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        assert_eq!(Some(DateFlag::Relative), DateFlag::from_config(& c));
    }
    #[test]
    fn test_from_config_format() {
        let mut c = Config::with_none();
        c.date = Some("+%F".into());
        assert_eq!(
            Some(DateFlag::Formatted("%F".to_string())), DateFlag::from_config(& c)
        );
    }
    #[test]
    fn test_from_config_format_invalid() {
        let mut c = Config::with_none();
        c.date = Some("+%J".into());
        assert_eq!(None, DateFlag::from_config(& c));
    }
    #[test]
    fn test_from_config_classic_mode() {
        let mut c = Config::with_none();
        c.date = Some("relative".into());
        c.classic = Some(true);
        assert_eq!(Some(DateFlag::Date), DateFlag::from_config(& c));
    }
    #[test]
    #[serial_test::serial]
    fn test_from_environment_none() {
        std::env::set_var("TIME_STYLE", "");
        assert_eq!(None, DateFlag::from_environment());
    }
    #[test]
    #[serial_test::serial]
    fn test_from_environment_full_iso() {
        std::env::set_var("TIME_STYLE", "full-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %T.%f %z".into())), DateFlag::from_environment()
        );
    }
    #[test]
    #[serial_test::serial]
    fn test_from_environment_long_iso() {
        std::env::set_var("TIME_STYLE", "long-iso");
        assert_eq!(
            Some(DateFlag::Formatted("%F %R".into())), DateFlag::from_environment()
        );
    }
    #[test]
    #[serial_test::serial]
    fn test_from_environment_iso() {
        std::env::set_var("TIME_STYLE", "iso");
        assert_eq!(Some(DateFlag::Iso), DateFlag::from_environment());
    }
    #[test]
    #[serial_test::serial]
    fn test_from_environment_format() {
        std::env::set_var("TIME_STYLE", "+%F");
        assert_eq!(Some(DateFlag::Formatted("%F".into())), DateFlag::from_environment());
    }
    #[test]
    #[serial_test::serial]
    fn test_parsing_order_arg() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = vec!["lsd", "--date", "+%F"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%F".into()), DateFlag::configure_from(& matches, &
            config)
        );
    }
    #[test]
    #[serial_test::serial]
    fn test_parsing_order_env() {
        std::env::set_var("TIME_STYLE", "+%R");
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%R".into()), DateFlag::configure_from(& matches, &
            config)
        );
    }
    #[test]
    #[serial_test::serial]
    fn test_parsing_order_config() {
        std::env::set_var("TIME_STYLE", "");
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut config = Config::with_none();
        config.date = Some("+%c".into());
        assert_eq!(
            DateFlag::Formatted("%c".into()), DateFlag::configure_from(& matches, &
            config)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    use config_file::Config;
    #[test]
    fn test_from_config_classic_true() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let config = Config {
            classic: Some(rug_fuzz_0),
            ..Config::with_none()
        };
        debug_assert_eq!(
            < flags::date::DateFlag as flags::Configurable < flags::date::DateFlag > >
            ::from_config(& config), Some(flags::date::DateFlag::Date)
        );
             }
}
}
}    }
    #[test]
    fn test_from_config_classic_false() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let config = Config {
            classic: Some(rug_fuzz_0),
            ..Config::with_none()
        };
        debug_assert_eq!(
            < flags::date::DateFlag as flags::Configurable < flags::date::DateFlag > >
            ::from_config(& config), None
        );
             }
}
}
}    }
    #[test]
    fn test_from_config_date_present() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let config = Config {
            classic: None,
            date: Some(rug_fuzz_0.to_string()),
            ..Config::with_none()
        };
        debug_assert_eq!(
            < flags::date::DateFlag as flags::Configurable < flags::date::DateFlag > >
            ::from_config(& config), Some(flags::date::DateFlag::Date)
        );
             }
}
}
}    }
    #[test]
    fn test_from_config_date_not_present() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_from_config_date_not_present = 0;
        let config = Config {
            classic: None,
            date: None,
            ..Config::with_none()
        };
        debug_assert_eq!(
            < flags::date::DateFlag as flags::Configurable < flags::date::DateFlag > >
            ::from_config(& config), None
        );
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_from_config_date_not_present = 0;
    }
    #[test]
    fn test_from_config_date_relative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let config = Config {
            classic: None,
            date: Some(rug_fuzz_0.to_string()),
            ..Config::with_none()
        };
        debug_assert_eq!(
            < flags::date::DateFlag as flags::Configurable < flags::date::DateFlag > >
            ::from_config(& config), Some(flags::date::DateFlag::Relative)
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_39 {
    use crate::flags::date::DateFlag;
    use crate::flags::Configurable;
    use clap::ArgMatches;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_39_rrrruuuugggg_test_default = 0;
        let default_date_flag = DateFlag::default();
        debug_assert_eq!(default_date_flag, DateFlag::Date);
        let _rug_ed_tests_llm_16_39_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_194 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_format_string_valid_format() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let result = DateFlag::from_format_string(value);
        debug_assert_eq!(result, Some(DateFlag::Formatted(value[1..].to_string())));
             }
}
}
}    }
    #[test]
    fn test_from_format_string_invalid_format() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let result = DateFlag::from_format_string(value);
        debug_assert_eq!(result, None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    #[test]
    fn test_from_str() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        crate::flags::date::DateFlag::from_str(&p0);
             }
}
}
}    }
}
