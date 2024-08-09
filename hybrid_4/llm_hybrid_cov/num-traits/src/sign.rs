use core::num::Wrapping;
use core::ops::Neg;
use crate::float::FloatCore;
use crate::Num;
/// Useful functions for signed numbers (i.e. numbers that can be negative).
pub trait Signed: Sized + Num + Neg<Output = Self> {
    /// Computes the absolute value.
    ///
    /// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`.
    ///
    /// For signed integers, `::MIN` will be returned if the number is `::MIN`.
    fn abs(&self) -> Self;
    /// The positive difference of two numbers.
    ///
    /// Returns `zero` if the number is less than or equal to `other`, otherwise the difference
    /// between `self` and `other` is returned.
    fn abs_sub(&self, other: &Self) -> Self;
    /// Returns the sign of the number.
    ///
    /// For `f32` and `f64`:
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * `NaN` if the number is `NaN`
    ///
    /// For signed integers:
    ///
    /// * `0` if the number is zero
    /// * `1` if the number is positive
    /// * `-1` if the number is negative
    fn signum(&self) -> Self;
    /// Returns true if the number is positive and false if the number is zero or negative.
    fn is_positive(&self) -> bool;
    /// Returns true if the number is negative and false if the number is zero or positive.
    fn is_negative(&self) -> bool;
}
macro_rules! signed_impl {
    ($($t:ty)*) => {
        $(impl Signed for $t { #[inline] fn abs(& self) -> $t { if self.is_negative() {
        -* self } else { * self } } #[inline] fn abs_sub(& self, other : &$t) -> $t { if
        * self <= * other { 0 } else { * self - * other } } #[inline] fn signum(& self)
        -> $t { match * self { n if n > 0 => 1, 0 => 0, _ => - 1, } } #[inline] fn
        is_positive(& self) -> bool { * self > 0 } #[inline] fn is_negative(& self) ->
        bool { * self < 0 } })*
    };
}
signed_impl!(isize i8 i16 i32 i64 i128);
impl<T: Signed> Signed for Wrapping<T>
where
    Wrapping<T>: Num + Neg<Output = Wrapping<T>>,
{
    #[inline]
    fn abs(&self) -> Self {
        Wrapping(self.0.abs())
    }
    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        Wrapping(self.0.abs_sub(&other.0))
    }
    #[inline]
    fn signum(&self) -> Self {
        Wrapping(self.0.signum())
    }
    #[inline]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
    #[inline]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}
