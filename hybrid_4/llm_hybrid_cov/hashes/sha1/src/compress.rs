use crate::{Block, BlockSizeUser, Sha1Core};
use digest::typenum::Unsigned;
cfg_if::cfg_if! {
    if #[cfg(feature = "force-soft")] { mod soft; use soft::compress as compress_inner; }
    else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] { mod soft; mod
    aarch64; use aarch64::compress as compress_inner; } else if #[cfg(any(target_arch =
    "x86", target_arch = "x86_64"))] { #[cfg(not(feature = "asm"))] mod soft;
    #[cfg(feature = "asm")] mod soft { pub use sha1_asm::compress; } mod x86; use
    x86::compress as compress_inner; } else { mod soft; use soft::compress as
    compress_inner; }
}
const BLOCK_SIZE: usize = <Sha1Core as BlockSizeUser>::BlockSize::USIZE;
/// SHA-1 compression function
#[cfg_attr(docsrs, doc(cfg(feature = "compress")))]
pub fn compress(state: &mut [u32; 5], blocks: &[Block<Sha1Core>]) {
    let blocks: &[[u8; BLOCK_SIZE]] = unsafe {
        &*(blocks as *const _ as *const [[u8; BLOCK_SIZE]])
    };
    compress_inner(state, blocks);
}
#[cfg(test)]
mod tests_rug_176 {
    use super::*;
    use crate::Sha1Core;
    use digest::core_api::BlockSizeUser;
    use digest::generic_array::GenericArray;
    #[test]
    fn test_compress() {
        let _rug_st_tests_rug_176_rrrruuuugggg_test_compress = 0;
        let rug_fuzz_0 = 0x67452301;
        let rug_fuzz_1 = 0xEFCDAB89;
        let rug_fuzz_2 = 0x98BADCFE;
        let rug_fuzz_3 = 0x10325476;
        let rug_fuzz_4 = 0xC3D2E1F0;
        let mut p0: [u32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let mut p1: [GenericArray<u8, <Sha1Core as BlockSizeUser>::BlockSize>; 1] = [
            GenericArray::<u8, <Sha1Core as BlockSizeUser>::BlockSize>::default(),
        ];
        crate::compress::compress(&mut p0, &p1);
        let _rug_ed_tests_rug_176_rrrruuuugggg_test_compress = 0;
    }
}
