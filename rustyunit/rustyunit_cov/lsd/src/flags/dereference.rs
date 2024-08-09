//! This module defines the [Dereference] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.
use super::Configurable;
use crate::config_file::Config;
use clap::ArgMatches;
/// The flag showing whether to dereference symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Dereference(pub bool);
impl Configurable<Self> for Dereference {
    /// Get a potential `Dereference` value from [ArgMatches].
    ///
    /// If the "dereference" argument is passed, this returns a `Dereference` with value `true` in
    /// a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("dereference") { Some(Self(true)) } else { None }
    }
    /// Get a potential `Dereference` value from a [Config].
    ///
    /// If the `Config::dereference` has value, this returns its value
    /// as the value of the `Dereference`, in a [Some], Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.dereference.as_ref().map(|deref| Self(*deref))
    }
}
#[cfg(test)]
mod test {
    use super::Dereference;
    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;
    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, Dereference::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_arg_matches_true() {
        let argv = vec!["lsd", "--dereference"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(Some(Dereference(true)), Dereference::from_arg_matches(& matches));
    }
    #[test]
    fn test_from_config_none() {
        assert_eq!(None, Dereference::from_config(& Config::with_none()));
    }
    #[test]
    fn test_from_config_true() {
        let mut c = Config::with_none();
        c.dereference = Some(true);
        assert_eq!(Some(Dereference(true)), Dereference::from_config(& c));
    }
    #[test]
    fn test_from_config_false() {
        let mut c = Config::with_none();
        c.dereference = Some(false);
        assert_eq!(Some(Dereference(false)), Dereference::from_config(& c));
    }
}
#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_config_with_dereference_value() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_from_config_with_dereference_value = 0;
        let rug_fuzz_0 = true;
        let mut config = Config::default();
        config.dereference = Some(rug_fuzz_0);
        let result = <Dereference as Configurable<Dereference>>::from_config(&config);
        debug_assert_eq!(result, Some(Dereference(true)));
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_from_config_with_dereference_value = 0;
    }
    #[test]
    fn test_from_config_without_dereference_value() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_from_config_without_dereference_value = 0;
        let config = Config::default();
        let result = <Dereference as Configurable<Dereference>>::from_config(&config);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_from_config_without_dereference_value = 0;
    }
}
