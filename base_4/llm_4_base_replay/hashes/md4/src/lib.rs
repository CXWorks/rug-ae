//! An implementation of the [MD4][1] cryptographic hash algorithm.
//!
//! # Usage
//!
//! ```rust
//! use md4::{Md4, Digest};
//! use hex_literal::hex;
//!
//! // create a Md4 hasher instance
//! let mut hasher = Md4::new();
//!
//! // process input message
//! hasher.update(b"hello world");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 16]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("aa010fbc1d14c795d86ef98c95479d17"));
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
#![warn(rust_2018_idioms)]
#![allow(clippy::many_single_char_names)]
pub use digest::{self, Digest};
use core::{convert::TryInto, fmt};
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
#[derive(Clone)]
pub struct Md4Core {
    block_len: u64,
    state: [u32; 4],
}
impl HashMarker for Md4Core {}
impl BlockSizeUser for Md4Core {
    type BlockSize = U64;
}
impl BufferKindUser for Md4Core {
    type BufferKind = Eager;
}
impl OutputSizeUser for Md4Core {
    type OutputSize = U16;
}
impl UpdateCore for Md4Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len = self.block_len.wrapping_add(blocks.len() as u64);
        for block in blocks {
            compress(&mut self.state, block);
        }
    }
}
impl FixedOutputCore for Md4Core {
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
        let mut state = self.state;
        buffer.len64_padding_le(bit_len, |block| compress(&mut state, block));
        for (chunk, v) in out.chunks_exact_mut(4).zip(state.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}
impl Default for Md4Core {
    #[inline]
    fn default() -> Self {
        let state = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];
        Self { state, block_len: 0 }
    }
}
impl Reset for Md4Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for Md4Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md4")
    }
}
impl fmt::Debug for Md4Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Md4Core { ... }")
    }
}
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Md4Core {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.2.4");
}
/// MD4 hasher state.
pub type Md4 = CoreWrapper<Md4Core>;
fn compress(state: &mut [u32; 4], input: &Block<Md4Core>) {
    fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }
    fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (x & z) | (y & z)
    }
    fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }
    fn op1(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        a.wrapping_add(f(b, c, d)).wrapping_add(k).rotate_left(s)
    }
    fn op2(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        a.wrapping_add(g(b, c, d))
            .wrapping_add(k)
            .wrapping_add(0x5A82_7999)
            .rotate_left(s)
    }
    fn op3(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        a.wrapping_add(h(b, c, d))
            .wrapping_add(k)
            .wrapping_add(0x6ED9_EBA1)
            .rotate_left(s)
    }
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut data = [0u32; 16];
    for (o, chunk) in data.iter_mut().zip(input.chunks_exact(4)) {
        *o = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for &i in &[0, 4, 8, 12] {
        a = op1(a, b, c, d, data[i], 3);
        d = op1(d, a, b, c, data[i + 1], 7);
        c = op1(c, d, a, b, data[i + 2], 11);
        b = op1(b, c, d, a, data[i + 3], 19);
    }
    for i in 0..4 {
        a = op2(a, b, c, d, data[i], 3);
        d = op2(d, a, b, c, data[i + 4], 5);
        c = op2(c, d, a, b, data[i + 8], 9);
        b = op2(b, c, d, a, data[i + 12], 13);
    }
    for &i in &[0, 2, 1, 3] {
        a = op3(a, b, c, d, data[i], 3);
        d = op3(d, a, b, c, data[i + 8], 9);
        c = op3(c, d, a, b, data[i + 4], 11);
        b = op3(b, c, d, a, data[i + 12], 15);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::Md4Core;
    use core::default::Default;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_default = 0;
        let rug_fuzz_0 = 0x6745_2301;
        let rug_fuzz_1 = 0xEFCD_AB89;
        let rug_fuzz_2 = 0x98BA_DCFE;
        let rug_fuzz_3 = 0x1032_5476;
        let md4 = Md4Core::default();
        let expected_state = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        debug_assert_eq!(
            md4.state, expected_state,
            "The default state does not match the expected state."
        );
        debug_assert_eq!(md4.block_len, 0, "The default block length should be 0.");
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5_llm_16_5 {
    use super::*;
    use crate::*;
    use digest::core_api::Block;
    use digest::core_api::UpdateCore;
    use digest::core_api::BlockSizeUser;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_update_blocks = 0;
        let rug_fuzz_0 = 1u8;
        let mut md4_core = Md4Core::default();
        let initial_state = md4_core.state;
        let block_data = [rug_fuzz_0; <Md4Core as BlockSizeUser>::BlockSize::USIZE];
        let block = Block::<Md4Core>::from(block_data);
        md4_core.update_blocks(&[block; 1]);
        debug_assert_ne!(md4_core.state, initial_state);
        debug_assert_eq!(md4_core.block_len, 1);
        let blocks = [block; 2];
        md4_core.update_blocks(&blocks);
        debug_assert_eq!(md4_core.block_len, 3);
        debug_assert_ne!(md4_core.state, initial_state);
        let _rug_ed_tests_llm_16_5_llm_16_5_rrrruuuugggg_test_update_blocks = 0;
    }
}
