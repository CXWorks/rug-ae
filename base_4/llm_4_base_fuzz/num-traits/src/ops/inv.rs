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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = rug_fuzz_0;
        let result = <&f32 as ops::inv::Inv>::inv(&val);
        debug_assert_eq!(result, 0.5f32);
             }
});    }
    #[test]
    #[should_panic]
    fn test_inv_f32_panic_on_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let val = rug_fuzz_0;
        let _ = <&f32 as ops::inv::Inv>::inv(&val);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let result = Inv::inv(&value);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    #[should_panic(expected = "attempted to divide by zero")]
    fn test_inv_f64_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero = rug_fuzz_0;
        let _ = Inv::inv(&zero);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_423_llm_16_423 {
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f32 = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(< f32 as Inv > ::inv(value), expected);
             }
});    }
    #[test]
    #[should_panic(expected = "attempted to divide by zero")]
    fn test_inv_f32_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f32 = rug_fuzz_0;
        let _result = <f32 as Inv>::inv(value);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_591_llm_16_591 {
    use super::*;
    use crate::*;
    use crate::ops::inv::Inv;
    #[test]
    fn test_inv() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(f64, f64, f64, f64, f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
