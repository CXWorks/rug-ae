use crate::digit_table::*;
use core::ptr;
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_mantissa_long(mut output: u64, mut result: *mut u8) {
    if (output >> 32) != 0 {
        let mut output2 = (output - 100_000_000 * (output / 100_000_000)) as u32;
        output /= 100_000_000;
        let c = output2 % 10_000;
        output2 /= 10_000;
        let d = output2 % 10_000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        let d0 = (d % 100) << 1;
        let d1 = (d / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(-2),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(-4),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d0 as isize),
            result.offset(-6),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d1 as isize),
            result.offset(-8),
            2,
        );
        result = result.offset(-8);
    }
    write_mantissa(output as u32, result);
}
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_mantissa(mut output: u32, mut result: *mut u8) {
    while output >= 10_000 {
        let c = output - 10_000 * (output / 10_000);
        output /= 10_000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(-2),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(-4),
            2,
        );
        result = result.offset(-4);
    }
    if output >= 100 {
        let c = (output % 100) << 1;
        output /= 100;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c as isize),
            result.offset(-2),
            2,
        );
        result = result.offset(-2);
    }
    if output >= 10 {
        let c = output << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c as isize),
            result.offset(-2),
            2,
        );
    } else {
        *result.offset(-1) = b'0' + output as u8;
    }
}
#[cfg(test)]
mod tests_llm_16_39_llm_16_39 {
    use super::*;
    use crate::*;
    use std::ptr;
    use std::mem;
    #[test]
    fn test_write_mantissa_single_digit() {
        const BUF_SIZE: usize = 5;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 1) as isize) };
        unsafe {
            write_mantissa(5, result);
            assert_eq!(* result, b'5');
        }
    }
    #[test]
    fn test_write_mantissa_two_digits() {
        const BUF_SIZE: usize = 5;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 2) as isize) };
        unsafe {
            write_mantissa(42, result);
            assert_eq!(* result.add(0), b'4');
            assert_eq!(* result.add(1), b'2');
        }
    }
    #[test]
    fn test_write_mantissa_three_digits() {
        const BUF_SIZE: usize = 5;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 3) as isize) };
        unsafe {
            write_mantissa(123, result);
            assert_eq!(* result.add(0), b'1');
            assert_eq!(* result.add(1), b'2');
            assert_eq!(* result.add(2), b'3');
        }
    }
    #[test]
    fn test_write_mantissa_four_digits() {
        const BUF_SIZE: usize = 5;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 4) as isize) };
        unsafe {
            write_mantissa(1234, result);
            assert_eq!(* result.add(0), b'1');
            assert_eq!(* result.add(1), b'2');
            assert_eq!(* result.add(2), b'3');
            assert_eq!(* result.add(3), b'4');
        }
    }
    #[test]
    fn test_write_mantissa_large_number() {
        const BUF_SIZE: usize = 10;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 6) as isize) };
        unsafe {
            write_mantissa(123456, result);
            assert_eq!(* result.add(0), b'1');
            assert_eq!(* result.add(1), b'2');
            assert_eq!(* result.add(2), b'3');
            assert_eq!(* result.add(3), b'4');
            assert_eq!(* result.add(4), b'5');
            assert_eq!(* result.add(5), b'6');
        }
    }
    #[test]
    fn test_write_mantissa_max_u32() {
        const BUF_SIZE: usize = 11;
        let mut buffer = [0u8; BUF_SIZE];
        let result = unsafe { buffer.as_mut_ptr().offset((BUF_SIZE - 10) as isize) };
        unsafe {
            write_mantissa(u32::MAX, result);
            let result_slice = std::slice::from_raw_parts(result, 10);
            let result_str = std::str::from_utf8_unchecked(result_slice);
            assert_eq!(result_str, "4294967295");
        }
    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    use std::ptr;
    use std::slice;
    const DIGIT_TABLE: &[u8] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";
    unsafe fn write_mantissa(n: u32, buf: *mut u8) {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u32, u32, u32, u32, i32, u32, i32, isize, usize, isize, usize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut n = n;
        let mut buf = buf;
        if n >= rug_fuzz_0 {
            let b = n % rug_fuzz_1;
            n /= rug_fuzz_2;
            let c = (b % rug_fuzz_3) << rug_fuzz_4;
            let d = (b / rug_fuzz_5) << rug_fuzz_6;
            ptr::copy_nonoverlapping(
                DIGIT_TABLE.as_ptr().add(c as usize),
                buf.offset(-rug_fuzz_7),
                rug_fuzz_8,
            );
            ptr::copy_nonoverlapping(
                DIGIT_TABLE.as_ptr().add(d as usize),
                buf.offset(-rug_fuzz_9),
                rug_fuzz_10,
            );
            buf = buf.offset(-rug_fuzz_11);
        }
             }
});    }
    #[test]
    fn test_write_mantissa_long() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24)) = <(u8, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, u64, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 32];
        let buffer_end = unsafe { buffer.as_mut_ptr().add(buffer.len()) };
        let tests = [
            (rug_fuzz_1, rug_fuzz_2),
            (rug_fuzz_3, rug_fuzz_4),
            (rug_fuzz_5, rug_fuzz_6),
            (rug_fuzz_7, rug_fuzz_8),
            (rug_fuzz_9, rug_fuzz_10),
            (rug_fuzz_11, rug_fuzz_12),
            (rug_fuzz_13, rug_fuzz_14),
            (rug_fuzz_15, rug_fuzz_16),
            (rug_fuzz_17, rug_fuzz_18),
            (rug_fuzz_19, rug_fuzz_20),
            (rug_fuzz_21, rug_fuzz_22),
            (u64::MAX, rug_fuzz_23),
        ];
        for (input, expected) in &tests {
            let expected_length = expected.len();
            let result_ptr = unsafe {
                write_mantissa_long(*input, buffer_end);
                buffer_end.offset(-(expected_length as isize))
            };
            let result_slice = unsafe {
                slice::from_raw_parts(result_ptr, expected_length)
            };
            let result_string = String::from_utf8(result_slice.to_vec())
                .expect(rug_fuzz_24);
            debug_assert_eq!(* expected, result_string);
        }
             }
});    }
}
