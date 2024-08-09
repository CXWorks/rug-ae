use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";
/// Safe API for formatting floating point numbers to text.
///
/// ## Example
///
/// ```
/// let mut buffer = ryu::Buffer::new();
/// let printed = buffer.format_finite(1.234);
/// assert_eq!(printed, "1.234");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; 24],
}
impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }
    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "inf", and negative infinity as "-inf" to match std::fmt.
    ///
    /// If your input is known to be finite, you may get better performance by
    /// calling the `format_finite` method instead of `format` to avoid the
    /// checks for special cases.
    #[cfg_attr(feature = "no-panic", inline)]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {
        if f.is_nonfinite() { f.format_nonfinite() } else { self.format_finite(f) }
    }
    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function **does not** check for NaN or infinity. If the input
    /// number is not a finite float, the printed representation will be some
    /// correctly formatted but unspecified numerical value.
    ///
    /// Please check [`is_finite`] yourself before calling this function, or
    /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
    ///
    /// [`is_finite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite
    /// [`is_nan`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan
    /// [`is_infinite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_infinite
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
        unsafe {
            let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
            str::from_utf8_unchecked(slice)
        }
    }
}
impl Copy for Buffer {}
impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        Buffer::new()
    }
}
impl Default for Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn default() -> Self {
        Buffer::new()
    }
}
/// A floating point number, f32 or f64, that can be written into a
/// [`ryu::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu` crate.
pub trait Float: Sealed {}
impl Float for f32 {}
impl Float for f64 {}
pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
}
impl Sealed for f32 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }
    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }
    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format32(self, result)
    }
}
impl Sealed for f64 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }
    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        const SIGN_MASK: u64 = 0x8000000000000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }
    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use crate::Buffer;
    use std::clone::Clone;
    #[test]
    fn buffer_clone_test() {
        let _rug_st_tests_llm_16_1_rrrruuuugggg_buffer_clone_test = 0;
        let buffer = Buffer::new();
        let buffer_clone = buffer.clone();
        debug_assert_ne!(
            & buffer as * const _ as usize, & buffer_clone as * const _ as usize
        );
        let _rug_ed_tests_llm_16_1_rrrruuuugggg_buffer_clone_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use super::*;
    use crate::*;
    #[test]
    fn test_default() {
        let _rug_st_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_default = 0;
        let buffer = Buffer::default();
        let buffer_new = Buffer::new();
        for (b_default, b_new) in buffer.bytes.iter().zip(buffer_new.bytes.iter()) {
            debug_assert_eq!(b_default.as_ptr(), b_new.as_ptr());
        }
        let _rug_ed_tests_llm_16_2_llm_16_2_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use super::*;
    use crate::*;
    #[test]
    fn test_format_nonfinite_nan() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_nan = 0;
        let nan: f32 = f32::NAN;
        debug_assert_eq!(< f32 as buffer::Sealed > ::format_nonfinite(nan), "NaN");
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_nan = 0;
    }
    #[test]
    fn test_format_nonfinite_negative_infinity() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_negative_infinity = 0;
        let neg_infinity: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(
            < f32 as buffer::Sealed > ::format_nonfinite(neg_infinity), "-Infinity"
        );
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_negative_infinity = 0;
    }
    #[test]
    fn test_format_nonfinite_infinity() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_infinity = 0;
        let infinity: f32 = f32::INFINITY;
        debug_assert_eq!(
            < f32 as buffer::Sealed > ::format_nonfinite(infinity), "Infinity"
        );
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_format_nonfinite_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_nonfinite_nan() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_is_nonfinite_nan = 0;
        debug_assert!(f32::NAN.is_nonfinite());
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_is_nonfinite_nan = 0;
    }
    #[test]
    fn test_is_nonfinite_infinity() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_is_nonfinite_infinity = 0;
        debug_assert!(f32::INFINITY.is_nonfinite());
        debug_assert!(f32::NEG_INFINITY.is_nonfinite());
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_is_nonfinite_infinity = 0;
    }
    #[test]
    fn test_is_nonfinite_finite() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(f32, f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_nonfinite());
        debug_assert!(! rug_fuzz_1.is_nonfinite());
        debug_assert!(! (- rug_fuzz_2).is_nonfinite());
             }
}
}
}    }
    #[test]
    fn test_is_nonfinite_subnormal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_nonfinite());
        debug_assert!(! (- rug_fuzz_1).is_nonfinite());
             }
}
}
}    }
    #[test]
    fn test_is_nonfinite_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_nonfinite());
        debug_assert!(! (- rug_fuzz_1).is_nonfinite());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;
    use crate::*;
    #[test]
    fn test_write_to_ryu_buffer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f32, f32, f32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let values: [f32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let mut buffer = [rug_fuzz_3; 16];
        for &v in &values {
            let len = unsafe {
                <f32 as buffer::Sealed>::write_to_ryu_buffer(v, buffer.as_mut_ptr())
            };
            let s = unsafe { std::str::from_utf8_unchecked(&buffer[..len]) };
            debug_assert_eq!(s.parse:: < f32 > ().unwrap(), v);
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    #[test]
    fn test_format_nonfinite_nan() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_nan = 0;
        debug_assert_eq!(f64::NAN.format_nonfinite(), "NaN");
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_nan = 0;
    }
    #[test]
    fn test_format_nonfinite_neg_infinity() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_neg_infinity = 0;
        debug_assert_eq!(f64::NEG_INFINITY.format_nonfinite(), "-Infinity");
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_neg_infinity = 0;
    }
    #[test]
    fn test_format_nonfinite_infinity() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_infinity = 0;
        debug_assert_eq!(f64::INFINITY.format_nonfinite(), "Infinity");
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_test_format_nonfinite_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_nonfinite_with_infinity() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_infinity = 0;
        debug_assert!(f64::INFINITY.is_nonfinite());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_infinity = 0;
    }
    #[test]
    fn test_is_nonfinite_with_negative_infinity() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_negative_infinity = 0;
        debug_assert!(f64::NEG_INFINITY.is_nonfinite());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_negative_infinity = 0;
    }
    #[test]
    fn test_is_nonfinite_with_nan() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_nan = 0;
        debug_assert!(f64::NAN.is_nonfinite());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_nan = 0;
    }
    #[test]
    fn test_is_nonfinite_with_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_nonfinite());
             }
}
}
}    }
    #[test]
    fn test_is_nonfinite_with_normal_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(! rug_fuzz_0.is_nonfinite());
             }
}
}
}    }
    #[test]
    fn test_is_nonfinite_with_subnormal_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let subnormal = rug_fuzz_0;
        debug_assert!(! subnormal.is_nonfinite());
             }
}
}
}    }
    #[test]
    fn test_is_nonfinite_with_max_value() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_max_value = 0;
        debug_assert!(! f64::MAX.is_nonfinite());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_max_value = 0;
    }
    #[test]
    fn test_is_nonfinite_with_min_positive_value() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_min_positive_value = 0;
        debug_assert!(! f64::MIN_POSITIVE.is_nonfinite());
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_test_is_nonfinite_with_min_positive_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    #[test]
    fn test_write_to_ryu_buffer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = [rug_fuzz_0; 24];
        let value = rug_fuzz_1;
        unsafe {
            let len = <f64 as buffer::Sealed>::write_to_ryu_buffer(
                value,
                buffer.as_mut_ptr(),
            );
            let s = std::str::from_utf8_unchecked(&buffer[..len]);
            debug_assert_eq!(s, "42.42");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use std::f32;
    use std::f64;
    #[test]
    fn test_format_finite_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_0);
        debug_assert_eq!(output, "1.234");
             }
}
}
}    }
    #[test]
    fn test_format_finite_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_0);
        debug_assert_eq!(output, "1.234");
             }
}
}
}    }
    #[test]
    fn test_format_nan_f32() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_nan_f32 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f32::NAN);
        debug_assert_eq!(output, "NaN");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_nan_f32 = 0;
    }
    #[test]
    fn test_format_nan_f64() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_nan_f64 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f64::NAN);
        debug_assert_eq!(output, "NaN");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_nan_f64 = 0;
    }
    #[test]
    fn test_format_infinity_f32() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_infinity_f32 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f32::INFINITY);
        debug_assert_eq!(output, "inf");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_infinity_f32 = 0;
    }
    #[test]
    fn test_format_infinity_f64() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_infinity_f64 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f64::INFINITY);
        debug_assert_eq!(output, "inf");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_infinity_f64 = 0;
    }
    #[test]
    fn test_format_negative_infinity_f32() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_negative_infinity_f32 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f32::NEG_INFINITY);
        debug_assert_eq!(output, "-inf");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_negative_infinity_f32 = 0;
    }
    #[test]
    fn test_format_negative_infinity_f64() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_format_negative_infinity_f64 = 0;
        let mut buffer = Buffer::new();
        let output = buffer.format(f64::NEG_INFINITY);
        debug_assert_eq!(output, "-inf");
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_format_negative_infinity_f64 = 0;
    }
    #[test]
    fn test_format_zero_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_0);
        debug_assert_eq!(output, "0");
             }
}
}
}    }
    #[test]
    fn test_format_zero_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(rug_fuzz_0);
        debug_assert_eq!(output, "0");
             }
}
}
}    }
    #[test]
    fn test_format_negative_zero_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(-rug_fuzz_0);
        debug_assert_eq!(output, "-0");
             }
}
}
}    }
    #[test]
    fn test_format_negative_zero_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let output = buffer.format(-rug_fuzz_0);
        debug_assert_eq!(output, "-0");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use std::mem::MaybeUninit;
    #[test]
    fn test_format_finite_f32() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let f: f32 = rug_fuzz_0;
        debug_assert!(f.is_finite());
        let result = buffer.format_finite(f);
        debug_assert_eq!(result, "123.456");
             }
}
}
}    }
    #[test]
    fn test_format_finite_f64() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut buffer = Buffer::new();
        let f: f64 = rug_fuzz_0;
        debug_assert!(f.is_finite());
        let result = buffer.format_finite(f);
        debug_assert_eq!(result, "1234.5678");
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_format_finite_nan() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_test_format_finite_nan = 0;
        let mut buffer = Buffer::new();
        let f: f64 = f64::NAN;
        debug_assert!(! f.is_finite());
        let _ = buffer.format_finite(f);
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_test_format_finite_nan = 0;
    }
    #[test]
    #[should_panic]
    fn test_format_finite_infinity() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_test_format_finite_infinity = 0;
        let mut buffer = Buffer::new();
        let f: f64 = f64::INFINITY;
        debug_assert!(! f.is_finite());
        let _ = buffer.format_finite(f);
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_test_format_finite_infinity = 0;
    }
    #[test]
    #[should_panic]
    fn test_format_finite_neg_infinity() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_test_format_finite_neg_infinity = 0;
        let mut buffer = Buffer::new();
        let f: f64 = f64::NEG_INFINITY;
        debug_assert!(! f.is_finite());
        let _ = buffer.format_finite(f);
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_test_format_finite_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Buffer;
    #[test]
    fn test_buffer_new() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_buffer_new = 0;
        let buffer = Buffer::new();
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_buffer_new = 0;
    }
}
