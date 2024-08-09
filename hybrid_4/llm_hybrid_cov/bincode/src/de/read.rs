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
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_consume_inside_bounds = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [4, 5]);
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_consume_inside_bounds = 0;
    }
    #[test]
    fn test_consume_at_bounds() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_consume_at_bounds = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_consume_at_bounds = 0;
    }
    #[test]
    fn test_consume_out_of_bounds() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_consume_out_of_bounds = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 10;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_consume_out_of_bounds = 0;
    }
    #[test]
    fn test_consume_zero() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_consume_zero = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_consume_zero = 0;
    }
    #[test]
    fn test_consume_multiple_times() {
        let _rug_st_tests_llm_16_20_rrrruuuugggg_test_consume_multiple_times = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        debug_assert_eq!(reader.slice, & [1, 2, 3, 4, 5]);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [3, 4, 5]);
        reader.consume(rug_fuzz_6);
        debug_assert_eq!(reader.slice, & [4, 5]);
        reader.consume(rug_fuzz_7);
        debug_assert_eq!(reader.slice, & []);
        let _rug_ed_tests_llm_16_20_rrrruuuugggg_test_consume_multiple_times = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;
    use crate::*;
    use crate::de::{DecodeError, Reader};
    use crate::de::read::SliceReader;
    #[test]
    fn read_exact_length() {
        let _rug_st_tests_llm_16_22_rrrruuuugggg_read_exact_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut buffer = [rug_fuzz_5; 5];
        let result = reader.read(&mut buffer);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, data);
        let _rug_ed_tests_llm_16_22_rrrruuuugggg_read_exact_length = 0;
    }
    #[test]
    fn read_partial_length() {
        let _rug_st_tests_llm_16_22_rrrruuuugggg_read_partial_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut buffer = [rug_fuzz_5; 3];
        let result = reader.read(&mut buffer);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, [1, 2, 3]);
        debug_assert_eq!(reader.slice, [4, 5]);
        let _rug_ed_tests_llm_16_22_rrrruuuugggg_read_partial_length = 0;
    }
    #[test]
    fn read_length_exceeding_slice() {
        let _rug_st_tests_llm_16_22_rrrruuuugggg_read_length_exceeding_slice = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
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
        let _rug_ed_tests_llm_16_22_rrrruuuugggg_read_length_exceeding_slice = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_196 {
    use super::*;
    use crate::*;
    use de::read::{BorrowReader, Reader, SliceReader};
    use crate::error::DecodeError;
    #[test]
    fn new_slice_reader() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_new_slice_reader = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(data);
        debug_assert_eq!(reader.slice, data);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_new_slice_reader = 0;
    }
    #[test]
    fn read_from_slice_reader() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_read_from_slice_reader = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let mut buffer = [rug_fuzz_5; 3];
        reader.read(&mut buffer).unwrap();
        debug_assert_eq!(buffer, [1, 2, 3]);
        debug_assert_eq!(reader.slice, & [4, 5]);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_read_from_slice_reader = 0;
    }
    #[test]
    fn take_bytes_from_slice_reader() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_take_bytes_from_slice_reader = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let bytes = reader.take_bytes(rug_fuzz_5).unwrap();
        debug_assert_eq!(bytes, & [1, 2, 3]);
        debug_assert_eq!(reader.slice, & [4, 5]);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_take_bytes_from_slice_reader = 0;
    }
    #[test]
    fn read_error_exceeding_length() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_read_error_exceeding_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let mut buffer = [rug_fuzz_5; 6];
        let result = reader.read(&mut buffer);
        debug_assert!(
            matches!(result, Err(DecodeError::UnexpectedEnd { additional : 1 }))
        );
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_read_error_exceeding_length = 0;
    }
    #[test]
    fn take_bytes_error_exceeding_length() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_take_bytes_error_exceeding_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 6;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let result = reader.take_bytes(rug_fuzz_5);
        debug_assert!(
            matches!(result, Err(DecodeError::UnexpectedEnd { additional : 1 }))
        );
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_take_bytes_error_exceeding_length = 0;
    }
    #[test]
    fn peek_read_within_bounds() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_peek_read_within_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 2;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let peeked = reader.peek_read(rug_fuzz_5).unwrap();
        debug_assert_eq!(peeked, & [1, 2]);
        debug_assert_eq!(reader.slice, data);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_peek_read_within_bounds = 0;
    }
    #[test]
    fn peek_read_exceeding_length() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_peek_read_exceeding_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 7;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        let peeked = reader.peek_read(rug_fuzz_5);
        debug_assert!(peeked.is_none());
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_peek_read_exceeding_length = 0;
    }
    #[test]
    fn consume_within_bounds() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_consume_within_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & [4, 5]);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_consume_within_bounds = 0;
    }
    #[test]
    fn consume_exceeding_length() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_consume_exceeding_length = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 7;
        let data = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(data);
        reader.consume(rug_fuzz_5);
        debug_assert_eq!(reader.slice, & []);
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_consume_exceeding_length = 0;
    }
}
#[cfg(test)]
mod tests_rug_258 {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;
    #[test]
    fn test_peek_read() {
        let _rug_st_tests_rug_258_rrrruuuugggg_test_peek_read = 0;
        let rug_fuzz_0 = 5;
        let sample_data = vec![0u8; 10];
        let cursor = Cursor::new(sample_data);
        let mut p0 = BufReader::new(cursor);
        let p1: usize = rug_fuzz_0;
        debug_assert!(crate ::de::read::Reader::peek_read(& mut p0, p1).is_none());
        let _rug_ed_tests_rug_258_rrrruuuugggg_test_peek_read = 0;
    }
}
#[cfg(test)]
mod tests_rug_259 {
    use super::*;
    use crate::de::read::SliceReader;
    #[test]
    fn test_consume() {
        let _rug_st_tests_rug_259_rrrruuuugggg_test_consume = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let p1: usize = rug_fuzz_5;
        crate::de::read::Reader::consume(&mut p0, p1);
        let _rug_ed_tests_rug_259_rrrruuuugggg_test_consume = 0;
    }
}
#[cfg(test)]
mod tests_rug_260 {
    use super::*;
    use crate::de::read::{Reader, SliceReader};
    #[test]
    fn test_read() {
        let _rug_st_tests_rug_260_rrrruuuugggg_test_read = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 0u8;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let mut p1 = [rug_fuzz_5; 5];
        p0.read(&mut p1).unwrap();
        debug_assert_eq!(p1, [1u8, 2, 3, 4, 5]);
        let _rug_ed_tests_rug_260_rrrruuuugggg_test_read = 0;
    }
}
#[cfg(test)]
mod tests_rug_261 {
    use super::*;
    use crate::de::read::Reader;
    use std::io::{self, BufReader, Cursor};
    #[test]
    fn test_peek_read() {
        let _rug_st_tests_rug_261_rrrruuuugggg_test_peek_read = 0;
        let rug_fuzz_0 = 5;
        let sample_data = vec![0u8; 10];
        let cursor = Cursor::new(sample_data);
        let mut p0: BufReader<Cursor<Vec<u8>>> = BufReader::new(cursor);
        let p1: usize = rug_fuzz_0;
        debug_assert_eq!(p0.peek_read(p1), Some(& [0u8; 5] [..]));
        let _rug_ed_tests_rug_261_rrrruuuugggg_test_peek_read = 0;
    }
}
#[cfg(test)]
mod tests_rug_262 {
    use super::*;
    use crate::de::read::Reader;
    use crate::de::read::SliceReader;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_262_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 3;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let p1: usize = rug_fuzz_5;
        p0.consume(p1);
        let _rug_ed_tests_rug_262_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_263 {
    use super::*;
    use crate::de::read::{Reader, SliceReader};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_263_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 8;
        let data = &[rug_fuzz_0; 16];
        let mut p0 = SliceReader::new(data);
        let p1: usize = rug_fuzz_1;
        debug_assert_eq!(
            < SliceReader as Reader > ::peek_read(& mut p0, p1), Some(& data[..p1])
        );
        let _rug_ed_tests_rug_263_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_264 {
    use crate::de::{self, DecodeError, read::{BorrowReader, SliceReader}};
    #[test]
    fn test_take_bytes() {
        let _rug_st_tests_rug_264_rrrruuuugggg_test_take_bytes = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 32;
        let data = &[rug_fuzz_0; 16];
        let mut p0 = SliceReader::new(data);
        let p1: usize = rug_fuzz_1;
        match p0.take_bytes(p1) {
            Ok(slice) => debug_assert_eq!(slice.len(), p1),
            Err(e) => panic!("Error occurred: {:?}", e),
        }
        let p1_overflow: usize = rug_fuzz_2;
        match p0.take_bytes(p1_overflow) {
            Ok(_) => panic!("Should have errored due to overflow"),
            Err(DecodeError::UnexpectedEnd { additional }) => {
                debug_assert_eq!(additional, p1_overflow - data.len() + p1)
            }
            Err(_) => panic!("Unexpected error type"),
        }
        let _rug_ed_tests_rug_264_rrrruuuugggg_test_take_bytes = 0;
    }
}
