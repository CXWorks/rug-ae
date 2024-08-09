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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(f32, f32, f32, f32, f32, f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_589_llm_16_589 {
    use crate::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_590 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_676 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_div_euclid_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_677_llm_16_677 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_678 {
    use crate::Euclid;
    #[test]
    fn test_div_euclid_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_679_llm_16_679 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_786_llm_16_786 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_787_llm_16_787 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_788_llm_16_788 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
    #[test]
    #[should_panic]
    fn test_div_euclid_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        <i16 as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_789_llm_16_789 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.rem_euclid(rug_fuzz_1), 2);
             }
});    }
    #[test]
    fn test_rem_euclid_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((- rug_fuzz_0).rem_euclid(rug_fuzz_1), 4);
             }
});    }
    #[test]
    fn test_rem_euclid_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.rem_euclid(rug_fuzz_1), 0);
             }
});    }
    #[test]
    #[should_panic(
        expected = "attempted to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        rug_fuzz_0.rem_euclid(rug_fuzz_1);
             }
});    }
    #[test]
    fn test_rem_euclid_negative_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.rem_euclid(- rug_fuzz_1), 2);
             }
});    }
    #[test]
    fn test_rem_euclid_both_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((- rug_fuzz_0).rem_euclid(- rug_fuzz_1), 4);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_896_llm_16_896 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_897_llm_16_897 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_898_llm_16_898 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_899_llm_16_899 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 2);
             }
});    }
    #[test]
    fn test_rem_euclid_negative_dividend() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& - rug_fuzz_0, & rug_fuzz_1), 1
        );
             }
});    }
    #[test]
    fn test_rem_euclid_negative_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & - rug_fuzz_1), - 1
        );
             }
});    }
    #[test]
    fn test_rem_euclid_both_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as Euclid > ::rem_euclid(& - rug_fuzz_0, & - rug_fuzz_1), - 2
        );
             }
});    }
    #[test]
    fn test_rem_euclid_zero_dividend() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as Euclid > ::rem_euclid(& rug_fuzz_0, & rug_fuzz_1), 0);
             }
});    }
    #[test]
    #[should_panic]
    fn test_rem_euclid_zero_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        <i32 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1006_llm_16_1006 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1007_llm_16_1007 {
    use super::*;
    use crate::*;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1008_llm_16_1008 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.div_euclid(rug_fuzz_1), 3);
        debug_assert_eq!(rug_fuzz_2.div_euclid(- rug_fuzz_3), - 4);
        debug_assert_eq!((- rug_fuzz_4).div_euclid(rug_fuzz_5), - 4);
        debug_assert_eq!((- rug_fuzz_6).div_euclid(- rug_fuzz_7), 3);
        debug_assert_eq!(rug_fuzz_8.div_euclid(rug_fuzz_9), 0);
        debug_assert_eq!(rug_fuzz_10.div_euclid(rug_fuzz_11), 1);
        debug_assert_eq!((- rug_fuzz_12).div_euclid(rug_fuzz_13), - 1);
        debug_assert_eq!(rug_fuzz_14.div_euclid(- rug_fuzz_15), - 1);
        debug_assert_eq!((- rug_fuzz_16).div_euclid(- rug_fuzz_17), 1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1009_llm_16_1009 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1116_llm_16_1116 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1118_llm_16_1118 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
    #[test]
    #[should_panic]
    fn test_div_euclid_i8_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        <i8 as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1119_llm_16_1119 {
    use super::*;
    use crate::*;
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1226_llm_16_1226 {
    use super::*;
    use crate::*;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1227_llm_16_1227 {
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_rem_euclid(rug_fuzz_1), Some(1));
        debug_assert_eq!(rug_fuzz_2.checked_rem_euclid(rug_fuzz_3), None);
        debug_assert_eq!((- rug_fuzz_4).checked_rem_euclid(rug_fuzz_5), Some(1));
        debug_assert_eq!(rug_fuzz_6.checked_rem_euclid(- rug_fuzz_7), Some(1));
        debug_assert_eq!((- rug_fuzz_8).checked_rem_euclid(- rug_fuzz_9), Some(1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1228 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
    #[test]
    #[should_panic]
    fn test_div_euclid_panic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = <isize as Euclid>::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1229_llm_16_1229 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1432_llm_16_1432 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn checked_div_euclid_for_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1433_llm_16_1433 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_rem_euclid(rug_fuzz_1), None);
        debug_assert_eq!((rug_fuzz_2).checked_rem_euclid(rug_fuzz_3), Some(2));
        debug_assert_eq!((rug_fuzz_4).checked_rem_euclid(rug_fuzz_5), Some(1));
        debug_assert_eq!((u128::MAX).checked_rem_euclid(rug_fuzz_6), Some(0));
        debug_assert_eq!((rug_fuzz_7).checked_rem_euclid(rug_fuzz_8), Some(3));
        debug_assert_eq!((rug_fuzz_9).checked_rem_euclid(rug_fuzz_10), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1434_llm_16_1434 {
    use super::*;
    use crate::*;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.div_euclid(rug_fuzz_1), 3u128);
        debug_assert_eq!(rug_fuzz_2.div_euclid(rug_fuzz_3), 2u128);
        debug_assert_eq!(rug_fuzz_4.div_euclid(rug_fuzz_5), 2u128);
        debug_assert_eq!(rug_fuzz_6.div_euclid(rug_fuzz_7), 1u128);
        debug_assert_eq!(rug_fuzz_8.div_euclid(rug_fuzz_9), 10u128);
        debug_assert_eq!(rug_fuzz_10.div_euclid(rug_fuzz_11), 0u128);
        debug_assert_eq!(rug_fuzz_12.div_euclid(rug_fuzz_13), 0u128);
             }
});    }
    #[test]
    #[should_panic]
    fn test_div_euclid_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        rug_fuzz_0.div_euclid(rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1435 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1537_llm_16_1537 {
    use super::*;
    use crate::*;
    use crate::CheckedDiv;
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(10)
        );
             }
});    }
    #[test]
    fn test_checked_div_euclid_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            None
        );
             }
});    }
    #[test]
    fn test_checked_div_euclid_negative_divisor() {
        let _rug_st_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_negative_divisor = 0;
        let _rug_ed_tests_llm_16_1537_llm_16_1537_rrrruuuugggg_test_checked_div_euclid_negative_divisor = 0;
    }
    #[test]
    fn test_checked_div_euclid_self_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            Some(0)
        );
             }
});    }
    #[test]
    fn test_checked_div_euclid_both_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedEuclid > ::checked_div_euclid(& rug_fuzz_0, & rug_fuzz_1),
            None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1538_llm_16_1538 {
    use super::*;
    use crate::*;
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1540_llm_16_1540 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        <u16 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1642_llm_16_1642 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1643_llm_16_1643 {
    use crate::ops::euclid::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1644_llm_16_1644 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 2u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 1u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 1u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 5u32);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 0u32);
        debug_assert_eq!(Euclid::div_euclid(& u32::MAX, & rug_fuzz_10), u32::MAX / 2u32);
             }
});    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_euclid_divide_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Euclid::div_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1645_llm_16_1645 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_u32_rem_euclid_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 1);
             }
});    }
    #[test]
    fn test_u32_rem_euclid_self_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 0);
             }
});    }
    #[test]
    fn test_u32_rem_euclid_larger_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        debug_assert_eq!(Euclid::rem_euclid(& a, & b), 7);
             }
});    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_u32_rem_euclid_zero_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u32 = rug_fuzz_0;
        let b: u32 = rug_fuzz_1;
        let _ = Euclid::rem_euclid(&a, &b);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1748_llm_16_1748 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1749 {
    use super::*;
    use crate::*;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_0, & rug_fuzz_1), 33u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_2, & rug_fuzz_3), 10u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_4, & rug_fuzz_5), 1u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_6, & rug_fuzz_7), 0u64);
        debug_assert_eq!(Euclid::div_euclid(& rug_fuzz_8, & rug_fuzz_9), 2u64);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1750_llm_16_1750 {
    use crate::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the remainder with a divisor of zero"
    )]
    fn test_rem_euclid_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        <u64 as Euclid>::rem_euclid(&rug_fuzz_0, &rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1853 {
    use crate::CheckedEuclid;
    #[test]
    fn test_checked_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1855_llm_16_1855 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1960_llm_16_1960 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_div_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1961_llm_16_1961 {
    use crate::ops::euclid::Euclid;
    #[test]
    fn test_rem_euclid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
