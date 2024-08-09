//! An implementation of the [MD5][1] cryptographic hash algorithm.
//!
//! # Usage
//!
//! ```rust
//! use md5::{Md5, Digest};
//! use hex_literal::hex;
//!
//! // create a Md5 hasher instance
//! let mut hasher = Md5::new();
//!
//! // process input message
//! hasher.update(b"hello world");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 16]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("5eb63bbbe01eeed093cb22bb8f5acdc3"));
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/MD5
//! [2]: https://github.com/RustCrypto/hashes
#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![warn(missing_docs, rust_2018_idioms)]
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern crate md5_asm as compress;
#[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
mod compress;
pub use digest::{self, Digest};
use compress::compress;
use core::{fmt, slice::from_ref};
#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U16, U64},
    HashMarker, Output,
};
/// Core MD5 hasher state.
#[derive(Clone)]
pub struct Md5Core {
    block_len: u64,
    state: [u32; 4],
}
impl HashMarker for Md5Core {}
impl BlockSizeUser for Md5Core {
    type BlockSize = U64;
}
impl BufferKindUser for Md5Core {
    type BufferKind = Eager;
}
impl OutputSizeUser for Md5Core {
    type OutputSize = U16;
}
impl UpdateCore for Md5Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len = self.block_len.wrapping_add(blocks.len() as u64);
        compress(&mut self.state, convert(blocks))
    }
}
impl FixedOutputCore for Md5Core {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bit_len = self
            .block_len
            .wrapping_mul(Self::BlockSize::U64)
            .wrapping_add(buffer.get_pos() as u64)
            .wrapping_mul(8);
        let mut s = self.state;
        buffer.len64_padding_le(bit_len, |b| compress(&mut s, convert(from_ref(b))));
        for (chunk, v) in out.chunks_exact_mut(4).zip(s.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}
impl Default for Md5Core {
    #[inline]
    fn default() -> Self {
        Self {
            block_len: 0,
            state: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476],
        }
    }
}
impl Reset for Md5Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for Md5Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md5")
    }
}
impl fmt::Debug for Md5Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md5Core { ... }")
    }
}
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Md5Core {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.2.5");
}
/// MD5 hasher state.
pub type Md5 = CoreWrapper<Md5Core>;
const BLOCK_SIZE: usize = <Md5Core as BlockSizeUser>::BlockSize::USIZE;
#[inline(always)]
fn convert(blocks: &[Block<Md5Core>]) -> &[[u8; BLOCK_SIZE]] {
    let p = blocks.as_ptr() as *const [u8; BLOCK_SIZE];
    unsafe { core::slice::from_raw_parts(p, blocks.len()) }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::Md5Core;
    use core::default::Default;
    #[test]
    fn md5core_default_test() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_md5core_default_test = 0;
        let md5core = Md5Core::default();
        debug_assert_eq!(md5core.block_len, 0);
        debug_assert_eq!(
            md5core.state, [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476]
        );
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_md5core_default_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use crate::Md5Core;
    use digest::Reset;
    use core::default::Default;
    #[test]
    fn test_reset() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_test_reset = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let mut core = Md5Core::default();
        core.state[rug_fuzz_0] = rug_fuzz_1;
        core.state[rug_fuzz_2] = rug_fuzz_3;
        core.state[rug_fuzz_4] = rug_fuzz_5;
        core.state[rug_fuzz_6] = rug_fuzz_7;
        core.block_len = rug_fuzz_8;
        core.reset();
        let default_core = Md5Core::default();
        debug_assert_eq!(core.state, default_core.state);
        debug_assert_eq!(core.block_len, default_core.block_len);
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use digest::{Digest, FixedOutput};
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_update_blocks = 0;
        let mut md5_core = Md5Core::default();
        let block = Block::<Md5Core>::default();
        let blocks = [block; 1];
        let initial_state = md5_core.state;
        Md5Core::update_blocks(&mut md5_core, &blocks);
        debug_assert_ne!(
            initial_state, md5_core.state,
            "State should be updated after processing a block"
        );
        debug_assert_eq!(
            md5_core.block_len, blocks.len() as u64,
            "block_len should be increased by the number of blocks"
        );
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::core_api::{BlockSizeUser, CoreProxy, CoreWrapper};
    #[test]
    fn test_convert() {
        let _rug_st_tests_rug_126_rrrruuuugggg_test_convert = 0;
        let block = GenericArray::<u8, <Md5Core as BlockSizeUser>::BlockSize>::default();
        let p0: &[Block<Md5Core>] = core::slice::from_ref(&block);
        crate::convert(p0);
        let _rug_ed_tests_rug_126_rrrruuuugggg_test_convert = 0;
    }
}
#[cfg(test)]
mod tests_rug_127 {
    use super::*;
    use crate::digest::core_api::{
        FixedOutputCore, BlockSizeUser, BufferKindUser, OutputSizeUser,
    };
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::{GenericArray, typenum::U16};
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_127_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0: Md5Core = Md5Core::default();
        let mut p1: BlockBuffer<
            <Md5Core as BlockSizeUser>::BlockSize,
            <Md5Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::<U64, Eager>::default();
        let mut p2: GenericArray<u8, <Md5Core as OutputSizeUser>::OutputSize> = GenericArray::<
            u8,
            U16,
        >::default();
        Md5Core::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_127_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
