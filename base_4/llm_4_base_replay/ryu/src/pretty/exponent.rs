use crate::digit_table::*;
use core::ptr;
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent3(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }
    debug_assert!(k < 1000);
    if k >= 100 {
        *result = b'0' + (k / 100) as u8;
        k %= 100;
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result.offset(1), 2);
        sign as usize + 3
    } else if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent2(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }
    debug_assert!(k < 100);
    if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    #[test]
    fn test_write_exponent2_positive_single_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, isize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 1);
        debug_assert_eq!(buffer[rug_fuzz_2], b'5');
        debug_assert_eq!(buffer[rug_fuzz_3], 0);
             }
}
}
}    }
    #[test]
    fn test_write_exponent2_positive_two_digits() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, isize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 2);
        debug_assert_eq!(& buffer[..rug_fuzz_2], b"10");
        debug_assert_eq!(buffer[rug_fuzz_3], 0);
             }
}
}
}    }
    #[test]
    fn test_write_exponent2_negative_single_digit() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, isize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 2);
        debug_assert_eq!(buffer[rug_fuzz_2], b'-');
        debug_assert_eq!(buffer[rug_fuzz_3], b'1');
        debug_assert_eq!(buffer[rug_fuzz_4], 0);
             }
}
}
}    }
    #[test]
    fn test_write_exponent2_negative_two_digits() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, isize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 3);
        debug_assert_eq!(buffer[rug_fuzz_2], b'-');
        debug_assert_eq!(& buffer[rug_fuzz_3..rug_fuzz_4], b"10");
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_write_exponent2_panic_on_large_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_write_exponent2_panic_on_large_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 3];
        unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
             }
}
}
}    }
}
