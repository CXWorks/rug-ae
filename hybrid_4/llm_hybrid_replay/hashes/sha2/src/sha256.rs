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

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
             }
}
}
}    }
}
