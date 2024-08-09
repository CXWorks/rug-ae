//! An implementation of the [RIPEMD] cryptographic hash.
//!
//! This crate implements only the modified 1996 versions, not the original
//! one from 1992.
//!
//! Note that RIPEMD-256 provides only the same security as RIPEMD-128,
//! and RIPEMD-320 provides only the same security as RIPEMD-160.
//!
//! # Usage
//!
//! ```rust
//! use hex_literal::hex;
//! use ripemd::{Ripemd160, Ripemd320, Digest};
//!
//! // create a RIPEMD-160 hasher instance
//! let mut hasher = Ripemd160::new();
//!
//! // process input message
//! hasher.update(b"Hello world!");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 20]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("7f772647d88750add82d8e1a7a3e5c0902a346a3"));
//!
//! // same for RIPEMD-320
//! let mut hasher = Ripemd320::new();
//! hasher.update(b"Hello world!");
//! let result = hasher.finalize();
//! assert_eq!(&result[..], &hex!("
//!     f1c1c231d301abcf2d7daae0269ff3e7bc68e623
//!     ad723aa068d316b056d26b7d1bb6f0cc0f28336d
//! ")[..]);
//! ```
//!
//! Also see [RustCrypto/hashes] readme.
//!
//! [RIPEMD]: https://en.wikipedia.org/wiki/RIPEMD
//! [RustCrypto/hashes]: https://github.com/RustCrypto/hashes
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
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper,
        FixedOutputCore, OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U16, U20, U32, U40, U64},
    HashMarker, Output,
};
mod c128;
mod c160;
mod c256;
mod c320;
macro_rules! impl_ripemd {
    (
        $name:ident, $wrapped_name:ident, $mod:ident, $alg_width:expr, $doc_name:expr,
        $output_size:ty $(,)?
    ) => {
        #[doc = "Core block-level"] #[doc = $doc_name] #[doc = " hasher state."]
        #[derive(Clone)] pub struct $name { h : [u32; $mod ::DIGEST_BUF_LEN], block_len :
        u64, } impl HashMarker for $name {} impl BlockSizeUser for $name { type BlockSize
        = U64; } impl BufferKindUser for $name { type BufferKind = Eager; } impl
        OutputSizeUser for $name { type OutputSize = $output_size; } impl UpdateCore for
        $name { #[inline] fn update_blocks(& mut self, blocks : & [Block < Self >]) {
        self.block_len += blocks.len() as u64; for block in blocks { $mod ::compress(&
        mut self.h, block.as_ref()); } } } impl FixedOutputCore for $name { #[inline] fn
        finalize_fixed_core(& mut self, buffer : & mut Buffer < Self >, out : & mut
        Output < Self >) { let bs = Self::BlockSize::U64; let bit_len = 8 * (buffer
        .get_pos() as u64 + bs * self.block_len); let mut h = self.h; buffer
        .len64_padding_le(bit_len, | block | $mod ::compress(& mut h, block.as_ref()));
        for (chunk, v) in out.chunks_exact_mut(4).zip(h.iter()) { chunk.copy_from_slice(&
        v.to_le_bytes()); } } } impl Default for $name { #[inline] fn default() -> Self {
        Self { h : $mod ::H0, block_len : 0, } } } impl Reset for $name { #[inline] fn
        reset(& mut self) { * self = Default::default(); } } impl AlgorithmName for $name
        { #[inline] fn write_alg_name(f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!("Ripemd", $alg_width)) } } impl fmt::Debug for $name {
        #[inline] fn fmt(& self, f : & mut fmt::Formatter <'_ >) -> fmt::Result { f
        .write_str(concat!("Ripemd", $alg_width, "Core { ... }")) } } #[doc = $doc_name]
        #[doc = " hasher."] pub type $wrapped_name = CoreWrapper <$name >;
    };
}
impl_ripemd!(Ripemd128Core, Ripemd128, c128, "128", "RIPEMD-128", U16);
impl_ripemd!(Ripemd160Core, Ripemd160, c160, "160", "RIPEMD-160", U20);
impl_ripemd!(Ripemd256Core, Ripemd256, c256, "256", "RIPEMD-256", U32);
impl_ripemd!(Ripemd320Core, Ripemd320, c320, "320", "RIPEMD-320", U40);
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd128Core {
    /// The OID used for the RIPEMD-160. There are two OIDs defined. The Teletrust one (which is
    /// used by almost anybody, including BouncyCastle, OpenSSL, GnuTLS, etc. and the ISO one
    /// (1.0.10118.3.0.50), which seems to be used by nobody.
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.2");
}
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd160Core {
    /// The OID used for the RIPEMD-160. There are two OIDs defined. The Teletrust one (which is
    /// used by almost anybody, including BouncyCastle, OpenSSL, GnuTLS, etc. and the ISO one
    /// (1.0.10118.3.0.49), which seems to be used by Go and nobody else.
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.1");
}
#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd256Core {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.3");
}
#[cfg(test)]
mod tests_llm_16_6_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::c160;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_test_default = 0;
        let ripemd = <Ripemd160Core as core::default::Default>::default();
        debug_assert_eq!(ripemd.h, c160::H0);
        debug_assert_eq!(ripemd.block_len, 0);
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use digest::Reset;
    #[test]
    fn test_reset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut core = Ripemd160Core::default();
        core.block_len = rug_fuzz_0;
        core.h = [rug_fuzz_1; digest::consts::U5::USIZE];
        let initial_state = Ripemd160Core::default();
        core.reset();
        debug_assert_eq!(core.block_len, initial_state.block_len);
        debug_assert_eq!(core.h, initial_state.h);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_11_llm_16_11 {
    use super::*;
    use crate::*;
    use crate::c256::H0;
    #[test]
    fn test_ripemd256core_default() {
        let _rug_st_tests_llm_16_11_llm_16_11_rrrruuuugggg_test_ripemd256core_default = 0;
        let ripemd256 = Ripemd256Core::default();
        debug_assert_eq!(ripemd256.h, H0);
        debug_assert_eq!(ripemd256.block_len, 0);
        let _rug_ed_tests_llm_16_11_llm_16_11_rrrruuuugggg_test_ripemd256core_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_16 {
    use crate::c320::H0 as Ripemd320H0;
    use crate::Ripemd320Core;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_16_rrrruuuugggg_test_default = 0;
        let default_core = Ripemd320Core::default();
        debug_assert_eq!(default_core.block_len, 0);
        debug_assert_eq!(default_core.h, Ripemd320H0);
        let _rug_ed_tests_llm_16_16_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    use crate::digest::core_api::UpdateCore;
    use crate::digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_148_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Ripemd128Core::default();
        let mut p1 = [
            GenericArray::<u8, <Ripemd128Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Ripemd128Core as UpdateCore>::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_148_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_149 {
    use super::*;
    use crate::digest::core_api::{BlockSizeUser, BufferKindUser, FixedOutputCore};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_149_rrrruuuugggg_test_rug = 0;
        let mut p0: Ripemd128Core = Ripemd128Core::default();
        let mut p1: BlockBuffer<
            <Ripemd128Core as BlockSizeUser>::BlockSize,
            <Ripemd128Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<
            u8,
            <Ripemd128Core as digest::OutputSizeUser>::OutputSize,
        > = GenericArray::default();
        Ripemd128Core::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
        let _rug_ed_tests_rug_149_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::digest::Reset;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_151_rrrruuuugggg_test_rug = 0;
        let mut p0 = Ripemd128Core::default();
        <Ripemd128Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_151_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_153 {
    use crate::Ripemd160Core;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    use crate::digest::generic_array::typenum::Unsigned;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_153_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Ripemd160Core::default();
        let mut p1 = [GenericArray::<
            u8,
            <Ripemd160Core as BlockSizeUser>::BlockSize,
        >::default(); 1];
        Ripemd160Core::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_153_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use crate::digest::core_api::{FixedOutputCore, BufferKindUser, BlockSizeUser};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::GenericArray;
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_154_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0: Ripemd160Core = Ripemd160Core::default();
        let mut p1: BlockBuffer<
            <Ripemd160Core as BlockSizeUser>::BlockSize,
            <Ripemd160Core as BufferKindUser>::BufferKind,
        > = BlockBuffer::default();
        let mut p2: GenericArray<u8, <Ripemd160Core as OutputSizeUser>::OutputSize> = GenericArray::<
            u8,
            <Ripemd160Core as OutputSizeUser>::OutputSize,
        >::default();
        <Ripemd160Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_154_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_156 {
    use super::*;
    use digest::core_api::UpdateCore;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    use crate::Ripemd256Core;
    #[test]
    fn test_update_blocks() {
        let _rug_st_tests_rug_156_rrrruuuugggg_test_update_blocks = 0;
        let mut p0 = Ripemd256Core::default();
        let mut p1 = [
            GenericArray::<u8, <Ripemd256Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Ripemd256Core as UpdateCore>::update_blocks(&mut p0, &p1);
        let _rug_ed_tests_rug_156_rrrruuuugggg_test_update_blocks = 0;
    }
}
#[cfg(test)]
mod tests_rug_157 {
    use super::*;
    use digest::core_api::{BlockSizeUser, BufferKindUser, FixedOutputCore};
    use digest::generic_array::GenericArray;
    use digest::OutputSizeUser;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_157_rrrruuuugggg_test_rug = 0;
        let mut p0 = Ripemd256Core::default();
        let mut p1 = digest::block_buffer::BlockBuffer::<
            <Ripemd256Core as BlockSizeUser>::BlockSize,
            <Ripemd256Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Ripemd256Core as OutputSizeUser>::OutputSize,
        >::default();
        <Ripemd256Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_157_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_158 {
    use super::*;
    use crate::digest::Reset;
    use crate::Ripemd256Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_158_rrrruuuugggg_test_rug = 0;
        let mut p0: Ripemd256Core = Ripemd256Core::default();
        <Ripemd256Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_158_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_160 {
    use crate::Ripemd320Core;
    use digest::core_api::{BlockSizeUser, UpdateCore};
    use digest::generic_array::GenericArray;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_160_rrrruuuugggg_test_rug = 0;
        let mut p0 = Ripemd320Core::default();
        let mut p1 = [
            GenericArray::<u8, <Ripemd320Core as BlockSizeUser>::BlockSize>::default(),
        ];
        <Ripemd320Core as UpdateCore>::update_blocks(&mut p0, &mut p1);
        let _rug_ed_tests_rug_160_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_161 {
    use super::*;
    use crate::digest::core_api::{BlockSizeUser, BufferKindUser, FixedOutputCore};
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::{generic_array::GenericArray, OutputSizeUser};
    #[test]
    fn test_finalize_fixed_core() {
        let _rug_st_tests_rug_161_rrrruuuugggg_test_finalize_fixed_core = 0;
        let mut p0 = Ripemd320Core::default();
        let mut p1 = BlockBuffer::<
            <Ripemd320Core as BlockSizeUser>::BlockSize,
            <Ripemd320Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2 = GenericArray::<
            u8,
            <Ripemd320Core as OutputSizeUser>::OutputSize,
        >::default();
        <Ripemd320Core as FixedOutputCore>::finalize_fixed_core(
            &mut p0,
            &mut p1,
            &mut p2,
        );
        let _rug_ed_tests_rug_161_rrrruuuugggg_test_finalize_fixed_core = 0;
    }
}
#[cfg(test)]
mod tests_rug_162 {
    use super::*;
    use crate::digest::Reset;
    use crate::Ripemd320Core;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_162_rrrruuuugggg_test_rug = 0;
        let mut p0: Ripemd320Core = Ripemd320Core::default();
        <Ripemd320Core as Reset>::reset(&mut p0);
        let _rug_ed_tests_rug_162_rrrruuuugggg_test_rug = 0;
    }
}
