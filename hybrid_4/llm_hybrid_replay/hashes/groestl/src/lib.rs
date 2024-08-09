//! An implementation of the [Grøstl][1] cryptographic hash function.
//!
//! # Usage
//!
//! ```
//! use groestl::{Digest, Groestl256};
//! use hex_literal::hex;
//!
//! // create a Groestl-256 hasher instance
//! let mut hasher = Groestl256::default();
//!
//! // process input message
//! hasher.update(b"my message");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 32]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("
//!     dc0283ca481efa76b7c19dd5a0b763dff0e867451bd9488a9c59f6c8b8047a86
//! "));
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/Grøstl
//! [2]: https://github.com/RustCrypto/hashes
#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
pub use digest::{self, Digest};
use core::fmt;
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        CtVariableCoreWrapper, OutputSizeUser, RtVariableCoreWrapper, TruncSide,
        UpdateCore, VariableOutputCore,
    },
    typenum::{Unsigned, U128, U28, U32, U48, U64},
    HashMarker, InvalidOutputSize, Output,
};
mod compress1024;
mod compress512;
mod table;
/// Lowest-level core hasher state of the short Groestl variant.
#[derive(Clone)]
pub struct GroestlShortVarCore {
    state: [u64; compress512::COLS],
    blocks_len: u64,
}
impl HashMarker for GroestlShortVarCore {}
impl BlockSizeUser for GroestlShortVarCore {
    type BlockSize = U64;
}
impl BufferKindUser for GroestlShortVarCore {
    type BufferKind = Eager;
}
impl UpdateCore for GroestlShortVarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.blocks_len += blocks.len() as u64;
        for block in blocks {
            compress512::compress(&mut self.state, block.as_ref());
        }
    }
}
impl OutputSizeUser for GroestlShortVarCore {
    type OutputSize = U32;
}
impl VariableOutputCore for GroestlShortVarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Right;
    #[inline]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        if output_size > Self::OutputSize::USIZE {
            return Err(InvalidOutputSize);
        }
        let mut state = [0; compress512::COLS];
        state[compress512::COLS - 1] = 8 * output_size as u64;
        let blocks_len = 0;
        Ok(Self { state, blocks_len })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let blocks_len = if buffer.remaining() <= 8 {
            self.blocks_len + 2
        } else {
            self.blocks_len + 1
        };
        buffer
            .len64_padding_be(
                blocks_len,
                |block| { compress512::compress(&mut self.state, block.as_ref()) },
            );
        let res = compress512::p(&self.state);
        let n = compress512::COLS / 2;
        for (chunk, v) in out.chunks_exact_mut(8).zip(res[n..].iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}
impl AlgorithmName for GroestlShortVarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroestlShort")
    }
}
impl fmt::Debug for GroestlShortVarCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroestlShortVarCore { ... }")
    }
}
/// Short Groestl variant which allows to choose output size at runtime.
pub type GroestlShortVar = RtVariableCoreWrapper<GroestlShortVarCore>;
/// Core hasher state of the short Groestl variant generic over output size.
pub type GroestlShortCore<OutSize> = CtVariableCoreWrapper<GroestlShortVarCore, OutSize>;
/// Hasher state of the short Groestl variant generic over output size.
pub type GroestlShort<OutSize> = CoreWrapper<GroestlShortCore<OutSize>>;
/// Groestl-224 hasher state.
pub type Groestl224 = CoreWrapper<GroestlShortCore<U28>>;
/// Groestl-256 hasher state.
pub type Groestl256 = CoreWrapper<GroestlShortCore<U32>>;
/// Lowest-level core hasher state of the long Groestl variant.
#[derive(Clone)]
pub struct GroestlLongVarCore {
    state: [u64; compress1024::COLS],
    blocks_len: u64,
}
impl HashMarker for GroestlLongVarCore {}
impl BlockSizeUser for GroestlLongVarCore {
    type BlockSize = U128;
}
impl BufferKindUser for GroestlLongVarCore {
    type BufferKind = Eager;
}
impl UpdateCore for GroestlLongVarCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.blocks_len += blocks.len() as u64;
        for block in blocks {
            compress1024::compress(&mut self.state, block.as_ref());
        }
    }
}
impl OutputSizeUser for GroestlLongVarCore {
    type OutputSize = U64;
}
impl VariableOutputCore for GroestlLongVarCore {
    const TRUNC_SIDE: TruncSide = TruncSide::Right;
    #[inline]
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        if output_size > Self::OutputSize::USIZE {
            return Err(InvalidOutputSize);
        }
        let mut state = [0; compress1024::COLS];
        state[compress1024::COLS - 1] = 8 * output_size as u64;
        let blocks_len = 0;
        Ok(Self { state, blocks_len })
    }
    #[inline]
    fn finalize_variable_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let blocks_len = if buffer.remaining() <= 8 {
            self.blocks_len + 2
        } else {
            self.blocks_len + 1
        };
        buffer
            .len64_padding_be(
                blocks_len,
                |block| { compress1024::compress(&mut self.state, block.as_ref()) },
            );
        let res = compress1024::p(&self.state);
        let n = compress1024::COLS / 2;
        for (chunk, v) in out.chunks_exact_mut(8).zip(res[n..].iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}
