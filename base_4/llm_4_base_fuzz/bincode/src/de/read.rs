//! This module contains reader-based structs and traits.
//!
//! Because `std::io::Read` is only limited to `std` and not `core`, we provide 2 alternative readers.
//!
//! [Reader] is a reader for sources that do not own their data. It is assumed that the reader's data is dropped after the `read` method is called. This reader is incapable of reading borrowed data, like `&str` and `&[u8]`.
//!
//! [BorrowReader] is an extension of `Reader` that also allows returning borrowed data. A `BorrowReader` allows reading `&str` and `&[u8]`.
//!
//! Specifically the `Reader` trait is used by [Decode] and the `BorrowReader` trait is used by `[BorrowDecode]`.
//!
//! [Decode]: ../trait.Decode.html
//! [BorrowDecode]: ../trait.BorrowDecode.html
use crate::error::DecodeError;
/// A reader for owned data. See the module documentation for more information.
pub trait Reader {
    /// Fill the given `bytes` argument with values. Exactly the length of the given slice must be filled, or else an error must be returned.
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError>;
    /// If this reader wraps a buffer of any kind, this function lets callers access contents of
    /// the buffer without passing data through a buffer first.
    #[inline]
    fn peek_read(&mut self, _: usize) -> Option<&[u8]> {
        None
    }
    /// If an implementation of `peek_read` is provided, an implementation of this function
    /// must be provided so that subsequent reads or peek-reads do not return the same bytes
    #[inline]
    fn consume(&mut self, _: usize) {}
}
impl<T> Reader for &mut T
where
    T: Reader,
{
    #[inline]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        (**self).read(bytes)
    }
    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&[u8]> {
        (**self).peek_read(n)
    }
    #[inline]
    fn consume(&mut self, n: usize) {
        (*self).consume(n)
    }
}
/// A reader for borrowed data. Implementors of this must also implement the [Reader] trait. See the module documentation for more information.
pub trait BorrowReader<'storage>: Reader {
    /// Read exactly `length` bytes and return a slice to this data. If not enough bytes could be read, an error should be returned.
    ///
    /// *note*: Exactly `length` bytes must be returned. If less bytes are returned, bincode may panic. If more bytes are returned, the excess bytes may be discarded.
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError>;
}
/// A reader type for `&[u8]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data.
pub struct SliceReader<'storage> {
    pub(crate) slice: &'storage [u8],
}
impl<'storage> SliceReader<'storage> {
    /// Constructs a slice reader
    pub fn new(bytes: &'storage [u8]) -> SliceReader<'storage> {
        SliceReader { slice: bytes }
    }
}
impl<'storage> Reader for SliceReader<'storage> {
    #[inline(always)]
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), DecodeError> {
        if bytes.len() > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd {
                additional: bytes.len() - self.slice.len(),
            });
        }
        let (read_slice, remaining) = self.slice.split_at(bytes.len());
        bytes.copy_from_slice(read_slice);
        self.slice = remaining;
        Ok(())
    }
    #[inline]
    fn peek_read(&mut self, n: usize) -> Option<&'storage [u8]> {
        self.slice.get(..n)
    }
    #[inline]
    fn consume(&mut self, n: usize) {
        self.slice = self.slice.get(n..).unwrap_or_default();
    }
}
impl<'storage> BorrowReader<'storage> for SliceReader<'storage> {
    #[inline(always)]
    fn take_bytes(&mut self, length: usize) -> Result<&'storage [u8], DecodeError> {
        if length > self.slice.len() {
            return Err(DecodeError::UnexpectedEnd {
                additional: length - self.slice.len(),
            });
        }
        let (read_slice, remaining) = self.slice.split_at(length);
        self.slice = remaining;
        Ok(read_slice)
    }
}
#[cfg(test)]
mod tests_llm_16_20 {
    use super::*;
    use crate::*;
    #[test]
    fn test_consume_inside_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [4, 5]);
             }
});    }
    #[test]
    fn test_consume_at_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
             }
});    }
    #[test]
    fn test_consume_out_of_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
             }
});    }
    #[test]
    fn test_consume_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
             }
});    }
    #[test]
    fn test_consume_multiple_times() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [3, 4, 5]);
        reader.consume(rug_fuzz_6);
        debug_assert_eq!(reader.slice, & [4, 5]);
        reader.consume(rug_fuzz_7);
        debug_assert_eq!(reader.slice, & []);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;
    use crate::*;
    use crate::de::{DecodeError, Reader};
    use crate::de::read::SliceReader;
    #[test]
    fn read_exact_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut buffer = [rug_fuzz_5; 5];
        let result = reader.read(&mut buffer);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, data);
             }
});    }
    #[test]
    fn read_partial_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut buffer = [rug_fuzz_5; 3];
        let result = reader.read(&mut buffer);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, [1, 2, 3]);
        debug_assert_eq!(reader.slice, [4, 5]);
             }
});    }
    #[test]
    fn read_length_exceeding_slice() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut buffer = [rug_fuzz_5; 6];
        let result = reader.read(&mut buffer);
        debug_assert!(result.is_err());
        if let Err(DecodeError::UnexpectedEnd { additional }) = result {
            debug_assert_eq!(additional, 1);
        } else {
            panic!("Expected DecodeError::UnexpectedEnd but got a different error");
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_196 {
    use super::*;
    use crate::*;
    use de::read::{BorrowReader, Reader, SliceReader};
    use crate::error::DecodeError;
    #[test]
    fn new_slice_reader() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(data);
        debug_assert_eq!(reader.slice, data);
             }
});    }
    #[test]
    fn read_from_slice_reader() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let mut buffer = [rug_fuzz_5; 3];
        reader.read(&mut buffer).unwrap();
        debug_assert_eq!(buffer, [1, 2, 3]);
        debug_assert_eq!(reader.slice, & [4, 5]);
             }
});    }
    #[test]
    fn take_bytes_from_slice_reader() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let bytes = reader.take_bytes(rug_fuzz_5).unwrap();
        debug_assert_eq!(bytes, & [1, 2, 3]);
        debug_assert_eq!(reader.slice, & [4, 5]);
             }
});    }
    #[test]
    fn read_error_exceeding_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let mut buffer = [rug_fuzz_5; 6];
        let result = reader.read(&mut buffer);
        debug_assert!(
            matches!(result, Err(DecodeError::UnexpectedEnd { additional : 1 }))
        );
             }
});    }
    #[test]
    fn take_bytes_error_exceeding_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let result = reader.take_bytes(rug_fuzz_5);
        debug_assert!(
            matches!(result, Err(DecodeError::UnexpectedEnd { additional : 1 }))
        );
             }
});    }
    #[test]
    fn peek_read_within_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let peeked = reader.peek_read(rug_fuzz_5).unwrap();
        debug_assert_eq!(peeked, & [1, 2]);
        debug_assert_eq!(reader.slice, data);
             }
});    }
    #[test]
    fn peek_read_exceeding_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let peeked = reader.peek_read(rug_fuzz_5);
        debug_assert!(peeked.is_none());
             }
});    }
    #[test]
    fn consume_within_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [4, 5]);
             }
});    }
    #[test]
    fn consume_exceeding_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
             }
});    }
}