macro_rules! signed_float_impl {
    ($t:ty) => {
        impl Signed for $t { #[doc =
        " Computes the absolute value. Returns `NAN` if the number is `NAN`."] #[inline]
        fn abs(& self) -> $t { FloatCore::abs(* self) } #[doc =
        " The positive difference of two numbers. Returns `0.0` if the number is"] #[doc
        = " less than or equal to `other`, otherwise the difference between`self`"] #[doc
        = " and `other` is returned."] #[inline] fn abs_sub(& self, other : &$t) -> $t {
        if * self <= * other { 0. } else { * self - * other } } #[doc = " # Returns"]
        #[doc = ""] #[doc = " - `1.0` if the number is positive, `+0.0` or `INFINITY`"]
        #[doc = " - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`"] #[doc =
        " - `NAN` if the number is NaN"] #[inline] fn signum(& self) -> $t {
        FloatCore::signum(* self) } #[doc =
        " Returns `true` if the number is positive, including `+0.0` and `INFINITY`"]
        #[inline] fn is_positive(& self) -> bool { FloatCore::is_sign_positive(* self) }
        #[doc =
        " Returns `true` if the number is negative, including `-0.0` and `NEG_INFINITY`"]
        #[inline] fn is_negative(& self) -> bool { FloatCore::is_sign_negative(* self) }
        }
    };
}
signed_float_impl!(f32);
signed_float_impl!(f64);
/// Computes the absolute value.
///
/// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`
///
/// For signed integers, `::MIN` will be returned if the number is `::MIN`.
#[inline(always)]
pub fn abs<T: Signed>(value: T) -> T {
    value.abs()
}
/// The positive difference of two numbers.
///
/// Returns zero if `x` is less than or equal to `y`, otherwise the difference
/// between `x` and `y` is returned.
#[inline(always)]
pub fn abs_sub<T: Signed>(x: T, y: T) -> T {
    x.abs_sub(&y)
}
/// Returns the sign of the number.
///
/// For `f32` and `f64`:
///
/// * `1.0` if the number is positive, `+0.0` or `INFINITY`
/// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// * `NaN` if the number is `NaN`
///
/// For signed integers:
///
/// * `0` if the number is zero
/// * `1` if the number is positive
/// * `-1` if the number is negative
#[inline(always)]
pub fn signum<T: Signed>(value: T) -> T {
    value.signum()
}
/// A trait for values which cannot be negative
pub trait Unsigned: Num {}
macro_rules! empty_trait_impl {
    ($name:ident for $($t:ty)*) => {
        $(impl $name for $t {})*
    };
}
empty_trait_impl!(Unsigned for usize u8 u16 u32 u64 u128);
impl<T: Unsigned> Unsigned for Wrapping<T>
where
    Wrapping<T>: Num,
{}
#[test]
fn unsigned_wrapping_is_unsigned() {
    fn require_unsigned<T: Unsigned>(_: &T) {}
    require_unsigned(&Wrapping(42_u32));
}
#[test]
fn signed_wrapping_is_signed() {
    fn require_signed<T: Signed>(_: &T) {}
    require_signed(&Wrapping(-42));
}
#[cfg(test)]
mod tests_llm_16_426_llm_16_426 {
    use crate::Signed;
    #[test]
    fn test_abs_positive() {
        let _rug_st_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_positive = 0;
        let rug_fuzz_0 = 3.14;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as Signed > ::abs(& x), 3.14);
        let _rug_ed_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_positive = 0;
    }
    #[test]
    fn test_abs_negative() {
        let _rug_st_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_negative = 0;
        let rug_fuzz_0 = 3.14;
        let x: f32 = -rug_fuzz_0;
        debug_assert_eq!(< f32 as Signed > ::abs(& x), 3.14);
        let _rug_ed_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_negative = 0;
    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_zero = 0;
        let rug_fuzz_0 = 0.0;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as Signed > ::abs(& x), 0.0);
        let _rug_ed_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_zero = 0;
    }
    #[test]
    fn test_abs_nan() {
        let _rug_st_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_nan = 0;
        let x: f32 = f32::NAN;
        debug_assert!(< f32 as Signed > ::abs(& x).is_nan());
        let _rug_ed_tests_llm_16_426_llm_16_426_rrrruuuugggg_test_abs_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_427_llm_16_427 {
    use super::*;
    use crate::*;
    #[test]
    fn test_abs_sub_positive() {
        let _rug_st_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_positive = 0;
        let rug_fuzz_0 = 5.0;
        let rug_fuzz_1 = 3.0;
        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as Signed > ::abs_sub(& x, & y), 2.0);
        let _rug_ed_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_positive = 0;
    }
    #[test]
    fn test_abs_sub_negative() {
        let _rug_st_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_negative = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 5.0;
        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as Signed > ::abs_sub(& x, & y), 0.0);
        let _rug_ed_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_negative = 0;
    }
    #[test]
    fn test_abs_sub_zero() {
        let _rug_st_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_zero = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 3.0;
        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as Signed > ::abs_sub(& x, & y), 0.0);
        let _rug_ed_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_zero = 0;
    }
    #[test]
    fn test_abs_sub_negative_numbers() {
        let _rug_st_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_negative_numbers = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 5.0;
        let x: f32 = -rug_fuzz_0;
        let y: f32 = -rug_fuzz_1;
        debug_assert_eq!(< f32 as Signed > ::abs_sub(& x, & y), 2.0);
        let _rug_ed_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_negative_numbers = 0;
    }
    #[test]
    fn test_abs_sub_one_negative_one_positive() {
        let _rug_st_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_one_negative_one_positive = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 5.0;
        let x: f32 = -rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as Signed > ::abs_sub(& x, & y), 0.0);
        let _rug_ed_tests_llm_16_427_llm_16_427_rrrruuuugggg_test_abs_sub_one_negative_one_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_428_llm_16_428 {
    use crate::sign::Signed;
    use crate::float::FloatCore;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_428_llm_16_428_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 1.0;
        debug_assert!(< f32 as Signed > ::is_negative(& - rug_fuzz_0));
        debug_assert!(< f32 as Signed > ::is_negative(& - rug_fuzz_1));
        debug_assert!(< f32 as Signed > ::is_negative(& f32::NEG_INFINITY));
        debug_assert!(! < f32 as Signed > ::is_negative(& rug_fuzz_2));
        debug_assert!(! < f32 as Signed > ::is_negative(& rug_fuzz_3));
        debug_assert!(! < f32 as Signed > ::is_negative(& f32::INFINITY));
        debug_assert!(! < f32 as Signed > ::is_negative(& f32::NAN));
        let _rug_ed_tests_llm_16_428_llm_16_428_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_429_llm_16_429 {
    use crate::sign::Signed;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_llm_16_429_llm_16_429_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 1.0;
        debug_assert!(< f32 as Signed > ::is_positive(& rug_fuzz_0));
        debug_assert!(< f32 as Signed > ::is_positive(& rug_fuzz_1));
        debug_assert!(< f32 as Signed > ::is_positive(& f32::INFINITY));
        debug_assert!(! < f32 as Signed > ::is_positive(& - rug_fuzz_2));
        debug_assert!(! < f32 as Signed > ::is_positive(& - rug_fuzz_3));
        debug_assert!(! < f32 as Signed > ::is_positive(& f32::NEG_INFINITY));
        debug_assert!(< f32 as Signed > ::is_positive(& f32::MIN_POSITIVE));
        let _rug_ed_tests_llm_16_429_llm_16_429_rrrruuuugggg_test_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_430_llm_16_430 {
    use crate::sign::Signed;
    #[test]
    fn test_signum() {
        let _rug_st_tests_llm_16_430_llm_16_430_rrrruuuugggg_test_signum = 0;
        let rug_fuzz_0 = 1.0_f32;
        let rug_fuzz_1 = 1.0_f32;
        let rug_fuzz_2 = 0.0_f32;
        let rug_fuzz_3 = 0.0_f32;
        debug_assert_eq!(rug_fuzz_0.signum(), 1.0);
        debug_assert_eq!((- rug_fuzz_1).signum(), - 1.0);
        debug_assert_eq!(rug_fuzz_2.signum(), 1.0);
        debug_assert_eq!((- rug_fuzz_3).signum(), - 1.0);
        debug_assert_eq!(f32::INFINITY.signum(), 1.0);
        debug_assert_eq!(f32::NEG_INFINITY.signum(), - 1.0);
        debug_assert!(f32::NAN.signum().is_nan());
        let _rug_ed_tests_llm_16_430_llm_16_430_rrrruuuugggg_test_signum = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_595_llm_16_595 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_595_llm_16_595_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 3.5;
        let rug_fuzz_1 = 2.5;
        let rug_fuzz_2 = 4.0;
        let rug_fuzz_3 = 3.5;
        let rug_fuzz_4 = 2.5;
        let rug_fuzz_5 = 3.5;
        let a: f64 = rug_fuzz_0;
        let b: f64 = rug_fuzz_1;
        let c: f64 = rug_fuzz_2;
        let d: f64 = rug_fuzz_3;
        debug_assert_eq!(Signed::abs_sub(& a, & b), 1.0);
        debug_assert_eq!(Signed::abs_sub(& a, & c), 0.0);
        debug_assert_eq!(Signed::abs_sub(& a, & d), 0.0);
        let e: f64 = -rug_fuzz_4;
        let f: f64 = -rug_fuzz_5;
        debug_assert_eq!(Signed::abs_sub(& e, & f), 1.0);
        debug_assert_eq!(Signed::abs_sub(& a, & f), 7.0);
        let _rug_ed_tests_llm_16_595_llm_16_595_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_596 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_596_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 0.0;
        debug_assert!(< f64 as sign::Signed > ::is_negative(& - rug_fuzz_0));
        debug_assert!(< f64 as sign::Signed > ::is_negative(& - rug_fuzz_1));
        debug_assert!(< f64 as sign::Signed > ::is_negative(& std::f64::NEG_INFINITY));
        debug_assert!(! < f64 as sign::Signed > ::is_negative(& rug_fuzz_2));
        debug_assert!(! < f64 as sign::Signed > ::is_negative(& rug_fuzz_3));
        debug_assert!(! < f64 as sign::Signed > ::is_negative(& std::f64::INFINITY));
        debug_assert!(! < f64 as sign::Signed > ::is_negative(& std::f64::NAN));
        let _rug_ed_tests_llm_16_596_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_598_llm_16_598 {
    use crate::Signed;
    use core::f64;
    use core::f64::NAN;
    #[test]
    fn test_signum_positive() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive = 0;
        let rug_fuzz_0 = 1.0f64;
        debug_assert_eq!(rug_fuzz_0.signum(), 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive = 0;
    }
    #[test]
    fn test_signum_positive_zero() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.signum(), 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive_zero = 0;
    }
    #[test]
    fn test_signum_positive_infinity() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive_infinity = 0;
        debug_assert_eq!(f64::INFINITY.signum(), 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_positive_infinity = 0;
    }
    #[test]
    fn test_signum_negative() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative = 0;
        let rug_fuzz_0 = 1.0f64;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative = 0;
    }
    #[test]
    fn test_signum_negative_zero() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative_zero = 0;
    }
    #[test]
    fn test_signum_negative_infinity() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative_infinity = 0;
        debug_assert_eq!(f64::NEG_INFINITY.signum(), - 1.0);
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_negative_infinity = 0;
    }
    #[test]
    fn test_signum_nan() {
        let _rug_st_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_nan = 0;
        debug_assert!(NAN.signum().is_nan());
        let _rug_ed_tests_llm_16_598_llm_16_598_rrrruuuugggg_test_signum_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_704_llm_16_704 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_positive() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_positive = 0;
        let rug_fuzz_0 = 123;
        let positive_value: i128 = rug_fuzz_0;
        debug_assert_eq!(< i128 as Signed > ::abs(& positive_value), 123);
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_positive = 0;
    }
    #[test]
    fn test_abs_negative() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_negative = 0;
        let rug_fuzz_0 = 123;
        let negative_value: i128 = -rug_fuzz_0;
        debug_assert_eq!(< i128 as Signed > ::abs(& negative_value), 123);
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_negative = 0;
    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let zero_value: i128 = rug_fuzz_0;
        debug_assert_eq!(< i128 as Signed > ::abs(& zero_value), 0);
        let _rug_ed_tests_llm_16_704_llm_16_704_rrrruuuugggg_test_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_705 {
    use super::*;
    use crate::*;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_705_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 15;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 15;
        let rug_fuzz_8 = 15;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 10;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 10;
        let rug_fuzz_14 = 10;
        let rug_fuzz_15 = 15;
        let rug_fuzz_16 = 15;
        let rug_fuzz_17 = 10;
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 5
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& - rug_fuzz_8, & - rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& - rug_fuzz_10, & rug_fuzz_11), 0
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& rug_fuzz_12, & - rug_fuzz_13), 20
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& - rug_fuzz_14, & rug_fuzz_15), 0
        );
        debug_assert_eq!(
            < i128 as sign::Signed > ::abs_sub(& rug_fuzz_16, & - rug_fuzz_17), 25
        );
        let _rug_ed_tests_llm_16_705_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_706 {
    use crate::Signed;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_706_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i128 as Signed > ::is_negative(& - rug_fuzz_0), true);
        debug_assert_eq!(< i128 as Signed > ::is_negative(& rug_fuzz_1), false);
        debug_assert_eq!(< i128 as Signed > ::is_negative(& rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_706_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_707_llm_16_707 {
    use crate::Signed;
    #[test]
    fn test_i128_is_positive() {
        let _rug_st_tests_llm_16_707_llm_16_707_rrrruuuugggg_test_i128_is_positive = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i128 as Signed > ::is_positive(& rug_fuzz_0), false);
        debug_assert_eq!(< i128 as Signed > ::is_positive(& rug_fuzz_1), true);
        debug_assert_eq!(< i128 as Signed > ::is_positive(& - rug_fuzz_2), false);
        debug_assert_eq!(< i128 as Signed > ::is_positive(& i128::MAX), true);
        debug_assert_eq!(< i128 as Signed > ::is_positive(& i128::MIN), false);
        let _rug_ed_tests_llm_16_707_llm_16_707_rrrruuuugggg_test_i128_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_708_llm_16_708 {
    use crate::sign::Signed;
    #[test]
    fn test_signum_positive() {
        let _rug_st_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_positive = 0;
        let rug_fuzz_0 = 5i128;
        debug_assert_eq!(rug_fuzz_0.signum(), 1);
        let _rug_ed_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_positive = 0;
    }
    #[test]
    fn test_signum_negative() {
        let _rug_st_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_negative = 0;
        let rug_fuzz_0 = 5i128;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1);
        let _rug_ed_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_negative = 0;
    }
    #[test]
    fn test_signum_zero() {
        let _rug_st_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_zero = 0;
        let rug_fuzz_0 = 0i128;
        debug_assert_eq!(rug_fuzz_0.signum(), 0);
        let _rug_ed_tests_llm_16_708_llm_16_708_rrrruuuugggg_test_signum_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_814_llm_16_814 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_positive() {
        let _rug_st_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_positive = 0;
        let rug_fuzz_0 = 42;
        let x: i16 = rug_fuzz_0;
        debug_assert_eq!(< i16 as Signed > ::abs(& x), 42);
        let _rug_ed_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_positive = 0;
    }
    #[test]
    fn test_abs_negative() {
        let _rug_st_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_negative = 0;
        let rug_fuzz_0 = 42;
        let x: i16 = -rug_fuzz_0;
        debug_assert_eq!(< i16 as Signed > ::abs(& x), 42);
        let _rug_ed_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_negative = 0;
    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let x: i16 = rug_fuzz_0;
        debug_assert_eq!(< i16 as Signed > ::abs(& x), 0);
        let _rug_ed_tests_llm_16_814_llm_16_814_rrrruuuugggg_test_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_815 {
    use super::*;
    use crate::*;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_815_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 10;
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 5
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& - rug_fuzz_8, & - rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& rug_fuzz_10, & - rug_fuzz_11), 15
        );
        debug_assert_eq!(
            < i16 as sign::Signed > ::abs_sub(& - rug_fuzz_12, & rug_fuzz_13), 0
        );
        let _rug_ed_tests_llm_16_815_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_816 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_816_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i16 as sign::Signed > ::is_negative(& - rug_fuzz_0), true);
        debug_assert_eq!(< i16 as sign::Signed > ::is_negative(& rug_fuzz_1), false);
        debug_assert_eq!(< i16 as sign::Signed > ::is_negative(& rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_816_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_817 {
    use super::*;
    use crate::*;
    #[test]
    fn is_positive_tests() {
        let _rug_st_tests_llm_16_817_rrrruuuugggg_is_positive_tests = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i16 as sign::Signed > ::is_positive(& rug_fuzz_0), false);
        debug_assert_eq!(< i16 as sign::Signed > ::is_positive(& rug_fuzz_1), true);
        debug_assert_eq!(< i16 as sign::Signed > ::is_positive(& - rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_817_rrrruuuugggg_is_positive_tests = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_818_llm_16_818 {
    use crate::sign::Signed;
    #[test]
    fn signum_positive() {
        let _rug_st_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_positive = 0;
        let rug_fuzz_0 = 5i16;
        debug_assert_eq!(rug_fuzz_0.signum(), 1);
        let _rug_ed_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_positive = 0;
    }
    #[test]
    fn signum_zero() {
        let _rug_st_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_zero = 0;
        let rug_fuzz_0 = 0i16;
        debug_assert_eq!(rug_fuzz_0.signum(), 0);
        let _rug_ed_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_zero = 0;
    }
    #[test]
    fn signum_negative() {
        let _rug_st_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_negative = 0;
        let rug_fuzz_0 = 5i16;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1);
        let _rug_ed_tests_llm_16_818_llm_16_818_rrrruuuugggg_signum_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_924_llm_16_924 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_positive() {
        let _rug_st_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_positive = 0;
        let rug_fuzz_0 = 42;
        let positive = rug_fuzz_0;
        debug_assert_eq!(< i32 as Signed > ::abs(& positive), 42);
        let _rug_ed_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_positive = 0;
    }
    #[test]
    fn test_abs_negative() {
        let _rug_st_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_negative = 0;
        let rug_fuzz_0 = 42;
        let negative = -rug_fuzz_0;
        debug_assert_eq!(< i32 as Signed > ::abs(& negative), 42);
        let _rug_ed_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_negative = 0;
    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let zero = rug_fuzz_0;
        debug_assert_eq!(< i32 as Signed > ::abs(& zero), 0);
        let _rug_ed_tests_llm_16_924_llm_16_924_rrrruuuugggg_test_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_925_llm_16_925 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_925_llm_16_925_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 6;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 6;
        let rug_fuzz_8 = 6;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 3);
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0);
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 3);
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& - rug_fuzz_8, & - rug_fuzz_9), 0);
        debug_assert_eq!(< i32 as Signed > ::abs_sub(& rug_fuzz_10, & rug_fuzz_11), 0);
        let _rug_ed_tests_llm_16_925_llm_16_925_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_926 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_926_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i32 as sign::Signed > ::is_negative(& rug_fuzz_0), false);
        debug_assert_eq!(< i32 as sign::Signed > ::is_negative(& - rug_fuzz_1), true);
        debug_assert_eq!(< i32 as sign::Signed > ::is_negative(& rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_926_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_927 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_llm_16_927_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i32 as sign::Signed > ::is_positive(& rug_fuzz_0), false);
        debug_assert_eq!(< i32 as sign::Signed > ::is_positive(& rug_fuzz_1), true);
        debug_assert_eq!(< i32 as sign::Signed > ::is_positive(& - rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_927_rrrruuuugggg_test_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_928_llm_16_928 {
    use crate::sign::Signed;
    #[test]
    fn signum_positive() {
        let _rug_st_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_positive = 0;
        let rug_fuzz_0 = 5i32;
        debug_assert_eq!(rug_fuzz_0.signum(), 1);
        let _rug_ed_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_positive = 0;
    }
    #[test]
    fn signum_zero() {
        let _rug_st_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_zero = 0;
        let rug_fuzz_0 = 0i32;
        debug_assert_eq!(rug_fuzz_0.signum(), 0);
        let _rug_ed_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_zero = 0;
    }
    #[test]
    fn signum_negative() {
        let _rug_st_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_negative = 0;
        let rug_fuzz_0 = 5i32;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1);
        let _rug_ed_tests_llm_16_928_llm_16_928_rrrruuuugggg_signum_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1034_llm_16_1034 {
    use crate::sign::Signed;
    #[test]
    fn test_abs_positive() {
        let _rug_st_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_positive = 0;
        let rug_fuzz_0 = 42;
        let x: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as Signed > ::abs(& x), 42);
        let _rug_ed_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_positive = 0;
    }
    #[test]
    fn test_abs_negative() {
        let _rug_st_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_negative = 0;
        let rug_fuzz_0 = 42;
        let x: i64 = -rug_fuzz_0;
        debug_assert_eq!(< i64 as Signed > ::abs(& x), 42);
        let _rug_ed_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_negative = 0;
    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let x: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as Signed > ::abs(& x), 0);
        let _rug_ed_tests_llm_16_1034_llm_16_1034_rrrruuuugggg_test_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1035 {
    use super::*;
    use crate::*;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_1035_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 5
        );
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& - rug_fuzz_8, & - rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < i64 as sign::Signed > ::abs_sub(& rug_fuzz_10, & rug_fuzz_11), 0
        );
        let _rug_ed_tests_llm_16_1035_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1036 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_1036_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert!(< i64 as sign::Signed > ::is_negative(& - rug_fuzz_0));
        debug_assert!(! < i64 as sign::Signed > ::is_negative(& rug_fuzz_1));
        debug_assert!(! < i64 as sign::Signed > ::is_negative(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_1036_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1037_llm_16_1037 {
    use crate::sign::Signed;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_llm_16_1037_llm_16_1037_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i64 as Signed > ::is_positive(& rug_fuzz_0), false);
        debug_assert_eq!(< i64 as Signed > ::is_positive(& rug_fuzz_1), true);
        debug_assert_eq!(< i64 as Signed > ::is_positive(& - rug_fuzz_2), false);
        debug_assert_eq!(< i64 as Signed > ::is_positive(& i64::MAX), true);
        debug_assert_eq!(< i64 as Signed > ::is_positive(& i64::MIN), false);
        let _rug_ed_tests_llm_16_1037_llm_16_1037_rrrruuuugggg_test_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1038_llm_16_1038 {
    use crate::sign::Signed;
    #[test]
    fn signum_positive() {
        let _rug_st_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_positive = 0;
        let rug_fuzz_0 = 5i64;
        debug_assert_eq!(rug_fuzz_0.signum(), 1);
        let _rug_ed_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_positive = 0;
    }
    #[test]
    fn signum_zero() {
        let _rug_st_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_zero = 0;
        let rug_fuzz_0 = 0i64;
        debug_assert_eq!(rug_fuzz_0.signum(), 0);
        let _rug_ed_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_zero = 0;
    }
    #[test]
    fn signum_negative() {
        let _rug_st_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_negative = 0;
        let rug_fuzz_0 = 5i64;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1);
        let _rug_ed_tests_llm_16_1038_llm_16_1038_rrrruuuugggg_signum_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1144 {
    use super::*;
    use crate::*;
    #[test]
    fn test_i8_abs_positive() {
        let _rug_st_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_positive = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(< i8 as sign::Signed > ::abs(& value), 42);
        let _rug_ed_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_positive = 0;
    }
    #[test]
    fn test_i8_abs_negative() {
        let _rug_st_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        debug_assert_eq!(< i8 as sign::Signed > ::abs(& value), 42);
        let _rug_ed_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_negative = 0;
    }
    #[test]
    fn test_i8_abs_zero() {
        let _rug_st_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(< i8 as sign::Signed > ::abs(& value), 0);
        let _rug_ed_tests_llm_16_1144_rrrruuuugggg_test_i8_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1145 {
    use super::*;
    use crate::*;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_1145_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 5;
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& - rug_fuzz_4, & - rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 5
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& rug_fuzz_8, & rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& - rug_fuzz_10, & rug_fuzz_11), 10
        );
        debug_assert_eq!(
            < i8 as sign::Signed > ::abs_sub(& rug_fuzz_12, & - rug_fuzz_13), 10
        );
        let _rug_ed_tests_llm_16_1145_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1146 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_1146_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        debug_assert!(< i8 as sign::Signed > ::is_negative(& - rug_fuzz_0));
        debug_assert!(! < i8 as sign::Signed > ::is_negative(& rug_fuzz_1));
        debug_assert!(! < i8 as sign::Signed > ::is_negative(& rug_fuzz_2));
        let _rug_ed_tests_llm_16_1146_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1147 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_llm_16_1147_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert!(< i8 as sign::Signed > ::is_positive(& rug_fuzz_0));
        debug_assert!(! < i8 as sign::Signed > ::is_positive(& rug_fuzz_1));
        debug_assert!(! < i8 as sign::Signed > ::is_positive(& - rug_fuzz_2));
        let _rug_ed_tests_llm_16_1147_rrrruuuugggg_test_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1148 {
    use crate::sign::Signed;
    #[test]
    fn signum_i8() {
        let _rug_st_tests_llm_16_1148_rrrruuuugggg_signum_i8 = 0;
        let rug_fuzz_0 = 5i8;
        let rug_fuzz_1 = 0i8;
        let rug_fuzz_2 = 5i8;
        debug_assert_eq!(rug_fuzz_0.signum(), 1i8);
        debug_assert_eq!(rug_fuzz_1.signum(), 0i8);
        debug_assert_eq!((- rug_fuzz_2).signum(), - 1i8);
        let _rug_ed_tests_llm_16_1148_rrrruuuugggg_signum_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1254_llm_16_1254 {
    use crate::sign::Signed;
    #[test]
    fn abs_positive() {
        let _rug_st_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_positive = 0;
        let rug_fuzz_0 = 5;
        let value: isize = rug_fuzz_0;
        debug_assert_eq!(value.abs(), 5);
        let _rug_ed_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_positive = 0;
    }
    #[test]
    fn abs_negative() {
        let _rug_st_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_negative = 0;
        let rug_fuzz_0 = 5;
        let value: isize = -rug_fuzz_0;
        debug_assert_eq!(value.abs(), 5);
        let _rug_ed_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_negative = 0;
    }
    #[test]
    fn abs_zero() {
        let _rug_st_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let value: isize = rug_fuzz_0;
        debug_assert_eq!(value.abs(), 0);
        let _rug_ed_tests_llm_16_1254_llm_16_1254_rrrruuuugggg_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1255 {
    use super::*;
    use crate::*;
    #[test]
    fn abs_sub_test() {
        let _rug_st_tests_llm_16_1255_rrrruuuugggg_abs_sub_test = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 10;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 5;
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& rug_fuzz_0, & rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& - rug_fuzz_6, & - rug_fuzz_7), 0
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& - rug_fuzz_8, & - rug_fuzz_9), 5
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& rug_fuzz_10, & - rug_fuzz_11), 15
        );
        debug_assert_eq!(
            < isize as sign::Signed > ::abs_sub(& - rug_fuzz_12, & rug_fuzz_13), 0
        );
        let _rug_ed_tests_llm_16_1255_rrrruuuugggg_abs_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1256 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_llm_16_1256_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< isize as sign::Signed > ::is_negative(& - rug_fuzz_0), true);
        debug_assert_eq!(< isize as sign::Signed > ::is_negative(& rug_fuzz_1), false);
        debug_assert_eq!(< isize as sign::Signed > ::is_negative(& rug_fuzz_2), false);
        let _rug_ed_tests_llm_16_1256_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1257 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_llm_16_1257_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< isize as sign::Signed > ::is_positive(& rug_fuzz_0), false);
        debug_assert_eq!(< isize as sign::Signed > ::is_positive(& rug_fuzz_1), true);
        debug_assert_eq!(< isize as sign::Signed > ::is_positive(& - rug_fuzz_2), false);
        debug_assert_eq!(< isize as sign::Signed > ::is_positive(& isize::MAX), true);
        debug_assert_eq!(< isize as sign::Signed > ::is_positive(& isize::MIN), false);
        let _rug_ed_tests_llm_16_1257_rrrruuuugggg_test_is_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1258_llm_16_1258 {
    use crate::sign::Signed;
    #[test]
    fn signum_positive() {
        let _rug_st_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_positive = 0;
        let rug_fuzz_0 = 5isize;
        debug_assert_eq!(rug_fuzz_0.signum(), 1);
        let _rug_ed_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_positive = 0;
    }
    #[test]
    fn signum_zero() {
        let _rug_st_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_zero = 0;
        let rug_fuzz_0 = 0isize;
        debug_assert_eq!(rug_fuzz_0.signum(), 0);
        let _rug_ed_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_zero = 0;
    }
    #[test]
    fn signum_negative() {
        let _rug_st_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_negative = 0;
        let rug_fuzz_0 = 5isize;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1);
        let _rug_ed_tests_llm_16_1258_llm_16_1258_rrrruuuugggg_signum_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1302_llm_16_1302 {
    use crate::Wrapping;
    use crate::sign::Signed;
    #[test]
    fn test_wrapping_abs_positive() {
        let _rug_st_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_positive = 0;
        let rug_fuzz_0 = 5;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.abs(), Wrapping(5));
        let _rug_ed_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_positive = 0;
    }
    #[test]
    fn test_wrapping_abs_negative() {
        let _rug_st_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_negative = 0;
        let rug_fuzz_0 = 5;
        let num = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(num.abs(), Wrapping(5));
        let _rug_ed_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_negative = 0;
    }
    #[test]
    fn test_wrapping_abs_zero() {
        let _rug_st_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_zero = 0;
        let rug_fuzz_0 = 0;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.abs(), Wrapping(0));
        let _rug_ed_tests_llm_16_1302_llm_16_1302_rrrruuuugggg_test_wrapping_abs_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1303_llm_16_1303 {
    use crate::sign::Signed;
    use crate::Wrapping;
    #[test]
    fn abs_sub_with_positive_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_positive_numbers = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 3;
        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(3));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_positive_numbers = 0;
    }
    #[test]
    fn abs_sub_with_negative_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_negative_numbers = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 3;
        let a = Wrapping(-rug_fuzz_0);
        let b = Wrapping(-rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(3));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_negative_numbers = 0;
    }
    #[test]
    fn abs_sub_with_positive_and_negative_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_positive_and_negative_numbers = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 3;
        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(-rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(9));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_positive_and_negative_numbers = 0;
    }
    #[test]
    fn abs_sub_with_negative_and_positive_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_negative_and_positive_numbers = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 3;
        let a = Wrapping(-rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(9));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_negative_and_positive_numbers = 0;
    }
    #[test]
    fn abs_sub_with_zero() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 3;
        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(3));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_zero = 0;
    }
    #[test]
    fn abs_sub_with_same_positive_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_same_positive_numbers = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 3;
        let a = Wrapping(rug_fuzz_0);
        let b = Wrapping(rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(0));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_same_positive_numbers = 0;
    }
    #[test]
    fn abs_sub_with_same_negative_numbers() {
        let _rug_st_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_same_negative_numbers = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 3;
        let a = Wrapping(-rug_fuzz_0);
        let b = Wrapping(-rug_fuzz_1);
        debug_assert_eq!(a.abs_sub(& b), Wrapping(0));
        let _rug_ed_tests_llm_16_1303_llm_16_1303_rrrruuuugggg_abs_sub_with_same_negative_numbers = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1305_llm_16_1305 {
    use crate::Wrapping;
    use crate::Signed;
    #[test]
    fn test_is_positive_for_signed_integer() {
        let _rug_st_tests_llm_16_1305_llm_16_1305_rrrruuuugggg_test_is_positive_for_signed_integer = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 5i32;
        let rug_fuzz_2 = 0i32;
        debug_assert!(Wrapping(rug_fuzz_0).is_positive());
        debug_assert!(! Wrapping(- rug_fuzz_1).is_positive());
        debug_assert!(! Wrapping(rug_fuzz_2).is_positive());
        let _rug_ed_tests_llm_16_1305_llm_16_1305_rrrruuuugggg_test_is_positive_for_signed_integer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1306_llm_16_1306 {
    use crate::sign::Signed;
    use std::num::Wrapping;
    #[test]
    fn signum_positive() {
        let _rug_st_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_positive = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 100i32;
        debug_assert_eq!(Wrapping(rug_fuzz_0).signum(), Wrapping(1i32));
        debug_assert_eq!(Wrapping(rug_fuzz_1).signum(), Wrapping(1i32));
        let _rug_ed_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_positive = 0;
    }
    #[test]
    fn signum_negative() {
        let _rug_st_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_negative = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 100i32;
        debug_assert_eq!(Wrapping(- rug_fuzz_0).signum(), Wrapping(- 1i32));
        debug_assert_eq!(Wrapping(- rug_fuzz_1).signum(), Wrapping(- 1i32));
        let _rug_ed_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_negative = 0;
    }
    #[test]
    fn signum_zero() {
        let _rug_st_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_zero = 0;
        let rug_fuzz_0 = 0i32;
        debug_assert_eq!(Wrapping(rug_fuzz_0).signum(), Wrapping(0i32));
        let _rug_ed_tests_llm_16_1306_llm_16_1306_rrrruuuugggg_signum_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2104_llm_16_2104 {
    use crate::sign::Signed;
    use crate::sign::abs;
    use std::num::Wrapping;
    #[test]
    fn test_abs_positive_value() {
        let _rug_st_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_positive_value = 0;
        let rug_fuzz_0 = 5;
        let value = Wrapping(rug_fuzz_0);
        let result = abs(value);
        debug_assert_eq!(result, Wrapping(5));
        let _rug_ed_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_positive_value = 0;
    }
    #[test]
    fn test_abs_negative_value() {
        let _rug_st_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_negative_value = 0;
        let rug_fuzz_0 = 5;
        let value = Wrapping(-rug_fuzz_0);
        let result = abs(value);
        debug_assert_eq!(result, Wrapping(5));
        let _rug_ed_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_negative_value = 0;
    }
    #[test]
    fn test_abs_zero_value() {
        let _rug_st_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_zero_value = 0;
        let rug_fuzz_0 = 0;
        let value = Wrapping(rug_fuzz_0);
        let result = abs(value);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_zero_value = 0;
    }
    #[test]
    fn test_abs_min_value() {
        let _rug_st_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_min_value = 0;
        let value = Wrapping(i32::MIN);
        let result = abs(value);
        debug_assert_eq!(result, Wrapping(i32::MIN));
        let _rug_ed_tests_llm_16_2104_llm_16_2104_rrrruuuugggg_test_abs_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2105_llm_16_2105 {
    use crate::sign::abs_sub;
    use crate::sign::Signed;
    use std::num::Wrapping;
    #[test]
    fn test_abs_sub_positive() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_positive = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let x = Wrapping(rug_fuzz_0);
        let y = Wrapping(rug_fuzz_1);
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(2));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_positive = 0;
    }
    #[test]
    fn test_abs_sub_negative() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_negative = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 5;
        let x = Wrapping(rug_fuzz_0);
        let y = Wrapping(rug_fuzz_1);
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_negative = 0;
    }
    #[test]
    fn test_abs_sub_zero() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let x = Wrapping(rug_fuzz_0);
        let y = Wrapping(rug_fuzz_1);
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_zero = 0;
    }
    #[test]
    fn test_abs_sub_negative_values() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_negative_values = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 5;
        let x = Wrapping(-rug_fuzz_0);
        let y = Wrapping(-rug_fuzz_1);
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(3));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_negative_values = 0;
    }
    #[test]
    fn test_abs_sub_min_max() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_min_max = 0;
        let x = Wrapping(i32::min_value());
        let y = Wrapping(i32::max_value());
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(0));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_min_max = 0;
    }
    #[test]
    fn test_abs_sub_max_min() {
        let _rug_st_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_max_min = 0;
        let x = Wrapping(i32::max_value());
        let y = Wrapping(i32::min_value());
        let result = abs_sub(x, y);
        debug_assert_eq!(result, Wrapping(i32::max_value() - i32::min_value()));
        let _rug_ed_tests_llm_16_2105_llm_16_2105_rrrruuuugggg_test_abs_sub_max_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2106 {
    use crate::signum;
    use std::num::Wrapping;
    #[test]
    fn signum_positive_integer() {
        let _rug_st_tests_llm_16_2106_rrrruuuugggg_signum_positive_integer = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(signum(Wrapping(rug_fuzz_0)), Wrapping(1));
        debug_assert_eq!(signum(Wrapping(rug_fuzz_1)), Wrapping(1));
        let _rug_ed_tests_llm_16_2106_rrrruuuugggg_signum_positive_integer = 0;
    }
    #[test]
    fn signum_negative_integer() {
        let _rug_st_tests_llm_16_2106_rrrruuuugggg_signum_negative_integer = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(signum(Wrapping(- rug_fuzz_0)), Wrapping(- 1));
        debug_assert_eq!(signum(Wrapping(- rug_fuzz_1)), Wrapping(- 1));
        let _rug_ed_tests_llm_16_2106_rrrruuuugggg_signum_negative_integer = 0;
    }
    #[test]
    fn signum_zero_integer() {
        let _rug_st_tests_llm_16_2106_rrrruuuugggg_signum_zero_integer = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(signum(Wrapping(rug_fuzz_0)), Wrapping(0));
        let _rug_ed_tests_llm_16_2106_rrrruuuugggg_signum_zero_integer = 0;
    }
    #[test]
    fn signum_max_min_integer() {
        let _rug_st_tests_llm_16_2106_rrrruuuugggg_signum_max_min_integer = 0;
        debug_assert_eq!(signum(Wrapping(i32::MAX)), Wrapping(1));
        debug_assert_eq!(signum(Wrapping(i32::MIN)), Wrapping(- 1));
        let _rug_ed_tests_llm_16_2106_rrrruuuugggg_signum_max_min_integer = 0;
    }
}
#[cfg(test)]
mod tests_rug_133 {
    use crate::Signed;
    use std::num::Wrapping;
    use std::ops::Neg;
    use crate::Num;
    #[test]
    fn test_is_negative() {
        let _rug_st_tests_rug_133_rrrruuuugggg_test_is_negative = 0;
        let rug_fuzz_0 = 10;
        let p0: Wrapping<i32> = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(p0.is_negative(), true);
        let _rug_ed_tests_rug_133_rrrruuuugggg_test_is_negative = 0;
    }
}
#[cfg(test)]
mod tests_rug_134 {
    use super::*;
    use crate::sign::Signed;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_134_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0.5;
        let mut p0: f64 = -rug_fuzz_0;
        debug_assert_eq!(< f64 as Signed > ::abs(& p0), 0.5);
        let _rug_ed_tests_rug_134_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_135 {
    use super::*;
    use crate::Signed;
    #[test]
    fn test_is_positive() {
        let _rug_st_tests_rug_135_rrrruuuugggg_test_is_positive = 0;
        let rug_fuzz_0 = 3.14;
        let p0: f64 = rug_fuzz_0;
        debug_assert!(< f64 as Signed > ::is_positive(& p0));
        let _rug_ed_tests_rug_135_rrrruuuugggg_test_is_positive = 0;
    }
}
