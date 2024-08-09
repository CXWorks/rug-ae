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
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_single = 0;
        let rug_fuzz_0 = 32;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 1);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_single = 0;
    }
    #[test]
    fn test_update_blocks_multiple() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_multiple = 0;
        let rug_fuzz_0 = 32;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block, block]);
        debug_assert_eq!(core.block_len, 2);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_multiple = 0;
    }
    #[test]
    fn test_update_blocks_no_blocks() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_no_blocks = 0;
        let rug_fuzz_0 = 32;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        core.update_blocks(&[]);
        debug_assert_eq!(core.block_len, 0);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_no_blocks = 0;
    }
    #[test]
    fn test_update_blocks_incremental() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_incremental = 0;
        let rug_fuzz_0 = 32;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block]);
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 2);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_incremental = 0;
    }
    #[test]
    fn test_update_blocks_after_blocks() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_after_blocks = 0;
        let rug_fuzz_0 = 32;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        core.update_blocks(&[block, block]);
        core.update_blocks(&[block]);
        debug_assert_eq!(core.block_len, 3);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_after_blocks = 0;
    }
    #[test]
    fn test_update_blocks_max_len() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_max_len = 0;
        let rug_fuzz_0 = 32;
        let rug_fuzz_1 = 0;
        let mut core = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<Sha256VarCore>::default();
        let block_size = <Sha256VarCore as BlockSizeUser>::BlockSize::U64 as u64;
        let max_blocks = u64::MAX / block_size;
        for _ in rug_fuzz_1..max_blocks {
            core.update_blocks(&[block]);
        }
        debug_assert_eq!(core.block_len, max_blocks);
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_update_blocks_max_len = 0;
    }
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
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_valid_224 = 0;
        let rug_fuzz_0 = 28;
        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_ok());
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_valid_224 = 0;
    }
    #[test]
    fn test_sha256_var_core_new_valid_256() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_valid_256 = 0;
        let rug_fuzz_0 = 32;
        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_ok());
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_valid_256 = 0;
    }
    #[test]
    fn test_sha256_var_core_new_invalid_size() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_invalid_size = 0;
        let rug_fuzz_0 = 30;
        debug_assert!(Sha256VarCore::new(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_sha256_var_core_new_invalid_size = 0;
    }
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
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_increases_block_length = 0;
        let rug_fuzz_0 = 64;
        let mut core = Sha512VarCore::new(rug_fuzz_0).unwrap();
        let initial_block_len = core.block_len;
        let block = GenericArray::<u8, U128>::default();
        let blocks = [block; 1];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, initial_block_len + blocks.len() as u128);
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_increases_block_length = 0;
    }
    #[test]
    fn update_blocks_processes_multiple_blocks() {
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_processes_multiple_blocks = 0;
        let rug_fuzz_0 = 64;
        let mut core = Sha512VarCore::new(rug_fuzz_0).unwrap();
        let block = GenericArray::<u8, U128>::default();
        let blocks = [block; 2];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, blocks.len() as u128);
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_processes_multiple_blocks = 0;
    }
    #[test]
    fn update_blocks_invalid_output_size_error() {
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_invalid_output_size_error = 0;
        let rug_fuzz_0 = 16;
        debug_assert!(Sha512VarCore::new(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_update_blocks_invalid_output_size_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use crate::core_api::Sha512VarCore;
    use digest::core_api::VariableOutputCore;
    use digest::InvalidOutputSize;
    #[test]
    fn sha512_var_core_new_valid_output_sizes() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_sha512_var_core_new_valid_output_sizes = 0;
        let rug_fuzz_0 = 28;
        let rug_fuzz_1 = 32;
        let rug_fuzz_2 = 48;
        let rug_fuzz_3 = 64;
        debug_assert!(Sha512VarCore::new(rug_fuzz_0).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_1).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_2).is_ok());
        debug_assert!(Sha512VarCore::new(rug_fuzz_3).is_ok());
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_sha512_var_core_new_valid_output_sizes = 0;
    }
    #[test]
    fn sha512_var_core_new_invalid_output_sizes() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_sha512_var_core_new_invalid_output_sizes = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 27;
        let rug_fuzz_3 = 29;
        let rug_fuzz_4 = 31;
        let rug_fuzz_5 = 33;
        let rug_fuzz_6 = 47;
        let rug_fuzz_7 = 49;
        let rug_fuzz_8 = 63;
        let rug_fuzz_9 = 65;
        let rug_fuzz_10 = 100;
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
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_sha512_var_core_new_invalid_output_sizes = 0;
    }
}
#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    use crate::digest::core_api::{VariableOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::{OutputSizeUser, InvalidOutputSize};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::{GenericArray, ArrayLength};
    use crate::Sha256VarCore;
    #[test]
    fn test_finalize_variable_core() {
        let _rug_st_tests_rug_215_rrrruuuugggg_test_finalize_variable_core = 0;
        let rug_fuzz_0 = 32;
        let mut p0 = Sha256VarCore::new(rug_fuzz_0).unwrap();
        let mut p1: BlockBuffer<
            <Sha256VarCore as BlockSizeUser>::BlockSize,
            <Sha256VarCore as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<u8, <Sha256VarCore as OutputSizeUser>::OutputSize> = GenericArray::default();
        <Sha256VarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_215_rrrruuuugggg_test_finalize_variable_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    use crate::core_api::Sha512VarCore;
    use crate::core_api::VariableOutputCore;
    use digest::core_api::VariableOutputCore as _;
    use digest::core_api::{BlockSizeUser, BufferKindUser, OutputSizeUser};
    use digest::generic_array::GenericArray;
    use digest::block_buffer::BlockBuffer;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_217_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 64;
        let mut p0 = Sha512VarCore::new(rug_fuzz_0).unwrap();
        let mut p1 = BlockBuffer::<
            <Sha512VarCore as BlockSizeUser>::BlockSize,
            <Sha512VarCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Sha512VarCore as OutputSizeUser>::OutputSize,
        >::default();
        <Sha512VarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_217_rrrruuuugggg_test_rug = 0;
    }
}
