//! Conversion range error
use core::fmt;
use crate::error;
/// An error type indicating that a conversion failed because the target type could not store the
/// initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange;
impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for ConversionRange {}
impl From<ConversionRange> for crate::Error {
    fn from(err: ConversionRange) -> Self {
        Self::ConversionRange(err)
    }
}
impl TryFrom<crate::Error> for ConversionRange {
    type Error = error::DifferentVariant;
    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ConversionRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use super::*;
    use crate::*;
    use crate::error::conversion_range::ConversionRange;
    use crate::error::different_variant::DifferentVariant;
    use crate::error::Error;
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_for_conversion_range() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_try_from_for_conversion_range = 0;
        let conversion_range_error = ConversionRange;
        let error: Error = Error::ConversionRange(conversion_range_error);
        let result = ConversionRange::try_from(error);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_try_from_for_conversion_range = 0;
    }
    #[test]
    fn test_try_from_for_different_variant() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_try_from_for_different_variant = 0;
        let different_variant_error = DifferentVariant;
        let error: Error = Error::DifferentVariant(different_variant_error);
        let result = ConversionRange::try_from(error);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_try_from_for_different_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_329 {
    use super::*;
    use crate::*;
    use crate::error::conversion_range::ConversionRange;
    use crate::error::Error;
    #[test]
    fn test_conversion_range_from() {
        let _rug_st_tests_llm_16_329_rrrruuuugggg_test_conversion_range_from = 0;
        let conversion_range_error = ConversionRange;
        let error: Error = conversion_range_error.into();
        match error {
            Error::ConversionRange(_) => {}
            _ => panic!("Error::from did not convert to Error::ConversionRange"),
        }
        let _rug_ed_tests_llm_16_329_rrrruuuugggg_test_conversion_range_from = 0;
    }
}
