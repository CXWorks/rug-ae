use super::Configurable;
use crate::config_file::Config;
use clap::ArgMatches;
/// The flag showing how to display symbolic arrow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymlinkArrow(String);
impl Configurable<Self> for SymlinkArrow {
    /// `SymlinkArrow` can not be configured by [ArgMatches]
    ///
    /// Return `None`
    fn from_arg_matches(_: &ArgMatches) -> Option<Self> {
        None
    }
    /// Get a potential `SymlinkArrow` value from a [Config].
    ///
    /// If the `Config::symlink-arrow` has value,
    /// returns its value as the value of the `SymlinkArrow`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        config.symlink_arrow.as_ref().map(|arrow| SymlinkArrow(arrow.to_string()))
    }
}
/// The default value for the `SymlinkArrow` is `\u{21d2}(⇒)`
impl Default for SymlinkArrow {
    fn default() -> Self {
        Self(String::from("\u{21d2}"))
    }
}
use std::fmt;
impl fmt::Display for SymlinkArrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(test)]
mod test {
    use crate::config_file::Config;
    use crate::flags::Configurable;
    use super::SymlinkArrow;
    #[test]
    fn test_symlink_arrow_from_config_utf8() {
        let mut c = Config::with_none();
        c.symlink_arrow = Some("↹".into());
        assert_eq!(
            Some(SymlinkArrow(String::from("\u{21B9}"))), SymlinkArrow::from_config(& c)
        );
    }
    #[test]
    fn test_symlink_arrow_from_args_none() {
        use clap::App;
        assert_eq!(
            None, SymlinkArrow::from_arg_matches(& App::new("lsd").get_matches())
        );
    }
    #[test]
    fn test_symlink_arrow_default() {
        assert_eq!(SymlinkArrow(String::from("\u{21d2}")), SymlinkArrow::default());
    }
    #[test]
    fn test_symlink_display() {
        assert_eq!("⇒", format!("{}", SymlinkArrow::default()));
    }
}
#[cfg(test)]
mod tests_llm_16_105 {
    use super::*;
    use crate::*;
    use clap::ArgMatches;
    #[test]
    fn test_from_arg_matches() {
        let _rug_st_tests_llm_16_105_rrrruuuugggg_test_from_arg_matches = 0;
        let arg_matches: ArgMatches = ArgMatches::new();
        let result = <flags::symlink_arrow::SymlinkArrow as flags::Configurable<
            flags::symlink_arrow::SymlinkArrow,
        >>::from_arg_matches(&arg_matches);
        let expected = None;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_105_rrrruuuugggg_test_from_arg_matches = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_106 {
    use super::*;
    use crate::*;
    use config_file::Config;
    #[test]
    fn test_from_config_with_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut config = Config::default();
        config.symlink_arrow = Some(rug_fuzz_0.to_string());
        let result = crate::flags::symlink_arrow::SymlinkArrow::from_config(&config);
        debug_assert!(result.is_some());
        debug_assert_eq!(result.unwrap().0, "->".to_string());
             }
});    }
    #[test]
    fn test_from_config_with_none() {
        let _rug_st_tests_llm_16_106_rrrruuuugggg_test_from_config_with_none = 0;
        let config = Config::default();
        let result = crate::flags::symlink_arrow::SymlinkArrow::from_config(&config);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_106_rrrruuuugggg_test_from_config_with_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_107 {
    use super::*;
    use crate::*;
    use crate::flags::{Config, SymlinkArrow};
    #[test]
    fn test_default() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let expected = SymlinkArrow(String::from(rug_fuzz_0));
        let result = SymlinkArrow::default();
        debug_assert_eq!(result, expected);
             }
});    }
}
