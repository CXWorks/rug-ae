use core::num::Wrapping;
use core::ops::{Add, Mul};
/// Defines an additive identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a + 0 = a       ∀ a ∈ Self
/// 0 + a = a       ∀ a ∈ Self
/// ```
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`, `0`.
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    fn zero() -> Self;
    /// Sets `self` to the additive identity element of `Self`, `0`.
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool;
}
macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t { #[inline] fn zero() -> $t { $v } #[inline] fn is_zero(& self)
        -> bool { * self == $v } }
    };
}
zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);
zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);
zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);
impl<T: Zero> Zero for Wrapping<T>
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn set_zero(&mut self) {
        self.0.set_zero();
    }
    fn zero() -> Self {
        Wrapping(T::zero())
    }
}
/// Defines a multiplicative identity element for `Self`.
///
/// # Laws
///
/// ```{.text}
/// a * 1 = a       ∀ a ∈ Self
/// 1 * a = a       ∀ a ∈ Self
/// ```
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    fn one() -> Self;
    /// Sets `self` to the multiplicative identity element of `Self`, `1`.
    fn set_one(&mut self) {
        *self = One::one();
    }
    /// Returns `true` if `self` is equal to the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    /// After a semver bump, this method will be required, and the
    /// `where Self: PartialEq` bound will be removed.
    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}
macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t { #[inline] fn one() -> $t { $v } #[inline] fn is_one(& self) ->
        bool { * self == $v } }
    };
}
one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(u128, 1);
one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(i128, 1);
one_impl!(f32, 1.0);
one_impl!(f64, 1.0);
impl<T: One> One for Wrapping<T>
where
    Wrapping<T>: Mul<Output = Wrapping<T>>,
{
    fn set_one(&mut self) {
        self.0.set_one();
    }
    fn one() -> Self {
        Wrapping(T::one())
    }
}
/// Returns the additive identity, `0`.
#[inline(always)]
pub fn zero<T: Zero>() -> T {
    Zero::zero()
}
/// Returns the multiplicative identity, `1`.
#[inline(always)]
pub fn one<T: One>() -> T {
    One::one()
}
#[test]
fn wrapping_identities() {
    macro_rules! test_wrapping_identities {
        ($($t:ty)+) => {
            $(assert_eq!(zero::<$t > (), zero::< Wrapping <$t >> ().0);
            assert_eq!(one::<$t > (), one::< Wrapping <$t >> ().0); assert_eq!((0 as $t)
            .is_zero(), Wrapping(0 as $t).is_zero()); assert_eq!((1 as $t).is_zero(),
            Wrapping(1 as $t).is_zero());)+
        };
    }
    test_wrapping_identities!(isize i8 i16 i32 i64 usize u8 u16 u32 u64);
}
#[test]
fn wrapping_is_zero() {
    fn require_zero<T: Zero>(_: &T) {}
    require_zero(&Wrapping(42));
}
#[test]
fn wrapping_is_one() {
    fn require_one<T: One>(_: &T) {}
    require_one(&Wrapping(42));
}
#[cfg(test)]
mod tests_llm_16_417_llm_16_417 {
    use crate::identities::One;
    #[test]
    fn test_f32_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< f32 as One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < f32 as One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < f32 as One > ::is_one(& rug_fuzz_2));
        debug_assert!(! < f32 as One > ::is_one(& f32::INFINITY));
        debug_assert!(! < f32 as One > ::is_one(& f32::NEG_INFINITY));
        debug_assert!(! < f32 as One > ::is_one(& f32::NAN));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_418 {
    use super::*;
    use crate::*;
    #[test]
    fn test_f32_one() {
        let _rug_st_tests_llm_16_418_rrrruuuugggg_test_f32_one = 0;
        let one_value: f32 = <f32 as identities::One>::one();
        debug_assert_eq!(one_value, 1.0f32);
        let _rug_ed_tests_llm_16_418_rrrruuuugggg_test_f32_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_419_llm_16_419 {
    use crate::identities::Zero;
    #[test]
    fn test_is_zero_with_zero_value() {
        let _rug_st_tests_llm_16_419_llm_16_419_rrrruuuugggg_test_is_zero_with_zero_value = 0;
        let value: f32 = Zero::zero();
        debug_assert!(< f32 as Zero > ::is_zero(& value));
        let _rug_ed_tests_llm_16_419_llm_16_419_rrrruuuugggg_test_is_zero_with_zero_value = 0;
    }
    #[test]
    fn test_is_zero_with_non_zero_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f32 = rug_fuzz_0;
        debug_assert!(! < f32 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_is_zero_with_negative_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f32 = -rug_fuzz_0;
        debug_assert!(! < f32 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_is_zero_with_positive_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f32 = rug_fuzz_0;
        debug_assert!(! < f32 as Zero > ::is_zero(& value));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_420_llm_16_420 {
    use crate::identities::Zero;
    #[test]
    fn zero_f32() {
        let _rug_st_tests_llm_16_420_llm_16_420_rrrruuuugggg_zero_f32 = 0;
        debug_assert_eq!(< f32 as Zero > ::zero(), 0f32);
        let _rug_ed_tests_llm_16_420_llm_16_420_rrrruuuugggg_zero_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_585 {
    use super::*;
    use crate::*;
    #[test]
    fn f64_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f64, f64, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< f64 as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < f64 as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < f64 as identities::One > ::is_one(& rug_fuzz_2));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_586 {
    use super::*;
    use crate::*;
    use crate::identities::One;
    #[test]
    fn test_one_f64() {
        let _rug_st_tests_llm_16_586_rrrruuuugggg_test_one_f64 = 0;
        let one_val: f64 = One::one();
        debug_assert_eq!(one_val, 1.0_f64);
        let _rug_ed_tests_llm_16_586_rrrruuuugggg_test_one_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_587_llm_16_587 {
    use crate::identities::Zero;
    #[test]
    fn test_f64_is_zero_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = rug_fuzz_0;
        debug_assert!(< f64 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_f64_is_zero_false() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = rug_fuzz_0;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_f64_is_zero_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = -rug_fuzz_0;
        debug_assert!(< f64 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_f64_is_zero_nan() {
        let _rug_st_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_nan = 0;
        let value: f64 = f64::NAN;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
        let _rug_ed_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_nan = 0;
    }
    #[test]
    fn test_f64_is_zero_infinity() {
        let _rug_st_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_infinity = 0;
        let value: f64 = f64::INFINITY;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
        let _rug_ed_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_infinity = 0;
    }
    #[test]
    fn test_f64_is_zero_negative_infinity() {
        let _rug_st_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_negative_infinity = 0;
        let value: f64 = f64::NEG_INFINITY;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
        let _rug_ed_tests_llm_16_587_llm_16_587_rrrruuuugggg_test_f64_is_zero_negative_infinity = 0;
    }
    #[test]
    fn test_f64_is_zero_small_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = rug_fuzz_0;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
             }
});    }
    #[test]
    fn test_f64_is_zero_small_negative_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: f64 = -rug_fuzz_0;
        debug_assert!(! < f64 as Zero > ::is_zero(& value));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_588_llm_16_588 {
    use crate::Zero;
    #[test]
    fn test_f64_zero() {
        let _rug_st_tests_llm_16_588_llm_16_588_rrrruuuugggg_test_f64_zero = 0;
        let z = <f64 as Zero>::zero();
        debug_assert_eq!(z, 0f64);
        let _rug_ed_tests_llm_16_588_llm_16_588_rrrruuuugggg_test_f64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_645 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i128 as identities::One > ::is_one(& rug_fuzz_0), true);
        debug_assert_eq!(< i128 as identities::One > ::is_one(& rug_fuzz_1), false);
        debug_assert_eq!(< i128 as identities::One > ::is_one(& rug_fuzz_2), false);
        debug_assert_eq!(< i128 as identities::One > ::is_one(& - rug_fuzz_3), false);
        debug_assert_eq!(
            < i128 as identities::One > ::is_one(& i128::max_value()), false
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_646 {
    use super::*;
    use crate::*;
    #[test]
    fn test_i128_one() {
        let _rug_st_tests_llm_16_646_rrrruuuugggg_test_i128_one = 0;
        debug_assert_eq!(< i128 as identities::One > ::one(), 1);
        let _rug_ed_tests_llm_16_646_rrrruuuugggg_test_i128_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_647 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< i128 as identities::Zero > ::is_zero(& rug_fuzz_0));
        debug_assert!(! < i128 as identities::Zero > ::is_zero(& rug_fuzz_1));
        debug_assert!(! < i128 as identities::Zero > ::is_zero(& - rug_fuzz_2));
        debug_assert!(! < i128 as identities::Zero > ::is_zero(& i128::MIN));
        debug_assert!(! < i128 as identities::Zero > ::is_zero(& i128::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_648 {
    use super::*;
    use crate::*;
    #[test]
    fn test_zero_i128() {
        let _rug_st_tests_llm_16_648_rrrruuuugggg_test_zero_i128 = 0;
        debug_assert_eq!(< i128 as identities::Zero > ::zero(), 0i128);
        let _rug_ed_tests_llm_16_648_rrrruuuugggg_test_zero_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_755_llm_16_755 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< i16 as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < i16 as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < i16 as identities::One > ::is_one(& rug_fuzz_2));
        debug_assert!(! < i16 as identities::One > ::is_one(& (- rug_fuzz_3)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_756 {
    use super::*;
    use crate::*;
    #[test]
    fn one_i16() {
        let _rug_st_tests_llm_16_756_rrrruuuugggg_one_i16 = 0;
        debug_assert_eq!(< i16 as identities::One > ::one(), 1i16);
        let _rug_ed_tests_llm_16_756_rrrruuuugggg_one_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_757_llm_16_757 {
    use crate::identities::Zero;
    #[test]
    fn test_i16_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< i16 as Zero > ::is_zero(& rug_fuzz_0));
        debug_assert!(! < i16 as Zero > ::is_zero(& rug_fuzz_1));
        debug_assert!(! < i16 as Zero > ::is_zero(& - rug_fuzz_2));
        debug_assert!(
            < i16 as Zero > ::is_zero(& i16::MIN.checked_neg().unwrap_or(i16::MIN))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_758_llm_16_758 {
    use crate::Zero;
    #[test]
    fn test_zero_for_i16() {
        let _rug_st_tests_llm_16_758_llm_16_758_rrrruuuugggg_test_zero_for_i16 = 0;
        debug_assert_eq!(< i16 as Zero > ::zero(), 0i16);
        let _rug_ed_tests_llm_16_758_llm_16_758_rrrruuuugggg_test_zero_for_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_865 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_one(), true);
        debug_assert_eq!(rug_fuzz_1.is_one(), false);
        debug_assert_eq!((- rug_fuzz_2).is_one(), false);
        debug_assert_eq!(rug_fuzz_3.is_one(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_866_llm_16_866 {
    use crate::identities::One;
    #[test]
    fn test_one_i32() {
        let _rug_st_tests_llm_16_866_llm_16_866_rrrruuuugggg_test_one_i32 = 0;
        let one_i32 = <i32 as One>::one();
        debug_assert_eq!(one_i32, 1);
        let _rug_ed_tests_llm_16_866_llm_16_866_rrrruuuugggg_test_one_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_867 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as identities::Zero > ::is_zero(& rug_fuzz_0), true);
             }
});    }
    #[test]
    fn test_is_zero_with_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as identities::Zero > ::is_zero(& rug_fuzz_0), false);
             }
});    }
    #[test]
    fn test_is_zero_with_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as identities::Zero > ::is_zero(& - rug_fuzz_0), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_868_llm_16_868 {
    use crate::identities::Zero;
    #[test]
    fn test_zero() {
        let _rug_st_tests_llm_16_868_llm_16_868_rrrruuuugggg_test_zero = 0;
        debug_assert_eq!(< i32 as Zero > ::zero(), 0);
        let _rug_ed_tests_llm_16_868_llm_16_868_rrrruuuugggg_test_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_975 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one_for_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< i64 as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < i64 as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < i64 as identities::One > ::is_one(& rug_fuzz_2));
        debug_assert!(! < i64 as identities::One > ::is_one(& - rug_fuzz_3));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_976_llm_16_976 {
    use crate::identities::One;
    #[test]
    fn one_i64() {
        let _rug_st_tests_llm_16_976_llm_16_976_rrrruuuugggg_one_i64 = 0;
        let one_value: i64 = One::one();
        debug_assert_eq!(one_value, 1);
        let _rug_ed_tests_llm_16_976_llm_16_976_rrrruuuugggg_one_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_977 {
    use super::*;
    use crate::*;
    #[test]
    fn i64_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as identities::Zero > ::is_zero(& rug_fuzz_0), true);
        debug_assert_eq!(< i64 as identities::Zero > ::is_zero(& rug_fuzz_1), false);
        debug_assert_eq!(< i64 as identities::Zero > ::is_zero(& - rug_fuzz_2), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_978_llm_16_978 {
    use crate::identities::Zero;
    #[test]
    fn test_zero_i64() {
        let _rug_st_tests_llm_16_978_llm_16_978_rrrruuuugggg_test_zero_i64 = 0;
        debug_assert_eq!(< i64 as Zero > ::zero(), 0i64);
        let _rug_ed_tests_llm_16_978_llm_16_978_rrrruuuugggg_test_zero_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1085 {
    use super::*;
    use crate::*;
    use crate::identities::One;
    #[test]
    fn test_is_one_for_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as One > ::is_one(& rug_fuzz_0), true);
        debug_assert_eq!(< i8 as One > ::is_one(& rug_fuzz_1), false);
        debug_assert_eq!(< i8 as One > ::is_one(& - rug_fuzz_2), false);
        debug_assert_eq!(< i8 as One > ::is_one(& rug_fuzz_3), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1086_llm_16_1086 {
    use crate::identities::One;
    #[test]
    fn test_one_for_i8() {
        let _rug_st_tests_llm_16_1086_llm_16_1086_rrrruuuugggg_test_one_for_i8 = 0;
        debug_assert_eq!(< i8 as One > ::one(), 1i8);
        let _rug_ed_tests_llm_16_1086_llm_16_1086_rrrruuuugggg_test_one_for_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1087 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero_for_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i8 as identities::Zero > ::is_zero(& rug_fuzz_0), true);
        debug_assert_eq!(< i8 as identities::Zero > ::is_zero(& rug_fuzz_1), false);
        debug_assert_eq!(< i8 as identities::Zero > ::is_zero(& - rug_fuzz_2), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1088_llm_16_1088 {
    use crate::identities::Zero;
    #[test]
    fn test_zero_for_i8() {
        let _rug_st_tests_llm_16_1088_llm_16_1088_rrrruuuugggg_test_zero_for_i8 = 0;
        debug_assert_eq!(< i8 as Zero > ::zero(), 0i8);
        let _rug_ed_tests_llm_16_1088_llm_16_1088_rrrruuuugggg_test_zero_for_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1195 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one_for_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< isize as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < isize as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < isize as identities::One > ::is_one(& rug_fuzz_2));
        debug_assert!(! < isize as identities::One > ::is_one(& isize::MIN));
        debug_assert!(! < isize as identities::One > ::is_one(& isize::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1196 {
    use super::*;
    use crate::*;
    #[test]
    fn one_isize() {
        let _rug_st_tests_llm_16_1196_rrrruuuugggg_one_isize = 0;
        debug_assert_eq!(< isize as identities::One > ::one(), 1isize);
        let _rug_ed_tests_llm_16_1196_rrrruuuugggg_one_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1197 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< isize as identities::Zero > ::is_zero(& rug_fuzz_0), true);
        debug_assert_eq!(< isize as identities::Zero > ::is_zero(& rug_fuzz_1), false);
        debug_assert_eq!(< isize as identities::Zero > ::is_zero(& - rug_fuzz_2), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1198_llm_16_1198 {
    use super::*;
    use crate::*;
    use crate::identities::Zero;
    #[test]
    fn zero_isize() {
        let _rug_st_tests_llm_16_1198_llm_16_1198_rrrruuuugggg_zero_isize = 0;
        debug_assert_eq!(< isize as Zero > ::zero(), 0isize);
        let _rug_ed_tests_llm_16_1198_llm_16_1198_rrrruuuugggg_zero_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1291_llm_16_1291 {
    use crate::identities::One;
    use std::num::Wrapping;
    #[test]
    fn one_for_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_1), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_2), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_3), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_4), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_5), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_6), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_7), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_8), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_9), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_10), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_11), Wrapping::one());
        debug_assert_eq!(Wrapping(rug_fuzz_12), Wrapping::one());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1292_llm_16_1292 {
    use crate::identities::One;
    use std::num::Wrapping;
    #[test]
    fn set_one_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut num = Wrapping(rug_fuzz_0);
        One::set_one(&mut num);
        debug_assert_eq!(num, Wrapping(1i32));
             }
});    }
    #[test]
    fn set_one_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut num = Wrapping(rug_fuzz_0);
        One::set_one(&mut num);
        debug_assert_eq!(num, Wrapping(1i64));
             }
});    }
    #[test]
    fn set_one_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut num = Wrapping(rug_fuzz_0);
        One::set_one(&mut num);
        debug_assert_eq!(num, Wrapping(1u32));
             }
});    }
    #[test]
    fn set_one_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut num = Wrapping(rug_fuzz_0);
        One::set_one(&mut num);
        debug_assert_eq!(num, Wrapping(1u64));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1293_llm_16_1293 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::identities::Zero;
    #[test]
    fn is_zero_for_zero_int() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero = Wrapping(rug_fuzz_0);
        debug_assert!(Zero::is_zero(& zero));
             }
});    }
    #[test]
    fn is_zero_for_nonzero_int() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_zero = Wrapping(rug_fuzz_0);
        debug_assert!(! Zero::is_zero(& non_zero));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1294 {
    use crate::Wrapping;
    use crate::Zero;
    #[test]
    fn set_zero_for_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut wrapping_value = Wrapping(rug_fuzz_0);
        wrapping_value.set_zero();
        debug_assert_eq!(wrapping_value, Wrapping(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1295_llm_16_1295 {
    use crate::identities::Zero;
    use std::num::Wrapping;
    #[test]
    fn zero_for_wrapping() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, i64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Wrapping(rug_fuzz_0), Wrapping::zero());
        debug_assert_eq!(Wrapping(rug_fuzz_1), Wrapping::zero());
        debug_assert_eq!(Wrapping(rug_fuzz_2), Wrapping::zero());
        debug_assert_eq!(Wrapping(rug_fuzz_3), Wrapping::zero());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1401 {
    use super::*;
    use crate::*;
    #[test]
    fn u128_is_one_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< u128 as identities::One > ::is_one(& rug_fuzz_0));
             }
});    }
    #[test]
    fn u128_is_one_false() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! < u128 as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < u128 as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < u128 as identities::One > ::is_one(& u128::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1402 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u128_one() {
        let _rug_st_tests_llm_16_1402_rrrruuuugggg_test_u128_one = 0;
        debug_assert_eq!(< u128 as identities::One > ::one(), 1u128);
        let _rug_ed_tests_llm_16_1402_rrrruuuugggg_test_u128_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1403_llm_16_1403 {
    use crate::identities::Zero;
    #[test]
    fn test_u128_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u128 as Zero > ::is_zero(& rug_fuzz_0), true);
        debug_assert_eq!(< u128 as Zero > ::is_zero(& rug_fuzz_1), false);
        debug_assert_eq!(< u128 as Zero > ::is_zero(& u128::max_value()), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1404_llm_16_1404 {
    use crate::identities::Zero;
    #[test]
    fn u128_zero_test() {
        let _rug_st_tests_llm_16_1404_llm_16_1404_rrrruuuugggg_u128_zero_test = 0;
        debug_assert_eq!(< u128 as Zero > ::zero(), 0u128);
        let _rug_ed_tests_llm_16_1404_llm_16_1404_rrrruuuugggg_u128_zero_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1506_llm_16_1506 {
    use crate::identities::One;
    #[test]
    fn test_is_one_for_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_one());
        debug_assert!(! rug_fuzz_1.is_one());
        debug_assert!(! rug_fuzz_2.is_one());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1507_llm_16_1507 {
    use crate::identities::One;
    #[test]
    fn test_u16_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0, < u16 as One > ::one());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1508 {
    use crate::identities::Zero;
    #[test]
    fn is_zero_for_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< u16 as Zero > ::is_zero(& rug_fuzz_0));
        debug_assert!(! < u16 as Zero > ::is_zero(& rug_fuzz_1));
        debug_assert!(! < u16 as Zero > ::is_zero(& u16::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1509 {
    use crate::Zero;
    #[test]
    fn test_zero_u16() {
        let _rug_st_tests_llm_16_1509_rrrruuuugggg_test_zero_u16 = 0;
        debug_assert_eq!(< u16 as Zero > ::zero(), 0u16);
        let _rug_ed_tests_llm_16_1509_rrrruuuugggg_test_zero_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1611 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_one(), true);
        debug_assert_eq!(rug_fuzz_1.is_one(), false);
        debug_assert_eq!(rug_fuzz_2.is_one(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1612 {
    use super::*;
    use crate::*;
    #[test]
    fn one_u32() {
        let _rug_st_tests_llm_16_1612_rrrruuuugggg_one_u32 = 0;
        debug_assert_eq!(< u32 as identities::One > ::one(), 1u32);
        let _rug_ed_tests_llm_16_1612_rrrruuuugggg_one_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1613 {
    use super::*;
    use crate::*;
    #[test]
    fn u32_is_zero_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as identities::Zero > ::is_zero(& rug_fuzz_0), true);
             }
});    }
    #[test]
    fn u32_is_zero_false() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as identities::Zero > ::is_zero(& rug_fuzz_0), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1614 {
    use crate::identities::Zero;
    #[test]
    fn test_zero_u32() {
        let _rug_st_tests_llm_16_1614_rrrruuuugggg_test_zero_u32 = 0;
        let zero_value: u32 = <u32 as Zero>::zero();
        debug_assert_eq!(zero_value, 0u32);
        let _rug_ed_tests_llm_16_1614_rrrruuuugggg_test_zero_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1716 {
    use super::*;
    use crate::*;
    #[test]
    fn is_one_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as identities::One > ::is_one(& rug_fuzz_0), false);
        debug_assert_eq!(< u64 as identities::One > ::is_one(& rug_fuzz_1), true);
        debug_assert_eq!(< u64 as identities::One > ::is_one(& rug_fuzz_2), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1717_llm_16_1717 {
    use crate::identities::One;
    #[test]
    fn test_one_u64() {
        let _rug_st_tests_llm_16_1717_llm_16_1717_rrrruuuugggg_test_one_u64 = 0;
        let one_u64: u64 = One::one();
        debug_assert_eq!(one_u64, 1_u64);
        let _rug_ed_tests_llm_16_1717_llm_16_1717_rrrruuuugggg_test_one_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1718 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero_for_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u64 as identities::Zero > ::is_zero(& rug_fuzz_0), true);
        debug_assert_eq!(< u64 as identities::Zero > ::is_zero(& rug_fuzz_1), false);
        debug_assert_eq!(< u64 as identities::Zero > ::is_zero(& u64::MAX), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1719 {
    use super::*;
    use crate::*;
    #[test]
    fn test_zero_u64() {
        let _rug_st_tests_llm_16_1719_rrrruuuugggg_test_zero_u64 = 0;
        let value = <u64 as Zero>::zero();
        debug_assert_eq!(value, 0u64);
        let _rug_ed_tests_llm_16_1719_rrrruuuugggg_test_zero_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1822_llm_16_1822 {
    use crate::identities::One;
    #[test]
    fn test_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(rug_fuzz_0.is_one());
        debug_assert!(! rug_fuzz_1.is_one());
        debug_assert!(! rug_fuzz_2.is_one());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1823 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u8_one() {
        let _rug_st_tests_llm_16_1823_rrrruuuugggg_test_u8_one = 0;
        debug_assert_eq!(< u8 as identities::One > ::one(), 1u8);
        let _rug_ed_tests_llm_16_1823_rrrruuuugggg_test_u8_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1824 {
    use crate::identities::Zero;
    #[test]
    fn test_u8_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.is_zero(), true);
        debug_assert_eq!(rug_fuzz_1.is_zero(), false);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1825_llm_16_1825 {
    use crate::identities::Zero;
    #[test]
    fn u8_zero_test() {
        let _rug_st_tests_llm_16_1825_llm_16_1825_rrrruuuugggg_u8_zero_test = 0;
        debug_assert_eq!(< u8 as Zero > ::zero(), 0u8);
        let _rug_ed_tests_llm_16_1825_llm_16_1825_rrrruuuugggg_u8_zero_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1927 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< usize as identities::One > ::is_one(& rug_fuzz_0));
        debug_assert!(! < usize as identities::One > ::is_one(& rug_fuzz_1));
        debug_assert!(! < usize as identities::One > ::is_one(& rug_fuzz_2));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1928 {
    use super::*;
    use crate::*;
    #[test]
    fn one_usize() {
        let _rug_st_tests_llm_16_1928_rrrruuuugggg_one_usize = 0;
        debug_assert_eq!(< usize as identities::One > ::one(), 1);
        let _rug_ed_tests_llm_16_1928_rrrruuuugggg_one_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1929_llm_16_1929 {
    use crate::Zero;
    #[test]
    fn test_is_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(< usize as Zero > ::is_zero(& rug_fuzz_0));
        debug_assert!(! < usize as Zero > ::is_zero(& rug_fuzz_1));
        debug_assert!(! < usize as Zero > ::is_zero(& usize::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1930_llm_16_1930 {
    use crate::identities::Zero;
    #[test]
    fn zero_usize() {
        let _rug_st_tests_llm_16_1930_llm_16_1930_rrrruuuugggg_zero_usize = 0;
        debug_assert_eq!(< usize as Zero > ::zero(), 0);
        let _rug_ed_tests_llm_16_1930_llm_16_1930_rrrruuuugggg_zero_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2040_llm_16_2040 {
    use crate::identities::One;
    #[test]
    fn test_is_one_for_integer() {
        assert_eq!(1.is_one(), true);
        assert_eq!(0.is_one(), false);
        assert_eq!((- 1).is_one(), false);
    }
    #[test]
    fn test_is_one_for_float() {
        assert_eq!(1.0.is_one(), true);
        assert_eq!(0.0.is_one(), false);
        assert_eq!((- 1.0).is_one(), false);
        assert_eq!(1.1.is_one(), false);
    }
    #[test]
    fn test_is_one_for_custom_type() {
        use std::ops::Mul;
        #[derive(Debug, PartialEq)]
        struct CustomType(i32);
        impl Mul for CustomType {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                CustomType(self.0 * rhs.0)
            }
        }
        impl One for CustomType {
            fn one() -> Self {
                CustomType(1)
            }
        }
        assert_eq!(CustomType(1).is_one(), true);
        assert_eq!(CustomType(0).is_one(), false);
    }
}
#[cfg(test)]
mod tests_llm_16_2041 {
    use super::*;
    use crate::*;
    use crate::identities::One;
    #[test]
    fn test_set_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: i32 = rug_fuzz_0;
        One::set_one(&mut value);
        debug_assert_eq!(value, 1);
        let mut value: f32 = rug_fuzz_1;
        One::set_one(&mut value);
        debug_assert_eq!(value, 1.0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2042 {
    use crate::identities::Zero;
    use std::ops::Add;
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: i32,
    }
    impl Add for TestStruct {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            TestStruct {
                value: self.value + other.value,
            }
        }
    }
    impl Zero for TestStruct {
        fn zero() -> Self {
            TestStruct { value: 0 }
        }
        fn is_zero(&self) -> bool {
            self.value == 0
        }
        fn set_zero(&mut self) {
            *self = Self::zero();
        }
    }
    #[test]
    fn test_set_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut test_value = TestStruct { value: rug_fuzz_0 };
        test_value.set_zero();
        debug_assert_eq!(test_value, TestStruct::zero());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_2043_llm_16_2043 {
    use crate::identities::one;
    use crate::identities::One;
    use std::num::Wrapping;
    #[test]
    fn one_for_wrapping_u8() {
        let _rug_st_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_u8 = 0;
        let one_value: Wrapping<u8> = one();
        debug_assert_eq!(one_value, Wrapping(1u8));
        let _rug_ed_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_u8 = 0;
    }
    #[test]
    fn one_for_wrapping_i32() {
        let _rug_st_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_i32 = 0;
        let one_value: Wrapping<i32> = one();
        debug_assert_eq!(one_value, Wrapping(1i32));
        let _rug_ed_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_i32 = 0;
    }
    #[test]
    fn one_for_wrapping_u64() {
        let _rug_st_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_u64 = 0;
        let one_value: Wrapping<u64> = one();
        debug_assert_eq!(one_value, Wrapping(1u64));
        let _rug_ed_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_u64 = 0;
    }
    #[test]
    fn one_for_wrapping_i128() {
        let _rug_st_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_i128 = 0;
        let one_value: Wrapping<i128> = one();
        debug_assert_eq!(one_value, Wrapping(1i128));
        let _rug_ed_tests_llm_16_2043_llm_16_2043_rrrruuuugggg_one_for_wrapping_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2044_llm_16_2044 {
    use crate::identities::{self, Zero};
    use std::num::Wrapping;
    #[test]
    fn zero_for_wrapping() {
        let _rug_st_tests_llm_16_2044_llm_16_2044_rrrruuuugggg_zero_for_wrapping = 0;
        let z: Wrapping<i32> = identities::zero();
        debug_assert_eq!(z, Wrapping(0));
        let z: Wrapping<u32> = identities::zero();
        debug_assert_eq!(z, Wrapping(0u32));
        let _rug_ed_tests_llm_16_2044_llm_16_2044_rrrruuuugggg_zero_for_wrapping = 0;
    }
}
