use crate::{consts, sha256::compress256, sha512::compress512};
use core::{fmt, slice::from_ref};
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, OutputSizeUser,
        TruncSide, UpdateCore, VariableOutputCore,
    },
    typenum::{Unsigned, U128, U32, U64},
    HashMarker, InvalidOutputSize, Output,
};
/// Core block-level SHA-256 hasher with variable output size.
///
/// Supports initialization only for 28 and 32 byte output sizes,
/// i.e. 224 and 256 bits respectively.
#[derive(Clone)]
pub struct Sha256VarCore {
    state: consts::State256,
    block_len: u64,
}
impl HashMarker for Sha256VarCore {}
impl BlockSizeUser for Sha256VarCore {
    type BlockSize = U64;
}
impl BufferKindUser for Sha256VarCore {
    type BufferKind = Eager;
}
impl UpdateCore for Sha256VarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        compress256(&mut self.state, blocks);
    }
}
impl OutputSizeUser for Sha256VarCore {
    type OutputSize = U32;
}
impl VariableOutputCore for Sha256VarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Left;
    #[inline]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        let state = match output_size {
            28 => consts::H256_224,
            32 => consts::H256_256,
            _ => return Err(InvalidOutputSize),
        };
        let block_len = 0;
        Ok(Self { state, block_len })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bs = Self::BlockSize::U64;
        let bit_len = 8 * (buffer.get_pos() as u64 + bs * self.block_len);
        buffer.len64_padding_be(bit_len, |b| compress256(&mut self.state, from_ref(b)));
        for (chunk, v) in out.chunks_exact_mut(4).zip(self.state.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}
impl AlgorithmName for Sha256VarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sha256")
    }
}
impl fmt::Debug for Sha256VarCore {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sha256VarCore { ... }")
    }
}
/// Core block-level SHA-512 hasher with variable output size.
///
/// Supports initialization only for 28, 32, 48, and 64 byte output sizes,
/// i.e. 224, 256, 384, and 512 bits respectively.
#[derive(Clone)]
pub struct Sha512VarCore {
    state: consts::State512,
    block_len: u128,
}
impl HashMarker for Sha512VarCore {}
impl BlockSizeUser for Sha512VarCore {
    type BlockSize = U128;
}
impl BufferKindUser for Sha512VarCore {
    type BufferKind = Eager;
}
impl UpdateCore for Sha512VarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u128;
        compress512(&mut self.state, blocks);
    }
}
impl OutputSizeUser for Sha512VarCore {
    type OutputSize = U64;
}
impl VariableOutputCore for Sha512VarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Left;
    #[inline]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        let state = match output_size {
            28 => consts::H512_224,
            32 => consts::H512_256,
            48 => consts::H512_384,
            64 => consts::H512_512,
            _ => return Err(InvalidOutputSize),
        };
        let block_len = 0;
        Ok(Self { state, block_len })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bs = Self::BlockSize::U64 as u128;
        let bit_len = 8 * (buffer.get_pos() as u128 + bs * self.block_len);
        buffer.len128_padding_be(bit_len, |b| compress512(&mut self.state, from_ref(b)));
        for (chunk, v) in out.chunks_exact_mut(8).zip(self.state.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}
impl AlgorithmName for Sha512VarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sha512")
    }
}
impl fmt::Debug for Sha512VarCore {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sha512VarCore { ... }")
    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use super::*;
    use crate::*;
    use crate::*;
    use core::fmt;
    use digest::core_api::{UpdateCore, Block};
    use digest::InvalidOutputSize;
    use crate::core_api::BlockSizeUser;
    use digest::consts::{U32, U64};
    #[test]
    fn test_update_blocks_single() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 1);
             }
}
}
}    }
    #[test]
    fn test_update_blocks_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block, block]);
        debug_assert_eq!(core.block_len, 2);
             }
}
}
}    }
    #[test]
    fn test_update_blocks_no_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        core.update_blocks(&[]);
        debug_assert_eq!(core.block_len, 0);
             }
}
}
}    }
    #[test]
    fn test_update_blocks_incremental() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block]);
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 2);
             }
}
}
}    }
    #[test]
    fn test_update_blocks_after_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block, block]);
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 3);
             }
}
}
}    }
    #[test]
    fn test_update_blocks_max_len() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        let block_size = <Sha256VarCore as BlockSizeUser>::BlockSize::U64 as u64;
        let max_blocks = u64::MAX / block_size;
        for _ in rug_fuzz_1..max_blocks {
            core.update_blocks(&[block]);
        }
        debug_assert_eq!(core.block_len, max_blocks);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    use crate::core_api::Sha256VarCore;
    use digest::core_api::VariableOutputCore;
    use digest::InvalidOutputSize;
    #[test]
    fn test_sha256_var_core_new_valid_224() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_ok());
             }
}
}
}    }
    #[test]
    fn test_sha256_var_core_new_valid_256() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_ok());
             }
}
}
}    }
    #[test]
    fn test_sha256_var_core_new_invalid_size() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_6_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::consts::State512;
    use crate::core_api::Sha512VarCore;
    use crate::core_api::UpdateCore;
    use crate::core_api::Block;
    use crate::core_api::InvalidOutputSize;
    use digest::generic_array::typenum::U128;
    use digest::generic_array::GenericArray;
    #[test]
    fn update_blocks_increases_block_length() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha512VarCore::new(rug_fuzz_0).unwrap();
        let initial_block_len = core.block_len;
        let block = GenericArray::<u8, U128>::default();
        let blocks = [block; 1];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, initial_block_len + blocks.len() as u128);
             }
}
}
}    }
    #[test]
    fn update_blocks_processes_multiple_blocks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Sha512VarCore::new(rug_fuzz_0).unwrap();
        let block = GenericArray::<u8, U128>::default();
        let blocks = [block; 2];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, blocks.len() as u128);
             }
}
}
}    }
    #[test]
    fn update_blocks_invalid_output_size_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha512VarCore::new(rug_fuzz_0).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use crate::core_api::Sha512VarCore;
    use digest::core_api::VariableOutputCore;
    use digest::InvalidOutputSize;
    #[test]
    fn sha512_var_core_new_valid_output_sizes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha512VarCore::new(rug_fuzz_0).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_1).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_2).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_3).is_ok());
             }
}
}
}    }
    #[test]
    fn sha512_var_core_new_invalid_output_sizes() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Sha512VarCore::new(rug_fuzz_0).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_1).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_2).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_3).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_4).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_5).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_6).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_7).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_8).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_9).is_err());
        debug_assert!(Sha512VarCore::new(rug_fuzz_10).is_err());
        debug_assert!(Sha512VarCore::new(usize::MAX).is_err());
             }
}
}
}    }
}
