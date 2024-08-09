use core::ops::{Add, Mul, Sub};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};
macro_rules! overflowing_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, v : & Self) -> (Self,
        bool) { <$t >::$method (* self, * v) } }
    };
}
/// Performs addition with a flag for overflow.
pub trait OverflowingAdd: Sized + Add<Self, Output = Self> {
    /// Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_add(&self, v: &Self) -> (Self, bool);
}
overflowing_impl!(OverflowingAdd, overflowing_add, u8);
overflowing_impl!(OverflowingAdd, overflowing_add, u16);
overflowing_impl!(OverflowingAdd, overflowing_add, u32);
overflowing_impl!(OverflowingAdd, overflowing_add, u64);
overflowing_impl!(OverflowingAdd, overflowing_add, usize);
overflowing_impl!(OverflowingAdd, overflowing_add, u128);
overflowing_impl!(OverflowingAdd, overflowing_add, i8);
overflowing_impl!(OverflowingAdd, overflowing_add, i16);
overflowing_impl!(OverflowingAdd, overflowing_add, i32);
overflowing_impl!(OverflowingAdd, overflowing_add, i64);
overflowing_impl!(OverflowingAdd, overflowing_add, isize);
overflowing_impl!(OverflowingAdd, overflowing_add, i128);
/// Performs substraction with a flag for overflow.
pub trait OverflowingSub: Sized + Sub<Self, Output = Self> {
    /// Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_sub(&self, v: &Self) -> (Self, bool);
}
overflowing_impl!(OverflowingSub, overflowing_sub, u8);
overflowing_impl!(OverflowingSub, overflowing_sub, u16);
overflowing_impl!(OverflowingSub, overflowing_sub, u32);
overflowing_impl!(OverflowingSub, overflowing_sub, u64);
overflowing_impl!(OverflowingSub, overflowing_sub, usize);
overflowing_impl!(OverflowingSub, overflowing_sub, u128);
overflowing_impl!(OverflowingSub, overflowing_sub, i8);
overflowing_impl!(OverflowingSub, overflowing_sub, i16);
overflowing_impl!(OverflowingSub, overflowing_sub, i32);
overflowing_impl!(OverflowingSub, overflowing_sub, i64);
overflowing_impl!(OverflowingSub, overflowing_sub, isize);
overflowing_impl!(OverflowingSub, overflowing_sub, i128);
/// Performs multiplication with a flag for overflow.
pub trait OverflowingMul: Sized + Mul<Self, Output = Self> {
    /// Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow would occur.
    /// If an overflow would have occurred then the wrapped value is returned.
    fn overflowing_mul(&self, v: &Self) -> (Self, bool);
}
overflowing_impl!(OverflowingMul, overflowing_mul, u8);
overflowing_impl!(OverflowingMul, overflowing_mul, u16);
overflowing_impl!(OverflowingMul, overflowing_mul, u32);
overflowing_impl!(OverflowingMul, overflowing_mul, u64);
overflowing_impl!(OverflowingMul, overflowing_mul, usize);
overflowing_impl!(OverflowingMul, overflowing_mul, u128);
overflowing_impl!(OverflowingMul, overflowing_mul, i8);
overflowing_impl!(OverflowingMul, overflowing_mul, i16);
overflowing_impl!(OverflowingMul, overflowing_mul, i32);
overflowing_impl!(OverflowingMul, overflowing_mul, i64);
overflowing_impl!(OverflowingMul, overflowing_mul, isize);
overflowing_impl!(OverflowingMul, overflowing_mul, i128);
#[test]
fn test_overflowing_traits() {
    fn overflowing_add<T: OverflowingAdd>(a: T, b: T) -> (T, bool) {
        a.overflowing_add(&b)
    }
    fn overflowing_sub<T: OverflowingSub>(a: T, b: T) -> (T, bool) {
        a.overflowing_sub(&b)
    }
    fn overflowing_mul<T: OverflowingMul>(a: T, b: T) -> (T, bool) {
        a.overflowing_mul(&b)
    }
    assert_eq!(overflowing_add(5i16, 2), (7, false));
    assert_eq!(overflowing_add(i16::MAX, 1), (i16::MIN, true));
    assert_eq!(overflowing_sub(5i16, 2), (3, false));
    assert_eq!(overflowing_sub(i16::MIN, 1), (i16::MAX, true));
    assert_eq!(overflowing_mul(5i16, 2), (10, false));
    assert_eq!(overflowing_mul(1_000_000_000i32, 10), (1410065408, true));
}
#[cfg(test)]
mod tests_llm_16_684_llm_16_684 {
    use super::*;
    use crate::*;
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_684_llm_16_684_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(
            (< i128 as OverflowingSub > ::overflowing_sub(& rug_fuzz_0, & rug_fuzz_1)),
            (0, false)
        );
        debug_assert_eq!(
            (< i128 as OverflowingSub > ::overflowing_sub(& i128::MAX, & rug_fuzz_2)),
            (i128::MAX - 1, false)
        );
        debug_assert_eq!(
            (< i128 as OverflowingSub > ::overflowing_sub(& i128::MIN, & rug_fuzz_3)),
            (i128::MAX, true)
        );
        debug_assert_eq!(
            (< i128 as OverflowingSub > ::overflowing_sub(& rug_fuzz_4, & i128::MAX)),
            (i128::MIN + 1, true)
        );
        debug_assert_eq!(
            (< i128 as OverflowingSub > ::overflowing_sub(& (- rug_fuzz_5), &
            i128::MIN)), (0, true)
        );
        let _rug_ed_tests_llm_16_684_llm_16_684_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_794_llm_16_794 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 1000;
        let rug_fuzz_1 = 2000;
        let rug_fuzz_2 = 1000;
        let rug_fuzz_3 = 500;
        let rug_fuzz_4 = 1;
        let a: i16 = rug_fuzz_0;
        let b: i16 = rug_fuzz_1;
        let result = <i16 as OverflowingSub>::overflowing_sub(&a, &b);
        debug_assert_eq!(result, (1000i16.wrapping_sub(2000), true));
        let a: i16 = rug_fuzz_2;
        let b: i16 = rug_fuzz_3;
        let result = <i16 as OverflowingSub>::overflowing_sub(&a, &b);
        debug_assert_eq!(result, (500, false));
        let a: i16 = i16::MIN;
        let b: i16 = rug_fuzz_4;
        let result = <i16 as OverflowingSub>::overflowing_sub(&a, &b);
        debug_assert_eq!(result, (i16::MAX, true));
        let _rug_ed_tests_llm_16_794_llm_16_794_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_902_llm_16_902 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_llm_16_902_llm_16_902_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        debug_assert_eq!(
            < i32 as OverflowingAdd > ::overflowing_add(& rug_fuzz_0, & rug_fuzz_1), (4,
            false)
        );
        debug_assert_eq!(
            < i32 as OverflowingAdd > ::overflowing_add(& i32::MAX, & rug_fuzz_2),
            (i32::MIN, true)
        );
        debug_assert_eq!(
            < i32 as OverflowingAdd > ::overflowing_add(& i32::MIN, & - rug_fuzz_3),
            (i32::MAX, true)
        );
        debug_assert_eq!(
            < i32 as OverflowingAdd > ::overflowing_add(& rug_fuzz_4, & rug_fuzz_5), (0,
            false)
        );
        debug_assert_eq!(
            < i32 as OverflowingAdd > ::overflowing_add(& - rug_fuzz_6, & rug_fuzz_7),
            (0, false)
        );
        let _rug_ed_tests_llm_16_902_llm_16_902_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_903_llm_16_903 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_903_llm_16_903_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 6;
        let rug_fuzz_1 = 7;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let (result, overflow) = i32::overflowing_mul(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result, 42);
        debug_assert!(! overflow);
        let (result, overflow) = i32::overflowing_mul(i32::MAX, rug_fuzz_2);
        debug_assert_eq!(result, i32::MAX.wrapping_mul(2));
        debug_assert!(overflow);
        let (result, overflow) = i32::overflowing_mul(-rug_fuzz_3, rug_fuzz_4);
        debug_assert_eq!(result, - 6);
        debug_assert!(! overflow);
        let (result, overflow) = i32::overflowing_mul(rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(result, 0);
        debug_assert!(! overflow);
        let _rug_ed_tests_llm_16_903_llm_16_903_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_904_llm_16_904 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_904_llm_16_904_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 100i32;
        let rug_fuzz_1 = 200i32;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let (result, overflow) = <i32 as OverflowingSub>::overflowing_sub(&a, &b);
        debug_assert_eq!(result, - 100);
        debug_assert_eq!(overflow, false);
        let (result, overflow) = <i32 as OverflowingSub>::overflowing_sub(
            &i32::MIN,
            &rug_fuzz_2,
        );
        debug_assert_eq!(result, i32::MAX);
        debug_assert_eq!(overflow, true);
        let (result, overflow) = <i32 as OverflowingSub>::overflowing_sub(
            &rug_fuzz_3,
            &rug_fuzz_4,
        );
        debug_assert_eq!(result, 0);
        debug_assert_eq!(overflow, false);
        let (result, overflow) = <i32 as OverflowingSub>::overflowing_sub(
            &i32::MAX,
            &(-rug_fuzz_5),
        );
        debug_assert_eq!(result, i32::MIN);
        debug_assert_eq!(overflow, true);
        let _rug_ed_tests_llm_16_904_llm_16_904_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1012_llm_16_1012 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn overflowing_add_test() {
        let _rug_st_tests_llm_16_1012_llm_16_1012_rrrruuuugggg_overflowing_add_test = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let a: i64 = i64::MAX;
        let b: i64 = rug_fuzz_0;
        let result = <i64 as OverflowingAdd>::overflowing_add(&a, &b);
        debug_assert_eq!(result, (i64::MIN, true));
        let a: i64 = i64::MAX;
        let b: i64 = rug_fuzz_1;
        let result = <i64 as OverflowingAdd>::overflowing_add(&a, &b);
        debug_assert_eq!(result, (i64::MAX, false));
        let a: i64 = -rug_fuzz_2;
        let b: i64 = -rug_fuzz_3;
        let result = <i64 as OverflowingAdd>::overflowing_add(&a, &b);
        debug_assert_eq!(result, (- 2, false));
        let a: i64 = -rug_fuzz_4;
        let b: i64 = i64::MIN;
        let result = <i64 as OverflowingAdd>::overflowing_add(&a, &b);
        debug_assert_eq!(result, (i64::MAX, true));
        let _rug_ed_tests_llm_16_1012_llm_16_1012_rrrruuuugggg_overflowing_add_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1232 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_llm_16_1232_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = false;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = true;
        let rug_fuzz_5 = true;
        debug_assert_eq!((isize::MAX, rug_fuzz_0), isize::MAX.overflowing_add(0));
        debug_assert_eq!((isize::MIN, rug_fuzz_1), isize::MIN.overflowing_add(0));
        debug_assert_eq!((rug_fuzz_2, rug_fuzz_3), 0isize.overflowing_add(0));
        debug_assert_eq!((isize::MIN, rug_fuzz_4), isize::MAX.overflowing_add(1));
        debug_assert_eq!((isize::MAX, rug_fuzz_5), isize::MIN.overflowing_add(- 1));
        let _rug_ed_tests_llm_16_1232_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1234_llm_16_1234 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_1234_llm_16_1234_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 5isize;
        let rug_fuzz_1 = 0isize;
        let rug_fuzz_2 = 1isize;
        let rug_fuzz_3 = 1isize;
        let rug_fuzz_4 = 0isize;
        let rug_fuzz_5 = 0isize;
        debug_assert_eq!((rug_fuzz_0).overflowing_sub(rug_fuzz_1), (5isize, false));
        debug_assert_eq!(
            (isize::MAX).overflowing_sub(rug_fuzz_2), (isize::MAX - 1, false)
        );
        debug_assert_eq!((isize::MIN).overflowing_sub(rug_fuzz_3), (isize::MAX, true));
        debug_assert_eq!((rug_fuzz_4).overflowing_sub(isize::MAX), (isize::MIN, true));
        debug_assert_eq!(
            (rug_fuzz_5).overflowing_sub(isize::MIN), (isize::MIN.wrapping_sub(1), true)
        );
        let _rug_ed_tests_llm_16_1234_llm_16_1234_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1438_llm_16_1438 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add_u128() {
        let _rug_st_tests_llm_16_1438_llm_16_1438_rrrruuuugggg_test_overflowing_add_u128 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(u128::overflowing_add(u128::MAX, rug_fuzz_0), (0, true));
        debug_assert_eq!(u128::overflowing_add(rug_fuzz_1, rug_fuzz_2), (0, false));
        debug_assert_eq!(
            u128::overflowing_add(u128::MAX, rug_fuzz_3), (u128::MAX, false)
        );
        debug_assert_eq!(
            u128::overflowing_add(rug_fuzz_4, u128::MAX - rug_fuzz_5), (u128::MAX, false)
        );
        let _rug_ed_tests_llm_16_1438_llm_16_1438_rrrruuuugggg_test_overflowing_add_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1440_llm_16_1440 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_1440_llm_16_1440_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 200;
        let rug_fuzz_2 = 12345;
        let rug_fuzz_3 = 12340;
        let a: u128 = rug_fuzz_0;
        let b: u128 = rug_fuzz_1;
        let (result, overflow) = a.overflowing_sub(b);
        debug_assert_eq!(result, u128::MAX - 99);
        debug_assert!(overflow);
        let c: u128 = rug_fuzz_2;
        let d: u128 = rug_fuzz_3;
        let (result, overflow) = c.overflowing_sub(d);
        debug_assert_eq!(result, 5);
        debug_assert!(! overflow);
        let _rug_ed_tests_llm_16_1440_llm_16_1440_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1543_llm_16_1543 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_llm_16_1543_llm_16_1543_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < u16 as OverflowingAdd > ::overflowing_add(& rug_fuzz_0, & rug_fuzz_1), (5,
            false)
        );
        debug_assert_eq!(
            < u16 as OverflowingAdd > ::overflowing_add(& u16::MAX, & rug_fuzz_2), (0,
            true)
        );
        let _rug_ed_tests_llm_16_1543_llm_16_1543_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1544_llm_16_1544 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_1544_llm_16_1544_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 65535;
        let rug_fuzz_3 = 2;
        debug_assert_eq!(
            < u16 as OverflowingMul > ::overflowing_mul(& rug_fuzz_0, & rug_fuzz_1), (6,
            false)
        );
        debug_assert_eq!(
            < u16 as OverflowingMul > ::overflowing_mul(& rug_fuzz_2, & rug_fuzz_3),
            (65534, true)
        );
        let _rug_ed_tests_llm_16_1544_llm_16_1544_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1649_llm_16_1649 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_1649_llm_16_1649_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let a: u32 = u32::MAX;
        let b: u32 = rug_fuzz_0;
        let c: u32 = rug_fuzz_1;
        debug_assert_eq!(a.overflowing_mul(b), (u32::MAX, false));
        debug_assert_eq!(a.overflowing_mul(c), (u32::MAX.wrapping_mul(c), true));
        let _rug_ed_tests_llm_16_1649_llm_16_1649_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1753_llm_16_1753 {
    use super::*;
    use crate::*;
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_llm_16_1753_llm_16_1753_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(
            < u64 as OverflowingAdd > ::overflowing_add(& rug_fuzz_0, & rug_fuzz_1), (0,
            false)
        );
        debug_assert_eq!(
            < u64 as OverflowingAdd > ::overflowing_add(& u64::MAX, & rug_fuzz_2), (0,
            true)
        );
        debug_assert_eq!(
            < u64 as OverflowingAdd > ::overflowing_add(& (u64::MAX - rug_fuzz_3), &
            rug_fuzz_4), (u64::MAX, false)
        );
        let _rug_ed_tests_llm_16_1753_llm_16_1753_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1754_llm_16_1754 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_1754_llm_16_1754_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 3u64;
        let rug_fuzz_2 = 2u64;
        debug_assert_eq!(
            < u64 as OverflowingMul > ::overflowing_mul(& rug_fuzz_0, & rug_fuzz_1),
            (6u64, false)
        );
        debug_assert_eq!(
            < u64 as OverflowingMul > ::overflowing_mul(& u64::MAX, & rug_fuzz_2),
            (u64::MAX.wrapping_mul(2), true)
        );
        let _rug_ed_tests_llm_16_1754_llm_16_1754_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1755_llm_16_1755 {
    use super::*;
    use crate::*;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_1755_llm_16_1755_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert_eq!(u64::overflowing_sub(rug_fuzz_0, rug_fuzz_1), (2, false));
        debug_assert_eq!(u64::overflowing_sub(rug_fuzz_2, rug_fuzz_3), (u64::MAX, true));
        debug_assert_eq!(u64::overflowing_sub(u64::MAX, rug_fuzz_4), (u64::MAX, false));
        debug_assert_eq!(u64::overflowing_sub(rug_fuzz_5, u64::MAX), (1, true));
        let _rug_ed_tests_llm_16_1755_llm_16_1755_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1859_llm_16_1859 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn u8_overflowing_add() {
        let _rug_st_tests_llm_16_1859_llm_16_1859_rrrruuuugggg_u8_overflowing_add = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 27;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 157;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 255;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 255;
        let rug_fuzz_9 = 255;
        debug_assert_eq!(
            < u8 as OverflowingAdd > ::overflowing_add(& rug_fuzz_0, & rug_fuzz_1), (127,
            false)
        );
        debug_assert_eq!(
            < u8 as OverflowingAdd > ::overflowing_add(& rug_fuzz_2, & rug_fuzz_3), (1,
            true)
        );
        debug_assert_eq!(
            < u8 as OverflowingAdd > ::overflowing_add(& rug_fuzz_4, & rug_fuzz_5), (0,
            false)
        );
        debug_assert_eq!(
            < u8 as OverflowingAdd > ::overflowing_add(& rug_fuzz_6, & rug_fuzz_7), (0,
            true)
        );
        debug_assert_eq!(
            < u8 as OverflowingAdd > ::overflowing_add(& rug_fuzz_8, & rug_fuzz_9), (254,
            true)
        );
        let _rug_ed_tests_llm_16_1859_llm_16_1859_rrrruuuugggg_u8_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1860_llm_16_1860 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_1860_llm_16_1860_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 100;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 255;
        let rug_fuzz_7 = 2;
        debug_assert_eq!(
            < u8 as OverflowingMul > ::overflowing_mul(& rug_fuzz_0, & rug_fuzz_1), (6,
            false)
        );
        debug_assert_eq!(
            < u8 as OverflowingMul > ::overflowing_mul(& rug_fuzz_2, & rug_fuzz_3), (44,
            true)
        );
        debug_assert_eq!(
            < u8 as OverflowingMul > ::overflowing_mul(& rug_fuzz_4, & rug_fuzz_5), (0,
            false)
        );
        debug_assert_eq!(
            < u8 as OverflowingMul > ::overflowing_mul(& rug_fuzz_6, & rug_fuzz_7), (254,
            true)
        );
        let _rug_ed_tests_llm_16_1860_llm_16_1860_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1965_llm_16_1965 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_llm_16_1965_llm_16_1965_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2;
        let (result, overflow) = <usize as OverflowingMul>::overflowing_mul(
            &rug_fuzz_0,
            &rug_fuzz_1,
        );
        debug_assert_eq!(result, 6);
        debug_assert!(! overflow);
        let (result, overflow) = <usize as OverflowingMul>::overflowing_mul(
            &usize::MAX,
            &rug_fuzz_2,
        );
        debug_assert!(overflow);
        let _rug_ed_tests_llm_16_1965_llm_16_1965_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1966 {
    use super::*;
    use crate::*;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_llm_16_1966_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 5usize;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0usize;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(rug_fuzz_0.overflowing_sub(rug_fuzz_1), (2, false));
        debug_assert_eq!(rug_fuzz_2.overflowing_sub(rug_fuzz_3), (usize::MAX, true));
        let _rug_ed_tests_llm_16_1966_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_178 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_178_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0xffffffff;
        let rug_fuzz_1 = 1;
        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(
            < u32 as OverflowingAdd > ::overflowing_add(& p0, & p1), (0, true)
        );
        let _rug_ed_tests_rug_178_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_179 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_rug_179_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 1;
        let p0: usize = usize::MAX;
        let p1: usize = rug_fuzz_0;
        let (result, overflowed) = <usize as OverflowingAdd>::overflowing_add(&p0, &p1);
        debug_assert_eq!(result, 0);
        debug_assert!(overflowed);
        let _rug_ed_tests_rug_179_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_180 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_rug_180_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 120;
        let rug_fuzz_1 = 10;
        let mut p0: i8 = rug_fuzz_0;
        let mut p1: i8 = rug_fuzz_1;
        debug_assert_eq!(
            < i8 as OverflowingAdd > ::overflowing_add(& p0, & p1), (- 126, true)
        );
        let _rug_ed_tests_rug_180_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_rug_181_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 1;
        let mut p0: i16 = i16::MAX;
        let mut p1: i16 = rug_fuzz_0;
        debug_assert_eq!(
            < i16 as OverflowingAdd > ::overflowing_add(& p0, & p1), (i16::MIN, true)
        );
        let _rug_ed_tests_rug_181_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use crate::ops::overflowing::OverflowingAdd;
    #[test]
    fn test_overflowing_add() {
        let _rug_st_tests_rug_182_rrrruuuugggg_test_overflowing_add = 0;
        let rug_fuzz_0 = 1;
        let p0: i128 = i128::MAX;
        let p1: i128 = rug_fuzz_0;
        let (result, has_overflowed) = <i128 as OverflowingAdd>::overflowing_add(
            &p0,
            &p1,
        );
        debug_assert_eq!(result, i128::MIN);
        debug_assert!(has_overflowed);
        let _rug_ed_tests_rug_182_rrrruuuugggg_test_overflowing_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_rug_183_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 200;
        let p0: u8 = rug_fuzz_0;
        let p1: u8 = rug_fuzz_1;
        debug_assert_eq!(
            < u8 as OverflowingSub > ::overflowing_sub(& p0, & p1), (156, true)
        );
        let _rug_ed_tests_rug_183_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_184 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_rug_184_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 1000;
        let rug_fuzz_1 = 500;
        let mut p0: u16 = rug_fuzz_0;
        let mut p1: u16 = rug_fuzz_1;
        let (result, overflow) = <u16 as OverflowingSub>::overflowing_sub(&p0, &p1);
        debug_assert_eq!(result, 500);
        debug_assert!(! overflow);
        let _rug_ed_tests_rug_184_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_185 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_rug_185_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 200;
        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(
            < u32 as OverflowingSub > ::overflowing_sub(& p0, & p1), (p0
            .wrapping_sub(p1), true)
        );
        let _rug_ed_tests_rug_185_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_186 {
    use super::*;
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_rug_186_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 120;
        let rug_fuzz_1 = 100;
        let p0: i8 = rug_fuzz_0;
        let p1: i8 = rug_fuzz_1;
        debug_assert_eq!(
            < i8 as OverflowingSub > ::overflowing_sub(& p0, & p1), (20, false)
        );
        let _rug_ed_tests_rug_186_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_187 {
    use crate::ops::overflowing::OverflowingSub;
    #[test]
    fn test_overflowing_sub() {
        let _rug_st_tests_rug_187_rrrruuuugggg_test_overflowing_sub = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let p0: i64 = rug_fuzz_0;
        let p1: i64 = rug_fuzz_1;
        debug_assert_eq!(
            < i64 as OverflowingSub > ::overflowing_sub(& p0, & p1), (- 1, false)
        );
        let _rug_ed_tests_rug_187_rrrruuuugggg_test_overflowing_sub = 0;
    }
}
#[cfg(test)]
mod tests_rug_188 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_rug_188_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 340282366920938463463374607431768211455;
        let rug_fuzz_1 = 2;
        let mut p0: u128 = rug_fuzz_0;
        let mut p1: u128 = rug_fuzz_1;
        let (result, overflow) = <u128 as OverflowingMul>::overflowing_mul(&p0, &p1);
        debug_assert_eq!(result, 340282366920938463463374607431768211454);
        debug_assert!(overflow);
        let _rug_ed_tests_rug_188_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_rug_189_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 120;
        let rug_fuzz_1 = 3;
        let mut p0: i8 = rug_fuzz_0;
        let mut p1: i8 = rug_fuzz_1;
        debug_assert_eq!(
            < i8 as OverflowingMul > ::overflowing_mul(& p0, & p1), (- 80, true)
        );
        let _rug_ed_tests_rug_189_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_rug_190_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 32_767;
        let rug_fuzz_1 = 2;
        let mut p0: i16 = rug_fuzz_0;
        let mut p1: i16 = rug_fuzz_1;
        let result = <i16 as OverflowingMul>::overflowing_mul(&p0, &p1);
        debug_assert_eq!(result, (- 2, true));
        let _rug_ed_tests_rug_190_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_rug_191_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 2;
        let p0: i64 = rug_fuzz_0;
        let p1: i64 = i64::MAX;
        debug_assert_eq!(
            < i64 as OverflowingMul > ::overflowing_mul(& p0, & p1), (i64::MIN, true)
        );
        let _rug_ed_tests_rug_191_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_192_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1_000_000_000;
        let rug_fuzz_1 = 2;
        let mut p0: isize = rug_fuzz_0;
        let mut p1: isize = rug_fuzz_1;
        let result = <isize as OverflowingMul>::overflowing_mul(&p0, &p1);
        debug_assert_eq!(result, (2000000000, false));
        let _rug_ed_tests_rug_192_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_193 {
    use crate::ops::overflowing::OverflowingMul;
    #[test]
    fn test_overflowing_mul() {
        let _rug_st_tests_rug_193_rrrruuuugggg_test_overflowing_mul = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = false;
        let p0: i128 = i128::MAX;
        let p1: i128 = rug_fuzz_0;
        let (result, overflow) = <i128 as OverflowingMul>::overflowing_mul(&p0, &p1);
        debug_assert_eq!((i128::MAX, rug_fuzz_1), (result, overflow));
        let _rug_ed_tests_rug_193_rrrruuuugggg_test_overflowing_mul = 0;
    }
}
