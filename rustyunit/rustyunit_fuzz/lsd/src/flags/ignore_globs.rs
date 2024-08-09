//! This module defines the [IgnoreGlobs]. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](IgnoreGlobs::configure_from) method.
use crate::config_file::Config;
use clap::{ArgMatches, Error, ErrorKind};
use globset::{Glob, GlobSet, GlobSetBuilder};
/// The struct holding a [GlobSet] and methods to build it.
#[derive(Clone, Debug)]
pub struct IgnoreGlobs(pub GlobSet);
impl IgnoreGlobs {
    /// Returns a value from either [ArgMatches], a [Config] or a [Default] value. The first value
    /// that is not [None] is used. The order of precedence for the value used is:
    /// - [from_arg_matches](IgnoreGlobs::from_arg_matches)
    /// - [from_config](IgnoreGlobs::from_config)
    /// - [Default::default]
    ///
    /// # Errors
    ///
    /// If either of the [Glob::new] or [GlobSetBuilder.build] methods return an [Err].
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Result<Self, Error> {
        let mut result: Result<Self, Error> = Ok(Default::default());
        if !matches.is_present("ignore-config") {
            if let Some(value) = Self::from_config(config) {
                match value {
                    Ok(glob_set) => result = Ok(Self(glob_set)),
                    Err(err) => result = Err(err),
                }
            }
        }
        if let Some(value) = Self::from_arg_matches(matches) {
            match value {
                Ok(glob_set) => result = Ok(Self(glob_set)),
                Err(err) => result = Err(err),
            }
        }
        result
    }
    /// Get a potential [GlobSet] from [ArgMatches].
    ///
    /// If the "ignore-glob" argument has been passed, this returns a [Result] in a [Some] with
    /// either the built [GlobSet] or an [Error], if any error was encountered while creating the
    /// [GlobSet]. If the argument has not been passed, this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Result<GlobSet, Error>> {
        if matches.occurrences_of("ignore-glob") > 0 {
            if let Some(values) = matches.values_of("ignore-glob") {
                let mut glob_set_builder = GlobSetBuilder::new();
                for value in values {
                    match Self::create_glob(value) {
                        Ok(glob) => {
                            glob_set_builder.add(glob);
                        }
                        Err(err) => return Some(Err(err)),
                    }
                }
                Some(Self::create_glob_set(&glob_set_builder))
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Get a potential [GlobSet] from a [Config].
    ///
    /// If the `Config::ignore-globs` contains an Array of Strings,
    /// each of its values is used to build the [GlobSet]. If the building
    /// succeeds, the [GlobSet] is returned in the [Result] in a [Some]. If any error is
    /// encountered while building, an [Error] is returned in the Result instead. If the Config does
    /// not contain such a key, this returns [None].
    fn from_config(config: &Config) -> Option<Result<GlobSet, Error>> {
        if let Some(globs) = &config.ignore_globs {
            let mut glob_set_builder = GlobSetBuilder::new();
            for glob in globs.iter() {
                match Self::create_glob(glob) {
                    Ok(glob) => {
                        glob_set_builder.add(glob);
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
            Some(Self::create_glob_set(&glob_set_builder))
        } else {
            None
        }
    }
    /// Create a [Glob] from a provided pattern.
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob(pattern: &str) -> Result<Glob, Error> {
        match Glob::new(pattern) {
            Ok(glob) => Ok(glob),
            Err(err) => {
                Err(
                    Error::with_description(&err.to_string(), ErrorKind::ValueValidation),
                )
            }
        }
    }
    /// Create a [GlobSet] from a provided [GlobSetBuilder].
    ///
    /// This method is mainly a helper to wrap the handling of potential errors.
    fn create_glob_set(builder: &GlobSetBuilder) -> Result<GlobSet, Error> {
        match builder.build() {
            Ok(glob_set) => Ok(glob_set),
            Err(err) => {
                Err(
                    Error::with_description(&err.to_string(), ErrorKind::ValueValidation),
                )
            }
        }
    }
}
/// The default value of `IgnoreGlobs` is the empty [GlobSet], returned by [GlobSet::empty()].
impl Default for IgnoreGlobs {
    fn default() -> Self {
        Self(GlobSet::empty())
    }
}
#[cfg(test)]
mod test {
    use super::IgnoreGlobs;
    use crate::app;
    use crate::config_file::Config;
    #[test]
    fn test_configuration_from_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(
            match IgnoreGlobs::configure_from(& matches, & Config::with_none()) { Ok(_)
            => true, _ => false, }
        );
    }
    #[test]
    fn test_configuration_from_args() {
        let argv = vec!["lsd", "--ignore-glob", ".git"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(
            match IgnoreGlobs::configure_from(& matches, & Config::with_none()) { Ok(_)
            => true, _ => false, }
        );
    }
    #[test]
    fn test_configuration_from_config() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        let mut c = Config::with_none();
        c.ignore_globs = Some(vec![".git".into()].into());
        assert!(
            match IgnoreGlobs::configure_from(& matches, & c) { Ok(_) => true, _ =>
            false, }
        );
    }
    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert!(
            match IgnoreGlobs::from_arg_matches(& matches) { None => true, _ => false, }
        );
    }
    #[test]
    fn test_from_config_none() {
        assert!(
            match IgnoreGlobs::from_config(& Config::with_none()) { None => true, _ =>
            false, }
        );
    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_default = 0;
        let ignore_globs = IgnoreGlobs::default();
        debug_assert_eq!(ignore_globs.0.len(), 0);
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use clap::ArgMatches;
    use crate::config_file::Config;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_60_rrrruuuugggg_test_rug = 0;
        let mut p0: ArgMatches<'static> = ArgMatches::default();
        let p1 = Config::default();
        crate::flags::ignore_globs::IgnoreGlobs::configure_from(&p0, &p1);
        let _rug_ed_tests_rug_60_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use clap::ArgMatches;
    #[test]
    fn test_from_arg_matches() {
        let _rug_st_tests_rug_61_rrrruuuugggg_test_from_arg_matches = 0;
        let mut p0: ArgMatches<'static> = ArgMatches::default();
        crate::flags::ignore_globs::IgnoreGlobs::from_arg_matches(&p0);
        let _rug_ed_tests_rug_61_rrrruuuugggg_test_from_arg_matches = 0;
    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::config_file::Config;
    #[test]
    fn test_from_config() {
        let _rug_st_tests_rug_62_rrrruuuugggg_test_from_config = 0;
        let mut p0 = Config::default();
        crate::flags::ignore_globs::IgnoreGlobs::from_config(&p0);
        let _rug_ed_tests_rug_62_rrrruuuugggg_test_from_config = 0;
    }
}
#[cfg(test)]
mod tests_rug_63 {
    use super::super::IgnoreGlobs;
    #[test]
    fn test_create_glob() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        IgnoreGlobs::create_glob(&p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use globset::GlobSetBuilder;
    #[test]
    fn test_create_glob_set() {
        let _rug_st_tests_rug_64_rrrruuuugggg_test_create_glob_set = 0;
        let mut p0 = GlobSetBuilder::new();
        crate::flags::ignore_globs::IgnoreGlobs::create_glob_set(&p0);
        let _rug_ed_tests_rug_64_rrrruuuugggg_test_create_glob_set = 0;
    }
}
