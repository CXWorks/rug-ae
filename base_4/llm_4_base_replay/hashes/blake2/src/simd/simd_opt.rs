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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24)) = <(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
