#![allow(clippy::many_single_char_names)]
use super::BLOCK_SIZE;
use core::convert::TryInto;
const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];
#[inline(always)]
fn add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ]
}
#[inline(always)]
fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}
#[inline]
pub fn sha1_first_add(e: u32, w0: [u32; 4]) -> [u32; 4] {
    let [a, b, c, d] = w0;
    [e.wrapping_add(a), b, c, d]
}
fn sha1msg1(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let [_, _, w2, w3] = a;
    let [w4, w5, _, _] = b;
    [a[0] ^ w2, a[1] ^ w3, a[2] ^ w4, a[3] ^ w5]
}
fn sha1msg2(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let [x0, x1, x2, x3] = a;
    let [_, w13, w14, w15] = b;
    let w16 = (x0 ^ w13).rotate_left(1);
    let w17 = (x1 ^ w14).rotate_left(1);
    let w18 = (x2 ^ w15).rotate_left(1);
    let w19 = (x3 ^ w16).rotate_left(1);
    [w16, w17, w18, w19]
}
#[inline]
fn sha1_first_half(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    sha1_first_add(abcd[0].rotate_left(30), msg)
}
fn sha1_digest_round_x4(abcd: [u32; 4], work: [u32; 4], i: i8) -> [u32; 4] {
    match i {
        0 => sha1rnds4c(abcd, add(work, [K[0]; 4])),
        1 => sha1rnds4p(abcd, add(work, [K[1]; 4])),
        2 => sha1rnds4m(abcd, add(work, [K[2]; 4])),
        3 => sha1rnds4p(abcd, add(work, [K[3]; 4])),
        _ => unreachable!("unknown icosaround index"),
    }
}
fn sha1rnds4c(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => {
            $c ^ ($a & ($b ^ $c))
        };
    }
    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_202!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);
    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_202!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);
    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_202!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);
    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_202!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);
    [b, c, d, e]
}
fn sha1rnds4p(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;
    macro_rules! bool3ary_150 {
        ($a:expr, $b:expr, $c:expr) => {
            $a ^ $b ^ $c
        };
    }
    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_150!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);
    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_150!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);
    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_150!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);
    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_150!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);
    [b, c, d, e]
}
fn sha1rnds4m(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) ^ ($a & $c) ^ ($b & $c)
        };
    }
    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_232!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);
    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_232!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);
    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_232!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);
    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_232!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);
    [b, c, d, e]
}
macro_rules! rounds4 {
    ($h0:ident, $h1:ident, $wk:expr, $i:expr) => {
        sha1_digest_round_x4($h0, sha1_first_half($h1, $wk), $i)
    };
}
macro_rules! schedule {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        sha1msg2(xor(sha1msg1($v0, $v1), $v2), $v3)
    };
}
macro_rules! schedule_rounds4 {
    (
        $h0:ident, $h1:ident, $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr, $i:expr
    ) => {
        $w4 = schedule!($w0, $w1, $w2, $w3); $h1 = rounds4!($h0, $h1, $w4, $i);
    };
}
#[inline(always)]
fn sha1_digest_block_u32(state: &mut [u32; 5], block: &[u32; 16]) {
    let mut w0 = [block[0], block[1], block[2], block[3]];
    let mut w1 = [block[4], block[5], block[6], block[7]];
    let mut w2 = [block[8], block[9], block[10], block[11]];
    let mut w3 = [block[12], block[13], block[14], block[15]];
    #[allow(clippy::needless_late_init)]
    let mut w4;
    let mut h0 = [state[0], state[1], state[2], state[3]];
    let mut h1 = sha1_first_add(state[4], w0);
    h1 = sha1_digest_round_x4(h0, h1, 0);
    h0 = rounds4!(h1, h0, w1, 0);
    h1 = rounds4!(h0, h1, w2, 0);
    h0 = rounds4!(h1, h0, w3, 0);
    schedule_rounds4!(h0, h1, w0, w1, w2, w3, w4, 0);
    schedule_rounds4!(h1, h0, w1, w2, w3, w4, w0, 1);
    schedule_rounds4!(h0, h1, w2, w3, w4, w0, w1, 1);
    schedule_rounds4!(h1, h0, w3, w4, w0, w1, w2, 1);
    schedule_rounds4!(h0, h1, w4, w0, w1, w2, w3, 1);
    schedule_rounds4!(h1, h0, w0, w1, w2, w3, w4, 1);
    schedule_rounds4!(h0, h1, w1, w2, w3, w4, w0, 2);
    schedule_rounds4!(h1, h0, w2, w3, w4, w0, w1, 2);
    schedule_rounds4!(h0, h1, w3, w4, w0, w1, w2, 2);
    schedule_rounds4!(h1, h0, w4, w0, w1, w2, w3, 2);
    schedule_rounds4!(h0, h1, w0, w1, w2, w3, w4, 2);
    schedule_rounds4!(h1, h0, w1, w2, w3, w4, w0, 3);
    schedule_rounds4!(h0, h1, w2, w3, w4, w0, w1, 3);
    schedule_rounds4!(h1, h0, w3, w4, w0, w1, w2, 3);
    schedule_rounds4!(h0, h1, w4, w0, w1, w2, w3, 3);
    schedule_rounds4!(h1, h0, w0, w1, w2, w3, w4, 3);
    let e = h1[0].rotate_left(30);
    let [a, b, c, d] = h0;
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}
pub fn compress(state: &mut [u32; 5], blocks: &[[u8; BLOCK_SIZE]]) {
    let mut block_u32 = [0u32; BLOCK_SIZE / 4];
    let mut state_cpy = *state;
    for block in blocks.iter() {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(4)) {
            *o = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        sha1_digest_block_u32(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_llm_16_10_llm_16_10 {
    use super::*;
    use crate::*;
    #[test]
    fn test_sha1_digest_round_x4() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_sha1_digest_round_x4 = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xdeadbeef;
        let rug_fuzz_5 = 0xcafebabe;
        let rug_fuzz_6 = 0x8badf00d;
        let rug_fuzz_7 = 0x0badc0de;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 3;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 4;
        let abcd = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let work = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let output_0 = sha1_digest_round_x4(abcd, work, rug_fuzz_8);
        let expected_0 = sha1rnds4c(abcd, add(work, [K[rug_fuzz_9]; 4]));
        debug_assert_eq!(output_0, expected_0, "sha1_digest_round_x4 failed for i = 0");
        let output_1 = sha1_digest_round_x4(abcd, work, rug_fuzz_10);
        let expected_1 = sha1rnds4p(abcd, add(work, [K[rug_fuzz_11]; 4]));
        debug_assert_eq!(output_1, expected_1, "sha1_digest_round_x4 failed for i = 1");
        let output_2 = sha1_digest_round_x4(abcd, work, rug_fuzz_12);
        let expected_2 = sha1rnds4m(abcd, add(work, [K[rug_fuzz_13]; 4]));
        debug_assert_eq!(output_2, expected_2, "sha1_digest_round_x4 failed for i = 2");
        let output_3 = sha1_digest_round_x4(abcd, work, rug_fuzz_14);
        let expected_3 = sha1rnds4p(abcd, add(work, [K[rug_fuzz_15]; 4]));
        debug_assert_eq!(output_3, expected_3, "sha1_digest_round_x4 failed for i = 3");
        let result = std::panic::catch_unwind(|| {
            sha1_digest_round_x4(abcd, work, rug_fuzz_16);
        });
        debug_assert!(
            result.is_err(), "sha1_digest_round_x4 should panic for invalid i"
        );
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_sha1_digest_round_x4 = 0;
    }
}
#[cfg(test)]
mod tests_rug_164 {
    use super::*;
    #[test]
    fn test_add() {
        let _rug_st_tests_rug_164_rrrruuuugggg_test_add = 0;
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
        let result = crate::compress::soft::add(p0, p1);
        debug_assert_eq!(result, [6, 8, 10, 12]);
        let _rug_ed_tests_rug_164_rrrruuuugggg_test_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_165 {
    use super::*;
    #[test]
    fn test_xor() {
        let _rug_st_tests_rug_165_rrrruuuugggg_test_xor = 0;
        let rug_fuzz_0 = 0x0f0f0f0f;
        let rug_fuzz_1 = 0x33333333;
        let rug_fuzz_2 = 0x55555555;
        let rug_fuzz_3 = 0xaaaaaaaa;
        let rug_fuzz_4 = 0xf0f0f0f0;
        let rug_fuzz_5 = 0xcccccccc;
        let rug_fuzz_6 = 0xffffffff;
        let rug_fuzz_7 = 0x55555555;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        debug_assert_eq!(
            xor(p0, p1), [p0[0] ^ p1[0], p0[1] ^ p1[1], p0[2] ^ p1[2], p0[3] ^ p1[3]]
        );
        let _rug_ed_tests_rug_165_rrrruuuugggg_test_xor = 0;
    }
}
#[cfg(test)]
mod tests_rug_166 {
    use crate::compress::soft::sha1_first_add;
    #[test]
    fn test_sha1_first_add() {
        let _rug_st_tests_rug_166_rrrruuuugggg_test_sha1_first_add = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xC3D2E1F0;
        let mut p0: u32 = rug_fuzz_0;
        let mut p1: [u32; 4] = [rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let result = sha1_first_add(p0, p1);
        debug_assert_eq!(result, [p0.wrapping_add(p1[0]), p1[1], p1[2], p1[3]]);
        let _rug_ed_tests_rug_166_rrrruuuugggg_test_sha1_first_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_167 {
    use super::*;
    #[test]
    fn test_sha1msg1() {
        let _rug_st_tests_rug_167_rrrruuuugggg_test_sha1msg1 = 0;
        let rug_fuzz_0 = 0x11111111;
        let rug_fuzz_1 = 0x22222222;
        let rug_fuzz_2 = 0x33333333;
        let rug_fuzz_3 = 0x44444444;
        let rug_fuzz_4 = 0x55555555;
        let rug_fuzz_5 = 0x66666666;
        let rug_fuzz_6 = 0x77777777;
        let rug_fuzz_7 = 0x88888888;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::compress::soft::sha1msg1(p0, p1);
        debug_assert_eq!(
            result, [p0[0] ^ p1[2], p0[1] ^ p1[3], p0[2] ^ p1[0], p0[3] ^ p1[1]]
        );
        let _rug_ed_tests_rug_167_rrrruuuugggg_test_sha1msg1 = 0;
    }
}
#[cfg(test)]
mod tests_rug_168 {
    use super::*;
    #[test]
    fn test_sha1msg2() {
        let _rug_st_tests_rug_168_rrrruuuugggg_test_sha1msg2 = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xC3D2E1F0;
        let rug_fuzz_5 = 0x18513773;
        let rug_fuzz_6 = 0x36AEF2B9;
        let rug_fuzz_7 = 0x6A296D5F;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let result = crate::compress::soft::sha1msg2(p0, p1);
        debug_assert_eq!(result, [0xD6BF77C2, 0xDAE5F606, 0xECBFAAFA, 0xB606A799]);
        let _rug_ed_tests_rug_168_rrrruuuugggg_test_sha1msg2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_169 {
    use crate::compress::soft::sha1_first_half;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_169_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xD76AA478;
        let rug_fuzz_5 = 0xE8C7B756;
        let rug_fuzz_6 = 0x242070DB;
        let rug_fuzz_7 = 0xC1BDCEEE;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        sha1_first_half(p0, p1);
        let _rug_ed_tests_rug_169_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_170 {
    use super::*;
    #[test]
    fn test_sha1rnds4c() {
        let _rug_st_tests_rug_170_rrrruuuugggg_test_sha1rnds4c = 0;
        let rug_fuzz_0 = 1732584193;
        let rug_fuzz_1 = 4023233417;
        let rug_fuzz_2 = 2562383102;
        let rug_fuzz_3 = 271733878;
        let rug_fuzz_4 = 123456789;
        let rug_fuzz_5 = 987654321;
        let rug_fuzz_6 = 192837465;
        let rug_fuzz_7 = 564738291;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        let _ = crate::compress::soft::sha1rnds4c(p0, p1);
        let _rug_ed_tests_rug_170_rrrruuuugggg_test_sha1rnds4c = 0;
    }
}
#[cfg(test)]
mod tests_rug_171 {
    use super::*;
    #[test]
    fn test_sha1rnds4p() {
        let _rug_st_tests_rug_171_rrrruuuugggg_test_sha1rnds4p = 0;
        let rug_fuzz_0 = 1732584193;
        let rug_fuzz_1 = 4023233417;
        let rug_fuzz_2 = 2562383102;
        let rug_fuzz_3 = 271733878;
        let rug_fuzz_4 = 271733878;
        let rug_fuzz_5 = 3285377520;
        let rug_fuzz_6 = 977477912;
        let rug_fuzz_7 = 1009589776;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        crate::compress::soft::sha1rnds4p(p0, p1);
        let _rug_ed_tests_rug_171_rrrruuuugggg_test_sha1rnds4p = 0;
    }
}
#[cfg(test)]
mod tests_rug_172 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_172_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1732584193;
        let rug_fuzz_1 = 4023233417;
        let rug_fuzz_2 = 2562383102;
        let rug_fuzz_3 = 271733878;
        let rug_fuzz_4 = 271733879;
        let rug_fuzz_5 = 3285377520;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 12345;
        let mut p0: [u32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: [u32; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        crate::compress::soft::sha1rnds4m(p0, p1);
        let _rug_ed_tests_rug_172_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_173 {
    use super::*;
    #[test]
    fn test_sha1_digest_block_u32() {
        let _rug_st_tests_rug_173_rrrruuuugggg_test_sha1_digest_block_u32 = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xC3D2E1F0;
        let rug_fuzz_5 = 0x5A827999;
        let rug_fuzz_6 = 0x6ED9EBA1;
        let rug_fuzz_7 = 0x8F1BBCDC;
        let rug_fuzz_8 = 0xCA62C1D6;
        let rug_fuzz_9 = 0xB8E1B814;
        let rug_fuzz_10 = 0xC69B3F93;
        let rug_fuzz_11 = 0xE473A353;
        let rug_fuzz_12 = 0xF2967C3F;
        let rug_fuzz_13 = 0xA4C663D9;
        let rug_fuzz_14 = 0xB6D996AB;
        let rug_fuzz_15 = 0xC7EDAF2F;
        let rug_fuzz_16 = 0xC9C1099F;
        let rug_fuzz_17 = 0xDA82238B;
        let rug_fuzz_18 = 0xE8B58FAF;
        let rug_fuzz_19 = 0xF9A64236;
        let rug_fuzz_20 = 0xFA799962;
        let mut p0: [u32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let mut p1: [u32; 16] = [
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
        ];
        crate::compress::soft::sha1_digest_block_u32(&mut p0, &p1);
        let _rug_ed_tests_rug_173_rrrruuuugggg_test_sha1_digest_block_u32 = 0;
    }
}
