#![allow(clippy::needless_range_loop)]
use crate::table::TABLE;
use core::{convert::TryInto, u64};
pub(crate) const COLS: usize = 8;
const ROUNDS: u64 = 10;
#[inline(always)]
fn column(x: &[u64; COLS], c: [usize; 8]) -> u64 {
    let mut t = 0;
    for i in 0..8 {
        let sl = 8 * (7 - i);
        let idx = ((x[c[i]] >> sl) & 0xFF) as usize;
        t ^= TABLE[i][idx];
    }
    t
}
#[inline(always)]
fn rndq(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= u64::MAX.wrapping_sub((i as u64) << 4) ^ r;
    }
    [
        column(&x, [1, 3, 5, 7, 0, 2, 4, 6]),
        column(&x, [2, 4, 6, 0, 1, 3, 5, 7]),
        column(&x, [3, 5, 7, 1, 2, 4, 6, 0]),
        column(&x, [4, 6, 0, 2, 3, 5, 7, 1]),
        column(&x, [5, 7, 1, 3, 4, 6, 0, 2]),
        column(&x, [6, 0, 2, 4, 5, 7, 1, 3]),
        column(&x, [7, 1, 3, 5, 6, 0, 2, 4]),
        column(&x, [0, 2, 4, 6, 7, 1, 3, 5]),
    ]
}
#[inline(always)]
fn rndp(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= ((i as u64) << 60) ^ r;
    }
    [
        column(&x, [0, 1, 2, 3, 4, 5, 6, 7]),
        column(&x, [1, 2, 3, 4, 5, 6, 7, 0]),
        column(&x, [2, 3, 4, 5, 6, 7, 0, 1]),
        column(&x, [3, 4, 5, 6, 7, 0, 1, 2]),
        column(&x, [4, 5, 6, 7, 0, 1, 2, 3]),
        column(&x, [5, 6, 7, 0, 1, 2, 3, 4]),
        column(&x, [6, 7, 0, 1, 2, 3, 4, 5]),
        column(&x, [7, 0, 1, 2, 3, 4, 5, 6]),
    ]
}
pub(crate) fn compress(h: &mut [u64; COLS], block: &[u8; 64]) {
    let mut q = [0u64; COLS];
    for (chunk, v) in block.chunks_exact(8).zip(q.iter_mut()) {
        *v = u64::from_be_bytes(chunk.try_into().unwrap());
    }
    let mut p = [0u64; COLS];
    for i in 0..COLS {
        p[i] = h[i] ^ q[i];
    }
    for i in 0..ROUNDS {
        q = rndq(q, i);
    }
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        h[i] ^= q[i] ^ p[i];
    }
}
pub(crate) fn p(h: &[u64; COLS]) -> [u64; COLS] {
    let mut p = *h;
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        p[i] ^= h[i];
    }
    p
}
#[cfg(test)]
mod tests_llm_16_18_llm_16_18 {
    use super::*;
    use crate::*;
    #[test]
    fn test_rndq() {
        let _rug_st_tests_llm_16_18_llm_16_18_rrrruuuugggg_test_rndq = 0;
        let rug_fuzz_0 = 0x0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0xf0e0d0c0b0a0908;
        let test_cases = [([rug_fuzz_0; COLS], rug_fuzz_1, [rug_fuzz_2; COLS])];
        for (input, round, expected) in test_cases.iter() {
            debug_assert_eq!(rndq(* input, * round), * expected);
        }
        let _rug_ed_tests_llm_16_18_llm_16_18_rrrruuuugggg_test_rndq = 0;
    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    #[test]
    fn test_column() {
        let _rug_st_tests_rug_111_rrrruuuugggg_test_column = 0;
        let rug_fuzz_0 = 0x0123456789abcdef;
        let rug_fuzz_1 = 0xfedcba9876543210;
        let rug_fuzz_2 = 0x0f1e2d3c4b5a6978;
        let rug_fuzz_3 = 0x89abcdef01234567;
        let rug_fuzz_4 = 0x76543210fedcba98;
        let rug_fuzz_5 = 0x89abcdef01234567;
        let rug_fuzz_6 = 0xfedcba9876543210;
        let rug_fuzz_7 = 0x0123456789abcdef;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 6;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 4;
        let rug_fuzz_12 = 3;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 0;
        let mut p0: [u64; COLS] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut p1: [usize; 8] = [
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let result = crate::compress512::column(&p0, p1);
        let _rug_ed_tests_rug_111_rrrruuuugggg_test_column = 0;
    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    #[test]
    fn test_rndp() {
        let _rug_st_tests_rug_112_rrrruuuugggg_test_rndp = 0;
        let rug_fuzz_0 = 0xFEDCBA9876543210;
        let rug_fuzz_1 = 0x0123456789ABCDEF;
        let rug_fuzz_2 = 0x0011223344556677;
        let rug_fuzz_3 = 0x8899AABBCCDDEEFF;
        let rug_fuzz_4 = 0xFFEEDDCCBBAA9988;
        let rug_fuzz_5 = 0x7766554433221100;
        let rug_fuzz_6 = 0xEFCDAB8967452301;
        let rug_fuzz_7 = 0x1032547698BADCFE;
        let rug_fuzz_8 = 32;
        let mut p0: [u64; COLS] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut p1: u64 = rug_fuzz_8;
        let _ = crate::compress512::rndp(p0, p1);
        let _rug_ed_tests_rug_112_rrrruuuugggg_test_rndp = 0;
    }
}
