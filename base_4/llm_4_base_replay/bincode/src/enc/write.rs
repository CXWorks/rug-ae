//! This module contains writer-based structs and traits.
//!
//! Because `std::io::Write` is only limited to `std` and not `core`, we provide our own [Writer].
use crate::error::EncodeError;
/// Trait that indicates that a struct can be used as a destination to encode data too. This is used by [Encode]
///
/// [Encode]: ../trait.Encode.html
pub trait Writer {
    /// Write `bytes` to the underlying writer. Exactly `bytes.len()` bytes must be written, or else an error should be returned.
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;
}
impl<T: Writer> Writer for &mut T {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        (**self).write(bytes)
    }
}
/// A helper struct that implements `Writer` for a `&[u8]` slice.
///
/// ```
/// use bincode::enc::write::{Writer, SliceWriter};
///
/// let destination = &mut [0u8; 100];
/// let mut writer = SliceWriter::new(destination);
/// writer.write(&[1, 2, 3, 4, 5]).unwrap();
///
/// assert_eq!(writer.bytes_written(), 5);
/// assert_eq!(destination[0..6], [1, 2, 3, 4, 5, 0]);
/// ```
pub struct SliceWriter<'storage> {
    slice: &'storage mut [u8],
    original_length: usize,
}
impl<'storage> SliceWriter<'storage> {
    /// Create a new instance of `SliceWriter` with the given byte array.
    pub fn new(bytes: &'storage mut [u8]) -> SliceWriter<'storage> {
        let original = bytes.len();
        SliceWriter {
            slice: bytes,
            original_length: original,
        }
    }
    /// Return the amount of bytes written so far.
    pub fn bytes_written(&self) -> usize {
        self.original_length - self.slice.len()
    }
}
impl<'storage> Writer for SliceWriter<'storage> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        if bytes.len() > self.slice.len() {
            return Err(EncodeError::UnexpectedEnd);
        }
        let (a, b) = core::mem::take(&mut self.slice).split_at_mut(bytes.len());
        a.copy_from_slice(bytes);
        self.slice = b;
        Ok(())
    }
}
/// A writer that counts how many bytes were written. This is useful for e.g. pre-allocating buffers bfeore writing to them.
#[derive(Default)]
pub struct SizeWriter {
    /// the amount of bytes that were written so far
    pub bytes_written: usize,
}
impl Writer for SizeWriter {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.bytes_written += bytes.len();
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use crate::enc::write::Writer;
    use crate::error::EncodeError;
    use std::io::Write;
    struct TestWriter(Vec<u8>);
    impl Writer for TestWriter {
        fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
            self.0.extend_from_slice(bytes);
            Ok(())
        }
    }
    #[test]
    fn write_to_writer_success() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = TestWriter(Vec::new());
        let data: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let result = Writer::write(&mut writer, data);
        debug_assert!(result.is_ok());
        debug_assert_eq!(writer.0, data);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_25 {
    use crate::enc::write::{SizeWriter, Writer};
    use crate::enc::EncodeError;
    #[test]
    fn test_size_writer_write() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = SizeWriter::default();
        debug_assert_eq!(writer.bytes_written, 0);
        let bytes = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        writer.write(bytes).unwrap();
        debug_assert_eq!(writer.bytes_written, 5);
        let more_bytes = &[rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9];
        writer.write(more_bytes).unwrap();
        debug_assert_eq!(writer.bytes_written, 10);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_263_llm_16_263 {
    use crate::enc::write::{SliceWriter, Writer};
    use crate::error::EncodeError;
    #[test]
    fn test_new_slicewriter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bytes = [rug_fuzz_0; 10];
        let slicewriter = SliceWriter::new(&mut bytes);
        debug_assert_eq!(slicewriter.bytes_written(), 0);
        debug_assert_eq!(slicewriter.original_length, 10);
        debug_assert_eq!(slicewriter.slice.len(), 10);
             }
}
}
}    }
    #[test]
    fn test_new_slicewriter_then_write() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bytes = [rug_fuzz_0; 10];
        let mut slicewriter = SliceWriter::new(&mut bytes);
        let data = [rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4, rug_fuzz_5];
        debug_assert!(slicewriter.write(& data).is_ok());
        debug_assert_eq!(slicewriter.bytes_written(), 5);
        debug_assert_eq!(slicewriter.slice.len(), 5);
        debug_assert_eq!(& bytes[..rug_fuzz_6], & data);
        debug_assert_eq!(& bytes[rug_fuzz_7..], & [0u8; 5]);
             }
}
}
}    }
    #[test]
    fn test_new_slicewriter_write_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bytes = [rug_fuzz_0; 5];
        let mut slicewriter = SliceWriter::new(&mut bytes);
        let data = [
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
        ];
        debug_assert!(
            matches!(slicewriter.write(& data), Err(EncodeError::UnexpectedEnd))
        );
        debug_assert_eq!(slicewriter.bytes_written(), 0);
        debug_assert_eq!(slicewriter.slice.len(), 5);
        debug_assert_eq!(& bytes, & [0u8; 5]);
             }
}
}
}    }
}
