#[allow(unused_macros)]
#[cfg(feature = "simd")]
macro_rules! transmute_shuffle {
    ($tmp:ident, $shuffle:ident, $vec:expr, $idx_n:expr, $idx:expr) => {
        unsafe { use crate ::simd::simdint::$shuffle; use crate ::simd::simdty::$tmp; use
        core::mem::transmute; const IDX : [u32; $idx_n] = $idx; let tmp_i : $tmp =
        transmute($vec); let tmp_o : $tmp = $shuffle (tmp_i, tmp_i, IDX);
        transmute(tmp_o) }
    };
}
#[cfg(feature = "simd")]
pub mod u32x4;
#[cfg(feature = "simd")]
pub mod u64x4;
#[cfg(not(feature = "simd"))]
macro_rules! simd_opt {
    ($vec:ident) => {
        pub mod $vec { use crate ::simd::simdty::$vec; #[inline(always)] pub fn
        rotate_right_const(vec : $vec, n : u32) -> $vec { $vec ::new(vec.0
        .rotate_right(n), vec.1.rotate_right(n), vec.2.rotate_right(n), vec.3
        .rotate_right(n),) } }
    };
}
#[cfg(not(feature = "simd"))]
simd_opt!(u32x4);
#[cfg(not(feature = "simd"))]
simd_opt!(u64x4);
#[cfg(test)]
mod tests_llm_16_59_llm_16_59 {
    use crate::simd::simd_opt::u64x4::rotate_right_const;
    use crate::simd::simdty::Simd4;
    #[test]
    fn test_rotate_right_func() {
        let _rug_st_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_rotate_right_func = 0;
        let rug_fuzz_0 = 0x1234567890ABCDEF;
        let rug_fuzz_1 = 0x1234567890ABCDEF;
        let rug_fuzz_2 = 0x1234567890ABCDEF;
        let rug_fuzz_3 = 0x1234567890ABCDEF;
        let rug_fuzz_4 = 0xEF90ABCD56781234;
        let rug_fuzz_5 = 0xEF90ABCD56781234;
        let rug_fuzz_6 = 0xEF90ABCD56781234;
        let rug_fuzz_7 = 0xEF90ABCD56781234;
        let rug_fuzz_8 = 0x7890ABCDEF123456;
        let rug_fuzz_9 = 0x7890ABCDEF123456;
        let rug_fuzz_10 = 0x7890ABCDEF123456;
        let rug_fuzz_11 = 0x7890ABCDEF123456;
        let rug_fuzz_12 = 0xCDEF567890AB1234;
        let rug_fuzz_13 = 0xCDEF567890AB1234;
        let rug_fuzz_14 = 0xCDEF567890AB1234;
        let rug_fuzz_15 = 0xCDEF567890AB1234;
        let rug_fuzz_16 = 16;
        let rug_fuzz_17 = 16;
        let rug_fuzz_18 = 32;
        let rug_fuzz_19 = 32;
        let rug_fuzz_20 = 48;
        let rug_fuzz_21 = 48;
        let rug_fuzz_22 = 64;
        let rug_fuzz_23 = 64;
        let rug_fuzz_24 = 0;
        let vec = Simd4::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let rotated_by_16 = Simd4::new(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let rotated_by_32 = Simd4::new(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let rotated_by_48 = Simd4::new(
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        );
        let rotated_by_64 = vec;
        debug_assert!(
            rotate_right_const(vec, rug_fuzz_16).0.rotate_right(rug_fuzz_17) ==
            rotated_by_16.0
        );
        debug_assert!(
            rotate_right_const(vec, rug_fuzz_18).0.rotate_right(rug_fuzz_19) ==
            rotated_by_32.0
        );
        debug_assert!(
            rotate_right_const(vec, rug_fuzz_20).0.rotate_right(rug_fuzz_21) ==
            rotated_by_48.0
        );
        debug_assert!(
            rotate_right_const(vec, rug_fuzz_22).0.rotate_right(rug_fuzz_23) ==
            rotated_by_64.0
        );
        debug_assert!(rotate_right_const(vec, rug_fuzz_24).0 == vec.0);
        let _rug_ed_tests_llm_16_59_llm_16_59_rrrruuuugggg_test_rotate_right_func = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use crate::simd::simd_opt::u32x4::rotate_right_const;
    use crate::simd::simdty::Simd4;
    #[test]
    fn test_rotate_right_const() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_rotate_right_const = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 16_u32;
        let mut p0 = Simd4::<u32>::new(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        let mut p1 = rug_fuzz_4;
        rotate_right_const(p0, p1);
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_rotate_right_const = 0;
    }
}
