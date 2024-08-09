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
        let _rug_st_tests_llm_16_4_llm_16_4_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i128;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 2i128;
        let rug_fuzz_5 = 2u32;
        let rug_fuzz_6 = 10i128;
        let rug_fuzz_7 = 5u32;
        let rug_fuzz_8 = 0i128;
        let rug_fuzz_9 = 0u32;
        let rug_fuzz_10 = 0i128;
        let rug_fuzz_11 = 5u32;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 100_000);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 0);
        let _rug_ed_tests_llm_16_4_llm_16_4_rrrruuuugggg_test_pow = 0;
    }
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
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 0i16;
        let rug_fuzz_3 = 0u16;
        let rug_fuzz_4 = 2i16;
        let rug_fuzz_5 = 3u16;
        let rug_fuzz_6 = 2i16;
        let rug_fuzz_7 = 2u16;
        let rug_fuzz_8 = 1i16;
        let rug_fuzz_9 = 100u16;
        let rug_fuzz_10 = 2i16;
        let rug_fuzz_11 = 0u16;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8_llm_16_8 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16() {
        let _rug_st_tests_llm_16_8_llm_16_8_rrrruuuugggg_test_pow_i16 = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 0i16;
        let rug_fuzz_5 = 0u32;
        let rug_fuzz_6 = 0i16;
        let rug_fuzz_7 = 1u32;
        let rug_fuzz_8 = 1i16;
        let rug_fuzz_9 = 0u32;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_8_llm_16_8_rrrruuuugggg_test_pow_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9_llm_16_9 {
    use crate::pow::Pow;
    #[test]
    fn pow_i16_u8() {
        let _rug_st_tests_llm_16_9_llm_16_9_rrrruuuugggg_pow_i16_u8 = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 0i16;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 2i16;
        let rug_fuzz_5 = 2u8;
        let rug_fuzz_6 = 2i16;
        let rug_fuzz_7 = 3u8;
        let rug_fuzz_8 = 1i16;
        let rug_fuzz_9 = 8u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_9_llm_16_9_rrrruuuugggg_pow_i16_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &i32 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&i32 as Pow<u16>>::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_12_llm_16_12 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_12_llm_16_12_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0_u32;
        let rug_fuzz_2 = 1_u32;
        let rug_fuzz_3 = 2_u32;
        let rug_fuzz_4 = 3_u32;
        let rug_fuzz_5 = 4_u32;
        let base: &i32 = &rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_2), 2);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_3), 4);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_4), 8);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_5), 16);
        let _rug_ed_tests_llm_16_12_llm_16_12_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_13_llm_16_13 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3i32;
        let rug_fuzz_1 = 2u8;
        let rug_fuzz_2 = 2i32;
        let rug_fuzz_3 = 5u8;
        let rug_fuzz_4 = 2i32;
        let rug_fuzz_5 = 3u8;
        let rug_fuzz_6 = 0i32;
        let rug_fuzz_7 = 0u8;
        let rug_fuzz_8 = 0i32;
        let rug_fuzz_9 = 1u8;
        let rug_fuzz_10 = 1i32;
        let rug_fuzz_11 = 0u8;
        let rug_fuzz_12 = 1i32;
        let rug_fuzz_13 = 100u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 32);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_12, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_13_llm_16_13_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_15_llm_16_15 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_with_u16() {
        let _rug_st_tests_llm_16_15_llm_16_15_rrrruuuugggg_pow_i64_with_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: &i64 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&i64 as Pow<u16>>::pow(base, exponent);
        debug_assert_eq!(result, 16);
        let _rug_ed_tests_llm_16_15_llm_16_15_rrrruuuugggg_pow_i64_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_16_llm_16_16 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_16_llm_16_16_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 0u32;
        let rug_fuzz_6 = 0i64;
        let rug_fuzz_7 = 3u32;
        let rug_fuzz_8 = 0i64;
        let rug_fuzz_9 = 0u32;
        let rug_fuzz_10 = 10i64;
        let rug_fuzz_11 = 1u32;
        let rug_fuzz_12 = 1i64;
        let rug_fuzz_13 = 100u32;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 10);
        debug_assert_eq!(Pow::pow(& rug_fuzz_12, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_16_llm_16_16_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_17_llm_16_17 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 3i64;
        let rug_fuzz_3 = 2u8;
        let rug_fuzz_4 = 0i64;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0i64;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 1i64;
        let rug_fuzz_9 = 0u8;
        let rug_fuzz_10 = 1i64;
        let rug_fuzz_11 = 1u8;
        let rug_fuzz_12 = 1i64;
        let rug_fuzz_13 = 2u8;
        let rug_fuzz_14 = 2i64;
        let rug_fuzz_15 = 0u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_10, rug_fuzz_11), - 1);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_14, rug_fuzz_15), 1);
        let _rug_ed_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_19_llm_16_19 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_19_llm_16_19_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &i8 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_19_llm_16_19_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_20_llm_16_20 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_u32() {
        let _rug_st_tests_llm_16_20_llm_16_20_rrrruuuugggg_pow_i8_u32 = 0;
        let rug_fuzz_0 = 2i8;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 2i8;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 2i8;
        let rug_fuzz_5 = 0u32;
        let rug_fuzz_6 = 0i8;
        let rug_fuzz_7 = 0u32;
        let rug_fuzz_8 = 0i8;
        let rug_fuzz_9 = 3u32;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8i8);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, rug_fuzz_3), - 8i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1i8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0i8);
        let _rug_ed_tests_llm_16_20_llm_16_20_rrrruuuugggg_pow_i8_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_21_llm_16_21 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_21_llm_16_21_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 8;
        let rug_fuzz_14 = 0;
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
        let _rug_ed_tests_llm_16_21_llm_16_21_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_23_llm_16_23 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_23_llm_16_23_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: isize = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_23_llm_16_23_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_24_llm_16_24 {
    use crate::pow::Pow;
    #[test]
    fn pow_test() {
        let _rug_st_tests_llm_16_24_llm_16_24_rrrruuuugggg_pow_test = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 100;
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
        let _rug_ed_tests_llm_16_24_llm_16_24_rrrruuuugggg_pow_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_25_llm_16_25 {
    use crate::Pow;
    #[test]
    fn pow_u8_test() {
        let _rug_st_tests_llm_16_25_llm_16_25_rrrruuuugggg_pow_u8_test = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0_u8;
        let rug_fuzz_2 = 1_u8;
        let rug_fuzz_3 = 2_u8;
        let rug_fuzz_4 = 3_u8;
        let rug_fuzz_5 = 4_u8;
        let base: &isize = &rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_2), 2);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_3), 4);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_4), 8);
        debug_assert_eq!(Pow::pow(base, rug_fuzz_5), 16);
        let _rug_ed_tests_llm_16_25_llm_16_25_rrrruuuugggg_pow_u8_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_26_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: isize = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(& base, exponent), 8);
        let _rug_ed_tests_llm_16_26_rrrruuuugggg_test_pow = 0;
    }
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
        let _rug_st_tests_llm_16_31_llm_16_31_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 4u8;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8i32));
        let exponent_ref = &rug_fuzz_2;
        let result_ref = base.pow(exponent_ref);
        debug_assert_eq!(result_ref, Wrapping(16i32));
        let _rug_ed_tests_llm_16_31_llm_16_31_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(256));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping = 0;
    }
    #[test]
    fn test_pow_wrapping_by_reference() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_by_reference = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(256));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_by_reference = 0;
    }
    #[test]
    fn test_pow_wrapping_zero_exponent() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_zero_exponent = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_zero_exponent = 0;
    }
    #[test]
    fn test_pow_wrapping_one_base() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_one_base = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 8;
        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_one_base = 0;
    }
    #[test]
    fn test_pow_wrapping_zero_base() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_zero_base = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 8;
        let base: Wrapping<i64> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_zero_base = 0;
    }
    #[test]
    fn test_pow_wrapping_negative_base() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_negative_base = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 5;
        let base: Wrapping<i64> = Wrapping(-rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(- 32));
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_negative_base = 0;
    }
    #[test]
    fn test_pow_wrapping_overflow() {
        let _rug_st_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_overflow = 0;
        let rug_fuzz_0 = 2;
        let base: Wrapping<i64> = Wrapping(i64::MAX);
        let exponent: u8 = rug_fuzz_0;
        let result = base.pow(exponent);
        let _rug_ed_tests_llm_16_33_llm_16_33_rrrruuuugggg_test_pow_wrapping_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34_llm_16_34 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_34_llm_16_34_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 8i64;
        let rug_fuzz_3 = 2i64;
        let rug_fuzz_4 = 3usize;
        let rug_fuzz_5 = 8i64;
        let rug_fuzz_6 = 0i64;
        let rug_fuzz_7 = 0usize;
        let rug_fuzz_8 = 1i64;
        let rug_fuzz_9 = 1i64;
        let rug_fuzz_10 = 10usize;
        let rug_fuzz_11 = 1i64;
        let rug_fuzz_12 = 1i64;
        let rug_fuzz_13 = 2usize;
        let rug_fuzz_14 = 1i64;
        let rug_fuzz_15 = 1i64;
        let rug_fuzz_16 = 3usize;
        let rug_fuzz_17 = 1i64;
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
        let _rug_ed_tests_llm_16_34_llm_16_34_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_isize() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8isize;
        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize = 0;
    }
    #[test]
    fn test_pow_wrapping_isize_ref() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_ref = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8isize;
        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_ref = 0;
    }
    #[test]
    fn test_pow_wrapping_isize_zero() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_zero = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1isize;
        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_isize_by_zero_ref() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_by_zero_ref = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1isize;
        let base = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_by_zero_ref = 0;
    }
    #[test]
    fn test_pow_wrapping_isize_overflow() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_overflow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 1;
        let base = Wrapping(isize::MAX);
        let exp: u8 = rug_fuzz_0;
        let result = base.pow(exp);
        let expected = Wrapping(rug_fuzz_1);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_pow_wrapping_isize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_38_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 3isize;
        let rug_fuzz_3 = 4usize;
        let rug_fuzz_4 = 0isize;
        let rug_fuzz_5 = 10usize;
        let rug_fuzz_6 = 10isize;
        let rug_fuzz_7 = 0usize;
        let rug_fuzz_8 = 10isize;
        let rug_fuzz_9 = 1usize;
        let rug_fuzz_10 = 5usize;
        let rug_fuzz_11 = 3isize;
        let rug_fuzz_12 = 10usize;
        let rug_fuzz_13 = 2isize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(8isize));
        debug_assert_eq!(Wrapping(rug_fuzz_2).pow(rug_fuzz_3), Wrapping(81isize));
        debug_assert_eq!(Wrapping(rug_fuzz_4).pow(rug_fuzz_5), Wrapping(0isize));
        debug_assert_eq!(Wrapping(rug_fuzz_6).pow(rug_fuzz_7), Wrapping(1isize));
        debug_assert_eq!(Wrapping(rug_fuzz_8).pow(rug_fuzz_9), Wrapping(10isize));
        let exp_ref = &rug_fuzz_10;
        debug_assert_eq!(Wrapping(rug_fuzz_11).pow(exp_ref), Wrapping(243isize));
        let exp_ref = &rug_fuzz_12;
        debug_assert_eq!(Wrapping(rug_fuzz_13).pow(exp_ref), Wrapping(1024isize));
        let _rug_ed_tests_llm_16_38_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128() {
        let _rug_st_tests_llm_16_39_llm_16_39_rrrruuuugggg_test_pow_wrapping_u128 = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 1u8;
        let rug_fuzz_5 = 1u128;
        let rug_fuzz_6 = 0u8;
        let rug_fuzz_7 = 0u128;
        let rug_fuzz_8 = 10u8;
        let rug_fuzz_9 = 0u8;
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
        let _rug_ed_tests_llm_16_39_llm_16_39_rrrruuuugggg_test_pow_wrapping_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_41_llm_16_41 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_41_llm_16_41_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 4u8;
        let rug_fuzz_2 = 16u16;
        let rug_fuzz_3 = 5u16;
        let rug_fuzz_4 = 3u8;
        let rug_fuzz_5 = 125u16;
        let rug_fuzz_6 = 2u16;
        let rug_fuzz_7 = 0u8;
        let rug_fuzz_8 = 1u16;
        let rug_fuzz_9 = 2u16;
        let rug_fuzz_10 = 1u8;
        let rug_fuzz_11 = 2u16;
        let rug_fuzz_12 = 3u16;
        let rug_fuzz_13 = 2u8;
        let rug_fuzz_14 = 9u16;
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
        let _rug_ed_tests_llm_16_41_llm_16_41_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_42_llm_16_42 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 4usize;
        let rug_fuzz_2 = 3u16;
        let rug_fuzz_3 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = <Wrapping<u16> as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(16u16));
        let base = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let result = <Wrapping<u16> as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(27u16));
        let _rug_ed_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_pow = 0;
    }
    #[test]
    fn test_pow_ref() {
        let _rug_st_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_pow_ref = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 4usize;
        let rug_fuzz_2 = 3u16;
        let rug_fuzz_3 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = <Wrapping<u16> as Pow<&usize>>::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(16u16));
        let base = Wrapping(rug_fuzz_2);
        let exponent = rug_fuzz_3;
        let result = <Wrapping<u16> as Pow<&usize>>::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(27u16));
        let _rug_ed_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_pow_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use std::num::Wrapping;
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 3;
        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert_eq!(result, Wrapping(8u32));
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44_llm_16_44 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    use crate::identities::{One, Zero};
    #[test]
    fn test_pow_wrapping_base_one() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_base_one = 0;
        let rug_fuzz_0 = 5;
        let base: Wrapping<u32> = Wrapping::one();
        let exponent: usize = rug_fuzz_0;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(1u32)
        );
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_base_one = 0;
    }
    #[test]
    fn test_pow_wrapping_base_zero() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_base_zero = 0;
        let rug_fuzz_0 = 5;
        let base: Wrapping<u32> = Wrapping::zero();
        let exponent: usize = rug_fuzz_0;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(0u32)
        );
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_base_zero = 0;
    }
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 5;
        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(32u32)
        );
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping = 0;
    }
    #[test]
    fn test_pow_wrapping_with_reference_exponent() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_with_reference_exponent = 0;
        let rug_fuzz_0 = 3u32;
        let rug_fuzz_1 = 3;
        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let exponent_ref: &usize = &exponent;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < & usize > > ::pow(base, exponent_ref),
            Wrapping(27u32)
        );
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_with_reference_exponent = 0;
    }
    #[test]
    fn test_pow_wrapping_zero_exponent() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_zero_exponent = 0;
        let rug_fuzz_0 = 5u32;
        let rug_fuzz_1 = 0;
        let base: Wrapping<u32> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u32 > as Pow < usize > > ::pow(base, exponent), Wrapping(1u32)
        );
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_zero_exponent = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_pow_wrapping_overflow() {
        let _rug_st_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_overflow = 0;
        let rug_fuzz_0 = 2;
        let base: Wrapping<u32> = Wrapping(u32::max_value());
        let exponent: usize = rug_fuzz_0;
        let _ = <Wrapping<u32> as Pow<usize>>::pow(base, exponent);
        let _rug_ed_tests_llm_16_44_llm_16_44_rrrruuuugggg_test_pow_wrapping_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45_llm_16_45 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow_wrapping_u64() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64 = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 5u8;
        let rug_fuzz_2 = 2u64;
        let rug_fuzz_3 = 5u32;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(Pow::pow(base, exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64 = 0;
    }
    #[test]
    fn test_pow_wrapping_u64_by_ref() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 5u8;
        let rug_fuzz_2 = 2u64;
        let rug_fuzz_3 = 5u32;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(Pow::pow(base, & exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref = 0;
    }
    #[test]
    fn test_pow_wrapping_u64_zero() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_zero = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 1u64;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_u64_by_ref_zero() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref_zero = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 1u64;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, & exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_u64_one() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_one = 0;
        let rug_fuzz_0 = 1u64;
        let rug_fuzz_1 = 8u8;
        let rug_fuzz_2 = 1u64;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_one = 0;
    }
    #[test]
    fn test_pow_wrapping_u64_by_ref_one() {
        let _rug_st_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref_one = 0;
        let rug_fuzz_0 = 1u64;
        let rug_fuzz_1 = 8u8;
        let rug_fuzz_2 = 1u64;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, & exponent), result);
        let _rug_ed_tests_llm_16_45_llm_16_45_rrrruuuugggg_test_pow_wrapping_u64_by_ref_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_46_llm_16_46 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3u64;
        let rug_fuzz_1 = 4usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(3u64.pow(4)));
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow = 0;
    }
    #[test]
    fn test_pow_ref() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_ref = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = &rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(2u64.pow(3)));
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_ref = 0;
    }
    #[test]
    fn test_pow_zero() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_zero = 0;
        let rug_fuzz_0 = 7u64;
        let rug_fuzz_1 = 0usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(1u64));
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_zero = 0;
    }
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2usize;
        let base = Wrapping(u64::MAX);
        let exp = rug_fuzz_0;
        let result = Pow::pow(base, exp);
        debug_assert_eq!(result, Wrapping(u64::MAX.wrapping_mul(u64::MAX)));
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_49_llm_16_49 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_wrapping_usize() {
        let _rug_st_tests_llm_16_49_llm_16_49_rrrruuuugggg_pow_wrapping_usize = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 8usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent), expected);
        let _rug_ed_tests_llm_16_49_llm_16_49_rrrruuuugggg_pow_wrapping_usize = 0;
    }
    #[test]
    fn pow_wrapping_usize_ref() {
        let _rug_st_tests_llm_16_49_llm_16_49_rrrruuuugggg_pow_wrapping_usize_ref = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 8usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let exponent_ref = &exponent;
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Pow::pow(base, exponent_ref), expected);
        let _rug_ed_tests_llm_16_49_llm_16_49_rrrruuuugggg_pow_wrapping_usize_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_usize() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_pow_wrapping_usize = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8usize));
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_pow_wrapping_usize = 0;
    }
    #[test]
    fn test_pow_wrapping_usize_reference() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_pow_wrapping_usize_reference = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(8usize));
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_pow_wrapping_usize_reference = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_51_llm_16_51 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_51_llm_16_51_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: u128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u128 as Pow<u16>>::pow(&base, exponent);
        debug_assert_eq!(result, 16);
        let _rug_ed_tests_llm_16_51_llm_16_51_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_53_llm_16_53 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_53_llm_16_53_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let x: &u128 = &rug_fuzz_0;
        let y: u8 = rug_fuzz_1;
        let result = <&u128 as Pow<u8>>::pow(x, y);
        debug_assert_eq!(result, 256);
        let _rug_ed_tests_llm_16_53_llm_16_53_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_55_llm_16_55 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_55_llm_16_55_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &u16 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u16);
        let _rug_ed_tests_llm_16_55_llm_16_55_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_56_llm_16_56 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_56_llm_16_56_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &u16 = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result: u16 = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_56_llm_16_56_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_57_llm_16_57 {
    use crate::pow::Pow;
    #[test]
    fn pow_u16_by_u8() {
        let _rug_st_tests_llm_16_57_llm_16_57_rrrruuuugggg_pow_u16_by_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let base: u16 = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exp);
        debug_assert_eq!(result, 256u16);
        let _rug_ed_tests_llm_16_57_llm_16_57_rrrruuuugggg_pow_u16_by_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_58_llm_16_58 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_58_llm_16_58_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &u16 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u16 as Pow<usize>>::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_58_llm_16_58_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_59_llm_16_59 {
    use crate::pow::Pow;
    #[test]
    fn test_u32_pow_u16() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_u32_pow_u16 = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 2u16;
        let rug_fuzz_2 = 3u32;
        let rug_fuzz_3 = 3u16;
        let rug_fuzz_4 = 4u32;
        let rug_fuzz_5 = 4u16;
        let rug_fuzz_6 = 10u32;
        let rug_fuzz_7 = 0u16;
        let rug_fuzz_8 = 0u32;
        let rug_fuzz_9 = 10u16;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 4);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 256);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 0);
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_u32_pow_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_61_llm_16_61 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_61_llm_16_61_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let base: &u32 = &rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&u32 as Pow<u8>>::pow(base, exponent);
        debug_assert_eq!(result, 1000);
        let _rug_ed_tests_llm_16_61_llm_16_61_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_62_llm_16_62 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_62_llm_16_62_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let base: &u32 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 81);
        let _rug_ed_tests_llm_16_62_llm_16_62_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_63_llm_16_63 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_63_llm_16_63_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let base: &u64 = &rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 1024u64);
        let _rug_ed_tests_llm_16_63_llm_16_63_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_64_llm_16_64 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: &u64 = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&u64 as Pow<u32>>::pow(base, exponent);
        debug_assert_eq!(result, 16u64);
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_65_llm_16_65 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_65_llm_16_65_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let base: &u64 = &rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 100);
        let _rug_ed_tests_llm_16_65_llm_16_65_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_66_llm_16_66 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_66_llm_16_66_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &u64 = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u64);
        let _rug_ed_tests_llm_16_66_llm_16_66_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_68_llm_16_68 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_68_llm_16_68_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u8 = rug_fuzz_0;
        let exp: u32 = rug_fuzz_1;
        let result = <&u8 as Pow<u32>>::pow(&base, exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_68_llm_16_68_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u8 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&u8 as Pow<u8>>::pow(&base, exponent);
        debug_assert_eq!(result, 8u8);
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_70_llm_16_70 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_70_llm_16_70_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 3u8;
        let rug_fuzz_3 = 2usize;
        let rug_fuzz_4 = 0u8;
        let rug_fuzz_5 = 0usize;
        let rug_fuzz_6 = 0u8;
        let rug_fuzz_7 = 1usize;
        let rug_fuzz_8 = 1u8;
        let rug_fuzz_9 = 0usize;
        let rug_fuzz_10 = 10u8;
        let rug_fuzz_11 = 1usize;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 10);
        let _rug_ed_tests_llm_16_70_llm_16_70_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_72_llm_16_72 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_72_llm_16_72_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &usize = &rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_72_llm_16_72_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_73_llm_16_73 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u8() {
        let _rug_st_tests_llm_16_73_llm_16_73_rrrruuuugggg_test_pow_usize_u8 = 0;
        let rug_fuzz_0 = 3_usize;
        let rug_fuzz_1 = 2_u8;
        let rug_fuzz_2 = 2_usize;
        let rug_fuzz_3 = 5_u8;
        let rug_fuzz_4 = 0_usize;
        let rug_fuzz_5 = 0_u8;
        let rug_fuzz_6 = 0_usize;
        let rug_fuzz_7 = 1_u8;
        let rug_fuzz_8 = 1_usize;
        let rug_fuzz_9 = 0_u8;
        let rug_fuzz_10 = 10_usize;
        let rug_fuzz_11 = 3_u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, rug_fuzz_3), 32);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, rug_fuzz_11), 1000);
        let _rug_ed_tests_llm_16_73_llm_16_73_rrrruuuugggg_test_pow_usize_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_74_llm_16_74 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_74_llm_16_74_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8;
        let base: &usize = &rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result: usize = Pow::pow(base, exponent);
        let expected: usize = rug_fuzz_2;
        debug_assert_eq!(result, expected, "Testing 2^3");
        let _rug_ed_tests_llm_16_74_llm_16_74_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_75_llm_16_75 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_75_llm_16_75_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let base: i128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 81);
        let _rug_ed_tests_llm_16_75_llm_16_75_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_76_llm_16_76 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_u32() {
        let _rug_st_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_pow_i128_u32 = 0;
        let rug_fuzz_0 = 3i128;
        let rug_fuzz_1 = 2u32;
        let rug_fuzz_2 = 3i128;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 2i128;
        let rug_fuzz_5 = 0u32;
        let rug_fuzz_6 = 0i128;
        let rug_fuzz_7 = 2u32;
        let rug_fuzz_8 = 0i128;
        let rug_fuzz_9 = 0u32;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 9);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, & rug_fuzz_3), - 27);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, & rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_pow_i128_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_77_llm_16_77 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_ref_u8() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8 = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 81);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8 = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_i128_with_ref_u8_overflow() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_overflow = 0;
        let rug_fuzz_0 = 2;
        let base: i128 = i128::MAX;
        let exp: &u8 = &rug_fuzz_0;
        let _ = <&i128 as Pow<&u8>>::pow(&base, exp);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_overflow = 0;
    }
    #[test]
    fn test_pow_i128_with_ref_u8_zero() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_zero = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 0;
        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 1);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_zero = 0;
    }
    #[test]
    fn test_pow_i128_with_ref_u8_zero_base() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_zero_base = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 4;
        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 0);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_zero_base = 0;
    }
    #[test]
    fn test_pow_i128_with_ref_u8_one() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_one = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 4;
        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert_eq!(result, 1);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_one = 0;
    }
    #[test]
    fn test_pow_i128_with_ref_u8_large_exp() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_large_exp = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 127;
        let rug_fuzz_2 = 0;
        let base: i128 = rug_fuzz_0;
        let exp: &u8 = &rug_fuzz_1;
        let result = <&i128 as Pow<&u8>>::pow(&base, exp);
        debug_assert!(
            result > rug_fuzz_2, "power with large exponent should not overflow i128"
        );
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_pow_i128_with_ref_u8_large_exp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_78_llm_16_78 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_usize_ref() {
        let _rug_st_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_pow_i128_with_usize_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i128 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&i128 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_78_llm_16_78_rrrruuuugggg_test_pow_i128_with_usize_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_79_llm_16_79 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u16() {
        let _rug_st_tests_llm_16_79_llm_16_79_rrrruuuugggg_test_pow_i16_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i16 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_79_llm_16_79_rrrruuuugggg_test_pow_i16_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_80_llm_16_80 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u32() {
        let _rug_st_tests_llm_16_80_llm_16_80_rrrruuuugggg_test_pow_i16_u32 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i16 = rug_fuzz_0;
        let exp: u32 = rug_fuzz_1;
        let result = <&i16 as Pow<&u32>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_80_llm_16_80_rrrruuuugggg_test_pow_i16_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_81_llm_16_81 {
    use crate::Pow;
    #[test]
    fn test_pow_i16_u8_ref() {
        let _rug_st_tests_llm_16_81_llm_16_81_rrrruuuugggg_test_pow_i16_u8_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
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
        let _rug_ed_tests_llm_16_81_llm_16_81_rrrruuuugggg_test_pow_i16_u8_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_82_llm_16_82 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_82_llm_16_82_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i16 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&i16 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_82_llm_16_82_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_84_llm_16_84 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_84_llm_16_84_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i32 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i32 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_84_llm_16_84_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_85_llm_16_85 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i32 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = <&i32 as Pow<&u8>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_86_llm_16_86 {
    use crate::pow::Pow;
    #[test]
    fn pow_i32_usize() {
        let _rug_st_tests_llm_16_86_llm_16_86_rrrruuuugggg_pow_i32_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i32 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result: i32 = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_86_llm_16_86_rrrruuuugggg_pow_i32_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_87_llm_16_87 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_87_llm_16_87_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_87_llm_16_87_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_88_llm_16_88 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_88_llm_16_88_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i64 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i64 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_88_llm_16_88_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_u8() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_pow_i64_u8 = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0i64;
        let rug_fuzz_7 = 0u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 8i64);
        debug_assert_eq!(Pow::pow(& - rug_fuzz_2, & rug_fuzz_3), - 8i64);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1i64);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 1i64);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_pow_i64_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_90_llm_16_90 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_90_llm_16_90_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i64 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_90_llm_16_90_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_91_llm_16_91 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_u16() {
        let _rug_st_tests_llm_16_91_llm_16_91_rrrruuuugggg_test_pow_i8_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 3;
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
        let _rug_ed_tests_llm_16_91_llm_16_91_rrrruuuugggg_test_pow_i8_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_92_llm_16_92 {
    use crate::Pow;
    #[test]
    fn pow_i8_ref_with_u32_ref() {
        let _rug_st_tests_llm_16_92_llm_16_92_rrrruuuugggg_pow_i8_ref_with_u32_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&i8 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_92_llm_16_92_rrrruuuugggg_pow_i8_ref_with_u32_ref = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn pow_i8_ref_with_u32_ref_overflow() {
        let _rug_st_tests_llm_16_92_llm_16_92_rrrruuuugggg_pow_i8_ref_with_u32_ref_overflow = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let base: i8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let _result = <&i8 as Pow<&u32>>::pow(&base, &exponent);
        let _rug_ed_tests_llm_16_92_llm_16_92_rrrruuuugggg_pow_i8_ref_with_u32_ref_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_93_llm_16_93 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_93_llm_16_93_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &i8 = &rug_fuzz_0;
        let exponent: &u8 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_93_llm_16_93_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_94_llm_16_94 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_94_llm_16_94_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: i8 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_94_llm_16_94_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_95_llm_16_95 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_95_llm_16_95_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &isize = &rug_fuzz_0;
        let exponent: &u16 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_95_llm_16_95_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_96_llm_16_96 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_96_llm_16_96_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: isize = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_96_llm_16_96_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_97_llm_16_97 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: isize = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = <&isize as Pow<&u8>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_98_llm_16_98 {
    use crate::pow::Pow;
    #[test]
    fn pow_isize_usize() {
        let _rug_st_tests_llm_16_98_llm_16_98_rrrruuuugggg_pow_isize_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: isize = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_98_llm_16_98_rrrruuuugggg_pow_isize_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_99_llm_16_99 {
    use std::num::Wrapping;
    use crate::Pow;
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_99_llm_16_99_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 10u8;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 1u8;
        let rug_fuzz_5 = 10u8;
        let rug_fuzz_6 = 2i128;
        let rug_fuzz_7 = 2u8;
        let rug_fuzz_8 = 2i128;
        let rug_fuzz_9 = 3u8;
        let rug_fuzz_10 = 3i128;
        let rug_fuzz_11 = 4u8;
        let rug_fuzz_12 = 2i128;
        let rug_fuzz_13 = 2u8;
        let rug_fuzz_14 = 2i128;
        let rug_fuzz_15 = 3u8;
        let rug_fuzz_16 = 4i128;
        let rug_fuzz_17 = 0u8;
        let rug_fuzz_18 = 1u8;
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
        let _rug_ed_tests_llm_16_99_llm_16_99_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_100_llm_16_100 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_i128() {
        let _rug_st_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128 = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(5i128.pow(3)));
        let _rug_ed_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128 = 0;
    }
    #[test]
    fn test_pow_wrapping_i128_with_zero() {
        let _rug_st_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_with_zero = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 0;
        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_with_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_i128_with_one() {
        let _rug_st_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_with_one = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 1;
        let base = Wrapping::<i128>(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, base);
        let _rug_ed_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_with_one = 0;
    }
    #[test]
    fn test_pow_wrapping_i128_overflow() {
        let _rug_st_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_overflow = 0;
        let rug_fuzz_0 = 2;
        let base = Wrapping(i128::MAX);
        let exp: usize = rug_fuzz_0;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(i128::MAX.wrapping_pow(2)));
        let _rug_ed_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_overflow = 0;
    }
    #[test]
    fn test_pow_wrapping_i128_large_power() {
        let _rug_st_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_large_power = 0;
        let rug_fuzz_0 = 2i128;
        let rug_fuzz_1 = 100;
        let base = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = <&Wrapping<i128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(2i128.wrapping_pow(100)));
        let _rug_ed_tests_llm_16_100_llm_16_100_rrrruuuugggg_test_pow_wrapping_i128_large_power = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_101 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_i16() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16 = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 4u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(16));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16 = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_with_zero() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_with_zero = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 0u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_with_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_with_one() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_with_one = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 1u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(2));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_with_one = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_negative_base() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_negative_base = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3u8;
        let base = Wrapping(-rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(- 8));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_negative_base = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_large_exp() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_large_exp = 0;
        let rug_fuzz_0 = 3i16;
        let rug_fuzz_1 = 8u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(6561));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_large_exp = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_max() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_max = 0;
        let rug_fuzz_0 = 2u8;
        let base = Wrapping(i16::MAX);
        let exp = rug_fuzz_0;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_max = 0;
    }
    #[test]
    fn test_pow_wrapping_i16_min() {
        let _rug_st_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_min = 0;
        let rug_fuzz_0 = 2u8;
        let base = Wrapping(i16::MIN);
        let exp = rug_fuzz_0;
        let result = Wrapping::<i16>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_101_rrrruuuugggg_test_pow_wrapping_i16_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_102_llm_16_102 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_102_llm_16_102_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8i16));
        let _rug_ed_tests_llm_16_102_llm_16_102_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_103_llm_16_103 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_103_llm_16_103_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 8u8;
        let rug_fuzz_2 = 0i32;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 1i32;
        let rug_fuzz_5 = 8u8;
        let rug_fuzz_6 = 2i32;
        let rug_fuzz_7 = 0u8;
        let rug_fuzz_8 = 2i32;
        let rug_fuzz_9 = 5u8;
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
        let _rug_ed_tests_llm_16_103_llm_16_103_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_104_llm_16_104 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn pow_wrapping_values() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_values = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8_i32;
        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_values = 0;
    }
    #[test]
    fn pow_wrapping_overflow() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_overflow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 1_i32;
        let base: Wrapping<i32> = Wrapping(i32::MAX);
        let exp: usize = rug_fuzz_0;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_1);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_overflow = 0;
    }
    #[test]
    fn pow_wrapping_zero() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0_i32;
        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_zero = 0;
    }
    #[test]
    fn pow_wrapping_one() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_one = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1_i32;
        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_one = 0;
    }
    #[test]
    fn pow_wrapping_large_exponent() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_large_exponent = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 1_073_741_824_i32;
        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_large_exponent = 0;
    }
    #[test]
    fn pow_wrapping_exponent_zero() {
        let _rug_st_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_exponent_zero = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1_i32;
        let base: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_104_llm_16_104_rrrruuuugggg_pow_wrapping_exponent_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_105_llm_16_105 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_105_llm_16_105_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 5u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(32i64));
        let _rug_ed_tests_llm_16_105_llm_16_105_rrrruuuugggg_test_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_106_llm_16_106 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_106_llm_16_106_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 0i64;
        let rug_fuzz_3 = 0usize;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 2usize;
        let rug_fuzz_6 = 2i64;
        let rug_fuzz_7 = 3usize;
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
        let _rug_ed_tests_llm_16_106_llm_16_106_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_107_llm_16_107 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_107_llm_16_107_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i8;
        let rug_fuzz_1 = 3u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8i8));
        let _rug_ed_tests_llm_16_107_llm_16_107_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_108_llm_16_108 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_i8() {
        let _rug_st_tests_llm_16_108_llm_16_108_rrrruuuugggg_test_pow_wrapping_i8 = 0;
        let rug_fuzz_0 = 2i8;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 8i8;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = Wrapping(rug_fuzz_2);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_108_llm_16_108_rrrruuuugggg_test_pow_wrapping_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_109_llm_16_109 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_wrapping_isize_base_u8_exponent() {
        let _rug_st_tests_llm_16_109_llm_16_109_rrrruuuugggg_pow_wrapping_isize_base_u8_exponent = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 10_isize;
        let rug_fuzz_3 = 3u32;
        let base = Wrapping::<isize>(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = Wrapping::<isize>(rug_fuzz_2.pow(rug_fuzz_3));
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_109_llm_16_109_rrrruuuugggg_pow_wrapping_isize_base_u8_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_110 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_110_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: Wrapping<isize> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let result = <&Wrapping<isize> as pow::Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, Wrapping(8));
        let _rug_ed_tests_llm_16_110_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_111_llm_16_111 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_111_llm_16_111_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 7;
        let rug_fuzz_1 = 3_u8;
        let base = Wrapping::<u128>(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(7_u128.pow(3)));
        let _rug_ed_tests_llm_16_111_llm_16_111_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_112_llm_16_112 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128() {
        let _rug_st_tests_llm_16_112_llm_16_112_rrrruuuugggg_test_pow_wrapping_u128 = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 4usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = <&Wrapping<u128> as Pow<&usize>>::pow(&base, &exp);
        debug_assert_eq!(result, Wrapping(16u128));
        let _rug_ed_tests_llm_16_112_llm_16_112_rrrruuuugggg_test_pow_wrapping_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_113_llm_16_113 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_wrapping_u16_by_ref_u8() {
        let _rug_st_tests_llm_16_113_llm_16_113_rrrruuuugggg_pow_wrapping_u16_by_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: Wrapping<u16> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result: Wrapping<u16> = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(8));
        let _rug_ed_tests_llm_16_113_llm_16_113_rrrruuuugggg_pow_wrapping_u16_by_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_114_llm_16_114 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u16() {
        let _rug_st_tests_llm_16_114_llm_16_114_rrrruuuugggg_test_pow_wrapping_u16 = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 3;
        let base = Wrapping(rug_fuzz_0);
        let exponent: &usize = &rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8u16));
        let _rug_ed_tests_llm_16_114_llm_16_114_rrrruuuugggg_test_pow_wrapping_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_115_llm_16_115 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u32_with_ref_u8() {
        let _rug_st_tests_llm_16_115_llm_16_115_rrrruuuugggg_test_pow_wrapping_u32_with_ref_u8 = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 8u8;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(256u32));
        let _rug_ed_tests_llm_16_115_llm_16_115_rrrruuuugggg_test_pow_wrapping_u32_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_117_llm_16_117 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_wrapping_u64() {
        let _rug_st_tests_llm_16_117_llm_16_117_rrrruuuugggg_pow_wrapping_u64 = 0;
        let rug_fuzz_0 = 3u64;
        let rug_fuzz_1 = 4u8;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(3u64.pow(4)));
        let _rug_ed_tests_llm_16_117_llm_16_117_rrrruuuugggg_pow_wrapping_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_119_llm_16_119 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u8() {
        let _rug_st_tests_llm_16_119_llm_16_119_rrrruuuugggg_test_pow_wrapping_u8 = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3u8;
        let value = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = value.pow(&exponent);
        debug_assert_eq!(result, Wrapping(2u8.pow(3)));
        let _rug_ed_tests_llm_16_119_llm_16_119_rrrruuuugggg_test_pow_wrapping_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_120_llm_16_120 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u8() {
        let _rug_st_tests_llm_16_120_llm_16_120_rrrruuuugggg_test_pow_wrapping_u8 = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(8u8));
        let _rug_ed_tests_llm_16_120_llm_16_120_rrrruuuugggg_test_pow_wrapping_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_121_llm_16_121 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_usize() {
        let _rug_st_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_pow_wrapping_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let base: Wrapping<usize> = Wrapping(rug_fuzz_0);
        let exponent: u8 = rug_fuzz_1;
        let result: Wrapping<usize> = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(256));
        let _rug_ed_tests_llm_16_121_llm_16_121_rrrruuuugggg_test_pow_wrapping_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_122_llm_16_122 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_usize() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_usize = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 4usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(16));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_usize = 0;
    }
    #[test]
    fn pow_zero() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_zero = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 0usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_zero = 0;
    }
    #[test]
    fn pow_one() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_one = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 1usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(2));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_one = 0;
    }
    #[test]
    fn pow_large_exponent() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_large_exponent = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 10usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1024));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_large_exponent = 0;
    }
    #[test]
    fn pow_large_base() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_large_base = 0;
        let rug_fuzz_0 = 10usize;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1000));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_large_base = 0;
    }
    #[test]
    fn pow_wrapping() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_wrapping = 0;
        let rug_fuzz_0 = 2usize;
        let base = Wrapping(usize::MAX);
        let exponent = rug_fuzz_0;
        debug_assert_eq!(Pow::pow(base, & exponent), Wrapping(1));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_123_llm_16_123 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_ref_u16() {
        let _rug_st_tests_llm_16_123_llm_16_123_rrrruuuugggg_test_pow_u128_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let base: u128 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u128 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1024_u128);
        let _rug_ed_tests_llm_16_123_llm_16_123_rrrruuuugggg_test_pow_u128_with_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_124_llm_16_124 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_ref_with_u32_ref() {
        let _rug_st_tests_llm_16_124_llm_16_124_rrrruuuugggg_test_pow_u128_ref_with_u32_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let base: u128 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1024u128);
        let _rug_ed_tests_llm_16_124_llm_16_124_rrrruuuugggg_test_pow_u128_ref_with_u32_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_125_llm_16_125 {
    use crate::Pow;
    #[test]
    fn test_pow_u128_ref_with_u8_ref() {
        let _rug_st_tests_llm_16_125_llm_16_125_rrrruuuugggg_test_pow_u128_ref_with_u8_ref = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 3u128;
        let rug_fuzz_3 = 2u8;
        let rug_fuzz_4 = 0u128;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0u128;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 1u128;
        let rug_fuzz_9 = 0u8;
        let rug_fuzz_10 = 10u128;
        let rug_fuzz_11 = 5u8;
        debug_assert_eq!(Pow::pow(& rug_fuzz_0, & rug_fuzz_1), 8u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_2, & rug_fuzz_3), 9u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_4, & rug_fuzz_5), 1u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_6, & rug_fuzz_7), 0u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_8, & rug_fuzz_9), 1u128);
        debug_assert_eq!(Pow::pow(& rug_fuzz_10, & rug_fuzz_11), 100000u128);
        let _rug_ed_tests_llm_16_125_llm_16_125_rrrruuuugggg_test_pow_u128_ref_with_u8_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_126_llm_16_126 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_usize() {
        let _rug_st_tests_llm_16_126_llm_16_126_rrrruuuugggg_test_u128_pow_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 1_000_000_000_000;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 64;
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
        let _rug_ed_tests_llm_16_126_llm_16_126_rrrruuuugggg_test_u128_pow_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_127_llm_16_127 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_127_llm_16_127_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: u16 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 16);
        let _rug_ed_tests_llm_16_127_llm_16_127_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_128 {
    use crate::pow::Pow;
    #[test]
    fn pow_test() {
        let _rug_st_tests_llm_16_128_rrrruuuugggg_pow_test = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u16 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_128_rrrruuuugggg_pow_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_129_llm_16_129 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_u16_ref_with_u8_ref() {
        let _rug_st_tests_llm_16_129_llm_16_129_rrrruuuugggg_test_pow_u16_ref_with_u8_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u16 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_129_llm_16_129_rrrruuuugggg_test_pow_u16_ref_with_u8_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_131_llm_16_131 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_131_llm_16_131_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u32 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u32 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_131_llm_16_131_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_132_llm_16_132 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_132_llm_16_132_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u32 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        debug_assert_eq!(< & u32 as Pow < & u32 > > ::pow(& base, & exponent), 8);
        let _rug_ed_tests_llm_16_132_llm_16_132_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_133_llm_16_133 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_133_llm_16_133_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: &u32 = &rug_fuzz_0;
        let exponent: &u8 = &rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, 8u32);
        let _rug_ed_tests_llm_16_133_llm_16_133_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_134_llm_16_134 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_134_llm_16_134_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u32 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u32 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_134_llm_16_134_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_135_llm_16_135 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let base: u64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = <&u64 as Pow<&u16>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1024u64);
        let _rug_ed_tests_llm_16_135_llm_16_135_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_136_llm_16_136 {
    use crate::pow::Pow;
    #[test]
    fn pow_u64_with_ref_u32() {
        let _rug_st_tests_llm_16_136_llm_16_136_rrrruuuugggg_pow_u64_with_ref_u32 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u64 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_136_llm_16_136_rrrruuuugggg_pow_u64_with_ref_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_137_llm_16_137 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_137_llm_16_137_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_137_llm_16_137_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_138_llm_16_138 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_138_llm_16_138_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u64 = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = <&u64 as Pow<&usize>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_138_llm_16_138_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_139_llm_16_139 {
    use crate::pow::Pow;
    use core::convert::From;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_139_llm_16_139_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u8 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8u8);
        let _rug_ed_tests_llm_16_139_llm_16_139_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_140_llm_16_140 {
    use super::*;
    use crate::*;
    #[test]
    fn pow_u8_ref_with_u32_ref() {
        let _rug_st_tests_llm_16_140_llm_16_140_rrrruuuugggg_pow_u8_ref_with_u32_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u8 = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = <&u8 as Pow<&u32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8_u8);
        let _rug_ed_tests_llm_16_140_llm_16_140_rrrruuuugggg_pow_u8_ref_with_u32_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_141_llm_16_141 {
    use crate::pow::Pow;
    #[test]
    fn test_u8_pow() {
        let _rug_st_tests_llm_16_141_llm_16_141_rrrruuuugggg_test_u8_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u8 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_141_llm_16_141_rrrruuuugggg_test_u8_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_142_llm_16_142 {
    use crate::pow::Pow;
    #[test]
    fn pow_u8_by_usize() {
        let _rug_st_tests_llm_16_142_llm_16_142_rrrruuuugggg_pow_u8_by_usize = 0;
        let rug_fuzz_0 = 5u8;
        let rug_fuzz_1 = 0usize;
        let rug_fuzz_2 = 5u8;
        let rug_fuzz_3 = 1usize;
        let rug_fuzz_4 = 5u8;
        let rug_fuzz_5 = 2usize;
        let rug_fuzz_6 = 5u8;
        let rug_fuzz_7 = 3usize;
        debug_assert_eq!((& rug_fuzz_0).pow(& rug_fuzz_1), 1u8);
        debug_assert_eq!((& rug_fuzz_2).pow(& rug_fuzz_3), 5u8);
        debug_assert_eq!((& rug_fuzz_4).pow(& rug_fuzz_5), 25u8);
        debug_assert_eq!((& rug_fuzz_6).pow(& rug_fuzz_7), 125u8);
        let _rug_ed_tests_llm_16_142_llm_16_142_rrrruuuugggg_pow_u8_by_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_143_llm_16_143 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u16_ref() {
        let _rug_st_tests_llm_16_143_llm_16_143_rrrruuuugggg_test_pow_usize_ref_with_u16_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: usize = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = <&usize as Pow<&u16>>::pow(&base, &exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_143_llm_16_143_rrrruuuugggg_test_pow_usize_ref_with_u16_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_144_llm_16_144 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u32_ref() {
        let _rug_st_tests_llm_16_144_llm_16_144_rrrruuuugggg_test_pow_usize_ref_with_u32_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: usize = rug_fuzz_0;
        let exponent: u32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_144_llm_16_144_rrrruuuugggg_test_pow_usize_ref_with_u32_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_145_llm_16_145 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_ref_with_u8_ref() {
        let _rug_st_tests_llm_16_145_llm_16_145_rrrruuuugggg_test_pow_usize_ref_with_u8_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
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
        let _rug_ed_tests_llm_16_145_llm_16_145_rrrruuuugggg_test_pow_usize_ref_with_u8_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_146_llm_16_146 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_146_llm_16_146_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: usize = rug_fuzz_0;
        let exponent: usize = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_146_llm_16_146_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_696_llm_16_696 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_ref_u16() {
        let _rug_st_tests_llm_16_696_llm_16_696_rrrruuuugggg_test_pow_i128_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: i128 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = <i128 as Pow<&u16>>::pow(base, &exp);
        debug_assert_eq!(result, 16);
        let _rug_ed_tests_llm_16_696_llm_16_696_rrrruuuugggg_test_pow_i128_with_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_697_llm_16_697 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn pow_i128_with_ref_u32() {
        let _rug_st_tests_llm_16_697_llm_16_697_rrrruuuugggg_pow_i128_with_ref_u32 = 0;
        let rug_fuzz_0 = 2i128;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 1u32;
        let rug_fuzz_4 = 2i128;
        let rug_fuzz_5 = 2u32;
        let rug_fuzz_6 = 2i128;
        let rug_fuzz_7 = 3u32;
        let rug_fuzz_8 = 2i128;
        let rug_fuzz_9 = 2u32;
        let rug_fuzz_10 = 2i128;
        let rug_fuzz_11 = 3u32;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 4i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, & rug_fuzz_11), - 8i128);
        let _rug_ed_tests_llm_16_697_llm_16_697_rrrruuuugggg_pow_i128_with_ref_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_698_llm_16_698 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_698_llm_16_698_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 1;
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
        let _rug_ed_tests_llm_16_698_llm_16_698_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_699_llm_16_699 {
    use crate::Pow;
    #[test]
    fn test_pow_i128_with_usize() {
        let _rug_st_tests_llm_16_699_llm_16_699_rrrruuuugggg_test_pow_i128_with_usize = 0;
        let rug_fuzz_0 = 0_i128;
        let rug_fuzz_1 = 0_usize;
        let rug_fuzz_2 = 0_i128;
        let rug_fuzz_3 = 1_usize;
        let rug_fuzz_4 = 0_i128;
        let rug_fuzz_5 = 2_usize;
        let rug_fuzz_6 = 1_i128;
        let rug_fuzz_7 = 0_usize;
        let rug_fuzz_8 = 1_i128;
        let rug_fuzz_9 = 1_usize;
        let rug_fuzz_10 = 1_i128;
        let rug_fuzz_11 = 2_usize;
        let rug_fuzz_12 = 2_i128;
        let rug_fuzz_13 = 0_usize;
        let rug_fuzz_14 = 2_i128;
        let rug_fuzz_15 = 1_usize;
        let rug_fuzz_16 = 2_i128;
        let rug_fuzz_17 = 2_usize;
        let rug_fuzz_18 = 2_i128;
        let rug_fuzz_19 = 3_usize;
        let rug_fuzz_20 = 2_i128;
        let rug_fuzz_21 = 3_usize;
        let rug_fuzz_22 = 3_i128;
        let rug_fuzz_23 = 2_usize;
        let rug_fuzz_24 = 3_i128;
        let rug_fuzz_25 = 3_usize;
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
        let _rug_ed_tests_llm_16_699_llm_16_699_rrrruuuugggg_test_pow_i128_with_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_700_llm_16_700 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128_with_u16() {
        let _rug_st_tests_llm_16_700_llm_16_700_rrrruuuugggg_test_pow_i128_with_u16 = 0;
        let rug_fuzz_0 = 2i128;
        let rug_fuzz_1 = 4u16;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 3u16;
        let rug_fuzz_4 = 0i128;
        let rug_fuzz_5 = 0u16;
        let rug_fuzz_6 = 0i128;
        let rug_fuzz_7 = 1u16;
        let rug_fuzz_8 = 1i128;
        let rug_fuzz_9 = 0u16;
        let rug_fuzz_10 = 1i128;
        let rug_fuzz_11 = 0u16;
        let rug_fuzz_12 = 1i128;
        let rug_fuzz_13 = 1u16;
        let rug_fuzz_14 = 1i128;
        let rug_fuzz_15 = 2u16;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, rug_fuzz_1), 16i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, rug_fuzz_3), - 8i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, rug_fuzz_5), 1i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, rug_fuzz_7), 0i128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, rug_fuzz_9), 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, rug_fuzz_11), 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_12, rug_fuzz_13), - 1i128);
        debug_assert_eq!(Pow::pow(- rug_fuzz_14, rug_fuzz_15), 1i128);
        let _rug_ed_tests_llm_16_700_llm_16_700_rrrruuuugggg_test_pow_i128_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_701_llm_16_701 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i128() {
        let _rug_st_tests_llm_16_701_llm_16_701_rrrruuuugggg_test_pow_i128 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 6;
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1024);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 32);
        debug_assert_eq!(< i128 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 64);
        let _rug_ed_tests_llm_16_701_llm_16_701_rrrruuuugggg_test_pow_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_702_llm_16_702 {
    use crate::Pow;
    #[test]
    fn test_pow_i128() {
        let _rug_st_tests_llm_16_702_llm_16_702_rrrruuuugggg_test_pow_i128 = 0;
        let rug_fuzz_0 = 2i128;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 1u8;
        let rug_fuzz_4 = 2i128;
        let rug_fuzz_5 = 2u8;
        let rug_fuzz_6 = 2i128;
        let rug_fuzz_7 = 3u8;
        let rug_fuzz_8 = 2i128;
        let rug_fuzz_9 = 2u8;
        let rug_fuzz_10 = 2i128;
        let rug_fuzz_11 = 3u8;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
        let _rug_ed_tests_llm_16_702_llm_16_702_rrrruuuugggg_test_pow_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_703_llm_16_703 {
    use crate::pow::Pow;
    #[test]
    fn i128_pow_usize() {
        let _rug_st_tests_llm_16_703_llm_16_703_rrrruuuugggg_i128_pow_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(- rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i128 as Pow < usize > > ::pow(i128::MAX, rug_fuzz_10), 1);
        debug_assert_eq!(
            < i128 as Pow < usize > > ::pow(i128::MIN, rug_fuzz_11), i128::MIN
        );
        let _rug_ed_tests_llm_16_703_llm_16_703_rrrruuuugggg_i128_pow_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_806_llm_16_806 {
    use crate::pow::Pow;
    #[test]
    fn i16_pow_u16() {
        let _rug_st_tests_llm_16_806_llm_16_806_rrrruuuugggg_i16_pow_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(
            < i16 as Pow < & u16 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), - 8
        );
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< i16 as Pow < & u16 > > ::pow(- rug_fuzz_10, & u16::MAX), 1);
        let _rug_ed_tests_llm_16_806_llm_16_806_rrrruuuugggg_i16_pow_u16 = 0;
    }
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
        let _rug_st_tests_llm_16_808_llm_16_808_rrrruuuugggg_test_pow_i16_with_ref_u8 = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 2i16;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0i16;
        let rug_fuzz_7 = 3u8;
        let rug_fuzz_8 = 1u8;
        let rug_fuzz_9 = 1u8;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(i16::MAX, & rug_fuzz_8), i16::MAX);
        debug_assert_eq!(Pow::pow(i16::MIN, & rug_fuzz_9), i16::MIN);
        let _rug_ed_tests_llm_16_808_llm_16_808_rrrruuuugggg_test_pow_i16_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_809_llm_16_809 {
    use crate::Pow;
    #[test]
    fn test_i16_pow() {
        let _rug_st_tests_llm_16_809_llm_16_809_rrrruuuugggg_test_i16_pow = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 3usize;
        let rug_fuzz_4 = 2i16;
        let rug_fuzz_5 = 0usize;
        let rug_fuzz_6 = 0i16;
        let rug_fuzz_7 = 3usize;
        let rug_fuzz_8 = 0i16;
        let rug_fuzz_9 = 0usize;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_809_llm_16_809_rrrruuuugggg_test_i16_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_810_llm_16_810 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u16() {
        let _rug_st_tests_llm_16_810_llm_16_810_rrrruuuugggg_test_pow_i16_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(< i16 as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
        let _rug_ed_tests_llm_16_810_llm_16_810_rrrruuuugggg_test_pow_i16_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_811_llm_16_811 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_811_llm_16_811_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 100;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 101;
        let rug_fuzz_14 = 10;
        let rug_fuzz_15 = 4;
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< i16 as Pow < u32 > > ::pow(rug_fuzz_14, rug_fuzz_15), 10000);
        let _rug_ed_tests_llm_16_811_llm_16_811_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_812_llm_16_812 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16_u8() {
        let _rug_st_tests_llm_16_812_llm_16_812_rrrruuuugggg_test_pow_i16_u8 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 4;
        let rug_fuzz_16 = 3;
        let rug_fuzz_17 = 4;
        let rug_fuzz_18 = 3;
        let rug_fuzz_19 = 5;
        let rug_fuzz_20 = 10;
        let rug_fuzz_21 = 3;
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
        let _rug_ed_tests_llm_16_812_llm_16_812_rrrruuuugggg_test_pow_i16_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_813_llm_16_813 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i16() {
        let _rug_st_tests_llm_16_813_llm_16_813_rrrruuuugggg_test_pow_i16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 4;
        let rug_fuzz_12 = 7;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 7;
        let rug_fuzz_15 = 3;
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
        let _rug_ed_tests_llm_16_813_llm_16_813_rrrruuuugggg_test_pow_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_916_llm_16_916 {
    use crate::pow::Pow;
    #[test]
    fn test_i32_pow_u16_ref() {
        let _rug_st_tests_llm_16_916_llm_16_916_rrrruuuugggg_test_i32_pow_u16_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 5);
        debug_assert_eq!(< i32 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 81);
        debug_assert_eq!(
            < i32 as Pow < & u16 > > ::pow(- rug_fuzz_8, & rug_fuzz_9), - 27
        );
        let _rug_ed_tests_llm_16_916_llm_16_916_rrrruuuugggg_test_i32_pow_u16_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_917 {
    use crate::Pow;
    use std::convert::From;
    #[test]
    fn i32_pow_with_reference_u32() {
        let _rug_st_tests_llm_16_917_rrrruuuugggg_i32_pow_with_reference_u32 = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 3i32;
        let rug_fuzz_3 = 1u32;
        let rug_fuzz_4 = 4i32;
        let rug_fuzz_5 = 2u32;
        let rug_fuzz_6 = 5i32;
        let rug_fuzz_7 = 3u32;
        let rug_fuzz_8 = 3i32;
        let rug_fuzz_9 = 2u32;
        let rug_fuzz_10 = 2i32;
        let rug_fuzz_11 = 3u32;
        let rug_fuzz_12 = 2i32;
        let rug_fuzz_13 = 31u32;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 3i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 16i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 125i32);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 9i32);
        debug_assert_eq!(Pow::pow(- rug_fuzz_10, & rug_fuzz_11), - 8i32);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 2i32.pow(31u32));
        let _rug_ed_tests_llm_16_917_rrrruuuugggg_i32_pow_with_reference_u32 = 0;
    }
    #[test]
    #[should_panic]
    fn i32_pow_with_reference_u32_overflow() {
        let _rug_st_tests_llm_16_917_rrrruuuugggg_i32_pow_with_reference_u32_overflow = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 32u32;
        Pow::pow(rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_917_rrrruuuugggg_i32_pow_with_reference_u32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_918_llm_16_918 {
    use crate::Pow;
    #[test]
    fn test_pow_i32_with_ref_u8() {
        let _rug_st_tests_llm_16_918_llm_16_918_rrrruuuugggg_test_pow_i32_with_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 5;
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
        let _rug_ed_tests_llm_16_918_llm_16_918_rrrruuuugggg_test_pow_i32_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_919_llm_16_919 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i32_with_ref_usize() {
        let _rug_st_tests_llm_16_919_llm_16_919_rrrruuuugggg_test_pow_i32_with_ref_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 4;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 100;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 100;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 101;
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
        let _rug_ed_tests_llm_16_919_llm_16_919_rrrruuuugggg_test_pow_i32_with_ref_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_920_llm_16_920 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_920_llm_16_920_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 2;
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8);
        debug_assert_eq!(< i32 as Pow < u16 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 9);
        let _rug_ed_tests_llm_16_920_llm_16_920_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_921_llm_16_921 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_921_llm_16_921_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 2;
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 2);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 8);
        debug_assert_eq!(< i32 as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 4);
        let _rug_ed_tests_llm_16_921_llm_16_921_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_922_llm_16_922 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i32_u8() {
        let _rug_st_tests_llm_16_922_llm_16_922_rrrruuuugggg_test_pow_i32_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0u8;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0u8;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1u8;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 0u8;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 0u8;
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(rug_fuzz_14, rug_fuzz_15), 1);
        debug_assert_eq!(< i32 as Pow < u8 > > ::pow(- rug_fuzz_16, rug_fuzz_17), 1);
        let _rug_ed_tests_llm_16_922_llm_16_922_rrrruuuugggg_test_pow_i32_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_923 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_923_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 0;
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(
            < i32 as Pow < usize > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8
        );
        debug_assert_eq!(< i32 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_923_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1026_llm_16_1026 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_with_u16_ref() {
        let _rug_st_tests_llm_16_1026_llm_16_1026_rrrruuuugggg_test_pow_i64_with_u16_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 5u16;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0u16;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2u16;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 3u16;
        let rug_fuzz_10 = 7;
        let rug_fuzz_11 = 0u16;
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
        let _rug_ed_tests_llm_16_1026_llm_16_1026_rrrruuuugggg_test_pow_i64_with_u16_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1027_llm_16_1027 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1027_llm_16_1027_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 10;
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
        let _rug_ed_tests_llm_16_1027_llm_16_1027_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1028_llm_16_1028 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_u8_ref() {
        let _rug_st_tests_llm_16_1028_llm_16_1028_rrrruuuugggg_test_pow_i64_u8_ref = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 2;
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 512);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 16);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 9);
        let _rug_ed_tests_llm_16_1028_llm_16_1028_rrrruuuugggg_test_pow_i64_u8_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1029_llm_16_1029 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_with_usize_ref() {
        let _rug_st_tests_llm_16_1029_llm_16_1029_rrrruuuugggg_test_pow_i64_with_usize_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
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
        let _rug_ed_tests_llm_16_1029_llm_16_1029_rrrruuuugggg_test_pow_i64_with_usize_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1030_llm_16_1030 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1030_llm_16_1030_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 9;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 10;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 16;
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
        let _rug_ed_tests_llm_16_1030_llm_16_1030_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1031_llm_16_1031 {
    use crate::pow::Pow;
    #[test]
    fn pow_i64_u32() {
        let _rug_st_tests_llm_16_1031_llm_16_1031_rrrruuuugggg_pow_i64_u32 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 100;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 100;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 101;
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 4);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(< i64 as Pow < u32 > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 1);
        let _rug_ed_tests_llm_16_1031_llm_16_1031_rrrruuuugggg_pow_i64_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1032_llm_16_1032 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i64_u8() {
        let _rug_st_tests_llm_16_1032_llm_16_1032_rrrruuuugggg_test_pow_i64_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 2);
        debug_assert_eq!(< i64 as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 2);
        let _rug_ed_tests_llm_16_1032_llm_16_1032_rrrruuuugggg_test_pow_i64_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1033_llm_16_1033 {
    use crate::pow::Pow;
    #[test]
    fn test_i64_pow() {
        let _rug_st_tests_llm_16_1033_llm_16_1033_rrrruuuugggg_test_i64_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 10;
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), - 8);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 10);
        debug_assert_eq!(< i64 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_1033_llm_16_1033_rrrruuuugggg_test_i64_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1136_llm_16_1136 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_with_ref_u16() {
        let _rug_st_tests_llm_16_1136_llm_16_1136_rrrruuuugggg_test_pow_i8_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< i8 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_1136_llm_16_1136_rrrruuuugggg_test_pow_i8_with_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1137_llm_16_1137 {
    use crate::pow::Pow;
    #[test]
    fn test_i8_pow_u32_ref() {
        let _rug_st_tests_llm_16_1137_llm_16_1137_rrrruuuugggg_test_i8_pow_u32_ref = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2u32;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 2u32;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3u32;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 0u32;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2u32;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0u32;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 10u32;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 10u32;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 10u32;
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 4);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(- rug_fuzz_2, & rug_fuzz_3), 4);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(- rug_fuzz_4, & rug_fuzz_5), - 8);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 1);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_14, & rug_fuzz_15), 0);
        debug_assert_eq!(< i8 as Pow < & u32 > > ::pow(rug_fuzz_16, & rug_fuzz_17), 0);
        let _rug_ed_tests_llm_16_1137_llm_16_1137_rrrruuuugggg_test_i8_pow_u32_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1138_llm_16_1138 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i8_with_ref_u8() {
        let _rug_st_tests_llm_16_1138_llm_16_1138_rrrruuuugggg_test_pow_i8_with_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2u8;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0u8;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0u8;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1u8;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 10u8;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 3u8;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 2u8;
        let rug_fuzz_18 = 2;
        let rug_fuzz_19 = 3u8;
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
        let _rug_ed_tests_llm_16_1138_llm_16_1138_rrrruuuugggg_test_pow_i8_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1139_llm_16_1139 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_i8_with_usize() {
        let _rug_st_tests_llm_16_1139_llm_16_1139_rrrruuuugggg_test_pow_i8_with_usize = 0;
        let rug_fuzz_0 = 2i8;
        let rug_fuzz_1 = 0usize;
        let rug_fuzz_2 = 2i8;
        let rug_fuzz_3 = 1usize;
        let rug_fuzz_4 = 2i8;
        let rug_fuzz_5 = 2usize;
        let rug_fuzz_6 = 2i8;
        let rug_fuzz_7 = 2usize;
        let rug_fuzz_8 = 2i8;
        let rug_fuzz_9 = 3usize;
        let rug_fuzz_10 = 0i8;
        let rug_fuzz_11 = 0usize;
        let rug_fuzz_12 = 0i8;
        let rug_fuzz_13 = 1usize;
        let rug_fuzz_14 = 0i8;
        let rug_fuzz_15 = 2usize;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4i8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_6, & rug_fuzz_7), 4i8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), - 8i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 0i8);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 0i8);
        let _rug_ed_tests_llm_16_1139_llm_16_1139_rrrruuuugggg_test_pow_i8_with_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1140_llm_16_1140 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_u16() {
        let _rug_st_tests_llm_16_1140_llm_16_1140_rrrruuuugggg_pow_i8_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 65535;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 16;
        let rug_fuzz_14 = 2i16;
        let rug_fuzz_15 = 16;
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
        let _rug_ed_tests_llm_16_1140_llm_16_1140_rrrruuuugggg_pow_i8_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1141_llm_16_1141 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1141_llm_16_1141_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 8;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 9;
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_12, rug_fuzz_13), 1);
        debug_assert_eq!(< i8 as Pow < u32 > > ::pow(- rug_fuzz_14, rug_fuzz_15), - 1);
        let _rug_ed_tests_llm_16_1141_llm_16_1141_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1142_llm_16_1142 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8() {
        let _rug_st_tests_llm_16_1142_llm_16_1142_rrrruuuugggg_test_pow_i8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 7;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 7;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 8;
        let rug_fuzz_18 = 2;
        let rug_fuzz_19 = 8;
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
        let _rug_ed_tests_llm_16_1142_llm_16_1142_rrrruuuugggg_test_pow_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1143_llm_16_1143 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1143_llm_16_1143_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(- rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(< i8 as Pow < usize > > ::pow(- rug_fuzz_8, rug_fuzz_9), - 8);
        let _rug_ed_tests_llm_16_1143_llm_16_1143_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1246_llm_16_1246 {
    use crate::pow::Pow;
    #[test]
    fn pow_isize_with_ref_u16() {
        let _rug_st_tests_llm_16_1246_llm_16_1246_rrrruuuugggg_pow_isize_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0u16;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2u16;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 3u16;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0u16;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 0u16;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 16u16;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 3u16;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 4u16;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 10u16;
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
        let _rug_ed_tests_llm_16_1246_llm_16_1246_rrrruuuugggg_pow_isize_with_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1247_llm_16_1247 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn pow_isize_with_ref_u32() {
        let _rug_st_tests_llm_16_1247_llm_16_1247_rrrruuuugggg_pow_isize_with_ref_u32 = 0;
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
        let _rug_ed_tests_llm_16_1247_llm_16_1247_rrrruuuugggg_pow_isize_with_ref_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1248_llm_16_1248 {
    use crate::Pow;
    #[test]
    fn test_pow_isize_ref_u8() {
        let _rug_st_tests_llm_16_1248_llm_16_1248_rrrruuuugggg_test_pow_isize_ref_u8 = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2isize;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 0isize;
        let rug_fuzz_5 = 3u8;
        let rug_fuzz_6 = 2isize;
        let rug_fuzz_7 = 0u8;
        let rug_fuzz_8 = 2isize;
        let rug_fuzz_9 = 0u8;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(- rug_fuzz_2, & rug_fuzz_3), - 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(Pow::pow(- rug_fuzz_8, & rug_fuzz_9), 1);
        let _rug_ed_tests_llm_16_1248_llm_16_1248_rrrruuuugggg_test_pow_isize_ref_u8 = 0;
    }
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
        let _rug_st_tests_llm_16_1250_llm_16_1250_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 16;
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
        let _rug_ed_tests_llm_16_1250_llm_16_1250_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1251_llm_16_1251 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1251_llm_16_1251_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< isize as Pow < u32 > > ::pow(- rug_fuzz_8, rug_fuzz_9), 9);
        debug_assert_eq!(
            < isize as Pow < u32 > > ::pow(- rug_fuzz_10, rug_fuzz_11), - 8
        );
        let _rug_ed_tests_llm_16_1251_llm_16_1251_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1252_llm_16_1252 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_isize_u8() {
        let _rug_st_tests_llm_16_1252_llm_16_1252_rrrruuuugggg_test_pow_isize_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 0;
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_2, rug_fuzz_3), - 8);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_12, rug_fuzz_13), - 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(- rug_fuzz_14, rug_fuzz_15), 1);
        debug_assert_eq!(< isize as Pow < u8 > > ::pow(rug_fuzz_16, rug_fuzz_17), 1);
        let _rug_ed_tests_llm_16_1252_llm_16_1252_rrrruuuugggg_test_pow_isize_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1309_llm_16_1309 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1309_llm_16_1309_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2_i128;
        let rug_fuzz_1 = 3_u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = <Wrapping<i128> as Pow<u8>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(8_i128));
        let _rug_ed_tests_llm_16_1309_llm_16_1309_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1312_llm_16_1312 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_1312_llm_16_1312_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2i16;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, Wrapping(8i16));
        let _rug_ed_tests_llm_16_1312_llm_16_1312_rrrruuuugggg_test_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1313_llm_16_1313 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_for_wrapping_i16() {
        let _rug_st_tests_llm_16_1313_llm_16_1313_rrrruuuugggg_pow_for_wrapping_i16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
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
        let _rug_ed_tests_llm_16_1313_llm_16_1313_rrrruuuugggg_pow_for_wrapping_i16 = 0;
    }
    #[test]
    fn pow_for_wrapping_i16_with_reference() {
        let _rug_st_tests_llm_16_1313_llm_16_1313_rrrruuuugggg_pow_for_wrapping_i16_with_reference = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
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
        let _rug_ed_tests_llm_16_1313_llm_16_1313_rrrruuuugggg_pow_for_wrapping_i16_with_reference = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1314_llm_16_1314 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_for_wrapping_i16() {
        let _rug_st_tests_llm_16_1314_llm_16_1314_rrrruuuugggg_test_pow_for_wrapping_i16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
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
        let _rug_ed_tests_llm_16_1314_llm_16_1314_rrrruuuugggg_test_pow_for_wrapping_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1316_llm_16_1316 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(8));
        let _rug_ed_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow = 0;
    }
    #[test]
    fn test_pow_zero() {
        let _rug_st_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_zero = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 5usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_zero = 0;
    }
    #[test]
    fn test_pow_one() {
        let _rug_st_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_one = 0;
        let rug_fuzz_0 = 1i32;
        let rug_fuzz_1 = 100usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_one = 0;
    }
    #[test]
    fn test_pow_of_zero() {
        let _rug_st_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_of_zero = 0;
        let rug_fuzz_0 = 10i32;
        let rug_fuzz_1 = 0usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_of_zero = 0;
    }
    #[test]
    fn test_pow_wrapping() {
        let _rug_st_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_wrapping = 0;
        let rug_fuzz_0 = 2usize;
        let base = Wrapping(i32::MAX);
        let exp = rug_fuzz_0;
        let result = base.pow(&exp);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_1316_llm_16_1316_rrrruuuugggg_test_pow_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1317_llm_16_1317 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    use crate::Bounded;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1317_llm_16_1317_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 31;
        let rug_fuzz_8 = 2;
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
        let _rug_ed_tests_llm_16_1317_llm_16_1317_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1321_llm_16_1321 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1321_llm_16_1321_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 1u8;
        let rug_fuzz_6 = 0i64;
        let rug_fuzz_7 = 5u8;
        let rug_fuzz_8 = 1i64;
        let rug_fuzz_9 = 5u8;
        let rug_fuzz_10 = 1i64;
        let rug_fuzz_11 = 2u8;
        let rug_fuzz_12 = 1i64;
        let rug_fuzz_13 = 3u8;
        let rug_fuzz_14 = 1u8;
        let rug_fuzz_15 = 0u8;
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
        let _rug_ed_tests_llm_16_1321_llm_16_1321_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1327_llm_16_1327 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1327_llm_16_1327_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 2u8;
        let rug_fuzz_2 = 3isize;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 0isize;
        let rug_fuzz_5 = 5u8;
        let rug_fuzz_6 = 1isize;
        let rug_fuzz_7 = 8u8;
        let rug_fuzz_8 = 2isize;
        let rug_fuzz_9 = 3u8;
        let rug_fuzz_10 = 2isize;
        let rug_fuzz_11 = 4u8;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(& rug_fuzz_1), Wrapping(4isize));
        debug_assert_eq!(Wrapping(rug_fuzz_2).pow(& rug_fuzz_3), Wrapping(27isize));
        debug_assert_eq!(Wrapping(rug_fuzz_4).pow(& rug_fuzz_5), Wrapping(0isize));
        debug_assert_eq!(Wrapping(rug_fuzz_6).pow(& rug_fuzz_7), Wrapping(1isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_8).pow(& rug_fuzz_9), Wrapping(- 8isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_10).pow(& rug_fuzz_11), Wrapping(16isize));
        let _rug_ed_tests_llm_16_1327_llm_16_1327_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1330_llm_16_1330 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn pow_usize_wrapping_isize() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize = 0;
        let rug_fuzz_0 = 2_isize;
        let rug_fuzz_1 = 3_usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(8_isize));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize = 0;
    }
    #[test]
    fn pow_usize_wrapping_isize_overflow() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_overflow = 0;
        let rug_fuzz_0 = 2_usize;
        let base = Wrapping(isize::MAX);
        let exponent = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_overflow = 0;
    }
    #[test]
    fn pow_usize_wrapping_isize_underflow() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_underflow = 0;
        let rug_fuzz_0 = 2_usize;
        let base = Wrapping(isize::MIN);
        let exponent = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_underflow = 0;
    }
    #[test]
    fn pow_usize_wrapping_isize_ref() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_ref = 0;
        let rug_fuzz_0 = 3_isize;
        let rug_fuzz_1 = 4_usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = &rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(81_isize));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_ref = 0;
    }
    #[test]
    fn pow_usize_wrapping_isize_zero_exponent() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_zero_exponent = 0;
        let rug_fuzz_0 = 10_isize;
        let rug_fuzz_1 = 0_usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1_isize));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_zero_exponent = 0;
    }
    #[test]
    fn pow_usize_wrapping_isize_zero_base() {
        let _rug_st_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_zero_base = 0;
        let rug_fuzz_0 = 0_isize;
        let rug_fuzz_1 = 5_usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0_isize));
        let _rug_ed_tests_llm_16_1330_llm_16_1330_rrrruuuugggg_pow_usize_wrapping_isize_zero_base = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1333_llm_16_1333 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u128_u8() {
        let _rug_st_tests_llm_16_1333_llm_16_1333_rrrruuuugggg_test_pow_wrapping_u128_u8 = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 8u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u128 > as Pow < u8 > > ::pow(base, exp), Wrapping(256u128)
        );
        let _rug_ed_tests_llm_16_1333_llm_16_1333_rrrruuuugggg_test_pow_wrapping_u128_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1334_llm_16_1334 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1334_llm_16_1334_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        debug_assert_eq!(result, Wrapping(8u128));
        let _rug_ed_tests_llm_16_1334_llm_16_1334_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1335_llm_16_1335 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow_wrapping_u16() {
        let _rug_st_tests_llm_16_1335_llm_16_1335_rrrruuuugggg_test_pow_wrapping_u16 = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 8u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(256u16));
        let _rug_ed_tests_llm_16_1335_llm_16_1335_rrrruuuugggg_test_pow_wrapping_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1336_llm_16_1336 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u16_with_ref_usize() {
        let _rug_st_tests_llm_16_1336_llm_16_1336_rrrruuuugggg_test_pow_wrapping_u16_with_ref_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let base: Wrapping<u16> = Wrapping(rug_fuzz_0);
        let exponent: usize = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(16));
        let _rug_ed_tests_llm_16_1336_llm_16_1336_rrrruuuugggg_test_pow_wrapping_u16_with_ref_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1337_llm_16_1337 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1337_llm_16_1337_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 3u8;
        let rug_fuzz_2 = 8u16;
        let rug_fuzz_3 = 0u16;
        let rug_fuzz_4 = 0u8;
        let rug_fuzz_5 = 1u16;
        let rug_fuzz_6 = 5u16;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 5u16;
        let rug_fuzz_9 = 3u16;
        let rug_fuzz_10 = 5u8;
        let rug_fuzz_11 = 243u16;
        let rug_fuzz_12 = 65535u16;
        let rug_fuzz_13 = 2u8;
        let rug_fuzz_14 = 1u16;
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
        let _rug_ed_tests_llm_16_1337_llm_16_1337_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1338_llm_16_1338 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn pow_basic() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_basic = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 3usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(8u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_basic = 0;
    }
    #[test]
    fn pow_zero_exponent() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_zero_exponent = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 0usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(1u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_zero_exponent = 0;
    }
    #[test]
    fn pow_one_exponent() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_one_exponent = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 1usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(2u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_one_exponent = 0;
    }
    #[test]
    fn pow_zero_base() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_zero_base = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 3usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(0u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_zero_base = 0;
    }
    #[test]
    fn pow_one_base() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_one_base = 0;
        let rug_fuzz_0 = 1u16;
        let rug_fuzz_1 = 3usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(1u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_one_base = 0;
    }
    #[test]
    fn pow_large_exponent() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_large_exponent = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 16usize;
        debug_assert_eq!(Wrapping(rug_fuzz_0).pow(rug_fuzz_1), Wrapping(0u16));
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_large_exponent = 0;
    }
    #[test]
    #[should_panic]
    fn pow_overflow() {
        let _rug_st_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_overflow = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 15usize;
        let rug_fuzz_2 = 2u32;
        let _result = Wrapping(rug_fuzz_0)
            .pow(rug_fuzz_1)
            .0
            .overflowing_pow(rug_fuzz_2)
            .1;
        let _rug_ed_tests_llm_16_1338_llm_16_1338_rrrruuuugggg_pow_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1342_llm_16_1342 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow_wrapping_u32() {
        let _rug_st_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32 = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 4usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(16u32));
        let _rug_ed_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32 = 0;
    }
    #[test]
    fn test_pow_wrapping_u32_ref() {
        let _rug_st_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_ref = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 4usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, Wrapping(16u32));
        let _rug_ed_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_ref = 0;
    }
    #[test]
    fn test_pow_wrapping_u32_zero() {
        let _rug_st_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_zero = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 0usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1u32));
        let _rug_ed_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_u32_large() {
        let _rug_st_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_large = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 31usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(1u32 << 31));
        let _rug_ed_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_large = 0;
    }
    #[test]
    fn test_pow_wrapping_u32_overflow() {
        let _rug_st_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_overflow = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 32usize;
        let base = Wrapping(rug_fuzz_0);
        let exponent = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, Wrapping(0u32));
        let _rug_ed_tests_llm_16_1342_llm_16_1342_rrrruuuugggg_test_pow_wrapping_u32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1343_llm_16_1343 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1343_llm_16_1343_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 5;
        let base: Wrapping<u64> = Wrapping(rug_fuzz_0);
        let exp: u8 = rug_fuzz_1;
        let result = <Wrapping<u64> as Pow<&u8>>::pow(base, &exp);
        debug_assert_eq!(result, Wrapping(2u64.pow(5)));
        let _rug_ed_tests_llm_16_1343_llm_16_1343_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1345_llm_16_1345 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn wrapping_pow_test() {
        let _rug_st_tests_llm_16_1345_llm_16_1345_rrrruuuugggg_wrapping_pow_test = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 5u8;
        let rug_fuzz_2 = 7u64;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 0u64;
        let rug_fuzz_5 = 8u8;
        let rug_fuzz_6 = 1u8;
        let rug_fuzz_7 = 2u8;
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
        let _rug_ed_tests_llm_16_1345_llm_16_1345_rrrruuuugggg_wrapping_pow_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1346_llm_16_1346 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1346_llm_16_1346_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert_eq!(result, Wrapping(8u64));
        let _rug_ed_tests_llm_16_1346_llm_16_1346_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1348_llm_16_1348 {
    use std::num::Wrapping;
    use std::ops::Mul;
    #[test]
    fn test_pow_wrapping_u8() {
        let _rug_st_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8 = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 8u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = base.mul(base).mul(base);
        debug_assert_eq!(Wrapping(rug_fuzz_2), result);
        let _rug_ed_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8 = 0;
    }
    #[test]
    fn test_pow_wrapping_u8_max() {
        let _rug_st_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_max = 0;
        let rug_fuzz_0 = 1u32;
        let base = Wrapping(u8::MAX);
        let exp = rug_fuzz_0;
        let result = base;
        debug_assert_eq!(Wrapping(u8::MAX), result);
        let _rug_ed_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_max = 0;
    }
    #[test]
    fn test_pow_wrapping_u8_zero() {
        let _rug_st_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_zero = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 10u32;
        let rug_fuzz_2 = 0u8;
        let rug_fuzz_3 = 0u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Wrapping(rug_fuzz_3), result);
        let _rug_ed_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_zero = 0;
    }
    #[test]
    fn test_pow_wrapping_u8_one() {
        let _rug_st_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_one = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 100u32;
        let rug_fuzz_2 = 1u8;
        let rug_fuzz_3 = 1u8;
        let base = Wrapping(rug_fuzz_0);
        let exp = rug_fuzz_1;
        let result = Wrapping(rug_fuzz_2);
        debug_assert_eq!(Wrapping(rug_fuzz_3), result);
        let _rug_ed_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_one = 0;
    }
    #[test]
    fn test_pow_wrapping_u8_wrapping() {
        let _rug_st_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_wrapping = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 1u8;
        let base = Wrapping(u8::MAX);
        let exp = rug_fuzz_0;
        let result = base.mul(base);
        debug_assert_eq!(Wrapping(rug_fuzz_1), result);
        let _rug_ed_tests_llm_16_1348_llm_16_1348_rrrruuuugggg_test_pow_wrapping_u8_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1352_llm_16_1352 {
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn pow_wrapped_usize() {
        let _rug_st_tests_llm_16_1352_llm_16_1352_rrrruuuugggg_pow_wrapped_usize = 0;
        let rug_fuzz_0 = 2usize;
        let rug_fuzz_1 = 3usize;
        let base = Wrapping(rug_fuzz_0);
        let exp = &rug_fuzz_1;
        let result = <Wrapping<usize> as Pow<&usize>>::pow(base, exp);
        debug_assert_eq!(result, Wrapping(8usize));
        let _rug_ed_tests_llm_16_1352_llm_16_1352_rrrruuuugggg_pow_wrapped_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1452_llm_16_1452 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_u16_ref() {
        let _rug_st_tests_llm_16_1452_llm_16_1452_rrrruuuugggg_test_u128_pow_u16_ref = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 0u16;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 1u16;
        let rug_fuzz_4 = 2u128;
        let rug_fuzz_5 = 2u16;
        let rug_fuzz_6 = 2u128;
        let rug_fuzz_7 = 3u16;
        let rug_fuzz_8 = 2u128;
        let rug_fuzz_9 = 4u16;
        let rug_fuzz_10 = 0u128;
        let rug_fuzz_11 = 0u16;
        let rug_fuzz_12 = 0u128;
        let rug_fuzz_13 = 1u16;
        let rug_fuzz_14 = 0u128;
        let rug_fuzz_15 = 2u16;
        let rug_fuzz_16 = 1u128;
        let rug_fuzz_17 = 0u16;
        let rug_fuzz_18 = 1u128;
        let rug_fuzz_19 = 1u16;
        let rug_fuzz_20 = 1u128;
        let rug_fuzz_21 = 100u16;
        let rug_fuzz_22 = 10u128;
        let rug_fuzz_23 = 3u16;
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
        let _rug_ed_tests_llm_16_1452_llm_16_1452_rrrruuuugggg_test_u128_pow_u16_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1453_llm_16_1453 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_pow_ref_u32() {
        let _rug_st_tests_llm_16_1453_llm_16_1453_rrrruuuugggg_test_pow_u128_pow_ref_u32 = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 1u32;
        let rug_fuzz_4 = 2u128;
        let rug_fuzz_5 = 2u32;
        let rug_fuzz_6 = 2u128;
        let rug_fuzz_7 = 3u32;
        let rug_fuzz_8 = 2u128;
        let rug_fuzz_9 = 4u32;
        let rug_fuzz_10 = 10u128;
        let rug_fuzz_11 = 5u32;
        let rug_fuzz_12 = 0u128;
        let rug_fuzz_13 = 10u32;
        let rug_fuzz_14 = 0u128;
        let rug_fuzz_15 = 0u32;
        let rug_fuzz_16 = 1u128;
        let rug_fuzz_17 = 100u32;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 16u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 100000u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_12, & rug_fuzz_13), 0u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_14, & rug_fuzz_15), 1u128);
        debug_assert_eq!(Pow::pow(rug_fuzz_16, & rug_fuzz_17), 1u128);
        let _rug_ed_tests_llm_16_1453_llm_16_1453_rrrruuuugggg_test_pow_u128_pow_ref_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1454_llm_16_1454 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_ref_u8() {
        let _rug_st_tests_llm_16_1454_llm_16_1454_rrrruuuugggg_test_pow_u128_with_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 100);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 81);
        debug_assert_eq!(< u128 as Pow < & u8 > > ::pow(u128::MAX, & rug_fuzz_10), 1);
        debug_assert_eq!(
            < u128 as Pow < & u8 > > ::pow(u128::MAX, & rug_fuzz_11), u128::MAX
        );
        let _rug_ed_tests_llm_16_1454_llm_16_1454_rrrruuuugggg_test_pow_u128_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1455_llm_16_1455 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u128_with_reference_usize() {
        let _rug_st_tests_llm_16_1455_llm_16_1455_rrrruuuugggg_test_pow_u128_with_reference_usize = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 3usize;
        let rug_fuzz_2 = 0u128;
        let rug_fuzz_3 = 0usize;
        let rug_fuzz_4 = 0u128;
        let rug_fuzz_5 = 10usize;
        let rug_fuzz_6 = 10u128;
        let rug_fuzz_7 = 1usize;
        let rug_fuzz_8 = 10u128;
        let rug_fuzz_9 = 2usize;
        let rug_fuzz_10 = 2u128;
        let rug_fuzz_11 = 0usize;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 10);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 100);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_1455_llm_16_1455_rrrruuuugggg_test_pow_u128_with_reference_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1456_llm_16_1456 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1456_llm_16_1456_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4u16;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3u16;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0u16;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1u16;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0u16;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1u16;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 5u16;
        let rug_fuzz_14 = 0u16;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 127u16;
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
        let _rug_ed_tests_llm_16_1456_llm_16_1456_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1457_llm_16_1457 {
    use crate::pow::Pow;
    #[test]
    fn test_u128_pow_u32() {
        let _rug_st_tests_llm_16_1457_llm_16_1457_rrrruuuugggg_test_u128_pow_u32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 10;
        let rug_fuzz_17 = 4;
        let rug_fuzz_18 = 10;
        let rug_fuzz_19 = 5;
        let rug_fuzz_20 = 0;
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
        let _rug_ed_tests_llm_16_1457_llm_16_1457_rrrruuuugggg_test_u128_pow_u32 = 0;
    }
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
        let _rug_st_tests_llm_16_1459_llm_16_1459_rrrruuuugggg_pow_usize_for_u128 = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2u128;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2u128;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0u128;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0u128;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 0u128;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 1u128;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 1u128;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 1u128;
        let rug_fuzz_19 = 2;
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
        let _rug_ed_tests_llm_16_1459_llm_16_1459_rrrruuuugggg_pow_usize_for_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1557_llm_16_1557 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1557_llm_16_1557_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u16;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 3u16;
        let rug_fuzz_3 = 4u16;
        let rug_fuzz_4 = 0u16;
        let rug_fuzz_5 = 0u16;
        let rug_fuzz_6 = 0u16;
        let rug_fuzz_7 = 1u16;
        let rug_fuzz_8 = 1u16;
        let rug_fuzz_9 = 10u16;
        let rug_fuzz_10 = 10u16;
        let rug_fuzz_11 = 3u16;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 81);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_10, & rug_fuzz_11), 1000);
        let _rug_ed_tests_llm_16_1557_llm_16_1557_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1558_llm_16_1558 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1558_llm_16_1558_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 4;
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1024);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< u16 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 81);
        let _rug_ed_tests_llm_16_1558_llm_16_1558_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1559_llm_16_1559 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u16_ref_u8() {
        let _rug_st_tests_llm_16_1559_llm_16_1559_rrrruuuugggg_test_pow_u16_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2u8;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1u8;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0u8;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1u8;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 4u8;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 3u8;
        let rug_fuzz_14 = 10;
        let rug_fuzz_15 = 3u8;
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
        let _rug_ed_tests_llm_16_1559_llm_16_1559_rrrruuuugggg_test_pow_u16_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1560_llm_16_1560 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u16() {
        let _rug_st_tests_llm_16_1560_llm_16_1560_rrrruuuugggg_test_pow_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(
            < u16 as Pow < & usize > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1000
        );
        let _rug_ed_tests_llm_16_1560_llm_16_1560_rrrruuuugggg_test_pow_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1561 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1561_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 4;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 0;
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
        let _rug_ed_tests_llm_16_1561_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1562_llm_16_1562 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1562_llm_16_1562_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 4;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 0;
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 100);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 81);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(u16::MAX, rug_fuzz_12), u16::MAX);
        debug_assert_eq!(< u16 as Pow < u32 > > ::pow(u16::MAX, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_1562_llm_16_1562_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1563_llm_16_1563 {
    use crate::pow::Pow;
    #[test]
    fn u16_pow_u8() {
        let _rug_st_tests_llm_16_1563_llm_16_1563_rrrruuuugggg_u16_pow_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 255;
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 10);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u16 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_1563_llm_16_1563_rrrruuuugggg_u16_pow_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1662_llm_16_1662 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1662_llm_16_1662_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1024);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 81);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 1);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< u32 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(
            < u32 as Pow < & u16 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1000
        );
        let _rug_ed_tests_llm_16_1662_llm_16_1662_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1663_llm_16_1663 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1663_llm_16_1663_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 5;
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 9);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(< u32 as Pow < & u32 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(
            < u32 as Pow < & u32 > > ::pow(rug_fuzz_12, & rug_fuzz_13), 3125
        );
        let _rug_ed_tests_llm_16_1663_llm_16_1663_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1664_llm_16_1664 {
    use crate::pow::Pow;
    #[test]
    fn test_u32_pow_ref_u8() {
        let _rug_st_tests_llm_16_1664_llm_16_1664_rrrruuuugggg_test_u32_pow_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 2;
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 100);
        let _rug_ed_tests_llm_16_1664_llm_16_1664_rrrruuuugggg_test_u32_pow_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1665_llm_16_1665 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow = 0;
    }
    #[test]
    fn test_pow_zero_exponent() {
        let _rug_st_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_zero_exponent = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 1);
        let _rug_ed_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_zero_exponent = 0;
    }
    #[test]
    fn test_pow_one_base() {
        let _rug_st_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_one_base = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 10;
        let base: u32 = rug_fuzz_0;
        let exp: usize = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 1);
        let _rug_ed_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_one_base = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_overflow() {
        let _rug_st_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_overflow = 0;
        let rug_fuzz_0 = 2;
        let base: u32 = u32::MAX;
        let exp: usize = rug_fuzz_0;
        let _ = Pow::pow(base, &exp);
        let _rug_ed_tests_llm_16_1665_llm_16_1665_rrrruuuugggg_test_pow_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1666_llm_16_1666 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1666_llm_16_1666_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4u16;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3u16;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0u16;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1u16;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0u16;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2u16;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 10u16;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 16u16;
        let rug_fuzz_16 = 5;
        let rug_fuzz_17 = 5u16;
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_12, rug_fuzz_13), 1024);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_14, rug_fuzz_15), 65536);
        debug_assert_eq!(< u32 as Pow < u16 > > ::pow(rug_fuzz_16, rug_fuzz_17), 3125);
        let _rug_ed_tests_llm_16_1666_llm_16_1666_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1667_llm_16_1667 {
    use crate::pow::Pow;
    #[test]
    fn pow_for_u32() {
        let _rug_st_tests_llm_16_1667_llm_16_1667_rrrruuuugggg_pow_for_u32 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1000);
        let _rug_ed_tests_llm_16_1667_llm_16_1667_rrrruuuugggg_pow_for_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1668_llm_16_1668 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1668_llm_16_1668_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 4u8;
        let rug_fuzz_2 = 3u32;
        let rug_fuzz_3 = 0u8;
        let rug_fuzz_4 = 0u32;
        let rug_fuzz_5 = 5u8;
        let rug_fuzz_6 = 5u32;
        let rug_fuzz_7 = 1u8;
        let rug_fuzz_8 = 2u32;
        let rug_fuzz_9 = 8u8;
        let rug_fuzz_10 = 9u32;
        let rug_fuzz_11 = 3u8;
        let rug_fuzz_12 = 0u8;
        let rug_fuzz_13 = 1u8;
        let rug_fuzz_14 = 2u8;
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 5u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 256u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 729u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_12), 1u32);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_13), u32::MAX);
        debug_assert_eq!(< u32 as Pow < u8 > > ::pow(u32::MAX, rug_fuzz_14), 1u32);
        let _rug_ed_tests_llm_16_1668_llm_16_1668_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1669_llm_16_1669 {
    use crate::pow::Pow;
    #[test]
    fn u32_pow_usize() {
        let _rug_st_tests_llm_16_1669_llm_16_1669_rrrruuuugggg_u32_pow_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 4;
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 100);
        debug_assert_eq!(< u32 as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 81);
        let _rug_ed_tests_llm_16_1669_llm_16_1669_rrrruuuugggg_u32_pow_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1767_llm_16_1767 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u64_with_ref_u16() {
        let _rug_st_tests_llm_16_1767_llm_16_1767_rrrruuuugggg_test_pow_u64_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
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
        let _rug_ed_tests_llm_16_1767_llm_16_1767_rrrruuuugggg_test_pow_u64_with_ref_u16 = 0;
    }
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
        let _rug_st_tests_llm_16_1769_llm_16_1769_rrrruuuugggg_test_pow_u64_by_ref_u8 = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 2u64;
        let rug_fuzz_3 = 1u8;
        let rug_fuzz_4 = 2u64;
        let rug_fuzz_5 = 2u8;
        let rug_fuzz_6 = 2u64;
        let rug_fuzz_7 = 3u8;
        let rug_fuzz_8 = 3u64;
        let rug_fuzz_9 = 4u8;
        debug_assert_eq!(Pow::pow(rug_fuzz_0, & rug_fuzz_1), 1);
        debug_assert_eq!(Pow::pow(rug_fuzz_2, & rug_fuzz_3), 2);
        debug_assert_eq!(Pow::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(Pow::pow(rug_fuzz_6, & rug_fuzz_7), 8);
        debug_assert_eq!(Pow::pow(rug_fuzz_8, & rug_fuzz_9), 81);
        let _rug_ed_tests_llm_16_1769_llm_16_1769_rrrruuuugggg_test_pow_u64_by_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1771_llm_16_1771 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1771_llm_16_1771_rrrruuuugggg_test_pow = 0;
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
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 10;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 100;
        let rug_fuzz_22 = 1_000_000;
        let rug_fuzz_23 = 2;
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
        let _rug_ed_tests_llm_16_1771_llm_16_1771_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1772_llm_16_1772 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1772_llm_16_1772_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 4;
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 10);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< u64 as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 81);
        let _rug_ed_tests_llm_16_1772_llm_16_1772_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1773_llm_16_1773 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1773_llm_16_1773_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1u8;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2u8;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3u8;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 4u8;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 2u8;
        let rug_fuzz_12 = 4;
        let rug_fuzz_13 = 2u8;
        let rug_fuzz_14 = 5;
        let rug_fuzz_15 = 3u8;
        let rug_fuzz_16 = 10;
        let rug_fuzz_17 = 5u8;
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 9);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_12, rug_fuzz_13), 16);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_14, rug_fuzz_15), 125);
        debug_assert_eq!(< u64 as Pow < u8 > > ::pow(rug_fuzz_16, rug_fuzz_17), 100000);
        let _rug_ed_tests_llm_16_1773_llm_16_1773_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1774_llm_16_1774 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1774_llm_16_1774_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 4;
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 5);
        debug_assert_eq!(< u64 as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 81);
        let _rug_ed_tests_llm_16_1774_llm_16_1774_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1873_llm_16_1873 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8_ref_u16() {
        let _rug_st_tests_llm_16_1873_llm_16_1873_rrrruuuugggg_test_pow_u8_ref_u16 = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 0u8;
        let rug_fuzz_3 = 0u16;
        let rug_fuzz_4 = 0u8;
        let rug_fuzz_5 = 10u16;
        let rug_fuzz_6 = 1u8;
        let rug_fuzz_7 = 10u16;
        let rug_fuzz_8 = 10u8;
        let rug_fuzz_9 = 2u16;
        let rug_fuzz_10 = 3u8;
        let rug_fuzz_11 = 4u16;
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1u8);
        debug_assert_eq!(< u8 as Pow < & u16 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 100u8);
        debug_assert_eq!(
            < u8 as Pow < & u16 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 81u8
        );
        let _rug_ed_tests_llm_16_1873_llm_16_1873_rrrruuuugggg_test_pow_u8_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1875_llm_16_1875 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8_ref_u8() {
        let _rug_st_tests_llm_16_1875_llm_16_1875_rrrruuuugggg_test_pow_u8_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 9);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 1);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_8, & rug_fuzz_9), 2);
        debug_assert_eq!(< u8 as Pow < & u8 > > ::pow(rug_fuzz_10, & rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_1875_llm_16_1875_rrrruuuugggg_test_pow_u8_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1876_llm_16_1876 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_u8() {
        let _rug_st_tests_llm_16_1876_llm_16_1876_rrrruuuugggg_test_pow_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_0, & rug_fuzz_1), 1);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_2, & rug_fuzz_3), 2);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_4, & rug_fuzz_5), 4);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_6, & rug_fuzz_7), 8);
        debug_assert_eq!(< u8 as Pow < & usize > > ::pow(rug_fuzz_8, & rug_fuzz_9), 9);
        debug_assert_eq!(
            < u8 as Pow < & usize > > ::pow(rug_fuzz_10, & rug_fuzz_11), 27
        );
        let _rug_ed_tests_llm_16_1876_llm_16_1876_rrrruuuugggg_test_pow_u8 = 0;
    }
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
        let _rug_st_tests_llm_16_1879_llm_16_1879_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< u8 as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        let _rug_ed_tests_llm_16_1879_llm_16_1879_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1978_llm_16_1978 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_with_ref_u16() {
        let _rug_st_tests_llm_16_1978_llm_16_1978_rrrruuuugggg_test_pow_usize_with_ref_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 7;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 4;
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_0, & rug_fuzz_1), 8);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_4, & rug_fuzz_5), 7);
        debug_assert_eq!(< usize as Pow < & u16 > > ::pow(rug_fuzz_6, & rug_fuzz_7), 81);
        let _rug_ed_tests_llm_16_1978_llm_16_1978_rrrruuuugggg_test_pow_usize_with_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1980_llm_16_1980 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_with_ref_u8() {
        let _rug_st_tests_llm_16_1980_llm_16_1980_rrrruuuugggg_test_pow_usize_with_ref_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let base: usize = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_1980_llm_16_1980_rrrruuuugggg_test_pow_usize_with_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1982_llm_16_1982 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u16() {
        let _rug_st_tests_llm_16_1982_llm_16_1982_rrrruuuugggg_test_pow_usize_u16 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2;
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< usize as Pow < u16 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(
            < usize as Pow < u16 > > ::pow(rug_fuzz_8, u16::MAX), usize::pow(2, u16::MAX
            as u32)
        );
        let _rug_ed_tests_llm_16_1982_llm_16_1982_rrrruuuugggg_test_pow_usize_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1983_llm_16_1983 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1983_llm_16_1983_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 10;
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_2, rug_fuzz_3), 27);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< usize as Pow < u32 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_1983_llm_16_1983_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1984_llm_16_1984 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_usize_u8() {
        let _rug_st_tests_llm_16_1984_llm_16_1984_rrrruuuugggg_test_pow_usize_u8 = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 8;
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_0, rug_fuzz_1), 16);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_8, rug_fuzz_9), 100);
        debug_assert_eq!(< usize as Pow < u8 > > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        let _rug_ed_tests_llm_16_1984_llm_16_1984_rrrruuuugggg_test_pow_usize_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1985_llm_16_1985 {
    use crate::pow::Pow;
    #[test]
    fn pow_usize_usize() {
        let _rug_st_tests_llm_16_1985_llm_16_1985_rrrruuuugggg_pow_usize_usize = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 100;
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_2, rug_fuzz_3), 9);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_10, rug_fuzz_11), 2);
        debug_assert_eq!(< usize as Pow < usize > > ::pow(rug_fuzz_12, rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_1985_llm_16_1985_rrrruuuugggg_pow_usize_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2052_llm_16_2052 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_float_impls() {
        let _rug_st_tests_llm_16_2052_llm_16_2052_rrrruuuugggg_test_pow_float_impls = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0f64;
        let rug_fuzz_3 = 1e-6;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_2052_llm_16_2052_rrrruuuugggg_test_pow_float_impls = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2053_llm_16_2053 {
    use crate::Pow;
    #[test]
    fn test_pow_f32() {
        let _rug_st_tests_llm_16_2053_llm_16_2053_rrrruuuugggg_test_pow_f32 = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 3.0f32;
        let rug_fuzz_2 = 8.0f32;
        let base = rug_fuzz_0;
        let exponent = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2053_llm_16_2053_rrrruuuugggg_test_pow_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2054_llm_16_2054 {
    use crate::Pow;
    use std::f64;
    use std::f32;
    #[test]
    fn test_pow_f64_with_f32() {
        let _rug_st_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32 = 0;
    }
    #[test]
    fn test_pow_f64_with_f32_fractional() {
        let _rug_st_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_fractional = 0;
        let rug_fuzz_0 = 8.0;
        let rug_fuzz_1 = 0.33;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = base.powf(exponent as f64);
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_fractional = 0;
    }
    #[test]
    fn test_pow_f64_with_f32_negative() {
        let _rug_st_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 0.25;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_negative = 0;
    }
    #[test]
    fn test_pow_f64_with_f32_zero() {
        let _rug_st_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_zero = 0;
    }
    #[test]
    fn test_pow_f64_with_f32_one() {
        let _rug_st_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_one = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 2.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2054_llm_16_2054_rrrruuuugggg_test_pow_f64_with_f32_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2055 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_ref() {
        let _rug_st_tests_llm_16_2055_rrrruuuugggg_test_pow_f64_ref = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2055_rrrruuuugggg_test_pow_f64_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2056_llm_16_2056 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_2056_llm_16_2056_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, base.powf(exponent));
        let _rug_ed_tests_llm_16_2056_llm_16_2056_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2057_llm_16_2057 {
    use crate::Pow;
    use core::f32;
    #[test]
    fn pow_f32_i16() {
        let _rug_st_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0f32;
        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16 = 0;
    }
    #[test]
    fn pow_f32_i16_negative() {
        let _rug_st_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0.125f32;
        let base: f32 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_negative = 0;
    }
    #[test]
    fn pow_f32_i16_zero() {
        let _rug_st_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1.0f32;
        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_zero = 0;
    }
    #[test]
    fn pow_f32_i16_base_zero() {
        let _rug_st_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_base_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0.0f32;
        let base: f32 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_base_zero = 0;
    }
    #[test]
    fn pow_f32_i16_base_negative() {
        let _rug_st_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_base_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0f32;
        let base: f32 = -rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = -rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2057_llm_16_2057_rrrruuuugggg_pow_f32_i16_base_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2058_llm_16_2058 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_i16() {
        let _rug_st_tests_llm_16_2058_llm_16_2058_rrrruuuugggg_test_pow_f64_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 0;
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
        let _rug_ed_tests_llm_16_2058_llm_16_2058_rrrruuuugggg_test_pow_f64_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2059_llm_16_2059 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_i16() {
        let _rug_st_tests_llm_16_2059_llm_16_2059_rrrruuuugggg_pow_f32_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exp: i16 = rug_fuzz_1;
        let result = Pow::pow(base, &exp);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2059_llm_16_2059_rrrruuuugggg_pow_f32_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2060_llm_16_2060 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_i16() {
        let _rug_st_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16 = 0;
    }
    #[test]
    fn test_pow_f64_i16_negative() {
        let _rug_st_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0.125;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_negative = 0;
    }
    #[test]
    fn test_pow_f64_i16_zero() {
        let _rug_st_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_f64_i16_zero_base_zero_exponent() {
        let _rug_st_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero_base_zero_exponent = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let _ = base.pow(&exponent);
        let _rug_ed_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero_base_zero_exponent = 0;
    }
    #[test]
    fn test_pow_f64_i16_zero_base() {
        let _rug_st_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero_base = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2060_llm_16_2060_rrrruuuugggg_test_pow_f64_i16_zero_base = 0;
    }
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
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32 = 0;
    }
    #[test]
    fn test_pow_f64_neg_i32() {
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_neg_i32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 0.125);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_neg_i32 = 0;
    }
    #[test]
    fn test_pow_f64_zero_i32() {
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_zero_i32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_zero_i32 = 0;
    }
    #[test]
    fn test_pow_f64_i32_zero() {
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_zero = 0;
    }
    #[test]
    fn test_pow_f64_i32_one() {
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_one = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_one = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_f64_i32_nan() {
        let _rug_st_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_nan = 0;
        let rug_fuzz_0 = 3;
        let base: f64 = f64::NAN;
        let exponent: i32 = rug_fuzz_0;
        let _result = <&f64 as Pow<&i32>>::pow(&base, &exponent);
        let _rug_ed_tests_llm_16_2062_llm_16_2062_rrrruuuugggg_test_pow_f64_i32_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2063_llm_16_2063 {
    use crate::pow::Pow;
    #[test]
    fn pow_i32_for_f32() {
        let _rug_st_tests_llm_16_2063_llm_16_2063_rrrruuuugggg_pow_i32_for_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let base: f32 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2063_llm_16_2063_rrrruuuugggg_pow_i32_for_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2064 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_positive_integer() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_positive_integer = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 8.0);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_positive_integer = 0;
    }
    #[test]
    fn test_pow_zero() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 1.0);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_zero = 0;
    }
    #[test]
    fn test_pow_negative_integer() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_negative_integer = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 0.125);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_negative_integer = 0;
    }
    #[test]
    fn test_pow_one() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_one = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 2.0);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_one = 0;
    }
    #[test]
    fn test_pow_fractional_base() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_fractional_base = 0;
        let rug_fuzz_0 = 0.5;
        let rug_fuzz_1 = 2;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(base, & exponent), 0.25);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_fractional_base = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_negative_base_integer_exponent() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_negative_base_integer_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2;
        let base: f64 = -rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let _ = Pow::pow(base, &exponent);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_negative_base_integer_exponent = 0;
    }
    #[test]
    fn test_pow_large_exponent() {
        let _rug_st_tests_llm_16_2064_rrrruuuugggg_test_pow_large_exponent = 0;
        let rug_fuzz_0 = 1.0001;
        let rug_fuzz_1 = 10000;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 3.0;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = Pow::pow(base, &exponent);
        debug_assert!(result > rug_fuzz_2 && result < rug_fuzz_3);
        let _rug_ed_tests_llm_16_2064_rrrruuuugggg_test_pow_large_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2065_llm_16_2065 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i8() {
        let _rug_st_tests_llm_16_2065_llm_16_2065_rrrruuuugggg_test_pow_f32_i8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5.0;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 0;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        debug_assert_eq!(Pow::pow(& base, & exponent), 8.0);
        let base: f32 = rug_fuzz_2;
        let exponent: i8 = -rug_fuzz_3;
        debug_assert_eq!(Pow::pow(& base, & exponent), 0.04);
        let base: f32 = rug_fuzz_4;
        let exponent: i8 = rug_fuzz_5;
        debug_assert_eq!(Pow::pow(& base, & exponent), 1.0);
        let _rug_ed_tests_llm_16_2065_llm_16_2065_rrrruuuugggg_test_pow_f32_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2066_llm_16_2066 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_f64() {
        let _rug_st_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64 = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_i8_f64_overflow() {
        let _rug_st_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_overflow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 127;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let _ = Pow::pow(&base, &exponent);
        let _rug_ed_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_overflow = 0;
    }
    #[test]
    fn test_pow_i8_f64_negative_exponent() {
        let _rug_st_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_negative_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = -rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 0.125);
        let _rug_ed_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_negative_exponent = 0;
    }
    #[test]
    fn test_pow_i8_f64_zero_exponent() {
        let _rug_st_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_zero_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_zero_exponent = 0;
    }
    #[test]
    fn test_pow_i8_f64_one_exponent() {
        let _rug_st_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_one_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_2066_llm_16_2066_rrrruuuugggg_test_pow_i8_f64_one_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2067 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_f32_i8() {
        let _rug_st_tests_llm_16_2067_rrrruuuugggg_test_pow_f32_i8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected, "2.0 to the power of 3 should be 8.0");
        let _rug_ed_tests_llm_16_2067_rrrruuuugggg_test_pow_f32_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2068_llm_16_2068 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_2068_llm_16_2068_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0.0;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 2.0;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 2.0;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 2.0;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 2.0;
        let rug_fuzz_17 = 1.0;
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
        let _rug_ed_tests_llm_16_2068_llm_16_2068_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2070_llm_16_2070 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_ref_u16() {
        let _rug_st_tests_llm_16_2070_llm_16_2070_rrrruuuugggg_test_pow_f64_ref_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 5;
        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 32.0);
        let _rug_ed_tests_llm_16_2070_llm_16_2070_rrrruuuugggg_test_pow_f64_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2071_llm_16_2071 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_with_u16() {
        let _rug_st_tests_llm_16_2071_llm_16_2071_rrrruuuugggg_test_pow_f32_with_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2071_llm_16_2071_rrrruuuugggg_test_pow_f32_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2072_llm_16_2072 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_by_ref_u16() {
        let _rug_st_tests_llm_16_2072_llm_16_2072_rrrruuuugggg_test_pow_f64_by_ref_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 4;
        let base: f64 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = base.pow(&exp);
        debug_assert_eq!(result, 16.0);
        let _rug_ed_tests_llm_16_2072_llm_16_2072_rrrruuuugggg_test_pow_f64_by_ref_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2073_llm_16_2073 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_u8() {
        let _rug_st_tests_llm_16_2073_llm_16_2073_rrrruuuugggg_pow_f32_u8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(&exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2073_llm_16_2073_rrrruuuugggg_pow_f32_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2074_llm_16_2074 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_ref_u8() {
        let _rug_st_tests_llm_16_2074_llm_16_2074_rrrruuuugggg_test_pow_f64_ref_u8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, &exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2074_llm_16_2074_rrrruuuugggg_test_pow_f64_ref_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2076_llm_16_2076 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64() {
        let _rug_st_tests_llm_16_2076_llm_16_2076_rrrruuuugggg_test_pow_f64 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exp: u8 = rug_fuzz_1;
        let result = base.pow(&exp);
        let expected = f64::powi(base, exp.into());
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2076_llm_16_2076_rrrruuuugggg_test_pow_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2077_llm_16_2077 {
    use crate::Pow;
    #[test]
    fn test_pow_f32_ref_with_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_f32 = 0;
    }
    #[test]
    fn test_pow_f32_ref_with_negative_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_negative_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.25);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_negative_f32 = 0;
    }
    #[test]
    fn test_pow_f32_ref_with_zero_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_zero_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_zero_f32 = 0;
    }
    #[test]
    fn test_pow_f32_ref_with_one_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_one_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_one_f32 = 0;
    }
    #[test]
    fn test_pow_f32_ref_with_fractional_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_fractional_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.5;
        let rug_fuzz_2 = 1.4142135;
        let rug_fuzz_3 = 1e-5;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_fractional_f32 = 0;
    }
    #[test]
    fn test_pow_f32_ref_with_large_f32() {
        let _rug_st_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_large_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 10.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1024.0);
        let _rug_ed_tests_llm_16_2077_llm_16_2077_rrrruuuugggg_test_pow_f32_ref_with_large_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2079_llm_16_2079 {
    use crate::Pow;
    #[test]
    fn pow_f32() {
        let _rug_st_tests_llm_16_2079_llm_16_2079_rrrruuuugggg_pow_f32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = Pow::pow(base, exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2079_llm_16_2079_rrrruuuugggg_pow_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2080_llm_16_2080 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_f64() {
        let _rug_st_tests_llm_16_2080_llm_16_2080_rrrruuuugggg_test_pow_f32_f64 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let rug_fuzz_3 = 0.0001;
        let base: f64 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = <f64 as Pow<f32>>::pow(base, exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_2080_llm_16_2080_rrrruuuugggg_test_pow_f32_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2081_llm_16_2081 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_2081_llm_16_2081_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2081_llm_16_2081_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2082_llm_16_2082 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64() {
        let _rug_st_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64 = 0;
    }
    #[test]
    fn test_pow_f64_zero() {
        let _rug_st_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.0;
        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_zero = 0;
    }
    #[test]
    fn test_pow_f64_one() {
        let _rug_st_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_one = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1.0;
        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_one = 0;
    }
    #[test]
    fn test_pow_f64_fraction() {
        let _rug_st_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_fraction = 0;
        let rug_fuzz_0 = 4.0;
        let rug_fuzz_1 = 0.5;
        let base: f64 = rug_fuzz_0;
        let exp: f64 = rug_fuzz_1;
        let result: f64 = base.pow(exp);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_fraction = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero or for a divisor that does not fit into an i32"
    )]
    fn test_pow_f64_negative() {
        let _rug_st_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_negative = 0;
        let rug_fuzz_0 = 4.0;
        let rug_fuzz_1 = 1.0;
        let base: f64 = rug_fuzz_0;
        let exp: f64 = -rug_fuzz_1;
        let _result: f64 = base.pow(exp);
        let _rug_ed_tests_llm_16_2082_llm_16_2082_rrrruuuugggg_test_pow_f64_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2083_llm_16_2083 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i16() {
        let _rug_st_tests_llm_16_2083_llm_16_2083_rrrruuuugggg_test_pow_f32_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3.0;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 9.0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0;
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
        let _rug_ed_tests_llm_16_2083_llm_16_2083_rrrruuuugggg_test_pow_f32_i16 = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_f32_i16_panic() {
        let _rug_st_tests_llm_16_2083_llm_16_2083_rrrruuuugggg_test_pow_f32_i16_panic = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1;
        let base: f32 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let _ = Pow::pow(&base, exponent);
        let _rug_ed_tests_llm_16_2083_llm_16_2083_rrrruuuugggg_test_pow_f32_i16_panic = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2084_llm_16_2084 {
    use crate::Pow;
    #[test]
    fn test_pow_for_f64_with_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_i16_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_i16_exponent = 0;
    }
    #[test]
    fn test_pow_for_f64_with_negative_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_negative_i16_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0.125;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_negative_i16_exponent = 0;
    }
    #[test]
    fn test_pow_for_f64_with_zero_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_zero_i16_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_f64_with_zero_i16_exponent = 0;
    }
    #[test]
    fn test_pow_for_zero_f64_with_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_i16_exponent = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_i16_exponent = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_pow_for_zero_f64_with_negative_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_negative_i16_exponent = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = -rug_fuzz_1;
        let _result = Pow::pow(&base, exponent);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_negative_i16_exponent = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_pow_for_zero_f64_with_zero_i16_exponent() {
        let _rug_st_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_zero_i16_exponent = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let _result = Pow::pow(&base, exponent);
        let _rug_ed_tests_llm_16_2084_llm_16_2084_rrrruuuugggg_test_pow_for_zero_f64_with_zero_i16_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2085_llm_16_2085 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i16() {
        let _rug_st_tests_llm_16_2085_llm_16_2085_rrrruuuugggg_test_pow_f32_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0f32;
        let rug_fuzz_3 = 4.0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0.0625f32;
        let rug_fuzz_6 = 1.5;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1.0f32;
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
        let _rug_ed_tests_llm_16_2085_llm_16_2085_rrrruuuugggg_test_pow_f32_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2086_llm_16_2086 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_i16() {
        let _rug_st_tests_llm_16_2086_llm_16_2086_rrrruuuugggg_test_pow_f64_i16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: i16 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2086_llm_16_2086_rrrruuuugggg_test_pow_f64_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2087_llm_16_2087 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_positive_exponent() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_positive_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_positive_exponent = 0;
    }
    #[test]
    fn test_pow_zero_exponent() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_zero_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_zero_exponent = 0;
    }
    #[test]
    fn test_pow_negative_exponent() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_negative_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0.125;
        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_negative_exponent = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_special_case_nan() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_nan = 0;
        let rug_fuzz_0 = 2;
        let base: &f32 = &f32::NAN;
        let exponent: i32 = rug_fuzz_0;
        let _ = base.pow(exponent);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_nan = 0;
    }
    #[test]
    fn test_pow_special_case_infinity() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_infinity = 0;
        let rug_fuzz_0 = 2;
        let base: &f32 = &f32::INFINITY;
        let exponent: i32 = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, f32::INFINITY);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_infinity = 0;
    }
    #[test]
    fn test_pow_special_case_negative_infinity() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_negative_infinity = 0;
        let rug_fuzz_0 = 3;
        let base: &f32 = &f32::NEG_INFINITY;
        let exponent: i32 = rug_fuzz_0;
        let result = base.pow(exponent);
        debug_assert_eq!(result, f32::NEG_INFINITY);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_negative_infinity = 0;
    }
    #[test]
    fn test_pow_special_case_zero() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_zero = 0;
    }
    #[test]
    #[should_panic]
    fn test_pow_special_case_zero_negative_exponent() {
        let _rug_st_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_zero_negative_exponent = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: &f32 = &rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let _ = base.pow(exponent);
        let _rug_ed_tests_llm_16_2087_llm_16_2087_rrrruuuugggg_test_pow_special_case_zero_negative_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2088_llm_16_2088 {
    use crate::pow::Pow;
    #[test]
    fn pow_f64_i32() {
        let _rug_st_tests_llm_16_2088_llm_16_2088_rrrruuuugggg_pow_f64_i32 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 2;
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
        let _rug_ed_tests_llm_16_2088_llm_16_2088_rrrruuuugggg_pow_f64_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2090 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow_positive_exponent() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_positive_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_positive_exponent = 0;
    }
    #[test]
    fn test_pow_zero_exponent() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_zero_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_zero_exponent = 0;
    }
    #[test]
    fn test_pow_negative_exponent() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_negative_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.125);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_negative_exponent = 0;
    }
    #[test]
    fn test_pow_one_exponent() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_one_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_one_exponent = 0;
    }
    #[test]
    fn test_pow_one_base() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_one_base = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 10;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_one_base = 0;
    }
    #[test]
    fn test_pow_zero_base() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_zero_base = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_zero_base = 0;
    }
    #[test]
    fn test_pow_large_exponent() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_large_exponent = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 30;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1073741824.0);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_large_exponent = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_pow_overflow() {
        let _rug_st_tests_llm_16_2090_rrrruuuugggg_test_pow_overflow = 0;
        let rug_fuzz_0 = 2.0;
        let base: f64 = rug_fuzz_0;
        let exponent: i32 = i32::MAX;
        let _result = base.pow(exponent);
        let _rug_ed_tests_llm_16_2090_rrrruuuugggg_test_pow_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2091_llm_16_2091 {
    use crate::pow::Pow;
    #[test]
    fn pow_i8_for_f32_reference() {
        let _rug_st_tests_llm_16_2091_llm_16_2091_rrrruuuugggg_pow_i8_for_f32_reference = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0.25;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0.0;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 2.5;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1.0;
        let rug_fuzz_12 = 2.5;
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
        let _rug_ed_tests_llm_16_2091_llm_16_2091_rrrruuuugggg_pow_i8_for_f32_reference = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2092_llm_16_2092 {
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_i8_f64_ref() {
        let _rug_st_tests_llm_16_2092_llm_16_2092_rrrruuuugggg_test_pow_i8_f64_ref = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_2092_llm_16_2092_rrrruuuugggg_test_pow_i8_f64_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2093_llm_16_2093 {
    use super::*;
    use crate::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow_f32_i8() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8 = 0;
    }
    #[test]
    fn test_pow_f32_negative_i8() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_negative_i8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0.125;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = -rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_negative_i8 = 0;
    }
    #[test]
    fn test_pow_f32_i8_zero() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_zero = 0;
    }
    #[test]
    fn test_pow_f32_i8_zero_base() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_zero_base = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_zero_base = 0;
    }
    #[test]
    fn test_pow_f32_i8_one_base() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_one_base = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 3;
        let base: f32 = rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_one_base = 0;
    }
    #[test]
    fn test_pow_f32_i8_negative_base_even() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_negative_base_even = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 4;
        let base: f32 = -rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, 16.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_negative_base_even = 0;
    }
    #[test]
    fn test_pow_f32_i8_negative_base_odd() {
        let _rug_st_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_negative_base_odd = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f32 = -rug_fuzz_0;
        let exponent: i8 = rug_fuzz_1;
        let result = base.pow(exponent);
        debug_assert_eq!(result, - 8.0);
        let _rug_ed_tests_llm_16_2093_llm_16_2093_rrrruuuugggg_test_pow_f32_i8_negative_base_odd = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2096_llm_16_2096 {
    use crate::Pow;
    #[test]
    fn test_pow_f64_u16() {
        let _rug_st_tests_llm_16_2096_llm_16_2096_rrrruuuugggg_test_pow_f64_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2096_llm_16_2096_rrrruuuugggg_test_pow_f64_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2097_llm_16_2097 {
    use crate::pow::Pow;
    #[test]
    fn pow_f32_u16() {
        let _rug_st_tests_llm_16_2097_llm_16_2097_rrrruuuugggg_pow_f32_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let base: f32 = rug_fuzz_0;
        let exp: u16 = rug_fuzz_1;
        let result = base.pow(exp);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_2097_llm_16_2097_rrrruuuugggg_pow_f32_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2098_llm_16_2098 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_with_u16() {
        let _rug_st_tests_llm_16_2098_llm_16_2098_rrrruuuugggg_test_pow_f64_with_u16 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 1024.0;
        let base: f64 = rug_fuzz_0;
        let exponent: u16 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_2098_llm_16_2098_rrrruuuugggg_test_pow_f64_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2100_llm_16_2100 {
    use crate::pow::Pow;
    #[test]
    fn pow_f64_by_u8() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8 = 0;
    }
    #[test]
    fn pow_f64_by_u8_zero() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_zero = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_zero = 0;
    }
    #[test]
    fn pow_f64_by_u8_one() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_one = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&base, exponent);
        debug_assert_eq!(result, base);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_one = 0;
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn pow_f64_by_u8_negative() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&(-base), exponent);
        debug_assert_eq!(result, - 8.0);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_negative = 0;
    }
    #[test]
    fn pow_f64_by_u8_fraction() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_fraction = 0;
        let rug_fuzz_0 = 8.0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1.0;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = Pow::pow(&(rug_fuzz_2 / base), exponent);
        debug_assert_eq!(result, 1.0 / base);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_fraction = 0;
    }
    #[test]
    #[should_panic]
    fn pow_f64_by_u8_overflow() {
        let _rug_st_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_overflow = 0;
        let rug_fuzz_0 = 2;
        let base: f64 = f64::MAX;
        let exponent: u8 = rug_fuzz_0;
        let _result = Pow::pow(&base, exponent);
        let _rug_ed_tests_llm_16_2100_llm_16_2100_rrrruuuugggg_pow_f64_by_u8_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2102_llm_16_2102 {
    use crate::pow::Pow;
    #[test]
    fn test_pow_f64_u8() {
        let _rug_st_tests_llm_16_2102_llm_16_2102_rrrruuuugggg_test_pow_f64_u8 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0;
        let rug_fuzz_3 = 1e-10;
        let base: f64 = rug_fuzz_0;
        let exponent: u8 = rug_fuzz_1;
        let result = base.pow(exponent);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_2102_llm_16_2102_rrrruuuugggg_test_pow_f64_u8 = 0;
    }
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
        let _rug_st_tests_llm_16_2103_llm_16_2103_rrrruuuugggg_test_pow_with_wrapping = 0;
        let rug_fuzz_0 = 3i32;
        let rug_fuzz_1 = 4usize;
        let rug_fuzz_2 = 2u32;
        let rug_fuzz_3 = 0usize;
        let rug_fuzz_4 = 0u32;
        let rug_fuzz_5 = 0usize;
        let rug_fuzz_6 = 2i32;
        let rug_fuzz_7 = 5usize;
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
        let _rug_ed_tests_llm_16_2103_llm_16_2103_rrrruuuugggg_test_pow_with_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_rug_66 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_66_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let p0: u8 = rug_fuzz_0;
        let p1: u16 = rug_fuzz_1;
        debug_assert_eq!(< u8 as Pow < u16 > > ::pow(p0, p1), 8);
        let _rug_ed_tests_rug_66_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_67 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_67_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 2;
        let mut p0: &u8 = &rug_fuzz_0;
        let mut p1: u16 = rug_fuzz_1;
        p0.pow(p1);
        let _rug_ed_tests_rug_67_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_68 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_68_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let mut p0: u8 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< u8 as Pow < & u32 > > ::pow(p0, & p1), 8);
        let _rug_ed_tests_rug_68_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_69_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2u8;
        let rug_fuzz_1 = 3usize;
        let mut p0: u8 = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< u8 as Pow < usize > > ::pow(p0, p1), 8);
        let _rug_ed_tests_rug_69_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_70 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let mut p0: &i8 = &2i8;
        let mut p1: usize = 3usize;
        assert_eq!(<&'static i8 as Pow < usize >>::pow(p0, p1), 8);
    }
}
#[cfg(test)]
mod tests_rug_71 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_71_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let mut p0: u16 = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< u16 as Pow < usize > > ::pow(p0, p1), 8);
        let _rug_ed_tests_rug_71_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_72_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let p0: u16 = rug_fuzz_0;
        let p1: usize = rug_fuzz_1;
        let p0_ref: &u16 = &p0;
        let p1_ref: &usize = &p1;
        debug_assert_eq!(< & u16 as Pow < & usize > > ::pow(p0_ref, p1_ref), 8);
        let _rug_ed_tests_rug_72_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_73 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_73_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let mut p0: i16 = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let result = (&p0).pow(p1);
        debug_assert_eq!(result, 81);
        let _rug_ed_tests_rug_73_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_74 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_74_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< & u32 as Pow < u32 > > ::pow(& p0, p1), 8);
        let _rug_ed_tests_rug_74_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_76 {
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_76_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let mut p0: &i32 = &rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< & i32 as Pow < usize > > ::pow(p0, p1), 8);
        let _rug_ed_tests_rug_76_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_77 {
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_77_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let p0: u64 = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let p1_ref: &usize = &p1;
        debug_assert_eq!(< u64 as Pow < & usize > > ::pow(p0, p1_ref), 8);
        let _rug_ed_tests_rug_77_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_78 {
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_78_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let mut p0: &i64 = &rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(p0.pow(p1), 100);
        let _rug_ed_tests_rug_78_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_79 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_79_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123456789012345678901234567890u128;
        let rug_fuzz_1 = 5u32;
        let mut p0: u128 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        p0.pow(p1);
        let _rug_ed_tests_rug_79_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_81 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_81_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 123456789123456789i128;
        let rug_fuzz_1 = 3;
        let mut p0: i128 = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let result = (&p0).pow(p1);
        debug_assert_eq!(result, 1881676371789154860897069i128);
        let _rug_ed_tests_rug_81_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_82 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_82_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let p0: usize = rug_fuzz_0;
        let p1: u16 = rug_fuzz_1;
        debug_assert_eq!(< & usize as Pow < u16 > > ::pow(& p0, p1), 81);
        let _rug_ed_tests_rug_82_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_83 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_83_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 2;
        let p0: usize = rug_fuzz_0;
        let p1: &u32 = &rug_fuzz_1;
        debug_assert_eq!(< usize as Pow < & u32 > > ::pow(p0, p1), 9);
        let _rug_ed_tests_rug_83_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_84 {
    use super::*;
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_84_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< usize as Pow < & usize > > ::pow(p0, & p1), 81);
        let _rug_ed_tests_rug_84_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_85 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_85_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let mut p0: isize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< isize as Pow < usize > > ::pow(p0, p1), 81);
        let _rug_ed_tests_rug_85_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_86 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_86_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: u8 = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u8 > as Pow < u8 > > ::pow(p0, p1), Wrapping(42u8.pow(3))
        );
        let _rug_ed_tests_rug_86_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_87 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_87_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 3u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = &rug_fuzz_1;
        <Wrapping<u8> as Pow<&u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_87_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_88 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_pow() {
        let p0 = &Wrapping(42u8);
        let p1: u8 = 5;
        assert_eq!(
            <&'static Wrapping < u8 > as Pow < u8 >>::pow(p0, p1), Wrapping(42u8.pow(5))
        );
    }
}
#[cfg(test)]
mod tests_rug_89 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_89_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 3;
        let mut p0: Wrapping<u8> = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        <Wrapping<u8> as Pow<usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_89_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_90 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_90_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< & Wrapping < u8 > > ::pow(& p0, p1), Wrapping(42u8.pow(3)));
        let _rug_ed_tests_rug_90_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_91 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_91_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        <Wrapping<i8> as Pow<u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_91_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_92 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_92_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3;
        let p0 = Wrapping(rug_fuzz_0);
        let p1: &u8 = &rug_fuzz_1;
        debug_assert_eq!(Wrapping:: < i8 > ::pow(p0, p1), Wrapping(42i8.pow(3)));
        let _rug_ed_tests_rug_92_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_93_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: u8 = rug_fuzz_1;
        debug_assert_eq!(< & Wrapping < i8 > > ::pow(& p0, p1), Wrapping(42i8.pow(3)));
        let _rug_ed_tests_rug_93_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_94_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3usize;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        <Wrapping<i8> as Pow<usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_94_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_95_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3;
        let mut p0: Wrapping<i8> = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        Wrapping::<i8>::pow(p0, p1);
        let _rug_ed_tests_rug_95_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_96_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(< & Wrapping < i8 > > ::pow(& p0, p1), Wrapping(42i8.pow(3)));
        let _rug_ed_tests_rug_96_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_97 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_97_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42i16;
        let rug_fuzz_1 = 5u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = &rug_fuzz_1;
        <Wrapping<i16> as Pow<&u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_97_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_98 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_98_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42i16;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(
            < & Wrapping < i16 > as Pow < usize > > ::pow(& p0, p1), Wrapping(42i16
            .pow(3))
        );
        let _rug_ed_tests_rug_98_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_99 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_99_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42u32;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: u8 = rug_fuzz_1;
        let result = Wrapping::<u32>::pow(p0, p1);
        debug_assert_eq!(result, Wrapping(74088u32));
        let _rug_ed_tests_rug_99_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_100 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let mut p0 = std::num::Wrapping(42u32);
        let mut p1: &u8 = &3;
        assert_eq!(
            < std::num::Wrapping < u32 > as Pow <&'static u8 >>::pow(p0, p1),
            std::num::Wrapping(42u32.pow(3))
        );
    }
}
#[cfg(test)]
mod tests_rug_101 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_101_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42u32;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        let result = Wrapping::<u32>::pow(p0, p1);
        debug_assert_eq!(result, Wrapping(42u32.pow(3)));
        let _rug_ed_tests_rug_101_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let mut p0: &Wrapping<u32> = &Wrapping(42u32);
        let mut p1: &usize = &3;
        <&'static Wrapping<u32> as Pow<&usize>>::pow(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_103_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let mut p0 = Wrapping::<i32>(rug_fuzz_0);
        let mut p1: &u8 = &rug_fuzz_1;
        <std::num::Wrapping<i32>>::pow(p0, p1);
        let _rug_ed_tests_rug_103_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_104_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping::<i32>(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        <Wrapping<i32> as Pow<usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_104_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_105 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_105_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let mut p0_wrapped = Wrapping::<i32>(rug_fuzz_0);
        let p0 = &p0_wrapped;
        let p1: usize = rug_fuzz_1;
        debug_assert_eq!(< & _ as Pow < usize > > ::pow(p0, p1), Wrapping(100));
        let _rug_ed_tests_rug_105_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_106 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_106_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1234u64;
        let rug_fuzz_1 = 5;
        let mut p0: Wrapping<u64> = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        <Wrapping<u64> as Pow<&usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_106_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_107_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1234u64;
        let rug_fuzz_1 = 5usize;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        debug_assert_eq!(
            < & Wrapping < u64 > > ::pow(& p0, & p1), Wrapping(2861381721051424u64)
        );
        let _rug_ed_tests_rug_107_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_108 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_108_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 123i64;
        let rug_fuzz_1 = 45u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = &rug_fuzz_1;
        let result = <Wrapping<i64> as Pow<&u8>>::pow(p0, p1);
        debug_assert_eq!(result, Wrapping(123i64.pow(* p1 as u32)));
        let _rug_ed_tests_rug_108_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_109 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_109_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123i64;
        let rug_fuzz_1 = 5;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < i64 > as Pow < usize > > ::pow(p0, p1),
            Wrapping(2867971860299718107i64)
        );
        let _rug_ed_tests_rug_109_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_110 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_110_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123i64;
        let rug_fuzz_1 = 2usize;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        <Wrapping<i64> as Pow<&usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_110_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_111_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: &u8 = &rug_fuzz_1;
        debug_assert_eq!(
            < Wrapping < u128 > as Pow < & u8 > > ::pow(p0, p1), Wrapping(1000)
        );
        let _rug_ed_tests_rug_111_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_112_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        debug_assert_eq!(
            < std::num::Wrapping < u128 > as Pow < & usize > > ::pow(p0, p1),
            Wrapping(1000u128)
        );
        let _rug_ed_tests_rug_112_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_113 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_113_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let p1: usize = rug_fuzz_1;
        debug_assert_eq!(p0.pow(p1), Wrapping(1000u128));
        let _rug_ed_tests_rug_113_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_114 {
    use std::num::Wrapping;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_114_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 8u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = &rug_fuzz_1;
        debug_assert_eq!(Wrapping:: < i128 > ::pow(p0, p1), Wrapping(123_i128.pow(8)));
        let _rug_ed_tests_rug_114_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_115_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 5;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: u8 = rug_fuzz_1;
        <&Wrapping<i128> as Pow<u8>>::pow(&p0, p1);
        let _rug_ed_tests_rug_115_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_116_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        let result = Wrapping::<i128>::pow(p0, p1);
        debug_assert_eq!(result, Wrapping(1860867_i128));
        let _rug_ed_tests_rug_116_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_117 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_117_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 2;
        let mut p0: Wrapping<i128> = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        <Wrapping<i128> as Pow<&usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_117_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_118_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 4;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: usize = rug_fuzz_1;
        p0.pow(p1);
        let _rug_ed_tests_rug_118_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_119 {
    use super::*;
    use crate::pow::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_119_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12usize;
        let rug_fuzz_1 = 3u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        <std::num::Wrapping<usize> as Pow<u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_119_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_120 {
    use super::*;
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_120_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12usize;
        let rug_fuzz_1 = 8u8;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = &rug_fuzz_1;
        <std::num::Wrapping<usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_120_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_121 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_121_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12usize;
        let rug_fuzz_1 = 3usize;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1 = rug_fuzz_1;
        <std::num::Wrapping<usize> as Pow<usize>>::pow(p0, p1);
        let _rug_ed_tests_rug_121_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_122 {
    use std::num::Wrapping;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_122_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42isize;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: u8 = rug_fuzz_1;
        <Wrapping<isize> as Pow<u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_122_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_123 {
    use crate::Pow;
    use std::num::Wrapping;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_123_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 42isize;
        let rug_fuzz_1 = 3;
        let mut p0 = Wrapping(rug_fuzz_0);
        let mut p1: &usize = &rug_fuzz_1;
        let result = <Wrapping<isize> as Pow<&usize>>::pow(p0, p1);
        debug_assert_eq!(result, Wrapping(74088isize));
        let _rug_ed_tests_rug_123_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_124 {
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_124_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 5;
        let mut p0: f32 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        <f32 as Pow<u8>>::pow(p0, p1);
        let _rug_ed_tests_rug_124_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_125_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 8;
        let p0: f32 = rug_fuzz_0;
        let p1: &u8 = &rug_fuzz_1;
        debug_assert_eq!(< f32 > ::pow(p0, p1), 256.0);
        let _rug_ed_tests_rug_125_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_126_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let mut p0: f32 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        let result = p0.pow(p1);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_rug_126_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_127 {
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_127_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let mut p0: f32 = rug_fuzz_0;
        let mut p1: u16 = rug_fuzz_1;
        debug_assert_eq!(p0.pow(p1), 8.0);
        let _rug_ed_tests_rug_127_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_128 {
    use super::*;
    use crate::Pow;
    #[test]
    fn test_rug() {
        let mut p0: &'static f32 = &2.0f32;
        let mut p1: &'static u16 = &3;
        assert_eq!(<&'static f32 as Pow <&'static u16 >>::pow(p0, p1), 8.0);
    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_129_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let mut p0: f32 = rug_fuzz_0;
        let mut p1: i32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as Pow < i32 > > ::pow(p0, p1), 8.0);
        let _rug_ed_tests_rug_129_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_130 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_130_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let mut p0: f64 = rug_fuzz_0;
        let mut p1: i8 = rug_fuzz_1;
        debug_assert_eq!(< f64 > ::pow(p0, p1), 8.0);
        let _rug_ed_tests_rug_130_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_131 {
    use crate::Pow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_131_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let p0 = &base;
        let p1 = &exponent;
        debug_assert_eq!(< & f32 > ::pow(p0, p1), 8.0);
        let _rug_ed_tests_rug_131_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_132 {
    use super::*;
    use crate::pow::Pow;
    #[test]
    fn test_rug() {
        let mut p0: &f64 = &2.5;
        let mut p1: f32 = 3.0;
        let result = <&'static f64>::pow(p0, p1);
        assert_eq!(result, p0.powf(p1 as f64));
    }
}