impl AlgorithmName for GroestlLongVarCore {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroestlLong")
    }
}
impl fmt::Debug for GroestlLongVarCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroestlLongVarCore { ... }")
    }
}
/// Long Groestl variant which allows to choose output size at runtime.
pub type GroestlLongVar = RtVariableCoreWrapper<GroestlLongVarCore>;
/// Core hasher state of the long Groestl variant generic over output size.
pub type GroestlLongCore<OutSize> = CtVariableCoreWrapper<GroestlLongVarCore, OutSize>;
/// Hasher state of the long Groestl variant generic over output size.
pub type GroestlLong<OutSize> = CoreWrapper<GroestlLongCore<OutSize>>;
/// Groestl-384 hasher state.
pub type Groestl384 = CoreWrapper<GroestlLongCore<U48>>;
/// Groestl-512 hasher state.
pub type Groestl512 = CoreWrapper<GroestlLongCore<U64>>;
#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;
    use crate::*;
    use digest::core_api::{Block, BlockSizeUser, UpdateCore, VariableOutputCore};
    use digest::InvalidOutputSize;
    #[test]
    fn update_blocks_empty() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_update_blocks_empty = 0;
        let rug_fuzz_0 = 64;
        let mut core = GroestlLongVarCore::new(rug_fuzz_0).unwrap();
        let blocks = [];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.blocks_len, 0);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_update_blocks_empty = 0;
    }
    #[test]
    fn update_blocks_single() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_update_blocks_single = 0;
        let rug_fuzz_0 = 64;
        let mut core = GroestlLongVarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<GroestlLongVarCore>::default();
        let blocks = [block; 1];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.blocks_len, 1);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_update_blocks_single = 0;
    }
    #[test]
    fn update_blocks_multiple() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_update_blocks_multiple = 0;
        let rug_fuzz_0 = 64;
        let mut core = GroestlLongVarCore::new(rug_fuzz_0).unwrap();
        let block = Block::<GroestlLongVarCore>::default();
        let blocks = [block; 3];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.blocks_len, 3);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_update_blocks_multiple = 0;
    }
    #[test]
    fn update_blocks_state_change() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_update_blocks_state_change = 0;
        let rug_fuzz_0 = 64;
        let mut core = GroestlLongVarCore::new(rug_fuzz_0).unwrap();
        let initial_state = core.state.clone();
        let block = Block::<GroestlLongVarCore>::default();
        let blocks = [block; 1];
        core.update_blocks(&blocks);
        debug_assert!(
            core.state != initial_state, "State should change after update_blocks"
        );
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_update_blocks_state_change = 0;
    }
    #[test]
    fn update_blocks_invalid_size() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_update_blocks_invalid_size = 0;
        let rug_fuzz_0 = 65;
        let result = GroestlLongVarCore::new(rug_fuzz_0);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_update_blocks_invalid_size = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use digest::core_api::{Block, UpdateCore};
    #[test]
    fn update_blocks_test() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_update_blocks_test = 0;
        let rug_fuzz_0 = 32;
        let rug_fuzz_1 = 1u8;
        let mut core = GroestlShortVarCore::new(rug_fuzz_0).unwrap();
        let initial_state = core.state;
        let block_data = [rug_fuzz_1; 64];
        let blocks = [Block::<GroestlShortVarCore>::from(block_data)];
        core.update_blocks(&blocks);
        debug_assert_ne!(
            initial_state, core.state,
            "State should be different after processing a block."
        );
        debug_assert_eq!(
            core.blocks_len, 1,
            "blocks_len should be incremented by 1 after processing a block."
        );
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_update_blocks_test = 0;
    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::digest::core_api::VariableOutputCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_115_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 32;
        let output_size: usize = rug_fuzz_0;
        <GroestlShortVarCore>::new(output_size).unwrap();
        let _rug_ed_tests_rug_115_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use digest::core_api::{
        VariableOutputCore, OutputSizeUser, UpdateCore, BlockSizeUser, BufferKindUser,
    };
    use digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_finalize_variable_core() {
        let _rug_st_tests_rug_116_rrrruuuugggg_test_finalize_variable_core = 0;
        let rug_fuzz_0 = 32usize;
        let output_size = rug_fuzz_0;
        let mut p0 = GroestlShortVarCore::new(output_size).unwrap();
        let mut p1 = BlockBuffer::<
            <GroestlShortVarCore as BlockSizeUser>::BlockSize,
            <GroestlShortVarCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <GroestlShortVarCore as OutputSizeUser>::OutputSize,
        >::default();
        <GroestlShortVarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_116_rrrruuuugggg_test_finalize_variable_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use crate::digest::InvalidOutputSize;
    use crate::digest::core_api::VariableOutputCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_118_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 64;
        let p0: usize = rug_fuzz_0;
        <GroestlLongVarCore>::new(p0).unwrap();
        let _rug_ed_tests_rug_118_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_119 {
    use super::*;
    use crate::digest::core_api::{VariableOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    use crate::digest::OutputSizeUser;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_119_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 64;
        let mut p0 = GroestlLongVarCore::new(rug_fuzz_0).unwrap();
        let mut p1 = BlockBuffer::<
            <GroestlLongVarCore as BlockSizeUser>::BlockSize,
            <GroestlLongVarCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <GroestlLongVarCore as OutputSizeUser>::OutputSize,
        >::default();
        <GroestlLongVarCore as VariableOutputCore>::finalize_variable_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_119_rrrruuuugggg_test_rug = 0;
    }
}
