#[cfg(feature = "simd")]
use crate::simd::simdint;
use crate::simd::simdty::{u32x4, u64x4};
use core::ops::{Add, BitXor, Shl, Shr};
macro_rules! impl_ops {
    ($vec:ident) => {
        impl Add for $vec { type Output = Self; #[cfg(feature = "simd")]
        #[inline(always)] fn add(self, rhs : Self) -> Self::Output { unsafe {
        simdint::simd_add(self, rhs) } } #[cfg(not(feature = "simd"))] #[inline(always)]
        fn add(self, rhs : Self) -> Self::Output { $vec ::new(self.0.wrapping_add(rhs.0),
        self.1.wrapping_add(rhs.1), self.2.wrapping_add(rhs.2), self.3.wrapping_add(rhs
        .3),) } } impl BitXor for $vec { type Output = Self; #[cfg(feature = "simd")]
        #[inline(always)] fn bitxor(self, rhs : Self) -> Self::Output { unsafe {
        simdint::simd_xor(self, rhs) } } #[cfg(not(feature = "simd"))] #[inline(always)]
        fn bitxor(self, rhs : Self) -> Self::Output { $vec ::new(self.0 ^ rhs.0, self.1 ^
        rhs.1, self.2 ^ rhs.2, self.3 ^ rhs.3,) } } impl Shl <$vec > for $vec { type
        Output = Self; #[cfg(feature = "simd")] #[inline(always)] fn shl(self, rhs :
        Self) -> Self::Output { unsafe { simdint::simd_shl(self, rhs) } }
        #[cfg(not(feature = "simd"))] #[inline(always)] fn shl(self, rhs : Self) ->
        Self::Output { $vec ::new(self.0 << rhs.0, self.1 << rhs.1, self.2 << rhs.2, self
        .3 << rhs.3,) } } impl Shr <$vec > for $vec { type Output = Self; #[cfg(feature =
        "simd")] #[inline(always)] fn shr(self, rhs : Self) -> Self::Output { unsafe {
        simdint::simd_shr(self, rhs) } } #[cfg(not(feature = "simd"))] #[inline(always)]
        fn shr(self, rhs : Self) -> Self::Output { $vec ::new(self.0 >> rhs.0, self.1 >>
        rhs.1, self.2 >> rhs.2, self.3 >> rhs.3,) } }
    };
}
impl_ops!(u32x4);
impl_ops!(u64x4);
#[cfg(test)]
mod tests_llm_16_64_llm_16_64 {
    use super::*;
    use crate::*;
    fn assert_simd4_eq<T: core::cmp::Eq + core::fmt::Debug>(
        a: simd::simdty::Simd4<T>,
        b: simd::simdty::Simd4<T>,
    ) {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_assert_simd4_eq = 0;
        debug_assert_eq!(a.0, b.0);
        debug_assert_eq!(a.1, b.1);
        debug_assert_eq!(a.2, b.2);
        debug_assert_eq!(a.3, b.3);
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_assert_simd4_eq = 0;
    }
    #[test]
    fn test_shl() {
        let _rug_st_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_shl = 0;
        let rug_fuzz_0 = 1u32;
        let rug_fuzz_1 = 2u32;
        let rug_fuzz_2 = 4u32;
        let rug_fuzz_3 = 8u32;
        let rug_fuzz_4 = 1u32;
        let rug_fuzz_5 = 1u32;
        let rug_fuzz_6 = 2u32;
        let rug_fuzz_7 = 3u32;
        let rug_fuzz_8 = 2u32;
        let rug_fuzz_9 = 4u32;
        let rug_fuzz_10 = 16u32;
        let rug_fuzz_11 = 64u32;
        let a = simd::simdty::Simd4::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let b = simd::simdty::Simd4::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let result = a.shl(b);
        assert_simd4_eq(
            result,
            simd::simdty::Simd4::new(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
        );
        let _rug_ed_tests_llm_16_64_llm_16_64_rrrruuuugggg_test_shl = 0;
    }
}
