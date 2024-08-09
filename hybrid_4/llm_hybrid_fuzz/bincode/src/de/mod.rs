//! Decoder-based structs and traits.
mod decoder;
mod impl_core;
mod impl_tuples;
mod impls;
use self::read::{BorrowReader, Reader};
use crate::{
    config::{Config, InternalLimitConfig},
    error::DecodeError, utils::Sealed,
};
pub mod read;
pub use self::decoder::DecoderImpl;
/// Trait that makes a type able to be decoded, akin to serde's `DeserializeOwned` trait.
///
/// This trait should be implemented for types which do not have references to data in the reader. For types that contain e.g. `&str` and `&[u8]`, implement [BorrowDecode] instead.
///
/// Whenever you implement `Decode` for your type, the base trait `BorrowDecode` is automatically implemented.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to your type. Note that if the type contains any lifetimes, `BorrowDecode` will be implemented instead.
///
/// # Implementing this trait manually
///
/// If you want to implement this trait for your type, the easiest way is to add a `#[derive(bincode::Decode)]`, build and check your `target/generated/bincode/` folder. This should generate a `<Struct name>_Decode.rs` file.
///
/// For this struct:
///
/// ```
/// struct Entity {
///     pub x: f32,
///     pub y: f32,
/// }
/// ```
///
/// It will look something like:
///
/// ```
/// # struct Entity {
/// #     pub x: f32,
/// #     pub y: f32,
/// # }
/// impl bincode::Decode for Entity {
///     fn decode<D: bincode::de::Decoder>(
///         decoder: &mut D,
///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
///         Ok(Self {
///             x: bincode::Decode::decode(decoder)?,
///             y: bincode::Decode::decode(decoder)?,
///         })
///     }
/// }
/// impl<'de> bincode::BorrowDecode<'de> for Entity {
///     fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
///         decoder: &mut D,
///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
///         Ok(Self {
///             x: bincode::BorrowDecode::borrow_decode(decoder)?,
///             y: bincode::BorrowDecode::borrow_decode(decoder)?,
///         })
///     }
/// }
/// ```
///
/// From here you can add/remove fields, or add custom logic.
///
/// To get specific integer types, you can use:
/// ```
/// # struct Foo;
/// # impl bincode::Decode for Foo {
/// #     fn decode<D: bincode::de::Decoder>(
/// #         decoder: &mut D,
/// #     ) -> core::result::Result<Self, bincode::error::DecodeError> {
/// let x: u8 = bincode::Decode::decode(decoder)?;
/// let x = <u8 as bincode::Decode>::decode(decoder)?;
/// #         Ok(Foo)
/// #     }
/// # }
/// # bincode::impl_borrow_decode!(Foo);
/// ```
pub trait Decode: Sized {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError>;
}
/// Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait.
///
/// This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decode] instead.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to a type with a lifetime.
pub trait BorrowDecode<'de>: Sized {
    /// Attempt to decode this type with the given [BorrowDecode].
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError>;
}
/// Helper macro to implement `BorrowDecode` for any type that implements `Decode`.
#[macro_export]
macro_rules! impl_borrow_decode {
    ($ty:ty $(, $param:ident),*) => {
        impl <'de $(, $param)*> $crate ::BorrowDecode <'de > for $ty { fn borrow_decode <
        D : $crate ::de::BorrowDecoder <'de >> (decoder : & mut D,) ->
        core::result::Result < Self, $crate ::error::DecodeError > { $crate
        ::Decode::decode(decoder) } }
    };
}
/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
pub trait Decoder: Sealed {
    /// The concrete [Reader] type
    type R: Reader;
    /// The concrete [Config] type
    type C: Config;
    /// Returns a mutable reference to the reader
    fn reader(&mut self) -> &mut Self::R;
    /// Returns a reference to the config
    fn config(&self) -> &Self::C;
    /// Claim that `n` bytes are going to be read from the decoder.
    /// This can be used to validate `Configuration::Limit<N>()`.
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError>;
    /// Claim that we're going to read a container which contains `len` entries of `T`.
    /// This will correctly handle overflowing if `len * size_of::<T>() > usize::max_value`
    fn claim_container_read<T>(&mut self, len: usize) -> Result<(), DecodeError> {
        if <Self::C as InternalLimitConfig>::LIMIT.is_some() {
            match len.checked_mul(core::mem::size_of::<T>()) {
                Some(val) => self.claim_bytes_read(val),
                None => Err(DecodeError::LimitExceeded),
            }
        } else {
            Ok(())
        }
    }
    /// Notify the decoder that `n` bytes are being reclaimed.
    ///
    /// When decoding container types, a typical implementation would claim to read `len * size_of::<T>()` bytes.
    /// This is to ensure that bincode won't allocate several GB of memory while constructing the container.
    ///
    /// Because the implementation claims `len * size_of::<T>()`, but then has to decode each `T`, this would be marked
    /// as double. This function allows us to un-claim each `T` that gets decoded.
    ///
    /// We cannot check if `len * size_of::<T>()` is valid without claiming it, because this would mean that if you have
    /// a nested container (e.g. `Vec<Vec<T>>`), it does not know how much memory is already claimed, and could easily
    /// allocate much more than the user intends.
    /// ```
    /// # use bincode::de::{Decode, Decoder};
    /// # use bincode::error::DecodeError;
    /// # struct Container<T>(Vec<T>);
    /// # impl<T> Container<T> {
    /// #     fn with_capacity(cap: usize) -> Self {
    /// #         Self(Vec::with_capacity(cap))
    /// #     }
    /// #
    /// #     fn push(&mut self, t: T) {
    /// #         self.0.push(t);
    /// #     }
    /// # }
    /// impl<T: Decode> Decode for Container<T> {
    ///     fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
    ///         let len = u64::decode(decoder)?;
    ///         let len: usize = len.try_into().map_err(|_| DecodeError::OutsideUsizeRange(len))?;
    ///         // Make sure we don't allocate too much memory
    ///         decoder.claim_bytes_read(len * core::mem::size_of::<T>());
    ///
    ///         let mut result = Container::with_capacity(len);
    ///         for _ in 0..len {
    ///             // un-claim the memory
    ///             decoder.unclaim_bytes_read(core::mem::size_of::<T>());
    ///             result.push(T::decode(decoder)?)
    ///         }
    ///         Ok(result)
    ///     }
    /// }
    /// impl<'de, T: bincode::BorrowDecode<'de>> bincode::BorrowDecode<'de> for Container<T> {
    ///     fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
    ///         decoder: &mut D,
    ///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
    ///         let len = u64::borrow_decode(decoder)?;
    ///         let len: usize = len.try_into().map_err(|_| DecodeError::OutsideUsizeRange(len))?;
    ///         // Make sure we don't allocate too much memory
    ///         decoder.claim_bytes_read(len * core::mem::size_of::<T>());
    ///
    ///         let mut result = Container::with_capacity(len);
    ///         for _ in 0..len {
    ///             // un-claim the memory
    ///             decoder.unclaim_bytes_read(core::mem::size_of::<T>());
    ///             result.push(T::borrow_decode(decoder)?)
    ///         }
    ///         Ok(result)
    ///     }
    /// }
    /// ```
    fn unclaim_bytes_read(&mut self, n: usize);
}
/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
///
/// This is an extension of [Decode] that can also return borrowed data.
pub trait BorrowDecoder<'de>: Decoder {
    /// The concrete [BorrowReader] type
    type BR: BorrowReader<'de>;
    /// Rerturns a mutable reference to the borrow reader
    fn borrow_reader(&mut self) -> &mut Self::BR;
}
impl<'a, T> Decoder for &'a mut T
where
    T: Decoder,
{
    type R = T::R;
    type C = T::C;
    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }
    fn config(&self) -> &Self::C {
        T::config(self)
    }
    #[inline]
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError> {
        T::claim_bytes_read(self, n)
    }
    #[inline]
    fn unclaim_bytes_read(&mut self, n: usize) {
        T::unclaim_bytes_read(self, n)
    }
}
impl<'a, 'de, T> BorrowDecoder<'de> for &'a mut T
where
    T: BorrowDecoder<'de>,
{
    type BR = T::BR;
    fn borrow_reader(&mut self) -> &mut Self::BR {
        T::borrow_reader(self)
    }
}
/// Decodes only the option variant from the decoder. Will not read any more data than that.
#[inline]
pub(crate) fn decode_option_variant<D: Decoder>(
    decoder: &mut D,
    type_name: &'static str,
) -> Result<Option<()>, DecodeError> {
    let is_some = u8::decode(decoder)?;
    match is_some {
        0 => Ok(None),
        1 => Ok(Some(())),
        x => {
            Err(DecodeError::UnexpectedVariant {
                found: x as u32,
                allowed: &crate::error::AllowedEnumVariants::Range {
                    max: 1,
                    min: 0,
                },
                type_name,
            })
        }
    }
}
/// Decodes the length of any slice, container, etc from the decoder
#[inline]
pub(crate) fn decode_slice_len<D: Decoder>(
    decoder: &mut D,
) -> Result<usize, DecodeError> {
    let v = u64::decode(decoder)?;
    v.try_into().map_err(|_| DecodeError::OutsideUsizeRange(v))
}
#[cfg(test)]
mod tests_llm_16_34_llm_16_34 {
    use crate::{
        config, de::{BorrowDecoder, DecoderImpl},
        error::DecodeError, de::BorrowDecode,
    };
    use crate::de::read::SliceReader;
    use std::sync::atomic::AtomicI8;
    #[test]
    fn borrow_decode_atomic_i8() {
        let _rug_st_tests_llm_16_34_llm_16_34_rrrruuuugggg_borrow_decode_atomic_i8 = 0;
        let encoded_data = vec![0; 1];
        let mut reader = SliceReader::new(&encoded_data);
        let config = config::standard().with_big_endian();
        let mut decoder = DecoderImpl::new(reader, config);
        let atomic_i8_result = AtomicI8::borrow_decode(&mut decoder);
        debug_assert!(atomic_i8_result.is_ok());
        debug_assert_eq!(
            atomic_i8_result.unwrap().load(std::sync::atomic::Ordering::SeqCst), 0
        );
        let _rug_ed_tests_llm_16_34_llm_16_34_rrrruuuugggg_borrow_decode_atomic_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_117_llm_16_117 {
    use crate::de::{BorrowDecode, BorrowDecoder, DecodeError, decoder::DecoderImpl};
    use crate::config::{Config, Configuration, LittleEndian, Varint, NoLimit};
    use crate::de::read::SliceReader;
    #[test]
    fn test_borrow_decode_bool() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = [rug_fuzz_0];
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default()
            .with_big_endian();
        let reader = SliceReader::new(&input);
        let mut decoder = DecoderImpl::new(reader, config);
        let result = bool::borrow_decode(&mut decoder);
        debug_assert_eq!(result.unwrap(), true);
             }
});    }
    #[test]
    fn test_borrow_decode_bool_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = [rug_fuzz_0];
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default()
            .with_big_endian();
        let reader = SliceReader::new(&input);
        let mut decoder = DecoderImpl::new(reader, config);
        let result = bool::borrow_decode(&mut decoder);
        debug_assert!(
            matches!(result, Err(DecodeError::UnexpectedVariant { found : 2, .. }))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_124_llm_16_124 {
    use super::*;
    use crate::*;
    use crate::de::BorrowDecoder;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    use crate::config::{Config, Configuration, BigEndian, LittleEndian, Varint, NoLimit};
    #[test]
    fn test_borrow_decode_i64() -> Result<(), DecodeError> {
        let data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Varint, NoLimit>::default()
            .with_big_endian();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = i64::borrow_decode(&mut decoder)?;
        assert_eq!(result, 1);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_130_llm_16_130 {
    use super::*;
    use crate::*;
    use crate::de::{BorrowDecode, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Config, Configuration, LittleEndian, Varint, NoLimit};
    use crate::error::DecodeError;
    use std::marker::PhantomData;
    #[test]
    fn borrow_decode_for_phantom_data() {
        let _rug_st_tests_llm_16_130_llm_16_130_rrrruuuugggg_borrow_decode_for_phantom_data = 0;
        let input: &[u8] = &[];
        let reader = SliceReader::new(input);
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default()
            .with_big_endian();
        let mut decoder = DecoderImpl::new(reader, config);
        let result: Result<PhantomData<()>, DecodeError> = PhantomData::borrow_decode(
            &mut decoder,
        );
        debug_assert!(result.is_ok());
        let _rug_ed_tests_llm_16_130_llm_16_130_rrrruuuugggg_borrow_decode_for_phantom_data = 0;
    }
}
#[cfg(test)]
mod tests_rug_153 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_decode_option_variant() {
        let _rug_st_tests_rug_153_rrrruuuugggg_test_decode_option_variant = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = "Option<()>";
        let rug_fuzz_6 = 1u8;
        let rug_fuzz_7 = 2u8;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut slice_reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(slice_reader, config);
        let type_name = rug_fuzz_5;
        let result = crate::de::decode_option_variant(&mut decoder, type_name);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), None);
        let data_some = [rug_fuzz_6];
        let mut slice_reader_some = SliceReader::new(&data_some);
        let mut decoder_some = DecoderImpl::new(slice_reader_some, config);
        let result_some = crate::de::decode_option_variant(&mut decoder_some, type_name);
        debug_assert!(result_some.is_ok());
        debug_assert_eq!(result_some.unwrap(), Some(()));
        let data_invalid = [rug_fuzz_7];
        let mut slice_reader_invalid = SliceReader::new(&data_invalid);
        let mut decoder_invalid = DecoderImpl::new(slice_reader_invalid, config);
        let result_invalid = crate::de::decode_option_variant(
            &mut decoder_invalid,
            type_name,
        );
        debug_assert!(result_invalid.is_err());
        let _rug_ed_tests_rug_153_rrrruuuugggg_test_decode_option_variant = 0;
    }
}
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = crate::config::Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _ = crate::de::decode_slice_len(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_155 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, DecodeError};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_claim_container_read() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let p1: usize = rug_fuzz_5;
        debug_assert!(matches!(p0.claim_container_read:: < u32 > (p1), Ok(())));
             }
});    }
}
#[cfg(test)]
mod tests_rug_156 {
    use crate::de::{
        self, BorrowDecode, BorrowDecoder, read::SliceReader, decoder::DecoderImpl,
    };
    use crate::config::Configuration;
    use std::sync::atomic::AtomicBool;
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = AtomicBool::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_157 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecoder};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        <std::sync::atomic::AtomicU8 as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_158 {
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    use std::sync::atomic::AtomicU16;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let _result = AtomicU16::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_159 {
    use super::*;
    use crate::de::{self, BorrowDecoder, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _: Result<std::sync::atomic::AtomicU32, _> = <std::sync::atomic::AtomicU32 as BorrowDecode>::borrow_decode(
            &mut p0,
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_160 {
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config;
    use std::sync::atomic::AtomicU64;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = AtomicU64::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_161 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecode};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let v11 = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(v11, config);
        <std::sync::atomic::AtomicUsize as BorrowDecode>::borrow_decode(&mut p0)
            .unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_162 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0_reader = SliceReader::new(&data);
        let p0_config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(p0_reader, p0_config);
        <std::sync::atomic::AtomicI16 as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_164 {
    use crate::BorrowDecode;
    use crate::de::{self, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{self, BigEndian, Fixint, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, config::Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _ = <std::sync::atomic::AtomicI64 as de::BorrowDecode>::borrow_decode(
            &mut p0,
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_165 {
    use super::*;
    use crate::de::{self, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = <std::sync::atomic::AtomicIsize as BorrowDecode>::borrow_decode(
            &mut p0,
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_166 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::{self, read::SliceReader, DecoderImpl};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <std::string::String as BorrowDecode>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_167 {
    use super::*;
    use crate::de::{decoder::DecoderImpl, read::SliceReader};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::{Decode, BorrowDecode};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <Box<str>>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_168 {
    use crate::de::BorrowDecode;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        <std::ffi::CString>::borrow_decode(&mut decoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_169 {
    use super::*;
    use crate::de::{self, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let v11 = SliceReader::new(&data);
        let v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        <std::time::SystemTime as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_170 {
    use super::*;
    use crate::de::{self, BorrowDecoder, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{BigEndian, Fixint, Configuration, Limit};
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut slice_reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder_impl = DecoderImpl::new(slice_reader, config);
        let result = <std::path::PathBuf as BorrowDecode>::borrow_decode(
            &mut decoder_impl,
        );
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_171 {
    use super::*;
    use crate::de::{self, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        <std::net::IpAddr as BorrowDecode>::borrow_decode(&mut decoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_172 {
    use super::*;
    use crate::BorrowDecode;
    use crate::config::Configuration;
    use crate::de::{self, DecoderImpl, read::SliceReader};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let result = <std::net::Ipv4Addr as BorrowDecode>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_173 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecoder};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let slice_reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(slice_reader, config);
        let _result = <std::net::Ipv6Addr as BorrowDecode>::borrow_decode(&mut decoder);
             }
});    }
}
#[cfg(test)]
mod tests_rug_174 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::read::SliceReader;
    use crate::de::{BorrowDecoder, DecoderImpl};
    use crate::config::{BigEndian, Configuration, Fixint};
    use crate::config;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let v11 = SliceReader::new(&data);
        let v9 = Configuration::<BigEndian, Fixint, config::Limit<1024>>::default();
        let mut v12 = DecoderImpl::new(v11, v9);
        let mut p0 = &mut v12;
        <std::net::SocketAddr as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_175 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        <std::net::SocketAddrV4 as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_177 {
    use crate::de::{self, BorrowDecode};
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::{Decode, config::{self, Configuration}};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0_reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(p0_reader, config);
        debug_assert!(< u8 as BorrowDecode > ::borrow_decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_178 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::BorrowDecoder;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut sr = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(sr, config);
        <std::num::NonZeroU8 as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_179 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::{self, read::SliceReader, DecoderImpl};
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(v11, config);
        debug_assert!(u16::borrow_decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_180 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use std::num::NonZeroU16;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(NonZeroU16::borrow_decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use crate::de::{self, read::SliceReader, BorrowDecode, BorrowDecoder, DecoderImpl};
    use crate::error::DecodeError;
    use crate::config::{self, BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut slice_reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(slice_reader, config);
        let result = <u32 as BorrowDecode>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        debug_assert!(< std::num::NonZeroU32 > ::borrow_decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::<SliceReader<'_>, _>::new(reader, config);
        debug_assert!(
            < u64 as de::BorrowDecode > ::borrow_decode(& mut decoder).is_ok()
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_184 {
    use super::*;
    use crate::de::{
        BorrowDecode, BorrowDecoder, read::SliceReader, decoder::DecoderImpl,
    };
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        <std::num::NonZeroU64 as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use crate::de::{self, BorrowDecode, BorrowDecoder, DecoderImpl, read::SliceReader};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        <u128>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_186 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::{self, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_borrow_decode_nonzero_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <std::num::NonZeroU128>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecode};
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = crate::config::Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let result = <usize as BorrowDecode>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_188 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecoder};
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        <std::num::NonZeroUsize as BorrowDecode>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::BorrowDecode;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::de::read::SliceReader;
        use crate::de::DecoderImpl;
        use crate::config::Configuration;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let result = <i8 as BorrowDecode>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0_reader = SliceReader::new(&data);
        let mut p0_config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(p0_reader, p0_config);
        <std::num::NonZeroI8>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        <i16>::borrow_decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use crate::de::{self, BorrowDecoder, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, i16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let decode_result = <std::num::NonZeroI16 as BorrowDecode>::borrow_decode(
            &mut p0,
        );
        debug_assert!(decode_result.is_ok());
        if let Ok(non_zero) = decode_result {
            debug_assert!(non_zero.get() > rug_fuzz_5);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    #[test]
    fn test_borrow_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <i32 as BorrowDecode>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_194 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let _result = <std::num::NonZeroI32>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_195 {
    use super::*;
    use crate::de::{self, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, config);
        let _result = <std::num::NonZeroI64 as BorrowDecode>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_196 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <i128 as BorrowDecode>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_197 {
    use crate::de::BorrowDecode;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let config = crate::config::Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, config);
        let result = <std::num::NonZeroI128 as BorrowDecode>::borrow_decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_199 {
    use super::*;
    use crate::de::{self, BorrowDecoder, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _ = <std::num::NonZeroIsize as BorrowDecode>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_200 {
    use super::*;
    use crate::de::{self, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut dec_impl = DecoderImpl::new(reader, config);
        <f32 as BorrowDecode>::borrow_decode(&mut dec_impl).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_201 {
    use crate::de::{self, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        <f64 as BorrowDecode>::borrow_decode(&mut decoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_202 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_borrow_decode_for_char() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut slice_reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(slice_reader, config);
        let result = <char>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecode};
    use crate::de;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = <()>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_205 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    #[test]
    fn test_reader() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        p0.reader();
             }
});    }
}
#[cfg(test)]
mod tests_rug_206 {
    use super::*;
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_config() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        p0.config();
             }
});    }
}
#[cfg(test)]
mod tests_rug_207 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, Decoder};
    use crate::config::Configuration;
    use crate::de;
    #[test]
    fn test_claim_bytes_read() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder_impl = DecoderImpl::new(reader, config);
        let n: usize = rug_fuzz_5;
        decoder_impl.claim_bytes_read(n).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_208 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_unclaim_bytes_read() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let p1: usize = rug_fuzz_5;
        p0.unclaim_bytes_read(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_209 {
    use super::*;
    use crate::de::{BorrowDecoder, DecoderImpl, read::SliceReader};
    use crate::config::{BigEndian, Configuration, Fixint};
    use crate::de;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = Configuration::<BigEndian, Fixint>::default();
        let mut p0 = DecoderImpl::<
            SliceReader,
            Configuration<BigEndian, Fixint>,
        >::new(v11, v9);
        p0.borrow_reader();
             }
});    }
}
