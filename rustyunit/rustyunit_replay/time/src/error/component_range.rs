//! Component range error
use core::convert::TryFrom;
use core::fmt;
use crate::error;
/// An error type indicating that a component provided to a method was out of range, causing a
/// failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub(crate) name: &'static str,
    /// Minimum allowed value, inclusive.
    pub(crate) minimum: i64,
    /// Maximum allowed value, inclusive.
    pub(crate) maximum: i64,
    /// Value that was provided.
    pub(crate) value: i64,
    /// The minimum and/or maximum value is conditional on the value of other
    /// parameters.
    pub(crate) conditional_range: bool,
}
impl ComponentRange {
    /// Obtain the name of the component whose value was out of range.
    pub const fn name(self) -> &'static str {
        self.name
    }
}
impl fmt::Display for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{} must be in the range {}..={}", self.name, self.minimum, self.maximum
        )?;
        if self.conditional_range {
            f.write_str(", given values of other parameters")?;
        }
        Ok(())
    }
}
impl From<ComponentRange> for crate::Error {
    fn from(original: ComponentRange) -> Self {
        Self::ComponentRange(original)
    }
}
impl TryFrom<crate::Error> for ComponentRange {
    type Error = error::DifferentVariant;
    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ComponentRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}
/// **This trait implementation is deprecated and will be removed in a future breaking release.**
#[cfg(feature = "serde")]
impl serde::de::Expected for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a value in the range {}..={}", self.minimum, self.maximum)
    }
}
#[cfg(feature = "serde")]
impl ComponentRange {
    /// Convert the error to a deserialization error.
    pub(crate) fn into_de_error<E: serde::de::Error>(self) -> E {
        E::invalid_value(serde::de::Unexpected::Signed(self.value), &self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ComponentRange {}
#[cfg(test)]
mod tests_llm_16_337 {
    use super::*;
    use crate::*;
    #[test]
    fn test_name() {
        let _rug_st_tests_llm_16_337_rrrruuuugggg_test_name = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = false;
        let component_range = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        debug_assert_eq!(component_range.name(), "example");
        let _rug_ed_tests_llm_16_337_rrrruuuugggg_test_name = 0;
    }
}
#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::error::component_range::ComponentRange;
    use crate::error::Error;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_122_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = false;
        let mut p0 = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        let p0 = Error::ComponentRange(p0);
        let result = <Error>::from(p0);
        let _rug_ed_tests_rug_122_rrrruuuugggg_test_rug = 0;
    }
}
