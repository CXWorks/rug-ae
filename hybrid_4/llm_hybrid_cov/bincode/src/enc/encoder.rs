use super::{write::Writer, Encoder};
use crate::{config::Config, utils::Sealed};
/// An Encoder that writes bytes into a given writer `W`.
///
/// This struct should rarely be used.
/// In most cases, prefer any of the `encode` functions.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to write integers to the writer.
///
/// ```
/// # use bincode::enc::{write::SliceWriter, EncoderImpl, Encode};
/// let slice: &mut [u8] = &mut [0, 0, 0, 0];
/// let config = bincode::config::legacy().with_big_endian();
///
/// let mut encoder = EncoderImpl::new(SliceWriter::new(slice), config);
/// // this u32 can be any Encodable
/// 5u32.encode(&mut encoder).unwrap();
/// assert_eq!(encoder.into_writer().bytes_written(), 4);
/// assert_eq!(slice, [0, 0, 0, 5]);
/// ```
pub struct EncoderImpl<W: Writer, C: Config> {
    writer: W,
    config: C,
}
impl<W: Writer, C: Config> EncoderImpl<W, C> {
    /// Create a new Encoder
    pub fn new(writer: W, config: C) -> EncoderImpl<W, C> {
        EncoderImpl { writer, config }
    }
    /// Return the underlying writer
    #[inline]
    pub fn into_writer(self) -> W {
        self.writer
    }
}
impl<W: Writer, C: Config> Encoder for EncoderImpl<W, C> {
    type W = W;
    type C = C;
    #[inline]
    fn writer(&mut self) -> &mut Self::W {
        &mut self.writer
    }
    #[inline]
    fn config(&self) -> &Self::C {
        &self.config
    }
}
impl<W: Writer, C: Config> Sealed for EncoderImpl<W, C> {}
#[cfg(test)]
mod tests_llm_16_24_llm_16_24 {
    use crate::enc::{
        encoder::{Encoder, EncoderImpl},
        write::{SizeWriter, Writer},
    };
    use crate::config::{self, Config, Configuration, LittleEndian, NoLimit, Varint};
    #[test]
    fn test_writer() {
        let _rug_st_tests_llm_16_24_llm_16_24_rrrruuuugggg_test_writer = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let mut size_writer = SizeWriter::default();
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default();
        let mut encoder = EncoderImpl::new(&mut size_writer, config);
        let writer = encoder.writer();
        let bytes = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        writer.write(&bytes).unwrap();
        debug_assert_eq!(size_writer.bytes_written, 4);
        let _rug_ed_tests_llm_16_24_llm_16_24_rrrruuuugggg_test_writer = 0;
    }
}
#[cfg(test)]
mod tests_rug_325 {
    use super::*;
    use crate::enc::write::SizeWriter;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_325_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let mut p0 = SizeWriter {
            bytes_written: rug_fuzz_0,
        };
        let mut p1 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        crate::enc::encoder::EncoderImpl::<
            SizeWriter,
            Configuration<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >,
        >::new(p0, p1);
        let _rug_ed_tests_rug_325_rrrruuuugggg_test_rug = 0;
    }
}
