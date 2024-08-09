use digest::{generic_array::GenericArray, typenum::U128};
cfg_if::cfg_if! {
    if #[cfg(feature = "force-soft")] { mod soft; use soft::compress; } else if
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] { #[cfg(not(feature =
    "asm"))] mod soft; #[cfg(feature = "asm")] mod soft { pub (crate) fn compress(state :
    & mut [u64; 8], blocks : & [[u8; 128]]) { sha2_asm::compress512(state, blocks); } }
    mod x86; use x86::compress; } else if #[cfg(all(feature = "asm", target_arch =
    "aarch64"))] { mod soft; mod aarch64; use aarch64::compress; } else { mod soft; use
    soft::compress; }
}
/// Raw SHA-512 compression function.
///
/// This is a low-level "hazmat" API which provides direct access to the core
/// functionality of SHA-512.
#[cfg_attr(docsrs, doc(cfg(feature = "compress")))]
pub fn compress512(state: &mut [u64; 8], blocks: &[GenericArray<u8, U128>]) {
    let p = blocks.as_ptr() as *const [u8; 128];
    let blocks = unsafe { core::slice::from_raw_parts(p, blocks.len()) };
    compress(state, blocks)
}
#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::typenum::{U128, U64, UInt, UTerm, B0, B1};
    #[test]
    fn test_compress512() {
        let _rug_st_tests_rug_214_rrrruuuugggg_test_compress512 = 0;
        let rug_fuzz_0 = 0x6a09e667f3bcc908;
        let rug_fuzz_1 = 0xbb67ae8584caa73b;
        let rug_fuzz_2 = 0x3c6ef372fe94f82b;
        let rug_fuzz_3 = 0xa54ff53a5f1d36f1;
        let rug_fuzz_4 = 0x510e527fade682d1;
        let rug_fuzz_5 = 0x9b05688c2b3e6c1f;
        let rug_fuzz_6 = 0x1f83d9abfb41bd6b;
        let rug_fuzz_7 = 0x5be0cd19137e2179;
        let mut p0: [u64; 8] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let block = vec![42u8; 128];
        let block_generic_array = GenericArray::<u8, U128>::clone_from_slice(&block);
        let p1: &[GenericArray<u8, U128>] = &[block_generic_array];
        crate::sha512::compress512(&mut p0, &p1);
        let _rug_ed_tests_rug_214_rrrruuuugggg_test_compress512 = 0;
    }
}
