//! [![github]](https://github.com/dtolnay/itoa)&ensp;[![crates-io]](https://crates.io/crates/itoa)&ensp;[![docs-rs]](https://docs.rs/itoa)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This crate provides a fast conversion of integer primitives to decimal
//! strings. The implementation comes straight from [libcore] but avoids the
//! performance penalty of going through [`core::fmt::Formatter`].
//!
//! See also [`ryu`] for printing floating point primitives.
//!
//! [libcore]: https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L201-L254
//! [`core::fmt::Formatter`]: https://doc.rust-lang.org/std/fmt/struct.Formatter.html
//! [`ryu`]: https://github.com/dtolnay/ryu
//!
//! # Example
//!
//! ```
//! fn main() {
//!     let mut buffer = itoa::Buffer::new();
//!     let printed = buffer.format(128u64);
//!     assert_eq!(printed, "128");
//! }
//! ```
//!
//! # Performance (lower is better)
//!
//! ![performance](https://raw.githubusercontent.com/dtolnay/itoa/master/performance.png)
#![doc(html_root_url = "https://docs.rs/itoa/1.0.6")]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::must_use_candidate,
    clippy::unreadable_literal
)]
mod udiv128;
use core::mem::{self, MaybeUninit};
use core::{ptr, slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
/// A correctly sized stack allocation for the formatted integer to be written
/// into.
///
/// # Example
///
/// ```
/// let mut buffer = itoa::Buffer::new();
/// let printed = buffer.format(1234);
/// assert_eq!(printed, "1234");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; I128_MAX_LEN],
}
impl Default for Buffer {
    #[inline]
    fn default() -> Buffer {
        Buffer::new()
    }
}
impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        Buffer::new()
    }
}
impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Buffer {
        let bytes = [MaybeUninit::<u8>::uninit(); I128_MAX_LEN];
        Buffer { bytes }
    }
    /// Print an integer into this buffer and return a reference to its string
    /// representation within the buffer.
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<I: Integer>(&mut self, i: I) -> &str {
        i.write(unsafe {
            &mut *(&mut self.bytes as *mut [MaybeUninit<u8>; I128_MAX_LEN]
                as *mut <I as private::Sealed>::Buffer)
        })
    }
}
/// An integer that can be written into an [`itoa::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of itoa.
pub trait Integer: private::Sealed {}
mod private {
    pub trait Sealed: Copy {
        type Buffer: 'static;
        fn write(self, buf: &mut Self::Buffer) -> &str;
    }
}
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";
macro_rules! impl_Integer {
    ($($max_len:expr => $t:ident),* as $conv_fn:ident) => {
        $(impl Integer for $t {} impl private::Sealed for $t { type Buffer = [MaybeUninit
        < u8 >; $max_len]; #[allow(unused_comparisons)] #[inline] #[cfg_attr(feature =
        "no-panic", no_panic)] fn write(self, buf : & mut [MaybeUninit < u8 >; $max_len])
        -> & str { let is_nonnegative = self >= 0; let mut n = if is_nonnegative { self
        as $conv_fn } else { (! (self as $conv_fn)).wrapping_add(1) }; let mut curr = buf
        .len() as isize; let buf_ptr = buf.as_mut_ptr() as * mut u8; let lut_ptr =
        DEC_DIGITS_LUT.as_ptr(); unsafe { if mem::size_of::<$t > () >= 2 { while n >=
        10000 { let rem = (n % 10000) as isize; n /= 10000; let d1 = (rem / 100) << 1;
        let d2 = (rem % 100) << 1; curr -= 4; ptr::copy_nonoverlapping(lut_ptr
        .offset(d1), buf_ptr.offset(curr), 2); ptr::copy_nonoverlapping(lut_ptr
        .offset(d2), buf_ptr.offset(curr + 2), 2); } } let mut n = n as isize; if n >=
        100 { let d1 = (n % 100) << 1; n /= 100; curr -= 2;
        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2); } if n <
        10 { curr -= 1; * buf_ptr.offset(curr) = (n as u8) + b'0'; } else { let d1 = n <<
        1; curr -= 2; ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr),
        2); } if ! is_nonnegative { curr -= 1; * buf_ptr.offset(curr) = b'-'; } } let len
        = buf.len() - curr as usize; let bytes = unsafe { slice::from_raw_parts(buf_ptr
        .offset(curr), len) }; unsafe { str::from_utf8_unchecked(bytes) } } })*
    };
}
const I8_MAX_LEN: usize = 4;
const U8_MAX_LEN: usize = 3;
const I16_MAX_LEN: usize = 6;
const U16_MAX_LEN: usize = 5;
const I32_MAX_LEN: usize = 11;
const U32_MAX_LEN: usize = 10;
const I64_MAX_LEN: usize = 20;
const U64_MAX_LEN: usize = 20;
impl_Integer!(
    I8_MAX_LEN => i8, U8_MAX_LEN => u8, I16_MAX_LEN => i16, U16_MAX_LEN => u16,
    I32_MAX_LEN => i32, U32_MAX_LEN => u32 as u32
);
impl_Integer!(I64_MAX_LEN => i64, U64_MAX_LEN => u64 as u64);
#[cfg(target_pointer_width = "16")]
impl_Integer!(I16_MAX_LEN => isize, U16_MAX_LEN => usize as u16);
#[cfg(target_pointer_width = "32")]
impl_Integer!(I32_MAX_LEN => isize, U32_MAX_LEN => usize as u32);
#[cfg(target_pointer_width = "64")]
impl_Integer!(I64_MAX_LEN => isize, U64_MAX_LEN => usize as u64);
macro_rules! impl_Integer128 {
    ($($max_len:expr => $t:ident),*) => {
        $(impl Integer for $t {} impl private::Sealed for $t { type Buffer = [MaybeUninit
        < u8 >; $max_len]; #[allow(unused_comparisons)] #[inline] #[cfg_attr(feature =
        "no-panic", no_panic)] fn write(self, buf : & mut [MaybeUninit < u8 >; $max_len])
        -> & str { let is_nonnegative = self >= 0; let n = if is_nonnegative { self as
        u128 } else { (! (self as u128)).wrapping_add(1) }; let mut curr = buf.len() as
        isize; let buf_ptr = buf.as_mut_ptr() as * mut u8; unsafe { let (n, rem) =
        udiv128::udivmod_1e19(n); let buf1 = buf_ptr.offset(curr - U64_MAX_LEN as isize)
        as * mut [MaybeUninit < u8 >; U64_MAX_LEN]; curr -= rem.write(& mut * buf1).len()
        as isize; if n != 0 { let target = buf.len() as isize - 19;
        ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize); curr =
        target; let (n, rem) = udiv128::udivmod_1e19(n); let buf2 = buf_ptr.offset(curr -
        U64_MAX_LEN as isize) as * mut [MaybeUninit < u8 >; U64_MAX_LEN]; curr -= rem
        .write(& mut * buf2).len() as isize; if n != 0 { let target = buf.len() as isize
        - 38; ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
        curr = target; curr -= 1; * buf_ptr.offset(curr) = (n as u8) + b'0'; } } if !
        is_nonnegative { curr -= 1; * buf_ptr.offset(curr) = b'-'; } let len = buf.len()
        - curr as usize; let bytes = slice::from_raw_parts(buf_ptr.offset(curr), len);
        str::from_utf8_unchecked(bytes) } } })*
    };
}
const U128_MAX_LEN: usize = 39;
const I128_MAX_LEN: usize = 40;
impl_Integer128!(I128_MAX_LEN => i128, U128_MAX_LEN => u128);
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::Buffer;
    use std::clone::Clone;
    #[test]
    fn buffer_clone() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_buffer_clone = 0;
        let buffer = Buffer::new();
        let buffer_clone = buffer.clone();
        debug_assert_ne!(& buffer as * const _, & buffer_clone as * const _);
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_buffer_clone = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn buffer_default_creates_new_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let buffer = <Buffer as std::default::Default>::default();
        let uninit_byte = MaybeUninit::<u8>::uninit();
        let expected_bytes = [uninit_byte; I128_MAX_LEN];
        for i in rug_fuzz_0..I128_MAX_LEN {
            debug_assert_eq!(
                unsafe { buffer.bytes[i].as_ptr().read() }, unsafe { expected_bytes[i]
                .as_ptr().read() }
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::private::Sealed;
    use std::mem::MaybeUninit;
    const I128_MAX_LEN: usize = 40;
    #[test]
    fn test_i128_write_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buf = [MaybeUninit::uninit(); I128_MAX_LEN];
        let num: i128 = rug_fuzz_0;
        let written = <i128 as Sealed>::write(num, &mut buf);
        debug_assert_eq!(written, "123456789");
             }
});    }
    #[test]
    fn test_i128_write_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buf = [MaybeUninit::uninit(); I128_MAX_LEN];
        let num: i128 = -rug_fuzz_0;
        let written = <i128 as Sealed>::write(num, &mut buf);
        debug_assert_eq!(written, "-123456789");
             }
});    }
    #[test]
    fn test_i128_write_max() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_i128_write_max = 0;
        let mut buf = [MaybeUninit::uninit(); I128_MAX_LEN];
        let num = i128::max_value();
        let written = <i128 as Sealed>::write(num, &mut buf);
        debug_assert_eq!(written, & num.to_string());
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_i128_write_max = 0;
    }
    #[test]
    fn test_i128_write_min() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_i128_write_min = 0;
        let mut buf = [MaybeUninit::uninit(); I128_MAX_LEN];
        let num = i128::min_value();
        let written = <i128 as Sealed>::write(num, &mut buf);
        debug_assert_eq!(written, & num.to_string());
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_i128_write_min = 0;
    }
    #[test]
    fn test_i128_write_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buf = [MaybeUninit::uninit(); I128_MAX_LEN];
        let num: i128 = rug_fuzz_0;
        let written = <i128 as Sealed>::write(num, &mut buf);
        debug_assert_eq!(written, "0");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn test_write_positive_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer: [MaybeUninit<u8>; 6] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let num: i16 = rug_fuzz_0;
        let value_str = <i16 as private::Sealed>::write(num, &mut buffer);
        debug_assert_eq!(value_str, "1234");
             }
});    }
    #[test]
    fn test_write_negative_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer: [MaybeUninit<u8>; 6] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let num: i16 = -rug_fuzz_0;
        let value_str = <i16 as private::Sealed>::write(num, &mut buffer);
        debug_assert_eq!(value_str, "-1234");
             }
});    }
    #[test]
    fn test_write_zero_i16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer: [MaybeUninit<u8>; 6] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let num: i16 = rug_fuzz_0;
        let value_str = <i16 as private::Sealed>::write(num, &mut buffer);
        debug_assert_eq!(value_str, "0");
             }
});    }
    #[test]
    fn test_write_min_value_i16() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_write_min_value_i16 = 0;
        let mut buffer: [MaybeUninit<u8>; 6] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let num: i16 = i16::MIN;
        let value_str = <i16 as private::Sealed>::write(num, &mut buffer);
        debug_assert_eq!(value_str, "-32768");
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_write_min_value_i16 = 0;
    }
    #[test]
    fn test_write_max_value_i16() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_write_max_value_i16 = 0;
        let mut buffer: [MaybeUninit<u8>; 6] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let num: i16 = i16::MAX;
        let value_str = <i16 as private::Sealed>::write(num, &mut buffer);
        debug_assert_eq!(value_str, "32767");
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_write_max_value_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn test_write_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i64 = rug_fuzz_0;
        let mut buf: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <i64 as private::Sealed>::write(num, &mut buf);
        debug_assert_eq!(result, "12345");
             }
});    }
    #[test]
    fn test_write_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i64 = -rug_fuzz_0;
        let mut buf: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <i64 as private::Sealed>::write(num, &mut buf);
        debug_assert_eq!(result, "-12345");
             }
});    }
    #[test]
    fn test_write_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let num: i64 = rug_fuzz_0;
        let mut buf: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <i64 as private::Sealed>::write(num, &mut buf);
        debug_assert_eq!(result, "0");
             }
});    }
    #[test]
    fn test_write_max() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_write_max = 0;
        let num: i64 = i64::MAX;
        let mut buf: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <i64 as private::Sealed>::write(num, &mut buf);
        debug_assert_eq!(result, "9223372036854775807");
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_write_max = 0;
    }
    #[test]
    fn test_write_min() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_write_min = 0;
        let num: i64 = i64::MIN;
        let mut buf: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <i64 as private::Sealed>::write(num, &mut buf);
        debug_assert_eq!(result, "-9223372036854775808");
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_write_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn test_write_positive_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: isize = rug_fuzz_0;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "12345");
             }
});    }
    #[test]
    fn test_write_negative_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: isize = -rug_fuzz_0;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "-12345");
             }
});    }
    #[test]
    fn test_write_zero_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: isize = rug_fuzz_0;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "0");
             }
});    }
    #[test]
    fn test_write_small_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: isize = rug_fuzz_0;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "5");
             }
});    }
    #[test]
    fn test_write_large_isize() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_test_write_large_isize = 0;
        let value: isize = isize::MAX;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, & isize::MAX.to_string());
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_test_write_large_isize = 0;
    }
    #[test]
    fn test_write_large_negative_isize() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_test_write_large_negative_isize = 0;
        let value: isize = isize::MIN;
        let mut buffer = [MaybeUninit::uninit(); 20];
        let result = <isize as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, & isize::MIN.to_string());
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_test_write_large_negative_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use core::mem::MaybeUninit;
    #[test]
    fn test_write_u16() {
        fn write_u16(val: u16) -> String {
            let mut buf: [MaybeUninit<u8>; 5] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            let s = <u16 as private::Sealed>::write(val, &mut buf);
            s.to_owned()
        }
        assert_eq!(write_u16(0), "0");
        assert_eq!(write_u16(9), "9");
        assert_eq!(write_u16(10), "10");
        assert_eq!(write_u16(123), "123");
        assert_eq!(write_u16(9999), "9999");
        assert_eq!(write_u16(10000), "10000");
        assert_eq!(write_u16(65535), "65535");
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn write_u32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u32, &str, u32, &str, u32, &str, u32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let cases: &[(u32, &str)] = &[
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, rug_fuzz_5),
            (rug_fuzz_6, rug_fuzz_7),
            (u32::MAX, rug_fuzz_8),
        ];
        for &(num, expected) in cases {
            let mut buf: [MaybeUninit<u8>; 10] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            let result = <u32 as private::Sealed>::write(num, &mut buf);
            debug_assert_eq!(result, expected);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn write_positive_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u64 = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <u64 as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "1234567890");
             }
});    }
    #[test]
    fn write_max_u64() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_write_max_u64 = 0;
        let value: u64 = u64::MAX;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <u64 as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "18446744073709551615");
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_write_max_u64 = 0;
    }
    #[test]
    fn write_zero_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let value: u64 = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <u64 as private::Sealed>::write(value, &mut buffer);
        debug_assert_eq!(result, "0");
             }
});    }
    #[test]
    fn write_single_digit_u64() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for digit in rug_fuzz_0..rug_fuzz_1 {
            let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            let result = <u64 as private::Sealed>::write(digit, &mut buffer);
            debug_assert_eq!(result, digit.to_string().as_str());
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    const MAX_LEN: usize = 3;
    #[test]
    fn test_write() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer: [MaybeUninit<u8>; MAX_LEN] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let inputs: [u8; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let expected: [&str; 3] = [rug_fuzz_3, rug_fuzz_4, rug_fuzz_5];
        for (&input, &expected) in inputs.iter().zip(expected.iter()) {
            let output = <u8 as private::Sealed>::write(input, &mut buffer);
            debug_assert_eq!(output, expected, "Failed for input: {}", input);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_14_llm_16_14 {
    use super::*;
    use crate::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn test_write_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number: usize = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <usize as private::Sealed>::write(number, &mut buffer);
        debug_assert_eq!(result, "12345");
             }
});    }
    #[test]
    fn test_write_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number: usize = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <usize as private::Sealed>::write(number, &mut buffer);
        debug_assert_eq!(result, "0");
             }
});    }
    #[test]
    fn test_write_max_value() {
        let _rug_st_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_write_max_value = 0;
        let number: usize = std::usize::MAX;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <usize as private::Sealed>::write(number, &mut buffer);
        let expected = std::usize::MAX.to_string();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_14_llm_16_14_rrrruuuugggg_test_write_max_value = 0;
    }
    #[test]
    fn test_write_power_of_10() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number: usize = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <usize as private::Sealed>::write(number, &mut buffer);
        debug_assert_eq!(result, "10000");
             }
});    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 20 but the index is 20")]
    fn test_write_buffer_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number: usize = rug_fuzz_0;
             }
});    }
    #[test]
    fn test_write_small_positive() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let number: usize = rug_fuzz_0;
        let mut buffer: [MaybeUninit<u8>; 20] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let result = <usize as private::Sealed>::write(number, &mut buffer);
        debug_assert_eq!(result, "12");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_15_llm_16_15 {
    use super::*;
    use crate::*;
    #[test]
    fn test_format() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_0);
        debug_assert_eq!(output, "1234");
        let mut buffer = Buffer::new();
        let output = buffer.format(-rug_fuzz_1);
        debug_assert_eq!(output, "-5678");
        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_2);
        debug_assert_eq!(output, "0");
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::Buffer;
    use std::mem::MaybeUninit;
    use crate::I128_MAX_LEN;
    #[test]
    fn test_buffer_new() {
        let _rug_st_tests_rug_3_rrrruuuugggg_test_buffer_new = 0;
        let buffer = <Buffer>::new();
        debug_assert_eq!(buffer.bytes.len(), I128_MAX_LEN);
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_buffer_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::private::Sealed;
    use std::mem::MaybeUninit;
    #[test]
    fn test_write() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i8 = -rug_fuzz_0;
        let mut p1: [MaybeUninit<u8>; 4] = [MaybeUninit::uninit(); 4];
        p0.write(&mut p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::private::Sealed;
    use std::mem::MaybeUninit;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u128 = rug_fuzz_0;
        let mut p1: [MaybeUninit<u8>; 39] = [MaybeUninit::uninit(); 39];
        p0.write(&mut p1);
             }
});    }
}
