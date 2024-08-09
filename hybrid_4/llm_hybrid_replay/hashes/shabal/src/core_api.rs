use crate::consts;
use core::{convert::TryInto, fmt, mem, num::Wrapping};
use digest::{
    block_buffer::Eager, consts::U64,
    core_api::{
        AlgorithmName, BlockSizeUser, Buffer, BufferKindUser, OutputSizeUser, TruncSide,
        UpdateCore, VariableOutputCore,
    },
    generic_array::GenericArray, HashMarker, InvalidOutputSize, Output,
};
type BlockSize = U64;
type Block = GenericArray<u8, BlockSize>;
type M = [Wrapping<u32>; 16];
/// Inner state of Shabal hash functions.
#[derive(Clone)]
pub struct ShabalVarCore {
    a: [Wrapping<u32>; 12],
    b: M,
    c: M,
    w: Wrapping<u64>,
}
impl ShabalVarCore {
    #[allow(clippy::needless_range_loop)]
    fn add_m(&mut self, m: &M) {
        for i in 0..16 {
            self.b[i] += m[i];
        }
    }
    #[allow(clippy::needless_range_loop)]
    fn sub_m(&mut self, m: &M) {
        for i in 0..16 {
            self.c[i] -= m[i];
        }
    }
    fn xor_w(&mut self) {
        self.a[0].0 ^= self.w.0 as u32;
        self.a[1].0 ^= (self.w.0 >> 32) as u32;
    }
    fn perm(&mut self, m: &M) {
        self.b.iter_mut().for_each(|b| b.0 = b.0.rotate_left(17));
        self.perm_blocks(m);
        let a = &mut self.a;
        let c = &self.c;
        a[0] += c[11] + c[15] + c[3];
        a[1] += c[12] + c[0] + c[4];
        a[2] += c[13] + c[1] + c[5];
        a[3] += c[14] + c[2] + c[6];
        a[4] += c[15] + c[3] + c[7];
        a[5] += c[0] + c[4] + c[8];
        a[6] += c[1] + c[5] + c[9];
        a[7] += c[2] + c[6] + c[10];
        a[8] += c[3] + c[7] + c[11];
        a[9] += c[4] + c[8] + c[12];
        a[10] += c[5] + c[9] + c[13];
        a[11] += c[6] + c[10] + c[14];
    }
    #[allow(clippy::too_many_arguments)]
    fn perm_elt(
        &mut self,
        xa0: usize,
        xa1: usize,
        xb0: usize,
        xb1: usize,
        xb2: usize,
        xb3: usize,
        xc0: usize,
        xm: Wrapping<u32>,
    ) {
        let a = &mut self.a;
        let b = &mut self.b;
        let xc = self.c[xc0];
        let t1 = Wrapping(a[xa1].0.rotate_left(15));
        let t2 = t1 * Wrapping(5);
        let t3 = (a[xa0] ^ t2 ^ xc) * Wrapping(3);
        a[xa0] = t3 ^ b[xb1] ^ (b[xb2] & !b[xb3]) ^ xm;
        let t = Wrapping(b[xb0].0.rotate_left(1));
        b[xb0] = !(t ^ a[xa0]);
    }
    fn perm_blocks(&mut self, m: &M) {
        self.perm_elt(0, 11, 0, 13, 9, 6, 8, m[0]);
        self.perm_elt(1, 0, 1, 14, 10, 7, 7, m[1]);
        self.perm_elt(2, 1, 2, 15, 11, 8, 6, m[2]);
        self.perm_elt(3, 2, 3, 0, 12, 9, 5, m[3]);
        self.perm_elt(4, 3, 4, 1, 13, 10, 4, m[4]);
        self.perm_elt(5, 4, 5, 2, 14, 11, 3, m[5]);
        self.perm_elt(6, 5, 6, 3, 15, 12, 2, m[6]);
        self.perm_elt(7, 6, 7, 4, 0, 13, 1, m[7]);
        self.perm_elt(8, 7, 8, 5, 1, 14, 0, m[8]);
        self.perm_elt(9, 8, 9, 6, 2, 15, 15, m[9]);
        self.perm_elt(10, 9, 10, 7, 3, 0, 14, m[10]);
        self.perm_elt(11, 10, 11, 8, 4, 1, 13, m[11]);
        self.perm_elt(0, 11, 12, 9, 5, 2, 12, m[12]);
        self.perm_elt(1, 0, 13, 10, 6, 3, 11, m[13]);
        self.perm_elt(2, 1, 14, 11, 7, 4, 10, m[14]);
        self.perm_elt(3, 2, 15, 12, 8, 5, 9, m[15]);
        self.perm_elt(4, 3, 0, 13, 9, 6, 8, m[0]);
        self.perm_elt(5, 4, 1, 14, 10, 7, 7, m[1]);
        self.perm_elt(6, 5, 2, 15, 11, 8, 6, m[2]);
        self.perm_elt(7, 6, 3, 0, 12, 9, 5, m[3]);
        self.perm_elt(8, 7, 4, 1, 13, 10, 4, m[4]);
        self.perm_elt(9, 8, 5, 2, 14, 11, 3, m[5]);
        self.perm_elt(10, 9, 6, 3, 15, 12, 2, m[6]);
        self.perm_elt(11, 10, 7, 4, 0, 13, 1, m[7]);
        self.perm_elt(0, 11, 8, 5, 1, 14, 0, m[8]);
        self.perm_elt(1, 0, 9, 6, 2, 15, 15, m[9]);
        self.perm_elt(2, 1, 10, 7, 3, 0, 14, m[10]);
        self.perm_elt(3, 2, 11, 8, 4, 1, 13, m[11]);
        self.perm_elt(4, 3, 12, 9, 5, 2, 12, m[12]);
        self.perm_elt(5, 4, 13, 10, 6, 3, 11, m[13]);
        self.perm_elt(6, 5, 14, 11, 7, 4, 10, m[14]);
        self.perm_elt(7, 6, 15, 12, 8, 5, 9, m[15]);
        self.perm_elt(8, 7, 0, 13, 9, 6, 8, m[0]);
        self.perm_elt(9, 8, 1, 14, 10, 7, 7, m[1]);
        self.perm_elt(10, 9, 2, 15, 11, 8, 6, m[2]);
        self.perm_elt(11, 10, 3, 0, 12, 9, 5, m[3]);
        self.perm_elt(0, 11, 4, 1, 13, 10, 4, m[4]);
        self.perm_elt(1, 0, 5, 2, 14, 11, 3, m[5]);
        self.perm_elt(2, 1, 6, 3, 15, 12, 2, m[6]);
        self.perm_elt(3, 2, 7, 4, 0, 13, 1, m[7]);
        self.perm_elt(4, 3, 8, 5, 1, 14, 0, m[8]);
        self.perm_elt(5, 4, 9, 6, 2, 15, 15, m[9]);
        self.perm_elt(6, 5, 10, 7, 3, 0, 14, m[10]);
        self.perm_elt(7, 6, 11, 8, 4, 1, 13, m[11]);
        self.perm_elt(8, 7, 12, 9, 5, 2, 12, m[12]);
        self.perm_elt(9, 8, 13, 10, 6, 3, 11, m[13]);
        self.perm_elt(10, 9, 14, 11, 7, 4, 10, m[14]);
        self.perm_elt(11, 10, 15, 12, 8, 5, 9, m[15]);
    }
    fn swap_b_c(&mut self) {
        mem::swap(&mut self.b, &mut self.c);
    }
}
#[inline]
fn read_m(input: &Block) -> M {
    let mut m = [Wrapping(0); 16];
    for (o, chunk) in m.iter_mut().zip(input.chunks_exact(4)) {
        let a = chunk.try_into().unwrap();
        *o = Wrapping(u32::from_le_bytes(a));
    }
    m
}
impl HashMarker for ShabalVarCore {}
impl BlockSizeUser for ShabalVarCore {
    type BlockSize = BlockSize;
}
impl BufferKindUser for ShabalVarCore {
    type BufferKind = Eager;
}
impl UpdateCore for ShabalVarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block]) {
        for block in blocks {
            let m = read_m(block);
            self.add_m(&m);
            self.xor_w();
            self.perm(&m);
            self.sub_m(&m);
            self.swap_b_c();
            self.w += Wrapping(1);
        }
    }
}
impl OutputSizeUser for ShabalVarCore {
    type OutputSize = U64;
}
impl VariableOutputCore for ShabalVarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Right;
    #[inline]
    #[allow(clippy::needless_range_loop)]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        let init = match output_size {
            24 => consts::INIT_192,
            28 => consts::INIT_224,
            32 => consts::INIT_256,
            48 => consts::INIT_384,
            64 => consts::INIT_512,
            _ => return Err(InvalidOutputSize),
        };
        let w = Wrapping(1);
        let mut a = [Wrapping(0u32); 12];
        let mut b = [Wrapping(0u32); 16];
        let mut c = [Wrapping(0u32); 16];
        for i in 0..12 {
            a[i] = Wrapping(init.0[i]);
        }
        for i in 0..16 {
            b[i] = Wrapping(init.1[i]);
            c[i] = Wrapping(init.2[i]);
        }
        Ok(Self { a, b, c, w })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let pos = buffer.get_pos();
        let block = buffer.pad_with_zeros();
        block[pos] = 0x80;
        let m = read_m(block);
        self.add_m(&m);
        self.xor_w();
        self.perm(&m);
        for _ in 0..3 {
            self.swap_b_c();
            self.xor_w();
            self.perm(&m);
        }
        for (chunk, v) in out.chunks_exact_mut(4).zip(self.b.iter()) {
            chunk.copy_from_slice(&v.0.to_le_bytes());
        }
    }
}
impl AlgorithmName for ShabalVarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Shabal")
    }
}
impl fmt::Debug for ShabalVarCore {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ShabalVarCore { ... }")
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use core::num::Wrapping;
    #[test]
    fn test_add_m() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, u32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let mut m = [Wrapping(rug_fuzz_1); 16];
        for i in rug_fuzz_2..rug_fuzz_3 {
            m[i] = Wrapping(i as u32);
        }
        let initial_b = core.b;
        core.add_m(&m);
        for i in rug_fuzz_4..rug_fuzz_5 {
            debug_assert_eq!(core.b[i], initial_b[i] + m[i]);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use core_api::ShabalVarCore;
    use std::num::Wrapping;
    fn create_test_core() -> ShabalVarCore {
        let a = [Wrapping(0x0); 12];
        let b = [Wrapping(0x0); 16];
        let c = [Wrapping(0x0); 16];
        ShabalVarCore {
            a,
            b,
            c,
            w: Wrapping(0x0),
        }
    }
    #[test]
    fn perm_correctness() {
        let mut core = create_test_core();
        let m = [Wrapping(0x0); 16];
        let expected_a = core.a;
        let expected_b = core
            .b
            .iter()
            .map(|x| Wrapping(x.0.rotate_left(17)))
            .collect::<Vec<_>>();
        let expected_c = core.c;
        core.perm(&m);
        assert_eq!(
            core.a, expected_a, "Expected a to be {:?}, but found {:?}.", expected_a,
            core.a
        );
        assert!(
            core.b.iter().zip(expected_b.iter()).all(| (x, y) | x == y),
            "Expected b to be {:?}, but found {:?}.", expected_b, core.b
        );
        assert_eq!(
            core.c, expected_c, "Expected c to be {:?}, but found {:?}.", expected_c,
            core.c
        );
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_perm_elt() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(usize, usize, usize, usize, usize, usize, usize, usize, u32, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let a_orig = core.a;
        let b_orig = core.b;
        let c_orig = core.c;
        let xa0 = rug_fuzz_1;
        let xa1 = rug_fuzz_2;
        let xb0 = rug_fuzz_3;
        let xb1 = rug_fuzz_4;
        let xb2 = rug_fuzz_5;
        let xb3 = rug_fuzz_6;
        let xc0 = rug_fuzz_7;
        let xm = Wrapping(rug_fuzz_8);
        core.perm_elt(xa0, xa1, xb0, xb1, xb2, xb3, xc0, xm);
        debug_assert_ne!(core.a[xa0], a_orig[xa0], "a[xa0] should have changed");
        debug_assert_ne!(core.b[xb0], b_orig[xb0], "b[xb0] should have changed");
        debug_assert_eq!(core.a[xa1], a_orig[xa1], "a[xa1] should not have changed");
        debug_assert_eq!(core.b[xb1], b_orig[xb1], "b[xb1] should not have changed");
        debug_assert_eq!(core.b[xb2], b_orig[xb2], "b[xb2] should not have changed");
        debug_assert_eq!(core.b[xb3], b_orig[xb3], "b[xb3] should not have changed");
        debug_assert_eq!(core.c[xc0], c_orig[xc0], "c[xc0] should not have changed");
        for i in rug_fuzz_9..rug_fuzz_10 {
            debug_assert_eq!(core.a[i], a_orig[i], "a[{}] should not have changed", i);
        }
        for i in rug_fuzz_11..rug_fuzz_12 {
            debug_assert_eq!(core.b[i], b_orig[i], "b[{}] should not have changed", i);
            debug_assert_eq!(core.c[i], c_orig[i], "c[{}] should not have changed", i);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use core::num::Wrapping;
    use digest::InvalidOutputSize;
    #[test]
    fn test_swap_b_c() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut shabal = ShabalVarCore::new(rug_fuzz_0).unwrap();
        shabal.b = [Wrapping(rug_fuzz_1); 16];
        shabal.c = [Wrapping(rug_fuzz_2); 16];
        let original_b = shabal.b;
        let original_c = shabal.c;
        shabal.swap_b_c();
        debug_assert_eq!(shabal.b, original_c, "b should have the original values of c");
        debug_assert_eq!(shabal.c, original_b, "c should have the original values of b");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_xor_w() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u32, u32, u32, u64, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = ShabalVarCore {
            a: [Wrapping(rug_fuzz_0); 12],
            b: [Wrapping(rug_fuzz_1); 16],
            c: [Wrapping(rug_fuzz_2); 16],
            w: Wrapping(rug_fuzz_3),
        };
        core.xor_w();
        debug_assert_eq!(core.a[rug_fuzz_4], Wrapping(0x9ABCDEF0));
        debug_assert_eq!(core.a[rug_fuzz_5], Wrapping(0x12345678));
        for i in rug_fuzz_6..core.a.len() {
            debug_assert_eq!(core.a[i], Wrapping(0));
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_288 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::generic_array::typenum::U16;
    #[test]
    fn test_read_m() {
        let _rug_st_tests_rug_288_rrrruuuugggg_test_read_m = 0;
        let mut p0: Block = GenericArray::default();
        let _ = p0.iter_mut().enumerate().for_each(|(i, byte)| *byte = i as u8);
        crate::core_api::read_m(&p0);
        let _rug_ed_tests_rug_288_rrrruuuugggg_test_read_m = 0;
    }
}
#[cfg(test)]
mod tests_rug_289 {
    use super::*;
    use crate::core_api::ShabalVarCore;
    use std::num::Wrapping;
    #[test]
    fn test_sub_m() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let p1: [Wrapping<u32>; 16] = [
            Wrapping(rug_fuzz_1),
            Wrapping(rug_fuzz_2),
            Wrapping(rug_fuzz_3),
            Wrapping(rug_fuzz_4),
            Wrapping(rug_fuzz_5),
            Wrapping(rug_fuzz_6),
            Wrapping(rug_fuzz_7),
            Wrapping(rug_fuzz_8),
            Wrapping(rug_fuzz_9),
            Wrapping(rug_fuzz_10),
            Wrapping(rug_fuzz_11),
            Wrapping(rug_fuzz_12),
            Wrapping(rug_fuzz_13),
            Wrapping(rug_fuzz_14),
            Wrapping(rug_fuzz_15),
            Wrapping(rug_fuzz_16),
        ];
        p0.sub_m(&p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_290 {
    use super::*;
    use crate::core_api::ShabalVarCore;
    use std::num::Wrapping;
    #[test]
    fn test_perm_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, u32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let p1 = {
            let mut message = [rug_fuzz_1; 16];
            for i in rug_fuzz_2..rug_fuzz_3 {
                message[i] = i as u32;
            }
            let m = message.map(Wrapping);
            m
        };
        crate::core_api::ShabalVarCore::perm_blocks(&mut p0, &p1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_291 {
    use super::*;
    use crate::digest::core_api::UpdateCore;
    use crate::core_api::ShabalVarCore;
    use digest::generic_array::GenericArray;
    use digest::typenum::{B0, B1, UInt, UTerm};
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let mut p1 = GenericArray::<
            u8,
            UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>,
        >::default();
        p0.update_blocks(&[p1]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_292 {
    use super::*;
    use crate::digest::core_api::VariableOutputCore;
    use crate::core_api::ShabalVarCore;
    use digest::InvalidOutputSize;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let sizes = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        for &size in &sizes {
            let p0: usize = size;
            debug_assert!(< ShabalVarCore as VariableOutputCore > ::new(p0).is_ok());
        }
        let invalid_sizes = [rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8];
        for &size in &invalid_sizes {
            let p0: usize = size;
            debug_assert!(< ShabalVarCore as VariableOutputCore > ::new(p0).is_err());
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_293 {
    use crate::core_api::ShabalVarCore;
    use crate::digest::{
        block_buffer::BlockBuffer,
        core_api::{BlockSizeUser, BufferKindUser, VariableOutputCore},
        generic_array::GenericArray, OutputSizeUser,
    };
    #[test]
    fn test_finalize_variable_core() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = ShabalVarCore::new(rug_fuzz_0).unwrap();
        let mut p1 = BlockBuffer::default();
        let mut p2 = GenericArray::<
            u8,
            <ShabalVarCore as OutputSizeUser>::OutputSize,
        >::default();
        p0.finalize_variable_core(&mut p1, &mut p2);
             }
}
}
}    }
}
