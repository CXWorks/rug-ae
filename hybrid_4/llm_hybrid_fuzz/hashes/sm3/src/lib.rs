//! An implementation of the [SM3] cryptographic hash function defined
//! in OSCCA GM/T 0004-2012.
//!
//! # Usage
//! Hasher functionality is expressed via traits defined in the [`digest`]
//! crate.
//!
//! ```rust
//! use hex_literal::hex;
//! use sm3::{Digest, Sm3};
//!
//! // create a hasher object, to use it do not forget to import `Digest` trait
//! let mut hasher = Sm3::new();
//!
//! // write input message
//! hasher.update(b"hello world");
//!
//! // read hash digest and consume hasher
//! let result = hasher.finalize();
//!
//! assert_eq!(result[..], hex!("
//!     44f0061e69fa6fdfc290c494654a05dc0c053da7e5c52b84ef93a9d67d3fff88
//! ")[..]);
//! ```
//!
//! Also see [RustCrypto/hashes] readme.
//!
//! [SM3]: https://en.wikipedia.org/wiki/SM3_(hash_function)
//! [RustCrypto/hashes]: https://github.com/RustCrypto/hashes
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
pub use digest::{self, Digest};
use core::{fmt, slice::from_ref};
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U32, U64},
    HashMarker, Output,
};
mod compress;
mod consts;
use compress::compress;
/// Core SM3 hasher state.
#[derive(Clone)]
pub struct Sm3Core {
    block_len: u64,
    h: [u32; 8],
}
impl HashMarker for Sm3Core {}
impl BlockSizeUser for Sm3Core {
    type BlockSize = U64;
}
impl BufferKindUser for Sm3Core {
    type BufferKind = Eager;
}
impl OutputSizeUser for Sm3Core {
    type OutputSize = U32;
}
impl UpdateCore for Sm3Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        compress(&mut self.h, blocks);
    }
}
impl FixedOutputCore for Sm3Core {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let bs = Self::BlockSize::U64;
        let bit_len = 8 * (buffer.get_pos() as u64 + bs * self.block_len);
        let mut h = self.h;
        buffer.len64_padding_be(bit_len, |b| compress(&mut h, from_ref(b)));
        for (chunk, v) in out.chunks_exact_mut(4).zip(h.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}
impl Default for Sm3Core {
    #[inline]
    fn default() -> Self {
        Self {
            h: consts::H0,
            block_len: 0,
        }
    }
}
impl Reset for Sm3Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for Sm3Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sm3")
    }
}
impl fmt::Debug for Sm3Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sm3Core { ... }")
    }
}
/// Sm3 hasher state.
pub type Sm3 = CoreWrapper<Sm3Core>;
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    use digest::Digest;
    #[test]
    fn default_initializes_properly() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_default_initializes_properly = 0;
        let sm3 = Sm3Core::default();
        debug_assert_eq!(sm3.block_len, 0);
        debug_assert_eq!(sm3.h, consts::H0);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_default_initializes_properly = 0;
    }
}
#[cfg(test)]
mod tests_rug_305 {
    use super::*;
    use crate::Sm3Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_305_rrrruuuugggg_test_rug = 0;
        let mut p0 = Sm3Core::default();
        let mut p1 = [
            GenericArray::<u8, <Sm3Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Sm3Core as UpdateCore>::update_blocks(&mut p0, &mut p1);
        let _rug_ed_tests_rug_305_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_306 {
    use super::*;
    use crate::digest::core_api::{FixedOutputCore, BlockSizeUser, BufferKindUser};
    use crate::digest::block_buffer::BlockBuffer;
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_306_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Sm3Core::default();
        let mut p1 = BlockBuffer::<
            <Sm3Core as BlockSizeUser>::BlockSize,
            <Sm3Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Sm3Core as OutputSizeUser>::OutputSize,
        >::default();
        <Sm3Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_306_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_307 {
    use super::*;
    use crate::digest::Reset;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_307_rrrruuuugggg_test_rug = 0;
        let mut p0: Sm3Core = Sm3Core::default();
        <Sm3Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_307_rrrruuuugggg_test_rug = 0;
    }
}
