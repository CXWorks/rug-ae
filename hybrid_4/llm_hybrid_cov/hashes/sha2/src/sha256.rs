use digest::{generic_array::GenericArray, typenum::U64};
cfg_if::cfg_if! {
    if #[cfg(feature = "force-soft")] { mod soft; use soft::compress; } else if
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] { #[cfg(not(feature =
    "asm"))] mod soft; #[cfg(feature = "asm")] mod soft { pub (crate) use
    sha2_asm::compress256 as compress; } mod x86; use x86::compress; } else if
    #[cfg(all(feature = "asm", target_arch = "aarch64"))] { mod soft; mod aarch64; use
    aarch64::compress; } else { mod soft; use soft::compress; }
}
/// Raw SHA-256 compression function.
///
/// This is a low-level "hazmat" API which provides direct access to the core
/// functionality of SHA-256.
#[cfg_attr(docsrs, doc(cfg(feature = "compress")))]
pub fn compress256(state: &mut [u32; 8], blocks: &[GenericArray<u8, U64>]) {
    let p = blocks.as_ptr() as *const [u8; 64];
    let blocks = unsafe { core::slice::from_raw_parts(p, blocks.len()) };
    compress(state, blocks)
}
#[cfg(test)]
mod tests_rug_194 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::typenum::{B0, B1, UInt, UTerm};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_194_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0x6a09e667;
        let rug_fuzz_1 = 0xbb67ae85;
        let rug_fuzz_2 = 0x3c6ef372;
        let rug_fuzz_3 = 0xa54ff53a;
        let rug_fuzz_4 = 0x510e527f;
        let rug_fuzz_5 = 0x9b05688c;
        let rug_fuzz_6 = 0x1f83d9ab;
        let rug_fuzz_7 = 0x5be0cd19;
        let mut p0: [u32; 8] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let mut p1: [GenericArray<
            u8,
            UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>,
        >; 1] = [GenericArray::default()];
        crate::sha256::compress256(&mut p0, &p1);
        let _rug_ed_tests_rug_194_rrrruuuugggg_test_rug = 0;
    }
}
