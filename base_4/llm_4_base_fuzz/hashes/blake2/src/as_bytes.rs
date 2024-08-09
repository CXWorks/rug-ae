use core::mem;
use core::slice;
#[allow(clippy::missing_safety_doc)]
pub unsafe trait Safe {}
pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
    fn as_mut_bytes(&mut self) -> &mut [u8];
}
impl<T: Safe> AsBytes for [T] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * mem::size_of::<T>(),
            )
        }
    }
    #[inline]
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut u8,
                self.len() * mem::size_of::<T>(),
            )
        }
    }
}
unsafe impl Safe for u8 {}
unsafe impl Safe for u16 {}
unsafe impl Safe for u32 {}
unsafe impl Safe for u64 {}
unsafe impl Safe for i8 {}
unsafe impl Safe for i16 {}
unsafe impl Safe for i32 {}
unsafe impl Safe for i64 {}
#[cfg(test)]
mod tests_llm_16_17_llm_16_17 {
    use crate::AsBytes;
    use std::slice;
    #[test]
    fn test_as_bytes_with_u32_slice() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let slice: &[u32] = &[rug_fuzz_0, rug_fuzz_1];
        let bytes = AsBytes::as_bytes(slice);
        debug_assert_eq!(bytes, & [0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90]);
             }
});    }
    #[test]
    fn test_as_bytes_with_empty_slice() {
        let _rug_st_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_as_bytes_with_empty_slice = 0;
        let slice: &[u32] = &[];
        let bytes = AsBytes::as_bytes(slice);
        debug_assert_eq!(bytes, & []);
        let _rug_ed_tests_llm_16_17_llm_16_17_rrrruuuugggg_test_as_bytes_with_empty_slice = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;
    use crate::*;
    use std::slice;
    use std::mem;
    #[test]
    fn test_as_mut_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22)) = <(i32, i32, i32, i32, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, usize, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data: [i32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let data_len = data.len() * mem::size_of::<i32>();
        let byte_slice = data.as_mut_bytes();
        debug_assert_eq!(byte_slice.len(), data_len);
        let expected_bytes: [u8; 16] = [
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
        ];
        debug_assert_eq!(byte_slice, expected_bytes);
        byte_slice[rug_fuzz_20] = rug_fuzz_21;
        debug_assert_eq!(data[rug_fuzz_22], - 1);
             }
});    }
}
