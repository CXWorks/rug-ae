use core::ops::{Add, Mul, Sub};
/// Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
/// `SaturatingMul` instead.
pub trait Saturating {
    /// Saturating addition operator.
    /// Returns a+b, saturating at the numeric bounds instead of overflowing.
    fn saturating_add(self, v: Self) -> Self;
    /// Saturating subtraction operator.
    /// Returns a-b, saturating at the numeric bounds instead of overflowing.
    fn saturating_sub(self, v: Self) -> Self;
}
macro_rules! deprecated_saturating_impl {
    ($trait_name:ident for $($t:ty)*) => {
        $(impl $trait_name for $t { #[inline] fn saturating_add(self, v : Self) -> Self {
        Self::saturating_add(self, v) } #[inline] fn saturating_sub(self, v : Self) ->
        Self { Self::saturating_sub(self, v) } })*
    };
}
deprecated_saturating_impl!(Saturating for isize i8 i16 i32 i64 i128);
deprecated_saturating_impl!(Saturating for usize u8 u16 u32 u64 u128);
macro_rules! saturating_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, v : & Self) -> Self { <$t
        >::$method (* self, * v) } }
    };
}
/// Performs addition that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingAdd: Sized + Add<Self, Output = Self> {
    /// Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of
    /// the type.
    fn saturating_add(&self, v: &Self) -> Self;
}
saturating_impl!(SaturatingAdd, saturating_add, u8);
saturating_impl!(SaturatingAdd, saturating_add, u16);
saturating_impl!(SaturatingAdd, saturating_add, u32);
saturating_impl!(SaturatingAdd, saturating_add, u64);
saturating_impl!(SaturatingAdd, saturating_add, usize);
saturating_impl!(SaturatingAdd, saturating_add, u128);
saturating_impl!(SaturatingAdd, saturating_add, i8);
saturating_impl!(SaturatingAdd, saturating_add, i16);
saturating_impl!(SaturatingAdd, saturating_add, i32);
saturating_impl!(SaturatingAdd, saturating_add, i64);
saturating_impl!(SaturatingAdd, saturating_add, isize);
saturating_impl!(SaturatingAdd, saturating_add, i128);
/// Performs subtraction that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingSub: Sized + Sub<Self, Output = Self> {
    /// Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of
    /// the type.
    fn saturating_sub(&self, v: &Self) -> Self;
}
saturating_impl!(SaturatingSub, saturating_sub, u8);
saturating_impl!(SaturatingSub, saturating_sub, u16);
saturating_impl!(SaturatingSub, saturating_sub, u32);
saturating_impl!(SaturatingSub, saturating_sub, u64);
saturating_impl!(SaturatingSub, saturating_sub, usize);
saturating_impl!(SaturatingSub, saturating_sub, u128);
saturating_impl!(SaturatingSub, saturating_sub, i8);
saturating_impl!(SaturatingSub, saturating_sub, i16);
saturating_impl!(SaturatingSub, saturating_sub, i32);
saturating_impl!(SaturatingSub, saturating_sub, i64);
saturating_impl!(SaturatingSub, saturating_sub, isize);
saturating_impl!(SaturatingSub, saturating_sub, i128);
/// Performs multiplication that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingMul: Sized + Mul<Self, Output = Self> {
    /// Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of
    /// the type.
    fn saturating_mul(&self, v: &Self) -> Self;
}
saturating_impl!(SaturatingMul, saturating_mul, u8);
saturating_impl!(SaturatingMul, saturating_mul, u16);
saturating_impl!(SaturatingMul, saturating_mul, u32);
saturating_impl!(SaturatingMul, saturating_mul, u64);
saturating_impl!(SaturatingMul, saturating_mul, usize);
saturating_impl!(SaturatingMul, saturating_mul, u128);
saturating_impl!(SaturatingMul, saturating_mul, i8);
saturating_impl!(SaturatingMul, saturating_mul, i16);
saturating_impl!(SaturatingMul, saturating_mul, i32);
saturating_impl!(SaturatingMul, saturating_mul, i64);
saturating_impl!(SaturatingMul, saturating_mul, isize);
saturating_impl!(SaturatingMul, saturating_mul, i128);
#[test]
fn test_saturating_traits() {
    fn saturating_add<T: SaturatingAdd>(a: T, b: T) -> T {
        a.saturating_add(&b)
    }
    fn saturating_sub<T: SaturatingSub>(a: T, b: T) -> T {
        a.saturating_sub(&b)
    }
    fn saturating_mul<T: SaturatingMul>(a: T, b: T) -> T {
        a.saturating_mul(&b)
    }
    assert_eq!(saturating_add(255, 1), 255u8);
    assert_eq!(saturating_add(127, 1), 127i8);
    assert_eq!(saturating_add(- 128, - 1), - 128i8);
    assert_eq!(saturating_sub(0, 1), 0u8);
    assert_eq!(saturating_sub(- 128, 1), - 128i8);
    assert_eq!(saturating_sub(127, - 1), 127i8);
    assert_eq!(saturating_mul(255, 2), 255u8);
    assert_eq!(saturating_mul(127, 2), 127i8);
    assert_eq!(saturating_mul(- 128, 2), - 128i8);
}
#[cfg(test)]
mod tests_llm_16_685_llm_16_685 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_685_llm_16_685_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 100;
        let rug_fuzz_5 = 200;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(i128::saturating_add(i128::MAX, rug_fuzz_0), i128::MAX);
        debug_assert_eq!(i128::saturating_add(i128::MIN, - rug_fuzz_1), i128::MIN);
        debug_assert_eq!(i128::saturating_add(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(i128::saturating_add(rug_fuzz_4, rug_fuzz_5), 300);
        debug_assert_eq!(
            i128::saturating_add(i128::MAX - rug_fuzz_6, rug_fuzz_7), i128::MAX
        );
        debug_assert_eq!(
            i128::saturating_add(i128::MIN + rug_fuzz_8, - rug_fuzz_9), i128::MIN
        );
        debug_assert_eq!(i128::saturating_add(i128::MAX, i128::MIN), - 1);
        let _rug_ed_tests_llm_16_685_llm_16_685_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_686_llm_16_686 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_686_llm_16_686_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2;
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(i128::MAX, rug_fuzz_0), i128::MAX - 1
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(i128::MIN, - rug_fuzz_1), i128::MIN
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(rug_fuzz_2, i128::MAX), i128::MIN
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(i128::MIN, i128::MAX), i128::MIN
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(rug_fuzz_3, rug_fuzz_4), 0
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(- rug_fuzz_5, rug_fuzz_6), - 3
        );
        debug_assert_eq!(
            < i128 as Saturating > ::saturating_sub(i128::MIN + rug_fuzz_7, -
            rug_fuzz_8), i128::MIN
        );
        let _rug_ed_tests_llm_16_686_llm_16_686_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_688_llm_16_688 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_saturating_mul() {
        let _rug_st_tests_llm_16_688_llm_16_688_rrrruuuugggg_test_saturating_mul = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 1000
        );
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& i128::MAX, & rug_fuzz_2),
            i128::MAX
        );
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& i128::MIN, & rug_fuzz_3),
            i128::MIN
        );
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& i128::MIN, & - rug_fuzz_4),
            i128::MAX
        );
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& rug_fuzz_5, & i128::MAX), 0
        );
        debug_assert_eq!(
            < i128 as SaturatingMul > ::saturating_mul(& rug_fuzz_6, & i128::MAX),
            i128::MAX
        );
        let _rug_ed_tests_llm_16_688_llm_16_688_rrrruuuugggg_test_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_689_llm_16_689 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_689_llm_16_689_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 100;
        debug_assert_eq!(i128::saturating_sub(rug_fuzz_0, rug_fuzz_1), 90);
        debug_assert_eq!(i128::saturating_sub(rug_fuzz_2, rug_fuzz_3), - 10);
        debug_assert_eq!(i128::saturating_sub(i128::MIN, rug_fuzz_4), i128::MIN);
        debug_assert_eq!(i128::saturating_sub(i128::MAX, i128::MIN), i128::MAX);
        debug_assert_eq!(i128::saturating_sub(- rug_fuzz_5, i128::MAX), i128::MIN);
        let _rug_ed_tests_llm_16_689_llm_16_689_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_795_llm_16_795 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add_i16() {
        assert_eq!(i16::saturating_add(1000, 32767), 32767);
        assert_eq!(i16::saturating_add(0, 0), 0);
        assert_eq!(i16::saturating_add(- 32768, - 1), - 32768);
        assert_eq!(i16::saturating_add(32767, - 1000), 31767);
        assert_eq!(i16::saturating_add(- 32768, 1), - 32767);
        assert_eq!(i16::saturating_add(1, - 1), 0);
        assert_eq!(i16::saturating_add(- 32768, 32767), - 1);
        assert_eq!(i16::saturating_add(32767, 32767), 32767);
        assert_eq!(i16::saturating_add(- 32768, - 32768), - 32768);
    }
}
#[cfg(test)]
mod tests_llm_16_796_llm_16_796 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_796_llm_16_796_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        debug_assert_eq!(
            < i16 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < i16 as Saturating > ::saturating_sub(rug_fuzz_2, rug_fuzz_3), - 5
        );
        debug_assert_eq!(
            < i16 as Saturating > ::saturating_sub(i16::MIN, rug_fuzz_4), i16::MIN
        );
        debug_assert_eq!(
            < i16 as Saturating > ::saturating_sub(i16::MAX, - rug_fuzz_5), i16::MAX
        );
        debug_assert_eq!(
            < i16 as Saturating > ::saturating_sub(- rug_fuzz_6, i16::MAX), i16::MIN
        );
        let _rug_ed_tests_llm_16_796_llm_16_796_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_797_llm_16_797 {
    use crate::SaturatingAdd;
    #[test]
    fn test_saturating_add() {
        assert_eq!(i16::saturating_add(100, 32757), 32767);
        assert_eq!(i16::saturating_add(- 100, - 32757), - 32767);
        assert_eq!(i16::saturating_add(32767, 1), 32767);
        assert_eq!(i16::saturating_add(- 32768, - 1), - 32768);
    }
}
#[cfg(test)]
mod tests_llm_16_798_llm_16_798 {
    use super::*;
    use crate::*;
    #[test]
    fn i16_saturating_mul_test() {
        assert_eq!(i16::saturating_mul(100, 32), 3200);
        assert_eq!(i16::saturating_mul(1000, 1000), i16::MAX);
        assert_eq!(i16::saturating_mul(- 1000, 1000), i16::MIN);
        assert_eq!(i16::saturating_mul(- 32768, 1), - 32768);
        assert_eq!(i16::saturating_mul(0, 32767), 0);
        assert_eq!(i16::saturating_mul(1, - 32768), - 32768);
        assert_eq!(i16::saturating_mul(- 1, - 32768), i16::MAX);
    }
}
#[cfg(test)]
mod tests_llm_16_799_llm_16_799 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn saturating_sub_positive() {
        let _rug_st_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_positive = 0;
        let rug_fuzz_0 = 5i16;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2);
        let _rug_ed_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_positive = 0;
    }
    #[test]
    fn saturating_sub_negative() {
        let _rug_st_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_negative = 0;
        let rug_fuzz_0 = 5i16;
        let rug_fuzz_1 = 3;
        debug_assert_eq!((- rug_fuzz_0).saturating_sub(rug_fuzz_1), - 8);
        let _rug_ed_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_negative = 0;
    }
    #[test]
    fn saturating_sub_overflow() {
        let _rug_st_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_overflow = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(i16::MIN.saturating_sub(rug_fuzz_0), i16::MIN);
        let _rug_ed_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_overflow = 0;
    }
    #[test]
    fn saturating_sub_underflow() {
        let _rug_st_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_underflow = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(i16::MAX.saturating_sub(- rug_fuzz_0), i16::MAX);
        let _rug_ed_tests_llm_16_799_llm_16_799_rrrruuuugggg_saturating_sub_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_905_llm_16_905 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_905_llm_16_905_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 200;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 100;
        let rug_fuzz_7 = 200;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(rug_fuzz_0, rug_fuzz_1), 300
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(i32::MAX, rug_fuzz_2), i32::MAX
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(i32::MIN, - rug_fuzz_3), i32::MIN
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(rug_fuzz_4, rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(- rug_fuzz_6, - rug_fuzz_7), - 300
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(i32::MIN, rug_fuzz_8), i32::MIN
        );
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_add(i32::MAX, rug_fuzz_9), i32::MAX
        );
        let _rug_ed_tests_llm_16_905_llm_16_905_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_906_llm_16_906 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn saturating_sub_with_no_overflow() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 90
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow = 0;
    }
    #[test]
    fn saturating_sub_with_negative_result() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_negative_result = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 100;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), - 90
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_negative_result = 0;
    }
    #[test]
    fn saturating_sub_with_overflow_at_bottom() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_overflow_at_bottom = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(i32::MIN, rug_fuzz_0), i32::MIN
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_overflow_at_bottom = 0;
    }
    #[test]
    fn saturating_sub_with_no_overflow_at_bottom() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow_at_bottom = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(i32::MIN, rug_fuzz_0), i32::MIN
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow_at_bottom = 0;
    }
    #[test]
    fn saturating_sub_with_overflow_at_top() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_overflow_at_top = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(i32::MAX, - rug_fuzz_0), i32::MAX
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_overflow_at_top = 0;
    }
    #[test]
    fn saturating_sub_with_no_overflow_at_top() {
        let _rug_st_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow_at_top = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(
            < i32 as Saturating > ::saturating_sub(i32::MAX, rug_fuzz_0), i32::MAX
        );
        let _rug_ed_tests_llm_16_906_llm_16_906_rrrruuuugggg_saturating_sub_with_no_overflow_at_top = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_907_llm_16_907 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_907_llm_16_907_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 20;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert_eq!(
            < i32 as SaturatingAdd > ::saturating_add(& rug_fuzz_0, & rug_fuzz_1), 120
        );
        debug_assert_eq!(
            < i32 as SaturatingAdd > ::saturating_add(& i32::MAX, & rug_fuzz_2), i32::MAX
        );
        debug_assert_eq!(
            < i32 as SaturatingAdd > ::saturating_add(& i32::MIN, & - rug_fuzz_3),
            i32::MIN
        );
        debug_assert_eq!(
            < i32 as SaturatingAdd > ::saturating_add(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        let _rug_ed_tests_llm_16_907_llm_16_907_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_908_llm_16_908 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_saturating_mul() {
        let _rug_st_tests_llm_16_908_llm_16_908_rrrruuuugggg_test_saturating_mul = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 5;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 10;
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 50
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& i32::MAX, & rug_fuzz_2), i32::MAX
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& i32::MIN, & rug_fuzz_3), i32::MIN
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& i32::MAX, & - rug_fuzz_4),
            i32::MIN
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& i32::MIN, & - rug_fuzz_5),
            i32::MAX
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& - rug_fuzz_6, & rug_fuzz_7), - 50
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& rug_fuzz_8, & - rug_fuzz_9), - 50
        );
        debug_assert_eq!(
            < i32 as SaturatingMul > ::saturating_mul(& - rug_fuzz_10, & - rug_fuzz_11),
            50
        );
        let _rug_ed_tests_llm_16_908_llm_16_908_rrrruuuugggg_test_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_909_llm_16_909 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_909_llm_16_909_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(
            < i32 as SaturatingSub > ::saturating_sub(& rug_fuzz_0, & rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < i32 as SaturatingSub > ::saturating_sub(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i32 as SaturatingSub > ::saturating_sub(& i32::MIN, & rug_fuzz_4), i32::MIN
        );
        debug_assert_eq!(
            < i32 as SaturatingSub > ::saturating_sub(& i32::MAX, & - rug_fuzz_5),
            i32::MAX
        );
        let _rug_ed_tests_llm_16_909_llm_16_909_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1015_llm_16_1015 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1015_llm_16_1015_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MAX, rug_fuzz_0), i64::MAX
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MIN, - rug_fuzz_1), i64::MIN
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(rug_fuzz_2, rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(rug_fuzz_4, - rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MAX, rug_fuzz_6), i64::MAX
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MIN, rug_fuzz_7), i64::MIN
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MAX - rug_fuzz_8, rug_fuzz_9),
            i64::MAX
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_add(i64::MIN + rug_fuzz_10, -
            rug_fuzz_11), i64::MIN
        );
        let _rug_ed_tests_llm_16_1015_llm_16_1015_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1016_llm_16_1016 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_1016_llm_16_1016_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_sub(i64::MIN, rug_fuzz_2), i64::MIN
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_sub(i64::MAX, - rug_fuzz_3), i64::MAX
        );
        debug_assert_eq!(
            < i64 as Saturating > ::saturating_sub(rug_fuzz_4, i64::MAX), i64::MIN + 1
        );
        let _rug_ed_tests_llm_16_1016_llm_16_1016_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1017_llm_16_1017 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1017_llm_16_1017_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1234;
        let rug_fuzz_5 = 5678;
        let rug_fuzz_6 = 1234;
        let rug_fuzz_7 = 5678;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& i64::MAX, & rug_fuzz_0), i64::MAX
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& i64::MIN, & - rug_fuzz_1),
            i64::MIN
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& rug_fuzz_4, & rug_fuzz_5), 6912
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& - rug_fuzz_6, & - rug_fuzz_7), -
            6912
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& i64::MAX, & - rug_fuzz_8),
            i64::MAX - 1
        );
        debug_assert_eq!(
            < i64 as SaturatingAdd > ::saturating_add(& i64::MIN, & rug_fuzz_9), i64::MIN
            + 1
        );
        let _rug_ed_tests_llm_16_1017_llm_16_1017_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1018_llm_16_1018 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn saturating_mul_test() {
        let _rug_st_tests_llm_16_1018_llm_16_1018_rrrruuuugggg_saturating_mul_test = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 100
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& rug_fuzz_2, & i64::MAX), i64::MAX
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& - rug_fuzz_3, & rug_fuzz_4), -
            100
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& i64::MIN, & - rug_fuzz_5),
            i64::MAX
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& i64::MAX, & rug_fuzz_6), i64::MAX
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& i64::MIN, & rug_fuzz_7), i64::MIN
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& i64::MAX, & - rug_fuzz_8),
            i64::MIN
        );
        debug_assert_eq!(
            < i64 as SaturatingMul > ::saturating_mul(& rug_fuzz_9, & rug_fuzz_10), 0
        );
        let _rug_ed_tests_llm_16_1018_llm_16_1018_rrrruuuugggg_saturating_mul_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1019_llm_16_1019 {
    use crate::SaturatingSub;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_1019_llm_16_1019_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert_eq!(i64::saturating_sub(rug_fuzz_0, rug_fuzz_1), 0);
        debug_assert_eq!(i64::saturating_sub(i64::MAX, rug_fuzz_2), i64::MAX - 1);
        debug_assert_eq!(i64::saturating_sub(i64::MIN, rug_fuzz_3), i64::MIN);
        debug_assert_eq!(i64::saturating_sub(rug_fuzz_4, i64::MAX), - i64::MAX);
        debug_assert_eq!(i64::saturating_sub(i64::MIN, - i64::MAX), i64::MIN);
        debug_assert_eq!(i64::saturating_sub(rug_fuzz_5, i64::MIN), i64::MAX);
        let _rug_ed_tests_llm_16_1019_llm_16_1019_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1125_llm_16_1125 {
    use crate::Saturating;
    #[test]
    fn i8_saturating_add() {
        let _rug_st_tests_llm_16_1125_llm_16_1125_rrrruuuugggg_i8_saturating_add = 0;
        let rug_fuzz_0 = 100i8;
        let rug_fuzz_1 = 27i8;
        let rug_fuzz_2 = 100i8;
        let rug_fuzz_3 = 28i8;
        let rug_fuzz_4 = 100i8;
        let rug_fuzz_5 = 27i8;
        let rug_fuzz_6 = 100i8;
        let rug_fuzz_7 = 28i8;
        let rug_fuzz_8 = 100i8;
        let rug_fuzz_9 = 100i8;
        let rug_fuzz_10 = 0i8;
        let rug_fuzz_11 = 0i8;
        let rug_fuzz_12 = 0i8;
        let rug_fuzz_13 = 1i8;
        let rug_fuzz_14 = 0i8;
        let rug_fuzz_15 = 1i8;
        debug_assert_eq!(Saturating::saturating_add(rug_fuzz_0, rug_fuzz_1), 127i8);
        debug_assert_eq!(Saturating::saturating_add(rug_fuzz_2, rug_fuzz_3), 127i8);
        debug_assert_eq!(
            Saturating::saturating_add(- rug_fuzz_4, - rug_fuzz_5), - 127i8
        );
        debug_assert_eq!(
            Saturating::saturating_add(- rug_fuzz_6, - rug_fuzz_7), - 128i8
        );
        debug_assert_eq!(Saturating::saturating_add(- rug_fuzz_8, rug_fuzz_9), 0i8);
        debug_assert_eq!(Saturating::saturating_add(rug_fuzz_10, rug_fuzz_11), 0i8);
        debug_assert_eq!(Saturating::saturating_add(i8::MAX, rug_fuzz_12), i8::MAX);
        debug_assert_eq!(Saturating::saturating_add(i8::MAX, rug_fuzz_13), i8::MAX);
        debug_assert_eq!(Saturating::saturating_add(i8::MIN, rug_fuzz_14), i8::MIN);
        debug_assert_eq!(Saturating::saturating_add(i8::MIN, - rug_fuzz_15), i8::MIN);
        let _rug_ed_tests_llm_16_1125_llm_16_1125_rrrruuuugggg_i8_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1126 {
    use crate::Saturating;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1126_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 100;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        debug_assert_eq!(
            < i8 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 90
        );
        debug_assert_eq!(
            < i8 as Saturating > ::saturating_sub(rug_fuzz_2, rug_fuzz_3), - 90
        );
        debug_assert_eq!(
            < i8 as Saturating > ::saturating_sub(i8::MAX, rug_fuzz_4), 126
        );
        debug_assert_eq!(
            < i8 as Saturating > ::saturating_sub(i8::MIN, - rug_fuzz_5), - 128
        );
        debug_assert_eq!(
            < i8 as Saturating > ::saturating_sub(rug_fuzz_6, rug_fuzz_7), 0
        );
        debug_assert_eq!(< i8 as Saturating > ::saturating_sub(i8::MIN, i8::MAX), - 1);
        debug_assert_eq!(< i8 as Saturating > ::saturating_sub(i8::MAX, i8::MIN), 127);
        let _rug_ed_tests_llm_16_1126_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1127_llm_16_1127 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn saturating_add_i8() {
        assert_eq!(< i8 as SaturatingAdd >::saturating_add(& 100, & 27), 127_i8);
        assert_eq!(< i8 as SaturatingAdd >::saturating_add(& 100, & 127), 127_i8);
        assert_eq!(< i8 as SaturatingAdd >::saturating_add(&- 100, &- 27), - 127_i8);
        assert_eq!(< i8 as SaturatingAdd >::saturating_add(&- 100, &- 128), - 128_i8);
        assert_eq!(< i8 as SaturatingAdd >::saturating_add(& 0, & 0), 0_i8);
    }
}
#[cfg(test)]
mod tests_llm_16_1129_llm_16_1129 {
    use crate::SaturatingSub;
    #[test]
    fn i8_saturating_sub_test() {
        assert_eq!(i8::saturating_sub(0, 0), 0);
        assert_eq!(i8::saturating_sub(100, 1), 99);
        assert_eq!(i8::saturating_sub(0, 100), - 100);
        assert_eq!(i8::saturating_sub(- 100, 100), - 128);
        assert_eq!(i8::saturating_sub(- 128, 1), - 128);
        assert_eq!(i8::saturating_sub(127, - 1), 127);
        assert_eq!(i8::saturating_sub(- 127, 127), - 128);
    }
}
#[cfg(test)]
mod tests_llm_16_1235_llm_16_1235 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn saturating_add_test() {
        let _rug_st_tests_llm_16_1235_llm_16_1235_rrrruuuugggg_saturating_add_test = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 2;
        debug_assert_eq!(isize::saturating_add(isize::MAX, rug_fuzz_0), isize::MAX);
        debug_assert_eq!(isize::saturating_add(isize::MIN, - rug_fuzz_1), isize::MIN);
        debug_assert_eq!(isize::saturating_add(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(isize::saturating_add(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(isize::saturating_add(rug_fuzz_6, rug_fuzz_7), 1);
        debug_assert_eq!(isize::saturating_add(isize::MAX, rug_fuzz_8), isize::MAX);
        debug_assert_eq!(isize::saturating_add(isize::MIN, rug_fuzz_9), isize::MIN);
        debug_assert_eq!(isize::saturating_add(rug_fuzz_10, isize::MAX), isize::MAX);
        debug_assert_eq!(isize::saturating_add(- rug_fuzz_11, isize::MIN), isize::MIN);
        debug_assert_eq!(
            isize::saturating_add(isize::MAX / rug_fuzz_12, isize::MAX / rug_fuzz_13),
            isize::MAX
        );
        debug_assert_eq!(
            isize::saturating_add(isize::MIN / rug_fuzz_14, isize::MIN / rug_fuzz_15),
            isize::MIN
        );
        let _rug_ed_tests_llm_16_1235_llm_16_1235_rrrruuuugggg_saturating_add_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1236_llm_16_1236 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1236_llm_16_1236_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        debug_assert_eq!(
            < isize as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < isize as Saturating > ::saturating_sub(isize::MIN, rug_fuzz_2), isize::MIN
        );
        debug_assert_eq!(
            < isize as Saturating > ::saturating_sub(rug_fuzz_3, isize::MAX), -
            (isize::MAX as isize)
        );
        debug_assert_eq!(
            < isize as Saturating > ::saturating_sub(isize::MIN, isize::MAX), isize::MIN
            + 1
        );
        debug_assert_eq!(
            < isize as Saturating > ::saturating_sub(isize::MAX, isize::MIN), isize::MAX
        );
        let _rug_ed_tests_llm_16_1236_llm_16_1236_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1237_llm_16_1237 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn saturating_add_isize() {
        let _rug_st_tests_llm_16_1237_llm_16_1237_rrrruuuugggg_saturating_add_isize = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& isize::MAX, & rug_fuzz_0),
            isize::MAX
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& isize::MIN, & - rug_fuzz_1),
            isize::MIN
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& rug_fuzz_2, & rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& rug_fuzz_4, & rug_fuzz_5), 1
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& rug_fuzz_6, & isize::MAX),
            isize::MAX
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& - rug_fuzz_7, & isize::MIN),
            isize::MIN
        );
        debug_assert_eq!(
            < isize as SaturatingAdd > ::saturating_add(& - rug_fuzz_8, & rug_fuzz_9), 0
        );
        let _rug_ed_tests_llm_16_1237_llm_16_1237_rrrruuuugggg_saturating_add_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1238_llm_16_1238 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_saturating_mul() {
        let _rug_st_tests_llm_16_1238_llm_16_1238_rrrruuuugggg_test_saturating_mul = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 2;
        debug_assert_eq!(isize::saturating_mul(isize::MAX, rug_fuzz_0), isize::MAX);
        debug_assert_eq!(isize::saturating_mul(isize::MAX, rug_fuzz_1), 0);
        debug_assert_eq!(isize::saturating_mul(isize::MAX, rug_fuzz_2), isize::MAX);
        debug_assert_eq!(isize::saturating_mul(isize::MIN, rug_fuzz_3), isize::MIN);
        debug_assert_eq!(isize::saturating_mul(isize::MIN, rug_fuzz_4), 0);
        debug_assert_eq!(isize::saturating_mul(isize::MIN, rug_fuzz_5), isize::MIN);
        debug_assert_eq!(isize::saturating_mul(isize::MIN, - rug_fuzz_6), isize::MAX);
        debug_assert_eq!(isize::saturating_mul(rug_fuzz_7, rug_fuzz_8), 2);
        debug_assert_eq!(isize::saturating_mul(- rug_fuzz_9, - rug_fuzz_10), 2);
        debug_assert_eq!(isize::saturating_mul(rug_fuzz_11, - rug_fuzz_12), - 2);
        debug_assert_eq!(
            isize::saturating_mul(isize::MAX / rug_fuzz_13, rug_fuzz_14), isize::MAX - 1
        );
        debug_assert_eq!(
            isize::saturating_mul(isize::MAX / rug_fuzz_15, - rug_fuzz_16), isize::MIN +
            1
        );
        let _rug_ed_tests_llm_16_1238_llm_16_1238_rrrruuuugggg_test_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1239_llm_16_1239 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_1239_llm_16_1239_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 5isize;
        let rug_fuzz_1 = 3isize;
        let rug_fuzz_2 = 0isize;
        let rug_fuzz_3 = 3isize;
        let rug_fuzz_4 = 1isize;
        let rug_fuzz_5 = 1isize;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2isize);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0isize);
        debug_assert_eq!(isize::MIN.saturating_sub(rug_fuzz_4), isize::MIN);
        debug_assert_eq!(isize::MAX.saturating_sub(- rug_fuzz_5), isize::MAX);
        let _rug_ed_tests_llm_16_1239_llm_16_1239_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1442_llm_16_1442 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn u128_saturating_sub() {
        let _rug_st_tests_llm_16_1442_llm_16_1442_rrrruuuugggg_u128_saturating_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 50;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 50;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(u128::saturating_sub(rug_fuzz_0, rug_fuzz_1), 50);
        debug_assert_eq!(u128::saturating_sub(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(u128::saturating_sub(u128::MAX, rug_fuzz_4), u128::MAX - 1);
        debug_assert_eq!(u128::saturating_sub(u128::MIN, rug_fuzz_5), u128::MIN);
        let _rug_ed_tests_llm_16_1442_llm_16_1442_rrrruuuugggg_u128_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1443_llm_16_1443 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1443_llm_16_1443_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 2;
        debug_assert_eq!(SaturatingAdd::saturating_add(& rug_fuzz_0, & rug_fuzz_1), 0);
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& u128::MAX, & rug_fuzz_2), u128::MAX
        );
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& rug_fuzz_3, & u128::MAX), u128::MAX
        );
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& u128::MAX, & rug_fuzz_4), u128::MAX
        );
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& rug_fuzz_5, & u128::MAX), u128::MAX
        );
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& (u128::MAX / rug_fuzz_6), & (u128::MAX /
            rug_fuzz_7)), u128::MAX - 1
        );
        debug_assert_eq!(
            SaturatingAdd::saturating_add(& (u128::MAX / rug_fuzz_8 + rug_fuzz_9), &
            (u128::MAX / rug_fuzz_10)), u128::MAX
        );
        let _rug_ed_tests_llm_16_1443_llm_16_1443_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1444_llm_16_1444 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_u128_saturating_mul() {
        let _rug_st_tests_llm_16_1444_llm_16_1444_rrrruuuugggg_test_u128_saturating_mul = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2;
        debug_assert_eq!(
            < u128 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 0
        );
        debug_assert_eq!(
            < u128 as SaturatingMul > ::saturating_mul(& rug_fuzz_2, & rug_fuzz_3), 1
        );
        debug_assert_eq!(
            < u128 as SaturatingMul > ::saturating_mul(& rug_fuzz_4, & u128::MAX),
            u128::MAX
        );
        debug_assert_eq!(
            < u128 as SaturatingMul > ::saturating_mul(& rug_fuzz_5, & (u128::MAX /
            rug_fuzz_6)), u128::MAX - 1
        );
        debug_assert_eq!(
            < u128 as SaturatingMul > ::saturating_mul(& u128::MAX, & u128::MAX),
            u128::MAX
        );
        let _rug_ed_tests_llm_16_1444_llm_16_1444_rrrruuuugggg_test_u128_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1445_llm_16_1445 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn u128_saturating_sub() {
        let _rug_st_tests_llm_16_1445_llm_16_1445_rrrruuuugggg_u128_saturating_sub = 0;
        let rug_fuzz_0 = 10u128;
        let rug_fuzz_1 = 5u128;
        let rug_fuzz_2 = 0u128;
        let rug_fuzz_3 = 5u128;
        let rug_fuzz_4 = 0u128;
        let rug_fuzz_5 = 1u128;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 5u128);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0u128);
        debug_assert_eq!(u128::MAX.saturating_sub(u128::MAX), 0u128);
        debug_assert_eq!(rug_fuzz_4.saturating_sub(u128::MAX), 0u128);
        debug_assert_eq!(rug_fuzz_5.saturating_sub(u128::MAX), 0u128);
        let _rug_ed_tests_llm_16_1445_llm_16_1445_rrrruuuugggg_u128_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1546 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1546_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 123;
        let rug_fuzz_10 = 456;
        debug_assert_eq!(u16::saturating_add(u16::MAX, rug_fuzz_0), u16::MAX);
        debug_assert_eq!(u16::saturating_add(u16::MAX, rug_fuzz_1), u16::MAX);
        debug_assert_eq!(u16::saturating_add(rug_fuzz_2, u16::MAX), u16::MAX);
        debug_assert_eq!(
            u16::saturating_add(rug_fuzz_3, u16::MAX - rug_fuzz_4), u16::MAX
        );
        debug_assert_eq!(u16::saturating_add(rug_fuzz_5, rug_fuzz_6), 0);
        debug_assert_eq!(u16::saturating_add(rug_fuzz_7, rug_fuzz_8), 2);
        debug_assert_eq!(u16::saturating_add(rug_fuzz_9, rug_fuzz_10), 579);
        let _rug_ed_tests_llm_16_1546_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1547_llm_16_1547 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_1547_llm_16_1547_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 100u16;
        let rug_fuzz_1 = 101;
        let rug_fuzz_2 = 100u16;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0u16;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_0, rug_fuzz_1), 0);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_2, rug_fuzz_3), 99);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(Saturating::saturating_sub(u16::MAX, u16::MAX), 0);
        debug_assert_eq!(Saturating::saturating_sub(u16::MAX, rug_fuzz_6), u16::MAX);
        let _rug_ed_tests_llm_16_1547_llm_16_1547_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1548_llm_16_1548 {
    use crate::SaturatingAdd;
    #[test]
    fn saturating_add_test() {
        let _rug_st_tests_llm_16_1548_llm_16_1548_rrrruuuugggg_saturating_add_test = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        debug_assert_eq!(u16::saturating_add(rug_fuzz_0, rug_fuzz_1), 200);
        debug_assert_eq!(u16::saturating_add(u16::MAX, rug_fuzz_2), u16::MAX);
        debug_assert_eq!(u16::saturating_add(rug_fuzz_3, rug_fuzz_4), 0);
        debug_assert_eq!(u16::saturating_add(u16::MAX, u16::MAX), u16::MAX);
        let _rug_ed_tests_llm_16_1548_llm_16_1548_rrrruuuugggg_saturating_add_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1549_llm_16_1549 {
    #[test]
    fn saturating_mul_test() {
        let _rug_st_tests_llm_16_1549_llm_16_1549_rrrruuuugggg_saturating_mul_test = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 1000;
        let rug_fuzz_3 = 1000;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 65535;
        let rug_fuzz_6 = 65535;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 65535;
        let rug_fuzz_9 = 65535;
        debug_assert_eq!(u16::saturating_mul(rug_fuzz_0, rug_fuzz_1), 10000);
        debug_assert_eq!(u16::saturating_mul(rug_fuzz_2, rug_fuzz_3), 65535);
        debug_assert_eq!(u16::saturating_mul(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(u16::saturating_mul(rug_fuzz_6, rug_fuzz_7), 65535);
        debug_assert_eq!(u16::saturating_mul(rug_fuzz_8, rug_fuzz_9), 65535);
        let _rug_ed_tests_llm_16_1549_llm_16_1549_rrrruuuugggg_saturating_mul_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1550 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn saturating_sub_u16() {
        let _rug_st_tests_llm_16_1550_rrrruuuugggg_saturating_sub_u16 = 0;
        let rug_fuzz_0 = 5u16;
        let rug_fuzz_1 = 3u16;
        let rug_fuzz_2 = 0u16;
        let rug_fuzz_3 = 1u16;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1u16;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0);
        debug_assert_eq!(u16::MAX.saturating_sub(u16::MAX), 0);
        debug_assert_eq!(u16::MAX.saturating_sub(rug_fuzz_4), u16::MAX);
        debug_assert_eq!(rug_fuzz_5.saturating_sub(u16::MAX), 0);
        let _rug_ed_tests_llm_16_1550_rrrruuuugggg_saturating_sub_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1651 {
    use super::*;
    use crate::*;
    #[test]
    fn saturating_add_test() {
        let _rug_st_tests_llm_16_1651_rrrruuuugggg_saturating_add_test = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        debug_assert_eq!(
            < u32 as ops::saturating::Saturating > ::saturating_add(u32::MAX,
            rug_fuzz_0), u32::MAX
        );
        debug_assert_eq!(
            < u32 as ops::saturating::Saturating > ::saturating_add(u32::MAX,
            rug_fuzz_1), u32::MAX
        );
        debug_assert_eq!(
            < u32 as ops::saturating::Saturating > ::saturating_add(rug_fuzz_2,
            rug_fuzz_3), 0
        );
        debug_assert_eq!(
            < u32 as ops::saturating::Saturating > ::saturating_add(rug_fuzz_4,
            rug_fuzz_5), 3
        );
        debug_assert_eq!(
            < u32 as ops::saturating::Saturating > ::saturating_add(u32::MAX -
            rug_fuzz_6, rug_fuzz_7), u32::MAX
        );
        let _rug_ed_tests_llm_16_1651_rrrruuuugggg_saturating_add_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1652 {
    use super::*;
    use crate::*;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1652_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5u32;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0u32;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 3u32;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1u32;
        let rug_fuzz_7 = 0u32;
        let rug_fuzz_8 = 0;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0);
        debug_assert_eq!(rug_fuzz_4.saturating_sub(rug_fuzz_5), 1);
        debug_assert_eq!(rug_fuzz_6.saturating_sub(u32::MAX), 0);
        debug_assert_eq!(rug_fuzz_7.saturating_sub(rug_fuzz_8), 0);
        debug_assert_eq!(u32::MAX.saturating_sub(u32::MAX), 0);
        let _rug_ed_tests_llm_16_1652_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1653_llm_16_1653 {
    use crate::SaturatingAdd;
    use std::u32;
    #[test]
    fn test_u32_saturating_add() {
        let _rug_st_tests_llm_16_1653_llm_16_1653_rrrruuuugggg_test_u32_saturating_add = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 123;
        let rug_fuzz_9 = 456;
        debug_assert_eq!(u32::saturating_add(rug_fuzz_0, rug_fuzz_1), 0);
        debug_assert_eq!(u32::saturating_add(u32::MAX, rug_fuzz_2), u32::MAX);
        debug_assert_eq!(u32::saturating_add(rug_fuzz_3, u32::MAX), u32::MAX);
        debug_assert_eq!(u32::saturating_add(u32::MAX, rug_fuzz_4), u32::MAX);
        debug_assert_eq!(u32::saturating_add(rug_fuzz_5, u32::MAX), u32::MAX);
        debug_assert_eq!(
            u32::saturating_add(u32::MAX / rug_fuzz_6, u32::MAX / rug_fuzz_7), u32::MAX -
            1
        );
        debug_assert_eq!(u32::saturating_add(rug_fuzz_8, rug_fuzz_9), 579);
        let _rug_ed_tests_llm_16_1653_llm_16_1653_rrrruuuugggg_test_u32_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1654_llm_16_1654 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_saturating_mul() {
        let _rug_st_tests_llm_16_1654_llm_16_1654_rrrruuuugggg_test_saturating_mul = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0;
        debug_assert_eq!(
            < u32 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 50
        );
        debug_assert_eq!(
            < u32 as SaturatingMul > ::saturating_mul(& u32::MAX, & rug_fuzz_2), u32::MAX
        );
        debug_assert_eq!(
            < u32 as SaturatingMul > ::saturating_mul(& rug_fuzz_3, & u32::MAX), 0
        );
        let _rug_ed_tests_llm_16_1654_llm_16_1654_rrrruuuugggg_test_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1655_llm_16_1655 {
    use crate::SaturatingSub;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1655_llm_16_1655_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 100;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        debug_assert_eq!(u32::saturating_sub(rug_fuzz_0, rug_fuzz_1), 90);
        debug_assert_eq!(u32::saturating_sub(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(u32::saturating_sub(u32::MAX, rug_fuzz_4), u32::MAX - 1);
        debug_assert_eq!(u32::saturating_sub(rug_fuzz_5, rug_fuzz_6), 0);
        let _rug_ed_tests_llm_16_1655_llm_16_1655_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1756_llm_16_1756 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1756_llm_16_1756_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_add(rug_fuzz_0, rug_fuzz_1), 5
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_add(u64::MAX, rug_fuzz_2), u64::MAX
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_add(u64::MAX - rug_fuzz_3, rug_fuzz_4),
            u64::MAX
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_add(u64::MAX, rug_fuzz_5), u64::MAX
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_add(rug_fuzz_6, u64::MAX), u64::MAX
        );
        let _rug_ed_tests_llm_16_1756_llm_16_1756_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1757_llm_16_1757 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1757_llm_16_1757_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_sub(rug_fuzz_0, rug_fuzz_1), 2
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_sub(rug_fuzz_2, rug_fuzz_3), 0
        );
        debug_assert_eq!(< u64 as Saturating > ::saturating_sub(u64::MAX, u64::MAX), 0);
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_sub(u64::MAX, rug_fuzz_4), u64::MAX
        );
        debug_assert_eq!(
            < u64 as Saturating > ::saturating_sub(rug_fuzz_5, u64::MAX), 0
        );
        let _rug_ed_tests_llm_16_1757_llm_16_1757_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1759_llm_16_1759 {
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn saturating_mul_test() {
        let _rug_st_tests_llm_16_1759_llm_16_1759_rrrruuuugggg_saturating_mul_test = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0;
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 4
        );
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& u64::MAX, & rug_fuzz_2), u64::MAX
        );
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& u64::MAX, & u64::MAX), u64::MAX
        );
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& rug_fuzz_3, & u64::MAX), u64::MAX
        );
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& rug_fuzz_4, & u64::MAX), 0
        );
        debug_assert_eq!(
            < u64 as SaturatingMul > ::saturating_mul(& rug_fuzz_5, & rug_fuzz_6), 0
        );
        let _rug_ed_tests_llm_16_1759_llm_16_1759_rrrruuuugggg_saturating_mul_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1760_llm_16_1760 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1760_llm_16_1760_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5u64;
        let rug_fuzz_1 = 2u64;
        let rug_fuzz_2 = 0u64;
        let rug_fuzz_3 = 3u64;
        let rug_fuzz_4 = 1u64;
        let rug_fuzz_5 = 1u64;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 3u64);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0u64);
        debug_assert_eq!(u64::MAX.saturating_sub(rug_fuzz_4), u64::MAX - 1);
        debug_assert_eq!(rug_fuzz_5.saturating_sub(u64::MAX), 0u64);
        let _rug_ed_tests_llm_16_1760_llm_16_1760_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1862_llm_16_1862 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1862_llm_16_1862_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 155;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 255;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 254;
        let rug_fuzz_9 = 2;
        debug_assert_eq!(
            < u8 as Saturating > ::saturating_add(rug_fuzz_0, rug_fuzz_1), 200
        );
        debug_assert_eq!(
            < u8 as Saturating > ::saturating_add(rug_fuzz_2, rug_fuzz_3), 255
        );
        debug_assert_eq!(
            < u8 as Saturating > ::saturating_add(rug_fuzz_4, rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < u8 as Saturating > ::saturating_add(rug_fuzz_6, rug_fuzz_7), 255
        );
        debug_assert_eq!(
            < u8 as Saturating > ::saturating_add(rug_fuzz_8, rug_fuzz_9), 255
        );
        let _rug_ed_tests_llm_16_1862_llm_16_1862_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1863_llm_16_1863 {
    use crate::ops::saturating::Saturating;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1863_llm_16_1863_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 100u8;
        let rug_fuzz_1 = 50;
        let rug_fuzz_2 = 0u8;
        let rug_fuzz_3 = 50;
        let rug_fuzz_4 = 50u8;
        let rug_fuzz_5 = 100;
        let rug_fuzz_6 = 0u8;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 255u8;
        let rug_fuzz_9 = 255;
        let rug_fuzz_10 = 255u8;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1u8;
        let rug_fuzz_13 = 255;
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_0, rug_fuzz_1), 50);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_2, rug_fuzz_3), 0);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_4, rug_fuzz_5), 0);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_8, rug_fuzz_9), 0);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_10, rug_fuzz_11), 254);
        debug_assert_eq!(Saturating::saturating_sub(rug_fuzz_12, rug_fuzz_13), 0);
        let _rug_ed_tests_llm_16_1863_llm_16_1863_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1864_llm_16_1864 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn test_u8_saturating_add() {
        let _rug_st_tests_llm_16_1864_llm_16_1864_rrrruuuugggg_test_u8_saturating_add = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 200;
        let rug_fuzz_3 = 100;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 255;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& rug_fuzz_0, & rug_fuzz_1), 200
        );
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& rug_fuzz_2, & rug_fuzz_3), 255
        );
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& rug_fuzz_4, & rug_fuzz_5), 0
        );
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& rug_fuzz_6, & rug_fuzz_7), 255
        );
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& u8::MAX, & rug_fuzz_8), u8::MAX
        );
        debug_assert_eq!(
            < u8 as SaturatingAdd > ::saturating_add(& rug_fuzz_9, & u8::MAX), u8::MAX
        );
        let _rug_ed_tests_llm_16_1864_llm_16_1864_rrrruuuugggg_test_u8_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1866_llm_16_1866 {
    use crate::ops::saturating::SaturatingSub;
    #[test]
    fn test_saturating_sub() {
        let _rug_st_tests_llm_16_1866_llm_16_1866_rrrruuuugggg_test_saturating_sub = 0;
        let rug_fuzz_0 = 5u8;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0u8;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1u8;
        let rug_fuzz_6 = 2;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0);
        debug_assert_eq!(std::u8::MAX.saturating_sub(std::u8::MAX), 0);
        debug_assert_eq!(std::u8::MAX.saturating_sub(rug_fuzz_4), std::u8::MAX);
        debug_assert_eq!(rug_fuzz_5.saturating_sub(rug_fuzz_6), 0);
        let _rug_ed_tests_llm_16_1866_llm_16_1866_rrrruuuugggg_test_saturating_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1967_llm_16_1967 {
    use crate::ops::saturating::Saturating;
    use core::ops::Add;
    #[test]
    fn test_saturating_add() {
        let _rug_st_tests_llm_16_1967_llm_16_1967_rrrruuuugggg_test_saturating_add = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2;
        debug_assert_eq!(usize::saturating_add(rug_fuzz_0, rug_fuzz_1), 0);
        debug_assert_eq!(usize::saturating_add(usize::MAX, rug_fuzz_2), usize::MAX);
        debug_assert_eq!(usize::saturating_add(rug_fuzz_3, usize::MAX), usize::MAX);
        debug_assert_eq!(usize::saturating_add(usize::MAX, rug_fuzz_4), usize::MAX);
        debug_assert_eq!(
            usize::saturating_add(rug_fuzz_5, usize::MAX - rug_fuzz_6), usize::MAX
        );
        debug_assert_eq!(
            usize::saturating_add(usize::MAX / rug_fuzz_7, usize::MAX / rug_fuzz_8),
            usize::MAX - 1
        );
        let _rug_ed_tests_llm_16_1967_llm_16_1967_rrrruuuugggg_test_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1968_llm_16_1968 {
    use super::*;
    use crate::*;
    #[test]
    fn saturating_sub_test() {
        let _rug_st_tests_llm_16_1968_llm_16_1968_rrrruuuugggg_saturating_sub_test = 0;
        let rug_fuzz_0 = 5usize;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0usize;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1usize;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2);
        debug_assert_eq!(rug_fuzz_2.saturating_sub(rug_fuzz_3), 0);
        debug_assert_eq!(usize::MAX.saturating_sub(rug_fuzz_4), usize::MAX - 1);
        debug_assert_eq!(rug_fuzz_5.saturating_sub(usize::MAX), 0);
        let _rug_ed_tests_llm_16_1968_llm_16_1968_rrrruuuugggg_saturating_sub_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1969_llm_16_1969 {
    use crate::ops::saturating::SaturatingAdd;
    #[test]
    fn saturating_add() {
        let _rug_st_tests_llm_16_1969_llm_16_1969_rrrruuuugggg_saturating_add = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < usize as SaturatingAdd > ::saturating_add(& rug_fuzz_0, & rug_fuzz_1), 10
        );
        debug_assert_eq!(
            < usize as SaturatingAdd > ::saturating_add(& usize::MAX, & rug_fuzz_2),
            usize::MAX
        );
        let _rug_ed_tests_llm_16_1969_llm_16_1969_rrrruuuugggg_saturating_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1970_llm_16_1970 {
    use super::*;
    use crate::*;
    use crate::ops::saturating::SaturatingMul;
    #[test]
    fn test_saturating_mul() {
        let _rug_st_tests_llm_16_1970_llm_16_1970_rrrruuuugggg_test_saturating_mul = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 2;
        debug_assert_eq!(SaturatingMul::saturating_mul(& rug_fuzz_0, & rug_fuzz_1), 10);
        debug_assert_eq!(
            SaturatingMul::saturating_mul(& usize::MAX, & rug_fuzz_2), usize::MAX
        );
        debug_assert_eq!(
            SaturatingMul::saturating_mul(& rug_fuzz_3, & usize::MAX), usize::MAX
        );
        debug_assert_eq!(
            SaturatingMul::saturating_mul(& usize::MAX, & usize::MAX), usize::MAX
        );
        let _rug_ed_tests_llm_16_1970_llm_16_1970_rrrruuuugggg_test_saturating_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1971_llm_16_1971 {
    use crate::SaturatingSub;
    #[test]
    fn saturating_sub_with_no_overflow() {
        let _rug_st_tests_llm_16_1971_llm_16_1971_rrrruuuugggg_saturating_sub_with_no_overflow = 0;
        let rug_fuzz_0 = 5usize;
        let rug_fuzz_1 = 3usize;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 2usize);
        let _rug_ed_tests_llm_16_1971_llm_16_1971_rrrruuuugggg_saturating_sub_with_no_overflow = 0;
    }
    #[test]
    fn saturating_sub_with_underflow() {
        let _rug_st_tests_llm_16_1971_llm_16_1971_rrrruuuugggg_saturating_sub_with_underflow = 0;
        let rug_fuzz_0 = 0usize;
        let rug_fuzz_1 = 3usize;
        debug_assert_eq!(rug_fuzz_0.saturating_sub(rug_fuzz_1), 0usize);
        let _rug_ed_tests_llm_16_1971_llm_16_1971_rrrruuuugggg_saturating_sub_with_underflow = 0;
    }
}
