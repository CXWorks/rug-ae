use core::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};
/// Performs addition that returns `None` instead of wrapping around on
/// overflow.
pub trait CheckedAdd: Sized + Add<Self, Output = Self> {
    /// Adds two numbers, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_add(&self, v: &Self) -> Option<Self>;
}
macro_rules! checked_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, v : &$t) -> Option <$t >
        { <$t >::$method (* self, * v) } }
    };
}
checked_impl!(CheckedAdd, checked_add, u8);
checked_impl!(CheckedAdd, checked_add, u16);
checked_impl!(CheckedAdd, checked_add, u32);
checked_impl!(CheckedAdd, checked_add, u64);
checked_impl!(CheckedAdd, checked_add, usize);
checked_impl!(CheckedAdd, checked_add, u128);
checked_impl!(CheckedAdd, checked_add, i8);
checked_impl!(CheckedAdd, checked_add, i16);
checked_impl!(CheckedAdd, checked_add, i32);
checked_impl!(CheckedAdd, checked_add, i64);
checked_impl!(CheckedAdd, checked_add, isize);
checked_impl!(CheckedAdd, checked_add, i128);
/// Performs subtraction that returns `None` instead of wrapping around on underflow.
pub trait CheckedSub: Sized + Sub<Self, Output = Self> {
    /// Subtracts two numbers, checking for underflow. If underflow happens,
    /// `None` is returned.
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}
checked_impl!(CheckedSub, checked_sub, u8);
checked_impl!(CheckedSub, checked_sub, u16);
checked_impl!(CheckedSub, checked_sub, u32);
checked_impl!(CheckedSub, checked_sub, u64);
checked_impl!(CheckedSub, checked_sub, usize);
checked_impl!(CheckedSub, checked_sub, u128);
checked_impl!(CheckedSub, checked_sub, i8);
checked_impl!(CheckedSub, checked_sub, i16);
checked_impl!(CheckedSub, checked_sub, i32);
checked_impl!(CheckedSub, checked_sub, i64);
checked_impl!(CheckedSub, checked_sub, isize);
checked_impl!(CheckedSub, checked_sub, i128);
/// Performs multiplication that returns `None` instead of wrapping around on underflow or
/// overflow.
pub trait CheckedMul: Sized + Mul<Self, Output = Self> {
    /// Multiplies two numbers, checking for underflow or overflow. If underflow
    /// or overflow happens, `None` is returned.
    fn checked_mul(&self, v: &Self) -> Option<Self>;
}
checked_impl!(CheckedMul, checked_mul, u8);
checked_impl!(CheckedMul, checked_mul, u16);
checked_impl!(CheckedMul, checked_mul, u32);
checked_impl!(CheckedMul, checked_mul, u64);
checked_impl!(CheckedMul, checked_mul, usize);
checked_impl!(CheckedMul, checked_mul, u128);
checked_impl!(CheckedMul, checked_mul, i8);
checked_impl!(CheckedMul, checked_mul, i16);
checked_impl!(CheckedMul, checked_mul, i32);
checked_impl!(CheckedMul, checked_mul, i64);
checked_impl!(CheckedMul, checked_mul, isize);
checked_impl!(CheckedMul, checked_mul, i128);
/// Performs division that returns `None` instead of panicking on division by zero and instead of
/// wrapping around on underflow and overflow.
pub trait CheckedDiv: Sized + Div<Self, Output = Self> {
    /// Divides two numbers, checking for underflow, overflow and division by
    /// zero. If any of that happens, `None` is returned.
    fn checked_div(&self, v: &Self) -> Option<Self>;
}
checked_impl!(CheckedDiv, checked_div, u8);
checked_impl!(CheckedDiv, checked_div, u16);
checked_impl!(CheckedDiv, checked_div, u32);
checked_impl!(CheckedDiv, checked_div, u64);
checked_impl!(CheckedDiv, checked_div, usize);
checked_impl!(CheckedDiv, checked_div, u128);
checked_impl!(CheckedDiv, checked_div, i8);
checked_impl!(CheckedDiv, checked_div, i16);
checked_impl!(CheckedDiv, checked_div, i32);
checked_impl!(CheckedDiv, checked_div, i64);
checked_impl!(CheckedDiv, checked_div, isize);
checked_impl!(CheckedDiv, checked_div, i128);
/// Performs an integral remainder that returns `None` instead of panicking on division by zero and
/// instead of wrapping around on underflow and overflow.
pub trait CheckedRem: Sized + Rem<Self, Output = Self> {
    /// Finds the remainder of dividing two numbers, checking for underflow, overflow and division
    /// by zero. If any of that happens, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::CheckedRem;
    /// use std::i32::MIN;
    ///
    /// assert_eq!(CheckedRem::checked_rem(&10, &7), Some(3));
    /// assert_eq!(CheckedRem::checked_rem(&10, &-7), Some(3));
    /// assert_eq!(CheckedRem::checked_rem(&-10, &7), Some(-3));
    /// assert_eq!(CheckedRem::checked_rem(&-10, &-7), Some(-3));
    ///
    /// assert_eq!(CheckedRem::checked_rem(&10, &0), None);
    ///
    /// assert_eq!(CheckedRem::checked_rem(&MIN, &1), Some(0));
    /// assert_eq!(CheckedRem::checked_rem(&MIN, &-1), None);
    /// ```
    fn checked_rem(&self, v: &Self) -> Option<Self>;
}
checked_impl!(CheckedRem, checked_rem, u8);
checked_impl!(CheckedRem, checked_rem, u16);
checked_impl!(CheckedRem, checked_rem, u32);
checked_impl!(CheckedRem, checked_rem, u64);
checked_impl!(CheckedRem, checked_rem, usize);
checked_impl!(CheckedRem, checked_rem, u128);
checked_impl!(CheckedRem, checked_rem, i8);
checked_impl!(CheckedRem, checked_rem, i16);
checked_impl!(CheckedRem, checked_rem, i32);
checked_impl!(CheckedRem, checked_rem, i64);
checked_impl!(CheckedRem, checked_rem, isize);
checked_impl!(CheckedRem, checked_rem, i128);
macro_rules! checked_impl_unary {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self) -> Option <$t > { <$t
        >::$method (* self) } }
    };
}
/// Performs negation that returns `None` if the result can't be represented.
pub trait CheckedNeg: Sized {
    /// Negates a number, returning `None` for results that can't be represented, like signed `MIN`
    /// values that can't be positive, or non-zero unsigned values that can't be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::CheckedNeg;
    /// use std::i32::MIN;
    ///
    /// assert_eq!(CheckedNeg::checked_neg(&1_i32), Some(-1));
    /// assert_eq!(CheckedNeg::checked_neg(&-1_i32), Some(1));
    /// assert_eq!(CheckedNeg::checked_neg(&MIN), None);
    ///
    /// assert_eq!(CheckedNeg::checked_neg(&0_u32), Some(0));
    /// assert_eq!(CheckedNeg::checked_neg(&1_u32), None);
    /// ```
    fn checked_neg(&self) -> Option<Self>;
}
checked_impl_unary!(CheckedNeg, checked_neg, u8);
checked_impl_unary!(CheckedNeg, checked_neg, u16);
checked_impl_unary!(CheckedNeg, checked_neg, u32);
checked_impl_unary!(CheckedNeg, checked_neg, u64);
checked_impl_unary!(CheckedNeg, checked_neg, usize);
checked_impl_unary!(CheckedNeg, checked_neg, u128);
checked_impl_unary!(CheckedNeg, checked_neg, i8);
checked_impl_unary!(CheckedNeg, checked_neg, i16);
checked_impl_unary!(CheckedNeg, checked_neg, i32);
checked_impl_unary!(CheckedNeg, checked_neg, i64);
checked_impl_unary!(CheckedNeg, checked_neg, isize);
checked_impl_unary!(CheckedNeg, checked_neg, i128);
/// Performs a left shift that returns `None` on shifts larger than
/// the type width.
pub trait CheckedShl: Sized + Shl<u32, Output = Self> {
    /// Checked shift left. Computes `self << rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// ```
    /// use num_traits::CheckedShl;
    ///
    /// let x: u16 = 0x0001;
    ///
    /// assert_eq!(CheckedShl::checked_shl(&x, 0),  Some(0x0001));
    /// assert_eq!(CheckedShl::checked_shl(&x, 1),  Some(0x0002));
    /// assert_eq!(CheckedShl::checked_shl(&x, 15), Some(0x8000));
    /// assert_eq!(CheckedShl::checked_shl(&x, 16), None);
    /// ```
    fn checked_shl(&self, rhs: u32) -> Option<Self>;
}
macro_rules! checked_shift_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t { #[inline] fn $method (& self, rhs : u32) -> Option <$t
        > { <$t >::$method (* self, rhs) } }
    };
}
checked_shift_impl!(CheckedShl, checked_shl, u8);
checked_shift_impl!(CheckedShl, checked_shl, u16);
checked_shift_impl!(CheckedShl, checked_shl, u32);
checked_shift_impl!(CheckedShl, checked_shl, u64);
checked_shift_impl!(CheckedShl, checked_shl, usize);
checked_shift_impl!(CheckedShl, checked_shl, u128);
checked_shift_impl!(CheckedShl, checked_shl, i8);
checked_shift_impl!(CheckedShl, checked_shl, i16);
checked_shift_impl!(CheckedShl, checked_shl, i32);
checked_shift_impl!(CheckedShl, checked_shl, i64);
checked_shift_impl!(CheckedShl, checked_shl, isize);
checked_shift_impl!(CheckedShl, checked_shl, i128);
/// Performs a right shift that returns `None` on shifts larger than
/// the type width.
pub trait CheckedShr: Sized + Shr<u32, Output = Self> {
    /// Checked shift right. Computes `self >> rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// ```
    /// use num_traits::CheckedShr;
    ///
    /// let x: u16 = 0x8000;
    ///
    /// assert_eq!(CheckedShr::checked_shr(&x, 0),  Some(0x8000));
    /// assert_eq!(CheckedShr::checked_shr(&x, 1),  Some(0x4000));
    /// assert_eq!(CheckedShr::checked_shr(&x, 15), Some(0x0001));
    /// assert_eq!(CheckedShr::checked_shr(&x, 16), None);
    /// ```
    fn checked_shr(&self, rhs: u32) -> Option<Self>;
}
checked_shift_impl!(CheckedShr, checked_shr, u8);
checked_shift_impl!(CheckedShr, checked_shr, u16);
checked_shift_impl!(CheckedShr, checked_shr, u32);
checked_shift_impl!(CheckedShr, checked_shr, u64);
checked_shift_impl!(CheckedShr, checked_shr, usize);
checked_shift_impl!(CheckedShr, checked_shr, u128);
checked_shift_impl!(CheckedShr, checked_shr, i8);
checked_shift_impl!(CheckedShr, checked_shr, i16);
checked_shift_impl!(CheckedShr, checked_shr, i32);
checked_shift_impl!(CheckedShr, checked_shr, i64);
checked_shift_impl!(CheckedShr, checked_shr, isize);
checked_shift_impl!(CheckedShr, checked_shr, i128);
#[cfg(test)]
mod tests_llm_16_668_llm_16_668 {
    use crate::CheckedAdd;
    #[test]
    fn i128_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(i128::checked_add(i128::MAX, rug_fuzz_0), None);
        debug_assert_eq!(i128::checked_add(i128::MIN, - rug_fuzz_1), None);
        debug_assert_eq!(i128::checked_add(rug_fuzz_2, rug_fuzz_3), Some(0));
        debug_assert_eq!(i128::checked_add(rug_fuzz_4, rug_fuzz_5), Some(3));
        debug_assert_eq!(i128::checked_add(i128::MAX, rug_fuzz_6), Some(i128::MAX));
        debug_assert_eq!(
            i128::checked_add(i128::MAX - rug_fuzz_7, rug_fuzz_8), Some(i128::MAX)
        );
        debug_assert_eq!(i128::checked_add(i128::MIN, rug_fuzz_9), Some(i128::MIN));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_669_llm_16_669 {
    use crate::CheckedDiv;
    #[test]
    fn test_checked_div_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as CheckedDiv > ::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(10)
        );
        debug_assert_eq!(
            < i128 as CheckedDiv > ::checked_div(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i128 as CheckedDiv > ::checked_div(& i128::MIN, & - rug_fuzz_4), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_670_llm_16_670 {
    use crate::ops::checked::CheckedMul;
    #[test]
    fn test_checked_mul_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_2, & rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_4, & rug_fuzz_5), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_6, & rug_fuzz_7), Some(1)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& i128::MAX, & rug_fuzz_8),
            Some(i128::MAX)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_9, & i128::MAX),
            Some(i128::MAX)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& i128::MAX, & rug_fuzz_10), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_11, & i128::MAX), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& i128::MAX, & - rug_fuzz_12), Some(-
            i128::MAX)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& - rug_fuzz_13, & i128::MAX), Some(-
            i128::MAX)
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& i128::MAX, & rug_fuzz_14), None
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& rug_fuzz_15, & i128::MAX), None
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& i128::MIN, & - rug_fuzz_16), None
        );
        debug_assert_eq!(
            < i128 as CheckedMul > ::checked_mul(& - rug_fuzz_17, & i128::MIN), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_671_llm_16_671 {
    use crate::CheckedNeg;
    #[test]
    fn test_checked_neg_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_neg(), Some(0i128));
        debug_assert_eq!((rug_fuzz_1).checked_neg(), Some(- 1i128));
        debug_assert_eq!((- rug_fuzz_2).checked_neg(), Some(1i128));
        debug_assert_eq!((i128::MIN).checked_neg(), None);
        debug_assert_eq!((i128::MAX).checked_neg(), Some(- i128::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_672_llm_16_672 {
    use crate::CheckedRem;
    #[test]
    fn test_checked_rem_i128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24)) = <(i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_4, & rug_fuzz_5), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_6, & rug_fuzz_7), Some(1)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_8, & rug_fuzz_9), Some(2)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_10, & rug_fuzz_11), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_12, & rug_fuzz_13), Some(4)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& - rug_fuzz_14, & rug_fuzz_15), Some(-
            4)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& - rug_fuzz_16, & - rug_fuzz_17),
            Some(- 4)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_18, & - rug_fuzz_19), Some(4)
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& rug_fuzz_20, & rug_fuzz_21), None
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& - rug_fuzz_22, & rug_fuzz_23), None
        );
        debug_assert_eq!(
            < i128 as CheckedRem > ::checked_rem(& i128::MIN, & (- rug_fuzz_24)), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_673_llm_16_673 {
    use crate::ops::checked::CheckedShl;
    #[test]
    fn i128_checked_shl_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_shl(rug_fuzz_1), Some(1));
        debug_assert_eq!((rug_fuzz_2).checked_shl(rug_fuzz_3), Some(1i128 << 127));
             }
});    }
    #[test]
    fn i128_checked_shl_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_shl(rug_fuzz_1), None);
             }
});    }
    #[test]
    fn i128_checked_shl_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((- rug_fuzz_0).checked_shl(rug_fuzz_1), Some(- 1));
        debug_assert_eq!((- rug_fuzz_2).checked_shl(rug_fuzz_3), Some(- 1i128 << 127));
             }
});    }
    #[test]
    fn i128_checked_shl_large_shifts() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_shl(rug_fuzz_1), None);
        debug_assert_eq!((- rug_fuzz_2).checked_shl(rug_fuzz_3), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_674_llm_16_674 {
    use crate::ops::checked::CheckedShr;
    #[test]
    fn test_checked_shr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, i128, u32, i128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& - rug_fuzz_2, rug_fuzz_3), Some(-
            1i128 >> 1)
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(1)
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& rug_fuzz_6, rug_fuzz_7), Some(0)
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& - rug_fuzz_8, rug_fuzz_9), Some(- 1)
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& rug_fuzz_10, rug_fuzz_11), None
        );
        debug_assert_eq!(
            < i128 as CheckedShr > ::checked_shr(& - rug_fuzz_12, rug_fuzz_13), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_778_llm_16_778 {
    use crate::CheckedAdd;
    #[test]
    fn i16_checked_add_with_no_overflow() {
        assert_eq!(< i16 as CheckedAdd >::checked_add(& 7, & 8), Some(15));
    }
    #[test]
    fn i16_checked_add_with_positive_overflow() {
        assert_eq!(< i16 as CheckedAdd >::checked_add(& i16::MAX, & 1), None);
    }
    #[test]
    fn i16_checked_add_with_negative_overflow() {
        assert_eq!(< i16 as CheckedAdd >::checked_add(& i16::MIN, &- 1), None);
    }
    #[test]
    fn i16_checked_add_with_large_numbers() {
        assert_eq!(< i16 as CheckedAdd >::checked_add(& 1234, & 5678), Some(6912));
    }
    #[test]
    fn i16_checked_add_with_zero() {
        assert_eq!(< i16 as CheckedAdd >::checked_add(& 0, &- 32768), Some(- 32768));
    }
}
#[cfg(test)]
mod tests_llm_16_779_llm_16_779 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn checked_div_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, i32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(5));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(
            CheckedDiv::checked_div(& - rug_fuzz_4, & rug_fuzz_5), Some(- 5)
        );
        debug_assert_eq!(CheckedDiv::checked_div(& i16::MIN, & - rug_fuzz_6), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_780_llm_16_780 {
    use crate::CheckedMul;
    #[test]
    fn test_checked_mul() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i16, i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_mul(rug_fuzz_1), Some(50));
        debug_assert_eq!((i16::MAX).checked_mul(rug_fuzz_2), Some(i16::MAX));
        debug_assert_eq!((i16::MAX).checked_mul(rug_fuzz_3), None);
        debug_assert_eq!((i16::MIN).checked_mul(- rug_fuzz_4), Some(i16::MIN));
        debug_assert_eq!((i16::MIN).checked_mul(rug_fuzz_5), None);
        debug_assert_eq!((rug_fuzz_6).checked_mul(rug_fuzz_7), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_781_llm_16_781 {
    use crate::ops::checked::CheckedNeg;
    #[test]
    fn test_checked_neg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i16 as CheckedNeg > ::checked_neg(& rug_fuzz_0), Some(0));
        debug_assert_eq!(< i16 as CheckedNeg > ::checked_neg(& rug_fuzz_1), Some(- 1));
        debug_assert_eq!(< i16 as CheckedNeg > ::checked_neg(& - rug_fuzz_2), Some(1));
        debug_assert_eq!(< i16 as CheckedNeg > ::checked_neg(& i16::MIN), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_782_llm_16_782 {
    use crate::CheckedRem;
    #[test]
    fn test_i16_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i16, i16, i16, i16, i16, i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i16 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < i16 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(1)
        );
        debug_assert_eq!(
            < i16 as CheckedRem > ::checked_rem(& rug_fuzz_4, & rug_fuzz_5), None
        );
        debug_assert_eq!(
            < i16 as CheckedRem > ::checked_rem(& i16::MIN, & - rug_fuzz_6), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_783_llm_16_783 {
    use super::*;
    use crate::*;
    #[test]
    fn i16_checked_shl_works_correctly() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16, u32, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(i16::checked_shl(rug_fuzz_0, rug_fuzz_1), Some(1));
        debug_assert_eq!(i16::checked_shl(rug_fuzz_2, rug_fuzz_3), Some(2));
        debug_assert_eq!(i16::checked_shl(rug_fuzz_4, rug_fuzz_5), Some(16384));
        debug_assert_eq!(i16::checked_shl(- rug_fuzz_6, rug_fuzz_7), Some(- 16384));
        debug_assert_eq!(i16::checked_shl(rug_fuzz_8, rug_fuzz_9), None);
        debug_assert_eq!(i16::checked_shl(rug_fuzz_10, rug_fuzz_11), Some(- 2));
        debug_assert_eq!(i16::checked_shl(rug_fuzz_12, rug_fuzz_13), None);
        debug_assert_eq!(i16::checked_shl(rug_fuzz_14, u32::MAX), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_785_llm_16_785 {
    use super::*;
    use crate::*;
    #[test]
    fn i16_checked_sub_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i16 = rug_fuzz_0;
        let b: i16 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), Some(50));
             }
});    }
    #[test]
    fn i16_checked_sub_negative_result() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i16 = rug_fuzz_0;
        let b: i16 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), Some(- 50));
             }
});    }
    #[test]
    fn i16_checked_sub_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i16 = i16::MIN;
        let b: i16 = rug_fuzz_0;
        debug_assert_eq!(a.checked_sub(b), None);
             }
});    }
    #[test]
    fn i16_checked_sub_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i16 = i16::MAX;
        let b: i16 = -rug_fuzz_0;
        debug_assert_eq!(a.checked_sub(b), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_888_llm_16_888 {
    use super::*;
    use crate::*;
    use crate::ops::checked::CheckedAdd;
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as CheckedAdd > ::checked_add(& rug_fuzz_0, & rug_fuzz_1), Some(15)
        );
        debug_assert_eq!(
            < i32 as CheckedAdd > ::checked_add(& i32::MAX, & rug_fuzz_2), None
        );
        debug_assert_eq!(
            < i32 as CheckedAdd > ::checked_add(& - rug_fuzz_3, & rug_fuzz_4), Some(0)
        );
        debug_assert_eq!(
            < i32 as CheckedAdd > ::checked_add(& i32::MIN, & - rug_fuzz_5), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_889_llm_16_889 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as CheckedDiv > ::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(5)
        );
        debug_assert_eq!(
            < i32 as CheckedDiv > ::checked_div(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i32 as CheckedDiv > ::checked_div(& rug_fuzz_4, & - rug_fuzz_5), Some(- 5)
        );
        debug_assert_eq!(
            < i32 as CheckedDiv > ::checked_div(& - rug_fuzz_6, & - rug_fuzz_7), Some(5)
        );
        debug_assert_eq!(
            < i32 as CheckedDiv > ::checked_div(& i32::MIN, & - rug_fuzz_8), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_890_llm_16_890 {
    use crate::CheckedMul;
    #[test]
    fn test_checked_mul() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_mul(rug_fuzz_1), Some(0));
        debug_assert_eq!(rug_fuzz_2.checked_mul(rug_fuzz_3), Some(0));
        debug_assert_eq!(rug_fuzz_4.checked_mul(rug_fuzz_5), Some(0));
        debug_assert_eq!(rug_fuzz_6.checked_mul(rug_fuzz_7), Some(8));
        debug_assert_eq!(i32::MAX.checked_mul(rug_fuzz_8), Some(i32::MAX));
        debug_assert_eq!((- rug_fuzz_9).checked_mul(rug_fuzz_10), Some(- 8));
        debug_assert_eq!((- rug_fuzz_11).checked_mul(i32::MIN), None);
        debug_assert_eq!(rug_fuzz_12.checked_mul(- rug_fuzz_13), Some(- 8));
        debug_assert_eq!((- rug_fuzz_14).checked_mul(rug_fuzz_15), Some(- 8));
        debug_assert_eq!(i32::MAX.checked_mul(rug_fuzz_16), None);
        debug_assert_eq!(i32::MIN.checked_mul(- rug_fuzz_17), None);
        debug_assert_eq!(
            (i32::MAX / rug_fuzz_18 + rug_fuzz_19).checked_mul(rug_fuzz_20), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_891_llm_16_891 {
    use crate::ops::checked::CheckedNeg;
    #[test]
    fn test_checked_neg_i32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i32 as CheckedNeg > ::checked_neg(& rug_fuzz_0), Some(0));
        debug_assert_eq!(< i32 as CheckedNeg > ::checked_neg(& rug_fuzz_1), Some(- 1));
        debug_assert_eq!(< i32 as CheckedNeg > ::checked_neg(& - rug_fuzz_2), Some(1));
        debug_assert_eq!(< i32 as CheckedNeg > ::checked_neg(& i32::MIN), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_892_llm_16_892 {
    use crate::ops::checked::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(1));
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(CheckedRem::checked_rem(& i32::MIN, & - rug_fuzz_4), None);
        debug_assert_eq!(
            CheckedRem::checked_rem(& - rug_fuzz_5, & rug_fuzz_6), Some(- 1)
        );
        debug_assert_eq!(
            CheckedRem::checked_rem(& - rug_fuzz_7, & - rug_fuzz_8), Some(- 1)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_893_llm_16_893 {
    use crate::ops::checked::CheckedShl;
    #[test]
    fn test_checked_shl() {
        assert_eq!(< i32 as CheckedShl >::checked_shl(& 1, 0), Some(1));
        assert_eq!(< i32 as CheckedShl >::checked_shl(& 1, 31), Some(1 << 31));
        assert_eq!(< i32 as CheckedShl >::checked_shl(& 1, 32), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(&- 1, 31), Some(- 1 << 31));
        assert_eq!(< i32 as CheckedShl >::checked_shl(&- 1, 32), None);
        assert_eq!(
            < i32 as CheckedShl >::checked_shl(& (- 2147483648), 1), Some(- 2147483648 <<
            1)
        );
        assert_eq!(< i32 as CheckedShl >::checked_shl(& (- 2147483648), 31), None);
        assert_eq!(
            < i32 as CheckedShl >::checked_shl(& (2147483647), 1), Some(2147483647 << 1)
        );
        assert_eq!(< i32 as CheckedShl >::checked_shl(& (2147483647), 30), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(& (2147483647), 31), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(& 1, 100), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(&- 1, 100), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(& (2147483647), 100), None);
        assert_eq!(< i32 as CheckedShl >::checked_shl(& (- 2147483648), 100), None);
    }
}
#[cfg(test)]
mod tests_llm_16_894 {
    use crate::CheckedShr;
    #[test]
    fn test_checked_shr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(i32, u32, i32, u32, i32, u32, i32, u32, i32, u32, u32, u32, i32, u32, i32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(4)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(0)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& - rug_fuzz_6, rug_fuzz_7), Some(- 4)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& - rug_fuzz_8, rug_fuzz_9), Some(- 1)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& i32::MAX, rug_fuzz_10), Some(i32::MAX /
            2)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& i32::MIN, rug_fuzz_11), Some(i32::MIN /
            2)
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& rug_fuzz_12, rug_fuzz_13), None
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& - rug_fuzz_14, rug_fuzz_15), None
        );
        debug_assert_eq!(
            < i32 as CheckedShr > ::checked_shr(& rug_fuzz_16, u32::MAX), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_895_llm_16_895 {
    use crate::CheckedSub;
    #[test]
    fn test_checked_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i32 as CheckedSub > ::checked_sub(& rug_fuzz_0, & rug_fuzz_1), Some(2)
        );
        debug_assert_eq!(
            < i32 as CheckedSub > ::checked_sub(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i32 as CheckedSub > ::checked_sub(& i32::MIN, & rug_fuzz_4), None
        );
        debug_assert_eq!(
            < i32 as CheckedSub > ::checked_sub(& i32::MAX, & (- rug_fuzz_5)), None
        );
        debug_assert_eq!(
            < i32 as CheckedSub > ::checked_sub(& rug_fuzz_6, & rug_fuzz_7), Some(0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_998_llm_16_998 {
    use crate::ops::checked::CheckedAdd;
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: i64 = rug_fuzz_0;
        let b: i64 = rug_fuzz_1;
        let c: i64 = i64::MAX;
        let result = a.checked_add(b);
        debug_assert_eq!(result, Some(a + b));
        let overflow = c.checked_add(b);
        debug_assert_eq!(overflow, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_999_llm_16_999 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(5)
        );
        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& - rug_fuzz_4, & - rug_fuzz_5), Some(5)
        );
        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& - rug_fuzz_6, & rug_fuzz_7), Some(- 5)
        );
        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& rug_fuzz_8, & - rug_fuzz_9), Some(- 5)
        );
        debug_assert_eq!(
            < i64 as CheckedDiv > ::checked_div(& std::i64::MIN, & - rug_fuzz_10), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1000_llm_16_1000 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_mul() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(48));
        debug_assert_eq!(
            CheckedMul::checked_mul(& - rug_fuzz_2, & rug_fuzz_3), Some(- 48)
        );
        debug_assert_eq!(CheckedMul::checked_mul(& i64::MAX, & rug_fuzz_4), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1001_llm_16_1001 {
    use crate::CheckedNeg;
    #[test]
    fn checked_neg_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< i64 as CheckedNeg > ::checked_neg(& rug_fuzz_0), Some(0));
        debug_assert_eq!(< i64 as CheckedNeg > ::checked_neg(& - rug_fuzz_1), Some(1));
        debug_assert_eq!(< i64 as CheckedNeg > ::checked_neg(& rug_fuzz_2), Some(- 1));
        debug_assert_eq!(< i64 as CheckedNeg > ::checked_neg(& i64::MIN), None);
        debug_assert_eq!(
            < i64 as CheckedNeg > ::checked_neg(& i64::MAX), Some(i64::MIN + 1)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1002_llm_16_1002 {
    use crate::CheckedRem;
    #[test]
    fn test_checked_rem_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = rug_fuzz_0;
        let y: i64 = rug_fuzz_1;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, None);
             }
});    }
    #[test]
    fn test_checked_rem_with_positive_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = rug_fuzz_0;
        let y: i64 = rug_fuzz_1;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, Some(2));
             }
});    }
    #[test]
    fn test_checked_rem_with_negative_divisor() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = rug_fuzz_0;
        let y: i64 = -rug_fuzz_1;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, Some(2));
             }
});    }
    #[test]
    fn test_checked_rem_with_positive_dividend() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = -rug_fuzz_0;
        let y: i64 = rug_fuzz_1;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, Some(- 2));
             }
});    }
    #[test]
    fn test_checked_rem_with_negative_dividend() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = -rug_fuzz_0;
        let y: i64 = -rug_fuzz_1;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, Some(- 2));
             }
});    }
    #[test]
    fn test_checked_rem_with_min_value() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let x: i64 = i64::MIN;
        let y: i64 = -rug_fuzz_0;
        let result = x.checked_rem(y);
        debug_assert_eq!(result, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1004_llm_16_1004 {
    use crate::CheckedShr;
    #[test]
    fn checked_shr_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(4));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(1));
             }
});    }
    #[test]
    fn checked_shr_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0));
             }
});    }
    #[test]
    fn checked_shr_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), None);
             }
});    }
    #[test]
    fn checked_shr_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& - rug_fuzz_0, rug_fuzz_1), Some(- 4));
             }
});    }
    #[test]
    fn checked_shr_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(8));
             }
});    }
    #[test]
    fn checked_shr_full_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for i in rug_fuzz_0..rug_fuzz_1 {
            let power_of_two = rug_fuzz_2 << i;
            debug_assert_eq!(CheckedShr::checked_shr(& power_of_two, i), Some(1));
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1005_llm_16_1005 {
    use crate::CheckedSub;
    #[test]
    fn test_checked_sub_i64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(i64::checked_sub(rug_fuzz_0, rug_fuzz_1), Some(90));
        debug_assert_eq!(i64::checked_sub(i64::MIN, rug_fuzz_2), None);
        debug_assert_eq!(i64::checked_sub(rug_fuzz_3, rug_fuzz_4), Some(0));
        debug_assert_eq!(i64::checked_sub(- rug_fuzz_5, - rug_fuzz_6), Some(0));
        debug_assert_eq!(i64::checked_sub(i64::MIN, - rug_fuzz_7), None);
        debug_assert_eq!(i64::checked_sub(i64::MAX, i64::MAX), Some(0));
        debug_assert_eq!(i64::checked_sub(i64::MAX, i64::MIN), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1108_llm_16_1108 {
    use crate::ops::checked::CheckedAdd;
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_add(rug_fuzz_1), Some(3));
        debug_assert_eq!(i8::MAX.checked_add(rug_fuzz_2), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1109_llm_16_1109 {
    use crate::CheckedDiv;
    #[test]
    fn checked_div_i8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i8, i8, i8, i8, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Some(rug_fuzz_0), < i8 as CheckedDiv > ::checked_div(& 10, & 5)
        );
        debug_assert_eq!(Some(rug_fuzz_1), < i8 as CheckedDiv > ::checked_div(& 0, & 5));
        debug_assert_eq!(
            Some(- rug_fuzz_2), < i8 as CheckedDiv > ::checked_div(& - 10, & 5)
        );
        debug_assert_eq!(
            Some(- rug_fuzz_3), < i8 as CheckedDiv > ::checked_div(& 10, & - 5)
        );
        debug_assert_eq!(
            Some(rug_fuzz_4), < i8 as CheckedDiv > ::checked_div(& - 10, & - 5)
        );
        debug_assert_eq!(None, < i8 as CheckedDiv > ::checked_div(& 10, & 0));
        debug_assert_eq!(None, < i8 as CheckedDiv > ::checked_div(& i8::MIN, & - 1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1112_llm_16_1112 {
    use crate::ops::checked::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(2));
        debug_assert_eq!(
            CheckedRem::checked_rem(& - rug_fuzz_2, & rug_fuzz_3), Some(- 2)
        );
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_4, & - rug_fuzz_5), Some(2));
        debug_assert_eq!(
            CheckedRem::checked_rem(& - rug_fuzz_6, & - rug_fuzz_7), Some(- 2)
        );
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_8, & rug_fuzz_9), None);
        debug_assert_eq!(CheckedRem::checked_rem(& i8::MIN, & - rug_fuzz_10), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1218_llm_16_1218 {
    use crate::ops::checked::CheckedAdd;
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < isize as CheckedAdd > ::checked_add(& rug_fuzz_0, & rug_fuzz_1), Some(8)
        );
        debug_assert_eq!(
            < isize as CheckedAdd > ::checked_add(& isize::MAX, & rug_fuzz_2), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1219_llm_16_1219 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn checked_div_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(4));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(CheckedDiv::checked_div(& isize::MIN, & - rug_fuzz_4), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1220_llm_16_1220 {
    use crate::ops::checked::CheckedMul;
    #[test]
    fn checked_mul_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(4));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_2, & rug_fuzz_3), Some(0));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_4, & rug_fuzz_5), Some(0));
             }
});    }
    #[test]
    fn checked_mul_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& isize::MAX, & rug_fuzz_0), None);
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_1, & isize::MAX), None);
             }
});    }
    #[test]
    fn checked_mul_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& isize::MIN, & (- rug_fuzz_0)), None);
        debug_assert_eq!(CheckedMul::checked_mul(& (- rug_fuzz_1), & isize::MIN), None);
             }
});    }
    #[test]
    fn checked_mul_negatives() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            CheckedMul::checked_mul(& (- rug_fuzz_0), & rug_fuzz_1), Some(- 4)
        );
        debug_assert_eq!(
            CheckedMul::checked_mul(& rug_fuzz_2, & (- rug_fuzz_3)), Some(- 4)
        );
        debug_assert_eq!(
            CheckedMul::checked_mul(& (- rug_fuzz_4), & (- rug_fuzz_5)), Some(4)
        );
             }
});    }
    #[test]
    fn checked_mul_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& isize::MIN, & rug_fuzz_0), Some(0));
        debug_assert_eq!(
            CheckedMul::checked_mul(& isize::MAX, & rug_fuzz_1), Some(isize::MAX)
        );
        debug_assert_eq!(
            CheckedMul::checked_mul(& rug_fuzz_2, & isize::MAX), Some(isize::MAX)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1222_llm_16_1222 {
    use crate::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_rem(rug_fuzz_1), Some(1));
        debug_assert_eq!((- rug_fuzz_2).checked_rem(- rug_fuzz_3), Some(- 1));
        debug_assert_eq!(rug_fuzz_4.checked_rem(- rug_fuzz_5), Some(1));
        debug_assert_eq!((- rug_fuzz_6).checked_rem(rug_fuzz_7), Some(- 1));
        debug_assert_eq!(rug_fuzz_8.checked_rem(rug_fuzz_9), None);
        debug_assert_eq!(rug_fuzz_10.checked_rem(rug_fuzz_11), Some(0));
        debug_assert_eq!(isize::MIN.checked_rem(- rug_fuzz_12), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1223 {
    use super::*;
    use crate::*;
    use crate::CheckedShl;
    #[test]
    fn test_checked_shl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Some(rug_fuzz_0), 1isize.checked_shl(3));
        debug_assert_eq!(Some(rug_fuzz_1), 0isize.checked_shl(1));
        debug_assert_eq!(None, 1isize.checked_shl(isize::BITS as u32));
        debug_assert_eq!(None, 1isize.checked_shl(isize::BITS as u32 - 1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1224_llm_16_1224 {
    use crate::CheckedShr;
    #[test]
    fn checked_shr_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(isize, u32, u32, u32, u32, isize, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: isize = rug_fuzz_0;
        debug_assert_eq!(value.checked_shr(rug_fuzz_1), Some(2));
        debug_assert_eq!(value.checked_shr(rug_fuzz_2), Some(8));
        debug_assert_eq!((value.checked_shr(rug_fuzz_3 * rug_fuzz_4)), Some(0));
        let negative_value: isize = -rug_fuzz_5;
        debug_assert_eq!(negative_value.checked_shr(rug_fuzz_6), Some(- 2));
        debug_assert_eq!(negative_value.checked_shr(rug_fuzz_7), Some(- 8));
        debug_assert_eq!(
            (negative_value.checked_shr(rug_fuzz_8 * rug_fuzz_9)), Some(- 1)
        );
        let large_shift = rug_fuzz_10 * rug_fuzz_11 + rug_fuzz_12;
        debug_assert_eq!(value.checked_shr(large_shift), None);
        debug_assert_eq!(negative_value.checked_shr(large_shift), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1225_llm_16_1225 {
    use crate::CheckedSub;
    #[test]
    fn checked_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(isize, isize, isize, isize, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_sub(rug_fuzz_1), Some(2));
        debug_assert_eq!(isize::MIN.checked_sub(rug_fuzz_2), None);
        debug_assert_eq!(rug_fuzz_3.checked_sub(rug_fuzz_4), Some(0));
        debug_assert_eq!(
            isize::MAX.checked_sub(- rug_fuzz_5), Some(isize::MAX.checked_add(1)
            .unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1424_llm_16_1424 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_add_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_add(rug_fuzz_1), Some(3u128));
        debug_assert_eq!(u128::MAX.checked_add(rug_fuzz_2), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1427_llm_16_1427 {
    use crate::ops::checked::CheckedNeg;
    #[test]
    fn checked_neg_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!((rug_fuzz_0).checked_neg(), None);
        debug_assert_eq!((rug_fuzz_1).checked_neg(), None);
        debug_assert_eq!(u128::MAX.checked_neg(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1428_llm_16_1428 {
    use crate::ops::checked::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& rug_fuzz_4, & rug_fuzz_5), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& rug_fuzz_6, & rug_fuzz_7), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& rug_fuzz_8, & rug_fuzz_9), Some(3)
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& (u128::MAX - rug_fuzz_10), &
            (u128::MAX / rug_fuzz_11)), Some((u128::MAX / 2) - 1)
        );
        debug_assert_eq!(
            < u128 as CheckedRem > ::checked_rem(& u128::MAX, & (u128::MAX /
            rug_fuzz_12)), Some(1)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1429_llm_16_1429 {
    use crate::ops::checked::CheckedShl;
    #[test]
    fn test_checked_shl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_2, rug_fuzz_3), Some(1)
        );
        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_4, rug_fuzz_5), Some(2)
        );
        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_6, rug_fuzz_7), Some(1 <<
            127)
        );
        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_8, rug_fuzz_9), None
        );
        debug_assert_eq!(
            < u128 as CheckedShl > ::checked_shl(& rug_fuzz_10, rug_fuzz_11), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1430_llm_16_1430 {
    use crate::ops::checked::CheckedShr;
    #[test]
    fn test_checked_shr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128, u32, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(1)
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_6, rug_fuzz_7),
            Some(0x0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF_u128)
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_8, rug_fuzz_9), None
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_10, rug_fuzz_11), None
        );
        debug_assert_eq!(
            < u128 as CheckedShr > ::checked_shr(& rug_fuzz_12, u32::MAX), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1529_llm_16_1529 {
    use crate::ops::checked::CheckedAdd;
    #[test]
    fn checked_add_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_add(rug_fuzz_1), Some(15));
        debug_assert_eq!(u16::MAX.checked_add(rug_fuzz_2), None);
        debug_assert_eq!(u16::MAX.checked_add(rug_fuzz_3), Some(u16::MAX));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1530_llm_16_1530 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u16, u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(5));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_4, & rug_fuzz_5), Some(0));
        debug_assert_eq!(
            CheckedDiv::checked_div(& u16::MAX, & rug_fuzz_6), Some(u16::MAX)
        );
        debug_assert_eq!(CheckedDiv::checked_div(& u16::MAX, & u16::MAX), Some(1));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_7, & u16::MAX), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1531_llm_16_1531 {
    use crate::CheckedMul;
    #[test]
    fn checked_mul_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(50));
        debug_assert_eq!(
            CheckedMul::checked_mul(& rug_fuzz_2, & u16::MAX), Some(u16::MAX)
        );
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_3, & u16::MAX), Some(0));
        debug_assert_eq!(CheckedMul::checked_mul(& u16::MAX, & rug_fuzz_4), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1532_llm_16_1532 {
    use crate::ops::checked::CheckedNeg;
    #[test]
    fn test_checked_neg_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u16 as CheckedNeg > ::checked_neg(& rug_fuzz_0), Some(0));
        debug_assert_eq!(< u16 as CheckedNeg > ::checked_neg(& rug_fuzz_1), None);
        debug_assert_eq!(< u16 as CheckedNeg > ::checked_neg(& rug_fuzz_2), None);
        debug_assert_eq!(< u16 as CheckedNeg > ::checked_neg(& u16::MAX), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1533_llm_16_1533 {
    use crate::CheckedRem;
    #[test]
    fn checked_rem_with_non_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u16, u16, u16, u16, u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(1)
        );
        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& u16::MAX, & rug_fuzz_4), Some(0)
        );
        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& rug_fuzz_5, & rug_fuzz_6), Some(0)
        );
             }
});    }
    #[test]
    fn checked_rem_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u16, u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), None
        );
        debug_assert_eq!(
            < u16 as CheckedRem > ::checked_rem(& u16::MAX, & rug_fuzz_2), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1534_llm_16_1534 {
    use crate::ops::checked::CheckedShl;
    #[test]
    fn test_checked_shl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32, u16, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_2, rug_fuzz_3), Some(16)
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_4, rug_fuzz_5), Some(1)
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_6, rug_fuzz_7), Some(0xFFFE)
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_8, rug_fuzz_9), None
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_10, rug_fuzz_11), None
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_12, rug_fuzz_13), None
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_14, rug_fuzz_15), None
        );
        debug_assert_eq!(
            < u16 as CheckedShl > ::checked_shl(& rug_fuzz_16, rug_fuzz_17), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1535_llm_16_1535 {
    use crate::CheckedShr;
    #[test]
    fn test_checked_shr_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u16, u32, u16, u32, u16, u32, u16, u32, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(4u16));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(8u16));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(0u16));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_6, rug_fuzz_7), None);
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_8, u32::MAX), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1536_llm_16_1536 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_sub_with_no_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_sub(rug_fuzz_1), Some(75));
             }
});    }
    #[test]
    fn checked_sub_with_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_sub(rug_fuzz_1), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1634_llm_16_1634 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_add() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(Some(rug_fuzz_0), 2u32.checked_add(3));
        debug_assert_eq!(None, u32::MAX.checked_add(1));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1635_llm_16_1635 {
    use crate::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedDiv > ::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(10)
        );
        debug_assert_eq!(
            < u32 as CheckedDiv > ::checked_div(& rug_fuzz_2, & rug_fuzz_3), None
        );
        debug_assert_eq!(
            < u32 as CheckedDiv > ::checked_div(& std::u32::MAX, & rug_fuzz_4),
            Some(std::u32::MAX)
        );
        debug_assert_eq!(
            < u32 as CheckedDiv > ::checked_div(& rug_fuzz_5, & rug_fuzz_6), Some(0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1637_llm_16_1637 {
    use crate::ops::checked::CheckedNeg;
    #[test]
    fn checked_neg_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u32 as CheckedNeg > ::checked_neg(& rug_fuzz_0), None);
        debug_assert_eq!(< u32 as CheckedNeg > ::checked_neg(& rug_fuzz_1), None);
        debug_assert_eq!(< u32 as CheckedNeg > ::checked_neg(& rug_fuzz_2), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1638_llm_16_1638 {
    use crate::ops::checked::CheckedRem;
    #[test]
    fn checked_rem_with_non_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(1)
        );
             }
});    }
    #[test]
    fn checked_rem_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), None
        );
             }
});    }
    #[test]
    fn checked_rem_with_one() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
             }
});    }
    #[test]
    fn checked_rem_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& u32::MAX, & rug_fuzz_0), Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& u32::MAX, & u32::MAX), Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_1, & u32::MAX), Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1639_llm_16_1639 {
    use crate::CheckedShl;
    #[test]
    fn test_checked_shl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(1)
        );
        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& rug_fuzz_2, rug_fuzz_3), Some(32)
        );
        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& rug_fuzz_4, rug_fuzz_5), Some(1 << 31)
        );
        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& rug_fuzz_6, rug_fuzz_7), None
        );
        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& rug_fuzz_8, rug_fuzz_9), Some(0)
        );
        debug_assert_eq!(
            < u32 as CheckedShl > ::checked_shl(& u32::MAX, rug_fuzz_10), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1640_llm_16_1640 {
    use super::*;
    use crate::*;
    use crate::ops::checked::CheckedShr;
    #[test]
    fn checked_shr_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(4)
        );
        debug_assert_eq!(
            < u32 as CheckedShr > ::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(1)
        );
             }
});    }
    #[test]
    fn checked_shr_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(8)
        );
             }
});    }
    #[test]
    fn checked_shr_by_self_bits() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), None
        );
             }
});    }
    #[test]
    fn checked_shr_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u32 as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), None
        );
             }
});    }
    #[test]
    #[ignore]
    fn checked_shr_negative_behaviour() {
        let _rug_st_tests_llm_16_1640_llm_16_1640_rrrruuuugggg_checked_shr_negative_behaviour = 0;
        let _rug_ed_tests_llm_16_1640_llm_16_1640_rrrruuuugggg_checked_shr_negative_behaviour = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1641_llm_16_1641 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_sub(rug_fuzz_1), Some(5));
        debug_assert_eq!(rug_fuzz_2.checked_sub(rug_fuzz_3), None);
        debug_assert_eq!(u32::MAX.checked_sub(rug_fuzz_4), Some(u32::MAX - 1));
        debug_assert_eq!(rug_fuzz_5.checked_sub(u32::MAX), None);
        debug_assert_eq!(rug_fuzz_6.checked_sub(rug_fuzz_7), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1741_llm_16_1741 {
    use crate::ops::checked::CheckedMul;
    #[test]
    fn test_checked_mul() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as CheckedMul > ::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(6)
        );
        debug_assert_eq!(
            < u64 as CheckedMul > ::checked_mul(& u64::MAX, & rug_fuzz_2), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1743_llm_16_1743 {
    use crate::ops::checked::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(1)
        );
        debug_assert_eq!(
            < u64 as CheckedRem > ::checked_rem(& rug_fuzz_4, & rug_fuzz_5), None
        );
        debug_assert_eq!(
            < u64 as CheckedRem > ::checked_rem(& rug_fuzz_6, & rug_fuzz_7), Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedRem > ::checked_rem(& u64::MAX, & rug_fuzz_8), Some(0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1744_llm_16_1744 {
    use crate::CheckedShl;
    #[test]
    fn checked_shl_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u64, u32, u64, i32, u32, u32, u64, u32, u64, u32, u64, u32, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(u64::checked_shl(rug_fuzz_0, rug_fuzz_1), Some(4));
        debug_assert_eq!(u64::checked_shl(rug_fuzz_2 << rug_fuzz_3, rug_fuzz_4), None);
        debug_assert_eq!(u64::checked_shl(u64::MAX, rug_fuzz_5), None);
        debug_assert_eq!(u64::checked_shl(rug_fuzz_6, rug_fuzz_7), Some(0));
        debug_assert_eq!(u64::checked_shl(rug_fuzz_8, rug_fuzz_9), Some(1));
        debug_assert_eq!(u64::checked_shl(rug_fuzz_10, rug_fuzz_11), Some(1u64 << 63));
        debug_assert_eq!(u64::checked_shl(rug_fuzz_12, rug_fuzz_13), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1745_llm_16_1745 {
    use crate::ops::checked::CheckedShr;
    #[test]
    fn checked_shr_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0b100));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(0b10));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(0b1));
             }
});    }
    #[test]
    fn checked_shr_by_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0b1000)
        );
             }
});    }
    #[test]
    fn checked_shr_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(0));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_2, rug_fuzz_3), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1746_llm_16_1746 {
    use crate::ops::checked::CheckedSub;
    #[test]
    fn u64_checked_sub_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& rug_fuzz_0, & rug_fuzz_1), Some(5)
        );
        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& rug_fuzz_2, & rug_fuzz_3), None
        );
             }
});    }
    #[test]
    fn u64_checked_sub_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& rug_fuzz_0, & rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& u64::MAX, & rug_fuzz_2), Some(u64::MAX)
        );
        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& rug_fuzz_3, & u64::MAX), None
        );
             }
});    }
    #[test]
    fn u64_checked_sub_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& rug_fuzz_0, & rug_fuzz_1), None
        );
        debug_assert_eq!(
            < u64 as CheckedSub > ::checked_sub(& u64::MIN, & rug_fuzz_2), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1845_llm_16_1845 {
    use crate::CheckedAdd;
    #[test]
    fn checked_add_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(rug_fuzz_0.checked_add(rug_fuzz_1), Some(15));
        debug_assert_eq!(u8::MAX.checked_add(rug_fuzz_2), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1846_llm_16_1846 {
    use crate::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(10));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_4, & rug_fuzz_5), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1847_llm_16_1847 {
    use crate::ops::checked::CheckedMul;
    #[test]
    fn test_checked_mul_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_0, & rug_fuzz_1), Some(200));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_2, & rug_fuzz_3), Some(250));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_4, & rug_fuzz_5), Some(0));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_6, & rug_fuzz_7), Some(0));
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_8, & rug_fuzz_9), Some(255));
        debug_assert_eq!(
            CheckedMul::checked_mul(& rug_fuzz_10, & rug_fuzz_11), Some(255)
        );
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_12, & rug_fuzz_13), None);
        debug_assert_eq!(CheckedMul::checked_mul(& rug_fuzz_14, & rug_fuzz_15), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1848_llm_16_1848 {
    use crate::CheckedNeg;
    #[test]
    fn test_checked_neg() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< u8 as CheckedNeg > ::checked_neg(& rug_fuzz_0), None);
        for i in rug_fuzz_1..=u8::MAX {
            debug_assert_eq!(< u8 as CheckedNeg > ::checked_neg(& i), None);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1849_llm_16_1849 {
    use crate::CheckedRem;
    #[test]
    fn test_checked_rem() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < u8 as CheckedRem > ::checked_rem(& rug_fuzz_0, & rug_fuzz_1), None
        );
        debug_assert_eq!(
            < u8 as CheckedRem > ::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < u8 as CheckedRem > ::checked_rem(& rug_fuzz_4, & rug_fuzz_5), Some(1)
        );
        debug_assert_eq!(
            < u8 as CheckedRem > ::checked_rem(& rug_fuzz_6, & rug_fuzz_7), Some(0)
        );
        debug_assert_eq!(
            < u8 as CheckedRem > ::checked_rem(& rug_fuzz_8, & rug_fuzz_9), Some(0)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1850_llm_16_1850 {
    use crate::CheckedShl;
    #[test]
    fn test_checked_shl_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(u8, u32, u8, u32, u8, u32, u8, u32, u8, u32, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(2));
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_2, rug_fuzz_3), Some(16));
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_4, rug_fuzz_5), Some(128));
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_6, rug_fuzz_7), Some(0));
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_8, rug_fuzz_9), None);
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_10, rug_fuzz_11), None);
        debug_assert_eq!(CheckedShl::checked_shl(& rug_fuzz_12, u32::MAX), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1851_llm_16_1851 {
    use crate::CheckedShr;
    #[test]
    fn test_checked_shr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u32, u8, u32, u8, u32, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(4));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(1));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(0));
        debug_assert_eq!(CheckedShr::checked_shr(& rug_fuzz_6, rug_fuzz_7), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1852_llm_16_1852 {
    use crate::CheckedSub;
    #[test]
    fn test_checked_sub_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u8 = rug_fuzz_0;
        let b: u8 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), Some(50));
             }
});    }
    #[test]
    fn test_checked_sub_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u8 = rug_fuzz_0;
        let b: u8 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), None);
             }
});    }
    #[test]
    fn test_checked_sub_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u8 = rug_fuzz_0;
        let b: u8 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), Some(0));
             }
});    }
    #[test]
    fn test_checked_sub_with_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: u8 = rug_fuzz_0;
        let b: u8 = rug_fuzz_1;
        debug_assert_eq!(a.checked_sub(b), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1950_llm_16_1950 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_add_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a: usize = usize::MAX;
        let b: usize = rug_fuzz_0;
        let result = a.checked_add(b);
        debug_assert_eq!(result, None);
        let a: usize = usize::MAX - rug_fuzz_1;
        let b: usize = rug_fuzz_2;
        let result = a.checked_add(b);
        debug_assert_eq!(result, Some(usize::MAX));
        let a: usize = rug_fuzz_3;
        let b: usize = rug_fuzz_4;
        let result = a.checked_add(b);
        debug_assert_eq!(result, Some(0));
        let a: usize = rug_fuzz_5;
        let b: usize = rug_fuzz_6;
        let result = a.checked_add(b);
        debug_assert_eq!(result, Some(300));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1951_llm_16_1951 {
    use crate::ops::checked::CheckedDiv;
    #[test]
    fn test_checked_div() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_0, & rug_fuzz_1), Some(5));
        debug_assert_eq!(CheckedDiv::checked_div(& rug_fuzz_2, & rug_fuzz_3), None);
        debug_assert_eq!(
            CheckedDiv::checked_div(& std::usize::MAX, & rug_fuzz_4),
            Some(std::usize::MAX)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1952 {
    use crate::ops::checked::CheckedMul;
    #[test]
    fn test_checked_mul() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(None, usize::checked_mul(std::usize::MAX, 1));
        debug_assert_eq!(Some(rug_fuzz_0), usize::checked_mul(0, 0));
        debug_assert_eq!(Some(rug_fuzz_1), usize::checked_mul(0, std::usize::MAX));
        debug_assert_eq!(Some(std::usize::MAX), usize::checked_mul(std::usize::MAX, 1));
        debug_assert_eq!(None, usize::checked_mul(std::usize::MAX / 2 + 1, 2));
        debug_assert_eq!(
            Some(std::usize::MAX / rug_fuzz_2 * rug_fuzz_3),
            usize::checked_mul(std::usize::MAX / 2, 2)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1953_llm_16_1953 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_neg_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(< usize as CheckedNeg > ::checked_neg(& rug_fuzz_0), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1954_llm_16_1954 {
    use super::*;
    use crate::*;
    #[test]
    fn checked_rem_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, i32, i32, i32, i32, i32, i32, i32, usize, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_0, & rug_fuzz_1), Some(0));
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_2, & rug_fuzz_3), Some(1));
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_4, & rug_fuzz_5), None);
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_6, & rug_fuzz_7), Some(0));
        debug_assert_eq!(CheckedRem::checked_rem(& usize::MAX, & rug_fuzz_8), Some(0));
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_9, & rug_fuzz_10), Some(0));
        debug_assert_eq!(CheckedRem::checked_rem(& rug_fuzz_11, & rug_fuzz_12), Some(0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1955_llm_16_1955 {
    use crate::ops::checked::CheckedShl;
    #[test]
    fn checked_shl_basic() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, u32, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as CheckedShl > ::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(1)
        );
        debug_assert_eq!(
            < usize as CheckedShl > ::checked_shl(& rug_fuzz_2, rug_fuzz_3), Some(8)
        );
             }
});    }
    #[test]
    fn checked_shl_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as CheckedShl > ::checked_shl(& rug_fuzz_0, u32::BITS), None
        );
             }
});    }
    #[test]
    fn checked_shl_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as CheckedShl > ::checked_shl(& rug_fuzz_0, rug_fuzz_1), Some(0)
        );
        debug_assert_eq!(
            < usize as CheckedShl > ::checked_shl(& usize::MAX, rug_fuzz_2), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1956_llm_16_1956 {
    use crate::ops::checked::CheckedShr;
    #[test]
    fn test_checked_shr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, u32, usize, u32, usize, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            < usize as CheckedShr > ::checked_shr(& rug_fuzz_0, rug_fuzz_1), Some(8)
        );
        debug_assert_eq!(
            < usize as CheckedShr > ::checked_shr(& rug_fuzz_2, rug_fuzz_3), Some(0)
        );
        debug_assert_eq!(
            < usize as CheckedShr > ::checked_shr(& rug_fuzz_4, rug_fuzz_5), Some(1)
        );
        debug_assert_eq!(
            < usize as CheckedShr > ::checked_shr(& rug_fuzz_6, u32::BITS), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_1957_llm_16_1957 {
    use crate::ops::checked::CheckedSub;
    #[test]
    fn test_checked_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(CheckedSub::checked_sub(& rug_fuzz_0, & rug_fuzz_1), Some(2));
        debug_assert_eq!(CheckedSub::checked_sub(& rug_fuzz_2, & rug_fuzz_3), Some(0));
        debug_assert_eq!(CheckedSub::checked_sub(& rug_fuzz_4, & rug_fuzz_5), None);
             }
});    }
}
