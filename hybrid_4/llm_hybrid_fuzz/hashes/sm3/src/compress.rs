#![allow(clippy::many_single_char_names, clippy::too_many_arguments)]
use crate::{consts::T32, Block, Sm3Core};
use core::convert::TryInto;
#[inline(always)]
fn ff1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
#[inline(always)]
fn ff2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}
#[inline(always)]
fn gg1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
#[inline(always)]
fn gg2(x: u32, y: u32, z: u32) -> u32 {
    (y ^ z) & x ^ z
}
#[inline(always)]
fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}
#[inline(always)]
fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}
#[inline(always)]
fn w1(x: &[u32; 16], i: usize) -> u32 {
    x[i & 0x0f]
}
#[inline(always)]
fn w2(x: &mut [u32; 16], i: usize) -> u32 {
    let tw = w1(x, i) ^ w1(x, i - 9) ^ w1(x, i - 3).rotate_left(15);
    let tw = p1(tw) ^ w1(x, i - 13).rotate_left(7) ^ w1(x, i - 6);
    x[i & 0x0f] = tw;
    tw
}
#[inline(always)]
fn t(i: usize) -> u32 {
    T32[i]
}
fn sm3_round1(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    t: u32,
    w1: u32,
    w2: u32,
) -> [u32; 8] {
    let ss1 = (a.rotate_left(12).wrapping_add(e).wrapping_add(t)).rotate_left(7);
    let ss2 = ss1 ^ a.rotate_left(12);
    let d = d.wrapping_add(ff1(a, b, c)).wrapping_add(ss2).wrapping_add(w1 ^ w2);
    let h = h.wrapping_add(gg1(e, f, g)).wrapping_add(ss1).wrapping_add(w1);
    let b = b.rotate_left(9);
    let f = f.rotate_left(19);
    let h = p0(h);
    [a, b, c, d, e, f, g, h]
}
fn sm3_round2(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    t: u32,
    w1: u32,
    w2: u32,
) -> [u32; 8] {
    let ss1 = (a.rotate_left(12).wrapping_add(e).wrapping_add(t)).rotate_left(7);
    let ss2 = ss1 ^ a.rotate_left(12);
    let d = d.wrapping_add(ff2(a, b, c)).wrapping_add(ss2).wrapping_add(w1 ^ w2);
    let h = h.wrapping_add(gg2(e, f, g)).wrapping_add(ss1).wrapping_add(w1);
    let b = b.rotate_left(9);
    let f = f.rotate_left(19);
    let h = p0(h);
    [a, b, c, d, e, f, g, h]
}
macro_rules! R1 {
    (
        $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident,
        $t:expr, $w1:expr, $w2:expr
    ) => {
        { let out = sm3_round1($a, $b, $c, $d, $e, $f, $g, $h, $t, $w1, $w2); $a =
        out[0]; $b = out[1]; $c = out[2]; $d = out[3]; $e = out[4]; $f = out[5]; $g =
        out[6]; $h = out[7]; }
    };
}
macro_rules! R2 {
    (
        $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident,
        $t:expr, $w1:expr, $w2:expr
    ) => {
        { let out = sm3_round2($a, $b, $c, $d, $e, $f, $g, $h, $t, $w1, $w2); $a =
        out[0]; $b = out[1]; $c = out[2]; $d = out[3]; $e = out[4]; $f = out[5]; $g =
        out[6]; $h = out[7]; }
    };
}
fn compress_u32(state: &mut [u32; 8], block: &[u32; 16]) {
    let mut x: [u32; 16] = *block;
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];
    R1!(a, b, c, d, e, f, g, h, t(0), w1(& x, 0), w1(& x, 4));
    R1!(d, a, b, c, h, e, f, g, t(1), w1(& x, 1), w1(& x, 5));
    R1!(c, d, a, b, g, h, e, f, t(2), w1(& x, 2), w1(& x, 6));
    R1!(b, c, d, a, f, g, h, e, t(3), w1(& x, 3), w1(& x, 7));
    R1!(a, b, c, d, e, f, g, h, t(4), w1(& x, 4), w1(& x, 8));
    R1!(d, a, b, c, h, e, f, g, t(5), w1(& x, 5), w1(& x, 9));
    R1!(c, d, a, b, g, h, e, f, t(6), w1(& x, 6), w1(& x, 10));
    R1!(b, c, d, a, f, g, h, e, t(7), w1(& x, 7), w1(& x, 11));
    R1!(a, b, c, d, e, f, g, h, t(8), w1(& x, 8), w1(& x, 12));
    R1!(d, a, b, c, h, e, f, g, t(9), w1(& x, 9), w1(& x, 13));
    R1!(c, d, a, b, g, h, e, f, t(10), w1(& x, 10), w1(& x, 14));
    R1!(b, c, d, a, f, g, h, e, t(11), w1(& x, 11), w1(& x, 15));
    R1!(a, b, c, d, e, f, g, h, t(12), w1(& x, 12), w2(& mut x, 16));
    R1!(d, a, b, c, h, e, f, g, t(13), w1(& x, 13), w2(& mut x, 17));
    R1!(c, d, a, b, g, h, e, f, t(14), w1(& x, 14), w2(& mut x, 18));
    R1!(b, c, d, a, f, g, h, e, t(15), w1(& x, 15), w2(& mut x, 19));
    R2!(a, b, c, d, e, f, g, h, t(16), w1(& x, 16), w2(& mut x, 20));
    R2!(d, a, b, c, h, e, f, g, t(17), w1(& x, 17), w2(& mut x, 21));
    R2!(c, d, a, b, g, h, e, f, t(18), w1(& x, 18), w2(& mut x, 22));
    R2!(b, c, d, a, f, g, h, e, t(19), w1(& x, 19), w2(& mut x, 23));
    R2!(a, b, c, d, e, f, g, h, t(20), w1(& x, 20), w2(& mut x, 24));
    R2!(d, a, b, c, h, e, f, g, t(21), w1(& x, 21), w2(& mut x, 25));
    R2!(c, d, a, b, g, h, e, f, t(22), w1(& x, 22), w2(& mut x, 26));
    R2!(b, c, d, a, f, g, h, e, t(23), w1(& x, 23), w2(& mut x, 27));
    R2!(a, b, c, d, e, f, g, h, t(24), w1(& x, 24), w2(& mut x, 28));
    R2!(d, a, b, c, h, e, f, g, t(25), w1(& x, 25), w2(& mut x, 29));
    R2!(c, d, a, b, g, h, e, f, t(26), w1(& x, 26), w2(& mut x, 30));
    R2!(b, c, d, a, f, g, h, e, t(27), w1(& x, 27), w2(& mut x, 31));
    R2!(a, b, c, d, e, f, g, h, t(28), w1(& x, 28), w2(& mut x, 32));
    R2!(d, a, b, c, h, e, f, g, t(29), w1(& x, 29), w2(& mut x, 33));
    R2!(c, d, a, b, g, h, e, f, t(30), w1(& x, 30), w2(& mut x, 34));
    R2!(b, c, d, a, f, g, h, e, t(31), w1(& x, 31), w2(& mut x, 35));
    R2!(a, b, c, d, e, f, g, h, t(32), w1(& x, 32), w2(& mut x, 36));
    R2!(d, a, b, c, h, e, f, g, t(33), w1(& x, 33), w2(& mut x, 37));
    R2!(c, d, a, b, g, h, e, f, t(34), w1(& x, 34), w2(& mut x, 38));
    R2!(b, c, d, a, f, g, h, e, t(35), w1(& x, 35), w2(& mut x, 39));
    R2!(a, b, c, d, e, f, g, h, t(36), w1(& x, 36), w2(& mut x, 40));
    R2!(d, a, b, c, h, e, f, g, t(37), w1(& x, 37), w2(& mut x, 41));
    R2!(c, d, a, b, g, h, e, f, t(38), w1(& x, 38), w2(& mut x, 42));
    R2!(b, c, d, a, f, g, h, e, t(39), w1(& x, 39), w2(& mut x, 43));
    R2!(a, b, c, d, e, f, g, h, t(40), w1(& x, 40), w2(& mut x, 44));
    R2!(d, a, b, c, h, e, f, g, t(41), w1(& x, 41), w2(& mut x, 45));
    R2!(c, d, a, b, g, h, e, f, t(42), w1(& x, 42), w2(& mut x, 46));
    R2!(b, c, d, a, f, g, h, e, t(43), w1(& x, 43), w2(& mut x, 47));
    R2!(a, b, c, d, e, f, g, h, t(44), w1(& x, 44), w2(& mut x, 48));
    R2!(d, a, b, c, h, e, f, g, t(45), w1(& x, 45), w2(& mut x, 49));
    R2!(c, d, a, b, g, h, e, f, t(46), w1(& x, 46), w2(& mut x, 50));
    R2!(b, c, d, a, f, g, h, e, t(47), w1(& x, 47), w2(& mut x, 51));
    R2!(a, b, c, d, e, f, g, h, t(48), w1(& x, 48), w2(& mut x, 52));
    R2!(d, a, b, c, h, e, f, g, t(49), w1(& x, 49), w2(& mut x, 53));
    R2!(c, d, a, b, g, h, e, f, t(50), w1(& x, 50), w2(& mut x, 54));
    R2!(b, c, d, a, f, g, h, e, t(51), w1(& x, 51), w2(& mut x, 55));
    R2!(a, b, c, d, e, f, g, h, t(52), w1(& x, 52), w2(& mut x, 56));
    R2!(d, a, b, c, h, e, f, g, t(53), w1(& x, 53), w2(& mut x, 57));
    R2!(c, d, a, b, g, h, e, f, t(54), w1(& x, 54), w2(& mut x, 58));
    R2!(b, c, d, a, f, g, h, e, t(55), w1(& x, 55), w2(& mut x, 59));
    R2!(a, b, c, d, e, f, g, h, t(56), w1(& x, 56), w2(& mut x, 60));
    R2!(d, a, b, c, h, e, f, g, t(57), w1(& x, 57), w2(& mut x, 61));
    R2!(c, d, a, b, g, h, e, f, t(58), w1(& x, 58), w2(& mut x, 62));
    R2!(b, c, d, a, f, g, h, e, t(59), w1(& x, 59), w2(& mut x, 63));
    R2!(a, b, c, d, e, f, g, h, t(60), w1(& x, 60), w2(& mut x, 64));
    R2!(d, a, b, c, h, e, f, g, t(61), w1(& x, 61), w2(& mut x, 65));
    R2!(c, d, a, b, g, h, e, f, t(62), w1(& x, 62), w2(& mut x, 66));
    R2!(b, c, d, a, f, g, h, e, t(63), w1(& x, 63), w2(& mut x, 67));
    state[0] ^= a;
    state[1] ^= b;
    state[2] ^= c;
    state[3] ^= d;
    state[4] ^= e;
    state[5] ^= f;
    state[6] ^= g;
    state[7] ^= h;
}
pub(crate) fn compress(state: &mut [u32; 8], blocks: &[Block<Sm3Core>]) {
    for block in blocks {
        let mut w = [0u32; 16];
        for (o, chunk) in w.iter_mut().zip(block.chunks_exact(4)) {
            *o = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        compress_u32(state, &w);
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use crate::compress;
    use crate::compress::Block;
    use crate::Sm3Core;
    #[test]
    fn test_compress() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_compress = 0;
        let rug_fuzz_0 = 0x7380166f;
        let rug_fuzz_1 = 0x4914b2b9;
        let rug_fuzz_2 = 0x172442d7;
        let rug_fuzz_3 = 0xda8a0600;
        let rug_fuzz_4 = 0xa96f30bc;
        let rug_fuzz_5 = 0x163138aa;
        let rug_fuzz_6 = 0xe38dee4d;
        let rug_fuzz_7 = 0xb0fb0e4e;
        let rug_fuzz_8 = 0x61;
        let rug_fuzz_9 = 0x62;
        let rug_fuzz_10 = 0x63;
        let rug_fuzz_11 = 0x64;
        let rug_fuzz_12 = 0x65;
        let rug_fuzz_13 = 0x66;
        let rug_fuzz_14 = 0x67;
        let rug_fuzz_15 = 0x68;
        let rug_fuzz_16 = 0x69;
        let rug_fuzz_17 = 0x6a;
        let rug_fuzz_18 = 0x6b;
        let rug_fuzz_19 = 0x6c;
        let rug_fuzz_20 = 0x6d;
        let rug_fuzz_21 = 0x6e;
        let rug_fuzz_22 = 0x6f;
        let rug_fuzz_23 = 0x70;
        let rug_fuzz_24 = 0x71;
        let rug_fuzz_25 = 0x72;
        let rug_fuzz_26 = 0x73;
        let rug_fuzz_27 = 0x74;
        let rug_fuzz_28 = 0x75;
        let rug_fuzz_29 = 0x76;
        let rug_fuzz_30 = 0x77;
        let rug_fuzz_31 = 0x78;
        let rug_fuzz_32 = 0x79;
        let rug_fuzz_33 = 0x7a;
        let rug_fuzz_34 = 0x30;
        let rug_fuzz_35 = 0x31;
        let rug_fuzz_36 = 0x32;
        let rug_fuzz_37 = 0x33;
        let rug_fuzz_38 = 0x34;
        let rug_fuzz_39 = 0x35;
        let rug_fuzz_40 = 0x36;
        let rug_fuzz_41 = 0x37;
        let rug_fuzz_42 = 0x38;
        let rug_fuzz_43 = 0x39;
        let rug_fuzz_44 = 0x30;
        let rug_fuzz_45 = 0x31;
        let rug_fuzz_46 = 0x32;
        let rug_fuzz_47 = 0x33;
        let rug_fuzz_48 = 0x34;
        let rug_fuzz_49 = 0x35;
        let rug_fuzz_50 = 0x36;
        let rug_fuzz_51 = 0x37;
        let rug_fuzz_52 = 0x38;
        let rug_fuzz_53 = 0x39;
        let rug_fuzz_54 = 0x30;
        let rug_fuzz_55 = 0x31;
        let rug_fuzz_56 = 0x32;
        let rug_fuzz_57 = 0x33;
        let rug_fuzz_58 = 0x34;
        let rug_fuzz_59 = 0x35;
        let rug_fuzz_60 = 0x36;
        let rug_fuzz_61 = 0x37;
        let rug_fuzz_62 = 0x38;
        let rug_fuzz_63 = 0x39;
        let rug_fuzz_64 = 0x30;
        let rug_fuzz_65 = 0x31;
        let rug_fuzz_66 = 0x32;
        let rug_fuzz_67 = 0x33;
        let rug_fuzz_68 = 0x34;
        let rug_fuzz_69 = 0x35;
        let rug_fuzz_70 = 0x36;
        let rug_fuzz_71 = 0x37;
        let rug_fuzz_72 = 0x66c7f0f4;
        let rug_fuzz_73 = 0x62eeedd9;
        let rug_fuzz_74 = 0xd1f2d46b;
        let rug_fuzz_75 = 0xdc10e4e2;
        let rug_fuzz_76 = 0x4167c487;
        let rug_fuzz_77 = 0x5cf2f7a2;
        let rug_fuzz_78 = 0x297da02b;
        let rug_fuzz_79 = 0x8f4ba8e0;
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
        let data: [u8; 64] = [
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
        ];
        let blocks = [Block::<Sm3Core>::from(data)];
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
            state, expected_state, "compress function did not result in expected state"
        );
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_compress = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    #[test]
    fn test_compress_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut state = [rug_fuzz_0; 8];
        let block = [rug_fuzz_1; 16];
        debug_assert_eq!(state, [0, 0, 0, 0, 0, 0, 0, 0]);
        compress_u32(&mut state, &block);
        debug_assert_ne!(state, [0, 0, 0, 0, 0, 0, 0, 0]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_18_llm_16_18 {
    use super::*;
    use crate::*;
    fn init_test_array() -> [u32; 16] {
        [
            0x7380166f,
            0x4914b2b9,
            0x172442d7,
            0xda8a0600,
            0xa96f30bc,
            0x163138aa,
            0xe38dee4d,
            0xb0fb0e4e,
            0x58f1fae2,
            0xf8e2d4c2,
            0x05ba1f33,
            0x2e1aa175,
            0xefe2872f,
            0x6dcb5a8f,
            0x6fb077e1,
            0x4e4a6f7c,
        ]
    }
    #[test]
    fn test_w2() {
        let mut test_array = init_test_array();
        let i = 16;
        let result_w2 = w2(&mut test_array, i);
        let array_index = i & 0x0f;
        assert_eq!(
            result_w2, test_array[array_index],
            "w2 function did not correctly update the array at the expected index."
        );
    }
}
#[cfg(test)]
mod tests_rug_295 {
    use super::*;
    #[test]
    fn test_ff1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        debug_assert_eq!(crate ::compress::ff1(p0, p1, p2), p0 ^ p1 ^ p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_296 {
    use super::*;
    #[test]
    fn test_ff2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        let result = crate::compress::ff2(p0, p1, p2);
        debug_assert_eq!(result, (p0 & p1) | (p0 & p2) | (p1 & p2));
             }
});    }
}
#[cfg(test)]
mod tests_rug_297 {
    use super::*;
    #[test]
    fn test_gg1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        debug_assert_eq!(crate ::compress::gg1(p0, p1, p2), p0 ^ p1 ^ p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_298 {
    use super::*;
    #[test]
    fn test_gg2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        debug_assert_eq!(crate ::compress::gg2(p0, p1, p2), (p1 ^ p2) & p0 ^ p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_299 {
    use super::*;
    #[test]
    fn test_p0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        debug_assert_eq!(
            crate ::compress::p0(p0), p0 ^ p0.rotate_left(9) ^ p0.rotate_left(17)
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_300 {
    use super::*;
    #[test]
    fn test_p1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u32 = rug_fuzz_0;
        debug_assert_eq!(
            crate ::compress::p1(p0), p0 ^ p0.rotate_left(15) ^ p0.rotate_left(23)
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_301 {
    use super::*;
    #[test]
    fn test_w1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u32; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
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
        ];
        let mut p1: usize = rug_fuzz_16;
        debug_assert_eq!(p0[p1], crate ::compress::w1(& p0, p1));
             }
});    }
}
#[cfg(test)]
mod tests_rug_302 {
    use super::*;
    #[test]
    fn test_t() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let result = crate::compress::t(p0);
        debug_assert_eq!(result, T32[p0]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_303 {
    use super::*;
    #[test]
    fn test_sm3_round1() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        let mut p3: u32 = rug_fuzz_3;
        let mut p4: u32 = rug_fuzz_4;
        let mut p5: u32 = rug_fuzz_5;
        let mut p6: u32 = rug_fuzz_6;
        let mut p7: u32 = rug_fuzz_7;
        let mut p8: u32 = rug_fuzz_8;
        let mut p9: u32 = rug_fuzz_9;
        let mut p10: u32 = rug_fuzz_10;
        crate::compress::sm3_round1(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10);
             }
});    }
}
#[cfg(test)]
mod tests_rug_304 {
    use super::*;
    #[test]
    fn test_sm3_round2() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut p1: u32 = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        let mut p3: u32 = rug_fuzz_3;
        let mut p4: u32 = rug_fuzz_4;
        let mut p5: u32 = rug_fuzz_5;
        let mut p6: u32 = rug_fuzz_6;
        let mut p7: u32 = rug_fuzz_7;
        let mut p8: u32 = rug_fuzz_8;
        let mut p9: u32 = rug_fuzz_9;
        let mut p10: u32 = rug_fuzz_10;
        let result = crate::compress::sm3_round2(
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
        );
             }
});    }
}
