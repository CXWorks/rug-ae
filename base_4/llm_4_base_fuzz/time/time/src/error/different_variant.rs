//! Different variant error
use core::fmt;
/// An error type indicating that a [`TryFrom`](core::convert::TryFrom) call failed because the
/// original value was of a different variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentVariant;
impl fmt::Display for DifferentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value was of a different variant than required")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for DifferentVariant {}
impl From<DifferentVariant> for crate::Error {
    fn from(err: DifferentVariant) -> Self {
        Self::DifferentVariant(err)
    }
}
impl TryFrom<crate::Error> for DifferentVariant {
    type Error = Self;
    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::DifferentVariant(err) => Ok(err),
            _ => Err(Self),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_80_llm_16_80 {
    use crate::error::{
        ComponentRange, ConversionRange, DifferentVariant, Error, InvalidVariant,
    };
    use std::convert::TryFrom;
    #[test]
    fn try_from_error_for_different_variant() {
        let _rug_st_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_different_variant = 0;
        let err = Error::DifferentVariant(DifferentVariant);
        debug_assert_eq!(DifferentVariant::try_from(err), Ok(DifferentVariant));
        let _rug_ed_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_different_variant = 0;
    }
    #[test]
    fn try_from_error_for_conversion_range() {
        let _rug_st_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_conversion_range = 0;
        let err = Error::ConversionRange(ConversionRange);
        debug_assert_eq!(DifferentVariant::try_from(err), Err(DifferentVariant));
        let _rug_ed_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_conversion_range = 0;
    }
    #[test]
    fn try_from_error_for_component_range() {
        let _rug_st_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_component_range = 0;
        let rug_fuzz_0 = "test_component";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 20;
        let rug_fuzz_4 = false;
        let err = Error::ComponentRange(ComponentRange {
            name: rug_fuzz_0,
            minimum: rug_fuzz_1,
            maximum: rug_fuzz_2,
            value: rug_fuzz_3,
            conditional_range: rug_fuzz_4,
        });
        debug_assert_eq!(DifferentVariant::try_from(err), Err(DifferentVariant));
        let _rug_ed_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_component_range = 0;
    }
    #[test]
    fn try_from_error_for_invalid_variant() {
        let _rug_st_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_invalid_variant = 0;
        let err = Error::InvalidVariant(InvalidVariant);
        debug_assert_eq!(DifferentVariant::try_from(err), Err(DifferentVariant));
        let _rug_ed_tests_llm_16_80_llm_16_80_rrrruuuugggg_try_from_error_for_invalid_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_330 {
    use super::*;
    use crate::*;
    use crate::error::{DifferentVariant, Error};
    #[test]
    fn from_different_variant() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let different_variant_err = DifferentVariant;
        let error: Error = Error::from(different_variant_err);
        match error {
            Error::DifferentVariant(dv) => {
                debug_assert!(rug_fuzz_0, "Correct variant wrapped in Error")
            }
            _ => debug_assert!(rug_fuzz_1, "Incorrect variant wrapped in Error"),
        }
             }
});    }
}
