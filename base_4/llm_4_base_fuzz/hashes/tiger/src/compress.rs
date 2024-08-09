use super::tables::{T1, T2, T3, T4};
use super::State;
use core::convert::TryInto;
#[inline(always)]
fn round(a: &mut u64, b: &mut u64, c: &mut u64, x: &u64, mul: u8) {
    *c ^= *x;
    let c2: [u8; 8] = c.to_le_bytes();
    let a2 = T1[usize::from(c2[0])] ^ T2[usize::from(c2[2])] ^ T3[usize::from(c2[4])]
        ^ T4[usize::from(c2[6])];
    let b2 = T4[usize::from(c2[1])] ^ T3[usize::from(c2[3])] ^ T2[usize::from(c2[5])]
        ^ T1[usize::from(c2[7])];
    *a = a.wrapping_sub(a2);
    *b = b.wrapping_add(b2).wrapping_mul(u64::from(mul));
}
#[inline(always)]
fn pass(a: &mut u64, b: &mut u64, c: &mut u64, x: &[u64; 8], mul: u8) {
    round(a, b, c, &x[0], mul);
    round(b, c, a, &x[1], mul);
    round(c, a, b, &x[2], mul);
    round(a, b, c, &x[3], mul);
    round(b, c, a, &x[4], mul);
    round(c, a, b, &x[5], mul);
    round(a, b, c, &x[6], mul);
    round(b, c, a, &x[7], mul);
}
#[inline(always)]
fn key_schedule(x: &mut [u64; 8]) {
    x[0] = x[0].wrapping_sub(x[7] ^ 0xA5A5_A5A5_A5A5_A5A5);
    x[1] ^= x[0];
    x[2] = x[2].wrapping_add(x[1]);
    x[3] = x[3].wrapping_sub(x[2] ^ ((!x[1]) << 19));
    x[4] ^= x[3];
    x[5] = x[5].wrapping_add(x[4]);
    x[6] = x[6].wrapping_sub(x[5] ^ ((!x[4]) >> 23));
    x[7] ^= x[6];
    x[0] = x[0].wrapping_add(x[7]);
    x[1] = x[1].wrapping_sub(x[0] ^ ((!x[7]) << 19));
    x[2] ^= x[1];
    x[3] = x[3].wrapping_add(x[2]);
    x[4] = x[4].wrapping_sub(x[3] ^ ((!x[2]) >> 23));
    x[5] ^= x[4];
    x[6] = x[6].wrapping_add(x[5]);
    x[7] = x[7].wrapping_sub(x[6] ^ 0x0123_4567_89AB_CDEF);
}
pub(crate) fn compress(state: &mut State, raw_block: &[u8; 64]) {
    let mut block: [u64; 8] = Default::default();
    for (o, chunk) in block.iter_mut().zip(raw_block.chunks_exact(8)) {
        *o = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    let [mut a, mut b, mut c] = *state;
    pass(&mut a, &mut b, &mut c, &block, 5);
    key_schedule(&mut block);
    pass(&mut c, &mut a, &mut b, &block, 7);
    key_schedule(&mut block);
    pass(&mut b, &mut c, &mut a, &block, 9);
    state[0] ^= a;
    state[1] = b.wrapping_sub(state[1]);
    state[2] = c.wrapping_add(state[2]);
}
#[cfg(test)]
mod tests_llm_16_11_llm_16_11 {
    use crate::compress;
    use crate::State;
    #[test]
    fn test_compress() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_test_compress = 0;
        let rug_fuzz_0 = 0x0123456789ABCDEF;
        let rug_fuzz_1 = 0xFEDCBA9876543210;
        let rug_fuzz_2 = 0x0FEDCBA987654321;
        let rug_fuzz_3 = 0x01;
        let rug_fuzz_4 = 0x23;
        let rug_fuzz_5 = 0x45;
        let rug_fuzz_6 = 0x67;
        let rug_fuzz_7 = 0x89;
        let rug_fuzz_8 = 0xAB;
        let rug_fuzz_9 = 0xCD;
        let rug_fuzz_10 = 0xEF;
        let rug_fuzz_11 = 0xFE;
        let rug_fuzz_12 = 0xDC;
        let rug_fuzz_13 = 0xBA;
        let rug_fuzz_14 = 0x98;
        let rug_fuzz_15 = 0x76;
        let rug_fuzz_16 = 0x54;
        let rug_fuzz_17 = 0x32;
        let rug_fuzz_18 = 0x10;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 0;
        let rug_fuzz_29 = 0;
        let rug_fuzz_30 = 0;
        let rug_fuzz_31 = 0;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 0;
        let rug_fuzz_35 = 0;
        let rug_fuzz_36 = 0;
        let rug_fuzz_37 = 0;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 0;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 0;
        let rug_fuzz_44 = 0;
        let rug_fuzz_45 = 0;
        let rug_fuzz_46 = 0;
        let rug_fuzz_47 = 0;
        let rug_fuzz_48 = 0;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 0;
        let rug_fuzz_51 = 0;
        let rug_fuzz_52 = 0;
        let rug_fuzz_53 = 0;
        let rug_fuzz_54 = 0;
        let rug_fuzz_55 = 0;
        let rug_fuzz_56 = 0;
        let rug_fuzz_57 = 0;
        let rug_fuzz_58 = 0;
        let rug_fuzz_59 = 0;
        let rug_fuzz_60 = 0;
        let rug_fuzz_61 = 0;
        let rug_fuzz_62 = 0;
        let rug_fuzz_63 = 0;
        let rug_fuzz_64 = 0;
        let rug_fuzz_65 = 0;
        let rug_fuzz_66 = 0;
        let rug_fuzz_67 = 0x0123456789ABCDEF;
        let rug_fuzz_68 = 0xFEDCBA9876543210;
        let rug_fuzz_69 = 0x0FEDCBA987654321;
        let mut initial_state: State = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let raw_block: [u8; 64] = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
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
        ];
        let expected_state: State = [rug_fuzz_67, rug_fuzz_68, rug_fuzz_69];
        compress(&mut initial_state, &raw_block);
        debug_assert_eq!(
            initial_state, expected_state,
            "State after compression should match the expected state."
        );
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_test_compress = 0;
    }
}
