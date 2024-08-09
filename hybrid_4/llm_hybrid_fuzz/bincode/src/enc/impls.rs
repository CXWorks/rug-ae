use super::{write::Writer, Encode, Encoder};
use crate::{
    config::{Endian, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::EncodeError,
};
use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};
impl Encode for () {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}
impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}
impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u8::from(*self).encode(encoder)
    }
}
impl Encode for u8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self])
    }
}
impl Encode for NonZeroU8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for u16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroU16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for u32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroU32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for u64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroU64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroU128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for usize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&(*self as u64).to_be_bytes()),
                    Endian::Little => {
                        encoder.writer().write(&(*self as u64).to_le_bytes())
                    }
                }
            }
        }
    }
}
impl Encode for NonZeroUsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for i8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self as u8])
    }
}
impl Encode for NonZeroI8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for i16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroI16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for i32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroI32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for i64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroI64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for i128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                    Endian::Little => encoder.writer().write(&self.to_le_bytes()),
                }
            }
        }
    }
}
impl Encode for NonZeroI128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for isize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_isize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => {
                match E::C::ENDIAN {
                    Endian::Big => encoder.writer().write(&(*self as i64).to_be_bytes()),
                    Endian::Little => {
                        encoder.writer().write(&(*self as i64).to_le_bytes())
                    }
                }
            }
        }
    }
}
impl Encode for NonZeroIsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}
impl Encode for f32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}
impl Encode for f64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}
impl Encode for char {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_utf8(encoder.writer(), *self)
    }
}
impl<T> Encode for [T]
where
    T: Encode + 'static,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        super::encode_slice_len(encoder, self.len())?;
        if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u8>() {
            let t: &[u8] = unsafe { core::mem::transmute(self) };
            encoder.writer().write(t)?;
            return Ok(());
        }
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;
fn encode_utf8(writer: &mut impl Writer, c: char) -> Result<(), EncodeError> {
    let code = c as u32;
    if code < MAX_ONE_B {
        writer.write(&[c as u8])
    } else if code < MAX_TWO_B {
        let mut buf = [0u8; 2];
        buf[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[1] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else if code < MAX_THREE_B {
        let mut buf = [0u8; 3];
        buf[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else {
        let mut buf = [0u8; 4];
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    }
}
impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}
impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        super::encode_option_variant(encoder, self)?;
        if let Some(val) = self {
            val.encode(encoder)?;
        }
        Ok(())
    }
}
impl<T, U> Encode for Result<T, U>
where
    T: Encode,
    U: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Ok(val) => {
                0u32.encode(encoder)?;
                val.encode(encoder)
            }
            Err(err) => {
                1u32.encode(encoder)?;
                err.encode(encoder)
            }
        }
    }
}
impl<T> Encode for Cell<T>
where
    T: Encode + Copy,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(&self.get(), encoder)
    }
}
impl<T> Encode for RefCell<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let borrow_guard = self
            .try_borrow()
            .map_err(|e| EncodeError::RefCellAlreadyBorrowed {
                inner: e,
                type_name: core::any::type_name::<RefCell<T>>(),
            })?;
        T::encode(&borrow_guard, encoder)
    }
}
impl Encode for Duration {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_secs().encode(encoder)?;
        self.subsec_nanos().encode(encoder)?;
        Ok(())
    }
}
impl<T> Encode for Range<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start.encode(encoder)?;
        self.end.encode(encoder)?;
        Ok(())
    }
}
impl<T> Encode for RangeInclusive<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start().encode(encoder)?;
        self.end().encode(encoder)?;
        Ok(())
    }
}
impl<T> Encode for Bound<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Self::Unbounded => {
                0u32.encode(encoder)?;
            }
            Self::Included(val) => {
                1u32.encode(encoder)?;
                val.encode(encoder)?;
            }
            Self::Excluded(val) => {
                2u32.encode(encoder)?;
                val.encode(encoder)?;
            }
        }
        Ok(())
    }
}
impl<'a, T> Encode for &'a T
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
#[cfg(test)]
mod tests_llm_16_238 {
    use crate::{
        enc::{Encode, Encoder},
        enc::encoder::EncoderImpl, config::{self, Configuration},
        enc::write::SizeWriter, error::EncodeError, utils::Sealed,
    };
    use std::num::NonZeroI16;
    struct TestEncoder {
        writer: SizeWriter,
        config: Configuration,
    }
    impl Encoder for TestEncoder {
        type W = SizeWriter;
        type C = Configuration;
        fn writer(&mut self) -> &mut Self::W {
            &mut self.writer
        }
        fn config(&self) -> &Self::C {
            &self.config
        }
    }
    impl Sealed for TestEncoder {}
    #[test]
    fn encode_non_zero_i16() -> Result<(), EncodeError> {
        let non_zero_i16 = NonZeroI16::new(42).unwrap();
        let config = config::standard();
        let writer = SizeWriter { bytes_written: 0 };
        let mut encoder = TestEncoder { writer, config };
        non_zero_i16.encode(&mut encoder)?;
        assert_eq!(encoder.writer.bytes_written, 2);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_261 {
    use super::*;
    use crate::*;
    use crate::enc::write::SizeWriter;
    use crate::enc::write::Writer;
    use crate::enc::EncodeError;
    use crate::enc::impls::encode_utf8;
    #[test]
    fn test_encode_utf8_single_byte() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = SizeWriter {
            bytes_written: rug_fuzz_0,
        };
        debug_assert!(encode_utf8(& mut writer, rug_fuzz_1).is_ok());
        debug_assert_eq!(writer.bytes_written, 1);
             }
});    }
    #[test]
    fn test_encode_utf8_two_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = SizeWriter {
            bytes_written: rug_fuzz_0,
        };
        debug_assert!(encode_utf8(& mut writer, rug_fuzz_1).is_ok());
        debug_assert_eq!(writer.bytes_written, 2);
             }
});    }
    #[test]
    fn test_encode_utf8_three_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = SizeWriter {
            bytes_written: rug_fuzz_0,
        };
        debug_assert!(encode_utf8(& mut writer, rug_fuzz_1).is_ok());
        debug_assert_eq!(writer.bytes_written, 3);
             }
});    }
    #[test]
    fn test_encode_utf8_four_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, char) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut writer = SizeWriter {
            bytes_written: rug_fuzz_0,
        };
        debug_assert!(encode_utf8(& mut writer, rug_fuzz_1).is_ok());
        debug_assert_eq!(writer.bytes_written, 4);
             }
});    }
}
#[cfg(test)]
mod tests_rug_210 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer, Encoder};
    use crate::config::{Configuration, BigEndian, Fixint};
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = ();
        let mut v9 = Configuration::<
            BigEndian,
            Fixint,
            crate::config::Limit<1024>,
        >::default();
        let v10 = VecWriter::with_capacity(rug_fuzz_0);
        let mut p1 = EncoderImpl::new(v10, v9);
        debug_assert!(< () > ::encode(& p0, & mut p1).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_211 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::marker::PhantomData;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: PhantomData<u32> = PhantomData;
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_0);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <std::marker::PhantomData<u32>>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_212 {
    use super::*;
    use crate::enc::impls::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    use crate::error::EncodeError;
    #[test]
    fn test_encode_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: bool = rug_fuzz_0;
        let p1_config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let p1_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1_encoder = EncoderImpl::new(p1_writer, p1_config);
        debug_assert!(< bool as Encode > ::encode(& p0, & mut p1_encoder).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_213 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        debug_assert!(< u8 as Encode > ::encode(& p0, & mut p1).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use crate::Encode;
    use crate::config::Configuration;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::features::VecWriter;
    use std::num::NonZeroU8;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: NonZeroU8 = NonZeroU8::new(rug_fuzz_0).unwrap();
        let mut p1: EncoderImpl<VecWriter, Configuration> = EncoderImpl::new(
            VecWriter::with_capacity(rug_fuzz_1),
            Configuration::default(),
        );
        NonZeroU8::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    use crate::enc::{EncoderImpl, write::Writer, Encode, EncodeError};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    use crate::enc;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u16 = rug_fuzz_0;
        let mut config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        let result = <u16 as Encode>::encode(&p0, &mut p1);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_216 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::num::NonZeroU16;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroU16::new(rug_fuzz_0).unwrap();
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        NonZeroU16::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u32 = rug_fuzz_0;
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(&mut writer, config);
        p0.encode(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use crate::Encode;
    use std::num::NonZeroU32;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroU32::new(rug_fuzz_0).unwrap();
        let mut p1 = {
            let mut v9 = Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default();
            let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
            EncoderImpl::new(v10, v9)
        };
        NonZeroU32::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u64 = rug_fuzz_0;
        let mut v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        <u64>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    use crate::Encode;
    use std::num::NonZeroU64;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroU64::new(rug_fuzz_0).unwrap();
        let mut p1 = {
            let mut v9 = Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default();
            let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
            EncoderImpl::new(v10, v9)
        };
        <NonZeroU64 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_221 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: u128 = rug_fuzz_0;
        let mut p1: EncoderImpl<
            VecWriter,
            Configuration<BigEndian, Fixint, Limit<1024>>,
        > = EncoderImpl::new(
            VecWriter::with_capacity(rug_fuzz_1),
            Configuration::default(),
        );
        <u128 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_222 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::num::NonZeroU128;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v2 = NonZeroU128::new(rug_fuzz_0).unwrap();
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut encoder_impl = EncoderImpl::new(vec_writer, config);
        NonZeroU128::encode(&v2, &mut encoder_impl).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut encoder = EncoderImpl::new(vec_writer, config);
        p0.encode(&mut encoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_224 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = std::num::NonZeroUsize::new(rug_fuzz_0).unwrap();
        let mut p1_config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p1_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(p1_writer, p1_config);
        <std::num::NonZeroUsize as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_226 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::num::NonZeroI8;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i8, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroI8::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_2);
        let mut p1 = EncoderImpl::new(writer, config);
        p0.encode(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_227 {
    use crate::enc::{self, Encode, EncoderImpl};
    use crate::enc::write::Writer;
    use crate::config::{self, Configuration};
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i16, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: i16 = rug_fuzz_0;
        let configuration = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, configuration);
        p0.encode(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_228 {
    use super::*;
    use crate::Encode;
    use crate::enc::{self, EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i32 = rug_fuzz_0;
        let mut config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <i32 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_229 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, Encoder, EncodeError, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = std::num::NonZeroI32::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_2);
        let mut p1 = EncoderImpl::new(v10, v9);
        <std::num::NonZeroI32 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_230 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i64 = rug_fuzz_0;
        let mut cfg = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, cfg);
        <i64>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_231 {
    use super::*;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::num::NonZeroI64;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = NonZeroI64::new(rug_fuzz_0).unwrap();
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(&mut writer, config);
        NonZeroI64::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_232 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i128, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: i128 = rug_fuzz_0;
        let mut p1 = EncoderImpl::new(
            VecWriter::with_capacity(rug_fuzz_1),
            Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default(),
        );
        <i128>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_233 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::num::NonZeroI128;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i128, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroI128::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mut p1 = {
            let v9 = Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default();
            let v10 = VecWriter::with_capacity(rug_fuzz_2);
            EncoderImpl::new(v10, v9)
        };
        <NonZeroI128 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_234 {
    use super::*;
    use crate::Encode;
    use crate::enc::{self, EncoderImpl, write::Writer};
    use crate::config::{self, Configuration};
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(isize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: isize = rug_fuzz_0;
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        p0.encode(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_235 {
    use super::*;
    use std::num::NonZeroIsize;
    use crate::enc::{EncoderImpl, write::Writer, Encoder, EncodeError};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_nonzero_isize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(isize, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = NonZeroIsize::new(rug_fuzz_0).expect(rug_fuzz_1);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_2);
        let mut p1 = EncoderImpl::new(writer, config);
        let _result = <std::num::NonZeroIsize as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_236 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f32 = rug_fuzz_0;
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <f32 as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_237 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(f64, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: f64 = rug_fuzz_0;
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        let result = <f64 as Encode>::encode(&p0, &mut p1);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_238 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(char, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: char = rug_fuzz_0;
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        p0.encode(&mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_239 {
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v103: Vec<i32> = Vec::new();
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_0);
        let mut v19 = EncoderImpl::new(v10, v9);
        v103.encode(&mut v19).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_240 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: &str = rug_fuzz_0;
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut encoder = EncoderImpl::new(writer, config);
        <str>::encode(&p0, &mut encoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_241 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let mut p1: EncoderImpl<
            VecWriter,
            Configuration<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >,
        > = EncoderImpl::new(
            VecWriter::with_capacity(rug_fuzz_4),
            Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default(),
        );
        <[u8; 4]>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_242 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::{option::Option, result::Result};
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: i32 = rug_fuzz_0;
        let mut p0: Option<i32> = Some(data);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        debug_assert!(matches!(p0.encode(& mut p1), Result::Ok(_)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_244 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::cell::Cell;
    #[test]
    fn test_encode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v107 = Cell::new(rug_fuzz_0);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut v19 = EncoderImpl::new(v10, v9);
        <Cell<i32> as Encode>::encode(&v107, &mut v19).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_245 {
    use super::*;
    use crate::enc::{EncoderImpl, write::Writer, Encoder};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::cell::RefCell;
    #[test]
    fn test_encode_refcell() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = RefCell::new(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        <RefCell<i32> as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_246 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::time::Duration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Duration = Duration::new(rug_fuzz_0, rug_fuzz_1);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_2);
        let mut p1 = EncoderImpl::new(writer, config);
        let result = p0.encode(&mut p1);
        debug_assert!(result.is_ok(), "Encoding should be successful");
             }
});    }
}
#[cfg(test)]
mod tests_rug_247 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, Encoder, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = std::ops::Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let mut p1 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p2 = VecWriter::with_capacity(rug_fuzz_2);
        let mut p3 = EncoderImpl::new(p2, p1);
        <std::ops::Range<i32>>::encode(&p0, &mut p3).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_248 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::ops::RangeInclusive;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut range: RangeInclusive<i32> = rug_fuzz_0..=rug_fuzz_1;
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut writer = VecWriter::with_capacity(rug_fuzz_2);
        let mut encoder = EncoderImpl::new(writer, config);
        RangeInclusive::<i32>::encode(&range, &mut encoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_249 {
    use crate::enc::{EncoderImpl, write::Writer, Encode, EncodeError};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    use std::collections::Bound;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Bound<i32> = Bound::Included(rug_fuzz_0);
        let mut p1 = {
            let cfg = Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default();
            let writer = VecWriter::with_capacity(rug_fuzz_1);
            EncoderImpl::new(writer, cfg)
        };
        <Bound<i32> as Encode>::encode(&p0, &mut p1).unwrap();
             }
});    }
}
