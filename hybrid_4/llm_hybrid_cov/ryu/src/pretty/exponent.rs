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
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_positive_single_digit = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 1);
        debug_assert_eq!(buffer[rug_fuzz_2], b'5');
        debug_assert_eq!(buffer[rug_fuzz_3], 0);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_positive_single_digit = 0;
    }
    #[test]
    fn test_write_exponent2_positive_two_digits() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_positive_two_digits = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 2;
        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 2);
        debug_assert_eq!(& buffer[..rug_fuzz_2], b"10");
        debug_assert_eq!(buffer[rug_fuzz_3], 0);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_positive_two_digits = 0;
    }
    #[test]
    fn test_write_exponent2_negative_single_digit() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_negative_single_digit = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2;
        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 2);
        debug_assert_eq!(buffer[rug_fuzz_2], b'-');
        debug_assert_eq!(buffer[rug_fuzz_3], b'1');
        debug_assert_eq!(buffer[rug_fuzz_4], 0);
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_negative_single_digit = 0;
    }
    #[test]
    fn test_write_exponent2_negative_two_digits() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_negative_two_digits = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 3;
        let mut buffer = [rug_fuzz_0; 3];
        let len = unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
        debug_assert_eq!(len, 3);
        debug_assert_eq!(buffer[rug_fuzz_2], b'-');
        debug_assert_eq!(& buffer[rug_fuzz_3..rug_fuzz_4], b"10");
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_negative_two_digits = 0;
    }
    #[test]
    #[should_panic]
    fn test_write_exponent2_panic_on_large_negative() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_panic_on_large_negative = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 100;
        let mut buffer = [rug_fuzz_0; 3];
        unsafe { write_exponent2(-rug_fuzz_1, buffer.as_mut_ptr()) };
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_panic_on_large_negative = 0;
    }
    #[test]
    #[should_panic]
    fn test_write_exponent2_panic_on_large_positive() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_panic_on_large_positive = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 100;
        let mut buffer = [rug_fuzz_0; 3];
        unsafe { write_exponent2(rug_fuzz_1, buffer.as_mut_ptr()) };
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_write_exponent2_panic_on_large_positive = 0;
    }
}
#[cfg(test)]
mod tests_rug_17 {
    use std::mem;
    use std::ptr;
    use super::*;
    #[test]
    fn test_write_exponent3() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_write_exponent3 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 56;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 0;
        let mut buffer: [u8; 4] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        let mut p0: isize = rug_fuzz_0;
        let mut p1: *mut u8 = buffer.as_mut_ptr();
        let written = unsafe { crate::pretty::exponent::write_exponent3(p0, p1) };
        debug_assert_eq!(written, 4);
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_1) } as char, '1');
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_2) } as char, '2');
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_3) } as char, '3');
        p0 = -rug_fuzz_4;
        p1 = buffer.as_mut_ptr();
        let written = unsafe { crate::pretty::exponent::write_exponent3(p0, p1) };
        debug_assert_eq!(written, 3);
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_5) } as char, '-');
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_6) } as char, '5');
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_7) } as char, '6');
        p0 = rug_fuzz_8;
        let written = unsafe { crate::pretty::exponent::write_exponent3(p0, p1) };
        debug_assert_eq!(written, 2);
        debug_assert_eq!(unsafe { * p1.offset(rug_fuzz_9) } as char, '7');
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_write_exponent3 = 0;
    }
}
