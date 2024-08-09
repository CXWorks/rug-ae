use core::ops::{Div, Rem};
pub trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> {
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// This computes the integer `n` such that
    /// `self = n * v + self.rem_euclid(v)`.
    /// In other words, the result is `self / v` rounded to the integer `n`
    /// such that `self >= n * v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::div_euclid(&a, &b), 1); // 7 > 4 * 1
    /// assert_eq!(Euclid::div_euclid(&-a, &b), -2); // -7 >= 4 * -2
    /// assert_eq!(Euclid::div_euclid(&a, &-b), -1); // 7 >= -4 * -1
    /// assert_eq!(Euclid::div_euclid(&-a, &-b), 2); // -7 >= -4 * 2
    /// ```
    fn div_euclid(&self, v: &Self) -> Self;
    /// Calculates the least nonnegative remainder of `self (mod v)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < v.abs()` in
    /// most cases. However, due to a floating point round-off error it can
    /// result in `r == v.abs()`, violating the mathematical definition, if
    /// `self` is much smaller than `v.abs()` in magnitude and `self < 0.0`.
    /// This result is not an element of the function's codomain, but it is the
    /// closest floating point number in the real numbers and thus fulfills the
    /// property `self == self.div_euclid(v) * v + self.rem_euclid(v)`
    /// approximatively.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::Euclid;
    ///
    /// let a: i32 = 7;
    /// let b: i32 = 4;
    /// assert_eq!(Euclid::rem_euclid(&a, &b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &b), 1);
    /// assert_eq!(Euclid::rem_euclid(&a, &-b), 3);
    /// assert_eq!(Euclid::rem_euclid(&-a, &-b), 1);
    /// ```
    fn rem_euclid(&self, v: &Self) -> Self;
}
macro_rules! euclid_forward_impl {
    ($($t:ty)*) => {
        $(#[cfg(has_div_euclid)] impl Euclid for $t { #[inline] fn div_euclid(& self, v :
        &$t) -> Self { <$t >::div_euclid(* self, * v) } #[inline] fn rem_euclid(& self, v
        : &$t) -> Self { <$t >::rem_euclid(* self, * v) } })*
    };
}
macro_rules! euclid_int_impl {
    ($($t:ty)*) => {
        $(euclid_forward_impl!($t); #[cfg(not(has_div_euclid))] impl Euclid for $t {
        #[inline] fn div_euclid(& self, v : &$t) -> Self { let q = self / v; if self % v
        < 0 { return if * v > 0 { q - 1 } else { q + 1 } } q } #[inline] fn rem_euclid(&
        self, v : &$t) -> Self { let r = self % v; if r < 0 { if * v < 0 { r - v } else {
        r + v } } else { r } } })*
    };
}
macro_rules! euclid_uint_impl {
    ($($t:ty)*) => {
        $(euclid_forward_impl!($t); #[cfg(not(has_div_euclid))] impl Euclid for $t {
        #[inline] fn div_euclid(& self, v : &$t) -> Self { self / v } #[inline] fn
        rem_euclid(& self, v : &$t) -> Self { self % v } })*
    };
}
euclid_int_impl!(isize i8 i16 i32 i64 i128);
euclid_uint_impl!(usize u8 u16 u32 u64 u128);
#[cfg(all(has_div_euclid, feature = "std"))]
euclid_forward_impl!(f32 f64);
#[cfg(not(all(has_div_euclid, feature = "std")))]
impl Euclid for f32 {
    #[inline]
    fn div_euclid(&self, v: &f32) -> f32 {
        let q = <f32 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
    #[inline]
    fn rem_euclid(&self, v: &f32) -> f32 {
        let r = self % v;
        if r < 0.0 { r + <f32 as crate::float::FloatCore>::abs(*v) } else { r }
    }
}
#[cfg(not(all(has_div_euclid, feature = "std")))]
impl Euclid for f64 {
    #[inline]
    fn div_euclid(&self, v: &f64) -> f64 {
        let q = <f64 as crate::float::FloatCore>::trunc(self / v);
        if self % v < 0.0 {
            return if *v > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
    #[inline]
    fn rem_euclid(&self, v: &f64) -> f64 {
        let r = self % v;
        if r < 0.0 { r + <f64 as crate::float::FloatCore>::abs(*v) } else { r }
    }
}
pub trait CheckedEuclid: Euclid {
    /// Performs euclid division that returns `None` instead of panicking on division by zero
    /// and instead of wrapping around on underflow and overflow.
    fn checked_div_euclid(&self, v: &Self) -> Option<Self>;
    /// Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and
    /// division by zero. If any of that happens, `None` is returned.
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self>;
}
macro_rules! checked_euclid_forward_impl {
    ($($t:ty)*) => {
        $(#[cfg(has_div_euclid)] impl CheckedEuclid for $t { #[inline] fn
        checked_div_euclid(& self, v : &$t) -> Option < Self > { <$t
        >::checked_div_euclid(* self, * v) } #[inline] fn checked_rem_euclid(& self, v :
        &$t) -> Option < Self > { <$t >::checked_rem_euclid(* self, * v) } })*
    };
}
macro_rules! checked_euclid_int_impl {
    ($($t:ty)*) => {
        $(checked_euclid_forward_impl!($t); #[cfg(not(has_div_euclid))] impl
        CheckedEuclid for $t { #[inline] fn checked_div_euclid(& self, v : &$t) -> Option
        <$t > { if * v == 0 || (* self == Self::min_value() && * v == - 1) { None } else
        { Some(Euclid::div_euclid(self, v)) } } #[inline] fn checked_rem_euclid(& self, v
        : &$t) -> Option <$t > { if * v == 0 || (* self == Self::min_value() && * v == -
        1) { None } else { Some(Euclid::rem_euclid(self, v)) } } })*
    };
}
macro_rules! checked_euclid_uint_impl {
    ($($t:ty)*) => {
        $(checked_euclid_forward_impl!($t); #[cfg(not(has_div_euclid))] impl
        CheckedEuclid for $t { #[inline] fn checked_div_euclid(& self, v : &$t) -> Option
        <$t > { if * v == 0 { None } else { Some(Euclid::div_euclid(self, v)) } }
        #[inline] fn checked_rem_euclid(& self, v : &$t) -> Option <$t > { if * v == 0 {
        None } else { Some(Euclid::rem_euclid(self, v)) } } })*
    };
}
checked_euclid_int_impl!(isize i8 i16 i32 i64 i128);
checked_euclid_uint_impl!(usize u8 u16 u32 u64 u128);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn euclid_unsigned() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $({ let x : $t = 10; let y : $t = 3; assert_eq!(Euclid::div_euclid(& x, &
                y), 3); assert_eq!(Euclid::rem_euclid(& x, & y), 1); })+
            };
        }
        test_euclid!(usize u8 u16 u32 u64);
    }
    #[test]
    fn euclid_signed() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $({ let x : $t = 10; let y : $t = - 3; assert_eq!(Euclid::div_euclid(& x,
                & y), - 3); assert_eq!(Euclid::div_euclid(&- x, & y), 4);
                assert_eq!(Euclid::rem_euclid(& x, & y), 1);
                assert_eq!(Euclid::rem_euclid(&- x, & y), 2); let x : $t = $t
                ::min_value() + 1; let y : $t = - 1; assert_eq!(Euclid::div_euclid(& x, &
                y), $t ::max_value()); })+
            };
        }
        test_euclid!(isize i8 i16 i32 i64 i128);
    }
    #[test]
    fn euclid_float() {
        macro_rules! test_euclid {
            ($($t:ident)+) => {
                $({ let x : $t = 12.1; let y : $t = 3.2; assert!(Euclid::div_euclid(& x,
                & y) * y + Euclid::rem_euclid(& x, & y) - x <= 46.4 * <$t as crate
                ::float::FloatCore >::epsilon()); assert!(Euclid::div_euclid(& x, &- y) *
                - y + Euclid::rem_euclid(& x, &- y) - x <= 46.4 * <$t as crate
                ::float::FloatCore >::epsilon()); assert!(Euclid::div_euclid(&- x, & y) *
                y + Euclid::rem_euclid(&- x, & y) + x <= 46.4 * <$t as crate
                ::float::FloatCore >::epsilon()); assert!(Euclid::div_euclid(&- x, &- y)
                * - y + Euclid::rem_euclid(&- x, &- y) + x <= 46.4 * <$t as crate
                ::float::FloatCore >::epsilon()); })+
            };
        }
        test_euclid!(f32 f64);
    }
    #[test]
    fn euclid_checked() {
        macro_rules! test_euclid_checked {
            ($($t:ident)+) => {
                $({ assert_eq!(CheckedEuclid::checked_div_euclid(&$t ::min_value(), &-
                1), None); assert_eq!(CheckedEuclid::checked_rem_euclid(&$t
                ::min_value(), &- 1), None);
                assert_eq!(CheckedEuclid::checked_div_euclid(& 1, & 0), None);
                assert_eq!(CheckedEuclid::checked_rem_euclid(& 1, & 0), None); })+
            };
        }
        test_euclid_checked!(isize i8 i16 i32 i64 i128);
    }
}
#[cfg(test)]
mod tests_llm_16_421_llm_16_421 {
    use crate::Euclid;
    #[test]
    fn test_div_euclid_f32() {
        let _rug_st_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_div_euclid_f32 = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 10.0;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 10.0;
        let rug_fuzz_5 = 3.0;
        let rug_fuzz_6 = 10.0;
        let rug_fuzz_7 = 3.0;
        let a: f32 = rug_fuzz_0;
        let b: f32 = rug_fuzz_1;
        let result = <f32 as Euclid>::div_euclid(&a, &b);
        debug_assert_eq!(result, 3.0);
        let a: f32 = -rug_fuzz_2;
        let b: f32 = rug_fuzz_3;
        let result = <f32 as Euclid>::div_euclid(&a, &b);
        debug_assert_eq!(result, - 4.0);
        let a: f32 = rug_fuzz_4;
        let b: f32 = -rug_fuzz_5;
        let result = <f32 as Euclid>::div_euclid(&a, &b);
        debug_assert_eq!(result, - 4.0);
        let a: f32 = -rug_fuzz_6;
        let b: f32 = -rug_fuzz_7;
        let result = <f32 as Euclid>::div_euclid(&a, &b);
        debug_assert_eq!(result, 3.0);
        let _rug_ed_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_div_euclid_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_589_llm_16_589 {
    use crate::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 10.0;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 10.0;
        let rug_fuzz_5 = 3.0;
        let rug_fuzz_6 = 10.0;
        let rug_fuzz_7 = 3.0;
        let rug_fuzz_8 = 10.0;
        let rug_fuzz_9 = 0.0;
        debug_assert_eq!(
            < f64 as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3.0
        );
        debug_assert_eq!(
            < f64 as Euclid > ::div_euclid(& rug_fuzz_2, & - rug_fuzz_3), - 4.0
        );
        debug_assert_eq!(
            < f64 as Euclid > ::div_euclid(& - rug_fuzz_4, & rug_fuzz_5), - 4.0
        );
        debug_assert_eq!(
            < f64 as Euclid > ::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3.0
        );
        debug_assert!(
            < f64 as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9).is_infinite()
        );
        let _rug_ed_tests_llm_16_589_llm_16_589_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_590 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_590_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5.0_f64;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 5.0_f64;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 5.0_f64;
        let rug_fuzz_5 = 3.0;
        let rug_fuzz_6 = 5.0_f64;
        let rug_fuzz_7 = 3.0;
        let rug_fuzz_8 = 3.0_f64;
        let rug_fuzz_9 = 3.0;
        let rug_fuzz_10 = 3.0;
        let rug_fuzz_11 = 3.0;
        let rug_fuzz_12 = 3.0;
        let rug_fuzz_13 = 3.0;
        debug_assert_eq!(rug_fuzz_0.rem_euclid(rug_fuzz_1), 2.0);
        debug_assert_eq!((- rug_fuzz_2).rem_euclid(rug_fuzz_3), 1.0);
        debug_assert_eq!(rug_fuzz_4.rem_euclid(- rug_fuzz_5), - 1.0);
        debug_assert_eq!((- rug_fuzz_6).rem_euclid(- rug_fuzz_7), - 2.0);
        debug_assert_eq!(rug_fuzz_8.rem_euclid(rug_fuzz_9), 0.0);
        debug_assert!(f64::rem_euclid(f64::NAN, rug_fuzz_10).is_nan());
        debug_assert!(f64::rem_euclid(rug_fuzz_11, f64::NAN).is_nan());
        debug_assert!(f64::rem_euclid(f64::INFINITY, rug_fuzz_12).is_nan());
        debug_assert!(f64::rem_euclid(rug_fuzz_13, f64::INFINITY).is_nan());
        debug_assert!(f64::rem_euclid(f64::NAN, f64::NAN).is_nan());
        let _rug_ed_tests_llm_16_590_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_676 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_div_euclid_i128() {
        let _rug_st_tests_llm_16_676_rrrruuuugggg_checked_div_euclid_i128 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 100;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 100;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 100;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, &
            rug_fuzz_1), Some(10)
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, &
            rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_4, &
            rug_fuzz_5), Some(- 10)
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_6, &
            - rug_fuzz_7), Some(10)
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& rug_fuzz_8, & -
            rug_fuzz_9), Some(- 10)
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& i128::MIN, & -
            rug_fuzz_10), None
        );
        debug_assert_eq!(
            < i128 as ops::euclid::CheckedEuclid > ::checked_div_euclid(& i128::MIN, &
            rug_fuzz_11), Some(i128::MIN)
        );
        let _rug_ed_tests_llm_16_676_rrrruuuugggg_checked_div_euclid_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_677_llm_16_677 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid_i128() {
        let _rug_st_tests_llm_16_677_llm_16_677_rrrruuuugggg_test_checked_rem_euclid_i128 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 100;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 100;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 100;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & - rug_fuzz_3),
            Some(1)
        );
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_4, & rug_fuzz_5),
            Some(2)
        );
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(2)
        );
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            None
        );
        debug_assert_eq!(
            < i128 as CheckedEuclid > ::checked_rem_euclid(& i128::MIN, & - rug_fuzz_10),
            None
        );
        let _rug_ed_tests_llm_16_677_llm_16_677_rrrruuuugggg_test_checked_rem_euclid_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_678 {
    use crate::Euclid;
    #[test]
    fn test_div_euclid_i128() {
        let _rug_st_tests_llm_16_678_rrrruuuugggg_test_div_euclid_i128 = 0;
        let rug_fuzz_0 = 10i128;
        let rug_fuzz_1 = 3i128;
        let rug_fuzz_2 = 10i128;
        let rug_fuzz_3 = 3i128;
        let rug_fuzz_4 = 10i128;
        let rug_fuzz_5 = 3i128;
        let rug_fuzz_6 = 10i128;
        let rug_fuzz_7 = 3i128;
        let rug_fuzz_8 = 1i128;
        let rug_fuzz_9 = 1i128;
        let rug_fuzz_10 = 0i128;
        let rug_fuzz_11 = 1i128;
        let rug_fuzz_12 = 1i128;
        let rug_fuzz_13 = 1i128;
        let rug_fuzz_14 = 1i128;
        let rug_fuzz_15 = 1i128;
        let rug_fuzz_16 = 1i128;
        let rug_fuzz_17 = 1i128;
        let rug_fuzz_18 = 0i128;
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& rug_fuzz_2, & - rug_fuzz_3), - 4i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& - rug_fuzz_4, & rug_fuzz_5), - 4i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 1i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& rug_fuzz_10, & rug_fuzz_11), 0i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& - rug_fuzz_12, & - rug_fuzz_13), 1i128
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& i128::MAX, & rug_fuzz_14), i128::MAX
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& i128::MIN, & - rug_fuzz_15), i128::MIN
        );
        debug_assert_eq!(
            < i128 as Euclid > ::div_euclid(& i128::MIN, & rug_fuzz_16), i128::MIN
        );
        let result = std::panic::catch_unwind(|| {
            <i128 as Euclid>::div_euclid(&rug_fuzz_17, &rug_fuzz_18);
        });
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_678_rrrruuuugggg_test_div_euclid_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_679_llm_16_679 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_679_llm_16_679_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 123456789;
        let rug_fuzz_13 = 123456789;
        debug_assert_eq!(< i128 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& - rug_fuzz_2, & rug_fuzz_3), 1
        );
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& rug_fuzz_4, & - rug_fuzz_5), - 1
        );
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& - rug_fuzz_6, & - rug_fuzz_7), - 2
        );
        debug_assert_eq!(< i128 as Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& rug_fuzz_10, & rug_fuzz_11), 0
        );
        let large_pos = i128::MAX;
        let large_neg = i128::MIN;
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& large_pos, & rug_fuzz_12), large_pos
            .rem_euclid(123456789)
        );
        debug_assert_eq!(
            < i128 as Euclid > ::rem_euclid(& large_neg, & rug_fuzz_13), large_neg
            .rem_euclid(123456789)
        );
        let _rug_ed_tests_llm_16_679_llm_16_679_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_786_llm_16_786 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_786_llm_16_786_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & - rug_fuzz_3),
            Some(- 5)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_4, & rug_fuzz_5),
            Some(- 5)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(5)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_8, & rug_fuzz_9),
            None
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_div_euclid(& i16::MIN, & - rug_fuzz_10),
            None
        );
        let _rug_ed_tests_llm_16_786_llm_16_786_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_787_llm_16_787 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid_i16() {
        let _rug_st_tests_llm_16_787_llm_16_787_rrrruuuugggg_test_checked_rem_euclid_i16 = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 5;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 1;
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_2, & rug_fuzz_3),
            Some(1)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_4, & - rug_fuzz_5),
            Some(1)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(1)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            Some(0)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_10, & rug_fuzz_11),
            Some(0)
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_12, & rug_fuzz_13),
            None
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_14, &
            rug_fuzz_15), None
        );
        debug_assert_eq!(
            < i16 as CheckedEuclid > ::checked_rem_euclid(& i16::MIN, & - rug_fuzz_16),
            None
        );
        let _rug_ed_tests_llm_16_787_llm_16_787_rrrruuuugggg_test_checked_rem_euclid_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_788_llm_16_788 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(< i16 as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3);
        debug_assert_eq!(
            < i16 as Euclid > ::div_euclid(& rug_fuzz_2, & - rug_fuzz_3), - 4
        );
        debug_assert_eq!(
            < i16 as Euclid > ::div_euclid(& - rug_fuzz_4, & rug_fuzz_5), - 4
        );
        debug_assert_eq!(
            < i16 as Euclid > ::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3
        );
        debug_assert_eq!(< i16 as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(
            < i16 as Euclid > ::div_euclid(& rug_fuzz_10, & rug_fuzz_11), 1
        );
        let _rug_ed_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_div_euclid = 0;
    }
    #[test]
    #[should_panic]
    fn test_div_euclid_divide_by_zero() {
        let _rug_st_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        <i16 as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_788_llm_16_788_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_789_llm_16_789 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid_positive() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_positive = 0;
        let rug_fuzz_0 = 8_i16;
        let rug_fuzz_1 = 6;
        debug_assert_eq!(rug_fuzz_0.rem_euclid(rug_fuzz_1), 2);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_positive = 0;
    }
    #[test]
    fn test_rem_euclid_negative() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_negative = 0;
        let rug_fuzz_0 = 8_i16;
        let rug_fuzz_1 = 6;
        debug_assert_eq!((- rug_fuzz_0).rem_euclid(rug_fuzz_1), 4);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_negative = 0;
    }
    #[test]
    fn test_rem_euclid_zero() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_zero = 0;
        let rug_fuzz_0 = 0_i16;
        let rug_fuzz_1 = 6;
        debug_assert_eq!(rug_fuzz_0.rem_euclid(rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_zero = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempted to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_by_zero() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_by_zero = 0;
        let rug_fuzz_0 = 8_i16;
        let rug_fuzz_1 = 0;
        rug_fuzz_0.rem_euclid(rug_fuzz_1);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_by_zero = 0;
    }
    #[test]
    fn test_rem_euclid_negative_divisor() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_negative_divisor = 0;
        let rug_fuzz_0 = 8_i16;
        let rug_fuzz_1 = 6;
        debug_assert_eq!(rug_fuzz_0.rem_euclid(- rug_fuzz_1), 2);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_negative_divisor = 0;
    }
    #[test]
    fn test_rem_euclid_both_negative() {
        let _rug_st_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_both_negative = 0;
        let rug_fuzz_0 = 8_i16;
        let rug_fuzz_1 = 6;
        debug_assert_eq!((- rug_fuzz_0).rem_euclid(- rug_fuzz_1), 4);
        let _rug_ed_tests_llm_16_789_llm_16_789_rrrruuuugggg_test_rem_euclid_both_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_896_llm_16_896 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_896_llm_16_896_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 3;
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(3)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & rug_fuzz_5),
            None
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_6, & rug_fuzz_7),
            Some(- 3)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_8, & -
            rug_fuzz_9), Some(3)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_10, & rug_fuzz_11),
            Some(0)
        );
        let _rug_ed_tests_llm_16_896_llm_16_896_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_897_llm_16_897 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_897_llm_16_897_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & - rug_fuzz_3),
            Some(1)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_4, & rug_fuzz_5),
            Some(2)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(2)
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            None
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_10, &
            rug_fuzz_11), None
        );
        debug_assert_eq!(
            < i32 as CheckedEuclid > ::checked_rem_euclid(& i32::MIN, & - rug_fuzz_12),
            None
        );
        let _rug_ed_tests_llm_16_897_llm_16_897_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_898_llm_16_898 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_898_llm_16_898_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0;
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_2, & - rug_fuzz_3), - 4);
        debug_assert_eq!(Euclid::div_euclid(& - rug_fuzz_4, & rug_fuzz_5), - 4);
        debug_assert_eq!(Euclid::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_10, & rug_fuzz_11), 1);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_12, & - rug_fuzz_13), - 1);
        debug_assert_eq!(Euclid::div_euclid(& - rug_fuzz_14, & rug_fuzz_15), - 1);
        debug_assert_eq!(Euclid::div_euclid(& - rug_fuzz_16, & - rug_fuzz_17), 1);
        let result = std::panic::catch_unwind(|| {
            Euclid::div_euclid(&rug_fuzz_18, &rug_fuzz_19);
        });
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_898_llm_16_898_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_899_llm_16_899 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid_positive() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_positive = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(< i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_positive = 0;
    }
    #[test]
    fn test_rem_euclid_negative_dividend() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_negative_dividend = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& - rug_fuzz_0, & rug_fuzz_1), 1
        );
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_negative_dividend = 0;
    }
    #[test]
    fn test_rem_euclid_negative_divisor() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_negative_divisor = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & - rug_fuzz_1), - 1
        );
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_negative_divisor = 0;
    }
    #[test]
    fn test_rem_euclid_both_negative() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_both_negative = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& - rug_fuzz_0, & - rug_fuzz_1), - 2
        );
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_both_negative = 0;
    }
    #[test]
    fn test_rem_euclid_zero_dividend() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_zero_dividend = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(< i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_zero_dividend = 0;
    }
    #[test]
    #[should_panic]
    fn test_rem_euclid_zero_divisor() {
        let _rug_st_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_zero_divisor = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 0;
        <i32 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_899_llm_16_899_rrrruuuugggg_test_rem_euclid_zero_divisor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1006_llm_16_1006 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_1006_llm_16_1006_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 20;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 20;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 20;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 20;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 5;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 5;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 10;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 10;
        let rug_fuzz_19 = 0;
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_2, & rug_fuzz_3),
            Some(- 2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & - rug_fuzz_5),
            Some(- 2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_8, & rug_fuzz_9),
            Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_10, &
            rug_fuzz_11), Some(- 3)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_12, & -
            rug_fuzz_13), Some(- 3)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_14, & -
            rug_fuzz_15), Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_16, & rug_fuzz_17),
            None
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_18, &
            rug_fuzz_19), None
        );
        let _rug_ed_tests_llm_16_1006_llm_16_1006_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1007_llm_16_1007 {
    use super::*;
    use crate::*;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1007_llm_16_1007_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 0;
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & - rug_fuzz_3),
            Some(1)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_4, & rug_fuzz_5),
            Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_6, & -
            rug_fuzz_7), Some(2)
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            None
        );
        debug_assert_eq!(
            < i64 as CheckedEuclid > ::checked_rem_euclid(& - rug_fuzz_10, &
            rug_fuzz_11), None
        );
        let _rug_ed_tests_llm_16_1007_llm_16_1007_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1008_llm_16_1008 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1008_llm_16_1008_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10i64;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10i64;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10i64;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10i64;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0i64;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1i64;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1i64;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1i64;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1i64;
        let rug_fuzz_17 = 1;
        debug_assert_eq!(rug_fuzz_0.div_euclid(rug_fuzz_1), 3);
        debug_assert_eq!(rug_fuzz_2.div_euclid(- rug_fuzz_3), - 4);
        debug_assert_eq!((- rug_fuzz_4).div_euclid(rug_fuzz_5), - 4);
        debug_assert_eq!((- rug_fuzz_6).div_euclid(- rug_fuzz_7), 3);
        debug_assert_eq!(rug_fuzz_8.div_euclid(rug_fuzz_9), 0);
        debug_assert_eq!(rug_fuzz_10.div_euclid(rug_fuzz_11), 1);
        debug_assert_eq!((- rug_fuzz_12).div_euclid(rug_fuzz_13), - 1);
        debug_assert_eq!(rug_fuzz_14.div_euclid(- rug_fuzz_15), - 1);
        debug_assert_eq!((- rug_fuzz_16).div_euclid(- rug_fuzz_17), 1);
        let _rug_ed_tests_llm_16_1008_llm_16_1008_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1009_llm_16_1009 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1009_llm_16_1009_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 1;
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& - rug_fuzz_2, & rug_fuzz_3), 1
        );
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& rug_fuzz_4, & - rug_fuzz_5), - 1
        );
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& - rug_fuzz_6, & - rug_fuzz_7), - 2
        );
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& rug_fuzz_10, & rug_fuzz_11), 0
        );
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& - rug_fuzz_12, & rug_fuzz_13), 0
        );
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& rug_fuzz_14, & - rug_fuzz_15), 0
        );
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& i64::MIN, & rug_fuzz_16), 0);
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& i64::MIN, & - rug_fuzz_17), 0);
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& i64::MAX, & rug_fuzz_18), 0);
        debug_assert_eq!(< i64 as Euclid > ::rem_euclid(& i64::MAX, & - rug_fuzz_19), 0);
        let max = i64::MAX;
        debug_assert_eq!(
            < i64 as Euclid > ::rem_euclid(& max, & (max - rug_fuzz_20)), 1
        );
        let _rug_ed_tests_llm_16_1009_llm_16_1009_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1116_llm_16_1116 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_1116_llm_16_1116_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 2;
        debug_assert_eq!(
            < i8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < i8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            None
        );
        debug_assert_eq!(
            < i8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & - rug_fuzz_5),
            Some(- 5)
        );
        debug_assert_eq!(
            < i8 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_6, & rug_fuzz_7),
            Some(- 5)
        );
        debug_assert_eq!(
            < i8 as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_8, & - rug_fuzz_9),
            Some(5)
        );
        let _rug_ed_tests_llm_16_1116_llm_16_1116_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1118_llm_16_1118 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid_i8() {
        let _rug_st_tests_llm_16_1118_llm_16_1118_rrrruuuugggg_test_div_euclid_i8 = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 7;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        debug_assert_eq!(< i8 as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3);
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& - rug_fuzz_2, & rug_fuzz_3), - 4
        );
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& rug_fuzz_4, & - rug_fuzz_5), - 3
        );
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3
        );
        debug_assert_eq!(< i8 as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< i8 as Euclid > ::div_euclid(& rug_fuzz_10, & rug_fuzz_11), 0);
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& - rug_fuzz_12, & rug_fuzz_13), - 1
        );
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& rug_fuzz_14, & - rug_fuzz_15), - 1
        );
        debug_assert_eq!(
            < i8 as Euclid > ::div_euclid(& i8::MIN, & - rug_fuzz_16), i8::MIN
        );
        let _rug_ed_tests_llm_16_1118_llm_16_1118_rrrruuuugggg_test_div_euclid_i8 = 0;
    }
    #[test]
    #[should_panic]
    fn test_div_euclid_i8_divide_by_zero() {
        let _rug_st_tests_llm_16_1118_llm_16_1118_rrrruuuugggg_test_div_euclid_i8_divide_by_zero = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        <i8 as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_1118_llm_16_1118_rrrruuuugggg_test_div_euclid_i8_divide_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1119_llm_16_1119 {
    use super::*;
    use crate::*;
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1119_llm_16_1119_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 3;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 3;
        debug_assert_eq!(< i8 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        debug_assert_eq!(< i8 as Euclid > ::rem_euclid(& - rug_fuzz_2, & rug_fuzz_3), 1);
        debug_assert_eq!(
            < i8 as Euclid > ::rem_euclid(& rug_fuzz_4, & - rug_fuzz_5), - 1
        );
        debug_assert_eq!(
            < i8 as Euclid > ::rem_euclid(& - rug_fuzz_6, & - rug_fuzz_7), - 2
        );
        debug_assert_eq!(< i8 as Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(< i8 as Euclid > ::rem_euclid(& rug_fuzz_10, & rug_fuzz_11), 0);
        debug_assert_eq!(
            < i8 as Euclid > ::rem_euclid(& - rug_fuzz_12, & rug_fuzz_13), 0
        );
        let _rug_ed_tests_llm_16_1119_llm_16_1119_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1226_llm_16_1226 {
    use super::*;
    use crate::*;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_1226_llm_16_1226_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 1;
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(3)
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & rug_fuzz_5),
            None
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_6, & -
            rug_fuzz_7), Some(- 5)
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_8, &
            rug_fuzz_9), Some(- 5)
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& - rug_fuzz_10, & -
            rug_fuzz_11), Some(5)
        );
        debug_assert_eq!(
            < isize as CheckedEuclid > ::checked_div_euclid(& isize::MIN, & -
            rug_fuzz_12), None
        );
        let _rug_ed_tests_llm_16_1226_llm_16_1226_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1227_llm_16_1227 {
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1227_llm_16_1227_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 5isize;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 5isize;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 5isize;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 5isize;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 5isize;
        let rug_fuzz_9 = 2;
        debug_assert_eq!(rug_fuzz_0.checked_rem_euclid(rug_fuzz_1), Some(1));
        debug_assert_eq!(rug_fuzz_2.checked_rem_euclid(rug_fuzz_3), None);
        debug_assert_eq!((- rug_fuzz_4).checked_rem_euclid(rug_fuzz_5), Some(1));
        debug_assert_eq!(rug_fuzz_6.checked_rem_euclid(- rug_fuzz_7), Some(1));
        debug_assert_eq!((- rug_fuzz_8).checked_rem_euclid(- rug_fuzz_9), Some(1));
        let _rug_ed_tests_llm_16_1227_llm_16_1227_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1228 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1228_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3
        );
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& rug_fuzz_2, & - rug_fuzz_3), - 3
        );
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& - rug_fuzz_4, & rug_fuzz_5), - 4
        );
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 3
        );
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < isize as Euclid > ::div_euclid(& rug_fuzz_10, & - rug_fuzz_11), 0
        );
        let _rug_ed_tests_llm_16_1228_rrrruuuugggg_test_div_euclid = 0;
    }
    #[test]
    #[should_panic]
    fn test_div_euclid_panic() {
        let _rug_st_tests_llm_16_1228_rrrruuuugggg_test_div_euclid_panic = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let _ = <isize as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_1228_rrrruuuugggg_test_div_euclid_panic = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1229_llm_16_1229 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1229_llm_16_1229_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(
            < isize as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < isize as Euclid > ::rem_euclid(& rug_fuzz_2, & - rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < isize as Euclid > ::rem_euclid(& - rug_fuzz_4, & rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < isize as Euclid > ::rem_euclid(& - rug_fuzz_6, & - rug_fuzz_7), 1
        );
        debug_assert_eq!(
            < isize as Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0
        );
        let _rug_ed_tests_llm_16_1229_llm_16_1229_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1432_llm_16_1432 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn checked_div_euclid_for_u128() {
        let _rug_st_tests_llm_16_1432_llm_16_1432_rrrruuuugggg_checked_div_euclid_for_u128 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        debug_assert_eq!(
            < u128 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(10)
        );
        debug_assert_eq!(
            < u128 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            None
        );
        debug_assert_eq!(
            < u128 as CheckedEuclid > ::checked_div_euclid(& u128::MAX, & rug_fuzz_4),
            Some(u128::MAX)
        );
        debug_assert_eq!(
            < u128 as CheckedEuclid > ::checked_div_euclid(& u128::MAX, & rug_fuzz_5),
            Some(u128::MAX / 2)
        );
        let _rug_ed_tests_llm_16_1432_llm_16_1432_rrrruuuugggg_checked_div_euclid_for_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1433_llm_16_1433 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1433_llm_16_1433_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10u128;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 3u128;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 5u128;
        let rug_fuzz_10 = 5;
        debug_assert_eq!((rug_fuzz_0).checked_rem_euclid(rug_fuzz_1), None);
        debug_assert_eq!((rug_fuzz_2).checked_rem_euclid(rug_fuzz_3), Some(2));
        debug_assert_eq!((rug_fuzz_4).checked_rem_euclid(rug_fuzz_5), Some(1));
        debug_assert_eq!((u128::MAX).checked_rem_euclid(rug_fuzz_6), Some(0));
        debug_assert_eq!((rug_fuzz_7).checked_rem_euclid(rug_fuzz_8), Some(3));
        debug_assert_eq!((rug_fuzz_9).checked_rem_euclid(rug_fuzz_10), Some(0));
        let _rug_ed_tests_llm_16_1433_llm_16_1433_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1434_llm_16_1434 {
    use super::*;
    use crate::*;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1434_llm_16_1434_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 3u128;
        let rug_fuzz_2 = 10u128;
        let rug_fuzz_3 = 4u128;
        let rug_fuzz_4 = 10u128;
        let rug_fuzz_5 = 5u128;
        let rug_fuzz_6 = 10u128;
        let rug_fuzz_7 = 10u128;
        let rug_fuzz_8 = 10u128;
        let rug_fuzz_9 = 1u128;
        let rug_fuzz_10 = 1u128;
        let rug_fuzz_11 = 10u128;
        let rug_fuzz_12 = 0u128;
        let rug_fuzz_13 = 10u128;
        debug_assert_eq!(rug_fuzz_0.div_euclid(rug_fuzz_1), 3u128);
        debug_assert_eq!(rug_fuzz_2.div_euclid(rug_fuzz_3), 2u128);
        debug_assert_eq!(rug_fuzz_4.div_euclid(rug_fuzz_5), 2u128);
        debug_assert_eq!(rug_fuzz_6.div_euclid(rug_fuzz_7), 1u128);
        debug_assert_eq!(rug_fuzz_8.div_euclid(rug_fuzz_9), 10u128);
        debug_assert_eq!(rug_fuzz_10.div_euclid(rug_fuzz_11), 0u128);
        debug_assert_eq!(rug_fuzz_12.div_euclid(rug_fuzz_13), 0u128);
        let _rug_ed_tests_llm_16_1434_llm_16_1434_rrrruuuugggg_test_div_euclid = 0;
    }
    #[test]
    #[should_panic]
    fn test_div_euclid_divide_by_zero() {
        let _rug_st_tests_llm_16_1434_llm_16_1434_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 0u128;
        rug_fuzz_0.div_euclid(rug_fuzz_1);
        let _rug_ed_tests_llm_16_1434_llm_16_1434_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1435 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1435_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 7;
        let rug_fuzz_12 = 7;
        let rug_fuzz_13 = 7;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 100;
        let rug_fuzz_17 = 30;
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 1
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_2, & rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_6, & rug_fuzz_7), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_10, & rug_fuzz_11), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_12, & rug_fuzz_13), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& u128::MAX, & rug_fuzz_14), 0
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& u128::MAX, & rug_fuzz_15), 1
        );
        debug_assert_eq!(
            < u128 as ops::euclid::Euclid > ::rem_euclid(& rug_fuzz_16, & rug_fuzz_17),
            10
        );
        let _rug_ed_tests_llm_16_1435_rrrruuuugggg_test_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1537_llm_16_1537 {
    use super::*;
    use crate::*;
    use crate::CheckedDiv;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid_positive() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_positive = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(10)
        );
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_positive = 0;
    }
    #[test]
    fn test_checked_div_euclid_divide_by_zero() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_divide_by_zero = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 0;
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            None
        );
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_divide_by_zero = 0;
    }
    #[test]
    fn test_checked_div_euclid_negative_divisor() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_negative_divisor = 0;
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_negative_divisor = 0;
    }
    #[test]
    fn test_checked_div_euclid_self_is_zero() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_self_is_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(0)
        );
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_self_is_zero = 0;
    }
    #[test]
    fn test_checked_div_euclid_both_zero() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_both_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            None
        );
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_both_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1538_llm_16_1538 {
    use super::*;
    use crate::*;
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1538_llm_16_1538_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 123;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(2)
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_4, & rug_fuzz_5),
            Some(0)
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_6, & rug_fuzz_7),
            Some(1)
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            None
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& u16::MAX, & rug_fuzz_10),
            Some(0)
        );
        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_rem_euclid(& u16::MAX, & u16::MAX),
            Some(0)
        );
        let _rug_ed_tests_llm_16_1538_llm_16_1538_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1540_llm_16_1540 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1540_llm_16_1540_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 13u16;
        let rug_fuzz_1 = 4u16;
        let rug_fuzz_2 = 0u16;
        let rug_fuzz_3 = 1u16;
        let rug_fuzz_4 = 10u16;
        let rug_fuzz_5 = 10u16;
        let rug_fuzz_6 = 10u16;
        let rug_fuzz_7 = 3u16;
        debug_assert_eq!(
            < u16 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 1u16
        );
        debug_assert_eq!(
            < u16 as Euclid > ::rem_euclid(& rug_fuzz_2, & rug_fuzz_3), 0u16
        );
        debug_assert_eq!(
            < u16 as Euclid > ::rem_euclid(& rug_fuzz_4, & rug_fuzz_5), 0u16
        );
        debug_assert_eq!(
            < u16 as Euclid > ::rem_euclid(& rug_fuzz_6, & rug_fuzz_7), 1u16
        );
        let _rug_ed_tests_llm_16_1540_llm_16_1540_rrrruuuugggg_test_rem_euclid = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_divide_by_zero() {
        let _rug_st_tests_llm_16_1540_llm_16_1540_rrrruuuugggg_test_rem_euclid_divide_by_zero = 0;
        let rug_fuzz_0 = 13u16;
        let rug_fuzz_1 = 0u16;
        <u16 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_1540_llm_16_1540_rrrruuuugggg_test_rem_euclid_divide_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1642_llm_16_1642 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_1642_llm_16_1642_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 7;
        let rug_fuzz_8 = 1;
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(3)
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & rug_fuzz_5),
            None
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_6, & rug_fuzz_7),
            Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_div_euclid(& u32::MAX, & rug_fuzz_8),
            Some(u32::MAX)
        );
        let _rug_ed_tests_llm_16_1642_llm_16_1642_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1643_llm_16_1643 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1643_llm_16_1643_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 10u32;
        let rug_fuzz_1 = 3u32;
        let rug_fuzz_2 = 10u32;
        let rug_fuzz_3 = 10u32;
        let rug_fuzz_4 = 10u32;
        let rug_fuzz_5 = 0u32;
        let rug_fuzz_6 = 1u32;
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1u32)
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(0u32)
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_4, & rug_fuzz_5),
            None
        );
        debug_assert_eq!(
            < u32 as CheckedEuclid > ::checked_rem_euclid(& u32::MAX, & rug_fuzz_6),
            Some(0u32)
        );
        let _rug_ed_tests_llm_16_1643_llm_16_1643_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1644_llm_16_1644 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1644_llm_16_1644_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 5u32;
        let rug_fuzz_1 = 2u32;
        let rug_fuzz_2 = 5u32;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 5u32;
        let rug_fuzz_5 = 5u32;
        let rug_fuzz_6 = 5u32;
        let rug_fuzz_7 = 1u32;
        let rug_fuzz_8 = 0u32;
        let rug_fuzz_9 = 1u32;
        let rug_fuzz_10 = 2u32;
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 2u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 1u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 1u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 5u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0u32);
        debug_assert_eq!(Euclid::div_euclid(& u32::MAX, & rug_fuzz_10), u32::MAX / 2u32);
        let _rug_ed_tests_llm_16_1644_llm_16_1644_rrrruuuugggg_test_div_euclid = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_euclid_divide_by_zero() {
        let _rug_st_tests_llm_16_1644_llm_16_1644_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
        let rug_fuzz_0 = 5u32;
        let rug_fuzz_1 = 0u32;
        let _ = Euclid::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_1644_llm_16_1644_rrrruuuugggg_test_div_euclid_divide_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1645_llm_16_1645 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_u32_rem_euclid_positive() {
        let _rug_st_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_positive = 0;
        let rug_fuzz_0 = 7;
        let rug_fuzz_1 = 3;
        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 1);
        let _rug_ed_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_positive = 0;
    }
    #[test]
    fn test_u32_rem_euclid_self_divisor() {
        let _rug_st_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_self_divisor = 0;
        let rug_fuzz_0 = 7;
        let rug_fuzz_1 = 7;
        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 0);
        let _rug_ed_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_self_divisor = 0;
    }
    #[test]
    fn test_u32_rem_euclid_larger_divisor() {
        let _rug_st_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_larger_divisor = 0;
        let rug_fuzz_0 = 7;
        let rug_fuzz_1 = 10;
        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 7);
        let _rug_ed_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_larger_divisor = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_u32_rem_euclid_zero_divisor() {
        let _rug_st_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_zero_divisor = 0;
        let rug_fuzz_0 = 7;
        let rug_fuzz_1 = 0;
        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        let _ = Euclid::rem_euclid(&a, &b);
        let _rug_ed_tests_llm_16_1645_llm_16_1645_rrrruuuugggg_test_u32_rem_euclid_zero_divisor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1748_llm_16_1748 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {
        let _rug_st_tests_llm_16_1748_llm_16_1748_rrrruuuugggg_test_checked_rem_euclid = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 7;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(1)
        );
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_2, & rug_fuzz_3),
            None
        );
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_4, & rug_fuzz_5),
            Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_6, & rug_fuzz_7),
            Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& rug_fuzz_8, & rug_fuzz_9),
            Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedEuclid > ::checked_rem_euclid(& u64::MAX, & rug_fuzz_10),
            Some(1)
        );
        let _rug_ed_tests_llm_16_1748_llm_16_1748_rrrruuuugggg_test_checked_rem_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1749 {
    use super::*;
    use crate::*;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1749_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 100u64;
        let rug_fuzz_1 = 3u64;
        let rug_fuzz_2 = 100u64;
        let rug_fuzz_3 = 10u64;
        let rug_fuzz_4 = 100u64;
        let rug_fuzz_5 = 100u64;
        let rug_fuzz_6 = 100u64;
        let rug_fuzz_7 = 200u64;
        let rug_fuzz_8 = 5u64;
        let rug_fuzz_9 = 2u64;
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 33u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 10u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 1u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 0u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 2u64);
        let _rug_ed_tests_llm_16_1749_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1750_llm_16_1750 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1750_llm_16_1750_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 18;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 18;
        let rug_fuzz_11 = 7;
        let rug_fuzz_12 = 100;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 100;
        let rug_fuzz_15 = 100;
        let rug_fuzz_16 = 1234;
        let rug_fuzz_17 = 123;
        debug_assert_eq!(< u64 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        debug_assert_eq!(< u64 as Euclid > ::rem_euclid(& rug_fuzz_2, & rug_fuzz_3), 0);
        debug_assert_eq!(< u64 as Euclid > ::rem_euclid(& rug_fuzz_4, & rug_fuzz_5), 5);
        debug_assert_eq!(< u64 as Euclid > ::rem_euclid(& rug_fuzz_6, & rug_fuzz_7), 0);
        debug_assert_eq!(< u64 as Euclid > ::rem_euclid(& rug_fuzz_8, & rug_fuzz_9), 0);
        debug_assert_eq!(
            < u64 as Euclid > ::rem_euclid(& rug_fuzz_10, & rug_fuzz_11), 4
        );
        debug_assert_eq!(
            < u64 as Euclid > ::rem_euclid(& rug_fuzz_12, & rug_fuzz_13), 1
        );
        debug_assert_eq!(
            < u64 as Euclid > ::rem_euclid(& rug_fuzz_14, & rug_fuzz_15), 0
        );
        debug_assert_eq!(
            < u64 as Euclid > ::rem_euclid(& rug_fuzz_16, & rug_fuzz_17), 91
        );
        let _rug_ed_tests_llm_16_1750_llm_16_1750_rrrruuuugggg_test_rem_euclid = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_with_zero() {
        let _rug_st_tests_llm_16_1750_llm_16_1750_rrrruuuugggg_test_rem_euclid_with_zero = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 0;
        <u64 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
        let _rug_ed_tests_llm_16_1750_llm_16_1750_rrrruuuugggg_test_rem_euclid_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1853 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {
        let _rug_st_tests_llm_16_1853_rrrruuuugggg_test_checked_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 1;
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(5)
        );
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_2, & rug_fuzz_3),
            Some(3)
        );
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_4, & rug_fuzz_5),
            None
        );
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_6, & rug_fuzz_7),
            Some(0)
        );
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& u8::MAX, & rug_fuzz_8),
            Some(u8::MAX)
        );
        debug_assert_eq!(
            < u8 as CheckedEuclid > ::checked_div_euclid(& u8::MAX, & u8::MAX), Some(1)
        );
        let _rug_ed_tests_llm_16_1853_rrrruuuugggg_test_checked_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1855_llm_16_1855 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1855_llm_16_1855_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 8;
        let rug_fuzz_11 = 0;
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 5);
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 0);
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 3);
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 1);
        debug_assert_eq!(< u8 as Euclid > ::div_euclid(& u8::MAX, & u8::MAX), 1);
        let result = std::panic::catch_unwind(|| {
            <u8 as Euclid>::div_euclid(&rug_fuzz_10, &rug_fuzz_11);
        });
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_1855_llm_16_1855_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1960_llm_16_1960 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {
        let _rug_st_tests_llm_16_1960_llm_16_1960_rrrruuuugggg_test_div_euclid = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 11;
        debug_assert_eq!(
            < usize as Euclid > ::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 3
        );
        debug_assert_eq!(
            < usize as Euclid > ::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 2
        );
        debug_assert_eq!(
            < usize as Euclid > ::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 2
        );
        debug_assert_eq!(
            < usize as Euclid > ::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 1
        );
        debug_assert_eq!(
            < usize as Euclid > ::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0
        );
        let _rug_ed_tests_llm_16_1960_llm_16_1960_rrrruuuugggg_test_div_euclid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1961_llm_16_1961 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {
        let _rug_st_tests_llm_16_1961_llm_16_1961_rrrruuuugggg_test_rem_euclid = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 3;
        debug_assert_eq!(
            < usize as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < usize as Euclid > ::rem_euclid(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < usize as Euclid > ::rem_euclid(& rug_fuzz_4, & rug_fuzz_5), 5
        );
        debug_assert_eq!(
            < usize as Euclid > ::rem_euclid(& rug_fuzz_6, & rug_fuzz_7), 0
        );
        let x: usize = rug_fuzz_8;
        let y: usize = rug_fuzz_9;
        debug_assert_eq!(< usize as Euclid > ::rem_euclid(& x, & y), 2);
        let _rug_ed_tests_llm_16_1961_llm_16_1961_rrrruuuugggg_test_rem_euclid = 0;
    }
}
