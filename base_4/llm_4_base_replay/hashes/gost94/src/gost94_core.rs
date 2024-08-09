#![allow(clippy::many_single_char_names)]
use core::{convert::TryInto, fmt};
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block as TBlock, BlockSizeUser, Buffer, BufferKindUser,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U32},
    HashMarker, Output,
};
use crate::params::{Block, Gost94Params, SBox};
const C: Block = [
    0x00,
    0xff,
    0x00,
    0xff,
    0x00,
    0xff,
    0x00,
    0xff,
    0xff,
    0x00,
    0xff,
    0x00,
    0xff,
    0x00,
    0xff,
    0x00,
    0x00,
    0xff,
    0xff,
    0x00,
    0xff,
    0x00,
    0x00,
    0xff,
    0xff,
    0x00,
    0x00,
    0x00,
    0xff,
    0xff,
    0x00,
    0xff,
];
fn sbox(a: u32, s: &SBox) -> u32 {
    let mut v = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        let shft = 4 * i;
        let k = ((a & (0b1111u32 << shft)) >> shft) as usize;
        v += u32::from(s[i][k]) << shft;
    }
    v
}
fn g(a: u32, k: u32, s: &SBox) -> u32 {
    sbox(a.wrapping_add(k), s).rotate_left(11)
}
#[allow(clippy::needless_range_loop)]
fn encrypt(msg: &mut [u8], key: Block, sbox: &SBox) {
    let mut k = [0u32; 8];
    let mut a = u32::from_le_bytes(msg[0..4].try_into().unwrap());
    let mut b = u32::from_le_bytes(msg[4..8].try_into().unwrap());
    for (o, chunk) in k.iter_mut().zip(key.chunks_exact(4)) {
        *o = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for _ in 0..3 {
        for i in 0..8 {
            let t = b ^ g(a, k[i], sbox);
            b = a;
            a = t;
        }
    }
    for i in (0..8).rev() {
        let t = b ^ g(a, k[i], sbox);
        b = a;
        a = t;
    }
    msg[0..4].copy_from_slice(&b.to_le_bytes());
    msg[4..8].copy_from_slice(&a.to_le_bytes());
}
fn x(a: &Block, b: &Block) -> Block {
    let mut out = Block::default();
    for i in 0..32 {
        out[i] = a[i] ^ b[i];
    }
    out
}
fn x_mut(a: &mut Block, b: &Block) {
    for i in 0..32 {
        a[i] ^= b[i];
    }
}
fn a(x: Block) -> Block {
    let mut out = Block::default();
    out[..24].clone_from_slice(&x[8..]);
    for i in 0..8 {
        out[24 + i] = x[i] ^ x[i + 8];
    }
    out
}
fn p(y: Block) -> Block {
    let mut out = Block::default();
    for i in 0..4 {
        for k in 0..8 {
            out[i + 4 * k] = y[8 * i + k];
        }
    }
    out
}
fn psi(block: &mut Block) {
    let mut out = Block::default();
    out[..30].copy_from_slice(&block[2..]);
    out[30..].copy_from_slice(&block[..2]);
    out[30] ^= block[2];
    out[31] ^= block[3];
    out[30] ^= block[4];
    out[31] ^= block[5];
    out[30] ^= block[6];
    out[31] ^= block[7];
    out[30] ^= block[24];
    out[31] ^= block[25];
    out[30] ^= block[30];
    out[31] ^= block[31];
    block.copy_from_slice(&out);
}
#[inline(always)]
fn adc(a: &mut u64, b: u64, carry: &mut u64) {
    let ret = (*a as u128) + (b as u128) + (*carry as u128);
    *a = ret as u64;
    *carry = (ret >> 64) as u64;
}
/// Core GOST94 algorithm generic over parameters.
#[derive(Clone)]
pub struct Gost94Core<P: Gost94Params> {
    h: Block,
    n: [u64; 4],
    sigma: [u64; 4],
    _m: core::marker::PhantomData<P>,
}
impl<P: Gost94Params> Gost94Core<P> {
    fn shuffle(&mut self, m: &Block, s: &Block) {
        let mut res = Block::default();
        res.copy_from_slice(s);
        for _ in 0..12 {
            psi(&mut res);
        }
        x_mut(&mut res, m);
        psi(&mut res);
        x_mut(&mut self.h, &res);
        for _ in 0..61 {
            psi(&mut self.h);
        }
    }
    fn f(&mut self, m: &Block) {
        let mut s = Block::default();
        s.copy_from_slice(&self.h);
        let k = p(x(&self.h, m));
        encrypt(&mut s[0..8], k, &P::S_BOX);
        let u = a(self.h);
        let v = a(a(*m));
        let k = p(x(&u, &v));
        encrypt(&mut s[8..16], k, &P::S_BOX);
        let mut u = a(u);
        x_mut(&mut u, &C);
        let v = a(a(v));
        let k = p(x(&u, &v));
        encrypt(&mut s[16..24], k, &P::S_BOX);
        let u = a(u);
        let v = a(a(v));
        let k = p(x(&u, &v));
        encrypt(&mut s[24..32], k, &P::S_BOX);
        self.shuffle(m, &s);
    }
    fn update_sigma(&mut self, m: &Block) {
        let mut carry = 0;
        for (a, chunk) in self.sigma.iter_mut().zip(m.chunks_exact(8)) {
            let b = u64::from_le_bytes(chunk.try_into().unwrap());
            adc(a, b, &mut carry);
        }
    }
    fn update_n(&mut self, len: usize) {
        let mut carry = 0;
        adc(&mut self.n[0], (len as u64) << 3, &mut carry);
        adc(&mut self.n[1], (len as u64) >> 61, &mut carry);
        adc(&mut self.n[2], 0, &mut carry);
        adc(&mut self.n[3], 0, &mut carry);
    }
    #[inline(always)]
    fn compress(&mut self, block: &[u8; 32]) {
        self.f(block);
        self.update_sigma(block);
    }
}
impl<P: Gost94Params> HashMarker for Gost94Core<P> {}
impl<P: Gost94Params> BlockSizeUser for Gost94Core<P> {
    type BlockSize = U32;
}
impl<P: Gost94Params> BufferKindUser for Gost94Core<P> {
    type BufferKind = Eager;
}
impl<P: Gost94Params> OutputSizeUser for Gost94Core<P> {
    type OutputSize = U32;
}
impl<P: Gost94Params> UpdateCore for Gost94Core<P> {
    #[inline]
    fn update_blocks(&mut self, blocks: &[TBlock<Self>]) {
        let len = Self::BlockSize::USIZE * blocks.len();
        self.update_n(len);
        blocks.iter().for_each(|b| self.compress(b.as_ref()));
    }
}
impl<P: Gost94Params> FixedOutputCore for Gost94Core<P> {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        if buffer.get_pos() != 0 {
            self.update_n(buffer.get_pos());
            self.compress(buffer.pad_with_zeros().as_ref());
        }
        let mut buf = Block::default();
        for (o, v) in buf.chunks_exact_mut(8).zip(self.n.iter()) {
            o.copy_from_slice(&v.to_le_bytes());
        }
        self.f(&buf);
        for (o, v) in buf.chunks_exact_mut(8).zip(self.sigma.iter()) {
            o.copy_from_slice(&v.to_le_bytes());
        }
        self.f(&buf);
        out.copy_from_slice(&self.h);
    }
}
impl<P: Gost94Params> Default for Gost94Core<P> {
    #[inline]
    fn default() -> Self {
        Self {
            h: P::H0,
            n: Default::default(),
            sigma: Default::default(),
            _m: Default::default(),
        }
    }
}
impl<P: Gost94Params> Reset for Gost94Core<P> {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl<P: Gost94Params> AlgorithmName for Gost94Core<P> {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(P::NAME)
    }
}
impl<P: Gost94Params> fmt::Debug for Gost94Core<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(P::NAME)?;
        f.write_str("Core { .. }")
    }
}
#[cfg(test)]
mod tests_llm_16_1_llm_16_1 {
    use super::*;
    use crate::*;
    use gost94_core::Gost94Core;
    use params::CryptoProParam;
    #[test]
    fn test_default_gost94_core_with_cryptopro_param() {
        let _rug_st_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_default_gost94_core_with_cryptopro_param = 0;
        let core: Gost94Core<CryptoProParam> = Gost94Core::default();
        debug_assert_eq!(core.h, CryptoProParam::H0);
        debug_assert_eq!(core.n, [0; 4]);
        debug_assert_eq!(core.sigma, [0; 4]);
        let _rug_ed_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_default_gost94_core_with_cryptopro_param = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::gost94_core::Gost94Core;
    use crate::params::CryptoProParam;
    use crate::params::Gost94Params;
    use digest::Digest;
    use hex_literal::hex;
    #[test]
    fn compress_functionality() {
        let mut core = Gost94Core::<CryptoProParam>::default();
        let initial_h = core.h;
        let initial_sigma = core.sigma;
        let block = [0u8; 32];
        core.compress(&block);
        assert_ne!(core.h, initial_h, "compress should change core.h");
        assert_ne!(core.sigma, initial_sigma, "compress should change core.sigma");
    }
    #[test]
    fn compress_expected_output() {
        let mut core = Gost94Core::<CryptoProParam>::default();
        let block = hex!(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );
        let expected_h = core.h;
        let expected_sigma = core.sigma;
        core.compress(&block);
        assert_eq!(core.h, expected_h, "h after compress does not match expected");
        assert_eq!(
            core.sigma, expected_sigma, "sigma after compress does not match expected"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use crate::params::CryptoProParam;
    use crate::gost94_core::Gost94Core;
    use digest::Digest;
    use digest::generic_array::typenum::U32;
    use digest::generic_array::GenericArray;
    #[test]
    fn f_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Gost94Core::<CryptoProParam>::default();
        let m = [rug_fuzz_0; 32];
        let h0 = CryptoProParam::H0;
        core.h = m;
        core.f(&m);
        debug_assert_eq!(
            core.h, h0,
            "The `f` function should leave the `h` field unchanged for the zero input"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use crate::gost94_core::Gost94Core;
    use crate::params::CryptoProParam;
    use crate::params::Gost94Params;
    type TestHash = Gost94Core<CryptoProParam>;
    #[test]
    fn shuffle_test() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_shuffle_test = 0;
        let rug_fuzz_0 = 0x00;
        let rug_fuzz_1 = 0x01;
        let rug_fuzz_2 = 0x02;
        let rug_fuzz_3 = 0x03;
        let rug_fuzz_4 = 0x04;
        let rug_fuzz_5 = 0x05;
        let rug_fuzz_6 = 0x06;
        let rug_fuzz_7 = 0x07;
        let rug_fuzz_8 = 0x08;
        let rug_fuzz_9 = 0x09;
        let rug_fuzz_10 = 0x0A;
        let rug_fuzz_11 = 0x0B;
        let rug_fuzz_12 = 0x0C;
        let rug_fuzz_13 = 0x0D;
        let rug_fuzz_14 = 0x0E;
        let rug_fuzz_15 = 0x0F;
        let rug_fuzz_16 = 0x10;
        let rug_fuzz_17 = 0x11;
        let rug_fuzz_18 = 0x12;
        let rug_fuzz_19 = 0x13;
        let rug_fuzz_20 = 0x14;
        let rug_fuzz_21 = 0x15;
        let rug_fuzz_22 = 0x16;
        let rug_fuzz_23 = 0x17;
        let rug_fuzz_24 = 0x18;
        let rug_fuzz_25 = 0x19;
        let rug_fuzz_26 = 0x1A;
        let rug_fuzz_27 = 0x1B;
        let rug_fuzz_28 = 0x1C;
        let rug_fuzz_29 = 0x1D;
        let rug_fuzz_30 = 0x1E;
        let rug_fuzz_31 = 0x1F;
        let rug_fuzz_32 = 0x1F;
        let rug_fuzz_33 = 0x1E;
        let rug_fuzz_34 = 0x1D;
        let rug_fuzz_35 = 0x1C;
        let rug_fuzz_36 = 0x1B;
        let rug_fuzz_37 = 0x1A;
        let rug_fuzz_38 = 0x19;
        let rug_fuzz_39 = 0x18;
        let rug_fuzz_40 = 0x17;
        let rug_fuzz_41 = 0x16;
        let rug_fuzz_42 = 0x15;
        let rug_fuzz_43 = 0x14;
        let rug_fuzz_44 = 0x13;
        let rug_fuzz_45 = 0x12;
        let rug_fuzz_46 = 0x11;
        let rug_fuzz_47 = 0x10;
        let rug_fuzz_48 = 0x0F;
        let rug_fuzz_49 = 0x0E;
        let rug_fuzz_50 = 0x0D;
        let rug_fuzz_51 = 0x0C;
        let rug_fuzz_52 = 0x0B;
        let rug_fuzz_53 = 0x0A;
        let rug_fuzz_54 = 0x09;
        let rug_fuzz_55 = 0x08;
        let rug_fuzz_56 = 0x07;
        let rug_fuzz_57 = 0x06;
        let rug_fuzz_58 = 0x05;
        let rug_fuzz_59 = 0x04;
        let rug_fuzz_60 = 0x03;
        let rug_fuzz_61 = 0x02;
        let rug_fuzz_62 = 0x01;
        let rug_fuzz_63 = 0x00;
        let mut hasher = TestHash::default();
        let m = [
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
        ];
        let s = [
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
        ];
        let initial_h = hasher.h;
        hasher.shuffle(&m, &s);
        debug_assert_ne!(initial_h, hasher.h, "Shuffle should change inner state h");
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_shuffle_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use crate::gost94_core::Gost94Core;
    use crate::params::CryptoProParam;
    use crate::params::Gost94Params;
    #[test]
    fn test_update_n() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, u64, usize, usize, usize, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher: Gost94Core<CryptoProParam> = Default::default();
        let initial_n = hasher.n;
        hasher.update_n(rug_fuzz_0);
        debug_assert_eq!(
            hasher.n, initial_n, "update_n with zero length should not change anything"
        );
        hasher.update_n(rug_fuzz_1);
        let mut expected_n = initial_n;
        expected_n[rug_fuzz_2] += rug_fuzz_3;
        debug_assert_eq!(
            hasher.n, expected_n,
            "update_n with length 1 should increase the first element of `n` by 8"
        );
        hasher.update_n(usize::MAX);
        debug_assert!(
            hasher.n[rug_fuzz_4] < expected_n[rug_fuzz_5],
            "update_n with large input should overflow and carry to the next element"
        );
        debug_assert!(
            hasher.n[rug_fuzz_6] > rug_fuzz_7,
            "update_n with large input should carry to the next element of `n`"
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use params::CryptoProParam;
    #[test]
    fn update_sigma_test() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_update_sigma_test = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 0x01;
        let rug_fuzz_3 = 0x01;
        let rug_fuzz_4 = 0x01;
        let rug_fuzz_5 = 0x01;
        let rug_fuzz_6 = 0x01;
        let rug_fuzz_7 = 0x01;
        let rug_fuzz_8 = 0x01;
        let rug_fuzz_9 = 0x01;
        let rug_fuzz_10 = 0x02;
        let rug_fuzz_11 = 0x02;
        let rug_fuzz_12 = 0x02;
        let rug_fuzz_13 = 0x02;
        let rug_fuzz_14 = 0x02;
        let rug_fuzz_15 = 0x02;
        let rug_fuzz_16 = 0x02;
        let rug_fuzz_17 = 0x02;
        let rug_fuzz_18 = 0x03;
        let rug_fuzz_19 = 0x03;
        let rug_fuzz_20 = 0x03;
        let rug_fuzz_21 = 0x03;
        let rug_fuzz_22 = 0x03;
        let rug_fuzz_23 = 0x03;
        let rug_fuzz_24 = 0x03;
        let rug_fuzz_25 = 0x03;
        let rug_fuzz_26 = 0x04;
        let rug_fuzz_27 = 0x04;
        let rug_fuzz_28 = 0x04;
        let rug_fuzz_29 = 0x04;
        let rug_fuzz_30 = 0x04;
        let rug_fuzz_31 = 0x04;
        let rug_fuzz_32 = 0x04;
        let rug_fuzz_33 = 0x04;
        let mut core = Gost94Core::<CryptoProParam>::default();
        let m = [rug_fuzz_0; 32];
        debug_assert_eq!(core.sigma, [0; 4]);
        core.update_sigma(&m);
        debug_assert_eq!(core.sigma, [0; 4]);
        let m = [rug_fuzz_1; 32];
        core.update_sigma(&m);
        debug_assert_eq!(core.sigma, [0x0101010101010101; 4]);
        let m = [
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
        ];
        core.update_sigma(&m);
        debug_assert_eq!(
            core.sigma, [0x0101010101010101 + 0x0101010101010101, 0x0101010101010101 +
            0x0202020202020202, 0x0101010101010101 + 0x0303030303030303,
            0x0101010101010101 + 0x0404040404040404,]
        );
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_update_sigma_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_17_llm_16_17 {
    use super::*;
    use crate::*;
    #[test]
    fn test_sbox() {
        let _rug_st_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_sbox = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 9;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 13;
        let rug_fuzz_5 = 8;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 14;
        let rug_fuzz_8 = 6;
        let rug_fuzz_9 = 11;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 7;
        let rug_fuzz_13 = 15;
        let rug_fuzz_14 = 5;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 14;
        let rug_fuzz_17 = 11;
        let rug_fuzz_18 = 4;
        let rug_fuzz_19 = 12;
        let rug_fuzz_20 = 6;
        let rug_fuzz_21 = 13;
        let rug_fuzz_22 = 15;
        let rug_fuzz_23 = 10;
        let rug_fuzz_24 = 2;
        let rug_fuzz_25 = 3;
        let rug_fuzz_26 = 8;
        let rug_fuzz_27 = 1;
        let rug_fuzz_28 = 0;
        let rug_fuzz_29 = 7;
        let rug_fuzz_30 = 5;
        let rug_fuzz_31 = 9;
        let rug_fuzz_32 = 5;
        let rug_fuzz_33 = 8;
        let rug_fuzz_34 = 1;
        let rug_fuzz_35 = 13;
        let rug_fuzz_36 = 10;
        let rug_fuzz_37 = 3;
        let rug_fuzz_38 = 4;
        let rug_fuzz_39 = 2;
        let rug_fuzz_40 = 14;
        let rug_fuzz_41 = 15;
        let rug_fuzz_42 = 12;
        let rug_fuzz_43 = 7;
        let rug_fuzz_44 = 6;
        let rug_fuzz_45 = 0;
        let rug_fuzz_46 = 9;
        let rug_fuzz_47 = 11;
        let rug_fuzz_48 = 7;
        let rug_fuzz_49 = 13;
        let rug_fuzz_50 = 10;
        let rug_fuzz_51 = 1;
        let rug_fuzz_52 = 0;
        let rug_fuzz_53 = 8;
        let rug_fuzz_54 = 9;
        let rug_fuzz_55 = 15;
        let rug_fuzz_56 = 14;
        let rug_fuzz_57 = 4;
        let rug_fuzz_58 = 6;
        let rug_fuzz_59 = 12;
        let rug_fuzz_60 = 11;
        let rug_fuzz_61 = 2;
        let rug_fuzz_62 = 5;
        let rug_fuzz_63 = 3;
        let rug_fuzz_64 = 6;
        let rug_fuzz_65 = 12;
        let rug_fuzz_66 = 7;
        let rug_fuzz_67 = 1;
        let rug_fuzz_68 = 5;
        let rug_fuzz_69 = 15;
        let rug_fuzz_70 = 13;
        let rug_fuzz_71 = 8;
        let rug_fuzz_72 = 4;
        let rug_fuzz_73 = 10;
        let rug_fuzz_74 = 9;
        let rug_fuzz_75 = 14;
        let rug_fuzz_76 = 0;
        let rug_fuzz_77 = 3;
        let rug_fuzz_78 = 11;
        let rug_fuzz_79 = 2;
        let rug_fuzz_80 = 4;
        let rug_fuzz_81 = 11;
        let rug_fuzz_82 = 10;
        let rug_fuzz_83 = 0;
        let rug_fuzz_84 = 7;
        let rug_fuzz_85 = 2;
        let rug_fuzz_86 = 1;
        let rug_fuzz_87 = 13;
        let rug_fuzz_88 = 3;
        let rug_fuzz_89 = 6;
        let rug_fuzz_90 = 8;
        let rug_fuzz_91 = 5;
        let rug_fuzz_92 = 9;
        let rug_fuzz_93 = 12;
        let rug_fuzz_94 = 15;
        let rug_fuzz_95 = 14;
        let rug_fuzz_96 = 13;
        let rug_fuzz_97 = 11;
        let rug_fuzz_98 = 4;
        let rug_fuzz_99 = 1;
        let rug_fuzz_100 = 3;
        let rug_fuzz_101 = 15;
        let rug_fuzz_102 = 5;
        let rug_fuzz_103 = 9;
        let rug_fuzz_104 = 0;
        let rug_fuzz_105 = 10;
        let rug_fuzz_106 = 14;
        let rug_fuzz_107 = 7;
        let rug_fuzz_108 = 6;
        let rug_fuzz_109 = 8;
        let rug_fuzz_110 = 2;
        let rug_fuzz_111 = 12;
        let rug_fuzz_112 = 1;
        let rug_fuzz_113 = 15;
        let rug_fuzz_114 = 13;
        let rug_fuzz_115 = 0;
        let rug_fuzz_116 = 5;
        let rug_fuzz_117 = 7;
        let rug_fuzz_118 = 10;
        let rug_fuzz_119 = 4;
        let rug_fuzz_120 = 9;
        let rug_fuzz_121 = 2;
        let rug_fuzz_122 = 3;
        let rug_fuzz_123 = 14;
        let rug_fuzz_124 = 6;
        let rug_fuzz_125 = 11;
        let rug_fuzz_126 = 8;
        let rug_fuzz_127 = 12;
        let rug_fuzz_128 = 0x00000000;
        let rug_fuzz_129 = 0x00000000;
        let rug_fuzz_130 = 0x00000001;
        let rug_fuzz_131 = 0x04081005;
        let rug_fuzz_132 = 0x00000010;
        let rug_fuzz_133 = 0x04081050;
        let rug_fuzz_134 = 0x00000100;
        let rug_fuzz_135 = 0x04081500;
        let rug_fuzz_136 = 0x00001000;
        let rug_fuzz_137 = 0x040b0000;
        let rug_fuzz_138 = 0x00010000;
        let rug_fuzz_139 = 0x0c000000;
        let rug_fuzz_140 = 0x00100000;
        let rug_fuzz_141 = 0x40000000;
        let rug_fuzz_142 = 0x01000000;
        let rug_fuzz_143 = 0x05000000;
        let rug_fuzz_144 = 0x10000000;
        let rug_fuzz_145 = 0x01000000;
        let rug_fuzz_146 = 0x11111111;
        let rug_fuzz_147 = 0x4c3b7b91;
        let rug_fuzz_148 = 0xffffffff;
        let rug_fuzz_149 = 0x1adb9ef9;
        let test_sbox = [
            [
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
            ],
            [
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
            ],
            [
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
            ],
            [
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
            ],
            [
                rug_fuzz_64,
                rug_fuzz_65,
                rug_fuzz_66,
                rug_fuzz_67,
                rug_fuzz_68,
                rug_fuzz_69,
                rug_fuzz_70,
                rug_fuzz_71,
                rug_fuzz_72,
                rug_fuzz_73,
                rug_fuzz_74,
                rug_fuzz_75,
                rug_fuzz_76,
                rug_fuzz_77,
                rug_fuzz_78,
                rug_fuzz_79,
            ],
            [
                rug_fuzz_80,
                rug_fuzz_81,
                rug_fuzz_82,
                rug_fuzz_83,
                rug_fuzz_84,
                rug_fuzz_85,
                rug_fuzz_86,
                rug_fuzz_87,
                rug_fuzz_88,
                rug_fuzz_89,
                rug_fuzz_90,
                rug_fuzz_91,
                rug_fuzz_92,
                rug_fuzz_93,
                rug_fuzz_94,
                rug_fuzz_95,
            ],
            [
                rug_fuzz_96,
                rug_fuzz_97,
                rug_fuzz_98,
                rug_fuzz_99,
                rug_fuzz_100,
                rug_fuzz_101,
                rug_fuzz_102,
                rug_fuzz_103,
                rug_fuzz_104,
                rug_fuzz_105,
                rug_fuzz_106,
                rug_fuzz_107,
                rug_fuzz_108,
                rug_fuzz_109,
                rug_fuzz_110,
                rug_fuzz_111,
            ],
            [
                rug_fuzz_112,
                rug_fuzz_113,
                rug_fuzz_114,
                rug_fuzz_115,
                rug_fuzz_116,
                rug_fuzz_117,
                rug_fuzz_118,
                rug_fuzz_119,
                rug_fuzz_120,
                rug_fuzz_121,
                rug_fuzz_122,
                rug_fuzz_123,
                rug_fuzz_124,
                rug_fuzz_125,
                rug_fuzz_126,
                rug_fuzz_127,
            ],
        ];
        let test_pairs = [
            (rug_fuzz_128, rug_fuzz_129),
            (rug_fuzz_130, rug_fuzz_131),
            (rug_fuzz_132, rug_fuzz_133),
            (rug_fuzz_134, rug_fuzz_135),
            (rug_fuzz_136, rug_fuzz_137),
            (rug_fuzz_138, rug_fuzz_139),
            (rug_fuzz_140, rug_fuzz_141),
            (rug_fuzz_142, rug_fuzz_143),
            (rug_fuzz_144, rug_fuzz_145),
            (rug_fuzz_146, rug_fuzz_147),
            (rug_fuzz_148, rug_fuzz_149),
        ];
        for (input, expected) in test_pairs {
            debug_assert_eq!(sbox(input, & test_sbox), expected);
        }
        let _rug_ed_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_sbox = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;
    use crate::*;
    #[test]
    fn test_x_function() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let a = Block::from([rug_fuzz_0; 32]);
        let b = Block::from([rug_fuzz_1; 32]);
        let expected = Block::from([rug_fuzz_2; 32]);
        debug_assert_eq!(x(& a, & b), expected);
        let a = Block::from([rug_fuzz_3; 32]);
        let b = Block::from([rug_fuzz_4; 32]);
        let expected = Block::from([rug_fuzz_5; 32]);
        debug_assert_eq!(x(& a, & b), expected);
        let a = Block::from([rug_fuzz_6; 32]);
        let b = Block::from([rug_fuzz_7; 32]);
        let expected = Block::from([rug_fuzz_8; 32]);
        debug_assert_eq!(x(& a, & b), expected);
        let a = Block::from([rug_fuzz_9; 32]);
        let b = Block::from([rug_fuzz_10; 32]);
        let expected = Block::from([rug_fuzz_11; 32]);
        debug_assert_eq!(x(& a, & b), expected);
             }
}
}
}    }
}
