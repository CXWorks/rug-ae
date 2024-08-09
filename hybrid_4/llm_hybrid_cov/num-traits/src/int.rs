use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use crate::bounds::Bounded;
use crate::ops::checked::*;
use crate::ops::saturating::Saturating;
use crate::{Num, NumCast};
/// Generic trait for primitive integers.
///
/// The `PrimInt` trait is an abstraction over the builtin primitive integer types (e.g., `u8`,
/// `u32`, `isize`, `i128`, ...). It inherits the basic numeric traits and extends them with
/// bitwise operators and non-wrapping arithmetic.
///
/// The trait explicitly inherits `Copy`, `Eq`, `Ord`, and `Sized`. The intention is that all
/// types implementing this trait behave like primitive types that are passed by value by default
/// and behave like builtin integers. Furthermore, the types are expected to expose the integer
/// value in binary representation and support bitwise operators. The standard bitwise operations
/// (e.g., bitwise-and, bitwise-or, right-shift, left-shift) are inherited and the trait extends
/// these with introspective queries (e.g., `PrimInt::count_ones()`, `PrimInt::leading_zeros()`),
/// bitwise combinators (e.g., `PrimInt::rotate_left()`), and endianness converters (e.g.,
/// `PrimInt::to_be()`).
///
/// All `PrimInt` types are expected to be fixed-width binary integers. The width can be queried
/// via `T::zero().count_zeros()`. The trait currently lacks a way to query the width at
/// compile-time.
///
/// While a default implementation for all builtin primitive integers is provided, the trait is in
/// no way restricted to these. Other integer types that fulfil the requirements are free to
/// implement the trait was well.
///
/// This trait and many of the method names originate in the unstable `core::num::Int` trait from
/// the rust standard library. The original trait was never stabilized and thus removed from the
/// standard library.
pub trait PrimInt: Sized + Copy + Num + NumCast + Bounded + PartialOrd + Ord + Eq + Not<
        Output = Self,
    > + BitAnd<
        Output = Self,
    > + BitOr<
        Output = Self,
    > + BitXor<
        Output = Self,
    > + Shl<
        usize,
        Output = Self,
    > + Shr<
        usize,
        Output = Self,
    > + CheckedAdd<
        Output = Self,
    > + CheckedSub<
        Output = Self,
    > + CheckedMul<Output = Self> + CheckedDiv<Output = Self> + Saturating {
    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0b01001100u8;
    ///
    /// assert_eq!(n.count_ones(), 3);
    /// ```
    fn count_ones(self) -> u32;
    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0b01001100u8;
    ///
    /// assert_eq!(n.count_zeros(), 5);
    /// ```
    fn count_zeros(self) -> u32;
    /// Returns the number of leading ones in the binary representation
    /// of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0xF00Du16;
    ///
    /// assert_eq!(n.leading_ones(), 4);
    /// ```
    fn leading_ones(self) -> u32 {
        (!self).leading_zeros()
    }
    /// Returns the number of leading zeros in the binary representation
    /// of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0b0101000u16;
    ///
    /// assert_eq!(n.leading_zeros(), 10);
    /// ```
    fn leading_zeros(self) -> u32;
    /// Returns the number of trailing ones in the binary representation
    /// of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0xBEEFu16;
    ///
    /// assert_eq!(n.trailing_ones(), 4);
    /// ```
    fn trailing_ones(self) -> u32 {
        (!self).trailing_zeros()
    }
    /// Returns the number of trailing zeros in the binary representation
    /// of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0b0101000u16;
    ///
    /// assert_eq!(n.trailing_zeros(), 3);
    /// ```
    fn trailing_zeros(self) -> u32;
    /// Shifts the bits to the left by a specified amount, `n`, wrapping
    /// the truncated bits to the end of the resulting integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    /// let m = 0x3456789ABCDEF012u64;
    ///
    /// assert_eq!(n.rotate_left(12), m);
    /// ```
    fn rotate_left(self, n: u32) -> Self;
    /// Shifts the bits to the right by a specified amount, `n`, wrapping
    /// the truncated bits to the beginning of the resulting integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    /// let m = 0xDEF0123456789ABCu64;
    ///
    /// assert_eq!(n.rotate_right(12), m);
    /// ```
    fn rotate_right(self, n: u32) -> Self;
    /// Shifts the bits to the left by a specified amount, `n`, filling
    /// zeros in the least significant bits.
    ///
    /// This is bitwise equivalent to signed `Shl`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    /// let m = 0x3456789ABCDEF000u64;
    ///
    /// assert_eq!(n.signed_shl(12), m);
    /// ```
    fn signed_shl(self, n: u32) -> Self;
    /// Shifts the bits to the right by a specified amount, `n`, copying
    /// the "sign bit" in the most significant bits even for unsigned types.
    ///
    /// This is bitwise equivalent to signed `Shr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0xFEDCBA9876543210u64;
    /// let m = 0xFFFFEDCBA9876543u64;
    ///
    /// assert_eq!(n.signed_shr(12), m);
    /// ```
    fn signed_shr(self, n: u32) -> Self;
    /// Shifts the bits to the left by a specified amount, `n`, filling
    /// zeros in the least significant bits.
    ///
    /// This is bitwise equivalent to unsigned `Shl`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFi64;
    /// let m = 0x3456789ABCDEF000i64;
    ///
    /// assert_eq!(n.unsigned_shl(12), m);
    /// ```
    fn unsigned_shl(self, n: u32) -> Self;
    /// Shifts the bits to the right by a specified amount, `n`, filling
    /// zeros in the most significant bits.
    ///
    /// This is bitwise equivalent to unsigned `Shr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = -8i8; // 0b11111000
    /// let m = 62i8; // 0b00111110
    ///
    /// assert_eq!(n.unsigned_shr(2), m);
    /// ```
    fn unsigned_shr(self, n: u32) -> Self;
    /// Reverses the byte order of the integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    /// let m = 0xEFCDAB8967452301u64;
    ///
    /// assert_eq!(n.swap_bytes(), m);
    /// ```
    fn swap_bytes(self) -> Self;
    /// Reverses the order of bits in the integer.
    ///
    /// The least significant bit becomes the most significant bit, second least-significant bit
    /// becomes second most-significant bit, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x12345678u32;
    /// let m = 0x1e6a2c48u32;
    ///
    /// assert_eq!(n.reverse_bits(), m);
    /// assert_eq!(0u32.reverse_bits(), 0);
    /// ```
    fn reverse_bits(self) -> Self {
        reverse_bits_fallback(self)
    }
    /// Convert an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(u64::from_be(n), n)
    /// } else {
    ///     assert_eq!(u64::from_be(n), n.swap_bytes())
    /// }
    /// ```
    fn from_be(x: Self) -> Self;
    /// Convert an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(u64::from_le(n), n)
    /// } else {
    ///     assert_eq!(u64::from_le(n), n.swap_bytes())
    /// }
    /// ```
    fn from_le(x: Self) -> Self;
    /// Convert `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.to_be(), n)
    /// } else {
    ///     assert_eq!(n.to_be(), n.swap_bytes())
    /// }
    /// ```
    fn to_be(self) -> Self;
    /// Convert `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// let n = 0x0123456789ABCDEFu64;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.to_le(), n)
    /// } else {
    ///     assert_eq!(n.to_le(), n.swap_bytes())
    /// }
    /// ```
    fn to_le(self) -> Self;
    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_traits::PrimInt;
    ///
    /// assert_eq!(2i32.pow(4), 16);
    /// ```
    fn pow(self, exp: u32) -> Self;
}
fn one_per_byte<P: PrimInt>() -> P {
    let mut ret = P::one();
    let mut shift = 8;
    let mut b = ret.count_zeros() >> 3;
    while b != 0 {
        ret = (ret << shift) | ret;
        shift <<= 1;
        b >>= 1;
    }
    ret
}
fn reverse_bits_fallback<P: PrimInt>(i: P) -> P {
    let rep_01: P = one_per_byte();
    let rep_03 = (rep_01 << 1) | rep_01;
    let rep_05 = (rep_01 << 2) | rep_01;
    let rep_0f = (rep_03 << 2) | rep_03;
    let rep_33 = (rep_03 << 4) | rep_03;
    let rep_55 = (rep_05 << 4) | rep_05;
    let mut ret = i.swap_bytes();
    ret = ((ret & rep_0f) << 4) | ((ret >> 4) & rep_0f);
    ret = ((ret & rep_33) << 2) | ((ret >> 2) & rep_33);
    ret = ((ret & rep_55) << 1) | ((ret >> 1) & rep_55);
    ret
}
macro_rules! prim_int_impl {
    ($T:ty, $S:ty, $U:ty) => {
        impl PrimInt for $T { #[inline] fn count_ones(self) -> u32 { <$T
        >::count_ones(self) } #[inline] fn count_zeros(self) -> u32 { <$T
        >::count_zeros(self) } #[cfg(has_leading_trailing_ones)] #[inline] fn
        leading_ones(self) -> u32 { <$T >::leading_ones(self) } #[inline] fn
        leading_zeros(self) -> u32 { <$T >::leading_zeros(self) }
        #[cfg(has_leading_trailing_ones)] #[inline] fn trailing_ones(self) -> u32 { <$T
        >::trailing_ones(self) } #[inline] fn trailing_zeros(self) -> u32 { <$T
        >::trailing_zeros(self) } #[inline] fn rotate_left(self, n : u32) -> Self { <$T
        >::rotate_left(self, n) } #[inline] fn rotate_right(self, n : u32) -> Self { <$T
        >::rotate_right(self, n) } #[inline] fn signed_shl(self, n : u32) -> Self {
        ((self as $S) << n) as $T } #[inline] fn signed_shr(self, n : u32) -> Self {
        ((self as $S) >> n) as $T } #[inline] fn unsigned_shl(self, n : u32) -> Self {
        ((self as $U) << n) as $T } #[inline] fn unsigned_shr(self, n : u32) -> Self {
        ((self as $U) >> n) as $T } #[inline] fn swap_bytes(self) -> Self { <$T
        >::swap_bytes(self) } #[cfg(has_reverse_bits)] #[inline] fn reverse_bits(self) ->
        Self { <$T >::reverse_bits(self) } #[inline] fn from_be(x : Self) -> Self { <$T
        >::from_be(x) } #[inline] fn from_le(x : Self) -> Self { <$T >::from_le(x) }
        #[inline] fn to_be(self) -> Self { <$T >::to_be(self) } #[inline] fn to_le(self)
        -> Self { <$T >::to_le(self) } #[inline] fn pow(self, exp : u32) -> Self { <$T
        >::pow(self, exp) } }
    };
}
prim_int_impl!(u8, i8, u8);
prim_int_impl!(u16, i16, u16);
prim_int_impl!(u32, i32, u32);
prim_int_impl!(u64, i64, u64);
prim_int_impl!(u128, i128, u128);
prim_int_impl!(usize, isize, usize);
prim_int_impl!(i8, i8, u8);
prim_int_impl!(i16, i16, u16);
prim_int_impl!(i32, i32, u32);
prim_int_impl!(i64, i64, u64);
prim_int_impl!(i128, i128, u128);
prim_int_impl!(isize, isize, usize);
#[cfg(test)]
mod tests {
    use crate::int::PrimInt;
    #[test]
    pub fn reverse_bits() {
        use core::{i16, i32, i64, i8};
        assert_eq!(
            PrimInt::reverse_bits(0x0123_4567_89ab_cdefu64), 0xf7b3_d591_e6a2_c480
        );
        assert_eq!(PrimInt::reverse_bits(0i8), 0);
        assert_eq!(PrimInt::reverse_bits(- 1i8), - 1);
        assert_eq!(PrimInt::reverse_bits(1i8), i8::MIN);
        assert_eq!(PrimInt::reverse_bits(i8::MIN), 1);
        assert_eq!(PrimInt::reverse_bits(- 2i8), i8::MAX);
        assert_eq!(PrimInt::reverse_bits(i8::MAX), - 2);
        assert_eq!(PrimInt::reverse_bits(0i16), 0);
        assert_eq!(PrimInt::reverse_bits(- 1i16), - 1);
        assert_eq!(PrimInt::reverse_bits(1i16), i16::MIN);
        assert_eq!(PrimInt::reverse_bits(i16::MIN), 1);
        assert_eq!(PrimInt::reverse_bits(- 2i16), i16::MAX);
        assert_eq!(PrimInt::reverse_bits(i16::MAX), - 2);
        assert_eq!(PrimInt::reverse_bits(0i32), 0);
        assert_eq!(PrimInt::reverse_bits(- 1i32), - 1);
        assert_eq!(PrimInt::reverse_bits(1i32), i32::MIN);
        assert_eq!(PrimInt::reverse_bits(i32::MIN), 1);
        assert_eq!(PrimInt::reverse_bits(- 2i32), i32::MAX);
        assert_eq!(PrimInt::reverse_bits(i32::MAX), - 2);
        assert_eq!(PrimInt::reverse_bits(0i64), 0);
        assert_eq!(PrimInt::reverse_bits(- 1i64), - 1);
        assert_eq!(PrimInt::reverse_bits(1i64), i64::MIN);
        assert_eq!(PrimInt::reverse_bits(i64::MIN), 1);
        assert_eq!(PrimInt::reverse_bits(- 2i64), i64::MAX);
        assert_eq!(PrimInt::reverse_bits(i64::MAX), - 2);
    }
    #[test]
    pub fn reverse_bits_i128() {
        use core::i128;
        assert_eq!(PrimInt::reverse_bits(0i128), 0);
        assert_eq!(PrimInt::reverse_bits(- 1i128), - 1);
        assert_eq!(PrimInt::reverse_bits(1i128), i128::MIN);
        assert_eq!(PrimInt::reverse_bits(i128::MIN), 1);
        assert_eq!(PrimInt::reverse_bits(- 2i128), i128::MAX);
        assert_eq!(PrimInt::reverse_bits(i128::MAX), - 2);
    }
}
#[cfg(test)]
mod tests_llm_16_649_llm_16_649 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_649_llm_16_649_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 1i128;
        let rug_fuzz_3 = 0b1010i128;
        let rug_fuzz_4 = 0b1111_1111_1111_1111_1111_1111_1111_1111i128;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!((- rug_fuzz_2).count_ones(), 128);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 32);
        debug_assert_eq!(i128::MAX.count_ones(), 127);
        debug_assert_eq!(i128::MIN.count_ones(), 1);
        let _rug_ed_tests_llm_16_649_llm_16_649_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_650 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_650_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0b1000;
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(rug_fuzz_0), 128);
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(rug_fuzz_1), 127);
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(rug_fuzz_2), 126);
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(rug_fuzz_3), 124);
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(i128::MAX), 0);
        debug_assert_eq!(< i128 as int::PrimInt > ::count_zeros(i128::MIN), 0);
        let _rug_ed_tests_llm_16_650_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_651_llm_16_651 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x12_34_56_78_90_AB_CD_EF_i128;
        let big_endian_bytes = rug_fuzz_0.to_be();
        let value = <i128 as PrimInt>::from_be(big_endian_bytes);
        debug_assert_eq!(value, 0x12_34_56_78_90_AB_CD_EF_i128);
        let _rug_ed_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be = 0;
    }
    #[test]
    fn test_from_be_zero() {
        let _rug_st_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be_zero = 0;
        let rug_fuzz_0 = 0_i128;
        let zero_be = rug_fuzz_0.to_be();
        let value = <i128 as PrimInt>::from_be(zero_be);
        debug_assert_eq!(value, 0_i128);
        let _rug_ed_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be_zero = 0;
    }
    #[test]
    fn test_from_be_neg() {
        let _rug_st_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be_neg = 0;
        let rug_fuzz_0 = 0x12_34_56_78_90_AB_CD_EF_i128;
        let big_endian_bytes = (-rug_fuzz_0).to_be();
        let value = <i128 as PrimInt>::from_be(big_endian_bytes);
        debug_assert_eq!(value, - 0x12_34_56_78_90_AB_CD_EF_i128);
        let _rug_ed_tests_llm_16_651_llm_16_651_rrrruuuugggg_test_from_be_neg = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_652_llm_16_652 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_le() {
        let _rug_st_tests_llm_16_652_llm_16_652_rrrruuuugggg_test_from_le = 0;
        let rug_fuzz_0 = 0x1234_5678_90AB_CDEF_i128;
        let rug_fuzz_1 = 0x1234_5678_90AB_CDEF_i128;
        let little_endian_value = rug_fuzz_0.to_le();
        let result = i128::from_le(little_endian_value);
        debug_assert_eq!(rug_fuzz_1, result);
        let _rug_ed_tests_llm_16_652_llm_16_652_rrrruuuugggg_test_from_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_653 {
    use super::*;
    use crate::*;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_653_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 1i128;
        let rug_fuzz_3 = 127;
        let rug_fuzz_4 = 0i128;
        let rug_fuzz_5 = 64;
        debug_assert_eq!(< i128 as PrimInt > ::leading_ones(rug_fuzz_0), 0);
        debug_assert_eq!(< i128 as PrimInt > ::leading_ones(- rug_fuzz_1), 128);
        debug_assert_eq!(
            < i128 as PrimInt > ::leading_ones(rug_fuzz_2 << rug_fuzz_3), 1
        );
        debug_assert_eq!(
            < i128 as PrimInt > ::leading_ones(! rug_fuzz_4 << rug_fuzz_5), 64
        );
        let _rug_ed_tests_llm_16_653_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_654_llm_16_654 {
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_654_llm_16_654_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 0x7f_ff_ff_ff_ff_ff_ff_ff;
        let rug_fuzz_6 = 0x80_00_00_00_00_00_00_00;
        let rug_fuzz_7 = 0x80_00_00_00_00_00_00_01;
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_0), 128);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_1), 127);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_2), 126);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_3), 126);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_4), 125);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_5), 1);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_6), 0);
        debug_assert_eq!(< i128 as PrimInt > ::leading_zeros(rug_fuzz_7), 0);
        let _rug_ed_tests_llm_16_654_llm_16_654_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_655_llm_16_655 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_655_llm_16_655_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 10;
        debug_assert_eq!(< i128 as PrimInt > ::pow(rug_fuzz_0, rug_fuzz_1), 8);
        debug_assert_eq!(< i128 as PrimInt > ::pow(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(< i128 as PrimInt > ::pow(- rug_fuzz_4, rug_fuzz_5), - 8);
        debug_assert_eq!(< i128 as PrimInt > ::pow(- rug_fuzz_6, rug_fuzz_7), 4);
        debug_assert_eq!(< i128 as PrimInt > ::pow(rug_fuzz_8, rug_fuzz_9), 1);
        debug_assert_eq!(< i128 as PrimInt > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< i128 as PrimInt > ::pow(rug_fuzz_12, rug_fuzz_13), 0);
        let _rug_ed_tests_llm_16_655_llm_16_655_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_656 {
    use super::*;
    use crate::*;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_656_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b0000000000000000000000000000000000000000000000000000000000000010;
        let rug_fuzz_1 = 0b0100000000000000000000000000000000000000000000000000000000000000;
        let rug_fuzz_2 = 0b0000000000000000000000000000000000000000000000000000000000000000;
        let rug_fuzz_3 = 0b0000000000000000000000000000000000000000000000000000000000000000;
        let rug_fuzz_4 = 0b1111111111111111111111111111111111111111111111111111111111111111;
        let rug_fuzz_5 = 0b1111111111111111111111111111111111111111111111111111111111111111;
        let x: i128 = rug_fuzz_0;
        let expected: i128 = rug_fuzz_1;
        debug_assert_eq!(x.reverse_bits(), expected);
        let y: i128 = rug_fuzz_2;
        let expected_y: i128 = rug_fuzz_3;
        debug_assert_eq!(y.reverse_bits(), expected_y);
        let z: i128 = rug_fuzz_4;
        let expected_z: i128 = rug_fuzz_5;
        debug_assert_eq!(z.reverse_bits(), expected_z);
        let _rug_ed_tests_llm_16_656_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_657_llm_16_657 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_657_llm_16_657_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_0010_0011_0100_0101_0110_0111_1000_1001_1010_1011_1100_1101_1110_1111_0000;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 7;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 128;
        let rug_fuzz_7 = 127;
        let rug_fuzz_8 = 127;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 7;
        let value: i128 = rug_fuzz_0;
        debug_assert_eq!(value.rotate_left(rug_fuzz_1), value);
        let expected1 = value.rotate_left(rug_fuzz_2);
        debug_assert_eq!(value.rotate_left(rug_fuzz_3), expected1);
        let expected7 = value.rotate_left(rug_fuzz_4);
        debug_assert_eq!(value.rotate_left(rug_fuzz_5), expected7);
        debug_assert_eq!(value.rotate_left(rug_fuzz_6), value);
        let expected127 = value.rotate_left(rug_fuzz_7);
        debug_assert_eq!(value.rotate_left(rug_fuzz_8), expected127);
        let zero: i128 = rug_fuzz_9;
        debug_assert_eq!(zero.rotate_left(rug_fuzz_10), 0);
        let _rug_ed_tests_llm_16_657_llm_16_657_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_658_llm_16_658 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_658_llm_16_658_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 1i128;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1i128;
        let rug_fuzz_3 = 127;
        let rug_fuzz_4 = 1i128;
        let rug_fuzz_5 = 128;
        let rug_fuzz_6 = 1i128;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 0i128;
        let rug_fuzz_9 = 64;
        debug_assert_eq!(
            < i128 as PrimInt > ::rotate_right(rug_fuzz_0, rug_fuzz_1), 1i128
        );
        debug_assert_eq!(
            < i128 as PrimInt > ::rotate_right(rug_fuzz_2, rug_fuzz_3), 2i128
        );
        debug_assert_eq!(
            < i128 as PrimInt > ::rotate_right(rug_fuzz_4, rug_fuzz_5), 1i128
        );
        debug_assert_eq!(
            < i128 as PrimInt > ::rotate_right(- rug_fuzz_6, rug_fuzz_7), i128::MAX
        );
        debug_assert_eq!(
            < i128 as PrimInt > ::rotate_right(rug_fuzz_8, rug_fuzz_9), 0i128
        );
        let _rug_ed_tests_llm_16_658_llm_16_658_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_659_llm_16_659 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_659_llm_16_659_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 127;
        let rug_fuzz_4 = 1;
        let value: i128 = rug_fuzz_0;
        let result = <i128 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 1);
        let result = <i128 as PrimInt>::signed_shl(value, rug_fuzz_2);
        debug_assert_eq!(result, 2);
        let result = <i128 as PrimInt>::signed_shl(value, rug_fuzz_3);
        debug_assert_eq!(result, i128::min_value());
        let overflow_value: i128 = i128::max_value();
        let result = <i128 as PrimInt>::signed_shl(overflow_value, rug_fuzz_4);
        debug_assert_eq!(result, - 2);
        let _rug_ed_tests_llm_16_659_llm_16_659_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_660 {
    use crate::int::PrimInt;
    #[test]
    fn signed_shr_test() {
        let _rug_st_tests_llm_16_660_rrrruuuugggg_signed_shr_test = 0;
        let rug_fuzz_0 = 0x123456789abcdef0;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0x0123456789abcde;
        let rug_fuzz_3 = 0x123456789abcdef0;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 0x0123456789abcdef;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 0x123456789abcdef0;
        let rug_fuzz_17 = 128;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0x123456789abcdef0;
        let rug_fuzz_20 = 128;
        let rug_fuzz_21 = 1;
        let a: i128 = -rug_fuzz_0;
        let b: i128 = a.signed_shr(rug_fuzz_1);
        let expected: i128 = -rug_fuzz_2;
        debug_assert_eq!(b, expected);
        let a: i128 = rug_fuzz_3;
        let b: i128 = a.signed_shr(rug_fuzz_4);
        let expected: i128 = rug_fuzz_5;
        debug_assert_eq!(b, expected);
        let a: i128 = i128::MAX;
        let b: i128 = a.signed_shr(rug_fuzz_6);
        let expected: i128 = i128::MAX >> rug_fuzz_7;
        debug_assert_eq!(b, expected);
        let a: i128 = i128::MIN;
        let b: i128 = a.signed_shr(rug_fuzz_8);
        let expected: i128 = i128::MIN >> rug_fuzz_9;
        debug_assert_eq!(b, expected);
        let a: i128 = rug_fuzz_10;
        let b: i128 = a.signed_shr(rug_fuzz_11);
        let expected: i128 = rug_fuzz_12;
        debug_assert_eq!(b, expected);
        let a: i128 = -rug_fuzz_13;
        let b: i128 = a.signed_shr(rug_fuzz_14);
        let expected: i128 = -rug_fuzz_15;
        debug_assert_eq!(b, expected);
        let a: i128 = rug_fuzz_16;
        let b: i128 = a.signed_shr(rug_fuzz_17);
        let expected: i128 = rug_fuzz_18;
        debug_assert_eq!(b, expected);
        let a: i128 = -rug_fuzz_19;
        let b: i128 = a.signed_shr(rug_fuzz_20);
        let expected: i128 = -rug_fuzz_21;
        debug_assert_eq!(b, expected);
        let _rug_ed_tests_llm_16_660_rrrruuuugggg_signed_shr_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_661_llm_16_661 {
    use crate::int::PrimInt;
    #[test]
    fn test_swap_bytes_i128() {
        let _rug_st_tests_llm_16_661_llm_16_661_rrrruuuugggg_test_swap_bytes_i128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0x0123456789abcdef0123456789abcdef;
        let rug_fuzz_5 = 0x0123456789abcdef0123456789abcdef;
        let rug_fuzz_6 = 0x0123456789abcdef0123456789abcdef;
        let num: i128 = rug_fuzz_0;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), num);
        let num: i128 = rug_fuzz_1;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), num.swap_bytes());
        let num: i128 = (rug_fuzz_2 as i128).swap_bytes();
        let expected: i128 = rug_fuzz_3;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), expected);
        let num: i128 = i128::MAX;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), num.swap_bytes());
        let num: i128 = i128::MAX.swap_bytes();
        let expected: i128 = i128::MAX;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), expected);
        let num: i128 = i128::MIN;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), num.swap_bytes());
        let num: i128 = i128::MIN.swap_bytes();
        let expected: i128 = i128::MIN;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), expected);
        let num: i128 = rug_fuzz_4;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), num.swap_bytes());
        let num: i128 = rug_fuzz_5.swap_bytes();
        let expected: i128 = rug_fuzz_6;
        debug_assert_eq!(< i128 as PrimInt > ::swap_bytes(num), expected);
        let _rug_ed_tests_llm_16_661_llm_16_661_rrrruuuugggg_test_swap_bytes_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_662 {
    use crate::PrimInt;
    #[test]
    fn test_to_be() {
        let num: i128 = 0x1234567890ABCDEFi128;
        let big_endian_num = num.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(num, big_endian_num);
        } else if cfg!(target_endian = "little") {
            let bytes = num.to_be_bytes();
            let expected_num = i128::from_be_bytes(bytes);
            assert_eq!(expected_num, big_endian_num);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_663_llm_16_663 {
    use super::*;
    use crate::*;
    use crate::int::PrimInt;
    #[test]
    fn test_to_le() {
        if cfg!(target_endian = "big") {
            assert_eq!((0x0123456789ABCDEFi128).to_le(), 0xEFCDAB8967452301i128);
        } else {
            assert_eq!((0x0123456789ABCDEFi128).to_le(), 0x0123456789ABCDEFi128);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_664_llm_16_664 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_664_llm_16_664_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 3i128;
        let rug_fuzz_4 = 4i128;
        let rug_fuzz_5 = 0b101100i128;
        let rug_fuzz_6 = 1i128;
        let rug_fuzz_7 = 4i128;
        let rug_fuzz_8 = 8i128;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 2);
        debug_assert_eq!(i128::MAX.trailing_ones(), 0);
        debug_assert_eq!((- rug_fuzz_6).trailing_ones(), 128);
        debug_assert_eq!((- rug_fuzz_7).trailing_ones(), 2);
        debug_assert_eq!((- rug_fuzz_8).trailing_ones(), 3);
        let _rug_ed_tests_llm_16_664_llm_16_664_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_665 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_665_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 2i128;
        let rug_fuzz_3 = 16i128;
        let rug_fuzz_4 = 1024i128;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 10);
        debug_assert_eq!(i128::MAX.trailing_zeros(), 0);
        debug_assert_eq!(i128::MIN.trailing_zeros(), 127);
        let _rug_ed_tests_llm_16_665_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_666_llm_16_666 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_llm_16_666_llm_16_666_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 0x0F0F0F0F0F0F0F0F;
        let rug_fuzz_1 = 4;
        let value: i128 = rug_fuzz_0;
        let shifted = <i128 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(shifted, 0xF0F0F0F0F0F0F0F0);
        let _rug_ed_tests_llm_16_666_llm_16_666_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_667_llm_16_667 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shr() {
        let _rug_st_tests_llm_16_667_llm_16_667_rrrruuuugggg_test_unsigned_shr = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 12345;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 128u32;
        let rug_fuzz_6 = 12345;
        let value: i128 = -rug_fuzz_0;
        let shr_amount: u32 = rug_fuzz_1;
        let result = <i128 as PrimInt>::unsigned_shr(value, shr_amount);
        let expected = i128::max_value() >> shr_amount;
        debug_assert_eq!(result, expected, "unsigned_shr did not shift correctly");
        let zero_value: i128 = rug_fuzz_2;
        let result_zero = <i128 as PrimInt>::unsigned_shr(zero_value, shr_amount);
        debug_assert_eq!(result_zero, 0, "unsigned_shr did not shift zero correctly");
        let max_value = i128::max_value();
        let result_max = <i128 as PrimInt>::unsigned_shr(max_value, shr_amount);
        debug_assert_eq!(
            result_max, i128::max_value() >> shr_amount,
            "unsigned_shr did not shift max i128 value correctly"
        );
        let min_value = i128::min_value();
        let result_min = <i128 as PrimInt>::unsigned_shr(min_value, shr_amount);
        let expected_min = (i128::min_value() as u128 >> shr_amount) as i128;
        debug_assert_eq!(
            result_min, expected_min,
            "unsigned_shr did not shift min i128 value correctly"
        );
        let no_op_value: i128 = rug_fuzz_3;
        let result_no_op = <i128 as PrimInt>::unsigned_shr(no_op_value, rug_fuzz_4);
        debug_assert_eq!(
            result_no_op, no_op_value,
            "unsigned_shr with 0 shift did not result in the same value"
        );
        let max_shift_amount = rug_fuzz_5;
        let non_zero_value: i128 = rug_fuzz_6;
        let result_max_shift = <i128 as PrimInt>::unsigned_shr(
            non_zero_value,
            max_shift_amount,
        );
        debug_assert_eq!(
            result_max_shift, 0, "unsigned_shr with max shift did not result in zero"
        );
        let _rug_ed_tests_llm_16_667_llm_16_667_rrrruuuugggg_test_unsigned_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_759_llm_16_759 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_759_llm_16_759_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 0b1010i16;
        let rug_fuzz_4 = 0b1111i16;
        let rug_fuzz_5 = 1i16;
        let rug_fuzz_6 = 14;
        let rug_fuzz_7 = 1i16;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 4);
        debug_assert_eq!((rug_fuzz_5 << rug_fuzz_6).count_ones(), 1);
        debug_assert_eq!((- rug_fuzz_7).count_ones(), 16);
        let _rug_ed_tests_llm_16_759_llm_16_759_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_760_llm_16_760 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_zeros_for_i16() {
        let _rug_st_tests_llm_16_760_llm_16_760_rrrruuuugggg_test_count_zeros_for_i16 = 0;
        let rug_fuzz_0 = 0b0000_0000_0000_0000i16;
        let rug_fuzz_1 = 0b0000_0000_0000_0001i16;
        let rug_fuzz_2 = 0b0000_0000_0000_0010i16;
        let rug_fuzz_3 = 0b0000_0000_0000_0100i16;
        let rug_fuzz_4 = 0b0000_0000_0000_1000i16;
        let rug_fuzz_5 = 0b0000_0000_0001_0000i16;
        let rug_fuzz_6 = 0b0000_0000_0010_0000i16;
        let rug_fuzz_7 = 0b0000_0000_0100_0000i16;
        let rug_fuzz_8 = 0b0000_0000_1000_0000i16;
        let rug_fuzz_9 = 0b0000_0001_0000_0000i16;
        let rug_fuzz_10 = 0b0000_0010_0000_0000i16;
        let rug_fuzz_11 = 0b0000_0100_0000_0000i16;
        let rug_fuzz_12 = 0b0000_1000_0000_0000i16;
        let rug_fuzz_13 = 0b0001_0000_0000_0000i16;
        let rug_fuzz_14 = 0b0010_0000_0000_0000i16;
        let rug_fuzz_15 = 0b0100_0000_0000_0000i16;
        let rug_fuzz_16 = 2;
        debug_assert_eq!(i16::count_zeros(rug_fuzz_0), 16);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_1), 15);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_2), 14);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_3), 13);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_4), 12);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_5), 11);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_6), 10);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_7), 9);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_8), 8);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_9), 7);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_10), 6);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_11), 5);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_12), 4);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_13), 3);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_14), 2);
        debug_assert_eq!(i16::count_zeros(rug_fuzz_15), 1);
        debug_assert_eq!(i16::count_zeros(i16::MIN), 0);
        debug_assert_eq!(i16::count_zeros(- rug_fuzz_16), 0);
        let _rug_ed_tests_llm_16_760_llm_16_760_rrrruuuugggg_test_count_zeros_for_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_761 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_be() {
        assert_eq!(i16::from_be(0x0102), i16::from_be_bytes([0x01, 0x02]));
        if cfg!(target_endian = "big") {
            assert_eq!(i16::from_be(0x0102), 0x0102);
        } else {
            assert_eq!(i16::from_be(0x0102), 0x0201);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_762_llm_16_762 {
    #[test]
    fn test_from_le() {
        let _rug_st_tests_llm_16_762_llm_16_762_rrrruuuugggg_test_from_le = 0;
        let rug_fuzz_0 = 0x1234;
        let rug_fuzz_1 = 0x1234;
        debug_assert_eq!(i16::from_le(rug_fuzz_0), 0x1234);
        debug_assert_eq!(i16::from_le(- rug_fuzz_1), - 0x1234);
        let _rug_ed_tests_llm_16_762_llm_16_762_rrrruuuugggg_test_from_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_763_llm_16_763 {
    use super::*;
    use crate::*;
    use crate::*;
    #[test]
    #[cfg(has_leading_trailing_ones)]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_763_llm_16_763_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 1i16;
        let rug_fuzz_3 = 0b0111_1111_1111_1111i16;
        let rug_fuzz_4 = 0x80;
        let rug_fuzz_5 = 0x00;
        let rug_fuzz_6 = 0xff;
        let rug_fuzz_7 = 0x00;
        let rug_fuzz_8 = 0xff;
        let rug_fuzz_9 = 0xff;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 16);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 15);
        debug_assert_eq!((- rug_fuzz_2).leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 1);
        debug_assert_eq!(i16::from_ne_bytes([rug_fuzz_4, rug_fuzz_5]).leading_ones(), 0);
        debug_assert_eq!(i16::from_ne_bytes([rug_fuzz_6, rug_fuzz_7]).leading_ones(), 0);
        debug_assert_eq!(i16::from_ne_bytes([rug_fuzz_8, rug_fuzz_9]).leading_ones(), 0);
        let _rug_ed_tests_llm_16_763_llm_16_763_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_765_llm_16_765 {
    use super::*;
    use crate::*;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_765_llm_16_765_rrrruuuugggg_test_pow = 0;
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
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 3;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 2;
        let rug_fuzz_24 = 2;
        let rug_fuzz_25 = 14;
        debug_assert_eq!(i16::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(i16::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(i16::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(i16::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(i16::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(i16::pow(- rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(i16::pow(- rug_fuzz_12, rug_fuzz_13), - 2);
        debug_assert_eq!(i16::pow(- rug_fuzz_14, rug_fuzz_15), 4);
        debug_assert_eq!(i16::pow(- rug_fuzz_16, rug_fuzz_17), - 8);
        debug_assert_eq!(i16::pow(rug_fuzz_18, rug_fuzz_19), 1);
        debug_assert_eq!(i16::pow(rug_fuzz_20, rug_fuzz_21), 0);
        debug_assert_eq!(i16::pow(rug_fuzz_22, rug_fuzz_23), 0);
        debug_assert_eq!(i16::pow(rug_fuzz_24, rug_fuzz_25), - 32768);
        let _rug_ed_tests_llm_16_765_llm_16_765_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_769_llm_16_769 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl_i16_positive() {
        let _rug_st_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_positive = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000;
        let rug_fuzz_1 = 1;
        let value: i16 = rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b0010_0000_0000_0000);
        let _rug_ed_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_positive = 0;
    }
    #[test]
    fn test_signed_shl_i16_negative() {
        let _rug_st_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_negative = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000;
        let rug_fuzz_1 = 1;
        let value: i16 = -rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, - 0b0010_0000_0000_0000);
        let _rug_ed_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_negative = 0;
    }
    #[test]
    fn test_signed_shl_i16_overflow() {
        let _rug_st_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_overflow = 0;
        let rug_fuzz_0 = 0b0111_0000_0000_0000;
        let rug_fuzz_1 = 2;
        let value: i16 = rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, - 0b1000_0000_0000_0000);
        let _rug_ed_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_overflow = 0;
    }
    #[test]
    fn test_signed_shl_i16_shift_by_0() {
        let _rug_st_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_shift_by_0 = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000;
        let rug_fuzz_1 = 0;
        let value: i16 = rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_769_llm_16_769_rrrruuuugggg_test_signed_shl_i16_shift_by_0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_770_llm_16_770 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shr_positive() {
        let _rug_st_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_positive = 0;
        let rug_fuzz_0 = 0b0100_0001;
        let rug_fuzz_1 = 2;
        let value: i16 = rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, 16);
        let _rug_ed_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_positive = 0;
    }
    #[test]
    fn test_signed_shr_negative() {
        let _rug_st_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_negative = 0;
        let rug_fuzz_0 = 0b0100_0001;
        let rug_fuzz_1 = 2;
        let value: i16 = -rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, - 0b0001_0001);
        let _rug_ed_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_negative = 0;
    }
    #[test]
    fn test_signed_shr_zero() {
        let _rug_st_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2;
        let value: i16 = rug_fuzz_0;
        let result = <i16 as PrimInt>::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, 0);
        let _rug_ed_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_zero = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to shift right with overflow")]
    fn test_signed_shr_overflow() {
        let _rug_st_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_overflow = 0;
        let rug_fuzz_0 = 0b0100_0001;
        let rug_fuzz_1 = 16;
        let value: i16 = rug_fuzz_0;
        <i16 as PrimInt>::signed_shr(value, rug_fuzz_1);
        let _rug_ed_tests_llm_16_770_llm_16_770_rrrruuuugggg_test_signed_shr_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_771_llm_16_771 {
    use crate::PrimInt;
    #[test]
    fn test_swap_bytes_i16() {
        let _rug_st_tests_llm_16_771_llm_16_771_rrrruuuugggg_test_swap_bytes_i16 = 0;
        let rug_fuzz_0 = 0x1234;
        let x: i16 = rug_fuzz_0;
        let swapped = i16::swap_bytes(x);
        debug_assert_eq!(swapped, 0x3412);
        let _rug_ed_tests_llm_16_771_llm_16_771_rrrruuuugggg_test_swap_bytes_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_772 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let big_endian = if cfg!(target_endian = "big") { true } else { false };
        let num: i16 = 0x1234;
        let big_endian_num = num.to_be();
        if big_endian {
            assert_eq!(big_endian_num, num);
        } else {
            assert_eq!(big_endian_num, 0x3412);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_773_llm_16_773 {
    use crate::PrimInt;
    #[test]
    fn test_to_le() {
        let _rug_st_tests_llm_16_773_llm_16_773_rrrruuuugggg_test_to_le = 0;
        let rug_fuzz_0 = 0x1234_i16;
        let big_endian = rug_fuzz_0.to_be();
        let little_endian = big_endian.to_le();
        #[cfg(target_endian = "big")] debug_assert_eq!(little_endian, big_endian);
        #[cfg(target_endian = "little")] debug_assert_eq!(little_endian, 0x1234_i16);
        let _rug_ed_tests_llm_16_773_llm_16_773_rrrruuuugggg_test_to_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_774 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_774_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 3i16;
        let rug_fuzz_4 = 4i16;
        let rug_fuzz_5 = 8i16;
        let rug_fuzz_6 = 0b101100i16;
        let rug_fuzz_7 = 1i16;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 2);
        debug_assert_eq!((- rug_fuzz_7).trailing_ones(), 16);
        debug_assert_eq!(i16::MAX.trailing_ones(), 0);
        debug_assert_eq!(i16::MIN.trailing_ones(), 0);
        let _rug_ed_tests_llm_16_774_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_775_llm_16_775 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_775_llm_16_775_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 2i16;
        let rug_fuzz_3 = 4i16;
        let rug_fuzz_4 = 8i16;
        let rug_fuzz_5 = 16i16;
        let rug_fuzz_6 = 1024i16;
        let rug_fuzz_7 = 1024i16;
        let rug_fuzz_8 = 1i16;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 10);
        debug_assert_eq!((- rug_fuzz_7).trailing_zeros(), 10);
        debug_assert_eq!((- rug_fuzz_8).trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_775_llm_16_775_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_869 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_869_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 1i32;
        let rug_fuzz_2 = 2i32;
        let rug_fuzz_3 = 3i32;
        let rug_fuzz_4 = 1i32;
        let rug_fuzz_5 = 0b1010i32;
        let rug_fuzz_6 = 0b1111i32;
        let rug_fuzz_7 = 0b10000000i32;
        let rug_fuzz_8 = 0b01111111i32;
        let rug_fuzz_9 = 0x12345678i32;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 2);
        debug_assert_eq!((- rug_fuzz_4).count_ones(), 32);
        debug_assert_eq!(rug_fuzz_5.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_6.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_7.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_8.count_ones(), 7);
        debug_assert_eq!(rug_fuzz_9.count_ones(), 13);
        let _rug_ed_tests_llm_16_869_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_870_llm_16_870 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        assert_eq!(< i32 as int::PrimInt >::count_zeros(0b00000000), 32);
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(0b00000000000000000000000000000001), 31
        );
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(- 0b10000000000000000000000000000000), 0
        );
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(0b01010000000000000000000000000000), 1
        );
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(0b00101000000000000000000000000000), 2
        );
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(0b00100000000000000000000000000000), 2
        );
        assert_eq!(
            < i32 as int::PrimInt >::count_zeros(0b00010000000000000000000000000000), 3
        );
        assert_eq!(< i32 as int::PrimInt >::count_zeros(- 1), 0);
    }
}
#[cfg(test)]
mod tests_llm_16_871_llm_16_871 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_871_llm_16_871_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x12345678_i32;
        let big_endian_bytes = rug_fuzz_0.to_be();
        let value = <i32 as PrimInt>::from_be(big_endian_bytes);
        debug_assert_eq!(value, 0x12345678_i32);
        let _rug_ed_tests_llm_16_871_llm_16_871_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_872_llm_16_872 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        if cfg!(target_endian = "little") {
            assert_eq!(i32::from_le(0x12345678), 0x12345678);
        } else {
            assert_eq!(i32::from_le(0x12345678), 0x78563412);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_873 {
    use crate::PrimInt;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_873_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 1i32;
        let rug_fuzz_2 = 1i32;
        let rug_fuzz_3 = 0b0001_0000i32;
        let rug_fuzz_4 = 0b1000_0000i32;
        let rug_fuzz_5 = 0b1100_0000i32;
        let rug_fuzz_6 = 0b1110_0000i32;
        let rug_fuzz_7 = 0b1111_0000i32;
        let rug_fuzz_8 = 0b1111_1000i32;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 31);
        debug_assert_eq!((- rug_fuzz_2).leading_ones(), 32);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 1);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 2);
        debug_assert_eq!(rug_fuzz_6.leading_ones(), 3);
        debug_assert_eq!(rug_fuzz_7.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_8.leading_ones(), 5);
        let _rug_ed_tests_llm_16_873_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_874_llm_16_874 {
    use crate::int::PrimInt;
    #[test]
    fn leading_zeros_i32() {
        let _rug_st_tests_llm_16_874_llm_16_874_rrrruuuugggg_leading_zeros_i32 = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 1i32;
        let rug_fuzz_2 = 1i32;
        let rug_fuzz_3 = 2i32;
        let rug_fuzz_4 = 0x7fffffff_i32;
        debug_assert_eq!(rug_fuzz_0.leading_zeros(), 32);
        debug_assert_eq!(rug_fuzz_1.leading_zeros(), 31);
        debug_assert_eq!((- rug_fuzz_2).leading_zeros(), 0);
        debug_assert_eq!(rug_fuzz_3.leading_zeros(), 30);
        debug_assert_eq!(rug_fuzz_4.leading_zeros(), 1);
        debug_assert_eq!(i32::MIN.leading_zeros(), 0);
        let _rug_ed_tests_llm_16_874_llm_16_874_rrrruuuugggg_leading_zeros_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_875_llm_16_875 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_875_llm_16_875_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i32;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2i32;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2i32;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2i32;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 0i32;
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = 5i32;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1i32;
        let rug_fuzz_13 = 22;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 8);
        debug_assert_eq!(rug_fuzz_2.pow(rug_fuzz_3), 1);
        debug_assert_eq!((- rug_fuzz_4).pow(rug_fuzz_5), 4);
        debug_assert_eq!((- rug_fuzz_6).pow(rug_fuzz_7), - 8);
        debug_assert_eq!(rug_fuzz_8.pow(rug_fuzz_9), 0);
        debug_assert_eq!(rug_fuzz_10.pow(rug_fuzz_11), 5);
        debug_assert_eq!(rug_fuzz_12.pow(rug_fuzz_13), 1);
        let _rug_ed_tests_llm_16_875_llm_16_875_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_880_llm_16_880 {
    use crate::PrimInt;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_880_llm_16_880_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 16;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 32;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 1;
        let value: i32 = -rug_fuzz_0;
        let shift_amount: u32 = rug_fuzz_1;
        let result = <i32 as PrimInt>::signed_shr(value, shift_amount);
        let expected = -rug_fuzz_2;
        debug_assert_eq!(
            result, expected, "Shifting -16 right by 2 should result in -4."
        );
        let value: i32 = rug_fuzz_3;
        let shift_amount: u32 = rug_fuzz_4;
        let result = <i32 as PrimInt>::signed_shr(value, shift_amount);
        let expected = rug_fuzz_5;
        debug_assert_eq!(result, expected, "Shifting 16 right by 3 should result in 2.");
        let value: i32 = i32::MIN;
        let shift_amount: u32 = rug_fuzz_6;
        let result = <i32 as PrimInt>::signed_shr(value, shift_amount);
        let expected = i32::MIN / rug_fuzz_7;
        debug_assert_eq!(
            result, expected,
            "Shifting i32::MIN right by 1 should result in i32::MIN / 2."
        );
        let value: i32 = rug_fuzz_8;
        let shift_amount: u32 = rug_fuzz_9;
        let result = <i32 as PrimInt>::signed_shr(value, shift_amount);
        let expected = rug_fuzz_10;
        debug_assert_eq!(result, expected, "Shifting 1 right by 32 should result in 1.");
        let value: i32 = -rug_fuzz_11;
        let shift_amount: u32 = rug_fuzz_12;
        let result = <i32 as PrimInt>::signed_shr(value, shift_amount);
        let expected = -rug_fuzz_13;
        debug_assert_eq!(
            result, expected, "Shifting -1 right by 3 should result in -1."
        );
        let _rug_ed_tests_llm_16_880_llm_16_880_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_882_llm_16_882 {
    use crate::int::PrimInt;
    #[test]
    fn test_to_be() {
        let num: i32 = 0x12345678;
        let big_endian_num = num.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(big_endian_num, num);
        } else {
            assert_eq!(big_endian_num, num.swap_bytes());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_883_llm_16_883 {
    use crate::PrimInt;
    #[test]
    fn test_to_le() {
        if cfg!(target_endian = "little") {
            assert_eq!(0x12345678i32.to_le(), 0x12345678i32);
        } else if cfg!(target_endian = "big") {
            assert_eq!(0x12345678i32.to_le(), 0x78563412i32);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_884 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_884_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b1111;
        let rug_fuzz_3 = 0b1000;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 4);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 0);
        debug_assert_eq!(i32::MAX.trailing_ones(), 0);
        debug_assert_eq!((- rug_fuzz_4).trailing_ones(), 32);
        let _rug_ed_tests_llm_16_884_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_885_llm_16_885 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_885_llm_16_885_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0_i32;
        let rug_fuzz_1 = 1_i32;
        let rug_fuzz_2 = 2_i32;
        let rug_fuzz_3 = 4_i32;
        let rug_fuzz_4 = 8_i32;
        let rug_fuzz_5 = 16_i32;
        let rug_fuzz_6 = 16_i32;
        let rug_fuzz_7 = 0b01010000_i32;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 32;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 3);
        debug_assert_eq!((- rug_fuzz_5).trailing_zeros(), 4);
        debug_assert_eq!((rug_fuzz_6).trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_7.trailing_zeros(), 4);
        for i in rug_fuzz_8..rug_fuzz_9 {
            debug_assert_eq!((rug_fuzz_10 << i).trailing_zeros(), i);
        }
        debug_assert_eq!(i32::MAX.trailing_zeros(), 0);
        debug_assert_eq!(i32::MIN.trailing_zeros(), 31);
        let _rug_ed_tests_llm_16_885_llm_16_885_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_887_llm_16_887 {
    use super::*;
    use crate::*;
    #[test]
    fn test_unsigned_shr() {
        let _rug_st_tests_llm_16_887_llm_16_887_rrrruuuugggg_test_unsigned_shr = 0;
        let rug_fuzz_0 = 1i32;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1i32;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0i32;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(
            < i32 as PrimInt > ::unsigned_shr(- rug_fuzz_0, rug_fuzz_1), i32::MAX
        );
        debug_assert_eq!(
            < i32 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 0i32
        );
        debug_assert_eq!(
            < i32 as PrimInt > ::unsigned_shr(rug_fuzz_4, rug_fuzz_5), 0i32
        );
        let max_unsigned_shr1 = i32::MAX as u32 >> rug_fuzz_6;
        debug_assert_eq!(
            < i32 as PrimInt > ::unsigned_shr(i32::MAX, rug_fuzz_7), max_unsigned_shr1 as
            i32
        );
        let min_unsigned_shr1 = (i32::MIN as u32 >> rug_fuzz_8) as i32;
        debug_assert_eq!(
            < i32 as PrimInt > ::unsigned_shr(i32::MIN, rug_fuzz_9), min_unsigned_shr1
        );
        let _rug_ed_tests_llm_16_887_llm_16_887_rrrruuuugggg_test_unsigned_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_979_llm_16_979 {
    use super::*;
    use crate::*;
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_979_llm_16_979_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 1i64;
        let rug_fuzz_3 = 0b1010i64;
        let rug_fuzz_4 = 0b1001_1001i64;
        let rug_fuzz_5 = 0i64;
        let rug_fuzz_6 = 1234567890i64;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!((- rug_fuzz_2).count_ones(), 64);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_5.count_ones(), < i64 as PrimInt > ::count_ones(0i64));
        debug_assert_eq!(
            rug_fuzz_6.count_ones(), < i64 as PrimInt > ::count_ones(1234567890i64)
        );
        let _rug_ed_tests_llm_16_979_llm_16_979_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_980 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_980_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0b0000_0000_0000_0000_0000_0000_0000_0000i64;
        let rug_fuzz_1 = 0b0000_0000_0000_0000_0000_0000_0000_0001i64;
        let rug_fuzz_2 = 0b1000_0000_0000_0000_0000_0000_0000_0000i64;
        let rug_fuzz_3 = 0b0100_0000_0000_0000_0000_0000_0000_0000i64;
        let rug_fuzz_4 = 0b0010_0000_0000_0000_0000_0000_0000_0000i64;
        let rug_fuzz_5 = 0b1111_1111_1111_1111_1111_1111_1111_1110i64;
        let rug_fuzz_6 = 0b1111_1111_1111_1111_1111_1111_1111_1111i64;
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_0), 64);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_1), 63);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_2), 0);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_4), 2);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_5), 1);
        debug_assert_eq!(< i64 as int::PrimInt > ::count_zeros(rug_fuzz_6), 0);
        let _rug_ed_tests_llm_16_980_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_981_llm_16_981 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_981_llm_16_981_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x1234567890ABCDEFu64;
        let rug_fuzz_1 = 0x1234567890ABCDEFu64;
        let big_endian_value = rug_fuzz_0.to_be();
        let expected_value = rug_fuzz_1;
        let result_value = i64::from_be(big_endian_value as i64);
        debug_assert_eq!(result_value as u64, expected_value);
        let _rug_ed_tests_llm_16_981_llm_16_981_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_982_llm_16_982 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        let big_endian_bytes = if cfg!(target_endian = "big") {
            0x12_34_56_78_90_AB_CD_EF_i64.to_le_bytes()
        } else {
            0x12_34_56_78_90_AB_CD_EF_i64.to_be_bytes()
        };
        let num_from_bytes = i64::from_le_bytes(big_endian_bytes);
        let expected_num = <i64 as PrimInt>::from_le(num_from_bytes);
        assert_eq!(num_from_bytes, expected_num);
    }
}
#[cfg(test)]
mod tests_llm_16_983 {
    use super::*;
    use crate::*;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_983_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 0b1111_1111i64;
        let rug_fuzz_3 = 0b1000_0000_0000_0000i64;
        let rug_fuzz_4 = 1i64;
        let rug_fuzz_5 = 2i64;
        let rug_fuzz_6 = 256i64;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 63);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 56);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 47);
        debug_assert_eq!((- rug_fuzz_4).leading_ones(), 64);
        debug_assert_eq!((- rug_fuzz_5).leading_ones(), 0);
        debug_assert_eq!((- rug_fuzz_6).leading_ones(), 0);
        debug_assert_eq!(i64::MAX.leading_ones(), 0);
        debug_assert_eq!(i64::MIN.leading_ones(), 64);
        let _rug_ed_tests_llm_16_983_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_984_llm_16_984 {
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_984_llm_16_984_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 0x7FFFFFFFFFFFFFFF;
        let rug_fuzz_4 = 1i64;
        debug_assert_eq!(< i64 as PrimInt > ::leading_zeros(rug_fuzz_0), 64);
        debug_assert_eq!(< i64 as PrimInt > ::leading_zeros(rug_fuzz_1), 63);
        debug_assert_eq!(< i64 as PrimInt > ::leading_zeros(rug_fuzz_2), 62);
        debug_assert_eq!(< i64 as PrimInt > ::leading_zeros(rug_fuzz_3), 1);
        debug_assert_eq!(< i64 as PrimInt > ::leading_zeros(- rug_fuzz_4), 0);
        let _rug_ed_tests_llm_16_984_llm_16_984_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_985_llm_16_985 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_985_llm_16_985_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2i64;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2i64;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 3i64;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 3i64;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 3i64;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 3i64;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 0i64;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0i64;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 0i64;
        let rug_fuzz_21 = 2;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 2i64;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1);
        debug_assert_eq!(rug_fuzz_2.pow(rug_fuzz_3), 2);
        debug_assert_eq!(rug_fuzz_4.pow(rug_fuzz_5), 4);
        debug_assert_eq!(rug_fuzz_6.pow(rug_fuzz_7), 8);
        debug_assert_eq!((- rug_fuzz_8).pow(rug_fuzz_9), 1);
        debug_assert_eq!((- rug_fuzz_10).pow(rug_fuzz_11), - 3);
        debug_assert_eq!((- rug_fuzz_12).pow(rug_fuzz_13), 9);
        debug_assert_eq!((- rug_fuzz_14).pow(rug_fuzz_15), - 27);
        debug_assert_eq!(rug_fuzz_16.pow(rug_fuzz_17), 1);
        debug_assert_eq!(rug_fuzz_18.pow(rug_fuzz_19), 0);
        debug_assert_eq!(rug_fuzz_20.pow(rug_fuzz_21), 0);
        debug_assert_eq!(i64::MAX.pow(rug_fuzz_22), 1);
        debug_assert_eq!(i64::MAX.pow(rug_fuzz_23), i64::MAX);
        debug_assert_eq!(rug_fuzz_24.pow(u32::MAX), 0);
        let _rug_ed_tests_llm_16_985_llm_16_985_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_987 {
    use crate::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_987_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000_0000_0000_0000_0000_0001;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b0010_0000_0000_0000_0000_0000_0000_0000_0010;
        let value: i64 = rug_fuzz_0;
        let rotated = <i64 as PrimInt>::rotate_left(value, rug_fuzz_1);
        let expected = rug_fuzz_2;
        debug_assert_eq!(rotated, expected);
        let _rug_ed_tests_llm_16_987_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_988_llm_16_988 {
    use crate::int::PrimInt;
    #[test]
    fn rotate_right_test() {
        let _rug_st_tests_llm_16_988_llm_16_988_rrrruuuugggg_rotate_right_test = 0;
        let rug_fuzz_0 = 0b1011_0001_1110_1001_1011_0001_1110_1001;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = 32;
        let rug_fuzz_5 = 64;
        let rug_fuzz_6 = 128;
        let value: i64 = rug_fuzz_0;
        let rotate_by_0 = <i64 as PrimInt>::rotate_right(value, rug_fuzz_1);
        debug_assert_eq!(rotate_by_0, value);
        let rotate_by_8 = <i64 as PrimInt>::rotate_right(value, rug_fuzz_2);
        debug_assert_eq!(rotate_by_8, 0b1001_1011_0001_1110_1001_1011_0001_1110);
        let rotate_by_16 = <i64 as PrimInt>::rotate_right(value, rug_fuzz_3);
        debug_assert_eq!(rotate_by_16, 0b1110_1001_1011_0001_1110_1001_1011_0001);
        let rotate_by_32 = <i64 as PrimInt>::rotate_right(value, rug_fuzz_4);
        debug_assert_eq!(rotate_by_32, 0b1011_0001_1110_1001_1011_0001_1110_1001);
        let rotate_by_64 = <i64 as PrimInt>::rotate_right(value, rug_fuzz_5);
        debug_assert_eq!(rotate_by_64, value);
        let rotate_overflow = <i64 as PrimInt>::rotate_right(value, rug_fuzz_6);
        debug_assert_eq!(rotate_overflow, value);
        let _rug_ed_tests_llm_16_988_llm_16_988_rrrruuuugggg_rotate_right_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_989_llm_16_989 {
    use crate::int::PrimInt;
    #[test]
    fn signed_shl_positive_shift() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_positive_shift = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 3;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 8);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_positive_shift = 0;
    }
    #[test]
    fn signed_shl_negative_shift() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_negative_shift = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 3;
        let value: i64 = -rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, - 8);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_negative_shift = 0;
    }
    #[test]
    fn signed_shl_zero_shift() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_zero_shift = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 0;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 10);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_zero_shift = 0;
    }
    #[test]
    fn signed_shl_shift_by_64() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_by_64 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 63;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, i64::MIN);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_by_64 = 0;
    }
    #[test]
    fn signed_shl_big_shift() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_big_shift = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 63;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, i64::MIN);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_big_shift = 0;
    }
    #[test]
    fn signed_shl_shift_negative_value() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_negative_value = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let value: i64 = -rug_fuzz_0;
        let result = <i64 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, - 2);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_negative_value = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to shift left with overflow")]
    fn signed_shl_shift_overflow() {
        let _rug_st_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_overflow = 0;
        let rug_fuzz_0 = 1;
        let value: i64 = i64::MAX;
        let _ = <i64 as PrimInt>::signed_shl(value, rug_fuzz_0);
        let _rug_ed_tests_llm_16_989_llm_16_989_rrrruuuugggg_signed_shl_shift_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_990 {
    use super::*;
    use crate::*;
    #[test]
    fn test_signed_shr_positive() {
        let _rug_st_tests_llm_16_990_rrrruuuugggg_test_signed_shr_positive = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < i64 as int::PrimInt > ::signed_shr(rug_fuzz_0, rug_fuzz_1), 4
        );
        let _rug_ed_tests_llm_16_990_rrrruuuugggg_test_signed_shr_positive = 0;
    }
    #[test]
    fn test_signed_shr_negative() {
        let _rug_st_tests_llm_16_990_rrrruuuugggg_test_signed_shr_negative = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < i64 as int::PrimInt > ::signed_shr(- rug_fuzz_0, rug_fuzz_1), - 4
        );
        let _rug_ed_tests_llm_16_990_rrrruuuugggg_test_signed_shr_negative = 0;
    }
    #[test]
    #[should_panic]
    fn test_signed_shr_overflow() {
        let _rug_st_tests_llm_16_990_rrrruuuugggg_test_signed_shr_overflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 64;
        <i64 as int::PrimInt>::signed_shr(rug_fuzz_0, rug_fuzz_1);
        let _rug_ed_tests_llm_16_990_rrrruuuugggg_test_signed_shr_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_992 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let num = 0x12345678i64;
        let big_endian_num = num.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(num, big_endian_num);
        } else if cfg!(target_endian = "little") {
            assert_eq!(num.swap_bytes(), big_endian_num);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_993 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_le() {
        let big_endian_value = 0x1234567812345678i64;
        let expected_value = big_endian_value.to_le();
        if cfg!(target_endian = "big") {
            assert_eq!(expected_value, big_endian_value.swap_bytes());
        } else {
            assert_eq!(expected_value, big_endian_value);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_994_llm_16_994 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_994_llm_16_994_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 1i64;
        let rug_fuzz_4 = 2i64;
        let rug_fuzz_5 = 0b101100i64;
        let rug_fuzz_6 = 0b1000i64;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!((- rug_fuzz_3).trailing_ones(), 64);
        debug_assert_eq!((- rug_fuzz_4).trailing_ones(), 0);
        debug_assert_eq!((rug_fuzz_5).trailing_ones(), 2);
        debug_assert_eq!((rug_fuzz_6).trailing_ones(), 0);
        let _rug_ed_tests_llm_16_994_llm_16_994_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_995_llm_16_995 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_995_llm_16_995_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 2i64;
        let rug_fuzz_3 = 2i64;
        let rug_fuzz_4 = 8i64;
        let rug_fuzz_5 = 8i64;
        let rug_fuzz_6 = 16i64;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!((- rug_fuzz_3).trailing_zeros(), 1);
        debug_assert_eq!((rug_fuzz_4).trailing_zeros(), 3);
        debug_assert_eq!((- rug_fuzz_5).trailing_zeros(), 3);
        debug_assert_eq!((rug_fuzz_6).trailing_zeros(), 4);
        debug_assert_eq!(i64::MIN.trailing_zeros(), 0);
        debug_assert_eq!(i64::MAX.trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_995_llm_16_995_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_996_llm_16_996 {
    use crate::PrimInt;
    #[test]
    fn unsigned_shl_works() {
        let _rug_st_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_works = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let a: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(a, rug_fuzz_1), 32);
        let b: i64 = -rug_fuzz_2;
        debug_assert_eq!(
            < i64 as PrimInt > ::unsigned_shl(b, rug_fuzz_3), - 2i64 as u64 as i64
        );
        let c: i64 = i64::MAX;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(c, rug_fuzz_4), - 4);
        let _rug_ed_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_works = 0;
    }
    #[test]
    fn unsigned_shl_zero() {
        let _rug_st_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 8;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(rug_fuzz_0, rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_zero = 0;
    }
    #[test]
    fn unsigned_shl_edge_cases() {
        let _rug_st_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let max_shl_1: i64 = i64::MAX;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(max_shl_1, rug_fuzz_0), - 2);
        let min_shl_1: i64 = i64::MIN;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(min_shl_1, rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_edge_cases = 0;
    }
    #[test]
    fn unsigned_shl_boundaries() {
        let _rug_st_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_boundaries = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 63;
        let rug_fuzz_3 = 123;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 63;
        let value: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(value, rug_fuzz_1), 123);
        debug_assert_eq!(
            < i64 as PrimInt > ::unsigned_shl(value, rug_fuzz_2), 123i64
            .overflowing_shl(63).0
        );
        let value: i64 = -rug_fuzz_3;
        debug_assert_eq!(< i64 as PrimInt > ::unsigned_shl(value, rug_fuzz_4), - 123);
        debug_assert_eq!(
            < i64 as PrimInt > ::unsigned_shl(value, rug_fuzz_5), - 123i64
            .overflowing_shl(63).0
        );
        let _rug_ed_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_boundaries = 0;
    }
    #[test]
    #[should_panic]
    fn unsigned_shl_overflow() {
        let _rug_st_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_overflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 65;
        let value: i64 = rug_fuzz_0;
        <i64 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        let _rug_ed_tests_llm_16_996_llm_16_996_rrrruuuugggg_unsigned_shl_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_997 {
    use super::*;
    use crate::*;
    #[test]
    fn test_unsigned_shr() {
        let _rug_st_tests_llm_16_997_rrrruuuugggg_test_unsigned_shr = 0;
        let rug_fuzz_0 = 8_i64;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1_i64;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1_i64;
        let rug_fuzz_5 = 63;
        let rug_fuzz_6 = 63;
        let rug_fuzz_7 = 0_i64;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2_i64;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 8_i64;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 1_i64;
        let rug_fuzz_14 = 32;
        let rug_fuzz_15 = 32;
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 4_i64
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(- rug_fuzz_2, rug_fuzz_3), i64::MAX /
            2 + 1
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(rug_fuzz_4 << rug_fuzz_5, rug_fuzz_6),
            1_i64
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(rug_fuzz_7, rug_fuzz_8), 0_i64
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(- rug_fuzz_9, rug_fuzz_10), i64::MAX
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(- rug_fuzz_11, rug_fuzz_12), i64::MAX
            / 8 + 1
        );
        debug_assert_eq!(
            < i64 as int::PrimInt > ::unsigned_shr(rug_fuzz_13 << rug_fuzz_14,
            rug_fuzz_15), 1_i64
        );
        let _rug_ed_tests_llm_16_997_rrrruuuugggg_test_unsigned_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1090_llm_16_1090 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_zeros_i8() {
        let _rug_st_tests_llm_16_1090_llm_16_1090_rrrruuuugggg_test_count_zeros_i8 = 0;
        let rug_fuzz_0 = 0b0000_0001i8;
        let rug_fuzz_1 = 0b0000_0000i8;
        let rug_fuzz_2 = 0b0111_1110i8;
        let rug_fuzz_3 = 1i8;
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 7);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), 8);
        debug_assert_eq!(rug_fuzz_2.count_zeros(), 1);
        debug_assert_eq!((- rug_fuzz_3).count_zeros(), 0);
        let _rug_ed_tests_llm_16_1090_llm_16_1090_rrrruuuugggg_test_count_zeros_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1091_llm_16_1091 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let big_endian_value: i8 = 0x12;
        let native_value: i8 = i8::from_be(big_endian_value);
        if cfg!(target_endian = "big") {
            assert_eq!(native_value, big_endian_value);
        } else {
            let swapped = big_endian_value.swap_bytes();
            assert_eq!(native_value, swapped);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1092 {
    use crate::PrimInt;
    #[test]
    fn test_from_le() {
        let big_endian = if cfg!(target_endian = "big") { true } else { false };
        let value: i8 = 0x12;
        let le_value = i8::from_le(value);
        if big_endian {
            let swapped_value: i8 = value.swap_bytes();
            assert_eq!(le_value, swapped_value);
        } else {
            assert_eq!(le_value, value);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1093_llm_16_1093 {
    use crate::int::PrimInt;
    #[test]
    fn leading_ones_test() {
        let _rug_st_tests_llm_16_1093_llm_16_1093_rrrruuuugggg_leading_ones_test = 0;
        let rug_fuzz_0 = 0b00000000;
        let rug_fuzz_1 = 0b01111111;
        let rug_fuzz_2 = 0b00111111;
        let rug_fuzz_3 = 0b00011111;
        let rug_fuzz_4 = 0b00001111;
        debug_assert_eq!(i8::leading_ones(rug_fuzz_0), 0);
        debug_assert_eq!(i8::leading_ones(i8::MIN), 8);
        debug_assert_eq!(i8::leading_ones(rug_fuzz_1), 0);
        debug_assert_eq!(i8::leading_ones(rug_fuzz_2), 0);
        debug_assert_eq!(i8::leading_ones(rug_fuzz_3), 0);
        debug_assert_eq!(i8::leading_ones(rug_fuzz_4), 0);
        let _rug_ed_tests_llm_16_1093_llm_16_1093_rrrruuuugggg_leading_ones_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1097_llm_16_1097 {
    use crate::int::PrimInt;
    #[test]
    fn rotate_left_i8() {
        let _rug_st_tests_llm_16_1097_llm_16_1097_rrrruuuugggg_rotate_left_i8 = 0;
        let rug_fuzz_0 = 79i8;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 8;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 9;
        let rug_fuzz_7 = 15;
        let val: i8 = -rug_fuzz_0;
        debug_assert_eq!(val.rotate_left(rug_fuzz_1), - 79i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_2), 0b0001_1011_i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_3), - 79i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_4), 0b0110_0010_i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_5), - 115i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_6), 0b0110_0010_i8);
        debug_assert_eq!(val.rotate_left(rug_fuzz_7), - 115i8);
        let _rug_ed_tests_llm_16_1097_llm_16_1097_rrrruuuugggg_rotate_left_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1101 {
    use super::*;
    use crate::*;
    #[test]
    fn test_swap_bytes_i8() {
        let _rug_st_tests_llm_16_1101_rrrruuuugggg_test_swap_bytes_i8 = 0;
        let rug_fuzz_0 = 0x12;
        let x: i8 = rug_fuzz_0;
        let swapped = x.swap_bytes();
        debug_assert_eq!(swapped, x);
        let _rug_ed_tests_llm_16_1101_rrrruuuugggg_test_swap_bytes_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1102 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let _rug_st_tests_llm_16_1102_rrrruuuugggg_test_to_be = 0;
        let rug_fuzz_0 = 0x12;
        let rug_fuzz_1 = 0x12;
        let rug_fuzz_2 = 0;
        let x: i8 = rug_fuzz_0;
        debug_assert_eq!(x.to_be(), 0x12);
        let x: i8 = -rug_fuzz_1;
        debug_assert_eq!(x.to_be(), - 0x12);
        let x: i8 = i8::MIN;
        debug_assert_eq!(x.to_be(), i8::MIN);
        let x: i8 = i8::MAX;
        debug_assert_eq!(x.to_be(), i8::MAX);
        let x: i8 = rug_fuzz_2;
        debug_assert_eq!(x.to_be(), 0);
        let _rug_ed_tests_llm_16_1102_rrrruuuugggg_test_to_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1103_llm_16_1103 {
    use crate::PrimInt;
    #[test]
    fn test_to_le() {
        let big_endian: i8 = 0x12;
        let little_endian = big_endian.to_le();
        if cfg!(target_endian = "big") {
            assert_eq!(little_endian, big_endian.swap_bytes());
        } else {
            assert_eq!(little_endian, big_endian);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1104 {
    use crate::int::PrimInt;
    #[cfg(has_leading_trailing_ones)]
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1104_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 1i8;
        let rug_fuzz_2 = 2i8;
        let rug_fuzz_3 = 1i8;
        let rug_fuzz_4 = 2i8;
        let rug_fuzz_5 = 4i8;
        let rug_fuzz_6 = 0b0101_1000i8;
        let rug_fuzz_7 = 0b0001_0000i8;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!((- rug_fuzz_3).trailing_ones(), 8);
        debug_assert_eq!((- rug_fuzz_4).trailing_ones(), 1);
        debug_assert_eq!((- rug_fuzz_5).trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_7.trailing_ones(), 4);
        let _rug_ed_tests_llm_16_1104_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1105_llm_16_1105 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1105_llm_16_1105_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 1i8;
        let rug_fuzz_2 = 2i8;
        let rug_fuzz_3 = 1i8;
        let rug_fuzz_4 = 2i8;
        let rug_fuzz_5 = 8i8;
        let rug_fuzz_6 = 0b0101_0000i8;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!((- rug_fuzz_3).trailing_zeros(), 0);
        debug_assert_eq!((- rug_fuzz_4).trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 4);
        let _rug_ed_tests_llm_16_1105_llm_16_1105_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1106_llm_16_1106 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        assert_eq!(< i8 as PrimInt >::unsigned_shl(1, 0), 1);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(1, 1), 2);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(1, 7), - 128);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(- 128, 1), 0);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(- 1, 7), - 128);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(0, 8), 0);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(1, 8), 0);
        assert_eq!(< i8 as PrimInt >::unsigned_shl(1, 31), 0);
    }
}
#[cfg(test)]
mod tests_llm_16_1107_llm_16_1107 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shr() {
        let value: i8 = -0x80;
        let result = <i8 as PrimInt>::unsigned_shr(value, 7);
        assert_eq!(result, 1);
    }
}
#[cfg(test)]
mod tests_llm_16_1199_llm_16_1199 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones_for_isize() {
        let _rug_st_tests_llm_16_1199_llm_16_1199_rrrruuuugggg_test_count_ones_for_isize = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        let rug_fuzz_2 = 1isize;
        let rug_fuzz_3 = 0b1010isize;
        let rug_fuzz_4 = 0b0101isize;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!((- rug_fuzz_2).count_ones(), (isize::BITS as u32));
        debug_assert_eq!(rug_fuzz_3.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 2);
        debug_assert_eq!(isize::MAX.count_ones(), (isize::BITS as u32) - 1);
        debug_assert_eq!(isize::MIN.count_ones(), 1);
        let _rug_ed_tests_llm_16_1199_llm_16_1199_rrrruuuugggg_test_count_ones_for_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1200 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1200_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 3isize;
        let rug_fuzz_1 = 0isize;
        let rug_fuzz_2 = 1isize;
        let rug_fuzz_3 = 0b101000isize;
        debug_assert_eq!(
            rug_fuzz_0.count_zeros(), (isize::BITS - 1) - 3isize.leading_zeros()
        );
        debug_assert_eq!(rug_fuzz_1.count_zeros(), isize::BITS);
        debug_assert_eq!((- rug_fuzz_2).count_zeros(), 0);
        debug_assert_eq!(
            (rug_fuzz_3).count_zeros(), isize::BITS - 3 - (0b101000isize).leading_zeros()
        );
        let max_value = isize::MAX;
        debug_assert_eq!(max_value.count_zeros(), 1);
        let min_value = isize::MIN;
        debug_assert_eq!(min_value.count_zeros(), isize::BITS - 1);
        let _rug_ed_tests_llm_16_1200_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1202_llm_16_1202 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        if cfg!(target_endian = "little") {
            assert_eq!(< isize as PrimInt >::from_le(1234isize), 1234isize);
            assert_eq!(< isize as PrimInt >::from_le(- 1234isize), - 1234isize);
            assert_eq!(< isize as PrimInt >::from_le(isize::MIN), isize::MIN);
            assert_eq!(< isize as PrimInt >::from_le(isize::MAX), isize::MAX);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1203 {
    use super::*;
    use crate::*;
    #[cfg(has_leading_trailing_ones)]
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1203_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 0b0001_0000i32;
        let rug_fuzz_2 = 0b0111_0000i32;
        let rug_fuzz_3 = 0b1000_0000i32;
        let rug_fuzz_4 = 0b1111_0000i32;
        let rug_fuzz_5 = 0b1111_1111i32;
        let rug_fuzz_6 = 1i32;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 27);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 24);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 0);
        debug_assert_eq!(i32::MAX.leading_ones(), 0);
        debug_assert_eq!(i32::MIN.leading_ones(), 0);
        debug_assert_eq!((- rug_fuzz_6).leading_ones(), 32);
        let _rug_ed_tests_llm_16_1203_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1204_llm_16_1204 {
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1204_llm_16_1204_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0b0000_0001isize;
        let rug_fuzz_1 = 0b0000_0010isize;
        let rug_fuzz_2 = 0b0001_0000isize;
        let rug_fuzz_3 = 0b0100_0000isize;
        let rug_fuzz_4 = 0b1000_0000isize;
        let rug_fuzz_5 = 0isize;
        debug_assert_eq!(rug_fuzz_0.leading_zeros(), 63);
        debug_assert_eq!(rug_fuzz_1.leading_zeros(), 62);
        debug_assert_eq!(rug_fuzz_2.leading_zeros(), 59);
        debug_assert_eq!(rug_fuzz_3.leading_zeros(), 57);
        debug_assert_eq!(rug_fuzz_4.leading_zeros(), 56);
        debug_assert_eq!(isize::MAX.leading_zeros(), 0);
        debug_assert_eq!(isize::MIN.leading_zeros(), 0);
        debug_assert_eq!(
            rug_fuzz_5.leading_zeros(), (8 * std::mem::size_of:: < isize > ()) as u32
        );
        let _rug_ed_tests_llm_16_1204_llm_16_1204_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1205_llm_16_1205 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1205_llm_16_1205_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2isize;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 3isize;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0isize;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0isize;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 1isize;
        let rug_fuzz_9 = 100;
        let rug_fuzz_10 = 1isize;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 1isize;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 10isize;
        let rug_fuzz_15 = 0;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 16);
        debug_assert_eq!((- rug_fuzz_2).pow(rug_fuzz_3), - 27);
        debug_assert_eq!(rug_fuzz_4.pow(rug_fuzz_5), 1);
        debug_assert_eq!(rug_fuzz_6.pow(rug_fuzz_7), 0);
        debug_assert_eq!(rug_fuzz_8.pow(rug_fuzz_9), 1);
        debug_assert_eq!((- rug_fuzz_10).pow(rug_fuzz_11), 1);
        debug_assert_eq!((- rug_fuzz_12).pow(rug_fuzz_13), - 1);
        debug_assert_eq!(rug_fuzz_14.pow(rug_fuzz_15), 1);
        let _rug_ed_tests_llm_16_1205_llm_16_1205_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1206_llm_16_1206 {
    use crate::int::PrimInt;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_1206_llm_16_1206_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b00000000000000000000000000000000;
        let rug_fuzz_1 = 0b00000000000000000000000000000001;
        let rug_fuzz_2 = 0b00000000000000000000000000001000;
        let rug_fuzz_3 = 0b10000000000000000000000000000000;
        let rug_fuzz_4 = 0b01010101010101010101010101010101;
        debug_assert_eq!(
            < isize as PrimInt > ::reverse_bits(rug_fuzz_0),
            0b00000000000000000000000000000000
        );
        debug_assert_eq!(
            < isize as PrimInt > ::reverse_bits(rug_fuzz_1),
            0b10000000000000000000000000000000
        );
        debug_assert_eq!(
            < isize as PrimInt > ::reverse_bits(rug_fuzz_2),
            0b00010000000000000000000000000000
        );
        debug_assert_eq!(
            < isize as PrimInt > ::reverse_bits(rug_fuzz_3),
            0b00000000000000000000000000000001
        );
        debug_assert_eq!(
            < isize as PrimInt > ::reverse_bits(rug_fuzz_4),
            0b10101010101010101010101010101010
        );
        let _rug_ed_tests_llm_16_1206_llm_16_1206_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1207 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_1207_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b1011_0001_0000_0000_0000_0000_0000_0000;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0b1011_0001_0000_0000_0000_0000_0000_0000;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 0b1011_0001_0000_0000_0000_0000_0000_0000;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 0;
        let value: isize = rug_fuzz_0;
        let result = <isize as int::PrimInt>::rotate_left(value, rug_fuzz_1);
        let expected = rug_fuzz_2 << rug_fuzz_3
            | rug_fuzz_4 >> (isize::BITS as u32 - rug_fuzz_5);
        debug_assert_eq!(result, expected);
        let result = <isize as int::PrimInt>::rotate_left(value, isize::BITS as u32);
        debug_assert_eq!(result, value);
        let result = <isize as int::PrimInt>::rotate_left(value, rug_fuzz_6);
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_1207_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1208 {
    use super::*;
    use crate::*;
    #[test]
    fn rotate_right_basic() {
        let _rug_st_tests_llm_16_1208_rrrruuuugggg_rotate_right_basic = 0;
        let rug_fuzz_0 = 5isize;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 5isize;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 5isize;
        let rug_fuzz_5 = 31;
        let rug_fuzz_6 = 5isize;
        let rug_fuzz_7 = 32;
        let rug_fuzz_8 = 5isize;
        let rug_fuzz_9 = 63;
        let rug_fuzz_10 = 5isize;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(rug_fuzz_0.rotate_right(rug_fuzz_1), 5isize);
        debug_assert_eq!(
            rug_fuzz_2.rotate_right(rug_fuzz_3), (5isize >> 1) | (5isize << (isize::BITS
            - 1))
        );
        debug_assert_eq!(
            rug_fuzz_4.rotate_right(rug_fuzz_5), (5isize >> 31) | (5isize << (isize::BITS
            - 31))
        );
        debug_assert_eq!(rug_fuzz_6.rotate_right(rug_fuzz_7), 5isize);
        debug_assert_eq!(
            rug_fuzz_8.rotate_right(rug_fuzz_9), (5isize >> 63) | (5isize << (isize::BITS
            - 63))
        );
        debug_assert_eq!(
            (- rug_fuzz_10).rotate_right(rug_fuzz_11), ((- 5isize as usize) >> 1) as
            isize | ((- 5isize as usize).rotate_left(1) as isize)
        );
        let _rug_ed_tests_llm_16_1208_rrrruuuugggg_rotate_right_basic = 0;
    }
    #[test]
    fn rotate_right_edge_cases() {
        let _rug_st_tests_llm_16_1208_rrrruuuugggg_rotate_right_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            isize::MAX.rotate_right(rug_fuzz_0), (isize::MAX >> 1) | (isize::MAX <<
            (isize::BITS - 1))
        );
        debug_assert_eq!(
            isize::MIN.rotate_right(rug_fuzz_1), (isize::MIN >> 1) | (isize::MIN <<
            (isize::BITS - 1))
        );
        let _rug_ed_tests_llm_16_1208_rrrruuuugggg_rotate_right_edge_cases = 0;
    }
    #[test]
    fn rotate_right_full_rotation() {
        let _rug_st_tests_llm_16_1208_rrrruuuugggg_rotate_right_full_rotation = 0;
        let rug_fuzz_0 = 123isize;
        let value = rug_fuzz_0;
        let rotation = isize::BITS as u32;
        debug_assert_eq!(value.rotate_right(rotation), value);
        let _rug_ed_tests_llm_16_1208_rrrruuuugggg_rotate_right_full_rotation = 0;
    }
    #[test]
    fn rotate_right_multiple_full_rotation() {
        let _rug_st_tests_llm_16_1208_rrrruuuugggg_rotate_right_multiple_full_rotation = 0;
        let rug_fuzz_0 = 456isize;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let value = rug_fuzz_0;
        let rotation = isize::BITS as u32;
        debug_assert_eq!(value.rotate_right(rotation * rug_fuzz_1), value);
        debug_assert_eq!(value.rotate_right(rotation * rug_fuzz_2), value);
        let _rug_ed_tests_llm_16_1208_rrrruuuugggg_rotate_right_multiple_full_rotation = 0;
    }
    #[test]
    fn rotate_right_rotate_left_identity() {
        let _rug_st_tests_llm_16_1208_rrrruuuugggg_rotate_right_rotate_left_identity = 0;
        let rug_fuzz_0 = 789isize;
        let rug_fuzz_1 = 5u32;
        let value = rug_fuzz_0;
        let rotation = rug_fuzz_1;
        debug_assert_eq!(value.rotate_right(rotation).rotate_left(rotation), value);
        let _rug_ed_tests_llm_16_1208_rrrruuuugggg_rotate_right_rotate_left_identity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1209_llm_16_1209 {
    use super::*;
    use crate::*;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1209_llm_16_1209_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 4isize;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0isize;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1isize;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1isize;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1isize;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        debug_assert_eq!((- rug_fuzz_0).signed_shl(rug_fuzz_1), - 8isize);
        debug_assert_eq!((rug_fuzz_2).signed_shl(rug_fuzz_3), 0isize);
        debug_assert_eq!((rug_fuzz_4).signed_shl(rug_fuzz_5), 2isize);
        debug_assert_eq!((rug_fuzz_6).signed_shl(rug_fuzz_7), 1isize);
        debug_assert_eq!((- rug_fuzz_8).signed_shl(rug_fuzz_9), - 2isize);
        debug_assert_eq!(
            (isize::MAX).signed_shl(rug_fuzz_10), isize::MAX.wrapping_shl(1)
        );
        debug_assert_eq!(
            (isize::MIN).signed_shl(rug_fuzz_11), isize::MIN.wrapping_shl(1)
        );
        let _rug_ed_tests_llm_16_1209_llm_16_1209_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1210_llm_16_1210 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shr_positive() {
        let _rug_st_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_positive = 0;
        let rug_fuzz_0 = 0b1000;
        let rug_fuzz_1 = 2;
        let value: isize = rug_fuzz_0;
        let result = PrimInt::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b10);
        let _rug_ed_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_positive = 0;
    }
    #[test]
    fn test_signed_shr_negative() {
        let _rug_st_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_negative = 0;
        let rug_fuzz_0 = 0b1000;
        let rug_fuzz_1 = 2;
        let value: isize = -rug_fuzz_0;
        let result = PrimInt::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, - 0b10);
        let _rug_ed_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_negative = 0;
    }
    #[test]
    fn test_signed_shr_zero() {
        let _rug_st_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2;
        let value: isize = rug_fuzz_0;
        let result = PrimInt::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, 0);
        let _rug_ed_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_zero = 0;
    }
    #[test]
    fn test_signed_shr_shift_by_zero() {
        let _rug_st_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_shift_by_zero = 0;
        let rug_fuzz_0 = 0b1000;
        let rug_fuzz_1 = 0;
        let value: isize = rug_fuzz_0;
        let result = PrimInt::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_1210_llm_16_1210_rrrruuuugggg_test_signed_shr_shift_by_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1211_llm_16_1211 {
    use crate::int::PrimInt;
    #[test]
    fn test_swap_bytes_isize() {
        let _rug_st_tests_llm_16_1211_llm_16_1211_rrrruuuugggg_test_swap_bytes_isize = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 0x7856341200000000_isize;
        let rug_fuzz_3 = 0x78563412_isize;
        let a: isize = rug_fuzz_0;
        let byte_count = std::mem::size_of::<isize>();
        let expected: isize = if byte_count == rug_fuzz_1 {
            rug_fuzz_2
        } else {
            rug_fuzz_3
        };
        debug_assert_eq!(a.swap_bytes(), expected);
        let _rug_ed_tests_llm_16_1211_llm_16_1211_rrrruuuugggg_test_swap_bytes_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1212_llm_16_1212 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let _rug_st_tests_llm_16_1212_llm_16_1212_rrrruuuugggg_test_to_be = 0;
        let rug_fuzz_0 = 0x0123_4567;
        let rug_fuzz_1 = 0x0102_0304;
        let rug_fuzz_2 = 0x0000_0000;
        let rug_fuzz_3 = 0x6745_2301;
        let rug_fuzz_4 = 0x0403_0201;
        let rug_fuzz_5 = 0x0000_0000;
        let native_endian_values: [isize; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let big_endian_values: [isize; 3] = [rug_fuzz_3, rug_fuzz_4, rug_fuzz_5];
        for (&native, &big_endian) in native_endian_values.iter().zip(&big_endian_values)
        {
            debug_assert_eq!(
                native.to_be(), if cfg!(target_endian = "big") { native } else {
                big_endian }
            );
        }
        let _rug_ed_tests_llm_16_1212_llm_16_1212_rrrruuuugggg_test_to_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1213_llm_16_1213 {
    use crate::int::PrimInt;
    #[test]
    fn test_to_le() {
        let big_endian: [u8; 8] = if cfg!(target_endian = "big") {
            [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]
        } else {
            [0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12]
        };
        let number = isize::from_be_bytes(big_endian);
        let expected = isize::from_le_bytes(big_endian);
        assert_eq!(number.to_le(), expected);
    }
}
#[cfg(test)]
mod tests_llm_16_1214_llm_16_1214 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1214_llm_16_1214_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0b0000_0000;
        let rug_fuzz_1 = 0b0001_0000;
        let rug_fuzz_2 = 0b0000_0001;
        let rug_fuzz_3 = 0b0000_0011;
        let rug_fuzz_4 = 0b0000_0111;
        let rug_fuzz_5 = 0b0000_1111;
        let rug_fuzz_6 = 0b1111_1111;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 1;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 3);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 4);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 8);
        debug_assert_eq!(isize::MIN.trailing_ones(), 0);
        let max_trailing_ones = (rug_fuzz_7 * std::mem::size_of::<isize>() - rug_fuzz_8)
            as u32;
        debug_assert_eq!(isize::MAX.trailing_ones(), max_trailing_ones);
        let _rug_ed_tests_llm_16_1214_llm_16_1214_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1215_llm_16_1215 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1215_llm_16_1215_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        let rug_fuzz_2 = 2isize;
        let rug_fuzz_3 = 2isize;
        let rug_fuzz_4 = 4isize;
        let rug_fuzz_5 = 8isize;
        let rug_fuzz_6 = 16isize;
        let rug_fuzz_7 = 16isize;
        let rug_fuzz_8 = 1024isize;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1isize;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!((- rug_fuzz_3).trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 4);
        debug_assert_eq!((- rug_fuzz_7).trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_8.trailing_zeros(), 10);
        let max_trailing_zeros = (isize::BITS - rug_fuzz_9) as u32;
        debug_assert_eq!(
            (rug_fuzz_10 << max_trailing_zeros).trailing_zeros(), max_trailing_zeros
        );
        let _rug_ed_tests_llm_16_1215_llm_16_1215_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1216_llm_16_1216 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_llm_16_1216_llm_16_1216_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 31;
        let rug_fuzz_5 = 32;
        let rug_fuzz_6 = 32;
        let rug_fuzz_7 = 8;
        let value: isize = rug_fuzz_0;
        let result_0 = <isize as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(result_0, 4);
        let result_1 = <isize as PrimInt>::unsigned_shl(value, rug_fuzz_2);
        debug_assert_eq!(result_1, 8);
        let result_2 = <isize as PrimInt>::unsigned_shl(value, rug_fuzz_3);
        debug_assert_eq!(result_2, 16);
        let result_31 = <isize as PrimInt>::unsigned_shl(value, rug_fuzz_4);
        debug_assert_eq!(result_31, 4isize.wrapping_shl(31));
        let result_32 = <isize as PrimInt>::unsigned_shl(value, rug_fuzz_5);
        let wrap_32 = rug_fuzz_6 % (std::mem::size_of::<isize>() as u32 * rug_fuzz_7);
        debug_assert_eq!(result_32, 4isize.wrapping_shl(wrap_32));
        let _rug_ed_tests_llm_16_1216_llm_16_1216_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1217_llm_16_1217 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shr_test() {
        let _rug_st_tests_llm_16_1217_llm_16_1217_rrrruuuugggg_unsigned_shr_test = 0;
        let rug_fuzz_0 = 12345;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3;
        let value: isize = -rug_fuzz_0;
        let shifted = PrimInt::unsigned_shr(value, rug_fuzz_1);
        let expected = ((value as usize) >> rug_fuzz_2) as isize;
        debug_assert_eq!(shifted, expected, "unsigned_shr did not shift correctly");
        let _rug_ed_tests_llm_16_1217_llm_16_1217_rrrruuuugggg_unsigned_shr_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1405_llm_16_1405 {
    use crate::PrimInt;
    #[test]
    fn count_ones_test() {
        let _rug_st_tests_llm_16_1405_llm_16_1405_rrrruuuugggg_count_ones_test = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1u128;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 4u128;
        let rug_fuzz_4 = 3u128;
        let rug_fuzz_5 = 0xF0u128;
        let rug_fuzz_6 = 0x123456789ABCDEF0u128;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_5.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_6.count_ones(), 32);
        debug_assert_eq!(u128::MAX.count_ones(), 128);
        let _rug_ed_tests_llm_16_1405_llm_16_1405_rrrruuuugggg_count_ones_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1406_llm_16_1406 {
    use crate::PrimInt;
    #[test]
    fn count_zeros_u128() {
        let _rug_st_tests_llm_16_1406_llm_16_1406_rrrruuuugggg_count_zeros_u128 = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1u128;
        let rug_fuzz_2 = 2;
        debug_assert_eq!(u128::max_value().count_zeros(), 0);
        debug_assert_eq!(u128::min_value().count_zeros(), 128);
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 128);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), 127);
        debug_assert_eq!((u128::max_value() / rug_fuzz_2).count_zeros(), 0);
        let _rug_ed_tests_llm_16_1406_llm_16_1406_rrrruuuugggg_count_zeros_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1407_llm_16_1407 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_1407_llm_16_1407_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x11223344u128;
        let big_endian_bytes = rug_fuzz_0.to_be();
        let number = u128::from_be(big_endian_bytes);
        debug_assert_eq!(number, 0x11223344u128);
        let _rug_ed_tests_llm_16_1407_llm_16_1407_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1408_llm_16_1408 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        let _rug_st_tests_llm_16_1408_llm_16_1408_rrrruuuugggg_test_from_le = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 0x0123456789ABCDEF0123456789ABCDEF;
        let big_endian = u128::from_le_bytes([rug_fuzz_0; 16]);
        debug_assert_eq!(< u128 as PrimInt > ::from_le(big_endian), big_endian);
        let little_endian = u128::to_le(rug_fuzz_1);
        debug_assert_eq!(
            < u128 as PrimInt > ::from_le(little_endian),
            0x0123456789ABCDEF0123456789ABCDEF
        );
        let _rug_ed_tests_llm_16_1408_llm_16_1408_rrrruuuugggg_test_from_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1409 {
    use super::*;
    use crate::*;
    #[cfg(has_leading_trailing_ones)]
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1409_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u128;
        let rug_fuzz_1 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u128;
        let rug_fuzz_2 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000u128;
        let rug_fuzz_3 = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u128;
        let rug_fuzz_4 = 0b01111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u128;
        let rug_fuzz_5 = 0b00111100_00000000_00000000_00000000_00000000_00000000_00000000_00000000u128;
        debug_assert_eq!(u128::leading_ones(rug_fuzz_0), 0);
        debug_assert_eq!(u128::leading_ones(rug_fuzz_1), 128);
        debug_assert_eq!(u128::leading_ones(rug_fuzz_2), 64);
        debug_assert_eq!(u128::leading_ones(rug_fuzz_3), 1);
        debug_assert_eq!(u128::leading_ones(rug_fuzz_4), 0);
        debug_assert_eq!(u128::leading_ones(rug_fuzz_5), 0);
        let _rug_ed_tests_llm_16_1409_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1410_llm_16_1410 {
    use crate::int::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1410_llm_16_1410_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
        let rug_fuzz_3 = 0x0000_0000_0000_0000_0000_0000_0000_0001;
        debug_assert_eq!(u128::leading_zeros(rug_fuzz_0), 128);
        debug_assert_eq!(u128::leading_zeros(rug_fuzz_1), 127);
        debug_assert_eq!(u128::leading_zeros(u128::MAX), 0);
        debug_assert_eq!(u128::leading_zeros(rug_fuzz_2), 1);
        debug_assert_eq!(u128::leading_zeros(rug_fuzz_3), 127);
        let _rug_ed_tests_llm_16_1410_llm_16_1410_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1411_llm_16_1411 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1411_llm_16_1411_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u128;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2u128;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2u128;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2u128;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 0u128;
        let rug_fuzz_11 = 10;
        let rug_fuzz_12 = 10u128;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 10u128;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 1;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1);
        debug_assert_eq!(rug_fuzz_2.pow(rug_fuzz_3), 2);
        debug_assert_eq!(rug_fuzz_4.pow(rug_fuzz_5), 4);
        debug_assert_eq!(rug_fuzz_6.pow(rug_fuzz_7), 8);
        debug_assert_eq!(rug_fuzz_8.pow(rug_fuzz_9), 16);
        debug_assert_eq!(rug_fuzz_10.pow(rug_fuzz_11), 0);
        debug_assert_eq!(rug_fuzz_12.pow(rug_fuzz_13), 1);
        debug_assert_eq!(rug_fuzz_14.pow(rug_fuzz_15), 100);
        debug_assert_eq!(u128::MAX.pow(rug_fuzz_16), u128::MAX);
        let _rug_ed_tests_llm_16_1411_llm_16_1411_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1412 {
    use super::*;
    use crate::*;
    #[test]
    fn test_reverse_bits_u128() {
        let _rug_st_tests_llm_16_1412_rrrruuuugggg_test_reverse_bits_u128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b10;
        let rug_fuzz_3 = 0b0101;
        let rug_fuzz_4 = 0x123456789abcdef0;
        debug_assert_eq!(< u128 as int::PrimInt > ::reverse_bits(rug_fuzz_0), 0);
        debug_assert_eq!(< u128 as int::PrimInt > ::reverse_bits(u128::MAX), u128::MAX);
        debug_assert_eq!(< u128 as int::PrimInt > ::reverse_bits(rug_fuzz_1), 1 << 127);
        debug_assert_eq!(
            < u128 as int::PrimInt > ::reverse_bits(rug_fuzz_2), 0b01 << 127
        );
        debug_assert_eq!(
            < u128 as int::PrimInt > ::reverse_bits(rug_fuzz_3), 0b1010 << 124
        );
        debug_assert_eq!(
            < u128 as int::PrimInt > ::reverse_bits(rug_fuzz_4), 0x0f7b3d591e6a2c48 << 64
        );
        let _rug_ed_tests_llm_16_1412_rrrruuuugggg_test_reverse_bits_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1413 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_1413_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0x1234567890ABCDEF1234567890ABCDEF;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0x234567890ABCDEF1234567890ABCDEF1;
        let value: u128 = rug_fuzz_0;
        let rotate_by = rug_fuzz_1;
        let result = u128::rotate_left(value, rotate_by);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1413_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1414_llm_16_1414 {
    use crate::int::PrimInt;
    #[test]
    fn rotate_right_u128() {
        let _rug_st_tests_llm_16_1414_llm_16_1414_rrrruuuugggg_rotate_right_u128 = 0;
        let rug_fuzz_0 = 0x123456789abcdef0;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0xf123456789abcdef;
        let rug_fuzz_3 = 128;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 0xef0123456789abcd;
        let rug_fuzz_6 = 0;
        let value: u128 = rug_fuzz_0;
        let rotate_by = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let result = u128::rotate_right(value, rotate_by);
        debug_assert_eq!(result, expected, "rotate_right: u128 values do not match");
        let rotate_by = rug_fuzz_3;
        let expected = value;
        let result = u128::rotate_right(value, rotate_by);
        debug_assert_eq!(
            result, expected,
            "rotate_right: Full rotation should yield the original value"
        );
        let rotate_by = rug_fuzz_4;
        let expected = rug_fuzz_5;
        let result = u128::rotate_right(value, rotate_by);
        debug_assert_eq!(
            result, expected,
            "rotate_right: u128 values do not match after 8 bits rotation"
        );
        let rotate_by = rug_fuzz_6;
        let expected = value;
        let result = u128::rotate_right(value, rotate_by);
        debug_assert_eq!(
            result, expected, "rotate_right: No rotation should yield the original value"
        );
        let _rug_ed_tests_llm_16_1414_llm_16_1414_rrrruuuugggg_rotate_right_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1415_llm_16_1415 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1415_llm_16_1415_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 7;
        let value: u128 = rug_fuzz_0;
        let shifted = <u128 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(shifted, 128);
        let _rug_ed_tests_llm_16_1415_llm_16_1415_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1416_llm_16_1416 {
    use crate::int::PrimInt;
    #[test]
    fn signed_shr_works_correctly() {
        let _rug_st_tests_llm_16_1416_llm_16_1416_rrrruuuugggg_signed_shr_works_correctly = 0;
        let rug_fuzz_0 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
        let rug_fuzz_1 = 64;
        let rug_fuzz_2 = 0xFFFF_FFFF_FFFF_FFFF;
        let input: u128 = rug_fuzz_0;
        let shift_amount: u32 = rug_fuzz_1;
        let result = u128::signed_shr(input, shift_amount);
        let expected: u128 = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1416_llm_16_1416_rrrruuuugggg_signed_shr_works_correctly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1417_llm_16_1417 {
    use crate::int::PrimInt;
    #[test]
    fn test_u128_swap_bytes() {
        let _rug_st_tests_llm_16_1417_llm_16_1417_rrrruuuugggg_test_u128_swap_bytes = 0;
        let rug_fuzz_0 = 0x1234_5678_90AB_CDEF_1234_5678_90AB_CDEF;
        let rug_fuzz_1 = 0xEFCD_AB90_7856_3412_EFCD_AB90_7856_3412;
        let x: u128 = rug_fuzz_0;
        let swapped = x.swap_bytes();
        let expected: u128 = rug_fuzz_1;
        debug_assert_eq!(swapped, expected);
        let _rug_ed_tests_llm_16_1417_llm_16_1417_rrrruuuugggg_test_u128_swap_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1418_llm_16_1418 {
    use crate::int::PrimInt;
    #[test]
    fn test_u128_to_be() {
        let num: u128 = 0x123456789ABCDEF0;
        if cfg!(target_endian = "little") {
            assert_eq!(num.to_be(), 0xF0DEBC9A78563412);
        } else {
            assert_eq!(num.to_be(), num);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1419_llm_16_1419 {
    use crate::int::PrimInt;
    #[test]
    fn test_u128_to_le() {
        let _rug_st_tests_llm_16_1419_llm_16_1419_rrrruuuugggg_test_u128_to_le = 0;
        let rug_fuzz_0 = 0x0102030405060708090A0B0C0D0E0F10;
        let big_endian_number: u128 = rug_fuzz_0;
        let little_endian_number = u128::to_le(big_endian_number);
        #[cfg(target_endian = "big")]
        let expected = big_endian_number.swap_bytes();
        #[cfg(target_endian = "little")]
        let expected = big_endian_number;
        debug_assert_eq!(little_endian_number, expected);
        let _rug_ed_tests_llm_16_1419_llm_16_1419_rrrruuuugggg_test_u128_to_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1420 {
    use crate::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1420_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1u128;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 3u128;
        let rug_fuzz_4 = 4u128;
        let rug_fuzz_5 = 0b1011000u128;
        let rug_fuzz_6 = 0b1011001u128;
        let rug_fuzz_7 = 0b1011011u128;
        let rug_fuzz_8 = 0b1111u128;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_7.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_8.trailing_ones(), 4);
        debug_assert_eq!(u128::MAX.trailing_ones(), 128);
        let _rug_ed_tests_llm_16_1420_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1421_llm_16_1421 {
    use super::*;
    use crate::*;
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1421_llm_16_1421_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1u128;
        let rug_fuzz_2 = 2u128;
        let rug_fuzz_3 = 8u128;
        let rug_fuzz_4 = 16u128;
        let rug_fuzz_5 = 1024u128;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 63;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 63;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 10);
        debug_assert_eq!(u128::MAX.trailing_zeros(), 0);
        debug_assert_eq!(u128::MIN.trailing_zeros(), 0);
        let power_of_two: u128 = rug_fuzz_6 << rug_fuzz_7;
        debug_assert_eq!(power_of_two.trailing_zeros(), 63);
        let all_lower_bits_set: u128 = (rug_fuzz_8 << rug_fuzz_9) - rug_fuzz_10;
        debug_assert_eq!(all_lower_bits_set.trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_1421_llm_16_1421_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1422_llm_16_1422 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shl_basic() {
        let _rug_st_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_basic = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 4;
        let num: u128 = rug_fuzz_0;
        let shifted = num.unsigned_shl(rug_fuzz_1);
        debug_assert_eq!(shifted, 16);
        let _rug_ed_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_basic = 0;
    }
    #[test]
    fn unsigned_shl_by_zero() {
        let _rug_st_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_by_zero = 0;
        let rug_fuzz_0 = 1234;
        let rug_fuzz_1 = 0;
        let num: u128 = rug_fuzz_0;
        let shifted = num.unsigned_shl(rug_fuzz_1);
        debug_assert_eq!(shifted, num);
        let _rug_ed_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_by_zero = 0;
    }
    #[test]
    fn unsigned_shl_full() {
        let _rug_st_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_full = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 127;
        let num: u128 = rug_fuzz_0;
        let shifted = num.unsigned_shl(rug_fuzz_1);
        debug_assert_eq!(shifted, 1 << 127);
        let _rug_ed_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_full = 0;
    }
    #[test]
    #[should_panic]
    fn unsigned_shl_overflow() {
        let _rug_st_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_overflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 128;
        let num: u128 = rug_fuzz_0;
        num.unsigned_shl(rug_fuzz_1);
        let _rug_ed_tests_llm_16_1422_llm_16_1422_rrrruuuugggg_unsigned_shl_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1423_llm_16_1423 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shr() {
        let _rug_st_tests_llm_16_1423_llm_16_1423_rrrruuuugggg_test_unsigned_shr = 0;
        let rug_fuzz_0 = 0b1010_1000_1111_0000_1010_1000_1111_0000_1010_1000_1111_0000_1010_1000_1111_0000u128;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0b101_0100_0111_1000_0101_0100_0111_1000_0101_0100_0111_1000_0101_0100_0111_1000u128;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0b1010_1000_1111_0000_1010_1000_1111_0000_1010_1000_1111_0000_1010_1000_1111u128;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 128;
        let rug_fuzz_7 = 256;
        let value: u128 = rug_fuzz_0;
        debug_assert_eq!(u128::unsigned_shr(value, rug_fuzz_1), value);
        let expected_1 = rug_fuzz_2;
        debug_assert_eq!(u128::unsigned_shr(value, rug_fuzz_3), expected_1);
        let expected_4 = rug_fuzz_4;
        debug_assert_eq!(u128::unsigned_shr(value, rug_fuzz_5), expected_4);
        debug_assert_eq!(u128::unsigned_shr(value, rug_fuzz_6), 0);
        let over_shift = rug_fuzz_7;
        debug_assert_eq!(u128::unsigned_shr(value, over_shift), value);
        let _rug_ed_tests_llm_16_1423_llm_16_1423_rrrruuuugggg_test_unsigned_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1510 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_1510_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 0b1010u16;
        let rug_fuzz_3 = 0b1111u16;
        let rug_fuzz_4 = 0b1000000000000000u16;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 1);
        debug_assert_eq!(u16::MAX.count_ones(), 16);
        let _rug_ed_tests_llm_16_1510_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1511_llm_16_1511 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1511_llm_16_1511_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b0001_0000_0000_0000;
        let rug_fuzz_3 = 0b1000_0000_0000_0000;
        debug_assert_eq!(< u16 as PrimInt > ::count_zeros(rug_fuzz_0), 16);
        debug_assert_eq!(< u16 as PrimInt > ::count_zeros(rug_fuzz_1), 15);
        debug_assert_eq!(< u16 as PrimInt > ::count_zeros(rug_fuzz_2), 11);
        debug_assert_eq!(< u16 as PrimInt > ::count_zeros(rug_fuzz_3), 0);
        debug_assert_eq!(< u16 as PrimInt > ::count_zeros(u16::MAX), 0);
        let _rug_ed_tests_llm_16_1511_llm_16_1511_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1512_llm_16_1512 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let big_endian_value: u16 = u16::from_be(0x1234);
        let expected: u16 = if cfg!(target_endian = "big") { 0x1234 } else { 0x3412 };
        assert_eq!(< u16 as PrimInt >::from_be(big_endian_value), expected);
    }
}
#[cfg(test)]
mod tests_llm_16_1513_llm_16_1513 {
    use crate::PrimInt;
    #[test]
    fn test_from_le() {
        let num: u16 = 0x1234;
        if cfg!(target_endian = "little") {
            assert_eq!(u16::from_le(num), num);
        } else {
            assert_eq!(u16::from_le(num), num.swap_bytes());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1514 {
    use super::*;
    use crate::*;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1514_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 0b1000_0000_0000_0000u16;
        let rug_fuzz_3 = 0b1100_0000_0000_0000u16;
        let rug_fuzz_4 = 0b1111_0000_0000_0000u16;
        let rug_fuzz_5 = 0b1111_1000_0000_0000u16;
        let rug_fuzz_6 = 0b1111_1100_0000_0000u16;
        let rug_fuzz_7 = 0b1111_1110_0000_0000u16;
        let rug_fuzz_8 = 0b1111_1111_0000_0000u16;
        let rug_fuzz_9 = 0b1111_1111_1000_0000u16;
        let rug_fuzz_10 = 0b1111_1111_1100_0000u16;
        let rug_fuzz_11 = 0b1111_1111_1110_0000u16;
        let rug_fuzz_12 = 0b1111_1111_1111_0000u16;
        let rug_fuzz_13 = 0b1111_1111_1111_1000u16;
        let rug_fuzz_14 = 0b1111_1111_1111_1100u16;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 1);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 5);
        debug_assert_eq!(rug_fuzz_6.leading_ones(), 6);
        debug_assert_eq!(rug_fuzz_7.leading_ones(), 7);
        debug_assert_eq!(rug_fuzz_8.leading_ones(), 8);
        debug_assert_eq!(rug_fuzz_9.leading_ones(), 9);
        debug_assert_eq!(rug_fuzz_10.leading_ones(), 10);
        debug_assert_eq!(rug_fuzz_11.leading_ones(), 11);
        debug_assert_eq!(rug_fuzz_12.leading_ones(), 12);
        debug_assert_eq!(rug_fuzz_13.leading_ones(), 13);
        debug_assert_eq!(rug_fuzz_14.leading_ones(), 14);
        debug_assert_eq!(u16::MAX.leading_ones(), 16);
        let _rug_ed_tests_llm_16_1514_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1515_llm_16_1515 {
    use crate::int::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1515_llm_16_1515_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0b0000_0000_0000_0000_u16;
        let rug_fuzz_1 = 0b0000_0000_0000_0001_u16;
        let rug_fuzz_2 = 0b1000_0000_0000_0000_u16;
        let rug_fuzz_3 = 0b0100_0000_0000_0000_u16;
        let rug_fuzz_4 = 0b0010_0000_0000_0000_u16;
        let rug_fuzz_5 = 0b0001_0000_0000_0000_u16;
        let rug_fuzz_6 = 0b0000_1000_0000_0000_u16;
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_0), 16);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_1), 15);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_2), 0);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_3), 1);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_4), 2);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_5), 3);
        debug_assert_eq!(u16::leading_zeros(rug_fuzz_6), 4);
        debug_assert_eq!(u16::leading_zeros(u16::MAX), 0);
        let _rug_ed_tests_llm_16_1515_llm_16_1515_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1517_llm_16_1517 {
    use crate::int::PrimInt;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_1517_llm_16_1517_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b0000_0001_0010_1100;
        let rug_fuzz_1 = 0b0011_0100_1000_0000;
        let original: u16 = rug_fuzz_0;
        let expected: u16 = rug_fuzz_1;
        debug_assert_eq!(original.reverse_bits(), expected);
        let _rug_ed_tests_llm_16_1517_llm_16_1517_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1518 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_1518_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b1011_0001_0010_1111;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = 20;
        let value: u16 = rug_fuzz_0;
        let rotated = value.rotate_left(rug_fuzz_1);
        debug_assert_eq!(rotated, 0b0010_1111_1011_0001);
        debug_assert_eq!(value.rotate_left(rug_fuzz_2), value);
        let bit_width = rug_fuzz_3;
        debug_assert_eq!(value.rotate_left(bit_width), value);
        let rotate_by = rug_fuzz_4;
        debug_assert_eq!(
            value.rotate_left(rotate_by), value.rotate_left(rotate_by % bit_width)
        );
        let _rug_ed_tests_llm_16_1518_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1519_llm_16_1519 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_1519_llm_16_1519_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b0101_0111_1001_0101;
        let rug_fuzz_1 = 8;
        let value: u16 = rug_fuzz_0;
        let result = PrimInt::rotate_right(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b1001_0101_0101_0111);
        let _rug_ed_tests_llm_16_1519_llm_16_1519_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1520_llm_16_1520 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1520_llm_16_1520_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b1001_0000_0000_0000;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0b0000_0000_0001_0000;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 0b0000_0000_0000_0001;
        let rug_fuzz_7 = 15;
        let rug_fuzz_8 = 0b0101_0101_0101_0101;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0b0000_0001_0000_0000;
        let rug_fuzz_11 = 16;
        let value: u16 = rug_fuzz_0;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b0010_0000_0000_0000);
        let value: u16 = rug_fuzz_2;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_3);
        debug_assert_eq!(result, 0b1000_0000_0000_0000);
        let value: u16 = rug_fuzz_4;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_5);
        debug_assert_eq!(result, 0b0000_0001_0000_0000);
        let value: u16 = rug_fuzz_6;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_7);
        debug_assert_eq!(result, 0b1000_0000_0000_0000);
        let value: u16 = rug_fuzz_8;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_9);
        debug_assert_eq!(result, 0b0101_0101_0101_0101);
        let value: u16 = rug_fuzz_10;
        let result = <u16 as PrimInt>::signed_shl(value, rug_fuzz_11);
        debug_assert_eq!(result, 0b0000_0000_0000_0000);
        let _rug_ed_tests_llm_16_1520_llm_16_1520_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1521_llm_16_1521 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_1521_llm_16_1521_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 0b1000_0000_0000_0000;
        let rug_fuzz_1 = 1;
        let value: u16 = rug_fuzz_0;
        let shifted = <u16 as PrimInt>::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(shifted, 0b1100_0000_0000_0000);
        let _rug_ed_tests_llm_16_1521_llm_16_1521_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1522_llm_16_1522 {
    use crate::int::PrimInt;
    #[test]
    fn test_u16_swap_bytes() {
        let _rug_st_tests_llm_16_1522_llm_16_1522_rrrruuuugggg_test_u16_swap_bytes = 0;
        let rug_fuzz_0 = 0x1234_u16;
        let rug_fuzz_1 = 0x0000_u16;
        let rug_fuzz_2 = 0xFFFF_u16;
        let rug_fuzz_3 = 0x00FF_u16;
        let rug_fuzz_4 = 0xFF00_u16;
        debug_assert_eq!(rug_fuzz_0.swap_bytes(), 0x3412);
        debug_assert_eq!(rug_fuzz_1.swap_bytes(), 0x0000);
        debug_assert_eq!(rug_fuzz_2.swap_bytes(), 0xFFFF);
        debug_assert_eq!(rug_fuzz_3.swap_bytes(), 0xFF00);
        debug_assert_eq!(rug_fuzz_4.swap_bytes(), 0x00FF);
        let _rug_ed_tests_llm_16_1522_llm_16_1522_rrrruuuugggg_test_u16_swap_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1523 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let little_endian_value: u16 = 0x1234;
        let big_endian_value: u16 = little_endian_value.to_be();
        if cfg!(target_endian = "little") {
            assert_eq!(big_endian_value, 0x3412);
        } else {
            assert_eq!(big_endian_value, 0x1234);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1524_llm_16_1524 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_le() {
        let big_endian_value = 0x1234u16;
        let little_endian_value = big_endian_value.to_le();
        if cfg!(target_endian = "little") {
            assert_eq!(little_endian_value, big_endian_value);
        } else if cfg!(target_endian = "big") {
            assert_eq!(little_endian_value, big_endian_value.swap_bytes());
        } else {
            panic!("Unknown target endianness");
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1525_llm_16_1525 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1525_llm_16_1525_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 2u16;
        let rug_fuzz_3 = 3u16;
        let rug_fuzz_4 = 4u16;
        let rug_fuzz_5 = 0xff00u16;
        let rug_fuzz_6 = 0x00ffu16;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 8);
        debug_assert_eq!(u16::MAX.trailing_ones(), 16);
        let _rug_ed_tests_llm_16_1525_llm_16_1525_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1526_llm_16_1526 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1526_llm_16_1526_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 2u16;
        let rug_fuzz_3 = 8u16;
        let rug_fuzz_4 = 16u16;
        let rug_fuzz_5 = 1024u16;
        let rug_fuzz_6 = 0b1101000u16;
        let rug_fuzz_7 = 0b111u16;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 10);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 3);
        debug_assert_eq!(u16::MIN.trailing_zeros(), 16);
        debug_assert_eq!(rug_fuzz_7.trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_1526_llm_16_1526_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1527_llm_16_1527 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_llm_16_1527_llm_16_1527_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 4;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 15;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 15;
        let rug_fuzz_12 = 0x8000;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 16;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 32;
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_4, rug_fuzz_5), 16);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_6, rug_fuzz_7), 256);
        debug_assert_eq!(
            < u16 as PrimInt > ::unsigned_shl(rug_fuzz_8, rug_fuzz_9), 32768
        );
        debug_assert_eq!(
            < u16 as PrimInt > ::unsigned_shl(rug_fuzz_10, rug_fuzz_11), 0x8000
        );
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_12, rug_fuzz_13), 0);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_14, rug_fuzz_15), 0);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shl(rug_fuzz_16, rug_fuzz_17), 0);
        let _rug_ed_tests_llm_16_1527_llm_16_1527_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1528_llm_16_1528 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shr_basic() {
        let _rug_st_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_basic = 0;
        let rug_fuzz_0 = 0b1000_0000_0000_0000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b1000_0000_0000_0000;
        let rug_fuzz_3 = 15;
        debug_assert_eq!(
            < u16 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1),
            0b0100_0000_0000_0000
        );
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 1);
        let _rug_ed_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_basic = 0;
    }
    #[test]
    fn unsigned_shr_zero() {
        let _rug_st_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_zero = 0;
    }
    #[test]
    fn unsigned_shr_all_bits() {
        let _rug_st_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_all_bits = 0;
        let rug_fuzz_0 = 0xFFFF;
        let rug_fuzz_1 = 8;
        debug_assert_eq!(
            < u16 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 0x00FF
        );
        let _rug_ed_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_all_bits = 0;
    }
    #[test]
    fn unsigned_shr_overflow_shift() {
        let _rug_st_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_overflow_shift = 0;
        let rug_fuzz_0 = 0xFFFF;
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = 0xFFFF;
        let rug_fuzz_3 = 32;
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 0);
        debug_assert_eq!(< u16 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 0);
        let _rug_ed_tests_llm_16_1528_llm_16_1528_rrrruuuugggg_unsigned_shr_overflow_shift = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1615 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_1615_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        let rug_fuzz_2 = 0b1010u32;
        let rug_fuzz_3 = 0b1111u32;
        let rug_fuzz_4 = 0xFFFFFFFFu32;
        let rug_fuzz_5 = 0x0F0F0F0Fu32;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 32);
        debug_assert_eq!(rug_fuzz_5.count_ones(), 16);
        let _rug_ed_tests_llm_16_1615_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1616 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1616_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        let rug_fuzz_2 = 0b00010000u32;
        let rug_fuzz_3 = 0b01010101010101010101010101010101u32;
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 32);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), 31);
        debug_assert_eq!(rug_fuzz_2.count_zeros(), 27);
        debug_assert_eq!(rug_fuzz_3.count_zeros(), 16);
        debug_assert_eq!(u32::MAX.count_zeros(), 0);
        let _rug_ed_tests_llm_16_1616_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1617_llm_16_1617 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_1617_llm_16_1617_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x12345678u32;
        let rug_fuzz_1 = 0x12345678u32;
        let big_endian_bytes = rug_fuzz_0.to_be();
        let expected = rug_fuzz_1;
        debug_assert_eq!(u32::from_be(big_endian_bytes), expected);
        let _rug_ed_tests_llm_16_1617_llm_16_1617_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1618_llm_16_1618 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        let big_endian = if cfg!(target_endian = "big") { true } else { false };
        let x: u32 = 0x12345678;
        if big_endian {
            let bytes = x.to_le_bytes();
            let expected = u32::from_le_bytes(bytes);
            assert_eq!(< u32 as PrimInt >::from_le(x), expected);
        } else {
            assert_eq!(< u32 as PrimInt >::from_le(x), x);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1619 {
    use super::*;
    use crate::*;
    #[cfg(has_leading_trailing_ones)]
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1619_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b1100_0000_0000_0000_0000_0000_0000_0000u32;
        let rug_fuzz_3 = 0b1111_1111_0000_0000_0000_0000_0000_0000u32;
        let rug_fuzz_4 = 0b1000_0000_1000_0000_0000_0000_0000_0000u32;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 2);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 8);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 1);
        debug_assert_eq!(u32::MAX.leading_ones(), 32);
        let _rug_ed_tests_llm_16_1619_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1620_llm_16_1620 {
    use crate::int::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1620_llm_16_1620_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        let rug_fuzz_2 = 0xffffu32;
        let rug_fuzz_3 = 0xffffffffu32;
        let rug_fuzz_4 = 0x80000000u32;
        let rug_fuzz_5 = 0x00ff0000u32;
        debug_assert_eq!(rug_fuzz_0.leading_zeros(), 32);
        debug_assert_eq!(rug_fuzz_1.leading_zeros(), 31);
        debug_assert_eq!(rug_fuzz_2.leading_zeros(), 16);
        debug_assert_eq!(rug_fuzz_3.leading_zeros(), 0);
        debug_assert_eq!(rug_fuzz_4.leading_zeros(), 1);
        debug_assert_eq!(rug_fuzz_5.leading_zeros(), 8);
        let _rug_ed_tests_llm_16_1620_llm_16_1620_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1621_llm_16_1621 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1621_llm_16_1621_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u32;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 3u32;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 5u32;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0u32;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 1u32;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 1;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 8);
        debug_assert_eq!(rug_fuzz_2.pow(rug_fuzz_3), 9);
        debug_assert_eq!(rug_fuzz_4.pow(rug_fuzz_5), 1);
        debug_assert_eq!(rug_fuzz_6.pow(rug_fuzz_7), 0);
        debug_assert_eq!(rug_fuzz_8.pow(rug_fuzz_9), 1);
        debug_assert_eq!(u32::MAX.pow(rug_fuzz_10), u32::MAX);
        let _rug_ed_tests_llm_16_1621_llm_16_1621_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1622 {
    use super::*;
    use crate::*;
    #[test]
    #[cfg(has_reverse_bits)]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_1622_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b00000000000000000000000000000001;
        let rug_fuzz_1 = 0b00000000000000000000000000000000;
        let rug_fuzz_2 = 0b10000000000000000000000000000000;
        let rug_fuzz_3 = 0b01010101010101010101010101010101;
        let rug_fuzz_4 = 0b11111111111111111111111111111111;
        debug_assert_eq!(
            < u32 as int::PrimInt > ::reverse_bits(rug_fuzz_0),
            0b10000000000000000000000000000000
        );
        debug_assert_eq!(
            < u32 as int::PrimInt > ::reverse_bits(rug_fuzz_1),
            0b00000000000000000000000000000000
        );
        debug_assert_eq!(
            < u32 as int::PrimInt > ::reverse_bits(rug_fuzz_2),
            0b00000000000000000000000000000001
        );
        debug_assert_eq!(
            < u32 as int::PrimInt > ::reverse_bits(rug_fuzz_3),
            0b10101010101010101010101010101010
        );
        debug_assert_eq!(
            < u32 as int::PrimInt > ::reverse_bits(rug_fuzz_4),
            0b11111111111111111111111111111111
        );
        let _rug_ed_tests_llm_16_1622_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1623_llm_16_1623 {
    use crate::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_1623_llm_16_1623_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_0010_0100_1000_1001_0010_0100_1001;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0b0010_0100_1000_1001_0010_0100_1001_0001;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 32;
        let rug_fuzz_5 = 36;
        let rug_fuzz_6 = 4;
        let rug_fuzz_7 = 64;
        let value: u32 = rug_fuzz_0;
        let result = value.rotate_left(rug_fuzz_1);
        let expected = rug_fuzz_2;
        debug_assert_eq!(
            result, expected, "Rotating left 4 bits should yield the expected result"
        );
        let result = value.rotate_left(rug_fuzz_3);
        debug_assert_eq!(
            result, value, "Rotating left 0 bits should yield the same value"
        );
        let result = value.rotate_left(rug_fuzz_4);
        debug_assert_eq!(
            result, value,
            "Rotating left by the number of bits in the type should yield the same value"
        );
        let result = value.rotate_left(rug_fuzz_5);
        let expected = value.rotate_left(rug_fuzz_6);
        debug_assert_eq!(
            result, expected,
            "Rotating left by more than the number of bits in the type should work correctly"
        );
        let result = value.rotate_left(rug_fuzz_7);
        debug_assert_eq!(
            result, value,
            "Rotating left by a multiple of the number of bits in the type should yield the same value"
        );
        let _rug_ed_tests_llm_16_1623_llm_16_1623_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1624_llm_16_1624 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_1624_llm_16_1624_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b1011_0000_0000_0000_0000_0000_1101_0011;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 0b0011_1011_0000_0000_0000_0000_0000_1101;
        let rug_fuzz_3 = 32;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 40;
        let rug_fuzz_6 = 8;
        let value: u32 = rug_fuzz_0;
        let result = u32::rotate_right(value, rug_fuzz_1);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let result = u32::rotate_right(value, rug_fuzz_3);
        debug_assert_eq!(result, value);
        let result = u32::rotate_right(value, rug_fuzz_4);
        debug_assert_eq!(result, value);
        let result = u32::rotate_right(value, rug_fuzz_5);
        let expected = u32::rotate_right(value, rug_fuzz_6);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1624_llm_16_1624_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1625_llm_16_1625 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1625_llm_16_1625_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let value: u32 = rug_fuzz_0;
        let shift: u32 = rug_fuzz_1;
        let result = PrimInt::signed_shl(value, shift);
        debug_assert_eq!(result, 4);
        let _rug_ed_tests_llm_16_1625_llm_16_1625_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1626_llm_16_1626 {
    use super::*;
    use crate::*;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_1626_llm_16_1626_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 0b11110000;
        let rug_fuzz_1 = 4;
        let val: u32 = rug_fuzz_0;
        let shift = rug_fuzz_1;
        let result = val.signed_shr(shift);
        debug_assert_eq!(result, 0b1111);
        let _rug_ed_tests_llm_16_1626_llm_16_1626_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1627 {
    use super::*;
    use crate::*;
    #[test]
    fn test_swap_bytes_u32() {
        let _rug_st_tests_llm_16_1627_rrrruuuugggg_test_swap_bytes_u32 = 0;
        let rug_fuzz_0 = 0x12345678u32;
        debug_assert_eq!(rug_fuzz_0.swap_bytes(), 0x78563412u32);
        let _rug_ed_tests_llm_16_1627_rrrruuuugggg_test_swap_bytes_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1628 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let num: u32 = 0x12345678;
        let big_endian = num.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(big_endian, num);
        } else {
            assert_eq!(big_endian, 0x78563412);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1629 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_le() {
        let big_endian: u32 = u32::from_be(0x12345678);
        let little_endian: u32 = big_endian.to_le();
        if cfg!(target_endian = "little") {
            assert_eq!(little_endian, big_endian);
        } else if cfg!(target_endian = "big") {
            assert_eq!(little_endian, u32::from_be_bytes(big_endian.to_be_bytes()));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1630_llm_16_1630 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1630_llm_16_1630_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        let rug_fuzz_2 = 2u32;
        let rug_fuzz_3 = 3u32;
        let rug_fuzz_4 = 4u32;
        let rug_fuzz_5 = 5u32;
        let rug_fuzz_6 = 0b1110_1000u32;
        let rug_fuzz_7 = 0b0001_1111u32;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_7.trailing_ones(), 5);
        debug_assert_eq!(u32::MAX.trailing_ones(), 32);
        let _rug_ed_tests_llm_16_1630_llm_16_1630_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1631_llm_16_1631 {
    use crate::int::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1631_llm_16_1631_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        let rug_fuzz_2 = 2u32;
        let rug_fuzz_3 = 0b0010_1000u32;
        let rug_fuzz_4 = 0b1000_0000u32;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 7);
        debug_assert_eq!(u32::max_value().trailing_zeros(), 0);
        debug_assert_eq!((u32::max_value() - rug_fuzz_5).trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_1631_llm_16_1631_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1632_llm_16_1632 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_llm_16_1632_llm_16_1632_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 31;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        debug_assert_eq!(< u32 as PrimInt > ::unsigned_shl(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u32 as PrimInt > ::unsigned_shl(rug_fuzz_2, rug_fuzz_3), 32);
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shl(rug_fuzz_4, rug_fuzz_5), 1 << 31
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shl(u32::MAX, rug_fuzz_6), u32::MAX << 1
        );
        debug_assert_eq!(< u32 as PrimInt > ::unsigned_shl(rug_fuzz_7, rug_fuzz_8), 0);
        let _rug_ed_tests_llm_16_1632_llm_16_1632_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1633_llm_16_1633 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shr_test() {
        let _rug_st_tests_llm_16_1633_llm_16_1633_rrrruuuugggg_unsigned_shr_test = 0;
        let rug_fuzz_0 = 8u32;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1u32;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0u32;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 31;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0u32;
        let rug_fuzz_10 = 0;
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 4u32
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 0u32
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(rug_fuzz_4, rug_fuzz_5), 0u32
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(u32::MAX, rug_fuzz_6), 0x7FFF_FFFFu32
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(u32::MAX, rug_fuzz_7), 0x1u32
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(u32::MAX, rug_fuzz_8), u32::MAX
        );
        debug_assert_eq!(
            < u32 as PrimInt > ::unsigned_shr(rug_fuzz_9, rug_fuzz_10), 0u32
        );
        let _rug_ed_tests_llm_16_1633_llm_16_1633_rrrruuuugggg_unsigned_shr_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1720_llm_16_1720 {
    use crate::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_1720_llm_16_1720_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0b0000;
        let rug_fuzz_1 = 0b0001;
        let rug_fuzz_2 = 0b0010;
        let rug_fuzz_3 = 0b0011;
        let rug_fuzz_4 = 0b0101;
        let rug_fuzz_5 = 0b1111;
        let rug_fuzz_6 = 0xffff_ffff_ffff_ffff;
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_0), 0);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_1), 1);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_2), 1);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_4), 2);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_5), 4);
        debug_assert_eq!(< u64 as PrimInt > ::count_ones(rug_fuzz_6), 64);
        let _rug_ed_tests_llm_16_1720_llm_16_1720_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1721 {
    use crate::PrimInt;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1721_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 1_u64;
        let rug_fuzz_2 = 0xF0_u64;
        let rug_fuzz_3 = 0xFFFFFFFF_u64;
        let rug_fuzz_4 = 0xFFFFFFFFFFFFFFFF_u64;
        let rug_fuzz_5 = 0x8000000000000000_u64;
        let rug_fuzz_6 = 0x7FFFFFFFFFFFFFFF_u64;
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 64);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), 63);
        debug_assert_eq!(rug_fuzz_2.count_zeros(), 60);
        debug_assert_eq!(rug_fuzz_3.count_zeros(), 32);
        debug_assert_eq!(rug_fuzz_4.count_zeros(), 0);
        debug_assert_eq!(rug_fuzz_5.count_zeros(), 0);
        debug_assert_eq!(rug_fuzz_6.count_zeros(), 1);
        let _rug_ed_tests_llm_16_1721_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1722_llm_16_1722 {
    use crate::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_1722_llm_16_1722_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x12345678u64;
        let big_endian_bytes = rug_fuzz_0.to_be();
        let num = <u64 as PrimInt>::from_be(big_endian_bytes);
        debug_assert_eq!(num, 0x12345678u64);
        let _rug_ed_tests_llm_16_1722_llm_16_1722_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1723_llm_16_1723 {
    use crate::PrimInt;
    #[test]
    fn test_from_le() {
        let native_endian = 0x123456789ABCDEF0u64;
        if cfg!(target_endian = "little") {
            assert_eq!(u64::from_le(native_endian), native_endian);
        } else {
            let swapped_endian = native_endian.swap_bytes();
            assert_eq!(u64::from_le(native_endian), swapped_endian);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1724_llm_16_1724 {
    use crate::int::PrimInt;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1724_llm_16_1724_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        let rug_fuzz_2 = 0xFFFFFFFFFFFFFFFFu64;
        let rug_fuzz_3 = 0x8000000000000000u64;
        let rug_fuzz_4 = 0xF000000000000000u64;
        let rug_fuzz_5 = 0x0F00000000000000u64;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 64);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 1);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 0);
        let _rug_ed_tests_llm_16_1724_llm_16_1724_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1725_llm_16_1725 {
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1725_llm_16_1725_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0xFFFFFFFF;
        let rug_fuzz_5 = 0x1FFFFFFFF;
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_0), 64);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_1), 63);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_2), 62);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_3), 62);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_4), 32);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(rug_fuzz_5), 31);
        debug_assert_eq!(< u64 as PrimInt > ::leading_zeros(u64::MAX), 0);
        let _rug_ed_tests_llm_16_1725_llm_16_1725_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1726_llm_16_1726 {
    use crate::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1726_llm_16_1726_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2u64;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 5u64;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0u64;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1u64;
        let rug_fuzz_8 = 32;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 8);
        debug_assert_eq!(rug_fuzz_2.pow(rug_fuzz_3), 1);
        debug_assert_eq!(rug_fuzz_4.pow(rug_fuzz_5), 0);
        debug_assert_eq!(u64::MAX.pow(rug_fuzz_6), u64::MAX);
        debug_assert_eq!(rug_fuzz_7.pow(rug_fuzz_8), 1);
        let _rug_ed_tests_llm_16_1726_llm_16_1726_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1727 {
    use super::*;
    use crate::*;
    #[test]
    fn test_reverse_bits_u64() {
        let _rug_st_tests_llm_16_1727_rrrruuuugggg_test_reverse_bits_u64 = 0;
        let rug_fuzz_0 = 0b0000000000000000000000000000000000000000000000000000000000000001u64;
        let rug_fuzz_1 = 0b0000000000000000000000000000000000000000000000000000000000000000u64;
        let rug_fuzz_2 = 0b1111111111111111111111111111111111111111111111111111111111111111u64;
        let rug_fuzz_3 = 0b1000000000000000000000000000000000000000000000000000000000000001u64;
        let rug_fuzz_4 = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
        debug_assert_eq!(
            < u64 as int::PrimInt > ::reverse_bits(rug_fuzz_0),
            0b1000000000000000000000000000000000000000000000000000000000000000u64
        );
        debug_assert_eq!(
            < u64 as int::PrimInt > ::reverse_bits(rug_fuzz_1),
            0b0000000000000000000000000000000000000000000000000000000000000000u64
        );
        debug_assert_eq!(
            < u64 as int::PrimInt > ::reverse_bits(rug_fuzz_2),
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        );
        debug_assert_eq!(
            < u64 as int::PrimInt > ::reverse_bits(rug_fuzz_3),
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        );
        debug_assert_eq!(
            < u64 as int::PrimInt > ::reverse_bits(rug_fuzz_4),
            0b1010101010101010101010101010101010101010101010101010101010101010u64
        );
        let _rug_ed_tests_llm_16_1727_rrrruuuugggg_test_reverse_bits_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1728_llm_16_1728 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_llm_16_1728_llm_16_1728_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_u64;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b0001_u64;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0b0001_u64;
        let rug_fuzz_5 = 64;
        let rug_fuzz_6 = 0b0001_u64;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0b1001_u64;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0b1001_u64;
        let rug_fuzz_11 = 64;
        let rug_fuzz_12 = 0b1001_u64;
        let rug_fuzz_13 = 65;
        let rug_fuzz_14 = 0b1001_u64;
        let rug_fuzz_15 = 67;
        debug_assert_eq!(rug_fuzz_0.rotate_left(rug_fuzz_1), 0b0010);
        debug_assert_eq!(rug_fuzz_2.rotate_left(rug_fuzz_3), 0b0100);
        debug_assert_eq!(rug_fuzz_4.rotate_left(rug_fuzz_5), 0b0001);
        debug_assert_eq!(rug_fuzz_6.rotate_left(rug_fuzz_7), 0b0001);
        debug_assert_eq!(rug_fuzz_8.rotate_left(rug_fuzz_9), 0b0011);
        debug_assert_eq!(rug_fuzz_10.rotate_left(rug_fuzz_11), 0b1001);
        debug_assert_eq!(rug_fuzz_12.rotate_left(rug_fuzz_13), 0b0011);
        debug_assert_eq!(rug_fuzz_14.rotate_left(rug_fuzz_15), 0b1001);
        let _rug_ed_tests_llm_16_1728_llm_16_1728_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1729_llm_16_1729 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_1729_llm_16_1729_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b1011;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0b1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011;
        let value: u64 = rug_fuzz_0;
        let n = rug_fuzz_1;
        let result = value.rotate_right(n);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1729_llm_16_1729_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1730_llm_16_1730 {
    use crate::int::PrimInt;
    #[test]
    fn signed_shl_basic() {
        let _rug_st_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_basic = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 15;
        let rug_fuzz_11 = 1;
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_8, rug_fuzz_9), 32);
        debug_assert_eq!(< u64 as PrimInt > ::signed_shl(rug_fuzz_10, rug_fuzz_11), 30);
        let _rug_ed_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_basic = 0;
    }
    #[test]
    fn signed_shl_edge_cases() {
        let _rug_st_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_edge_cases = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 63;
        let max_value = u64::MAX;
        debug_assert_eq!(
            < u64 as PrimInt > ::signed_shl(max_value, rug_fuzz_0), max_value
        );
        debug_assert_eq!(
            < u64 as PrimInt > ::signed_shl(max_value >> rug_fuzz_1, rug_fuzz_2),
            max_value - 1
        );
        debug_assert_eq!(
            < u64 as PrimInt > ::signed_shl(rug_fuzz_3, rug_fuzz_4), 1_u64
            .rotate_left(63)
        );
        let _rug_ed_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_edge_cases = 0;
    }
    #[test]
    #[should_panic]
    fn signed_shl_overflow() {
        let _rug_st_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_overflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 65;
        let _ = <u64 as PrimInt>::signed_shl(rug_fuzz_0, rug_fuzz_1);
        let _rug_ed_tests_llm_16_1730_llm_16_1730_rrrruuuugggg_signed_shl_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1731_llm_16_1731 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_1731_llm_16_1731_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 0b0100_0000;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0b0101;
        let rug_fuzz_5 = 64;
        let rug_fuzz_6 = 0b0101;
        let rug_fuzz_7 = 0;
        let pos_num: u64 = rug_fuzz_0;
        debug_assert_eq!(
            < u64 as PrimInt > ::signed_shr(pos_num, rug_fuzz_1), 0b0001_0000
        );
        let signed_num: u64 = rug_fuzz_2;
        debug_assert_eq!(
            < u64 as PrimInt > ::signed_shr(signed_num, rug_fuzz_3),
            0b0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
        );
        let num: u64 = rug_fuzz_4;
        debug_assert_eq!(< u64 as PrimInt > ::signed_shr(num, rug_fuzz_5), 0b0);
        let num: u64 = rug_fuzz_6;
        debug_assert_eq!(< u64 as PrimInt > ::signed_shr(num, rug_fuzz_7), 0b0101);
        let _rug_ed_tests_llm_16_1731_llm_16_1731_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1732_llm_16_1732 {
    use crate::int::PrimInt;
    #[test]
    fn test_swap_bytes_u64() {
        let _rug_st_tests_llm_16_1732_llm_16_1732_rrrruuuugggg_test_swap_bytes_u64 = 0;
        let rug_fuzz_0 = 0x123456789ABCDEF0;
        let x: u64 = rug_fuzz_0;
        let swapped = <u64 as PrimInt>::swap_bytes(x);
        debug_assert_eq!(swapped, 0xF0DEBC9A78563412);
        let _rug_ed_tests_llm_16_1732_llm_16_1732_rrrruuuugggg_test_swap_bytes_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1733 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_be() {
        let native_endian = 0x0123456789ABCDEFu64;
        let big_endian = native_endian.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(native_endian, big_endian);
        } else {
            assert_eq!(big_endian, 0xEFCDAB8967452301u64);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1734_llm_16_1734 {
    use crate::int::PrimInt;
    #[test]
    fn test_to_le() {
        let big_endian = 0x123456789ABCDEF0u64;
        let expected = big_endian.to_le();
        if cfg!(target_endian = "little") {
            assert_eq!(big_endian, expected);
        } else if cfg!(target_endian = "big") {
            let swapped = expected.to_be();
            assert_eq!(big_endian, swapped);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1735 {
    use super::*;
    use crate::*;
    #[test]
    #[cfg(has_leading_trailing_ones)]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1735_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        let rug_fuzz_2 = 2u64;
        let rug_fuzz_3 = 3u64;
        let rug_fuzz_4 = 0b1001000u64;
        let rug_fuzz_5 = 0b1001001u64;
        let rug_fuzz_6 = 0b1011111u64;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 5);
        debug_assert_eq!(u64::MAX.trailing_ones(), 64);
        let _rug_ed_tests_llm_16_1735_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1736 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1736_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        let rug_fuzz_2 = 2u64;
        let rug_fuzz_3 = 4u64;
        let rug_fuzz_4 = 8u64;
        let rug_fuzz_5 = 16u64;
        let rug_fuzz_6 = 0b1010000u64;
        let rug_fuzz_7 = 0b100000000u64;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1u64;
        let rug_fuzz_10 = 63;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_7.trailing_zeros(), 8);
        debug_assert_eq!(u64::MAX.trailing_zeros(), 0);
        debug_assert_eq!((u64::MAX - rug_fuzz_8).trailing_zeros(), 0);
        debug_assert_eq!(u64::MAX.trailing_zeros(), 0);
        debug_assert_eq!((rug_fuzz_9 << rug_fuzz_10).trailing_zeros(), 63);
        let _rug_ed_tests_llm_16_1736_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1737_llm_16_1737 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shl_test() {
        let _rug_st_tests_llm_16_1737_llm_16_1737_rrrruuuugggg_unsigned_shl_test = 0;
        let rug_fuzz_0 = 0x0123456789ABCDEF;
        let rug_fuzz_1 = 4;
        let value: u64 = rug_fuzz_0;
        let shift_amount: u32 = rug_fuzz_1;
        let result = <u64 as PrimInt>::unsigned_shl(value, shift_amount);
        debug_assert_eq!(result, 0x123456789ABCDEF0);
        let _rug_ed_tests_llm_16_1737_llm_16_1737_rrrruuuugggg_unsigned_shl_test = 0;
    }
    #[test]
    #[should_panic]
    fn unsigned_shl_overflow_test() {
        let _rug_st_tests_llm_16_1737_llm_16_1737_rrrruuuugggg_unsigned_shl_overflow_test = 0;
        let rug_fuzz_0 = 1;
        let value: u64 = u64::MAX;
        let shift_amount: u32 = rug_fuzz_0;
        let _result = <u64 as PrimInt>::unsigned_shl(value, shift_amount);
        let _rug_ed_tests_llm_16_1737_llm_16_1737_rrrruuuugggg_unsigned_shl_overflow_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1738_llm_16_1738 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shr() {
        let _rug_st_tests_llm_16_1738_llm_16_1738_rrrruuuugggg_test_unsigned_shr = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(< u64 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 4);
        debug_assert_eq!(< u64 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< u64 as PrimInt > ::unsigned_shr(rug_fuzz_4, rug_fuzz_5), 1);
        debug_assert_eq!(< u64 as PrimInt > ::unsigned_shr(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(
            < u64 as PrimInt > ::unsigned_shr(u64::MAX, rug_fuzz_8), u64::MAX >> 1
        );
        debug_assert_eq!(
            < u64 as PrimInt > ::unsigned_shr(u64::MAX, u64::BITS - rug_fuzz_9), 1
        );
        let _rug_ed_tests_llm_16_1738_llm_16_1738_rrrruuuugggg_test_unsigned_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1826 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_1826_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0b00000000;
        let rug_fuzz_1 = 0b00000001;
        let rug_fuzz_2 = 0b00010010;
        let rug_fuzz_3 = 0b10000001;
        let rug_fuzz_4 = 0b11111111;
        debug_assert_eq!(< u8 as int::PrimInt > ::count_ones(rug_fuzz_0), 0);
        debug_assert_eq!(< u8 as int::PrimInt > ::count_ones(rug_fuzz_1), 1);
        debug_assert_eq!(< u8 as int::PrimInt > ::count_ones(rug_fuzz_2), 2);
        debug_assert_eq!(< u8 as int::PrimInt > ::count_ones(rug_fuzz_3), 2);
        debug_assert_eq!(< u8 as int::PrimInt > ::count_ones(rug_fuzz_4), 8);
        let _rug_ed_tests_llm_16_1826_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1827 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1827_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0b0000_1111u8;
        let rug_fuzz_1 = 0b1111_0000u8;
        let rug_fuzz_2 = 0b1111_1111u8;
        let rug_fuzz_3 = 0b0000_0000u8;
        let rug_fuzz_4 = 0b0101_0101u8;
        let rug_fuzz_5 = 0b1010_1010u8;
        let rug_fuzz_6 = 0b1000_0000u8;
        let rug_fuzz_7 = 0b0000_0001u8;
        let rug_fuzz_8 = 0b1001_1001u8;
        let rug_fuzz_9 = 0b0110_0110u8;
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 4);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), 4);
        debug_assert_eq!(rug_fuzz_2.count_zeros(), 0);
        debug_assert_eq!(rug_fuzz_3.count_zeros(), 8);
        debug_assert_eq!(rug_fuzz_4.count_zeros(), 4);
        debug_assert_eq!(rug_fuzz_5.count_zeros(), 4);
        debug_assert_eq!(rug_fuzz_6.count_zeros(), 7);
        debug_assert_eq!(rug_fuzz_7.count_zeros(), 7);
        debug_assert_eq!(rug_fuzz_8.count_zeros(), 4);
        debug_assert_eq!(rug_fuzz_9.count_zeros(), 4);
        let _rug_ed_tests_llm_16_1827_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1828_llm_16_1828 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_llm_16_1828_llm_16_1828_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x12;
        let rug_fuzz_1 = 0x12;
        let big_endian_value: u8 = rug_fuzz_0;
        let expected_value: u8 = rug_fuzz_1;
        debug_assert_eq!(u8::from_be(big_endian_value), expected_value);
        let _rug_ed_tests_llm_16_1828_llm_16_1828_rrrruuuugggg_test_from_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1829_llm_16_1829 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        let big_endian = if cfg!(target_endian = "big") { true } else { false };
        let x: u8 = 0x12;
        let res = u8::from_le(x);
        if big_endian {
            assert_eq!(res, 0x12);
        } else {
            assert_eq!(res, 0x12);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1830 {
    use super::*;
    use crate::*;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1830_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0b00000000u8;
        let rug_fuzz_1 = 0b10000000u8;
        let rug_fuzz_2 = 0b11000000u8;
        let rug_fuzz_3 = 0b11100000u8;
        let rug_fuzz_4 = 0b11110000u8;
        let rug_fuzz_5 = 0b11111000u8;
        let rug_fuzz_6 = 0b11111100u8;
        let rug_fuzz_7 = 0b11111110u8;
        let rug_fuzz_8 = 0b11111111u8;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 2);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 3);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 5);
        debug_assert_eq!(rug_fuzz_6.leading_ones(), 6);
        debug_assert_eq!(rug_fuzz_7.leading_ones(), 7);
        debug_assert_eq!(rug_fuzz_8.leading_ones(), 8);
        let _rug_ed_tests_llm_16_1830_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1831_llm_16_1831 {
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1831_llm_16_1831_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0b0000_0001;
        let rug_fuzz_1 = 0b0000_0010;
        let rug_fuzz_2 = 0b0000_0100;
        let rug_fuzz_3 = 0b0000_1000;
        let rug_fuzz_4 = 0b0001_0000;
        let rug_fuzz_5 = 0b0010_0000;
        let rug_fuzz_6 = 0b0100_0000;
        let rug_fuzz_7 = 0b1000_0000;
        let rug_fuzz_8 = 0b0000_0000;
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_0), 7);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_1), 6);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_2), 5);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_3), 4);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_4), 3);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_5), 2);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_6), 1);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_7), 0);
        debug_assert_eq!(u8::leading_zeros(rug_fuzz_8), 8);
        let _rug_ed_tests_llm_16_1831_llm_16_1831_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1832_llm_16_1832 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1832_llm_16_1832_rrrruuuugggg_test_pow = 0;
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
        debug_assert_eq!(u8::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(u8::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(u8::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(u8::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(u8::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(u8::pow(rug_fuzz_10, rug_fuzz_11), 32);
        debug_assert_eq!(u8::pow(rug_fuzz_12, rug_fuzz_13), 64);
        debug_assert_eq!(u8::pow(rug_fuzz_14, rug_fuzz_15), 128);
        debug_assert_eq!(u8::pow(rug_fuzz_16, rug_fuzz_17), 0);
        let _rug_ed_tests_llm_16_1832_llm_16_1832_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1833_llm_16_1833 {
    use super::*;
    use crate::*;
    use crate::int::PrimInt;
    #[test]
    fn test_reverse_bits_u8() {
        let _rug_st_tests_llm_16_1833_llm_16_1833_rrrruuuugggg_test_reverse_bits_u8 = 0;
        let rug_fuzz_0 = 0b00000000;
        let rug_fuzz_1 = 0b00000001;
        let rug_fuzz_2 = 0b00000010;
        let rug_fuzz_3 = 0b00000100;
        let rug_fuzz_4 = 0b00001000;
        let rug_fuzz_5 = 0b00010000;
        let rug_fuzz_6 = 0b00100000;
        let rug_fuzz_7 = 0b01000000;
        let rug_fuzz_8 = 0b10000000;
        let rug_fuzz_9 = 0b11111111;
        let rug_fuzz_10 = 0b10101010;
        let rug_fuzz_11 = 0b01010101;
        let rug_fuzz_12 = 0b00110011;
        let rug_fuzz_13 = 0b11001100;
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_0), 0b00000000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_1), 0b10000000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_2), 0b01000000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_3), 0b00100000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_4), 0b00010000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_5), 0b00001000);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_6), 0b00000100);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_7), 0b00000010);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_8), 0b00000001);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_9), 0b11111111);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_10), 0b01010101);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_11), 0b10101010);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_12), 0b11001100);
        debug_assert_eq!(< u8 as PrimInt > ::reverse_bits(rug_fuzz_13), 0b00110011);
        let _rug_ed_tests_llm_16_1833_llm_16_1833_rrrruuuugggg_test_reverse_bits_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1834 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_left_u8() {
        let _rug_st_tests_llm_16_1834_rrrruuuugggg_test_rotate_left_u8 = 0;
        let rug_fuzz_0 = 0b1011_0001;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 8;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 16;
        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(value.rotate_left(rug_fuzz_1), 0b1011_0001);
        debug_assert_eq!(value.rotate_left(rug_fuzz_2), 0b0001_1011);
        debug_assert_eq!(value.rotate_left(rug_fuzz_3), 0b1011_0001);
        debug_assert_eq!(value.rotate_left(rug_fuzz_4), 0b0001_1011);
        debug_assert_eq!(value.rotate_left(rug_fuzz_5), 0b1011_0001);
        let _rug_ed_tests_llm_16_1834_rrrruuuugggg_test_rotate_left_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1835 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_1835_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b10110011;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 0b01110110;
        let value: u8 = rug_fuzz_0;
        let result = u8::rotate_right(value, rug_fuzz_1);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1835_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1836_llm_16_1836 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1836_llm_16_1836_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 0b0000_0001;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0b0000_0001;
        let rug_fuzz_3 = 7;
        let rug_fuzz_4 = 0b1000_0000;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0xFF;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 0b1010_1010;
        let rug_fuzz_9 = 0;
        debug_assert_eq!(
            < u8 as PrimInt > ::signed_shl(rug_fuzz_0, rug_fuzz_1), 0b0000_0010
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::signed_shl(rug_fuzz_2, rug_fuzz_3), 0b1000_0000
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::signed_shl(rug_fuzz_4, rug_fuzz_5), 0b0000_0000
        );
        debug_assert_eq!(< u8 as PrimInt > ::signed_shl(rug_fuzz_6, rug_fuzz_7), 0);
        debug_assert_eq!(
            < u8 as PrimInt > ::signed_shl(rug_fuzz_8, rug_fuzz_9), 0b1010_1010
        );
        let _rug_ed_tests_llm_16_1836_llm_16_1836_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1837_llm_16_1837 {
    use crate::*;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_1837_llm_16_1837_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 0b1111_1000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 0b1111_1100;
        let rug_fuzz_4 = 0b1111_1110;
        let initial: u8 = rug_fuzz_0;
        let shifted_1 = initial.signed_shr(rug_fuzz_1);
        let shifted_3 = initial.signed_shr(rug_fuzz_2);
        let expected_1: u8 = rug_fuzz_3;
        let expected_3: u8 = rug_fuzz_4;
        debug_assert_eq!(shifted_1, expected_1, "Shifting 1 bit to the right failed");
        debug_assert_eq!(shifted_3, expected_3, "Shifting 3 bits to the right failed");
        let _rug_ed_tests_llm_16_1837_llm_16_1837_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1838_llm_16_1838 {
    use crate::int::PrimInt;
    #[test]
    fn test_u8_swap_bytes() {
        let _rug_st_tests_llm_16_1838_llm_16_1838_rrrruuuugggg_test_u8_swap_bytes = 0;
        let rug_fuzz_0 = 0b1010_1010;
        let original: u8 = rug_fuzz_0;
        let swapped = original.swap_bytes();
        debug_assert_eq!(original, swapped, "Swapping bytes of a u8 should be a no-op.");
        let _rug_ed_tests_llm_16_1838_llm_16_1838_rrrruuuugggg_test_u8_swap_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1839_llm_16_1839 {
    use crate::PrimInt;
    #[test]
    fn test_u8_to_be() {
        let _rug_st_tests_llm_16_1839_llm_16_1839_rrrruuuugggg_test_u8_to_be = 0;
        let rug_fuzz_0 = 0x00;
        let rug_fuzz_1 = 0x7f;
        let rug_fuzz_2 = 0xff;
        let values: [u8; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        for &val in &values {
            debug_assert_eq!(val.to_be(), val);
        }
        let _rug_ed_tests_llm_16_1839_llm_16_1839_rrrruuuugggg_test_u8_to_be = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1840_llm_16_1840 {
    use crate::PrimInt;
    #[test]
    fn test_u8_to_le() {
        let _rug_st_tests_llm_16_1840_llm_16_1840_rrrruuuugggg_test_u8_to_le = 0;
        let rug_fuzz_0 = 0xAB;
        let num: u8 = rug_fuzz_0;
        debug_assert_eq!(num.to_le(), num);
        let _rug_ed_tests_llm_16_1840_llm_16_1840_rrrruuuugggg_test_u8_to_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1841_llm_16_1841 {
    use crate::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1841_llm_16_1841_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 2u8;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 4u8;
        let rug_fuzz_5 = 0b1110_0000u8;
        let rug_fuzz_6 = 0b0001_1111u8;
        let rug_fuzz_7 = 0b1111_1111u8;
        let rug_fuzz_8 = 0b1000_0000u8;
        let rug_fuzz_9 = 0b1101_1101u8;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 2);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 5);
        debug_assert_eq!(rug_fuzz_7.trailing_ones(), 8);
        debug_assert_eq!(rug_fuzz_8.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_9.trailing_ones(), 2);
        let _rug_ed_tests_llm_16_1841_llm_16_1841_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1842_llm_16_1842 {
    use crate::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1842_llm_16_1842_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 2u8;
        let rug_fuzz_3 = 3u8;
        let rug_fuzz_4 = 4u8;
        let rug_fuzz_5 = 8u8;
        let rug_fuzz_6 = 16u8;
        let rug_fuzz_7 = 32u8;
        let rug_fuzz_8 = 64u8;
        let rug_fuzz_9 = 128u8;
        let rug_fuzz_10 = 255u8;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_5.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_6.trailing_zeros(), 4);
        debug_assert_eq!(rug_fuzz_7.trailing_zeros(), 5);
        debug_assert_eq!(rug_fuzz_8.trailing_zeros(), 6);
        debug_assert_eq!(rug_fuzz_9.trailing_zeros(), 7);
        debug_assert_eq!(rug_fuzz_10.trailing_zeros(), 0);
        let _rug_ed_tests_llm_16_1842_llm_16_1842_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1843_llm_16_1843 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shl_basic() {
        let _rug_st_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_basic = 0;
        let rug_fuzz_0 = 0b0001_0010;
        let rug_fuzz_1 = 3;
        let value: u8 = rug_fuzz_0;
        let result = <u8 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b1_0010_000);
        let _rug_ed_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_basic = 0;
    }
    #[test]
    #[should_panic(expected = "shift operation overflowed")]
    fn unsigned_shl_overflow() {
        let _rug_st_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_overflow = 0;
        let rug_fuzz_0 = 0b1000_0000;
        let rug_fuzz_1 = 8;
        let value: u8 = rug_fuzz_0;
        let _result = <u8 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        let _rug_ed_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_overflow = 0;
    }
    #[test]
    fn unsigned_shl_zero() {
        let _rug_st_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_zero = 0;
        let rug_fuzz_0 = 0b0000_0000;
        let rug_fuzz_1 = 5;
        let value: u8 = rug_fuzz_0;
        let result = <u8 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 0);
        let _rug_ed_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_zero = 0;
    }
    #[test]
    fn unsigned_shl_no_shift() {
        let _rug_st_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_no_shift = 0;
        let rug_fuzz_0 = 0b0101_0101;
        let rug_fuzz_1 = 0;
        let value: u8 = rug_fuzz_0;
        let result = <u8 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_no_shift = 0;
    }
    #[test]
    fn unsigned_shl_full_shift() {
        let _rug_st_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_full_shift = 0;
        let rug_fuzz_0 = 0b0000_0001;
        let rug_fuzz_1 = 7;
        let value: u8 = rug_fuzz_0;
        let result = <u8 as PrimInt>::unsigned_shl(value, rug_fuzz_1);
        debug_assert_eq!(result, 0b1000_0000);
        let _rug_ed_tests_llm_16_1843_llm_16_1843_rrrruuuugggg_unsigned_shl_full_shift = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1844 {
    use super::*;
    use crate::*;
    #[test]
    fn unsigned_shr_works_for_u8() {
        let _rug_st_tests_llm_16_1844_rrrruuuugggg_unsigned_shr_works_for_u8 = 0;
        let rug_fuzz_0 = 0b11110000;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 0b00010000;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0b00000001;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 0b10000000;
        let rug_fuzz_7 = 7;
        let rug_fuzz_8 = 0b01010101;
        let rug_fuzz_9 = 1;
        debug_assert_eq!(
            < u8 as PrimInt > ::unsigned_shr(rug_fuzz_0, rug_fuzz_1), 0b00001111
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::unsigned_shr(rug_fuzz_2, rug_fuzz_3), 0b00000010
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::unsigned_shr(rug_fuzz_4, rug_fuzz_5), 0b00000000
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::unsigned_shr(rug_fuzz_6, rug_fuzz_7), 0b00000001
        );
        debug_assert_eq!(
            < u8 as PrimInt > ::unsigned_shr(rug_fuzz_8, rug_fuzz_9), 0b00101010
        );
        let _rug_ed_tests_llm_16_1844_rrrruuuugggg_unsigned_shr_works_for_u8 = 0;
    }
    #[test]
    #[should_panic]
    fn unsigned_shr_panics_when_shifting_u8_by_more_than_7() {
        let _rug_st_tests_llm_16_1844_rrrruuuugggg_unsigned_shr_panics_when_shifting_u8_by_more_than_7 = 0;
        let rug_fuzz_0 = 0xFF;
        let rug_fuzz_1 = 8;
        <u8 as PrimInt>::unsigned_shr(rug_fuzz_0, rug_fuzz_1);
        let _rug_ed_tests_llm_16_1844_rrrruuuugggg_unsigned_shr_panics_when_shifting_u8_by_more_than_7 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1931_llm_16_1931 {
    use crate::int::PrimInt;
    #[test]
    fn test_count_ones() {
        let _rug_st_tests_llm_16_1931_llm_16_1931_rrrruuuugggg_test_count_ones = 0;
        let rug_fuzz_0 = 0usize;
        let rug_fuzz_1 = 1usize;
        let rug_fuzz_2 = 0b1010usize;
        let rug_fuzz_3 = 0b1111usize;
        let rug_fuzz_4 = 0b10000000usize;
        debug_assert_eq!(rug_fuzz_0.count_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.count_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.count_ones(), 2);
        debug_assert_eq!(rug_fuzz_3.count_ones(), 4);
        debug_assert_eq!(rug_fuzz_4.count_ones(), 1);
        debug_assert_eq!(usize::MAX.count_ones(), usize::BITS as u32);
        let _rug_ed_tests_llm_16_1931_llm_16_1931_rrrruuuugggg_test_count_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1932 {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_zeros() {
        let _rug_st_tests_llm_16_1932_rrrruuuugggg_test_count_zeros = 0;
        let rug_fuzz_0 = 0usize;
        let rug_fuzz_1 = 1usize;
        let rug_fuzz_2 = 0b0001_0000usize;
        let rug_fuzz_3 = 0b1000_0000_0000_0000_0000_0000_0000_0000usize;
        debug_assert_eq!(rug_fuzz_0.count_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.count_zeros(), usize::BITS - 1);
        debug_assert_eq!(usize::MAX.count_zeros(), 0);
        debug_assert_eq!(rug_fuzz_2.count_zeros(), usize::BITS - 5);
        debug_assert_eq!(rug_fuzz_3.count_zeros(), usize::BITS - 32);
        let _rug_ed_tests_llm_16_1932_rrrruuuugggg_test_count_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1933_llm_16_1933 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_be() {
        if cfg!(target_endian = "big") {
            assert_eq!(< usize as PrimInt >::from_be(0x12345678), 0x12345678);
        } else {
            assert_eq!(
                < usize as PrimInt >::from_be(0x12345678), 0x12345678.swap_bytes()
            );
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1934_llm_16_1934 {
    use crate::int::PrimInt;
    #[test]
    fn test_from_le() {
        let values: [usize; 3] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98];
        if cfg!(target_endian = "little") {
            for &val in &values {
                assert_eq!(< usize as PrimInt >::from_le(val), val);
            }
        } else {
            for &val in &values {
                let expected = val.to_le();
                assert_eq!(< usize as PrimInt >::from_le(val), expected);
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1935_llm_16_1935 {
    use super::*;
    use crate::*;
    #[cfg(has_leading_trailing_ones)]
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_1935_llm_16_1935_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0usize;
        let rug_fuzz_1 = 1usize;
        let rug_fuzz_2 = 0xFFFF0000usize;
        let rug_fuzz_3 = 0x80000000usize;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 16);
        debug_assert_eq!(
            rug_fuzz_3.leading_ones(), if usize::BITS == 32 { 0 } else { 31 }
        );
        debug_assert_eq!(
            usize::MAX.leading_ones(), usize::BITS - usize::BITS.leading_zeros()
        );
        let _rug_ed_tests_llm_16_1935_llm_16_1935_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1936_llm_16_1936 {
    use crate::int::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_llm_16_1936_llm_16_1936_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0b0000_0000_0000_0000_usize;
        let rug_fuzz_1 = 0b0000_0000_0000_0001_usize;
        let rug_fuzz_2 = 0b0000_0001_0000_0000_usize;
        let rug_fuzz_3 = 0b1000_0000_0000_0000_usize;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 8;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        debug_assert_eq!(< usize as PrimInt > ::leading_zeros(rug_fuzz_0), 16);
        debug_assert_eq!(< usize as PrimInt > ::leading_zeros(rug_fuzz_1), 15);
        debug_assert_eq!(< usize as PrimInt > ::leading_zeros(rug_fuzz_2), 8);
        debug_assert_eq!(< usize as PrimInt > ::leading_zeros(rug_fuzz_3), 0);
        debug_assert_eq!(< usize as PrimInt > ::leading_zeros(usize::MAX), 0);
        debug_assert_eq!(
            < usize as PrimInt > ::leading_zeros(usize::MAX - rug_fuzz_4), 0
        );
        let usize_bits = std::mem::size_of::<usize>() * rug_fuzz_5;
        debug_assert_eq!(
            < usize as PrimInt > ::leading_zeros(rug_fuzz_6), usize_bits as u32 - 1
        );
        debug_assert_eq!(
            < usize as PrimInt > ::leading_zeros(rug_fuzz_7), usize_bits as u32
        );
        let _rug_ed_tests_llm_16_1936_llm_16_1936_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1937_llm_16_1937 {
    use crate::int::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_1937_llm_16_1937_rrrruuuugggg_test_pow = 0;
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
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 10;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 10;
        let rug_fuzz_17 = 3;
        let rug_fuzz_18 = 10;
        let rug_fuzz_19 = 4;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 2;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 100;
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_0, rug_fuzz_1), 1);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_2, rug_fuzz_3), 2);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_4, rug_fuzz_5), 4);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_6, rug_fuzz_7), 8);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_8, rug_fuzz_9), 16);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_10, rug_fuzz_11), 1);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_12, rug_fuzz_13), 10);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_14, rug_fuzz_15), 100);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_16, rug_fuzz_17), 1000);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_18, rug_fuzz_19), 10000);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_20, rug_fuzz_21), 1);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_22, rug_fuzz_23), 0);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_24, rug_fuzz_25), 0);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_26, rug_fuzz_27), 1);
        debug_assert_eq!(< usize as PrimInt > ::pow(rug_fuzz_28, rug_fuzz_29), 1);
        let _rug_ed_tests_llm_16_1937_llm_16_1937_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1938_llm_16_1938 {
    use crate::int::PrimInt;
    #[cfg(has_reverse_bits)]
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_1938_llm_16_1938_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0usize;
        let rug_fuzz_1 = 1usize;
        let rug_fuzz_2 = 1usize;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0b1001_0010_1101_0101_0011_1100_0101_0011usize;
        let rug_fuzz_5 = 0b1100_1010_0011_1100_1010_1101_0100_1001usize;
        debug_assert_eq!(rug_fuzz_0.reverse_bits(), 0usize);
        debug_assert_eq!(usize::MAX.reverse_bits(), usize::MAX);
        debug_assert_eq!(rug_fuzz_1.reverse_bits(), 1usize << (usize::BITS - 1));
        debug_assert_eq!(
            (rug_fuzz_2 << (usize::BITS - rug_fuzz_3)).reverse_bits(), 1usize
        );
        let num = rug_fuzz_4;
        let expected = rug_fuzz_5;
        debug_assert_eq!(num.reverse_bits(), expected);
        let _rug_ed_tests_llm_16_1938_llm_16_1938_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1939_llm_16_1939 {
    use super::*;
    use crate::*;
    use crate::*;
    use std::convert::TryInto;
    #[test]
    fn rotate_left_works() {
        let _rug_st_tests_llm_16_1939_llm_16_1939_rrrruuuugggg_rotate_left_works = 0;
        let rug_fuzz_0 = 8;
        let rug_fuzz_1 = 0usize;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 1usize;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1usize;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1usize;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1usize;
        let rug_fuzz_11 = 1usize;
        let rug_fuzz_12 = 1;
        let width: u32 = (std::mem::size_of::<usize>() * rug_fuzz_0).try_into().unwrap();
        let result = rug_fuzz_1.rotate_left(rug_fuzz_2);
        debug_assert_eq!(result, 0);
        let result = rug_fuzz_3.rotate_left(rug_fuzz_4);
        debug_assert_eq!(result, 1);
        let result = rug_fuzz_5.rotate_left(rug_fuzz_6);
        debug_assert_eq!(result, 1 << 1);
        let result = usize::MAX.rotate_left(rug_fuzz_7);
        debug_assert_eq!(result, (usize::MAX << 1) | 1);
        let result = rug_fuzz_8.rotate_left(width - rug_fuzz_9);
        debug_assert_eq!(result, 1);
        let result = rug_fuzz_10.rotate_left(width);
        debug_assert_eq!(result, 1);
        let result = rug_fuzz_11.rotate_left(width + rug_fuzz_12);
        debug_assert_eq!(result, 1 << 1);
        let _rug_ed_tests_llm_16_1939_llm_16_1939_rrrruuuugggg_rotate_left_works = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1940_llm_16_1940 {
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_llm_16_1940_llm_16_1940_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b0001_0001;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0b0001_0001;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 0b0001_0001;
        let rug_fuzz_5 = 8;
        let rug_fuzz_6 = 0b0001_0001;
        let rug_fuzz_7 = 0b0001_0001;
        let rug_fuzz_8 = 4;
        debug_assert_eq!(
            < usize as PrimInt > ::rotate_right(rug_fuzz_0, rug_fuzz_1), 0b0001_0001
        );
        debug_assert_eq!(
            < usize as PrimInt > ::rotate_right(rug_fuzz_2, rug_fuzz_3), 0b1000_1000
        );
        debug_assert_eq!(
            < usize as PrimInt > ::rotate_right(rug_fuzz_4, rug_fuzz_5), 0b0001_0001
        );
        let usize_bits = usize::count_ones(usize::MAX);
        debug_assert_eq!(
            < usize as PrimInt > ::rotate_right(rug_fuzz_6, usize_bits), 0b0001_0001
        );
        debug_assert_eq!(
            < usize as PrimInt > ::rotate_right(rug_fuzz_7, usize_bits + rug_fuzz_8),
            0b1000_1000
        );
        let _rug_ed_tests_llm_16_1940_llm_16_1940_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1941_llm_16_1941 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_llm_16_1941_llm_16_1941_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let value: usize = rug_fuzz_0;
        let shifted = <usize as PrimInt>::signed_shl(value, rug_fuzz_1);
        debug_assert_eq!(shifted, 4);
        let _rug_ed_tests_llm_16_1941_llm_16_1941_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1942_llm_16_1942 {
    use crate::int::PrimInt;
    #[test]
    fn test_signed_shr() {
        let _rug_st_tests_llm_16_1942_llm_16_1942_rrrruuuugggg_test_signed_shr = 0;
        let rug_fuzz_0 = 0xFFFF_FFFF_FFFF_FFFF;
        let rug_fuzz_1 = 4;
        let value: usize = rug_fuzz_0;
        let shifted = <usize as PrimInt>::signed_shr(value, rug_fuzz_1);
        debug_assert_eq!(shifted, 0x0FFF_FFFF_FFFF_FFFF);
        let _rug_ed_tests_llm_16_1942_llm_16_1942_rrrruuuugggg_test_signed_shr = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1943_llm_16_1943 {
    use super::*;
    use crate::*;
    #[test]
    fn test_swap_bytes_usize() {
        let value: usize = 0x12345678;
        let swapped = <usize as PrimInt>::swap_bytes(value);
        if cfg!(target_endian = "little") {
            assert_eq!(swapped, 0x78563412);
        } else {
            assert_eq!(swapped, 0x12345678);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1944_llm_16_1944 {
    use crate::PrimInt;
    #[test]
    fn test_to_be() {
        let num: usize = 0x12345678;
        let big_endian_num = num.to_be();
        if cfg!(target_endian = "big") {
            assert_eq!(num, big_endian_num);
        } else if cfg!(target_endian = "little") {
            let swapped_bytes = num.swap_bytes();
            assert_eq!(swapped_bytes, big_endian_num);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1945_llm_16_1945 {
    use crate::int::PrimInt;
    #[test]
    fn test_to_le() {
        let _rug_st_tests_llm_16_1945_llm_16_1945_rrrruuuugggg_test_to_le = 0;
        let rug_fuzz_0 = 0x12345678;
        let num: usize = rug_fuzz_0;
        #[cfg(target_endian = "little")]
        {
            debug_assert_eq!(num.to_le(), num);
        }
        #[cfg(target_endian = "big")]
        {
            let expected: usize = num.swap_bytes();
            debug_assert_eq!(num.to_le(), expected);
        }
        let _rug_ed_tests_llm_16_1945_llm_16_1945_rrrruuuugggg_test_to_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1946 {
    use super::*;
    use crate::*;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_1946_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0b0000_0000usize;
        let rug_fuzz_1 = 0b0000_0001usize;
        let rug_fuzz_2 = 0b0001_0000usize;
        let rug_fuzz_3 = 0b0001_0001usize;
        let rug_fuzz_4 = 0b0011_1111usize;
        let rug_fuzz_5 = 0usize;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 6);
        debug_assert_eq!(usize::MAX.trailing_ones(), (usize::BITS / 2) as u32);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 0);
        let _rug_ed_tests_llm_16_1946_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1947_llm_16_1947 {
    use crate::PrimInt;
    #[test]
    fn test_trailing_zeros() {
        let _rug_st_tests_llm_16_1947_llm_16_1947_rrrruuuugggg_test_trailing_zeros = 0;
        let rug_fuzz_0 = 1usize;
        let rug_fuzz_1 = 2usize;
        let rug_fuzz_2 = 0b100usize;
        let rug_fuzz_3 = 0b1000usize;
        let rug_fuzz_4 = 0usize;
        debug_assert_eq!(rug_fuzz_0.trailing_zeros(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_zeros(), 1);
        debug_assert_eq!(rug_fuzz_2.trailing_zeros(), 2);
        debug_assert_eq!(rug_fuzz_3.trailing_zeros(), 3);
        debug_assert_eq!(rug_fuzz_4.trailing_zeros(), usize::BITS as u32);
        let _rug_ed_tests_llm_16_1947_llm_16_1947_rrrruuuugggg_test_trailing_zeros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1948_llm_16_1948 {
    use crate::int::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_llm_16_1948_llm_16_1948_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 1;
        let value: usize = rug_fuzz_0;
        let shift: u32 = rug_fuzz_1;
        let result = PrimInt::unsigned_shl(value, shift);
        let expected = value << shift;
        debug_assert_eq!(
            result, expected, "Shifting {} by {} should result in {}", value, shift,
            expected
        );
        let _rug_ed_tests_llm_16_1948_llm_16_1948_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1949_llm_16_1949 {
    use crate::int::PrimInt;
    #[test]
    fn unsigned_shr_test() {
        let _rug_st_tests_llm_16_1949_llm_16_1949_rrrruuuugggg_unsigned_shr_test = 0;
        let rug_fuzz_0 = 0b1000_0000_0000_0000;
        let rug_fuzz_1 = 4;
        let value: usize = rug_fuzz_0;
        let shift_amount: u32 = rug_fuzz_1;
        let result = <usize as PrimInt>::unsigned_shr(value, shift_amount);
        debug_assert_eq!(result, 0b0000_1000_0000_0000);
        let _rug_ed_tests_llm_16_1949_llm_16_1949_rrrruuuugggg_unsigned_shr_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2045 {
    use super::*;
    use crate::*;
    #[test]
    fn test_leading_ones() {
        let _rug_st_tests_llm_16_2045_rrrruuuugggg_test_leading_ones = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 0u16;
        let rug_fuzz_2 = 0u32;
        let rug_fuzz_3 = 0u64;
        let rug_fuzz_4 = 0u128;
        let rug_fuzz_5 = 0xF0u8;
        let rug_fuzz_6 = 0xF00Du16;
        let rug_fuzz_7 = 0xF000_000Du32;
        let rug_fuzz_8 = 0xF000_0000_0000_000Du64;
        let rug_fuzz_9 = 0xF000_0000_0000_0000_0000_0000_0000_000Du128;
        debug_assert_eq!(rug_fuzz_0.leading_ones(), 8);
        debug_assert_eq!(rug_fuzz_1.leading_ones(), 16);
        debug_assert_eq!(rug_fuzz_2.leading_ones(), 32);
        debug_assert_eq!(rug_fuzz_3.leading_ones(), 64);
        debug_assert_eq!(rug_fuzz_4.leading_ones(), 128);
        debug_assert_eq!(rug_fuzz_5.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_6.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_7.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_8.leading_ones(), 4);
        debug_assert_eq!(rug_fuzz_9.leading_ones(), 4);
        let _rug_ed_tests_llm_16_2045_rrrruuuugggg_test_leading_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2046 {
    use super::*;
    use crate::*;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_llm_16_2046_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0x12345678u32;
        let rug_fuzz_1 = 0u32;
        let rug_fuzz_2 = 0xFFFFFFFFu32;
        let rug_fuzz_3 = 0x80000000u32;
        let rug_fuzz_4 = 0x00000001u32;
        let rug_fuzz_5 = 0x55555555u32;
        let rug_fuzz_6 = 0xAAAAAAAAu32;
        let rug_fuzz_7 = 0x0F0F0F0Fu32;
        let rug_fuzz_8 = 0xF0F0F0F0u32;
        debug_assert_eq!(rug_fuzz_0.reverse_bits(), 0x1e6a2c48u32);
        debug_assert_eq!(rug_fuzz_1.reverse_bits(), 0);
        debug_assert_eq!(rug_fuzz_2.reverse_bits(), 0xFFFFFFFFu32);
        debug_assert_eq!(rug_fuzz_3.reverse_bits(), 1u32);
        debug_assert_eq!(rug_fuzz_4.reverse_bits(), 0x80000000u32);
        debug_assert_eq!(rug_fuzz_5.reverse_bits(), 0xAAAAAAAAu32);
        debug_assert_eq!(rug_fuzz_6.reverse_bits(), 0x55555555u32);
        debug_assert_eq!(rug_fuzz_7.reverse_bits(), 0xF0F0F0F0u32);
        debug_assert_eq!(rug_fuzz_8.reverse_bits(), 0x0F0F0F0Fu32);
        let _rug_ed_tests_llm_16_2046_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2047 {
    use crate::PrimInt;
    #[test]
    fn test_trailing_ones() {
        let _rug_st_tests_llm_16_2047_rrrruuuugggg_test_trailing_ones = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 0u16;
        let rug_fuzz_2 = 0u32;
        let rug_fuzz_3 = 0u64;
        let rug_fuzz_4 = 0u128;
        let rug_fuzz_5 = 1u8;
        let rug_fuzz_6 = 1u16;
        let rug_fuzz_7 = 1u32;
        let rug_fuzz_8 = 1u64;
        let rug_fuzz_9 = 1u128;
        let rug_fuzz_10 = 0b1111_0000u8;
        let rug_fuzz_11 = 0b0011_1111u16;
        let rug_fuzz_12 = 0b0000_1111u32;
        let rug_fuzz_13 = 0xFu64;
        let rug_fuzz_14 = 0b1111_1111u128;
        let rug_fuzz_15 = 0u32;
        let rug_fuzz_16 = 0u64;
        let rug_fuzz_17 = 0u128;
        debug_assert_eq!(rug_fuzz_0.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_1.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_2.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_3.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_4.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_5.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_6.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_7.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_8.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_9.trailing_ones(), 1);
        debug_assert_eq!(rug_fuzz_10.trailing_ones(), 0);
        debug_assert_eq!(rug_fuzz_11.trailing_ones(), 6);
        debug_assert_eq!(rug_fuzz_12.trailing_ones(), 4);
        debug_assert_eq!(rug_fuzz_13.trailing_ones(), 4);
        debug_assert_eq!(rug_fuzz_14.trailing_ones(), 8);
        debug_assert_eq!((! rug_fuzz_15).trailing_ones(), 32);
        debug_assert_eq!((! rug_fuzz_16).trailing_ones(), 64);
        debug_assert_eq!((! rug_fuzz_17).trailing_ones(), 128);
        let _rug_ed_tests_llm_16_2047_rrrruuuugggg_test_trailing_ones = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2048_llm_16_2048 {
    use super::*;
    use crate::*;
    use crate::int::PrimInt;
    #[test]
    fn one_per_byte_i8() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i8 = 0;
        let res: i8 = one_per_byte::<i8>();
        debug_assert_eq!(res, 0x01);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i8 = 0;
    }
    #[test]
    fn one_per_byte_u8() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u8 = 0;
        let res: u8 = one_per_byte::<u8>();
        debug_assert_eq!(res, 0x01);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u8 = 0;
    }
    #[test]
    fn one_per_byte_i16() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i16 = 0;
        let res: i16 = one_per_byte::<i16>();
        debug_assert_eq!(res, 0x0101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i16 = 0;
    }
    #[test]
    fn one_per_byte_u16() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u16 = 0;
        let res: u16 = one_per_byte::<u16>();
        debug_assert_eq!(res, 0x0101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u16 = 0;
    }
    #[test]
    fn one_per_byte_i32() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i32 = 0;
        let res: i32 = one_per_byte::<i32>();
        debug_assert_eq!(res, 0x01010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i32 = 0;
    }
    #[test]
    fn one_per_byte_u32() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u32 = 0;
        let res: u32 = one_per_byte::<u32>();
        debug_assert_eq!(res, 0x01010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u32 = 0;
    }
    #[test]
    fn one_per_byte_i64() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i64 = 0;
        let res: i64 = one_per_byte::<i64>();
        debug_assert_eq!(res, 0x0101010101010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i64 = 0;
    }
    #[test]
    fn one_per_byte_u64() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u64 = 0;
        let res: u64 = one_per_byte::<u64>();
        debug_assert_eq!(res, 0x0101010101010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u64 = 0;
    }
    #[test]
    fn one_per_byte_i128() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i128 = 0;
        let res: i128 = one_per_byte::<i128>();
        debug_assert_eq!(res, 0x01010101010101010101010101010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_i128 = 0;
    }
    #[test]
    fn one_per_byte_u128() {
        let _rug_st_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u128 = 0;
        let res: u128 = one_per_byte::<u128>();
        debug_assert_eq!(res, 0x01010101010101010101010101010101);
        let _rug_ed_tests_llm_16_2048_llm_16_2048_rrrruuuugggg_one_per_byte_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2049_llm_16_2049 {
    use super::*;
    use crate::*;
    #[test]
    fn test_reverse_bits_fallback() {
        let _rug_st_tests_llm_16_2049_llm_16_2049_rrrruuuugggg_test_reverse_bits_fallback = 0;
        let rug_fuzz_0 = 0b1011_0001;
        let rug_fuzz_1 = 0b1000_1101;
        let rug_fuzz_2 = 0b1011_0001_1110_0010;
        let rug_fuzz_3 = 0b0100_0111_1000_1101;
        let rug_fuzz_4 = 0b1011_0001_1110_0010_1010_1001_0101_1100;
        let rug_fuzz_5 = 0b0011_1010_1001_0101_0100_0111_1000_1101;
        let rug_fuzz_6 = 0b1011_0001_1110_0010_1010_1001_0101_1100_1011_0001_1110_0010_1010_1001_0101_1100;
        let rug_fuzz_7 = 0b0011_1010_1001_0101_0100_0111_1000_1101_0011_1010_1001_0101_0100_0111_1000_1101;
        let rug_fuzz_8 = 0u8;
        let rug_fuzz_9 = 0u16;
        let rug_fuzz_10 = 0u32;
        let rug_fuzz_11 = 0u64;
        let original_8: u8 = rug_fuzz_0;
        let reversed_8: u8 = rug_fuzz_1;
        debug_assert_eq!(reverse_bits_fallback(original_8), reversed_8);
        let original_16: u16 = rug_fuzz_2;
        let reversed_16: u16 = rug_fuzz_3;
        debug_assert_eq!(reverse_bits_fallback(original_16), reversed_16);
        let original_32: u32 = rug_fuzz_4;
        let reversed_32: u32 = rug_fuzz_5;
        debug_assert_eq!(reverse_bits_fallback(original_32), reversed_32);
        let original_64: u64 = rug_fuzz_6;
        let reversed_64: u64 = rug_fuzz_7;
        debug_assert_eq!(reverse_bits_fallback(original_64), reversed_64);
        debug_assert_eq!(reverse_bits_fallback(rug_fuzz_8), 0u8);
        debug_assert_eq!(reverse_bits_fallback(rug_fuzz_9), 0u16);
        debug_assert_eq!(reverse_bits_fallback(rug_fuzz_10), 0u32);
        debug_assert_eq!(reverse_bits_fallback(rug_fuzz_11), 0u64);
        let _rug_ed_tests_llm_16_2049_llm_16_2049_rrrruuuugggg_test_reverse_bits_fallback = 0;
    }
}
#[cfg(test)]
mod tests_rug_42 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_42_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 2;
        let mut p0: u16 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< u16 as PrimInt > ::pow(p0, p1), 25);
        let _rug_ed_tests_rug_42_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_44_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0b0000_0111_i8;
        let p0: i8 = rug_fuzz_0;
        debug_assert_eq!(< i8 as PrimInt > ::leading_zeros(p0), 5);
        let _rug_ed_tests_rug_44_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_signed_shl() {
        let _rug_st_tests_rug_46_rrrruuuugggg_test_signed_shl = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 1;
        let mut p0: i8 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i8 as PrimInt > ::signed_shl(p0, p1), 10);
        let _rug_ed_tests_rug_46_rrrruuuugggg_test_signed_shl = 0;
    }
}
#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_47_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 32;
        let rug_fuzz_1 = 3;
        let mut p0: i8 = -rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i8 as PrimInt > ::signed_shr(p0, p1), - 4);
        let _rug_ed_tests_rug_47_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_rug_48_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b00011011;
        let mut p0: i8 = rug_fuzz_0;
        debug_assert_eq!(< i8 as PrimInt > ::reverse_bits(p0), p0.reverse_bits());
        let _rug_ed_tests_rug_48_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_rug_49 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_pow() {
        let _rug_st_tests_rug_49_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let p0: i8 = rug_fuzz_0;
        let p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i8 as PrimInt > ::pow(p0, p1), 81);
        let _rug_ed_tests_rug_49_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_leading_zeros() {
        let _rug_st_tests_rug_50_rrrruuuugggg_test_leading_zeros = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000;
        let p0: i16 = rug_fuzz_0;
        debug_assert_eq!(< i16 as PrimInt > ::leading_zeros(p0), 3);
        let _rug_ed_tests_rug_50_rrrruuuugggg_test_leading_zeros = 0;
    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_rug_51_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_0010_0100_1000;
        let rug_fuzz_1 = 8;
        let mut p0: i16 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let result = <i16 as PrimInt>::rotate_left(p0, p1);
        debug_assert_eq!(result, 0b0010_0100_1000_0001);
        let _rug_ed_tests_rug_51_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_rug_52 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_rug_52_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b0011_1100_1010_0101;
        let rug_fuzz_1 = 8;
        let mut p0: i16 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(
            < i16 as PrimInt > ::rotate_right(p0, p1), 0b0101_0011_1100_1010
        );
        let _rug_ed_tests_rug_52_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_rug_53_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 4;
        let p0: i16 = rug_fuzz_0;
        let p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i16 as PrimInt > ::unsigned_shl(p0, p1), 48);
        let _rug_ed_tests_rug_53_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_rug_54 {
    use crate::int::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_54_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12345;
        let rug_fuzz_1 = 3;
        let mut p0: i16 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i16 as PrimInt > ::unsigned_shr(p0, p1), 1543);
        let _rug_ed_tests_rug_54_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_55 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_rug_55_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b0000_0001_0101_1010;
        let mut p0: i16 = rug_fuzz_0;
        let result = p0.reverse_bits();
        debug_assert_eq!(result, 0b0101_1010_1000_0000);
        let _rug_ed_tests_rug_55_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_left() {
        let _rug_st_tests_rug_56_rrrruuuugggg_test_rotate_left = 0;
        let rug_fuzz_0 = 0b0001_0000_0000_0000_0000_0000_0000_1011;
        let rug_fuzz_1 = 5;
        let p0: i32 = rug_fuzz_0;
        let p1: u32 = rug_fuzz_1;
        debug_assert_eq!(
            < i32 as PrimInt > ::rotate_left(p0, p1),
            0b0000_0000_0001_0110_0001_0000_0000_0000
        );
        let _rug_ed_tests_rug_56_rrrruuuugggg_test_rotate_left = 0;
    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::int::PrimInt;
    #[test]
    fn test_rotate_right() {
        let _rug_st_tests_rug_57_rrrruuuugggg_test_rotate_right = 0;
        let rug_fuzz_0 = 0b10110011;
        let rug_fuzz_1 = 5;
        let mut p0: i32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i32 as PrimInt > ::rotate_right(p0, p1), 0b11101101);
        let _rug_ed_tests_rug_57_rrrruuuugggg_test_rotate_right = 0;
    }
}
#[cfg(test)]
mod tests_rug_58 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_58_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 1;
        let mut p0: i32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        <i32 as PrimInt>::signed_shl(p0, p1);
        let _rug_ed_tests_rug_58_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_59 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_unsigned_shl() {
        let _rug_st_tests_rug_59_rrrruuuugggg_test_unsigned_shl = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 2;
        let mut p0: i32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        debug_assert_eq!(< i32 as PrimInt > ::unsigned_shl(p0, p1), 16);
        let _rug_ed_tests_rug_59_rrrruuuugggg_test_unsigned_shl = 0;
    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_60_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x12345678;
        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as PrimInt > ::swap_bytes(p0), 0x78563412);
        let _rug_ed_tests_rug_60_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_61_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0b00000000000000011011010110010111;
        let mut p0: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as PrimInt > ::reverse_bits(p0), p0.reverse_bits());
        let _rug_ed_tests_rug_61_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_swap_bytes() {
        let _rug_st_tests_rug_62_rrrruuuugggg_test_swap_bytes = 0;
        let rug_fuzz_0 = 0x1234567812345678;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as PrimInt > ::swap_bytes(p0), p0.swap_bytes());
        let _rug_ed_tests_rug_62_rrrruuuugggg_test_swap_bytes = 0;
    }
}
#[cfg(test)]
mod tests_rug_63 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_reverse_bits() {
        let _rug_st_tests_rug_63_rrrruuuugggg_test_reverse_bits = 0;
        let rug_fuzz_0 = 0b0000001000000000000000000000000000000000000000000000000000000010;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(
            < i64 as PrimInt > ::reverse_bits(p0), 0b01000000100000000000000000000000000
        );
        let _rug_ed_tests_rug_63_rrrruuuugggg_test_reverse_bits = 0;
    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use crate::PrimInt;
    #[test]
    fn test_from_be() {
        let _rug_st_tests_rug_64_rrrruuuugggg_test_from_be = 0;
        let rug_fuzz_0 = 0x1234_5678_isize;
        let p0: isize = rug_fuzz_0.to_be();
        debug_assert_eq!(< isize as PrimInt > ::from_be(p0), 0x1234_5678_isize);
        let _rug_ed_tests_rug_64_rrrruuuugggg_test_from_be = 0;
    }
}
