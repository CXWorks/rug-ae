//! An implementation of the [MD2][1] cryptographic hash algorithm.
//!
//! # Usage
//!
//! ```rust
//! use md2::{Md2, Digest};
//! use hex_literal::hex;
//!
//! // create a Md2 hasher instance
//! let mut hasher = Md2::new();
//!
//! // process input message
//! hasher.update(b"hello world");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 16]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("d9cce882ee690a5c1ce70beff3a78c77"));
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/MD4
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
#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};
use digest::{
    block_buffer::Eager, consts::U16,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    HashMarker, Output,
};
mod consts;
/// Core MD2 hasher state.
#[derive(Clone)]
pub struct Md2Core {
    x: [u8; 48],
    checksum: Block<Self>,
}
impl Md2Core {
    fn compress(&mut self, block: &Block<Self>) {
        for j in 0..16 {
            self.x[16 + j] = block[j];
            self.x[32 + j] = self.x[16 + j] ^ self.x[j];
        }
        let mut t = 0u8;
        for j in 0..18u8 {
            for k in 0..48 {
                self.x[k] ^= consts::S[t as usize];
                t = self.x[k];
            }
            t = t.wrapping_add(j);
        }
        let mut l = self.checksum[15];
        for j in 0..16 {
            self.checksum[j] ^= consts::S[(block[j] ^ l) as usize];
            l = self.checksum[j];
        }
    }
}
impl HashMarker for Md2Core {}
impl BlockSizeUser for Md2Core {
    type BlockSize = U16;
}
impl BufferKindUser for Md2Core {
    type BufferKind = Eager;
}
impl OutputSizeUser for Md2Core {
    type OutputSize = U16;
}
impl UpdateCore for Md2Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        for block in blocks {
            self.compress(block)
        }
    }
}
impl FixedOutputCore for Md2Core {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let pos = buffer.get_pos();
        let rem = buffer.remaining() as u8;
        let block = buffer.pad_with_zeros();
        block[pos..].iter_mut().for_each(|b| *b = rem);
        self.compress(block);
        let checksum = self.checksum;
        self.compress(&checksum);
        out.copy_from_slice(&self.x[0..16]);
    }
}
impl Default for Md2Core {
    #[inline]
    fn default() -> Self {
        Self {
            x: [0; 48],
            checksum: Default::default(),
        }
    }
}
impl Reset for Md2Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for Md2Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md2")
    }
}
impl fmt::Debug for Md2Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md2Core { ... }")
    }
}
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Md2Core {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.2.2");
}
/// MD2 hasher state.
pub type Md2 = CoreWrapper<Md2Core>;
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    use digest::Digest;
    use digest::generic_array::typenum::U16;
    use digest::generic_array::GenericArray;
    #[test]
    fn md2core_default_test() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_md2core_default_test = 0;
        let md2core: Md2Core = Default::default();
        debug_assert_eq!(md2core.x, [0u8; 48]);
        let expected_checksum: GenericArray<u8, U16> = Default::default();
        debug_assert_eq!(md2core.checksum, expected_checksum);
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_md2core_default_test = 0;
    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {
        let _rug_st_tests_rug_129_rrrruuuugggg_test_compress = 0;
        let mut p0 = Md2Core::default();
        let mut p1 = GenericArray::<
            u8,
            <Md2Core as BlockSizeUser>::BlockSize,
        >::default();
        Md2Core::compress(&mut p0, &p1);
        let _rug_ed_tests_rug_129_rrrruuuugggg_test_compress = 0;
    }
}
#[cfg(test)]
mod tests_rug_130 {
    use super::*;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use digest::generic_array::GenericArray;
    use crate::Md2Core;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_130_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Md2Core::default();
        let block = GenericArray::<u8, <Md2Core as BlockSizeUser>::BlockSize>::default();
        let blocks = [block; 16];
        <Md2Core as UpdateCore>::update_blocks(&mut p0, &blocks);
        let _rug_ed_tests_rug_130_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::Md2Core;
    use crate::digest::core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::{GenericArray, typenum::U16};
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_131_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0: Md2Core = Md2Core::default();
        let mut p1: BlockBuffer<
            <Md2Core as BlockSizeUser>::BlockSize,
            <Md2Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::<_, _>::default();
        let mut p2: GenericArray<u8, U16> = Default::default();
        <Md2Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_131_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_132 {
    use super::*;
    use digest::Reset;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_132_rrrruuuugggg_test_rug = 0;
        let mut p0: Md2Core = Md2Core::default();
        <Md2Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_132_rrrruuuugggg_test_rug = 0;
    }
}
