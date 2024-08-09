//! SHA-256 `x86`/`x86_64` backend
#![allow(clippy::many_single_char_names)]
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
unsafe fn schedule(v0: __m128i, v1: __m128i, v2: __m128i, v3: __m128i) -> __m128i {
    let t1 = _mm_sha256msg1_epu32(v0, v1);
    let t2 = _mm_alignr_epi8(v3, v2, 4);
    let t3 = _mm_add_epi32(t1, t2);
    _mm_sha256msg2_epu32(t3, v3)
}
macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => {
        { let k = crate ::consts::K32X4[$i]; let kv = _mm_set_epi32(k[0] as i32, k[1] as
        i32, k[2] as i32, k[3] as i32); let t1 = _mm_add_epi32($rest, kv); $cdgh =
        _mm_sha256rnds2_epu32($cdgh, $abef, t1); let t2 = _mm_shuffle_epi32(t1, 0x0E);
        $abef = _mm_sha256rnds2_epu32($abef, $cdgh, t2); }
    };
}
macro_rules! schedule_rounds4 {
    (
        $abef:ident, $cdgh:ident, $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i:expr
    ) => {
        { $w4 = schedule($w0, $w1, $w2, $w3); rounds4!($abef, $cdgh, $w4, $i); }
    };
}
#[allow(clippy::cast_ptr_alignment)]
#[target_feature(enable = "sha,sse2,ssse3,sse4.1")]
unsafe fn digest_blocks(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    #[allow(non_snake_case)]
    let MASK: __m128i = _mm_set_epi64x(
        0x0C0D_0E0F_0809_0A0Bu64 as i64,
        0x0405_0607_0001_0203u64 as i64,
    );
    let state_ptr = state.as_ptr() as *const __m128i;
    let dcba = _mm_loadu_si128(state_ptr.add(0));
    let efgh = _mm_loadu_si128(state_ptr.add(1));
    let cdab = _mm_shuffle_epi32(dcba, 0xB1);
    let efgh = _mm_shuffle_epi32(efgh, 0x1B);
    let mut abef = _mm_alignr_epi8(cdab, efgh, 8);
    let mut cdgh = _mm_blend_epi16(efgh, cdab, 0xF0);
    for block in blocks {
        let abef_save = abef;
        let cdgh_save = cdgh;
        let data_ptr = block.as_ptr() as *const __m128i;
        let mut w0 = _mm_shuffle_epi8(_mm_loadu_si128(data_ptr.add(0)), MASK);
        let mut w1 = _mm_shuffle_epi8(_mm_loadu_si128(data_ptr.add(1)), MASK);
        let mut w2 = _mm_shuffle_epi8(_mm_loadu_si128(data_ptr.add(2)), MASK);
        let mut w3 = _mm_shuffle_epi8(_mm_loadu_si128(data_ptr.add(3)), MASK);
        let mut w4;
        rounds4!(abef, cdgh, w0, 0);
        rounds4!(abef, cdgh, w1, 1);
        rounds4!(abef, cdgh, w2, 2);
        rounds4!(abef, cdgh, w3, 3);
        schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 4);
        schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 5);
        schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 6);
        schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 7);
        schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 8);
        schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 9);
        schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 10);
        schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 11);
        schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 12);
        schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 13);
        schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 14);
        schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 15);
        abef = _mm_add_epi32(abef, abef_save);
        cdgh = _mm_add_epi32(cdgh, cdgh_save);
    }
    let feba = _mm_shuffle_epi32(abef, 0x1B);
    let dchg = _mm_shuffle_epi32(cdgh, 0xB1);
    let dcba = _mm_blend_epi16(feba, dchg, 0xF0);
    let hgef = _mm_alignr_epi8(dchg, feba, 8);
    let state_ptr_mut = state.as_mut_ptr() as *mut __m128i;
    _mm_storeu_si128(state_ptr_mut.add(0), dcba);
    _mm_storeu_si128(state_ptr_mut.add(1), hgef);
}
cpufeatures::new!(shani_cpuid, "sha", "sse2", "ssse3", "sse4.1");
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    if shani_cpuid::get() {
        unsafe {
            digest_blocks(state, blocks);
        }
    } else {
        super::soft::compress(state, blocks);
    }
}
#[cfg(test)]
mod tests_llm_16_24 {
    use crate::sha256::x86::compress;
    #[test]
    fn test_compress() {
        let _rug_st_tests_llm_16_24_rrrruuuugggg_test_compress = 0;
        let rug_fuzz_0 = 0x6a09e667;
        let rug_fuzz_1 = 0xbb67ae85;
        let rug_fuzz_2 = 0x3c6ef372;
        let rug_fuzz_3 = 0xa54ff53a;
        let rug_fuzz_4 = 0x510e527f;
        let rug_fuzz_5 = 0x9b05688c;
        let rug_fuzz_6 = 0x1f83d9ab;
        let rug_fuzz_7 = 0x5be0cd19;
        let rug_fuzz_8 = 0xe3;
        let rug_fuzz_9 = 0xb0;
        let rug_fuzz_10 = 0xc4;
        let rug_fuzz_11 = 0x42;
        let rug_fuzz_12 = 0x98;
        let rug_fuzz_13 = 0xfc;
        let rug_fuzz_14 = 0x1c;
        let rug_fuzz_15 = 0x14;
        let rug_fuzz_16 = 0x9a;
        let rug_fuzz_17 = 0xfb;
        let rug_fuzz_18 = 0xf4;
        let rug_fuzz_19 = 0xc8;
        let rug_fuzz_20 = 0x99;
        let rug_fuzz_21 = 0x6f;
        let rug_fuzz_22 = 0xb9;
        let rug_fuzz_23 = 0x24;
        let rug_fuzz_24 = 0x27;
        let rug_fuzz_25 = 0xae;
        let rug_fuzz_26 = 0x41;
        let rug_fuzz_27 = 0xe4;
        let rug_fuzz_28 = 0x64;
        let rug_fuzz_29 = 0x9b;
        let rug_fuzz_30 = 0x93;
        let rug_fuzz_31 = 0x4c;
        let rug_fuzz_32 = 0xa4;
        let rug_fuzz_33 = 0x95;
        let rug_fuzz_34 = 0x99;
        let rug_fuzz_35 = 0x1b;
        let rug_fuzz_36 = 0x78;
        let rug_fuzz_37 = 0x52;
        let rug_fuzz_38 = 0xb8;
        let rug_fuzz_39 = 0x55;
        let rug_fuzz_40 = 0xb9;
        let rug_fuzz_41 = 0x7a;
        let rug_fuzz_42 = 0x31;
        let rug_fuzz_43 = 0x7f;
        let rug_fuzz_44 = 0xf5;
        let rug_fuzz_45 = 0x1a;
        let rug_fuzz_46 = 0xfb;
        let rug_fuzz_47 = 0xc9;
        let rug_fuzz_48 = 0x61;
        let rug_fuzz_49 = 0x89;
        let rug_fuzz_50 = 0x5f;
        let rug_fuzz_51 = 0xe5;
        let rug_fuzz_52 = 0x75;
        let rug_fuzz_53 = 0xa4;
        let rug_fuzz_54 = 0xa6;
        let rug_fuzz_55 = 0x9f;
        let rug_fuzz_56 = 0x7b;
        let rug_fuzz_57 = 0x4a;
        let rug_fuzz_58 = 0x7a;
        let rug_fuzz_59 = 0x13;
        let rug_fuzz_60 = 0xe4;
        let rug_fuzz_61 = 0xae;
        let rug_fuzz_62 = 0x89;
        let rug_fuzz_63 = 0x3b;
        let rug_fuzz_64 = 0x44;
        let rug_fuzz_65 = 0x54;
        let rug_fuzz_66 = 0x78;
        let rug_fuzz_67 = 0x4b;
        let rug_fuzz_68 = 0x7d;
        let rug_fuzz_69 = 0xfb;
        let rug_fuzz_70 = 0x29;
        let rug_fuzz_71 = 0x2e;
        let rug_fuzz_72 = 0xd89e05c1;
        let rug_fuzz_73 = 0x07d4b2ab;
        let rug_fuzz_74 = 0x6530e69f;
        let rug_fuzz_75 = 0x8b9f46c2;
        let rug_fuzz_76 = 0x7b1d0c3e;
        let rug_fuzz_77 = 0xd187f9b8;
        let rug_fuzz_78 = 0xb7f0c8e5;
        let rug_fuzz_79 = 0xea0a1a99;
        let mut state = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let blocks = [
            [
                rug_fuzz_8,
                rug_fuzz_9,
                rug_fuzz_10,
                rug_fuzz_11,
                rug_fuzz_12,
                rug_fuzz_13,
                rug_fuzz_14,
                rug_fuzz_15,
                rug_fuzz_16,
                rug_fuzz_17,
                rug_fuzz_18,
                rug_fuzz_19,
                rug_fuzz_20,
                rug_fuzz_21,
                rug_fuzz_22,
                rug_fuzz_23,
                rug_fuzz_24,
                rug_fuzz_25,
                rug_fuzz_26,
                rug_fuzz_27,
                rug_fuzz_28,
                rug_fuzz_29,
                rug_fuzz_30,
                rug_fuzz_31,
                rug_fuzz_32,
                rug_fuzz_33,
                rug_fuzz_34,
                rug_fuzz_35,
                rug_fuzz_36,
                rug_fuzz_37,
                rug_fuzz_38,
                rug_fuzz_39,
                rug_fuzz_40,
                rug_fuzz_41,
                rug_fuzz_42,
                rug_fuzz_43,
                rug_fuzz_44,
                rug_fuzz_45,
                rug_fuzz_46,
                rug_fuzz_47,
                rug_fuzz_48,
                rug_fuzz_49,
                rug_fuzz_50,
                rug_fuzz_51,
                rug_fuzz_52,
                rug_fuzz_53,
                rug_fuzz_54,
                rug_fuzz_55,
                rug_fuzz_56,
                rug_fuzz_57,
                rug_fuzz_58,
                rug_fuzz_59,
                rug_fuzz_60,
                rug_fuzz_61,
                rug_fuzz_62,
                rug_fuzz_63,
                rug_fuzz_64,
                rug_fuzz_65,
                rug_fuzz_66,
                rug_fuzz_67,
                rug_fuzz_68,
                rug_fuzz_69,
                rug_fuzz_70,
                rug_fuzz_71,
            ],
        ];
        let expected_state = [
            rug_fuzz_72,
            rug_fuzz_73,
            rug_fuzz_74,
            rug_fuzz_75,
            rug_fuzz_76,
            rug_fuzz_77,
            rug_fuzz_78,
            rug_fuzz_79,
        ];
        compress(&mut state, &blocks);
        debug_assert_eq!(
            state, expected_state, "compress state does not match expected"
        );
        let _rug_ed_tests_llm_16_24_rrrruuuugggg_test_compress = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_26_llm_16_26 {
    use crate::sha256::x86::schedule;
    use std::arch::x86_64::{__m128i, _mm_set_epi32, _mm_loadu_si128, _mm_storeu_si128};
    use std::mem::transmute;
    use std::slice;
    unsafe fn to_array(v: __m128i) -> [u32; 4] {
        let mut arr: [u32; 4] = transmute(v);
        arr.reverse();
        arr
    }
    unsafe fn to_m128i(arr: [u32; 4]) -> __m128i {
        let mut arr = arr;
        arr.reverse();
        transmute(arr)
    }
    #[test]
    fn test_schedule() {
        unsafe {
            let arr_a: [u32; 4] = [0x6d6e6f70, 0x696a6b6c, 0x65666768, 0x61626364];
            let arr_b: [u32; 4] = [0x7d7e7f80, 0x797a7b7c, 0x75767778, 0x71727374];
            let arr_c: [u32; 4] = [0x8c8d8e8f, 0x88898a8b, 0x84858687, 0x80818283];
            let arr_d: [u32; 4] = [0x9c9d9e9f, 0x98999a9b, 0x94959697, 0x90919293];
            let v0 = to_m128i(arr_a);
            let v1 = to_m128i(arr_b);
            let v2 = to_m128i(arr_c);
            let v3 = to_m128i(arr_d);
            let result = schedule(v0, v1, v2, v3);
            let result_arr = to_array(result);
            let expected: [u32; 4] = [0x12345678, 0x9abcdef0, 0x0fedcba9, 0x87654321];
            assert_eq!(
                result_arr, expected,
                "schedule function did not produce the expected result"
            );
        }
    }
}
