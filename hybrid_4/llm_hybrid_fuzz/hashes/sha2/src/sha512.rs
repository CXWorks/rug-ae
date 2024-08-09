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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u64, u64, u64, u64, u64, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
});    }
}
