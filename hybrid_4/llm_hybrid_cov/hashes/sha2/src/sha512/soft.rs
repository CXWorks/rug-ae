#![allow(clippy::many_single_char_names)]
use crate::consts::{BLOCK_LEN, K64X2};
use core::convert::TryInto;
fn add(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    [a[0].wrapping_add(b[0]), a[1].wrapping_add(b[1])]
}
/// Not an intrinsic, but works like an unaligned load.
fn sha512load(v0: [u64; 2], v1: [u64; 2]) -> [u64; 2] {
    [v1[1], v0[0]]
}
/// Performs 2 rounds of the SHA-512 message schedule update.
pub fn sha512_schedule_x2(
    v0: [u64; 2],
    v1: [u64; 2],
    v4to5: [u64; 2],
    v7: [u64; 2],
) -> [u64; 2] {
    fn sigma0(x: u64) -> u64 {
        ((x << 63) | (x >> 1)) ^ ((x << 56) | (x >> 8)) ^ (x >> 7)
    }
    fn sigma1(x: u64) -> u64 {
        ((x << 45) | (x >> 19)) ^ ((x << 3) | (x >> 61)) ^ (x >> 6)
    }
    let [w1, w0] = v0;
    let [_, w2] = v1;
    let [w10, w9] = v4to5;
    let [w15, w14] = v7;
    let w16 = sigma1(w14).wrapping_add(w9).wrapping_add(sigma0(w1)).wrapping_add(w0);
    let w17 = sigma1(w15).wrapping_add(w10).wrapping_add(sigma0(w2)).wrapping_add(w1);
    [w17, w16]
}
/// Performs one round of the SHA-512 message block digest.
pub fn sha512_digest_round(
    ae: [u64; 2],
    bf: [u64; 2],
    cg: [u64; 2],
    dh: [u64; 2],
    wk0: u64,
) -> [u64; 2] {
    macro_rules! big_sigma0 {
        ($a:expr) => {
            ($a .rotate_right(28) ^ $a .rotate_right(34) ^ $a .rotate_right(39))
        };
    }
    macro_rules! big_sigma1 {
        ($a:expr) => {
            ($a .rotate_right(14) ^ $a .rotate_right(18) ^ $a .rotate_right(41))
        };
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => {
            $c ^ ($a & ($b ^ $c))
        };
    }
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) ^ ($a & $c) ^ ($b & $c)
        };
    }
    let [a0, e0] = ae;
    let [b0, f0] = bf;
    let [c0, g0] = cg;
    let [d0, h0] = dh;
    let x0 = big_sigma1!(e0)
        .wrapping_add(bool3ary_202!(e0, f0, g0))
        .wrapping_add(wk0)
        .wrapping_add(h0);
    let y0 = big_sigma0!(a0).wrapping_add(bool3ary_232!(a0, b0, c0));
    let (a1, _, _, _, e1, _, _, _) = (
        x0.wrapping_add(y0),
        a0,
        b0,
        c0,
        x0.wrapping_add(d0),
        e0,
        f0,
        g0,
    );
    [a1, e1]
}
/// Process a block with the SHA-512 algorithm.
pub fn sha512_digest_block_u64(state: &mut [u64; 8], block: &[u64; 16]) {
    let k = &K64X2;
    macro_rules! schedule {
        ($v0:expr, $v1:expr, $v4:expr, $v5:expr, $v7:expr) => {
            sha512_schedule_x2($v0, $v1, sha512load($v4, $v5), $v7)
        };
    }
    macro_rules! rounds4 {
        ($ae:ident, $bf:ident, $cg:ident, $dh:ident, $wk0:expr, $wk1:expr) => {
            { let [u, t] = $wk0; let [w, v] = $wk1; $dh = sha512_digest_round($ae, $bf,
            $cg, $dh, t); $cg = sha512_digest_round($dh, $ae, $bf, $cg, u); $bf =
            sha512_digest_round($cg, $dh, $ae, $bf, v); $ae = sha512_digest_round($bf,
            $cg, $dh, $ae, w); }
        };
    }
    let mut ae = [state[0], state[4]];
    let mut bf = [state[1], state[5]];
    let mut cg = [state[2], state[6]];
    let mut dh = [state[3], state[7]];
    let (mut w1, mut w0) = ([block[3], block[2]], [block[1], block[0]]);
    rounds4!(ae, bf, cg, dh, add(k[0], w0), add(k[1], w1));
    let (mut w3, mut w2) = ([block[7], block[6]], [block[5], block[4]]);
    rounds4!(ae, bf, cg, dh, add(k[2], w2), add(k[3], w3));
    let (mut w5, mut w4) = ([block[11], block[10]], [block[9], block[8]]);
    rounds4!(ae, bf, cg, dh, add(k[4], w4), add(k[5], w5));
    let (mut w7, mut w6) = ([block[15], block[14]], [block[13], block[12]]);
    rounds4!(ae, bf, cg, dh, add(k[6], w6), add(k[7], w7));
    let mut w8 = schedule!(w0, w1, w4, w5, w7);
    let mut w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[8], w8), add(k[9], w9));
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[10], w0), add(k[11], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[12], w2), add(k[13], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[14], w4), add(k[15], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[16], w6), add(k[17], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[18], w8), add(k[19], w9));
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[20], w0), add(k[21], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[22], w2), add(k[23], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[24], w4), add(k[25], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[26], w6), add(k[27], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[28], w8), add(k[29], w9));
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[30], w0), add(k[31], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[32], w2), add(k[33], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[34], w4), add(k[35], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[36], w6), add(k[37], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[38], w8), add(k[39], w9));
    let [a, e] = ae;
    let [b, f] = bf;
    let [c, g] = cg;
    let [d, h] = dh;
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}
pub fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) {
    let mut block_u32 = [0u64; BLOCK_LEN];
    let mut state_cpy = *state;
    for block in blocks {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(8)) {
            *o = u64::from_be_bytes(chunk.try_into().unwrap());
        }
        sha512_digest_block_u64(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_rug_195 {
    use crate::sha512::soft::add;
    #[test]
    fn test_add() {
        let _rug_st_tests_rug_195_rrrruuuugggg_test_add = 0;
        let rug_fuzz_0 = 12345678901234567890;
        let rug_fuzz_1 = 10987654321098765432;
        let rug_fuzz_2 = 9876543210987654321;
        let rug_fuzz_3 = 12345678901234567890;
        let mut p0: [u64; 2] = [rug_fuzz_0, rug_fuzz_1];
        let mut p1: [u64; 2] = [rug_fuzz_2, rug_fuzz_3];
        let result = add(p0, p1);
        debug_assert_eq!(result, [p0[0].wrapping_add(p1[0]), p0[1].wrapping_add(p1[1])]);
        let _rug_ed_tests_rug_195_rrrruuuugggg_test_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_196 {
    use super::*;
    #[test]
    fn test_sha512load() {
        let _rug_st_tests_rug_196_rrrruuuugggg_test_sha512load = 0;
        let rug_fuzz_0 = 0x123456789ABCDEF0;
        let rug_fuzz_1 = 0x0FEDCBA987654321;
        let rug_fuzz_2 = 0xFEDCBA9876543210;
        let rug_fuzz_3 = 0x0123456789ABCDEF;
        let mut p0: [u64; 2] = [rug_fuzz_0, rug_fuzz_1];
        let mut p1: [u64; 2] = [rug_fuzz_2, rug_fuzz_3];
        let result = crate::sha512::soft::sha512load(p0, p1);
        debug_assert_eq!(result, [p1[1], p0[0]]);
        let _rug_ed_tests_rug_196_rrrruuuugggg_test_sha512load = 0;
    }
}
#[cfg(test)]
mod tests_rug_197 {
    use super::*;
    #[test]
    fn test_sha512_schedule_x2() {
        let _rug_st_tests_rug_197_rrrruuuugggg_test_sha512_schedule_x2 = 0;
        let rug_fuzz_0 = 0x428a2f98d728ae22;
        let rug_fuzz_1 = 0x7137449123ef65cd;
        let rug_fuzz_2 = 0xb5c0fbcfec4d3b2f;
        let rug_fuzz_3 = 0xe9b5dba58189dbbc;
        let rug_fuzz_4 = 0x3956c25bf348b538;
        let rug_fuzz_5 = 0x59f111f1b605d019;
        let rug_fuzz_6 = 0x923f82a4af194f9b;
        let rug_fuzz_7 = 0xab1c5ed5da6d8118;
        let mut p0: [u64; 2] = [rug_fuzz_0, rug_fuzz_1];
        let mut p1: [u64; 2] = [rug_fuzz_2, rug_fuzz_3];
        let mut p2: [u64; 2] = [rug_fuzz_4, rug_fuzz_5];
        let mut p3: [u64; 2] = [rug_fuzz_6, rug_fuzz_7];
        crate::sha512::soft::sha512_schedule_x2(p0, p1, p2, p3);
        let _rug_ed_tests_rug_197_rrrruuuugggg_test_sha512_schedule_x2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_200 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_200_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x6a09e667f3bcc908;
        let rug_fuzz_1 = 0x510e527fade682d1;
        let rug_fuzz_2 = 0xbb67ae8584caa73b;
        let rug_fuzz_3 = 0x9b05688c2b3e6c1f;
        let rug_fuzz_4 = 0x3c6ef372fe94f82b;
        let rug_fuzz_5 = 0xa54ff53a5f1d36f1;
        let rug_fuzz_6 = 0x5be0cd19137e2179;
        let rug_fuzz_7 = 0x1f83d9abfb41bd6b;
        let rug_fuzz_8 = 0x428a2f98d728ae22;
        let mut p0: [u64; 2] = [rug_fuzz_0, rug_fuzz_1];
        let mut p1: [u64; 2] = [rug_fuzz_2, rug_fuzz_3];
        let mut p2: [u64; 2] = [rug_fuzz_4, rug_fuzz_5];
        let mut p3: [u64; 2] = [rug_fuzz_6, rug_fuzz_7];
        let mut p4: u64 = rug_fuzz_8;
        let result = crate::sha512::soft::sha512_digest_round(p0, p1, p2, p3, p4);
        let _rug_ed_tests_rug_200_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_201 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_201_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x6a09e667f3bcc908;
        let rug_fuzz_1 = 0xbb67ae8584caa73b;
        let rug_fuzz_2 = 0x3c6ef372fe94f82b;
        let rug_fuzz_3 = 0xa54ff53a5f1d36f1;
        let rug_fuzz_4 = 0x510e527fade682d1;
        let rug_fuzz_5 = 0x9b05688c2b3e6c1f;
        let rug_fuzz_6 = 0x1f83d9abfb41bd6b;
        let rug_fuzz_7 = 0x5be0cd19137e2179;
        let rug_fuzz_8 = 0xd89e05c107d57ee2;
        let rug_fuzz_9 = 0x17f5a5cab253566c;
        let rug_fuzz_10 = 0xd89e05c107d57ee2;
        let rug_fuzz_11 = 0x17f5a5cab253566c;
        let rug_fuzz_12 = 0xd89e05c107d57ee2;
        let rug_fuzz_13 = 0x17f5a5cab253566c;
        let rug_fuzz_14 = 0xd89e05c107d57ee2;
        let rug_fuzz_15 = 0x17f5a5cab253566c;
        let rug_fuzz_16 = 0xd89e05c107d57ee2;
        let rug_fuzz_17 = 0x17f5a5cab253566c;
        let rug_fuzz_18 = 0xd89e05c107d57ee2;
        let rug_fuzz_19 = 0x17f5a5cab253566c;
        let rug_fuzz_20 = 0xd89e05c107d57ee2;
        let rug_fuzz_21 = 0x17f5a5cab253566c;
        let rug_fuzz_22 = 0xd89e05c107d57ee2;
        let rug_fuzz_23 = 0x17f5a5cab253566c;
        let mut p0: [u64; 8] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut p1: [u64; 16] = [
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
        ];
        crate::sha512::soft::sha512_digest_block_u64(&mut p0, &p1);
        let _rug_ed_tests_rug_201_rrrruuuugggg_test_rug = 0;
    }
}
