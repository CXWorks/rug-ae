//! An implementation of the [Tiger][1] cryptographic hash algorithms.
//!
//! Tiger2 is a variant of the original Tiger with a small padding tweak.
//!
//! # Usage
//!
//! ```rust
//! use hex_literal::hex;
//! use tiger::{Tiger, Digest};
//!
//! // create a Tiger object
//! let mut hasher = Tiger::new();
//!
//! // process input message
//! hasher.update(b"hello world");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 24]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("4c8fbddae0b6f25832af45e7c62811bb64ec3e43691e9cc3"));
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/Tiger_(hash_function)
//! [2]: https://github.com/RustCrypto/hashes
#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]
pub use digest::{self, Digest};
use core::fmt;
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U24, U64},
    HashMarker, Output,
};
mod compress;
mod tables;
use compress::compress;
type State = [u64; 3];
const S0: State = [0x0123_4567_89AB_CDEF, 0xFEDC_BA98_7654_3210, 0xF096_A5B4_C3B2_E187];
/// Core Tiger hasher state.
#[derive(Clone)]
pub struct TigerCore {
    block_len: u64,
    state: State,
}
impl HashMarker for TigerCore {}
impl BlockSizeUser for TigerCore {
    type BlockSize = U64;
}
impl BufferKindUser for TigerCore {
    type BufferKind = Eager;
}
impl OutputSizeUser for TigerCore {
    type OutputSize = U24;
}
impl UpdateCore for TigerCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        for block in blocks {
            compress(&mut self.state, block.as_ref());
        }
    }
}
impl FixedOutputCore for TigerCore {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bs = Self::BlockSize::U64 as u64;
        let pos = buffer.get_pos() as u64;
        let bit_len = 8 * (pos + bs * self.block_len);
        buffer
            .digest_pad(
                1,
                &bit_len.to_le_bytes(),
                |b| { compress(&mut self.state, b.as_ref()) },
            );
        for (chunk, v) in out.chunks_exact_mut(8).zip(self.state.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}
impl Default for TigerCore {
    fn default() -> Self {
        Self { block_len: 0, state: S0 }
    }
}
impl Reset for TigerCore {
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for TigerCore {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Tiger")
    }
}
impl fmt::Debug for TigerCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TigerCore { ... }")
    }
}
/// Core Tiger2 hasher state.
#[derive(Clone)]
pub struct Tiger2Core {
    block_len: u64,
    state: State,
}
impl HashMarker for Tiger2Core {}
impl BlockSizeUser for Tiger2Core {
    type BlockSize = U64;
}
impl BufferKindUser for Tiger2Core {
    type BufferKind = Eager;
}
impl OutputSizeUser for Tiger2Core {
    type OutputSize = U24;
}
impl UpdateCore for Tiger2Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        for block in blocks {
            compress(&mut self.state, block.as_ref());
        }
    }
}
impl FixedOutputCore for Tiger2Core {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bs = Self::BlockSize::U64 as u64;
        let pos = buffer.get_pos() as u64;
        let bit_len = 8 * (pos + bs * self.block_len);
        buffer.len64_padding_le(bit_len, |b| compress(&mut self.state, b.as_ref()));
        for (chunk, v) in out.chunks_exact_mut(8).zip(self.state.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}
impl Default for Tiger2Core {
    fn default() -> Self {
        Self {
            block_len: 0,
            state: [0x0123_4567_89AB_CDEF, 0xFEDC_BA98_7654_3210, 0xF096_A5B4_C3B2_E187],
        }
    }
}
impl Reset for Tiger2Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for Tiger2Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Tiger2")
    }
}
impl fmt::Debug for Tiger2Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Tiger2Core { ... }")
    }
}
/// Tiger hasher state.
pub type Tiger = CoreWrapper<TigerCore>;
/// Tiger2 hasher state.
pub type Tiger2 = CoreWrapper<Tiger2Core>;
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::Tiger2Core;
    use core::default::Default;
    #[test]
    fn default_test() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_default_test = 0;
        let tiger2 = Tiger2Core::default();
        debug_assert_eq!(tiger2.block_len, 0);
        debug_assert_eq!(
            tiger2.state, [0x0123_4567_89AB_CDEF, 0xFEDC_BA98_7654_3210,
            0xF096_A5B4_C3B2_E187,]
        );
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_default_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use digest::core_api::{Block, UpdateCore};
    fn compress(state: &mut [u64; 3], block: &[u8]) {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_compress = 0;
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_compress = 0;
    }
    #[test]
    fn update_blocks_test() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_update_blocks_test = 0;
        let rug_fuzz_0 = 0u8;
        let mut core = Tiger2Core::default();
        let initial_state = core.state;
        let block = Block::<Tiger2Core>::from([rug_fuzz_0; 64]);
        let blocks = &[block];
        core.update_blocks(blocks);
        debug_assert_eq!(core.block_len, 1);
        let blocks = &[block, block, block];
        core.update_blocks(blocks);
        debug_assert_eq!(core.block_len, 4);
        debug_assert!(core.state != initial_state);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_update_blocks_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use digest::core_api::{Block, UpdateCore};
    fn compress_mock(state: &mut State, _block: &[u8]) {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_compress_mock = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        *state = [
            state[rug_fuzz_0].wrapping_add(rug_fuzz_1),
            state[rug_fuzz_2],
            state[rug_fuzz_3],
        ];
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_compress_mock = 0;
    }
    #[test]
    fn update_blocks_single_block() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_update_blocks_single_block = 0;
        let rug_fuzz_0 = 0u8;
        let mut core = TigerCore::default();
        let initial_state = core.state;
        let blocks = [Block::<TigerCore>::from([rug_fuzz_0; 64])];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, 1);
        debug_assert_ne!(
            core.state, initial_state,
            "State should have been updated by the compress function."
        );
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_update_blocks_single_block = 0;
    }
    #[test]
    fn update_blocks_multiple_blocks() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_update_blocks_multiple_blocks = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 2u8;
        let mut core = TigerCore::default();
        let initial_state = core.state;
        let blocks = [
            Block::<TigerCore>::from([rug_fuzz_0; 64]),
            Block::<TigerCore>::from([rug_fuzz_1; 64]),
            Block::<TigerCore>::from([rug_fuzz_2; 64]),
        ];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, 3);
        debug_assert_ne!(
            core.state, initial_state,
            "State should have been updated by the compress function."
        );
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_update_blocks_multiple_blocks = 0;
    }
    #[test]
    fn update_blocks_no_blocks() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_update_blocks_no_blocks = 0;
        let mut core = TigerCore::default();
        let initial_state = core.state;
        let blocks: [Block<TigerCore>; 0] = [];
        core.update_blocks(&blocks);
        debug_assert_eq!(core.block_len, 0);
        debug_assert_eq!(
            core.state, initial_state, "State should not have been updated."
        );
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_update_blocks_no_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_321 {
    use super::*;
    use digest::core_api::{
        FixedOutputCore, BlockSizeUser, OutputSizeUser, BufferKindUser,
    };
    use digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_321_rrrruuuugggg_test_rug = 0;
        let mut p0 = TigerCore::default();
        let mut p1 = BlockBuffer::<
            <TigerCore as BlockSizeUser>::BlockSize,
            <TigerCore as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <TigerCore as OutputSizeUser>::OutputSize,
        >::default();
        <TigerCore as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_321_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_322 {
    use super::*;
    use core::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_rug_322_rrrruuuugggg_test_default = 0;
        let result = <TigerCore as Default>::default();
        debug_assert_eq!(result.block_len, 0);
        debug_assert_eq!(result.state, S0);
        let _rug_ed_tests_rug_322_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_rug_323 {
    use super::*;
    use crate::digest::Reset;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_323_rrrruuuugggg_test_reset = 0;
        let mut p0 = TigerCore::default();
        <TigerCore as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_323_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_325 {
    use crate::Tiger2Core;
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    use crate::digest::core_api::{
        FixedOutputCore, BlockSizeUser, BufferKindUser, OutputSizeUser,
    };
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_325_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Tiger2Core::default();
        let mut p1 = BlockBuffer::<
            _,
            <Tiger2Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Tiger2Core as OutputSizeUser>::OutputSize,
        >::default();
        <Tiger2Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_325_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_326 {
    use super::*;
    use crate::digest::Reset;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_326_rrrruuuugggg_test_rug = 0;
        let mut p0 = Tiger2Core::default();
        <Tiger2Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_326_rrrruuuugggg_test_rug = 0;
    }
}
