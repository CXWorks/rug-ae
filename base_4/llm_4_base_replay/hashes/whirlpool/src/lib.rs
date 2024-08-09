//! An implementation of the [Whirlpool][1] cryptographic hash algorithm.
//!
//! This is the algorithm recommended by NESSIE (New European Schemes for
//! Signatures, Integrity and Encryption; an European research project).
//!
//! The constants used by Whirlpool were changed twice (2001 and 2003) - this
//! crate only implements the most recent standard. The two older Whirlpool
//! implementations (sometimes called Whirlpool-0 (pre 2001) and Whirlpool-T
//! (pre 2003)) were not used much anyway (both have never been recommended
//! by NESSIE).
//!
//! For details see [http://www.larc.usp.br/~pbarreto/WhirlpoolPage.html](https://web.archive.org/web/20171129084214/http://www.larc.usp.br/~pbarreto/WhirlpoolPage.html).
//!
//! # Usage
//!
//! ```rust
//! use whirlpool::{Whirlpool, Digest};
//! use hex_literal::hex;
//!
//! // create a hasher object, to use it do not forget to import `Digest` trait
//! let mut hasher = Whirlpool::new();
//! // write input message
//! hasher.update(b"Hello Whirlpool");
//! // read hash digest (it will consume hasher)
//! let result = hasher.finalize();
//!
//! assert_eq!(result[..], hex!("
//!     8eaccdc136903c458ea0b1376be2a5fc9dc5b8ce8892a3b4f43366e2610c206c
//!     a373816495e63db0fff2ff25f75aa7162f332c9f518c3036456502a8414d300a
//! ")[..]);
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/Whirlpool_(hash_function)
//! [2]: https://github.com/RustCrypto/hashes
#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![warn(missing_docs, rust_2018_idioms)]
pub use digest::{self, Digest};
#[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
mod compress;
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use whirlpool_asm as compress;
use compress::compress;
use core::fmt;
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U64},
    HashMarker, Output,
};
/// Core Whirlpool hasher state.
#[derive(Clone)]
pub struct WhirlpoolCore {
    bit_len: [u64; 4],
    state: [u64; 8],
}
impl HashMarker for WhirlpoolCore {}
impl BlockSizeUser for WhirlpoolCore {
    type BlockSize = U64;
}
impl BufferKindUser for WhirlpoolCore {
    type BufferKind = Eager;
}
impl OutputSizeUser for WhirlpoolCore {
    type OutputSize = U64;
}
impl UpdateCore for WhirlpoolCore {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        let block_bits = 8 * BLOCK_SIZE as u64;
        self.update_len(block_bits * (blocks.len() as u64));
        compress(&mut self.state, convert(blocks));
    }
}
impl FixedOutputCore for WhirlpoolCore {
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut Output<Self>,
    ) {
        let pos = buffer.get_pos();
        self.update_len(8 * pos as u64);
        let mut buf = [0u8; 4 * 8];
        for (chunk, v) in buf.chunks_exact_mut(8).zip(self.bit_len.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
        let mut state = self.state;
        buffer
            .digest_pad(
                0x80,
                &buf,
                |block| {
                    compress(&mut state, convert(core::slice::from_ref(block)));
                },
            );
        for (chunk, v) in out.chunks_exact_mut(8).zip(state.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
    }
}
impl WhirlpoolCore {
    fn update_len(&mut self, len: u64) {
        let mut carry = 0;
        adc(&mut self.bit_len[3], len, &mut carry);
        adc(&mut self.bit_len[2], 0, &mut carry);
        adc(&mut self.bit_len[1], 0, &mut carry);
        adc(&mut self.bit_len[0], 0, &mut carry);
    }
}
#[allow(clippy::derivable_impls)]
impl Default for WhirlpoolCore {
    #[inline]
    fn default() -> Self {
        Self {
            bit_len: Default::default(),
            state: [0u64; 8],
        }
    }
}
impl Reset for WhirlpoolCore {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}
impl AlgorithmName for WhirlpoolCore {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Whirlpool")
    }
}
impl fmt::Debug for WhirlpoolCore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WhirlpoolCore { ... }")
    }
}
/// Whirlpool hasher state.
pub type Whirlpool = CoreWrapper<WhirlpoolCore>;
#[inline(always)]
fn adc(a: &mut u64, b: u64, carry: &mut u64) {
    let ret = (*a as u128) + (b as u128) + (*carry as u128);
    *a = ret as u64;
    *carry = (ret >> 64) as u64;
}
const BLOCK_SIZE: usize = <WhirlpoolCore as BlockSizeUser>::BlockSize::USIZE;
#[inline(always)]
fn convert(blocks: &[Block<WhirlpoolCore>]) -> &[[u8; BLOCK_SIZE]] {
    let p = blocks.as_ptr() as *const [u8; BLOCK_SIZE];
    unsafe { core::slice::from_raw_parts(p, blocks.len()) }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    use core::default::Default;
    #[test]
    fn default_initializes_zeroed() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_default_initializes_zeroed = 0;
        let whirlpool_core = WhirlpoolCore::default();
        debug_assert_eq!(whirlpool_core.bit_len, [0u64; 4]);
        debug_assert_eq!(whirlpool_core.state, [0u64; 8]);
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_default_initializes_zeroed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    #[test]
    fn update_len_adds_length_properly() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_update_len_adds_length_properly = 0;
        let rug_fuzz_0 = 123456789;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let mut core = WhirlpoolCore::default();
        let initial_len = core.bit_len;
        let len_to_add = rug_fuzz_0;
        core.update_len(len_to_add);
        let mut expected_len = initial_len;
        expected_len[rug_fuzz_1] += len_to_add;
        debug_assert_eq!(
            core.bit_len, expected_len,
            "Length should be added to the last element of the array"
        );
        core.reset();
        let len_to_add = u64::MAX;
        core.update_len(len_to_add);
        core.update_len(rug_fuzz_2);
        let mut expected_len = initial_len;
        expected_len[rug_fuzz_3] = rug_fuzz_4;
        expected_len[rug_fuzz_5] += rug_fuzz_6;
        debug_assert_eq!(
            core.bit_len, expected_len,
            "Carry should propagate to the next element of the array"
        );
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_update_len_adds_length_properly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use crate::adc;
    #[test]
    fn test_adc_simple_addition() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_adc_simple_addition = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 0;
        let mut a: u64 = rug_fuzz_0;
        let b: u64 = rug_fuzz_1;
        let mut carry: u64 = rug_fuzz_2;
        adc(&mut a, b, &mut carry);
        debug_assert_eq!(a, 3);
        debug_assert_eq!(carry, 0);
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_adc_simple_addition = 0;
    }
    #[test]
    fn test_adc_with_carry() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_adc_with_carry = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let mut a: u64 = u64::MAX;
        let b: u64 = rug_fuzz_0;
        let mut carry: u64 = rug_fuzz_1;
        adc(&mut a, b, &mut carry);
        debug_assert_eq!(a, 0);
        debug_assert_eq!(carry, 1);
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_adc_with_carry = 0;
    }
    #[test]
    fn test_adc_large_numbers() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_adc_large_numbers = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let mut a: u64 = u64::MAX - rug_fuzz_0;
        let b: u64 = rug_fuzz_1;
        let mut carry: u64 = rug_fuzz_2;
        adc(&mut a, b, &mut carry);
        debug_assert_eq!(a, 1);
        debug_assert_eq!(carry, 1);
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_adc_large_numbers = 0;
    }
    #[test]
    fn test_adc_zero_addition() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_adc_zero_addition = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let mut a: u64 = rug_fuzz_0;
        let b: u64 = rug_fuzz_1;
        let mut carry: u64 = rug_fuzz_2;
        adc(&mut a, b, &mut carry);
        debug_assert_eq!(a, 0);
        debug_assert_eq!(carry, 0);
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_adc_zero_addition = 0;
    }
    #[test]
    fn test_adc_random_addition() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_adc_random_addition = 0;
        let rug_fuzz_0 = 123456789;
        let rug_fuzz_1 = 987654321;
        let rug_fuzz_2 = 0;
        let mut a: u64 = rug_fuzz_0;
        let b: u64 = rug_fuzz_1;
        let mut carry: u64 = rug_fuzz_2;
        adc(&mut a, b, &mut carry);
        debug_assert_eq!(a, 1111111110);
        debug_assert_eq!(carry, 0);
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_adc_random_addition = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10_llm_16_10 {
    use super::*;
    use crate::*;
    use digest::generic_array::GenericArray;
    use digest::generic_array::typenum::U64;
    use crate::BLOCK_SIZE;
    #[test]
    fn test_convert_empty() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_empty = 0;
        let blocks: &[GenericArray<u8, U64>] = &[];
        let converted = convert(blocks);
        debug_assert_eq!(converted.len(), 0);
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_empty = 0;
    }
    #[test]
    fn test_convert_single() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_single = 0;
        let rug_fuzz_0 = 0;
        let block = GenericArray::<u8, U64>::default();
        let blocks = &[block];
        let converted = convert(blocks);
        debug_assert_eq!(converted.len(), 1);
        debug_assert_eq!(converted[rug_fuzz_0], [0u8; BLOCK_SIZE]);
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_single = 0;
    }
    #[test]
    fn test_convert_multiple() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_multiple = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let block = GenericArray::<u8, U64>::default();
        let blocks = &[block, block];
        let converted = convert(blocks);
        debug_assert_eq!(converted.len(), 2);
        debug_assert_eq!(converted[rug_fuzz_0], [0u8; BLOCK_SIZE]);
        debug_assert_eq!(converted[rug_fuzz_1], [0u8; BLOCK_SIZE]);
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_multiple = 0;
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_convert_invalid_alignment() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_invalid_alignment = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let misaligned_block = [rug_fuzz_0; BLOCK_SIZE + 1];
        let misaligned_blocks = &misaligned_block[rug_fuzz_1..BLOCK_SIZE];
        let blocks = unsafe {
            core::slice::from_raw_parts(
                misaligned_blocks.as_ptr() as *const GenericArray<u8, U64>,
                rug_fuzz_2,
            )
        };
        let _converted = convert(blocks);
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_convert_invalid_alignment = 0;
    }
}
