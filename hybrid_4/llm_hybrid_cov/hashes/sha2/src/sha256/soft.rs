#![allow(clippy::many_single_char_names)]
use crate::consts::BLOCK_LEN;
use core::convert::TryInto;
#[inline(always)]
fn shl(v: [u32; 4], o: u32) -> [u32; 4] {
    [v[0] >> o, v[1] >> o, v[2] >> o, v[3] >> o]
}
#[inline(always)]
fn shr(v: [u32; 4], o: u32) -> [u32; 4] {
    [v[0] << o, v[1] << o, v[2] << o, v[3] << o]
}
#[inline(always)]
fn or(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] | b[0], a[1] | b[1], a[2] | b[2], a[3] | b[3]]
}
#[inline(always)]
fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}
#[inline(always)]
fn add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ]
}
fn sha256load(v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    [v3[3], v2[0], v2[1], v2[2]]
}
fn sha256swap(v0: [u32; 4]) -> [u32; 4] {
    [v0[2], v0[3], v0[0], v0[1]]
}
fn sha256msg1(v0: [u32; 4], v1: [u32; 4]) -> [u32; 4] {
    #[inline]
    fn sigma0x4(x: [u32; 4]) -> [u32; 4] {
        let t1 = or(shl(x, 7), shr(x, 25));
        let t2 = or(shl(x, 18), shr(x, 14));
        let t3 = shl(x, 3);
        xor(xor(t1, t2), t3)
    }
    add(v0, sigma0x4(sha256load(v0, v1)))
}
fn sha256msg2(v4: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    macro_rules! sigma1 {
        ($a:expr) => {
            $a .rotate_right(17) ^ $a .rotate_right(19) ^ ($a >> 10)
        };
    }
    let [x3, x2, x1, x0] = v4;
    let [w15, w14, _, _] = v3;
    let w16 = x0.wrapping_add(sigma1!(w14));
    let w17 = x1.wrapping_add(sigma1!(w15));
    let w18 = x2.wrapping_add(sigma1!(w16));
    let w19 = x3.wrapping_add(sigma1!(w17));
    [w19, w18, w17, w16]
}
fn sha256_digest_round_x2(cdgh: [u32; 4], abef: [u32; 4], wk: [u32; 4]) -> [u32; 4] {
    macro_rules! big_sigma0 {
        ($a:expr) => {
            ($a .rotate_right(2) ^ $a .rotate_right(13) ^ $a .rotate_right(22))
        };
    }
    macro_rules! big_sigma1 {
        ($a:expr) => {
            ($a .rotate_right(6) ^ $a .rotate_right(11) ^ $a .rotate_right(25))
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
    let [_, _, wk1, wk0] = wk;
    let [a0, b0, e0, f0] = abef;
    let [c0, d0, g0, h0] = cdgh;
    let x0 = big_sigma1!(e0)
        .wrapping_add(bool3ary_202!(e0, f0, g0))
        .wrapping_add(wk0)
        .wrapping_add(h0);
    let y0 = big_sigma0!(a0).wrapping_add(bool3ary_232!(a0, b0, c0));
    let (a1, b1, c1, d1, e1, f1, g1, h1) = (
        x0.wrapping_add(y0),
        a0,
        b0,
        c0,
        x0.wrapping_add(d0),
        e0,
        f0,
        g0,
    );
    let x1 = big_sigma1!(e1)
        .wrapping_add(bool3ary_202!(e1, f1, g1))
        .wrapping_add(wk1)
        .wrapping_add(h1);
    let y1 = big_sigma0!(a1).wrapping_add(bool3ary_232!(a1, b1, c1));
    let (a2, b2, _, _, e2, f2, _, _) = (
        x1.wrapping_add(y1),
        a1,
        b1,
        c1,
        x1.wrapping_add(d1),
        e1,
        f1,
        g1,
    );
    [a2, b2, e2, f2]
}
fn schedule(v0: [u32; 4], v1: [u32; 4], v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    let t1 = sha256msg1(v0, v1);
    let t2 = sha256load(v2, v3);
    let t3 = add(t1, t2);
    sha256msg2(t3, v3)
}
macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => {
        { let t1 = add($rest, crate ::consts::K32X4[$i]); $cdgh =
        sha256_digest_round_x2($cdgh, $abef, t1); let t2 = sha256swap(t1); $abef =
        sha256_digest_round_x2($abef, $cdgh, t2); }
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
/// Process a block with the SHA-256 algorithm.
fn sha256_digest_block_u32(state: &mut [u32; 8], block: &[u32; 16]) {
    let mut abef = [state[0], state[1], state[4], state[5]];
    let mut cdgh = [state[2], state[3], state[6], state[7]];
    let mut w0 = [block[3], block[2], block[1], block[0]];
    let mut w1 = [block[7], block[6], block[5], block[4]];
    let mut w2 = [block[11], block[10], block[9], block[8]];
    let mut w3 = [block[15], block[14], block[13], block[12]];
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
    let [a, b, e, f] = abef;
    let [c, d, g, h] = cdgh;
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    let mut block_u32 = [0u32; BLOCK_LEN];
    let mut state_cpy = *state;
    for block in blocks {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(4)) {
            *o = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        sha256_digest_block_u32(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    #[test]
    fn schedule_test() {
        let _rug_st_tests_llm_16_13_rrrruuuugggg_schedule_test = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let v0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let v1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let v2: [u32; 4] = [rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11];
        let v3: [u32; 4] = [rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15];
        let expected: [u32; 4] = [rug_fuzz_16, rug_fuzz_17, rug_fuzz_18, rug_fuzz_19];
        let result = schedule(v0, v1, v2, v3);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_13_rrrruuuugggg_schedule_test = 0;
    }
}
#[cfg(test)]
mod tests_rug_180 {
    use crate::sha256::soft::shl;
    #[test]
    fn test_shl() {
        let _rug_st_tests_rug_180_rrrruuuugggg_test_shl = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 0xdeadbeef;
        let rug_fuzz_3 = 0xfeedface;
        let rug_fuzz_4 = 4;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: u32 = rug_fuzz_4;
        shl(p0, p1);
        let _rug_ed_tests_rug_180_rrrruuuugggg_test_shl = 0;
    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    #[test]
    fn test_shr() {
        let _rug_st_tests_rug_181_rrrruuuugggg_test_shr = 0;
        let rug_fuzz_0 = 0x0u32;
        let rug_fuzz_1 = 3;
        let mut p0: [u32; 4] = [rug_fuzz_0; 4];
        let mut p1: u32 = rug_fuzz_1;
        crate::sha256::soft::shr(p0, p1);
        let _rug_ed_tests_rug_181_rrrruuuugggg_test_shr = 0;
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_182_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 0x13579bdf;
        let rug_fuzz_3 = 0xfedcba98;
        let rug_fuzz_4 = 0xffffffff;
        let rug_fuzz_5 = 0x00000000;
        let rug_fuzz_6 = 0xf0f0f0f0;
        let rug_fuzz_7 = 0x0f0f0f0f;
        let p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::sha256::soft::or(p0, p1);
        debug_assert_eq!(
            result, [p0[0] | p1[0], p0[1] | p1[1], p0[2] | p1[2], p0[3] | p1[3]]
        );
        let _rug_ed_tests_rug_182_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    #[test]
    fn test_xor() {
        let _rug_st_tests_rug_183_rrrruuuugggg_test_xor = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x9abcdef0;
        let rug_fuzz_2 = 0xdeadbeef;
        let rug_fuzz_3 = 0xfeedc0de;
        let rug_fuzz_4 = 0x87654321;
        let rug_fuzz_5 = 0x01234567;
        let rug_fuzz_6 = 0xbeefdead;
        let rug_fuzz_7 = 0xc0defeed;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::sha256::soft::xor(p0, p1);
        debug_assert_eq!(result, [0x95511559, 0x99fba997, 0x60356122, 0x3ed3cf33]);
        let _rug_ed_tests_rug_183_rrrruuuugggg_test_xor = 0;
    }
}
#[cfg(test)]
mod tests_rug_184 {
    use super::*;
    #[test]
    fn test_add() {
        let _rug_st_tests_rug_184_rrrruuuugggg_test_add = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 6;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 8;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::sha256::soft::add(p0, p1);
        debug_assert_eq!(result, [6, 8, 10, 12]);
        let _rug_ed_tests_rug_184_rrrruuuugggg_test_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_185_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x87654321;
        let rug_fuzz_2 = 0xABCDEF01;
        let rug_fuzz_3 = 0x10FEDCBA;
        let rug_fuzz_4 = 0xA1B2C3D4;
        let rug_fuzz_5 = 0xE5F60708;
        let rug_fuzz_6 = 0x11121314;
        let rug_fuzz_7 = 0x15161718;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::sha256::soft::sha256load(p0, p1);
        debug_assert_eq!(result, [p1[3], p0[0], p0[1], p0[2]]);
        let _rug_ed_tests_rug_185_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_186 {
    use super::*;
    #[test]
    fn test_sha256swap() {
        let _rug_st_tests_rug_186_rrrruuuugggg_test_sha256swap = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let result = crate::sha256::soft::sha256swap(p0);
        debug_assert_eq!(result, [3, 4, 1, 2]);
        let _rug_ed_tests_rug_186_rrrruuuugggg_test_sha256swap = 0;
    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    #[test]
    fn test_sha256msg1() {
        let _rug_st_tests_rug_187_rrrruuuugggg_test_sha256msg1 = 0;
        let rug_fuzz_0 = 111111111;
        let rug_fuzz_1 = 222222222;
        let rug_fuzz_2 = 333333333;
        let rug_fuzz_3 = 444444444;
        let rug_fuzz_4 = 555555555;
        let rug_fuzz_5 = 666666666;
        let rug_fuzz_6 = 777777777;
        let rug_fuzz_7 = 888888888;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        crate::sha256::soft::sha256msg1(p0, p1);
        let _rug_ed_tests_rug_187_rrrruuuugggg_test_sha256msg1 = 0;
    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_189_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x428a2f98;
        let rug_fuzz_1 = 0x71374491;
        let rug_fuzz_2 = 0xb5c0fbcf;
        let rug_fuzz_3 = 0xe9b5dba5;
        let rug_fuzz_4 = 0x3956c25b;
        let rug_fuzz_5 = 0x59f111f1;
        let rug_fuzz_6 = 0x923f82a4;
        let rug_fuzz_7 = 0xab1c5ed5;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        crate::sha256::soft::sha256msg2(p0, p1);
        let _rug_ed_tests_rug_189_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    #[test]
    fn test_sha256_digest_round_x2() {
        let _rug_st_tests_rug_190_rrrruuuugggg_test_sha256_digest_round_x2 = 0;
        let rug_fuzz_0 = 12345678;
        let rug_fuzz_1 = 87654321;
        let rug_fuzz_2 = 123454321;
        let rug_fuzz_3 = 43214321;
        let rug_fuzz_4 = 11111111;
        let rug_fuzz_5 = 22222222;
        let rug_fuzz_6 = 33333333;
        let rug_fuzz_7 = 44444444;
        let rug_fuzz_8 = 55555555;
        let rug_fuzz_9 = 66666666;
        let rug_fuzz_10 = 77777777;
        let rug_fuzz_11 = 88888888;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let mut p2: [u32; 4] = [rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11];
        crate::sha256::soft::sha256_digest_round_x2(p0, p1, p2);
        let _rug_ed_tests_rug_190_rrrruuuugggg_test_sha256_digest_round_x2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    #[test]
    fn test_sha256_digest_block_u32() {
        let _rug_st_tests_rug_191_rrrruuuugggg_test_sha256_digest_block_u32 = 0;
        let rug_fuzz_0 = 0x6a09e667;
        let rug_fuzz_1 = 0xbb67ae85;
        let rug_fuzz_2 = 0x3c6ef372;
        let rug_fuzz_3 = 0xa54ff53a;
        let rug_fuzz_4 = 0x510e527f;
        let rug_fuzz_5 = 0x9b05688c;
        let rug_fuzz_6 = 0x1f83d9ab;
        let rug_fuzz_7 = 0x5be0cd19;
        let rug_fuzz_8 = 0xd807aa98;
        let rug_fuzz_9 = 0x12835b01;
        let rug_fuzz_10 = 0x243185be;
        let rug_fuzz_11 = 0x550c7dc3;
        let rug_fuzz_12 = 0x72be5d74;
        let rug_fuzz_13 = 0x80deb1fe;
        let rug_fuzz_14 = 0x9bdc06a7;
        let rug_fuzz_15 = 0xc19bf174;
        let rug_fuzz_16 = 0xe49b69c1;
        let rug_fuzz_17 = 0xefbe4786;
        let rug_fuzz_18 = 0x0fc19dc6;
        let rug_fuzz_19 = 0x240ca1cc;
        let rug_fuzz_20 = 0x2de92c6f;
        let rug_fuzz_21 = 0x4a7484aa;
        let rug_fuzz_22 = 0x5cb0a9dc;
        let rug_fuzz_23 = 0x76f988da;
        let mut p0: [u32; 8] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut p1: [u32; 16] = [
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
        crate::sha256::soft::sha256_digest_block_u32(&mut p0, &p1);
        let _rug_ed_tests_rug_191_rrrruuuugggg_test_sha256_digest_block_u32 = 0;
    }
}
