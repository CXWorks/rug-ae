//! Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.
//!
//! ## Usage
//!
//! ### Simple usage
//!
//! For simple use-cases, you can call the [`hash()`] convenience function to
//! directly compute the CRC32 checksum for a given byte slice:
//!
//! ```rust
//! let checksum = crc32fast::hash(b"foo bar baz");
//! ```
//!
//! ### Advanced usage
//!
//! For use-cases that require more flexibility or performance, for example when
//! processing large amounts of data, you can create and manipulate a [`Hasher`]:
//!
//! ```rust
//! use crc32fast::Hasher;
//!
//! let mut hasher = Hasher::new();
//! hasher.update(b"foo bar baz");
//! let checksum = hasher.finalize();
//! ```
//!
//! ## Performance
//!
//! This crate contains multiple CRC32 implementations:
//!
//! - A fast baseline implementation which processes up to 16 bytes per iteration
//! - An optimized implementation for modern `x86` using `sse` and `pclmulqdq` instructions
//!
//! Calling the [`Hasher::new`] constructor at runtime will perform a feature detection to select the most
//! optimal implementation for the current CPU feature set.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(feature = "nightly", target_arch = "aarch64"),
    feature(stdsimd, aarch64_target_feature)
)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate cfg_if;
#[cfg(feature = "std")]
use std as core;
use core::fmt;
use core::hash;
mod baseline;
mod combine;
mod specialized;
mod table;
/// Computes the CRC32 hash of a byte slice.
///
/// Check out [`Hasher`] for more advanced use-cases.
pub fn hash(buf: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(buf);
    h.finalize()
}
#[derive(Clone)]
enum State {
    Baseline(baseline::State),
    Specialized(specialized::State),
}
#[derive(Clone)]
/// Represents an in-progress CRC32 computation.
pub struct Hasher {
    amount: u64,
    state: State,
}
const DEFAULT_INIT_STATE: u32 = 0;
impl Hasher {
    /// Create a new `Hasher`.
    ///
    /// This will perform a CPU feature detection at runtime to select the most
    /// optimal implementation for the current processor architecture.
    pub fn new() -> Self {
        Self::new_with_initial(DEFAULT_INIT_STATE)
    }
    /// Create a new `Hasher` with an initial CRC32 state.
    ///
    /// This works just like `Hasher::new`, except that it allows for an initial
    /// CRC32 state to be passed in.
    pub fn new_with_initial(init: u32) -> Self {
        Self::new_with_initial_len(init, 0)
    }
    /// Create a new `Hasher` with an initial CRC32 state.
    ///
    /// As `new_with_initial`, but also accepts a length (in bytes). The
    /// resulting object can then be used with `combine` to compute `crc(a ||
    /// b)` from `crc(a)`, `crc(b)`, and `len(b)`.
    pub fn new_with_initial_len(init: u32, amount: u64) -> Self {
        Self::internal_new_specialized(init, amount)
            .unwrap_or_else(|| Self::internal_new_baseline(init, amount))
    }
    #[doc(hidden)]
    pub fn internal_new_baseline(init: u32, amount: u64) -> Self {
        Hasher {
            amount,
            state: State::Baseline(baseline::State::new(init)),
        }
    }
    #[doc(hidden)]
    pub fn internal_new_specialized(init: u32, amount: u64) -> Option<Self> {
        {
            if let Some(state) = specialized::State::new(init) {
                return Some(Hasher {
                    amount,
                    state: State::Specialized(state),
                });
            }
        }
        None
    }
    /// Process the given byte slice and update the hash state.
    pub fn update(&mut self, buf: &[u8]) {
        self.amount += buf.len() as u64;
        match self.state {
            State::Baseline(ref mut state) => state.update(buf),
            State::Specialized(ref mut state) => state.update(buf),
        }
    }
    /// Finalize the hash state and return the computed CRC32 value.
    pub fn finalize(self) -> u32 {
        match self.state {
            State::Baseline(state) => state.finalize(),
            State::Specialized(state) => state.finalize(),
        }
    }
    /// Reset the hash state.
    pub fn reset(&mut self) {
        self.amount = 0;
        match self.state {
            State::Baseline(ref mut state) => state.reset(),
            State::Specialized(ref mut state) => state.reset(),
        }
    }
    /// Combine the hash state with the hash state for the subsequent block of bytes.
    pub fn combine(&mut self, other: &Self) {
        self.amount += other.amount;
        let other_crc = other.clone().finalize();
        match self.state {
            State::Baseline(ref mut state) => state.combine(other_crc, other.amount),
            State::Specialized(ref mut state) => state.combine(other_crc, other.amount),
        }
    }
}
impl fmt::Debug for Hasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("crc32fast::Hasher").finish()
    }
}
impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}
impl hash::Hasher for Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes)
    }
    fn finish(&self) -> u64 {
        u64::from(self.clone().finalize())
    }
}
#[cfg(test)]
mod test {
    use super::Hasher;
    quickcheck! {
        fn combine(bytes_1 : Vec < u8 >, bytes_2 : Vec < u8 >) -> bool { let mut hash_a =
        Hasher::new(); hash_a.update(& bytes_1); hash_a.update(& bytes_2); let mut hash_b
        = Hasher::new(); hash_b.update(& bytes_2); let mut hash_c = Hasher::new(); hash_c
        .update(& bytes_1); hash_c.combine(& hash_b); hash_a.finalize() == hash_c
        .finalize() } fn combine_from_len(bytes_1 : Vec < u8 >, bytes_2 : Vec < u8 >) ->
        bool { let mut hash_a = Hasher::new(); hash_a.update(& bytes_1); let a = hash_a
        .finalize(); let mut hash_b = Hasher::new(); hash_b.update(& bytes_2); let b =
        hash_b.finalize(); let mut hash_ab = Hasher::new(); hash_ab.update(& bytes_1);
        hash_ab.update(& bytes_2); let ab = hash_ab.finalize(); let mut reconstructed =
        Hasher::new_with_initial_len(a, bytes_1.len() as u64); let hash_b_reconstructed =
        Hasher::new_with_initial_len(b, bytes_2.len() as u64); reconstructed.combine(&
        hash_b_reconstructed); reconstructed.finalize() == ab }
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default_hasher() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_test_default_hasher = 0;
        let hasher = Hasher::default();
        let default_state = Hasher::new();
        debug_assert_eq!(format!("{:?}", hasher), format!("{:?}", default_state));
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_test_default_hasher = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use std::hash::Hasher as StdHasher;
    use super::*;
    use crate::*;
    #[test]
    fn test_finish_with_no_update() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_finish_with_no_update = 0;
        let hasher = Hasher::default();
        debug_assert_eq!(hasher.finish(), u64::from(Hasher::default().finalize()));
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_finish_with_no_update = 0;
    }
    #[test]
    fn test_finish_after_single_update() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Hasher::default();
        hasher.write(&[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let result = hasher.finish();
        debug_assert_eq!(result, u64::from(Hasher::default().finalize()));
             }
});    }
    #[test]
    fn test_finish_after_multiple_updates() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Hasher::default();
        hasher.write(&[rug_fuzz_0]);
        hasher.write(&[rug_fuzz_1]);
        hasher.write(&[rug_fuzz_2]);
        hasher.write(&[rug_fuzz_3]);
        let result = hasher.finish();
        debug_assert_eq!(result, u64::from(Hasher::default().finalize()));
             }
});    }
    #[test]
    fn test_finish_after_reset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Hasher::default();
        hasher.write(&[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        hasher.reset();
        debug_assert_eq!(hasher.finish(), u64::from(Hasher::default().finalize()));
             }
});    }
    #[test]
    fn test_finish_after_combine() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher1 = Hasher::default();
        hasher1.write(&[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        let mut hasher2 = Hasher::default();
        hasher2.write(&[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7]);
        hasher1.combine(&hasher2);
        let result = hasher1.finish();
        debug_assert_eq!(result, u64::from(Hasher::default().finalize()));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    #[test]
    fn test_hasher_combine() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u32, u32, u64, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_crc1 = rug_fuzz_0;
        let initial_crc2 = rug_fuzz_1;
        let length1 = rug_fuzz_2;
        let length2 = rug_fuzz_3;
        let mut hasher1 = Hasher::new_with_initial_len(initial_crc1, length1);
        let hasher2 = Hasher::new_with_initial_len(initial_crc2, length2);
        hasher1.combine(&hasher2);
        let combined_crc = hasher1.finalize();
        let expected_crc = rug_fuzz_4;
        debug_assert_eq!(
            combined_crc, expected_crc, "Combined CRC did not match the expected value"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    #[test]
    fn test_finalize() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_finalize = 0;
        let rug_fuzz_0 = b"hello";
        let rug_fuzz_1 = b"world";
        let mut hasher = Hasher::new();
        hasher.update(rug_fuzz_0);
        let hash = hasher.finalize();
        debug_assert_eq!(hash, 0x3610A686);
        hasher = Hasher::new();
        hasher.update(rug_fuzz_1);
        let hash = hasher.finalize();
        debug_assert_eq!(hash, 0x6A0BABBF);
        hasher = Hasher::new();
        let hash = hasher.finalize();
        debug_assert_eq!(hash, 0x0);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_finalize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6_llm_16_6 {
    use super::*;
    use crate::*;
    #[test]
    fn internal_new_baseline_initial_state() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let init = rug_fuzz_0;
        let amount = rug_fuzz_1;
        let hasher = Hasher::internal_new_baseline(init, amount);
        debug_assert_eq!(hasher.amount, amount);
        match hasher.state {
            State::Baseline(state) => debug_assert_eq!(state.finalize(), init),
            State::Specialized(_) => panic!("Expected baseline state, got specialized"),
        }
             }
});    }
    #[test]
    fn internal_new_baseline_amount() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let init = rug_fuzz_0;
        let amount = rug_fuzz_1;
        let hasher = Hasher::internal_new_baseline(init, amount);
        debug_assert_eq!(hasher.amount, amount);
             }
});    }
    #[test]
    fn internal_new_baseline_reset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Hasher::internal_new_baseline(rug_fuzz_0, rug_fuzz_1);
        hasher.reset();
        debug_assert_eq!(hasher.amount, 0);
        match hasher.state {
            State::Baseline(state) => debug_assert_eq!(state.finalize(), 0),
            State::Specialized(_) => panic!("Expected baseline state, got specialized"),
        }
             }
});    }
    #[test]
    fn internal_new_baseline_update_finalize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2_ext, mut rug_fuzz_3)) = <(u32, u64, [u8; 11], u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_2 = & rug_fuzz_2_ext;
        let mut hasher = Hasher::internal_new_baseline(rug_fuzz_0, rug_fuzz_1);
        let data = rug_fuzz_2;
        hasher.update(data);
        let result = hasher.finalize();
        debug_assert!(
            result != rug_fuzz_3, "Hash should not equal the initial state after update."
        );
             }
});    }
    #[test]
    fn internal_new_baseline_combine() {
        let _rug_st_tests_llm_16_6_llm_16_6_rrrruuuugggg_internal_new_baseline_combine = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0x12345678;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = b"hello world";
        let hasher1 = Hasher::internal_new_baseline(rug_fuzz_0, rug_fuzz_1);
        let mut hasher2 = Hasher::internal_new_baseline(rug_fuzz_2, rug_fuzz_3);
        hasher2.update(rug_fuzz_4);
        let initial_amount = hasher2.amount;
        hasher2.combine(&hasher1);
        debug_assert_eq!(hasher2.amount, initial_amount);
        let hasher1_final = hasher1.finalize();
        let hasher2_final = hasher2.finalize();
        debug_assert_eq!(
            hasher2_final, hasher1_final,
            "Hashes should be equal after combining with an empty hasher."
        );
        let _rug_ed_tests_llm_16_6_llm_16_6_rrrruuuugggg_internal_new_baseline_combine = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7_llm_16_7 {
    use super::*;
    use crate::*;
    #[test]
    fn test_internal_new_specialized_with_supported_algorithm() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let init = rug_fuzz_0;
        let amount = rug_fuzz_1;
        if let Some(hasher) = Hasher::internal_new_specialized(init, amount) {
            debug_assert_eq!(hasher.amount, amount);
            if let State::Specialized(_state) = hasher.state {} else {
                panic!("Expected specialized state");
            }
        } else {
            panic!("Expected to create specialized Hasher but got None");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_8_llm_16_8 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0_ext, mut rug_fuzz_1)) = <([u8; 9], u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
        let mut hasher = Hasher::new();
        hasher.update(rug_fuzz_0);
        let hash = hasher.finalize();
        let expected_hash = rug_fuzz_1;
        debug_assert_eq!(
            hash, expected_hash, "Hasher::new() did not produce the expected CRC32 hash"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    #[test]
    fn test_new_with_initial() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_new_with_initial = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = b"hello";
        let initial_crc = rug_fuzz_0;
        let mut hasher = Hasher::new_with_initial(initial_crc);
        hasher.update(rug_fuzz_1);
        let resulting_crc = hasher.finalize();
        debug_assert_ne!(initial_crc, resulting_crc);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_new_with_initial = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use crate::Hasher;
    #[test]
    fn test_new_with_initial_len() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u64, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let init_crc = rug_fuzz_0;
        let amount = rug_fuzz_1;
        let hasher = Hasher::new_with_initial_len(init_crc, amount);
        let hasher_default = Hasher::default();
        debug_assert_eq!(hasher.amount, amount);
        debug_assert_eq!(hasher.finalize(), hasher_default.finalize());
        let init_crc = rug_fuzz_2;
        let amount = rug_fuzz_3;
        let hasher = Hasher::new_with_initial_len(init_crc, amount);
        let mut hasher_default = Hasher::default();
        hasher_default.combine(&hasher);
        debug_assert_eq!(hasher.amount, amount);
        debug_assert_ne!(hasher.finalize(), hasher_default.finalize());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_11_llm_16_11 {
    use crate::Hasher;
    use std::hash::Hasher as StdHasher;
    #[test]
    fn reset_should_reset_amount_and_state() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u32, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut hasher = Hasher::new();
        hasher.write(&[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3]);
        hasher.reset();
        debug_assert_eq!(hasher.finish(), 0u64);
        hasher.write(&[rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7]);
        let mut new_hasher = Hasher::new_with_initial(rug_fuzz_8);
        new_hasher.write(&[rug_fuzz_9, rug_fuzz_10, rug_fuzz_11, rug_fuzz_12]);
        debug_assert_eq!(hasher.finish(), new_hasher.finish());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_23 {
    use crate::hash;
    #[test]
    fn test_hash_empty() {
        let _rug_st_tests_llm_16_23_rrrruuuugggg_test_hash_empty = 0;
        debug_assert_eq!(hash(& []), 0);
        let _rug_ed_tests_llm_16_23_rrrruuuugggg_test_hash_empty = 0;
    }
    #[test]
    fn test_hash_hello_world() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0_ext)) = <([u8; 11]) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_0 = & rug_fuzz_0_ext;
             }
});    }
    #[test]
    fn test_hash_single_byte() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(hash(& [rug_fuzz_0]), 0xd202ef8d);
             }
});    }
    #[test]
    fn test_hash_incrementing_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            hash(& [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9]), 0x7c9c7d0
        );
             }
});    }
    #[test]
    fn test_hash_repeating_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(hash(& [rug_fuzz_0; 256]), 0x29058c73);
             }
});    }
}
