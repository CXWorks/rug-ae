use super::{
    read::{BorrowReader, Reader},
    BorrowDecoder, Decoder,
};
use crate::{config::Config, error::DecodeError, utils::Sealed};
/// A Decoder that reads bytes from a given reader `R`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `decode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to read integers out of the reader.
///
/// ```
/// # let slice: &[u8] = &[0, 0, 0, 0];
/// # let some_reader = bincode::de::read::SliceReader::new(slice);
/// use bincode::de::{DecoderImpl, Decode};
/// let mut decoder = DecoderImpl::new(some_reader, bincode::config::standard());
/// // this u32 can be any Decode
/// let value = u32::decode(&mut decoder).unwrap();
/// ```
pub struct DecoderImpl<R, C: Config> {
    reader: R,
    config: C,
    bytes_read: usize,
}
impl<R: Reader, C: Config> DecoderImpl<R, C> {
    /// Construct a new Decoder
    pub fn new(reader: R, config: C) -> DecoderImpl<R, C> {
        DecoderImpl {
            reader,
            config,
            bytes_read: 0,
        }
    }
}
impl<R, C: Config> Sealed for DecoderImpl<R, C> {}
impl<'de, R: BorrowReader<'de>, C: Config> BorrowDecoder<'de> for DecoderImpl<R, C> {
    type BR = R;
    fn borrow_reader(&mut self) -> &mut Self::BR {
        &mut self.reader
    }
}
impl<R: Reader, C: Config> Decoder for DecoderImpl<R, C> {
    type R = R;
    type C = C;
    fn reader(&mut self) -> &mut Self::R {
        &mut self.reader
    }
    fn config(&self) -> &Self::C {
        &self.config
    }
    #[inline]
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError> {
        if let Some(limit) = C::LIMIT {
            self
                .bytes_read = self
                .bytes_read
                .checked_add(n)
                .ok_or(DecodeError::LimitExceeded)?;
            if self.bytes_read > limit {
                Err(DecodeError::LimitExceeded)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
    #[inline]
    fn unclaim_bytes_read(&mut self, n: usize) {
        if C::LIMIT.is_some() {
            self.bytes_read -= n;
        }
    }
}
#[cfg(test)]
mod tests_rug_288 {
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, Fixint, Configuration, Limit};
    #[test]
    fn test_new_decoder() {
        let _rug_st_tests_rug_288_rrrruuuugggg_test_new_decoder = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let mut p1 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let _decoder = DecoderImpl::new(p0, p1);
        let _rug_ed_tests_rug_288_rrrruuuugggg_test_new_decoder = 0;
    }
}
