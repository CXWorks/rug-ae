//! Invalid variant error
use core::fmt;
/// An error type indicating that a [`FromStr`](core::str::FromStr) call failed because the value
/// was not a valid variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidVariant;
impl fmt::Display for InvalidVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was not a valid variant")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for InvalidVariant {}
impl From<InvalidVariant> for crate::Error {
    fn from(err: InvalidVariant) -> Self {
        Self::InvalidVariant(err)
    }
}
impl TryFrom<crate::Error> for InvalidVariant {
    type Error = crate::error::DifferentVariant;
    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::InvalidVariant(err) => Ok(err),
            _ => Err(crate::error::DifferentVariant),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_81 {
    use super::*;
    use crate::*;
    use crate::error::{
        ComponentRange, ConversionRange, DifferentVariant, InvalidVariant,
    };
    #[test]
    fn test_try_from_invalid_variant_error() {
        let _rug_st_tests_llm_16_81_rrrruuuugggg_test_try_from_invalid_variant_error = 0;
        let error = crate::Error::InvalidVariant(InvalidVariant);
        let result = InvalidVariant::try_from(error);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_81_rrrruuuugggg_test_try_from_invalid_variant_error = 0;
    }
    #[test]
    fn test_try_from_different_error_variant() {
        let _rug_st_tests_llm_16_81_rrrruuuugggg_test_try_from_different_error_variant = 0;
        let rug_fuzz_0 = "minute";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 60;
        let rug_fuzz_4 = false;
        let error = crate::Error::ComponentRange(ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        });
        let result = InvalidVariant::try_from(error);
        debug_assert!(matches!(result, Err(DifferentVariant)));
        let error = crate::Error::ConversionRange(ConversionRange);
        let result = InvalidVariant::try_from(error);
        debug_assert!(matches!(result, Err(DifferentVariant)));
        let error = crate::Error::DifferentVariant(DifferentVariant);
        let result = InvalidVariant::try_from(error);
        debug_assert!(matches!(result, Err(DifferentVariant)));
        let _rug_ed_tests_llm_16_81_rrrruuuugggg_test_try_from_different_error_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_331 {
    use super::*;
    use crate::*;
    use crate::error::Error;
    use crate::error::invalid_variant::InvalidVariant;
    #[test]
    fn test_from_invalid_variant() {
        let _rug_st_tests_llm_16_331_rrrruuuugggg_test_from_invalid_variant = 0;
        let invalid_variant = InvalidVariant;
        let error: Error = Error::from(invalid_variant);
        match error {
            Error::InvalidVariant(_) => {}
            _ => panic!("Error::from did not convert to Error::InvalidVariant variant"),
        }
        let _rug_ed_tests_llm_16_331_rrrruuuugggg_test_from_invalid_variant = 0;
    }
}
