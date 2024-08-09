use crate::{CheckedMul, One};
use core::num::Wrapping;
use core::ops::Mul;
/// Binary operator for raising a value to a power.
pub trait Pow<RHS> {
    /// The result after applying the operator.
    type Output;
    /// Returns `self` to the power `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Pow;
    /// assert_eq!(Pow::pow(10u32, 2u32), 100);
    /// ```
    fn pow(self, rhs: RHS) -> Self::Output;
}
macro_rules! pow_impl {
    ($t:ty) => {
        pow_impl!($t, u8); pow_impl!($t, usize);
    };
    ($t:ty, $rhs:ty) => {
        pow_impl!($t, $rhs, usize, pow);
    };
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:expr) => {
        impl Pow <$rhs > for $t { type Output = $t; #[inline] fn pow(self, rhs : $rhs) ->
        $t { ($method) (self, <$desired_rhs >::from(rhs)) } } impl <'a > Pow <&'a $rhs >
        for $t { type Output = $t; #[inline] fn pow(self, rhs : &'a $rhs) -> $t {
        ($method) (self, <$desired_rhs >::from(* rhs)) } } impl <'a > Pow <$rhs > for &'a
        $t { type Output = $t; #[inline] fn pow(self, rhs : $rhs) -> $t { ($method) (*
        self, <$desired_rhs >::from(rhs)) } } impl <'a, 'b > Pow <&'a $rhs > for &'b $t {
        type Output = $t; #[inline] fn pow(self, rhs : &'a $rhs) -> $t { ($method) (*
        self, <$desired_rhs >::from(* rhs)) } }
    };
}
pow_impl!(u8, u8, u32, u8::pow);
pow_impl!(u8, u16, u32, u8::pow);
pow_impl!(u8, u32, u32, u8::pow);
pow_impl!(u8, usize);
pow_impl!(i8, u8, u32, i8::pow);
pow_impl!(i8, u16, u32, i8::pow);
pow_impl!(i8, u32, u32, i8::pow);
pow_impl!(i8, usize);
pow_impl!(u16, u8, u32, u16::pow);
pow_impl!(u16, u16, u32, u16::pow);
pow_impl!(u16, u32, u32, u16::pow);
pow_impl!(u16, usize);
pow_impl!(i16, u8, u32, i16::pow);
pow_impl!(i16, u16, u32, i16::pow);
pow_impl!(i16, u32, u32, i16::pow);
pow_impl!(i16, usize);
pow_impl!(u32, u8, u32, u32::pow);
pow_impl!(u32, u16, u32, u32::pow);
pow_impl!(u32, u32, u32, u32::pow);
pow_impl!(u32, usize);
pow_impl!(i32, u8, u32, i32::pow);
pow_impl!(i32, u16, u32, i32::pow);
pow_impl!(i32, u32, u32, i32::pow);
pow_impl!(i32, usize);
pow_impl!(u64, u8, u32, u64::pow);
pow_impl!(u64, u16, u32, u64::pow);
pow_impl!(u64, u32, u32, u64::pow);
pow_impl!(u64, usize);
pow_impl!(i64, u8, u32, i64::pow);
pow_impl!(i64, u16, u32, i64::pow);
pow_impl!(i64, u32, u32, i64::pow);
pow_impl!(i64, usize);
pow_impl!(u128, u8, u32, u128::pow);
pow_impl!(u128, u16, u32, u128::pow);
pow_impl!(u128, u32, u32, u128::pow);
pow_impl!(u128, usize);
pow_impl!(i128, u8, u32, i128::pow);
pow_impl!(i128, u16, u32, i128::pow);
pow_impl!(i128, u32, u32, i128::pow);
pow_impl!(i128, usize);
pow_impl!(usize, u8, u32, usize::pow);
pow_impl!(usize, u16, u32, usize::pow);
pow_impl!(usize, u32, u32, usize::pow);
pow_impl!(usize, usize);
pow_impl!(isize, u8, u32, isize::pow);
pow_impl!(isize, u16, u32, isize::pow);
pow_impl!(isize, u32, u32, isize::pow);
pow_impl!(isize, usize);
pow_impl!(Wrapping < u8 >);
pow_impl!(Wrapping < i8 >);
pow_impl!(Wrapping < u16 >);
pow_impl!(Wrapping < i16 >);
pow_impl!(Wrapping < u32 >);
pow_impl!(Wrapping < i32 >);
pow_impl!(Wrapping < u64 >);
pow_impl!(Wrapping < i64 >);
pow_impl!(Wrapping < u128 >);
pow_impl!(Wrapping < i128 >);
pow_impl!(Wrapping < usize >);
pow_impl!(Wrapping < isize >);
#[cfg(any(feature = "std", feature = "libm"))]
mod float_impls {
    use super::Pow;
    use crate::Float;
    pow_impl!(f32, i8, i32, < f32 as Float >::powi);
    pow_impl!(f32, u8, i32, < f32 as Float >::powi);
    pow_impl!(f32, i16, i32, < f32 as Float >::powi);
    pow_impl!(f32, u16, i32, < f32 as Float >::powi);
    pow_impl!(f32, i32, i32, < f32 as Float >::powi);
    pow_impl!(f64, i8, i32, < f64 as Float >::powi);
    pow_impl!(f64, u8, i32, < f64 as Float >::powi);
    pow_impl!(f64, i16, i32, < f64 as Float >::powi);
    pow_impl!(f64, u16, i32, < f64 as Float >::powi);
    pow_impl!(f64, i32, i32, < f64 as Float >::powi);
    pow_impl!(f32, f32, f32, < f32 as Float >::powf);
    pow_impl!(f64, f32, f64, < f64 as Float >::powf);
    pow_impl!(f64, f64, f64, < f64 as Float >::powf);
}
/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.
///
/// # Example
///
/// ```rust
/// use num_traits::pow;
///
/// assert_eq!(pow(2i8, 4), 16);
/// assert_eq!(pow(6u8, 3), 216);
/// assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
/// ```
#[inline]
pub fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: usize) -> T {
    if exp == 0 {
        return T::one();
    }
    while exp & 1 == 0 {
        base = base.clone() * base;
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }
    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.clone() * base;
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
    }
    acc
}
/// Raises a value to the power of exp, returning `None` if an overflow occurred.
///
/// Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.
///
/// Otherwise same as the `pow` function.
///
/// # Example
///
/// ```rust
/// use num_traits::checked_pow;
///
/// assert_eq!(checked_pow(2i8, 4), Some(16));
/// assert_eq!(checked_pow(7i8, 8), None);
/// assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
/// assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
/// ```
#[inline]
pub fn checked_pow<T: Clone + One + CheckedMul>(
    mut base: T,
    mut exp: usize,
) -> Option<T> {
    if exp == 0 {
        return Some(T::one());
    }
    while exp & 1 == 0 {
        base = base.checked_mul(&base)?;
        exp >>= 1;
    }
    if exp == 1 {
        return Some(base);
    }
    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = base.checked_mul(&base)?;
        if exp & 1 == 1 {
            acc = acc.checked_mul(&base)?;
        }
    }
    Some(acc)
}
#[cfg(test)]
mod tests_llm_16_3_llm_16_3 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i128_with_u16() {
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(& 2, 3u16), 8i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(&- 2, 3u16), - 8i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(& 0, 0u16), 1i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(& 0, 10u16), 0i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(& 10, 0u16), 1i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(& 10, 1u16), 10i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(&- 3, 4u16), 81i128);
        assert_eq!(<&'static i128 as Pow < u16 >>::pow(&- 3, 5u16), - 243i128);
    }
}
#[cfg(test)]
mod tests_llm_16_4_llm_16_4 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 100_000);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_5_llm_16_5 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i128() {
        assert_eq!(<&'static i128 as Pow < u8 >>::pow(& 2, 10), 1024);
    }
}
#[cfg(test)]
mod tests_llm_16_7_llm_16_7 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i16, u16, i16, u16, i16, u16, i16, u16, i16, u16, i16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_8_llm_16_8 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i16, u32, i16, u32, i16, u32, i16, u32, i16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_9_llm_16_9 {
    use crate::pow::Pow;
    #[test]
    fn pow_i16_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i16, u8, i16, u8, i16, u8, i16, u8, i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i32 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&i32 as Pow<u16>>::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_12_llm_16_12 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i32 = &rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_2), 2);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_3), 4);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_4), 8);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_5), 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_13_llm_16_13 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 32);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_12, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_15_llm_16_15 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_with_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i64 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&i64 as Pow<u16>>::pow(base, exponent);
        debug_assert_eq!(result, 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_16_llm_16_16 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 10);
        debug_assert_eq!(Pow::pow(& rug_fuzz_12, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_17_llm_16_17 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_10, rug_fuzz_11), - 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_14, rug_fuzz_15), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_19_llm_16_19 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i8 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_20_llm_16_20 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i8, u32, i8, u32, i8, u32, i8, u32, i8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8i8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0i8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_21_llm_16_21 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, u8, u8, i8, u8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i8 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 8i8);
        let base: i8 = rug_fuzz_2;
        let exponent: u8 = rug_fuzz_3;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 0i8);
        let base: i8 = rug_fuzz_4;
        let exponent: u8 = rug_fuzz_5;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 1i8);
        let base: i8 = -rug_fuzz_6;
        let exponent: u8 = rug_fuzz_7;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, - 8i8);
        let base: i8 = -rug_fuzz_8;
        let exponent: u8 = rug_fuzz_9;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 16i8);
        let base: i8 = i8::MIN;
        let exponent: u8 = rug_fuzz_10;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, i8::MIN);
        let base: i8 = i8::MIN;
        let exponent: u8 = rug_fuzz_11;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 1i8);
        let base: i8 = rug_fuzz_12;
        let exponent: u8 = rug_fuzz_13;
        let result = <&i8 as Pow<u8>>::pow(&base, exponent);
        debug_assert!(result > rug_fuzz_14);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_23_llm_16_23 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: isize = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_24_llm_16_24 {
    use crate::pow::Pow;
    #[test]
    fn pow_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(isize, u32, isize, u32, isize, u32, isize, u32, isize, u32, isize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< & isize as Pow < u32 > > ::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(
            < & isize as Pow < u32 > > ::pow(& - rug_fuzz_2, rug_fuzz_3), - 8
        );
        debug_assert_eq!(< & isize as Pow < u32 > > ::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(
            < & isize as Pow < u32 > > ::pow(& - rug_fuzz_6, rug_fuzz_7), 1
        );
        debug_assert_eq!(< & isize as Pow < u32 > > ::pow(& rug_fuzz_8, rug_fuzz_9), 10);
        debug_assert_eq!(
            < & isize as Pow < u32 > > ::pow(& rug_fuzz_10, rug_fuzz_11), 1
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use crate::Pow;
    #[test]
    fn pow_u8_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(isize, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &isize = &rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_2), 2);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_3), 4);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_4), 8);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_5), 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: isize = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(& base, exponent), 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_29_llm_16_29 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_i16() {
        let base: Wrapping<i16> = Wrapping(2);
        let exp: u8 = 8;
        let expected: Wrapping<i16> = Wrapping(256);
        assert_eq!(Pow::pow(base, exp), expected);
        let base: Wrapping<i16> = Wrapping(2);
        let exp: u8 = 15;
        let expected: Wrapping<i16> = Wrapping(-32768);
        assert_eq!(Pow::pow(base, exp), expected);
        let base: Wrapping<i16> = Wrapping(1);
        let exp: u8 = 0;
        let expected: Wrapping<i16> = Wrapping(1);
        assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(-2);
        let exp = 2u8;
        let expected = Wrapping(4);
        assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(-2);
        let exp = 3u8;
        let expected = Wrapping(-8);
        assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(2);
        let exp = 8u8;
        let expected = Wrapping(256);
        assert_eq!(Pow::pow(base, & exp), expected);
    }
}
#[cfg(test)]
mod tests_llm_16_31_llm_16_31 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8i32));
        let exponent_ref = &rug_fuzz_2;
        let result_ref = base.pow(exponent_ref);
        debug_assert_eq!(result_ref, Wrapping(16i32));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(256));
             }
});    }
    #[test]
    fn test_pow_wrapping_by_reference() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(256));
             }
});    }
    #[test]
    fn test_pow_wrapping_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping_one_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0));
             }
});    }
    #[test]
    fn test_pow_wrapping_negative_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(-rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(- 32));
             }
});    }
    #[test]
    fn test_pow_wrapping_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i64> = Wrapping(i64::MAX);
        let exponent: u8 = rug_fuzz_0;
        let result = base.pow(exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_34_llm_16_34 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i64, usize, i64, i64, usize, i64, i64, usize, i64, i64, usize, i64, i64, usize, i64, i64, usize, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_3);
        let exponent = rug_fuzz_4;
        let result = base.pow(&exponent);
        let expected = Wrapping(rug_fuzz_5);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_6);
        let exponent = rug_fuzz_7;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_8);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_9);
        let exponent = rug_fuzz_10;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_11);
        debug_assert_eq!(result, expected);
        let base = Wrapping(-rug_fuzz_12);
        let exponent = rug_fuzz_13;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_14);
        debug_assert_eq!(result, expected);
        let base = Wrapping(-rug_fuzz_15);
        let exponent = rug_fuzz_16;
        let result = base.pow(exponent);
        let expected = Wrapping(-rug_fuzz_17);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_pow_wrapping_isize_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_pow_wrapping_isize_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_pow_wrapping_isize_by_zero_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_pow_wrapping_isize_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(isize::MAX);
        let exp: u8 = rug_fuzz_0;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_1);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(isize, usize, isize, usize, isize, usize, isize, usize, isize, usize, usize, isize, usize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(8isize));
        debug_assert_eq!(Wrapping(rug_fuzz_2).pow(rug_fuzz_3), Wrapping(81isize));
        debug_assert_eq!(Wrapping(rug_fuzz_4).pow(rug_fuzz_5), Wrapping(0isize));
        debug_assert_eq!(Wrapping(rug_fuzz_6).pow(rug_fuzz_7), Wrapping(1isize));
        debug_assert_eq!(Wrapping(rug_fuzz_8).pow(rug_fuzz_9), Wrapping(10isize));
        let exp_ref = &rug_fuzz_10;
        debug_assert_eq!(Wrapping(rug_fuzz_11).pow(exp_ref), Wrapping(243isize));
        let exp_ref = &rug_fuzz_12;
        debug_assert_eq!(Wrapping(rug_fuzz_13).pow(exp_ref), Wrapping(1024isize));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u128, u8, u128, u8, u8, u128, u8, u128, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num = Wrapping(rug_fuzz_0);
        let pow_result = Pow::pow(num, rug_fuzz_1);
        debug_assert_eq!(pow_result, Wrapping(8u128));
        let num = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let pow_result = Pow::pow(num, &exponent);
        debug_assert_eq!(pow_result, Wrapping(8u128));
        let num = Wrapping(u128::MAX);
        let pow_result = Pow::pow(num, rug_fuzz_4);
        debug_assert_eq!(pow_result, Wrapping(u128::MAX));
        let num = Wrapping(rug_fuzz_5);
        let pow_result = Pow::pow(num, rug_fuzz_6);
        debug_assert_eq!(pow_result, Wrapping(1u128));
        let num = Wrapping(rug_fuzz_7);
        let pow_result = Pow::pow(num, rug_fuzz_8);
        debug_assert_eq!(pow_result, Wrapping(0u128));
        let num = Wrapping(u128::MAX);
        let pow_result = Pow::pow(num, rug_fuzz_9);
        debug_assert_eq!(pow_result, Wrapping(1u128));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_41_llm_16_41 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u16, u8, u16, u16, u8, u16, u16, u8, u16, u16, u8, u16, u16, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_3);
        let exponent = rug_fuzz_4;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_5);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_6);
        let exponent = rug_fuzz_7;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_8);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_9);
        let exponent = rug_fuzz_10;
        let result = base.pow(exponent);
        let expected = Wrapping(rug_fuzz_11);
        debug_assert_eq!(result, expected);
        let base = Wrapping(rug_fuzz_12);
        let exponent = rug_fuzz_13;
        let result = base.pow(&exponent);
        let expected = Wrapping(rug_fuzz_14);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_42_llm_16_42 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, usize, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = <Wrapping<u16> as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(16u16));
        let base = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let result = <Wrapping<u16> as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(27u16));
             }
});    }
    #[test]
    fn test_pow_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, usize, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = <Wrapping<u16> as Pow<&usize>>::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(16u16));
        let base = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let result = <Wrapping<u16> as Pow<&usize>>::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(27u16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use std::num::Wrapping;
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert_eq!(result, Wrapping(8u32));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_44_llm_16_44 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    use crate::identities::{One, Zero};
    #[test]
    fn test_pow_wrapping_base_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping::one();
        let exponent: usize = rug_fuzz_0;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(1u32)
        );
             }
});    }
    #[test]
    fn test_pow_wrapping_base_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping::zero();
        let exponent: usize = rug_fuzz_0;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(0u32)
        );
             }
});    }
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(32u32)
        );
             }
});    }
    #[test]
    fn test_pow_wrapping_with_reference_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let exponent_ref: &usize = &exponent;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < & usize > > ::pow(base, exponent_ref),
            Wrapping(27u32)
        );
             }
});    }
    #[test]
    fn test_pow_wrapping_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(1u32)
        );
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_pow_wrapping_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u32> = Wrapping(u32::max_value());
        let exponent: usize = rug_fuzz_0;
        let _ = <Wrapping<u32> as Pow<usize>>::pow(base, exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_45_llm_16_45 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow_wrapping_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u8, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(Pow::pow(base, exponent), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u64_by_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u8, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(Pow::pow(base, & exponent), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u64_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u8, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u64_by_ref_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u8, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, & exponent), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u64_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u8, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u64_by_ref_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u8, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, & exponent), result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_46_llm_16_46 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(3u64.pow(4)));
             }
});    }
    #[test]
    fn test_pow_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = &rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(2u64.pow(3)));
             }
});    }
    #[test]
    fn test_pow_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1u64));
             }
});    }
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(u64::MAX);
        let exp = rug_fuzz_0;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(u64::MAX.wrapping_mul(u64::MAX)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_49_llm_16_49 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_wrapping_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), expected);
             }
});    }
    #[test]
    fn pow_wrapping_usize_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let exponent_ref = &exponent;
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent_ref), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8usize));
             }
});    }
    #[test]
    fn test_pow_wrapping_usize_reference() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(8usize));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_51_llm_16_51 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u128 as Pow<u16>>::pow(&base, exponent);
        debug_assert_eq!(result, 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_53_llm_16_53 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: &u128 = &rug_fuzz_0;
        let y: u8 = rug_fuzz_1;
        let result = <&u128 as Pow<u8>>::pow(x, y);
        debug_assert_eq!(result, 256);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_55_llm_16_55 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u16 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_56_llm_16_56 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u16 = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result: u16 = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_57_llm_16_57 {
    use crate::pow::Pow;
    #[test]
    fn pow_u16_by_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u16 = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exp);
        debug_assert_eq!(result, 256u16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_58_llm_16_58 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u16 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u16 as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_59_llm_16_59 {
    use crate::pow::Pow;
    #[test]
    fn test_u32_pow_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u16, u32, u16, u32, u16, u32, u16, u32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 256);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_61_llm_16_61 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u32 = &rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&u32 as Pow<u8>>::pow(base, exponent);
        debug_assert_eq!(result, 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_62_llm_16_62 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u32 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_63_llm_16_63 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u64 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 1024u64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_64_llm_16_64 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u64 = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&u64 as Pow<u32>>::pow(base, exponent);
        debug_assert_eq!(result, 16u64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_65_llm_16_65 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u64 = &rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 100);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_66_llm_16_66 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u64 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_68_llm_16_68 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u8 = rug_fuzz_0;
        let exp: u32 = rug_fuzz_1;
        let result = <&u8 as Pow<u32>>::pow(&base, exp);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u8 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&u8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 8u8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_70_llm_16_70 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, usize, u8, usize, u8, usize, u8, usize, u8, usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 10);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_72_llm_16_72 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &usize = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_73_llm_16_73 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(usize, u8, usize, u8, usize, u8, usize, u8, usize, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 32);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_74_llm_16_74 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &usize = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result: usize = Pow::pow(base, exponent);
        let expected: usize = rug_fuzz_2;
        debug_assert_eq!(result, expected, "Testing 2^3");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_75_llm_16_75 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_76_llm_16_76 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, & rug_fuzz_3), - 27);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, & rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_77_llm_16_77 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 81);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_i128_with_ref_u8_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = i128::MAX;
        let exp: &u8 = &rug_fuzz_0;
        let _ = <&i128 as Pow<&u8>>::pow(&base, exp);
             }
});    }
    #[test]
    fn test_pow_i128_with_ref_u8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 1);
             }
});    }
    #[test]
    fn test_pow_i128_with_ref_u8_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 0);
             }
});    }
    #[test]
    fn test_pow_i128_with_ref_u8_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 1);
             }
});    }
    #[test]
    fn test_pow_i128_with_ref_u8_large_exp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, u8, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert!(
            result > rug_fuzz_2, "power with large exponent should not overflow i128"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_78_llm_16_78 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_usize_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&i128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_79_llm_16_79 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i16 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_80_llm_16_80 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i16 = rug_fuzz_0;
        let exp: u32 = rug_fuzz_1;
        let result = <&i16 as Pow<&u32>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_81_llm_16_81 {
    use crate::Pow;
    #[test]
    fn test_pow_i16_u8_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i16, u8, i16, u8, i16, u8, i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i16 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let base: i16 = rug_fuzz_2;
        let exponent: u8 = rug_fuzz_3;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1);
        let base: i16 = -rug_fuzz_4;
        let exponent: u8 = rug_fuzz_5;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 16);
        let base: i16 = -rug_fuzz_6;
        let exponent: u8 = rug_fuzz_7;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, - 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_82_llm_16_82 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i16 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&i16 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_84_llm_16_84 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i32 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i32 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_85_llm_16_85 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i32 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&i32 as Pow<&u8>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_86_llm_16_86 {
    use crate::pow::Pow;
    #[test]
    fn pow_i32_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i32 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result: i32 = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_87_llm_16_87 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_88_llm_16_88 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i64 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i64 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, u8, i64, u8, i64, u8, i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 8i64);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, & rug_fuzz_3), - 8i64);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1i64);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 1i64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_90_llm_16_90 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i64 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_91_llm_16_91 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i8, u16, i8, u16, i8, u16, i8, u16, i8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i8 = &rug_fuzz_0;
        let exponent: &u16 = &rug_fuzz_1;
        let result = <&i8 as Pow<&u16>>::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let base: &i8 = &rug_fuzz_2;
        let exponent: &u16 = &rug_fuzz_3;
        let result = <&i8 as Pow<&u16>>::pow(base, exponent);
        debug_assert_eq!(result, 1);
        let base: &i8 = &rug_fuzz_4;
        let exponent: &u16 = &rug_fuzz_5;
        let result = <&i8 as Pow<&u16>>::pow(base, exponent);
        debug_assert_eq!(result, 2);
        let base: &i8 = &rug_fuzz_6;
        let exponent: &u16 = &rug_fuzz_7;
        let result = <&i8 as Pow<&u16>>::pow(base, exponent);
        debug_assert_eq!(result, 0);
        let base: &i8 = &-rug_fuzz_8;
        let exponent: &u16 = &rug_fuzz_9;
        let result = <&i8 as Pow<&u16>>::pow(base, exponent);
        debug_assert_eq!(result, - 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_92_llm_16_92 {
    use crate::Pow;
    #[test]
    fn pow_i8_ref_with_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i8 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn pow_i8_ref_with_u32_ref_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let _result = <&i8 as Pow<&u32>>::pow(&base, &exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_93_llm_16_93 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &i8 = &rug_fuzz_0;
        let exponent: &u8 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_94_llm_16_94 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i8 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_95_llm_16_95 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &isize = &rug_fuzz_0;
        let exponent: &u16 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_96_llm_16_96 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: isize = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_97_llm_16_97 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: isize = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = <&isize as Pow<&u8>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_98_llm_16_98 {
    use crate::pow::Pow;
    #[test]
    fn pow_isize_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: isize = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exp);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_99_llm_16_99 {
    use std::num::Wrapping;
    use crate::Pow;
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(u8, u8, u8, u8, u8, u8, i128, u8, i128, u8, i128, u8, i128, u8, i128, u8, i128, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Wrapping:: < i128 > ::zero().pow(& rug_fuzz_0), Wrapping:: < i128 > ::zero()
        );
        debug_assert_eq!(
            Wrapping:: < i128 > ::zero().pow(& rug_fuzz_1), Wrapping:: < i128 > ::zero()
        );
        debug_assert_eq!(
            Wrapping:: < i128 > ::zero().pow(& rug_fuzz_2), Wrapping:: < i128 > ::zero()
        );
        debug_assert_eq!(
            Wrapping:: < i128 > ::one().pow(& rug_fuzz_3), Wrapping:: < i128 > ::one()
        );
        debug_assert_eq!(
            Wrapping:: < i128 > ::one().pow(& rug_fuzz_4), Wrapping:: < i128 > ::one()
        );
        debug_assert_eq!(
            Wrapping:: < i128 > ::one().pow(& rug_fuzz_5), Wrapping:: < i128 > ::one()
        );
        debug_assert_eq!(Wrapping(rug_fuzz_6).pow(& rug_fuzz_7), Wrapping(4i128));
        debug_assert_eq!(Wrapping(rug_fuzz_8).pow(& rug_fuzz_9), Wrapping(8i128));
        debug_assert_eq!(Wrapping(rug_fuzz_10).pow(& rug_fuzz_11), Wrapping(81i128));
        debug_assert_eq!(Wrapping(- rug_fuzz_12).pow(& rug_fuzz_13), Wrapping(4i128));
        debug_assert_eq!(Wrapping(- rug_fuzz_14).pow(& rug_fuzz_15), Wrapping(- 8i128));
        debug_assert_eq!(Wrapping(rug_fuzz_16).pow(& rug_fuzz_17), Wrapping(1i128));
        debug_assert_eq!(Wrapping(i128::MAX).pow(& rug_fuzz_18), Wrapping(i128::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_100_llm_16_100 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(5i128.pow(3)));
             }
});    }
    #[test]
    fn test_pow_wrapping_i128_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping_i128_with_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, base);
             }
});    }
    #[test]
    fn test_pow_wrapping_i128_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(i128::MAX);
        let exp: usize = rug_fuzz_0;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(i128::MAX.wrapping_pow(2)));
             }
});    }
    #[test]
    fn test_pow_wrapping_i128_large_power() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(2i128.wrapping_pow(100)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_101 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(16));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_with_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(2));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_negative_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(-rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(- 8));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_large_exp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(6561));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(i16::MAX);
        let exp = rug_fuzz_0;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping_i16_min() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(i16::MIN);
        let exp = rug_fuzz_0;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_102_llm_16_102 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8i16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_103_llm_16_103 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(256i32));
        let base = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(1i32));
        let base = Wrapping(rug_fuzz_4);
        let exponent = rug_fuzz_5;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(1i32));
        let base = Wrapping(rug_fuzz_6);
        let exponent = rug_fuzz_7;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(1i32));
        let base = Wrapping(-rug_fuzz_8);
        let exponent = rug_fuzz_9;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(- 32i32));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_104_llm_16_104 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn pow_wrapping_values() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn pow_wrapping_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(i32::MAX);
        let exp: usize = rug_fuzz_0;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_1);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn pow_wrapping_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn pow_wrapping_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn pow_wrapping_large_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn pow_wrapping_exponent_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_105_llm_16_105 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(32i64));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_106_llm_16_106 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, usize, i64, usize, i64, usize, i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(2i64.pow(3)));
        let base = Wrapping(rug_fuzz_2);
        let exp = rug_fuzz_3;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(-rug_fuzz_4);
        let exp = rug_fuzz_5;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(4));
        let base = Wrapping(-rug_fuzz_6);
        let exp = rug_fuzz_7;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(- 8));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_107_llm_16_107 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8i8));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_108_llm_16_108 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, usize, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_109_llm_16_109 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_wrapping_isize_base_u8_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(isize, u8, isize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping::<isize>(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = Wrapping::<isize>(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_110 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<isize> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let result = <&Wrapping<isize> as pow::Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, Wrapping(8));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_111_llm_16_111 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping::<u128>(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(7_u128.pow(3)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_112_llm_16_112 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = <&Wrapping<u128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(16u128));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_113_llm_16_113 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_wrapping_u16_by_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u16> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result: Wrapping<u16> = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_114_llm_16_114 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent: &usize = &rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8u16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_115_llm_16_115 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u32_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(256u32));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_117_llm_16_117 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_wrapping_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(3u64.pow(4)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_119_llm_16_119 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = value.pow(&exponent);
        debug_assert_eq!(result, Wrapping(2u8.pow(3)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_120_llm_16_120 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(8u8));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_121_llm_16_121 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<usize> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result: Wrapping<usize> = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(256));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_122_llm_16_122 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(16));
             }
});    }
    #[test]
    fn pow_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1));
             }
});    }
    #[test]
    fn pow_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(2));
             }
});    }
    #[test]
    fn pow_large_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1024));
             }
});    }
    #[test]
    fn pow_large_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1000));
             }
});    }
    #[test]
    fn pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(usize::MAX);
        let exponent = rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_123_llm_16_123 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u128 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1024_u128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_124_llm_16_124 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_ref_with_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u128 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1024u128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_125_llm_16_125 {
    use crate::Pow;
    #[test]
    fn test_pow_u128_ref_with_u8_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u128, u8, u128, u8, u128, u8, u128, u8, u128, u8, u128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 8u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, & rug_fuzz_3), 9u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 0u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, & rug_fuzz_9), 1u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, & rug_fuzz_11), 100000u128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_126_llm_16_126 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u128 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 16);
        let base: u128 = rug_fuzz_2;
        let exponent: usize = rug_fuzz_3;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 0);
        let base: u128 = rug_fuzz_4;
        let exponent: usize = rug_fuzz_5;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1);
        let base: u128 = rug_fuzz_6;
        let exponent: usize = rug_fuzz_7;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1);
        let base: u128 = rug_fuzz_8;
        let exponent: usize = rug_fuzz_9;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1);
        let base: u128 = rug_fuzz_10;
        let exponent: usize = rug_fuzz_11;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1_000_000_000_000_000_000_000_000);
        let base: u128 = rug_fuzz_12;
        let exponent: usize = rug_fuzz_13;
        let result = <&u128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1 << 64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_127_llm_16_127 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u16 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_128 {
    use crate::pow::Pow;
    #[test]
    fn pow_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u16 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_129_llm_16_129 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_u16_ref_with_u8_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u16 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_131_llm_16_131 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u32 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_132_llm_16_132 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        debug_assert_eq!(< & u32 as Pow < & u32 > > ::pow(& base, & exponent), 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_133_llm_16_133 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &u32 = &rug_fuzz_0;
        let exponent: &u8 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u32);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_134_llm_16_134 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u32 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_135_llm_16_135 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u64 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1024u64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_136_llm_16_136 {
    use crate::pow::Pow;
    #[test]
    fn pow_u64_with_ref_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u64 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_137_llm_16_137 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_138_llm_16_138 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u64 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u64 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_139_llm_16_139 {
    use crate::pow::Pow;
    use core::convert::From;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u8 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8u8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_140_llm_16_140 {
    use super::*;
    use crate::*;
    #[test]
    fn pow_u8_ref_with_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&u8 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8_u8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_141_llm_16_141 {
    use crate::pow::Pow;
    #[test]
    fn test_u8_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u8 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_142_llm_16_142 {
    use crate::pow::Pow;
    #[test]
    fn pow_u8_by_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, usize, u8, usize, u8, usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((& rug_fuzz_0).pow(& rug_fuzz_1), 1u8);
        debug_assert_eq!((& rug_fuzz_2).pow(& rug_fuzz_3), 5u8);
        debug_assert_eq!((& rug_fuzz_4).pow(& rug_fuzz_5), 25u8);
        debug_assert_eq!((& rug_fuzz_6).pow(& rug_fuzz_7), 125u8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_143_llm_16_143 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u16_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: usize = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = <&usize as Pow<&u16>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_144_llm_16_144 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: usize = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_145_llm_16_145 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u8_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, u8, usize, u8, usize, u8, usize, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < & usize as Pow < & u8 > > ::pow(& rug_fuzz_0, & rug_fuzz_1), 8
        );
        debug_assert_eq!(
            < & usize as Pow < & u8 > > ::pow(& rug_fuzz_2, & rug_fuzz_3), 9
        );
        debug_assert_eq!(
            < & usize as Pow < & u8 > > ::pow(& rug_fuzz_4, & rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < & usize as Pow < & u8 > > ::pow(& rug_fuzz_6, & rug_fuzz_7), 0
        );
        debug_assert_eq!(
            < & usize as Pow < & u8 > > ::pow(& rug_fuzz_8, & rug_fuzz_9), 1
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_146_llm_16_146 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: usize = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_696_llm_16_696 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: i128 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = <i128 as Pow<&u16>>::pow(base, &exp);
        debug_assert_eq!(result, 16);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_697_llm_16_697 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn pow_i128_with_ref_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 4i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, & rug_fuzz_11), - 8i128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_698_llm_16_698 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i128, u8, i128, u8, i128, u8, i128, u8, i128, u8, i128, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(
            < i128 as Pow < & u8 > > ::pow(- rug_fuzz_4, & rug_fuzz_5), - 8
        );
        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(- rug_fuzz_6, & rug_fuzz_7), 16);
        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 2);
        debug_assert_eq!(< i128 as Pow < & u8 > > ::pow(i128::MAX, & rug_fuzz_12), 1);
        debug_assert_eq!(
            < i128 as Pow < & u8 > > ::pow(i128::MIN, & rug_fuzz_13), i128::MIN
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_699_llm_16_699 {
    use crate::Pow;
    #[test]
    fn test_pow_i128_with_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24, mut rug_fuzz_25)) = <(i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 0_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 0_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 1_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 1_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 2_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_16, & rug_fuzz_17), 4_i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_18, & rug_fuzz_19), 8_i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_20, & rug_fuzz_21), - 8_i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_22, & rug_fuzz_23), 9_i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_24, & rug_fuzz_25), - 27_i128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_700_llm_16_700 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i128, u16, i128, u16, i128, u16, i128, u16, i128, u16, i128, u16, i128, u16, i128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, rug_fuzz_1), 16i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, rug_fuzz_3), - 8i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, rug_fuzz_5), 1i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, rug_fuzz_7), 0i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, rug_fuzz_9), 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, rug_fuzz_11), 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_12, rug_fuzz_13), - 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_14, rug_fuzz_15), 1i128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_701_llm_16_701 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1024);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 32);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_702_llm_16_702 {
    use crate::Pow;
    #[test]
    fn test_pow_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, u8, i128, u8, i128, u8, i128, u8, i128, u8, i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_703_llm_16_703 {
    use crate::pow::Pow;
    #[test]
    fn i128_pow_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, usize, i128, usize, i128, usize, i128, usize, i128, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(- rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(i128::MAX, rug_fuzz_10), 1);
        debug_assert_eq!(
            < i128 as Pow < usize > > ::pow(i128::MIN, rug_fuzz_11), i128::MIN
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_806_llm_16_806 {
    use crate::pow::Pow;
    #[test]
    fn i16_pow_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i16, u16, i16, u16, i16, u16, i16, u16, i16, u16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(
            < i16 as Pow < & u16 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), - 8
        );
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(- rug_fuzz_10, & u16::MAX), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_807_llm_16_807 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(2, & 2), 4);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(3, & 3), 27);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(0, & 0), 1);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(0, & 1), 0);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(1, & 0), 1);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(- 1, & 2), 1);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(- 1, & 3), - 1);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(- 2, & 2), 4);
        assert_eq!(< i16 as Pow <&'static u32 >>::pow(- 2, & 3), - 8);
    }
}
#[cfg(test)]
mod tests_llm_16_808_llm_16_808 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i16_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i16, u8, i16, u8, i16, u8, i16, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(i16::MAX, & rug_fuzz_8), i16::MAX);
        debug_assert_eq!(Pow::pow(i16::MIN, & rug_fuzz_9), i16::MIN);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_809_llm_16_809 {
    use crate::Pow;
    #[test]
    fn test_i16_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i16, usize, i16, usize, i16, usize, i16, usize, i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_810_llm_16_810 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i16, u16, i16, u16, i16, u16, i16, u16, i16, u16, i16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_811_llm_16_811 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_14, rug_fuzz_15), 10000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_812_llm_16_812 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21)) = <(i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8, i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 4);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 8);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_14, rug_fuzz_15), 81);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(- rug_fuzz_16, rug_fuzz_17), 81);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(- rug_fuzz_18, rug_fuzz_19), - 243);
        debug_assert_eq!(< i16 as Pow < u8 > > ::pow(rug_fuzz_20, rug_fuzz_21), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_813_llm_16_813 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i16, usize, i16, usize, i16, usize, i16, usize, i16, usize, i16, usize, i16, usize, i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 1);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 81);
        debug_assert_eq!(< i16 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(
            < i16 as Pow < usize > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 343
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_916_llm_16_916 {
    use crate::pow::Pow;
    #[test]
    fn test_i32_pow_u16_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u16, i32, u16, i32, u16, i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 5);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 81);
        debug_assert_eq!(
            < i32 as Pow < & u16 > > ::pow(- rug_fuzz_8, & rug_fuzz_9), - 27
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_917 {
    use crate::Pow;
    use std::convert::From;
    #[test]
    fn i32_pow_with_reference_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 3i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 16i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 125i32);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 9i32);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, & rug_fuzz_11), - 8i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 2i32.pow(31u32));
             }
});    }
    #[test]
    #[should_panic]
    fn i32_pow_with_reference_u32_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        Pow::pow(rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_918_llm_16_918 {
    use crate::Pow;
    #[test]
    fn test_pow_i32_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< i32 as Pow < & u8 > > ::pow(- rug_fuzz_8, & rug_fuzz_9), 4);
        debug_assert_eq!(
            < i32 as Pow < & u8 > > ::pow(- rug_fuzz_10, & rug_fuzz_11), - 27
        );
        debug_assert_eq!(
            < i32 as Pow < & u8 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 100000
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_919_llm_16_919 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i32_with_ref_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< i32 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 27);
        debug_assert_eq!(< i32 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(
            < i32 as Pow < & usize > > ::pow(- rug_fuzz_8, & rug_fuzz_9), - 8
        );
        debug_assert_eq!(
            < i32 as Pow < & usize > > ::pow(- rug_fuzz_10, & rug_fuzz_11), 16
        );
        debug_assert_eq!(
            < i32 as Pow < & usize > > ::pow(rug_fuzz_12, & rug_fuzz_13), 1
        );
        debug_assert_eq!(
            < i32 as Pow < & usize > > ::pow(- rug_fuzz_14, & rug_fuzz_15), 1
        );
        debug_assert_eq!(
            < i32 as Pow < & usize > > ::pow(- rug_fuzz_16, & rug_fuzz_17), - 1
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_920_llm_16_920 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 9);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_921_llm_16_921 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 2);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 8);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 4);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_922_llm_16_922 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i32_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8, i32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_14, rug_fuzz_15), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_16, rug_fuzz_17), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_923 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(
            < i32 as Pow < usize > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8
        );
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1026_llm_16_1026 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_with_u16_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, u16, i64, u16, i64, u16, i64, u16, i64, u16, i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(
            < i64 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 100_000
        );
        debug_assert_eq!(< i64 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i64 as Pow < & u16 > > ::pow(- rug_fuzz_6, & rug_fuzz_7), 4);
        debug_assert_eq!(
            < i64 as Pow < & u16 > > ::pow(- rug_fuzz_8, & rug_fuzz_9), - 27
        );
        debug_assert_eq!(< i64 as Pow < & u16 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1027_llm_16_1027 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< i64 as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 27);
        debug_assert_eq!(< i64 as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i64 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(
            < i64 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 100000
        );
        debug_assert_eq!(
            < i64 as Pow < & u32 > > ::pow(- rug_fuzz_10, & rug_fuzz_11), 4
        );
        debug_assert_eq!(
            < i64 as Pow < & u32 > > ::pow(- rug_fuzz_12, & rug_fuzz_13), - 27
        );
        debug_assert_eq!(
            < i64 as Pow < & u32 > > ::pow(rug_fuzz_14, & rug_fuzz_15), 1024
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1028_llm_16_1028 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_u8_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 512);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 16);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 9);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1029_llm_16_1029 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_with_usize_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, usize, i64, usize, i64, usize, i64, usize, i64, usize, i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 9);
        debug_assert_eq!(< i64 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< i64 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(
            < i64 as Pow < & usize > > ::pow(- rug_fuzz_8, & rug_fuzz_9), 4
        );
        debug_assert_eq!(
            < i64 as Pow < & usize > > ::pow(- rug_fuzz_10, & rug_fuzz_11), - 27
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1030_llm_16_1030 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i64, u16, i64, u16, i64, u16, i64, u16, i64, u16, i64, u16, i64, u16, i64, u16, i64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1024);
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 10);
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(
            < i64 as Pow < u16 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 512
        );
        debug_assert_eq!(
            < i64 as Pow < u16 > > ::pow(- rug_fuzz_14, rug_fuzz_15), 59049
        );
        debug_assert_eq!(< i64 as Pow < u16 > > ::pow(rug_fuzz_16, rug_fuzz_17), 65536);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1031_llm_16_1031 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1032_llm_16_1032 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 2);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 2);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1033_llm_16_1033 {
    use crate::pow::Pow;
    #[test]
    fn test_i64_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i64, usize, i64, usize, i64, usize, i64, usize, i64, usize, i64, usize, i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 10);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1136_llm_16_1136 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i8, u16, i8, u16, i8, u16, i8, u16, i8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1137_llm_16_1137 {
    use crate::pow::Pow;
    #[test]
    fn test_i8_pow_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), 4);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(- rug_fuzz_4, & rug_fuzz_5), - 8);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_14, & rug_fuzz_15), 0);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_16, & rug_fuzz_17), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1138_llm_16_1138 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i8_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 27);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 1);
        debug_assert_eq!(
            < i8 as Pow < & u8 > > ::pow(- rug_fuzz_14, & rug_fuzz_15), - 1
        );
        debug_assert_eq!(< i8 as Pow < & u8 > > ::pow(- rug_fuzz_16, & rug_fuzz_17), 4);
        debug_assert_eq!(
            < i8 as Pow < & u8 > > ::pow(- rug_fuzz_18, & rug_fuzz_19), - 8
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1139_llm_16_1139 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i8_with_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i8, usize, i8, usize, i8, usize, i8, usize, i8, usize, i8, usize, i8, usize, i8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4i8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_6, & rug_fuzz_7), 4i8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), - 8i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 0i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 0i8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1140_llm_16_1140 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i8, u16, i8, u16, i8, u16, i8, u16, i8, u16, i8, u16, i8, u16, i16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 1);
        #[allow(overflowing_literals)]
        {
            debug_assert!(
                (< i8 as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13) as i16) !=
                (rug_fuzz_14.pow(rug_fuzz_15))
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1141_llm_16_1141 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32, i8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1142_llm_16_1142 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8, i8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(- rug_fuzz_6, rug_fuzz_7), 16);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_12, rug_fuzz_13), - 128);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 128);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(rug_fuzz_16, rug_fuzz_17), 0);
        debug_assert_eq!(< i8 as Pow < u8 > > ::pow(- rug_fuzz_18, rug_fuzz_19), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1143_llm_16_1143 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i8, usize, i8, usize, i8, usize, i8, usize, i8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1246_llm_16_1246 {
    use crate::pow::Pow;
    #[test]
    fn pow_isize_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16, i32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(- rug_fuzz_6, & rug_fuzz_7), - 27);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_14, & rug_fuzz_15), - 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_16, & rug_fuzz_17), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_18, & rug_fuzz_19), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1247_llm_16_1247 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn pow_isize_with_ref_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(isize, u32, isize, u32, isize, u32, isize, u32, isize, u32, isize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< isize as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1);
        debug_assert_eq!(< isize as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 2);
        debug_assert_eq!(< isize as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< isize as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 8);
        debug_assert_eq!(
            < isize as Pow < & u32 > > ::pow(- rug_fuzz_8, & rug_fuzz_9), 4
        );
        debug_assert_eq!(
            < isize as Pow < & u32 > > ::pow(- rug_fuzz_10, & rug_fuzz_11), - 8
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1248_llm_16_1248 {
    use crate::Pow;
    #[test]
    fn test_pow_isize_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(isize, u8, isize, u8, isize, u8, isize, u8, isize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1249_llm_16_1249 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_isize() {
        let _rug_st_tests_llm_16_1249_llm_16_1249_rrrruuuugggg_test_pow_isize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 2;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 1;
        let rug_fuzz_28 = 2;
        let rug_fuzz_29 = 1;
        let rug_fuzz_30 = 3;
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 8
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_8, & rug_fuzz_9), 4
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_10, & rug_fuzz_11), - 8
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_12, & rug_fuzz_13), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_14, & rug_fuzz_15), 0
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_16, & rug_fuzz_17), 0
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_18, & rug_fuzz_19), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_20, & rug_fuzz_21), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(rug_fuzz_22, & usize::MAX), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_23, & rug_fuzz_24), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_25, & rug_fuzz_26), - 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_27, & rug_fuzz_28), 1
        );
        debug_assert_eq!(
            < isize as Pow < & usize > > ::pow(- rug_fuzz_29, & rug_fuzz_30), - 1
        );
        let _rug_ed_tests_llm_16_1249_llm_16_1249_rrrruuuugggg_test_pow_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1250_llm_16_1250 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(isize, u16, isize, u16, isize, u16, isize, u16, isize, u16, isize, u16, isize, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< isize as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< isize as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< isize as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< isize as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< isize as Pow < u16 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(
            < isize as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8
        );
        debug_assert_eq!(
            < isize as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 65_536
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1251_llm_16_1251 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(isize, u32, isize, u32, isize, u32, isize, u32, isize, u32, isize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 9);
        debug_assert_eq!(
            < isize as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1252_llm_16_1252 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_isize_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(isize, u8, isize, u8, isize, u8, isize, u8, isize, u8, isize, u8, isize, u8, isize, u8, isize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_14, rug_fuzz_15), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_16, rug_fuzz_17), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1309_llm_16_1309 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = <Wrapping<i128> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(8_i128));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1312_llm_16_1312 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(8i16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1313_llm_16_1313 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_for_wrapping_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i16, u8, i16, u8, i16, u8, i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < Wrapping < i16 > as Pow < u8 > > ::pow(Wrapping(rug_fuzz_0), rug_fuzz_1),
            Wrapping(8)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < u8 > > ::pow(Wrapping(rug_fuzz_2), rug_fuzz_3),
            Wrapping(0)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < u8 > > ::pow(Wrapping(rug_fuzz_4), rug_fuzz_5),
            Wrapping(1)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < u8 > > ::pow(Wrapping(- rug_fuzz_6), rug_fuzz_7),
            Wrapping(4)
        );
             }
});    }
    #[test]
    fn pow_for_wrapping_i16_with_reference() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i16, u8, i16, u8, i16, u8, i16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < Wrapping < i16 > as Pow < & u8 > > ::pow(Wrapping(rug_fuzz_0), &
            rug_fuzz_1), Wrapping(8)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < & u8 > > ::pow(Wrapping(rug_fuzz_2), &
            rug_fuzz_3), Wrapping(0)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < & u8 > > ::pow(Wrapping(rug_fuzz_4), &
            rug_fuzz_5), Wrapping(1)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < & u8 > > ::pow(Wrapping(- rug_fuzz_6), &
            rug_fuzz_7), Wrapping(4)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1314_llm_16_1314 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_for_wrapping_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i16, usize, i16, usize, i16, usize, i16, usize, i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < Wrapping < i16 > as Pow < usize > > ::pow(Wrapping(rug_fuzz_0),
            rug_fuzz_1), Wrapping(8)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < usize > > ::pow(Wrapping(rug_fuzz_2),
            rug_fuzz_3), Wrapping(1)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < usize > > ::pow(Wrapping(- rug_fuzz_4),
            rug_fuzz_5), Wrapping(- 8)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < usize > > ::pow(Wrapping(- rug_fuzz_6),
            rug_fuzz_7), Wrapping(4)
        );
        debug_assert_eq!(
            < Wrapping < i16 > as Pow < usize > > ::pow(Wrapping(rug_fuzz_8),
            rug_fuzz_9), Wrapping(1)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1316_llm_16_1316 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(8));
             }
});    }
    #[test]
    fn test_pow_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(0));
             }
});    }
    #[test]
    fn test_pow_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_of_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn test_pow_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(i32::MAX);
        let exp = rug_fuzz_0;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1317_llm_16_1317 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    use crate::Bounded;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u8, i32, u8, i32, u8, i32, u8, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(256));
        let base: Wrapping<i32> = Wrapping(rug_fuzz_2);
        let exponent: u8 = rug_fuzz_3;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(0));
        let base: Wrapping<i32> = Wrapping(rug_fuzz_4);
        let exponent: u8 = rug_fuzz_5;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(1));
        let base: Wrapping<i32> = Wrapping(rug_fuzz_6);
        let exponent: u8 = rug_fuzz_7;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(1 << 31));
        let base: Wrapping<i32> = Wrapping(rug_fuzz_8);
        let result = Pow::pow(base, Wrapping::<u8>::max_value().0);
        debug_assert_eq!(result, Wrapping(1 << 31));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1321_llm_16_1321 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, i64, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(8));
        let base = Wrapping(rug_fuzz_2);
        let exp = rug_fuzz_3;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(rug_fuzz_4);
        let exp = rug_fuzz_5;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(2));
        let base = Wrapping(rug_fuzz_6);
        let exp = rug_fuzz_7;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(0));
        let base = Wrapping(rug_fuzz_8);
        let exp = rug_fuzz_9;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(-rug_fuzz_10);
        let exp = rug_fuzz_11;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(-rug_fuzz_12);
        let exp = rug_fuzz_13;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(- 1));
        let base = Wrapping(i64::MAX);
        let exp = rug_fuzz_14;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(i64::MAX));
        let base = Wrapping(i64::MAX);
        let exp = rug_fuzz_15;
        let result = <Wrapping<i64> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1327_llm_16_1327 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(isize, u8, isize, u8, isize, u8, isize, u8, isize, u8, isize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(& rug_fuzz_1), Wrapping(4isize));
        debug_assert_eq!(Wrapping(rug_fuzz_2).pow(& rug_fuzz_3), Wrapping(27isize));
        debug_assert_eq!(Wrapping(rug_fuzz_4).pow(& rug_fuzz_5), Wrapping(0isize));
        debug_assert_eq!(Wrapping(rug_fuzz_6).pow(& rug_fuzz_7), Wrapping(1isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_8).pow(& rug_fuzz_9), Wrapping(- 8isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_10).pow(& rug_fuzz_11), Wrapping(16isize));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1330_llm_16_1330 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_usize_wrapping_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8_isize));
             }
});    }
    #[test]
    fn pow_usize_wrapping_isize_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(isize::MAX);
        let exponent = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
             }
});    }
    #[test]
    fn pow_usize_wrapping_isize_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(isize::MIN);
        let exponent = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0));
             }
});    }
    #[test]
    fn pow_usize_wrapping_isize_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = &rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(81_isize));
             }
});    }
    #[test]
    fn pow_usize_wrapping_isize_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1_isize));
             }
});    }
    #[test]
    fn pow_usize_wrapping_isize_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0_isize));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1333_llm_16_1333 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u128 > as Pow < u8 > > ::pow(base, exp), Wrapping(256u128)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1334_llm_16_1334 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(8u128));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1335_llm_16_1335 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(256u16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1336_llm_16_1336 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u16_with_ref_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u16> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(16));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1337_llm_16_1337 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u16, u8, u16, u16, u8, u16, u16, u8, u16, u16, u8, u16, u16, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(rug_fuzz_3);
        let exp = rug_fuzz_4;
        let expected = Wrapping(rug_fuzz_5);
        debug_assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(rug_fuzz_6);
        let exp = rug_fuzz_7;
        let expected = Wrapping(rug_fuzz_8);
        debug_assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(rug_fuzz_9);
        let exp = rug_fuzz_10;
        let expected = Wrapping(rug_fuzz_11);
        debug_assert_eq!(Pow::pow(base, exp), expected);
        let base = Wrapping(rug_fuzz_12);
        let exp = rug_fuzz_13;
        let expected = Wrapping(rug_fuzz_14);
        debug_assert_eq!(Pow::pow(base, exp), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1338_llm_16_1338 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(8u16));
             }
});    }
    #[test]
    fn pow_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(1u16));
             }
});    }
    #[test]
    fn pow_one_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(2u16));
             }
});    }
    #[test]
    fn pow_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(0u16));
             }
});    }
    #[test]
    fn pow_one_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(1u16));
             }
});    }
    #[test]
    fn pow_large_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(0u16));
             }
});    }
    #[test]
    #[should_panic]
    fn pow_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u16, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _result = Wrapping(rug_fuzz_0)
            .pow(rug_fuzz_1)
            .0
            .overflowing_pow(rug_fuzz_2)
            .1;
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1342_llm_16_1342 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(16u32));
             }
});    }
    #[test]
    fn test_pow_wrapping_u32_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(16u32));
             }
});    }
    #[test]
    fn test_pow_wrapping_u32_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1u32));
             }
});    }
    #[test]
    fn test_pow_wrapping_u32_large() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1u32 << 31));
             }
});    }
    #[test]
    fn test_pow_wrapping_u32_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0u32));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1343_llm_16_1343 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: Wrapping<u64> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = <Wrapping<u64> as Pow<&u8>>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(2u64.pow(5)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1345_llm_16_1345 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn wrapping_pow_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u8, u64, u8, u64, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = a.pow(exp);
        debug_assert_eq!(result, Wrapping(32u64));
        let a = Wrapping(rug_fuzz_2);
        let exp = rug_fuzz_3;
        let result = a.pow(exp);
        debug_assert_eq!(result, Wrapping(1u64));
        let a = Wrapping(rug_fuzz_4);
        let exp = rug_fuzz_5;
        let result = a.pow(exp);
        debug_assert_eq!(result, Wrapping(0u64));
        let a = Wrapping(u64::MAX);
        let exp = rug_fuzz_6;
        let result = a.pow(exp);
        debug_assert_eq!(result, Wrapping(u64::MAX));
        let a = Wrapping(u64::MAX);
        let exp = rug_fuzz_7;
        let result = a.pow(exp);
        debug_assert_eq!(result, Wrapping(1u64));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1346_llm_16_1346 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert_eq!(result, Wrapping(8u64));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1348_llm_16_1348 {
    use std::num::Wrapping;
    use std::ops::Mul;
    #[test]
    fn test_pow_wrapping_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.mul(base).mul(base);
        debug_assert_eq!(Wrapping(rug_fuzz_2), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u8_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(u8::MAX);
        let exp = rug_fuzz_0;
        let result = base;
        debug_assert_eq!(Wrapping(u8::MAX), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Wrapping(rug_fuzz_3), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u8_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u32, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Wrapping(rug_fuzz_3), result);
             }
});    }
    #[test]
    fn test_pow_wrapping_u8_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(u8::MAX);
        let exp = rug_fuzz_0;
        let result = base.mul(base);
        debug_assert_eq!(Wrapping(rug_fuzz_1), result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1352_llm_16_1352 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn pow_wrapped_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = &rug_fuzz_1;
        let result = <Wrapping<usize> as Pow<&usize>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(8usize));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1452_llm_16_1452 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_u16_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 16u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 0u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 0u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_16, & rug_fuzz_17), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_18, & rug_fuzz_19), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_20, & rug_fuzz_21), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_22, & rug_fuzz_23), 1000u128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1453_llm_16_1453 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_pow_ref_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 16u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 100000u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 0u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_16, & rug_fuzz_17), 1u128);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1454_llm_16_1454 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u128, u8, u128, u8, u128, u8, u128, u8, u128, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 100);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 81);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(u128::MAX, & rug_fuzz_10), 1);
        debug_assert_eq!(
            < u128 as Pow < & u8 > > ::pow(u128::MAX, & rug_fuzz_11), u128::MAX
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1455_llm_16_1455 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_reference_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 10);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 100);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1456_llm_16_1456 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u128, u16, u16, u128, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(
            < u128 as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 100000
        );
        debug_assert_eq!(< u128 as Pow < u16 > > ::pow(u128::MAX, rug_fuzz_14), 1);
        debug_assert_eq!(
            < u128 as Pow < u16 > > ::pow(rug_fuzz_15, rug_fuzz_16),
            170141183460469231731687303715884105728
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1457_llm_16_1457 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_12, rug_fuzz_13), 4);
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(rug_fuzz_14, rug_fuzz_15), 8);
        debug_assert_eq!(
            < u128 as Pow < u32 > > ::pow(rug_fuzz_16, rug_fuzz_17), 10_000
        );
        debug_assert_eq!(
            < u128 as Pow < u32 > > ::pow(rug_fuzz_18, rug_fuzz_19), 100_000
        );
        debug_assert_eq!(< u128 as Pow < u32 > > ::pow(u128::MAX, rug_fuzz_20), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1458_llm_16_1458 {
    use crate::pow::Pow;
    #[test]
    fn u128_pow_u8() {
        assert_eq!(< u128 as Pow < u8 >>::pow(2, 4), 16);
        assert_eq!(< u128 as Pow < u8 >>::pow(0, 0), 1);
        assert_eq!(< u128 as Pow < u8 >>::pow(0, 1), 0);
        assert_eq!(< u128 as Pow < u8 >>::pow(1, 0), 1);
        assert_eq!(< u128 as Pow < u8 >>::pow(u128::MAX, 0), 1);
        assert_eq!(< u128 as Pow < u8 >>::pow(u128::MAX, 1), u128::MAX);
        assert_eq!(< u128 as Pow < u8 >>::pow(2, 127), 2u128.pow(127));
        #[should_panic(expected = "attempt to multiply with overflow")]
        fn pow_overflow() {
            <u128 as Pow<u8>>::pow(2, 128);
        }
        pow_overflow();
    }
}
#[cfg(test)]
mod tests_llm_16_1459_llm_16_1459 {
    use crate::pow::Pow;
    #[test]
    fn pow_usize_for_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize, u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u128 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 1u128);
        debug_assert_eq!(< u128 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 2u128);
        debug_assert_eq!(< u128 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 4u128);
        debug_assert_eq!(< u128 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 8u128);
        debug_assert_eq!(< u128 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1u128);
        debug_assert_eq!(
            < u128 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 0u128
        );
        debug_assert_eq!(
            < u128 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 0u128
        );
        debug_assert_eq!(
            < u128 as Pow < usize > > ::pow(rug_fuzz_14, rug_fuzz_15), 1u128
        );
        debug_assert_eq!(
            < u128 as Pow < usize > > ::pow(rug_fuzz_16, rug_fuzz_17), 1u128
        );
        debug_assert_eq!(
            < u128 as Pow < usize > > ::pow(rug_fuzz_18, rug_fuzz_19), 1u128
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1557_llm_16_1557 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 81);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1558_llm_16_1558 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1024);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1559_llm_16_1559 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u16_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u16, u8, u16, u8, u16, u8, u16, u8, u16, u8, u16, u8, u16, u8, u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 81);
        debug_assert_eq!(< u16 as Pow < & u8 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 125);
        debug_assert_eq!(
            < u16 as Pow < & u8 > > ::pow(rug_fuzz_14, & rug_fuzz_15), 1000
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1560_llm_16_1560 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u16, usize, u16, usize, u16, usize, u16, usize, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(
            < u16 as Pow < & usize > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1000
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1561 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(
            < u16 as pow::Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 100
        );
        debug_assert_eq!(
            < u16 as pow::Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 81
        );
        debug_assert_eq!(
            < u16 as pow::Pow < u16 > > ::pow(u16::MAX, rug_fuzz_14), u16::MAX
        );
        debug_assert_eq!(< u16 as pow::Pow < u16 > > ::pow(u16::MAX, rug_fuzz_15), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1562_llm_16_1562 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 100);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 81);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(u16::MAX, rug_fuzz_12), u16::MAX);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(u16::MAX, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1563_llm_16_1563 {
    use crate::pow::Pow;
    #[test]
    fn u16_pow_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u16, u8, u16, u8, u16, u8, u16, u8, u16, u8, u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 10);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1662_llm_16_1662 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1024);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 81);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(
            < u32 as Pow < & u16 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1000
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1663_llm_16_1663 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 9);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(
            < u32 as Pow < & u32 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 3125
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1664_llm_16_1664 {
    use crate::pow::Pow;
    #[test]
    fn test_u32_pow_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u8, u32, u8, u32, u8, u32, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 100);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1665_llm_16_1665 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 8);
             }
});    }
    #[test]
    fn test_pow_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 1);
             }
});    }
    #[test]
    fn test_pow_one_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 1);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: u32 = u32::MAX;
        let exp: usize = rug_fuzz_0;
        let _ = Pow::pow(base, &exp);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1666_llm_16_1666 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 1024);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_14, rug_fuzz_15), 65536);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_16, rug_fuzz_17), 3125);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1667_llm_16_1667 {
    use crate::pow::Pow;
    #[test]
    fn pow_for_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1668_llm_16_1668 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(u32, u8, u32, u8, u32, u8, u32, u8, u32, u8, u32, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 5u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 256u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 729u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_12), 1u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_13), u32::MAX);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_14), 1u32);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1669_llm_16_1669 {
    use crate::pow::Pow;
    #[test]
    fn u32_pow_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u32, usize, u32, usize, u32, usize, u32, usize, u32, usize, u32, usize, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1767_llm_16_1767 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u64_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u16, u64, u16, u64, u16, u64, u16, u64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: u64 = rug_fuzz_0;
        let y: u16 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(x, & y), 8);
        let x: u64 = rug_fuzz_2;
        let y: u16 = rug_fuzz_3;
        debug_assert_eq!(Pow::pow(x, & y), 100000);
        let x: u64 = rug_fuzz_4;
        let y: u16 = rug_fuzz_5;
        debug_assert_eq!(Pow::pow(x, & y), 1);
        let x: u64 = rug_fuzz_6;
        let y: u16 = rug_fuzz_7;
        debug_assert_eq!(Pow::pow(x, & y), 0);
        let x: u64 = rug_fuzz_8;
        let y: u16 = rug_fuzz_9;
        debug_assert_eq!(Pow::pow(x, & y), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1768_llm_16_1768 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        assert_eq!(< u64 as Pow <&'static u32 >>::pow(2, & 2), 4);
        assert_eq!(< u64 as Pow <&'static u32 >>::pow(3, & 3), 27);
        assert_eq!(< u64 as Pow <&'static u32 >>::pow(2, & 0), 1);
        assert_eq!(< u64 as Pow <&'static u32 >>::pow(0, & 2), 0);
    }
}
#[cfg(test)]
mod tests_llm_16_1769_llm_16_1769 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u64_by_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u8, u64, u8, u64, u8, u64, u8, u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1771_llm_16_1771 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16, u64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 1000);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_14, rug_fuzz_15), 1);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_16, rug_fuzz_17), 0);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_18, rug_fuzz_19), 1);
        debug_assert_eq!(< u64 as Pow < u16 > > ::pow(rug_fuzz_20, rug_fuzz_21), 1);
        debug_assert_eq!(
            < u64 as Pow < u16 > > ::pow(rug_fuzz_22, rug_fuzz_23), 1_000_000_000_000
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1772_llm_16_1772 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u32, u64, u32, u64, u32, u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 10);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1773_llm_16_1773 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u64, u8, u64, u8, u64, u8, u64, u8, u64, u8, u64, u8, u64, u8, u64, u8, u64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 9);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_12, rug_fuzz_13), 16);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_14, rug_fuzz_15), 125);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_16, rug_fuzz_17), 100000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1774_llm_16_1774 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, usize, u64, usize, u64, usize, u64, usize, u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 5);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1873_llm_16_1873 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u16, u8, u16, u8, u16, u8, u16, u8, u16, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 100u8);
        debug_assert_eq!(
            < u8 as Pow < & u16 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 81u8
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1875_llm_16_1875 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 9);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 2);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1876_llm_16_1876 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, usize, u8, usize, u8, usize, u8, usize, u8, usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 2);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 8);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_8, & rug_fuzz_9), 9);
        debug_assert_eq!(
            < u8 as Pow < & usize > > ::pow(rug_fuzz_10, & rug_fuzz_11), 27
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1878_llm_16_1878 {
    use crate::pow::Pow;
    #[test]
    fn u8_pow_u32() {
        let _rug_st_tests_llm_16_1878_llm_16_1878_rrrruuuugggg_u8_pow_u32 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 6;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 7;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 8;
        let rug_fuzz_18 = 3;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 3;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 3;
        let rug_fuzz_23 = 2;
        let rug_fuzz_24 = 3;
        let rug_fuzz_25 = 3;
        let rug_fuzz_26 = 3;
        let rug_fuzz_27 = 4;
        let rug_fuzz_28 = 3;
        let rug_fuzz_29 = 5;
        let rug_fuzz_30 = 6;
        let rug_fuzz_31 = 2;
        let rug_fuzz_32 = 6;
        let rug_fuzz_33 = 3;
        let rug_fuzz_34 = 6;
        let rug_fuzz_35 = 4;
        let rug_fuzz_36 = 10;
        let rug_fuzz_37 = 2;
        let rug_fuzz_38 = 10;
        let rug_fuzz_39 = 3;
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 32);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_12, rug_fuzz_13), 64);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_14, rug_fuzz_15), 128);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_16, rug_fuzz_17), 0);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_18, rug_fuzz_19), 1);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_20, rug_fuzz_21), 3);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_22, rug_fuzz_23), 9);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_24, rug_fuzz_25), 27);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_26, rug_fuzz_27), 81);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_28, rug_fuzz_29), 243);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_30, rug_fuzz_31), 36);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_32, rug_fuzz_33), 216);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_34, rug_fuzz_35), 0);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_36, rug_fuzz_37), 100);
        debug_assert_eq!(< u8 as Pow < u32 > > ::pow(rug_fuzz_38, rug_fuzz_39), 0);
        let _rug_ed_tests_llm_16_1878_llm_16_1878_rrrruuuugggg_u8_pow_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1879_llm_16_1879 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1978_llm_16_1978 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_with_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, u16, usize, u16, usize, u16, usize, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 7);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 81);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1980_llm_16_1980 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_with_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: usize = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, 8);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1982_llm_16_1982 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(usize, u16, usize, u16, usize, u16, usize, u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(
            < usize as Pow < u16 > > ::pow(rug_fuzz_8, u16::MAX), usize::pow(2, u16::MAX
            as u32)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1983_llm_16_1983 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(usize, u32, usize, u32, usize, u32, usize, u32, usize, u32, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1984_llm_16_1984 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(usize, u8, usize, u8, usize, u8, usize, u8, usize, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 100);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1985_llm_16_1985 {
    use crate::pow::Pow;
    #[test]
    fn pow_usize_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2052_llm_16_2052 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_float_impls() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, f32, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2053_llm_16_2053 {
    use crate::Pow;
    #[test]
    fn test_pow_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = rug_fuzz_0;
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2054_llm_16_2054 {
    use crate::Pow;
    use std::f64;
    use std::f32;
    #[test]
    fn test_pow_f64_with_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f64_with_f32_fractional() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = base.powf(exponent as f64);
        debug_assert!((result - expected).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f64_with_f32_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f64_with_f32_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f64_with_f32_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f32, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2055 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2056_llm_16_2056 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, base.powf(exponent));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2057_llm_16_2057 {
    use crate::Pow;
    use core::f32;
    #[test]
    fn pow_f32_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
    #[test]
    fn pow_f32_i16_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
    #[test]
    fn pow_f32_i16_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
    #[test]
    fn pow_f32_i16_base_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
    #[test]
    fn pow_f32_i16_base_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = -rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = -rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2058_llm_16_2058 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(f64, i16, f64, i16, f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result: f64 = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
        let base: f64 = rug_fuzz_2;
        let exponent: i16 = -rug_fuzz_3;
        let result: f64 = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 0.125);
        let base: f64 = rug_fuzz_4;
        let exponent: i16 = rug_fuzz_5;
        let result: f64 = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2059_llm_16_2059 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exp: i16 = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2060_llm_16_2060 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_f64_i16_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i16, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f64_i16_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_f64_i16_zero_base_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let _ = base.pow(&exponent);
             }
});    }
    #[test]
    fn test_pow_f64_i16_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
}
#[cfg(test)]
mod test {
    use crate::pow::Pow;
    #[test]
    fn test_pow_for_f32_ref_with_i32_ref() {
        let base: f32 = 2.0;
        let exponent: i32 = 3;
        let result = Pow::pow(&base, &exponent);
        assert_eq!(result, 8.0);
    }
}
#[cfg(test)]
mod tests_llm_16_2062_llm_16_2062 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_f64_neg_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 0.125);
             }
});    }
    #[test]
    fn test_pow_f64_zero_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_f64_i32_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
    #[test]
    fn test_pow_f64_i32_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_f64_i32_nan() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = f64::NAN;
        let exponent: i32 = rug_fuzz_0;
        let _result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2063_llm_16_2063 {
    use crate::pow::Pow;
    #[test]
    fn pow_i32_for_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2064 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_positive_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 8.0);
             }
});    }
    #[test]
    fn test_pow_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 1.0);
             }
});    }
    #[test]
    fn test_pow_negative_integer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 0.125);
             }
});    }
    #[test]
    fn test_pow_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 2.0);
             }
});    }
    #[test]
    fn test_pow_fractional_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 0.25);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_negative_base_integer_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = -rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let _ = Pow::pow(base, &exponent);
             }
});    }
    #[test]
    fn test_pow_large_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, i32, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert!(result > rug_fuzz_2 && result < rug_fuzz_3);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2065_llm_16_2065 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(f32, i8, f32, i8, f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(& base, & exponent), 8.0);
        let base: f32 = rug_fuzz_2;
        let exponent: i8 = -rug_fuzz_3;
        debug_assert_eq!(Pow::pow(& base, & exponent), 0.04);
        let base: f32 = rug_fuzz_4;
        let exponent: i8 = rug_fuzz_5;
        debug_assert_eq!(Pow::pow(& base, & exponent), 1.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2066_llm_16_2066 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_i8_f64_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let _ = Pow::pow(&base, &exponent);
             }
});    }
    #[test]
    fn test_pow_i8_f64_negative_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = -rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 0.125);
             }
});    }
    #[test]
    fn test_pow_i8_f64_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_i8_f64_one_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 2.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2067 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_f32_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i8, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected, "2.0 to the power of 3 should be 8.0");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2068_llm_16_2068 {
    use crate::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(f64, i8, f64, i8, f64, i8, f64, i8, f64, i8, f64, i8, f64, i8, f64, i8, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        debug_assert_eq!(base.pow(& exponent), 8.0);
        let base: f64 = rug_fuzz_2;
        let exponent: i8 = -rug_fuzz_3;
        debug_assert_eq!(base.pow(& exponent), 0.125);
        let base: f64 = rug_fuzz_4;
        let exponent: i8 = rug_fuzz_5;
        debug_assert_eq!(base.pow(& exponent), 1.0);
        let base: f64 = rug_fuzz_6;
        let exponent: i8 = rug_fuzz_7;
        debug_assert_eq!(base.pow(& exponent), 0.0);
        let base: f64 = rug_fuzz_8;
        let exponent: i8 = -rug_fuzz_9;
        debug_assert!(base.pow(& exponent).is_infinite());
        let base: f64 = -rug_fuzz_10;
        let exponent: i8 = rug_fuzz_11;
        debug_assert_eq!(base.pow(& exponent), - 8.0);
        let base: f64 = -rug_fuzz_12;
        let exponent: i8 = rug_fuzz_13;
        debug_assert_eq!(base.pow(& exponent), 4.0);
        let base: f64 = rug_fuzz_14;
        let exponent: i8 = rug_fuzz_15;
        debug_assert_eq!(base.pow(& exponent), 2.0);
        let base: f64 = rug_fuzz_16;
        let exponent: i8 = i8::MIN;
        debug_assert_eq!(base.pow(& exponent), 0.0);
        let base: f64 = rug_fuzz_17;
        let exponent: i8 = i8::MAX;
        debug_assert_eq!(base.pow(& exponent), 1.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2070_llm_16_2070 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 32.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2071_llm_16_2071 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_with_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2072_llm_16_2072 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_by_ref_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, 16.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2073_llm_16_2073 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2074_llm_16_2074 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_ref_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2076_llm_16_2076 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = f64::powi(base, exp.into());
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2077_llm_16_2077 {
    use crate::Pow;
    #[test]
    fn test_pow_f32_ref_with_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_f32_ref_with_negative_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.25);
             }
});    }
    #[test]
    fn test_pow_f32_ref_with_zero_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_f32_ref_with_one_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 2.0);
             }
});    }
    #[test]
    fn test_pow_f32_ref_with_fractional_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f32, f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < rug_fuzz_3);
             }
});    }
    #[test]
    fn test_pow_f32_ref_with_large_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1024.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2079_llm_16_2079 {
    use crate::Pow;
    #[test]
    fn pow_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2080_llm_16_2080 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, f32, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = <f64 as Pow<f32>>::pow(base, exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2081_llm_16_2081 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2082_llm_16_2082 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_f64_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_f64_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 2.0);
             }
});    }
    #[test]
    fn test_pow_f64_fraction() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 2.0);
             }
});    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero or for a divisor that does not fit into an i32"
    )]
    fn test_pow_f64_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exp: f64 = -rug_fuzz_1;
        let _result: f64 = base.pow(exp);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2083_llm_16_2083 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(f32, i16, f32, i16, f32, f32, f32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
        let base: f32 = rug_fuzz_2;
        let exponent: i16 = -rug_fuzz_3;
        let result = Pow::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_4 / rug_fuzz_5).abs() < f32::EPSILON);
        let base: f32 = rug_fuzz_6;
        let exponent: i16 = rug_fuzz_7;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_f32_i16_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let _ = Pow::pow(&base, exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2084_llm_16_2084 {
    use crate::Pow;
    #[test]
    fn test_pow_for_f64_with_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_for_f64_with_negative_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i16, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
             }
});    }
    #[test]
    fn test_pow_for_f64_with_zero_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_for_zero_f64_with_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_pow_for_zero_f64_with_negative_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let _result = Pow::pow(&base, exponent);
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_pow_for_zero_f64_with_zero_i16_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let _result = Pow::pow(&base, exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2085_llm_16_2085 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(f32, i16, f32, f32, i16, f32, f32, i16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let base: f32 = rug_fuzz_3;
        let exponent: i16 = -rug_fuzz_4;
        let result = base.pow(exponent);
        let expected = rug_fuzz_5;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let base: f32 = rug_fuzz_6;
        let exponent: i16 = rug_fuzz_7;
        let result = base.pow(exponent);
        let expected = rug_fuzz_8;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2086_llm_16_2086 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i16, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2087_llm_16_2087 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_positive_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_negative_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_special_case_nan() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &f32::NAN;
        let exponent: i32 = rug_fuzz_0;
        let _ = base.pow(exponent);
             }
});    }
    #[test]
    fn test_pow_special_case_infinity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &f32::INFINITY;
        let exponent: i32 = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, f32::INFINITY);
             }
});    }
    #[test]
    fn test_pow_special_case_negative_infinity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &f32::NEG_INFINITY;
        let exponent: i32 = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, f32::NEG_INFINITY);
             }
});    }
    #[test]
    fn test_pow_special_case_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_pow_special_case_zero_negative_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let _ = base.pow(exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2088_llm_16_2088 {
    use crate::pow::Pow;
    #[test]
    fn pow_f64_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
        let exponent: i32 = rug_fuzz_2;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let exponent: i32 = -rug_fuzz_3;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.25);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2090 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_positive_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_zero_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_negative_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.125);
             }
});    }
    #[test]
    fn test_pow_one_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 2.0);
             }
});    }
    #[test]
    fn test_pow_one_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
    #[test]
    fn test_pow_large_exponent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1073741824.0);
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_pow_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i32 = i32::MAX;
        let _result = base.pow(exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2091_llm_16_2091 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_for_f32_reference() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(f32, i8, f32, i8, f32, f32, i8, f32, i8, f32, i8, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = <&f32 as Pow<i8>>::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
        let base: f32 = rug_fuzz_2;
        let exponent: i8 = -rug_fuzz_3;
        let result = <&f32 as Pow<i8>>::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_4).abs() < f32::EPSILON);
        let base: f32 = rug_fuzz_5;
        let exponent: i8 = rug_fuzz_6;
        let result = <&f32 as Pow<i8>>::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
        let base: f32 = rug_fuzz_7;
        let exponent: i8 = rug_fuzz_8;
        let result = <&f32 as Pow<i8>>::pow(&base, exponent);
        debug_assert_eq!(result, 0.0);
        let base: f32 = rug_fuzz_9;
        let exponent: i8 = -rug_fuzz_10;
        let result = <&f32 as Pow<i8>>::pow(&base, exponent);
        let expected: f32 = rug_fuzz_11 / rug_fuzz_12;
        debug_assert!((result - expected).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2092_llm_16_2092 {
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_f64_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, i8, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2093_llm_16_2093 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn test_pow_f32_negative_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, i8, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
             }
});    }
    #[test]
    fn test_pow_f32_i8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_f32_i8_zero_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
             }
});    }
    #[test]
    fn test_pow_f32_i8_one_base() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn test_pow_f32_i8_negative_base_even() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = -rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 16.0);
             }
});    }
    #[test]
    fn test_pow_f32_i8_negative_base_odd() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = -rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, - 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2096_llm_16_2096 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2097_llm_16_2097 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, u16, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f32 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2098_llm_16_2098 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_with_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, u16, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2100_llm_16_2100 {
    use crate::pow::Pow;
    #[test]
    fn pow_f64_by_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
             }
});    }
    #[test]
    fn pow_f64_by_u8_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
             }
});    }
    #[test]
    fn pow_f64_by_u8_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, base);
             }
});    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn pow_f64_by_u8_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&(-base), exponent);
        debug_assert_eq!(result, - 8.0);
             }
});    }
    #[test]
    fn pow_f64_by_u8_fraction() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, u8, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&(rug_fuzz_2 / base), exponent);
        debug_assert_eq!(result, 1.0 / base);
             }
});    }
    #[test]
    #[should_panic]
    fn pow_f64_by_u8_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = f64::MAX;
        let exponent: u8 = rug_fuzz_0;
        let _result = Pow::pow(&base, exponent);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2102_llm_16_2102 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, u8, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2103_llm_16_2103 {
    use crate::pow;
    use crate::identities::One;
    use crate::pow::Pow;
    use std::num::Wrapping;
    use std::ops::Mul;
    #[test]
    fn test_pow_with_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, usize, u32, usize, u32, usize, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = pow(base, exp);
        debug_assert_eq!(result, Wrapping(81));
        let base = Wrapping(rug_fuzz_2);
        let exp = rug_fuzz_3;
        let result = pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(rug_fuzz_4);
        let exp = rug_fuzz_5;
        let result = pow(base, exp);
        debug_assert_eq!(result, Wrapping(1));
        let base = Wrapping(rug_fuzz_6);
        let exp = rug_fuzz_7;
        let result = pow(base, exp);
        debug_assert_eq!(result, Wrapping(32));
             }
});    }
}
