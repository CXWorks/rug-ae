//! Component range error
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
    /// Whether the value's permitted range is conditional, i.e. whether an input with this
    /// value could have succeeded if the values of other components were different.
    pub const fn is_conditional(self) -> bool {
        self.conditional_range
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
mod tests_llm_16_78_llm_16_78 {
    use super::*;
    use crate::*;
    use crate::error::{ComponentRange, DifferentVariant, Error};
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_component_range_error() {
        let _rug_st_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_try_from_component_range_error = 0;
        let rug_fuzz_0 = "test_component";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 11;
        let rug_fuzz_4 = false;
        let component_range_error = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        let error = Error::from(component_range_error);
        if let Ok(result) = ComponentRange::try_from(error) {
            debug_assert_eq!(result.name(), "test_component");
            debug_assert_eq!(result.is_conditional(), false);
        } else {
            panic!("try_from should have parsed a ComponentRange error");
        }
        let _rug_ed_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_try_from_component_range_error = 0;
    }
    #[test]
    fn test_try_from_different_error_variant() {
        let _rug_st_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_try_from_different_error_variant = 0;
        let different_variant_error = DifferentVariant;
        let error = Error::from(different_variant_error);
        let result = ComponentRange::try_from(error);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_try_from_different_error_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_326 {
    use super::*;
    use crate::*;
    #[test]
    fn test_component_range_from() {
        let _rug_st_tests_llm_16_326_rrrruuuugggg_test_component_range_from = 0;
        let rug_fuzz_0 = "test_component";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 11;
        let rug_fuzz_4 = false;
        let component_range = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        let error = Error::from(component_range);
        match error {
            Error::ComponentRange(cr) => {
                debug_assert_eq!(cr.name(), "test_component");
                debug_assert_eq!(cr.is_conditional(), false);
            }
            _ => panic!("Error converted to incorrect variant"),
        }
        let _rug_ed_tests_llm_16_326_rrrruuuugggg_test_component_range_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_328 {
    use super::*;
    use crate::*;
    use crate::error::component_range::ComponentRange;
    #[test]
    fn test_component_range_name() {
        let _rug_st_tests_llm_16_328_rrrruuuugggg_test_component_range_name = 0;
        let rug_fuzz_0 = "year";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 9999;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = false;
        let component_range = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        debug_assert_eq!(component_range.name(), "year");
        let _rug_ed_tests_llm_16_328_rrrruuuugggg_test_component_range_name = 0;
    }
}
#[cfg(test)]
mod tests_rug_171 {
    use crate::error::component_range::ComponentRange;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_171_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "month";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = 13;
        let rug_fuzz_4 = false;
        let mut p0 = ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        };
        debug_assert_eq!(p0.is_conditional(), false);
        let _rug_ed_tests_rug_171_rrrruuuugggg_test_rug = 0;
    }
}
