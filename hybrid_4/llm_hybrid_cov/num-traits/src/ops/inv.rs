/// Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.
pub trait Inv {
    /// The result after applying the operator.
    type Output;
    /// Returns the multiplicative inverse of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::INFINITY;
    /// use num_traits::Inv;
    ///
    /// assert_eq!(7.0.inv() * 7.0, 1.0);
    /// assert_eq!((-0.0).inv(), -INFINITY);
    /// ```
    fn inv(self) -> Self::Output;
}
impl Inv for f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 {
        1.0 / self
    }
}
impl Inv for f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 {
        1.0 / self
    }
}
impl<'a> Inv for &'a f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 {
        1.0 / *self
    }
}
impl<'a> Inv for &'a f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 {
        1.0 / *self
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    #[test]
    fn test_inv_f32() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_inv_f32 = 0;
        let rug_fuzz_0 = 2.0f32;
        let val = rug_fuzz_0;
        let result = <&f32 as ops::inv::Inv>::inv(&val);
        debug_assert_eq!(result, 0.5f32);
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_inv_f32 = 0;
    }
    #[test]
    #[should_panic]
    fn test_inv_f32_panic_on_zero() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_inv_f32_panic_on_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let val = rug_fuzz_0;
        let _ = <&f32 as ops::inv::Inv>::inv(&val);
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_inv_f32_panic_on_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv_f64() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_inv_f64 = 0;
        let rug_fuzz_0 = 2.0f64;
        let rug_fuzz_1 = 0.5f64;
        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let result = Inv::inv(&value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_inv_f64 = 0;
    }
    #[test]
    #[should_panic(expected = "attempted to divide by zero")]
    fn test_inv_f64_panic() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_inv_f64_panic = 0;
        let rug_fuzz_0 = 0.0f64;
        let zero = rug_fuzz_0;
        let _ = Inv::inv(&zero);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_inv_f64_panic = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_423_llm_16_423 {
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv_f32() {
        let _rug_st_tests_llm_16_423_llm_16_423_rrrruuuugggg_test_inv_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.5;
        let value: f32 = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(< f32 as Inv > ::inv(value), expected);
        let _rug_ed_tests_llm_16_423_llm_16_423_rrrruuuugggg_test_inv_f32 = 0;
    }
    #[test]
    #[should_panic(expected = "attempted to divide by zero")]
    fn test_inv_f32_zero() {
        let _rug_st_tests_llm_16_423_llm_16_423_rrrruuuugggg_test_inv_f32_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f32 = rug_fuzz_0;
        let _result = <f32 as Inv>::inv(value);
        let _rug_ed_tests_llm_16_423_llm_16_423_rrrruuuugggg_test_inv_f32_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_591_llm_16_591 {
    use super::*;
    use crate::*;
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv() {
        let _rug_st_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_inv = 0;
        let rug_fuzz_0 = 2.0f64;
        let rug_fuzz_1 = 0.5f64;
        let rug_fuzz_2 = 2.0f64;
        let rug_fuzz_3 = 0.5f64;
        let rug_fuzz_4 = 1e-300f64;
        let rug_fuzz_5 = 1e300f64;
        let rug_fuzz_6 = 0.0f64;
        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(< f64 as Inv > ::inv(value), expected);
        let value = -rug_fuzz_2;
        let expected = -rug_fuzz_3;
        debug_assert_eq!(< f64 as Inv > ::inv(value), expected);
        let value = rug_fuzz_4;
        let expected = rug_fuzz_5;
        debug_assert_eq!(< f64 as Inv > ::inv(value), expected);
        let value = rug_fuzz_6;
        debug_assert!(< f64 as Inv > ::inv(value).is_infinite());
        let _rug_ed_tests_llm_16_591_llm_16_591_rrrruuuugggg_test_inv = 0;
    }
}
