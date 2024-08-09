use super::{
    read::{BorrowReader, Reader},
    BorrowDecode, BorrowDecoder, Decode, Decoder,
};
use crate::{
    config::{Endian, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::{DecodeError, IntegerType},
    impl_borrow_decode,
};
use core::{
    any::TypeId, cell::{Cell, RefCell},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};
impl Decode for bool {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u8::decode(decoder)? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(DecodeError::InvalidBooleanValue(x)),
        }
    }
}
impl_borrow_decode!(bool);
impl Decode for u8 {
    #[inline]
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(1)?;
        if let Some(buf) = decoder.reader().peek_read(1) {
            let byte = buf[0];
            decoder.reader().consume(1);
            Ok(byte)
        } else {
            let mut bytes = [0u8; 1];
            decoder.reader().read(&mut bytes)?;
            Ok(bytes[0])
        }
    }
}
impl_borrow_decode!(u8);
impl Decode for NonZeroU8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU8::new(u8::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::U8,
            })
    }
}
impl_borrow_decode!(NonZeroU8);
impl Decode for u16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(2)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => u16::from_le_bytes(bytes),
                        Endian::Big => u16::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(u16);
impl Decode for NonZeroU16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU16::new(u16::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::U16,
            })
    }
}
impl_borrow_decode!(NonZeroU16);
impl Decode for u32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => u32::from_le_bytes(bytes),
                        Endian::Big => u32::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(u32);
impl Decode for NonZeroU32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU32::new(u32::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::U32,
            })
    }
}
impl_borrow_decode!(NonZeroU32);
impl Decode for u64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => u64::from_le_bytes(bytes),
                        Endian::Big => u64::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(u64);
impl Decode for NonZeroU64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU64::new(u64::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::U64,
            })
    }
}
impl_borrow_decode!(NonZeroU64);
impl Decode for u128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(16)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => u128::from_le_bytes(bytes),
                        Endian::Big => u128::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(u128);
impl Decode for NonZeroU128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroU128::new(u128::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::U128,
            })
    }
}
impl_borrow_decode!(NonZeroU128);
impl Decode for usize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_usize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                let value = match D::C::ENDIAN {
                    Endian::Little => u64::from_le_bytes(bytes),
                    Endian::Big => u64::from_be_bytes(bytes),
                };
                value.try_into().map_err(|_| DecodeError::OutsideUsizeRange(value))
            }
        }
    }
}
impl_borrow_decode!(usize);
impl Decode for NonZeroUsize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroUsize::new(usize::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::Usize,
            })
    }
}
impl_borrow_decode!(NonZeroUsize);
impl Decode for i8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(1)?;
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }
}
impl_borrow_decode!(i8);
impl Decode for NonZeroI8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI8::new(i8::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::I8,
            })
    }
}
impl_borrow_decode!(NonZeroI8);
impl Decode for i16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(2)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => i16::from_le_bytes(bytes),
                        Endian::Big => i16::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(i16);
impl Decode for NonZeroI16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI16::new(i16::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::I16,
            })
    }
}
impl_borrow_decode!(NonZeroI16);
impl Decode for i32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => i32::from_le_bytes(bytes),
                        Endian::Big => i32::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(i32);
impl Decode for NonZeroI32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI32::new(i32::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::I32,
            })
    }
}
impl_borrow_decode!(NonZeroI32);
impl Decode for i64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => i64::from_le_bytes(bytes),
                        Endian::Big => i64::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(i64);
impl Decode for NonZeroI64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI64::new(i64::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::I64,
            })
    }
}
impl_borrow_decode!(NonZeroI64);
impl Decode for i128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(16)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => i128::from_le_bytes(bytes),
                        Endian::Big => i128::from_be_bytes(bytes),
                    },
                )
            }
        }
    }
}
impl_borrow_decode!(i128);
impl Decode for NonZeroI128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroI128::new(i128::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::I128,
            })
    }
}
impl_borrow_decode!(NonZeroI128);
impl Decode for isize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_isize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(
                    match D::C::ENDIAN {
                        Endian::Little => i64::from_le_bytes(bytes),
                        Endian::Big => i64::from_be_bytes(bytes),
                    } as isize,
                )
            }
        }
    }
}
impl_borrow_decode!(isize);
impl Decode for NonZeroIsize {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        NonZeroIsize::new(isize::decode(decoder)?)
            .ok_or(DecodeError::NonZeroTypeIsZero {
                non_zero_type: IntegerType::Isize,
            })
    }
}
impl_borrow_decode!(NonZeroIsize);
impl Decode for f32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(4)?;
        let mut bytes = [0u8; 4];
        decoder.reader().read(&mut bytes)?;
        Ok(
            match D::C::ENDIAN {
                Endian::Little => f32::from_le_bytes(bytes),
                Endian::Big => f32::from_be_bytes(bytes),
            },
        )
    }
}
impl_borrow_decode!(f32);
impl Decode for f64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(8)?;
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(
            match D::C::ENDIAN {
                Endian::Little => f64::from_le_bytes(bytes),
                Endian::Big => f64::from_be_bytes(bytes),
            },
        )
    }
}
impl_borrow_decode!(f64);
impl Decode for char {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut array = [0u8; 4];
        decoder.reader().read(&mut array[..1])?;
        let width = utf8_char_width(array[0]);
        if width == 0 {
            return Err(DecodeError::InvalidCharEncoding(array));
        }
        decoder.claim_bytes_read(width)?;
        if width == 1 {
            return Ok(array[0] as char);
        }
        decoder.reader().read(&mut array[1..width])?;
        let res = core::str::from_utf8(&array[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or(DecodeError::InvalidCharEncoding(array))?;
        Ok(res)
    }
}
impl_borrow_decode!(char);
impl<'a, 'de: 'a> BorrowDecode<'de> for &'a [u8] {
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = super::decode_slice_len(decoder)?;
        decoder.claim_bytes_read(len)?;
        decoder.borrow_reader().take_bytes(len)
    }
}
impl<'a, 'de: 'a> BorrowDecode<'de> for &'a str {
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let slice = <&[u8]>::borrow_decode(decoder)?;
        core::str::from_utf8(slice).map_err(|inner| DecodeError::Utf8 { inner })
    }
}
impl<T, const N: usize> Decode for [T; N]
where
    T: Decode + Sized + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            let ptr = &mut buf as *mut _ as *mut [T; N];
            let res = unsafe { ptr.read() };
            Ok(res)
        } else {
            let result = super::impl_core::collect_into_array(
                &mut (0..N)
                    .map(|_| {
                        decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                        T::decode(decoder)
                    }),
            );
            result.unwrap()
        }
    }
}
impl<'de, T, const N: usize> BorrowDecode<'de> for [T; N]
where
    T: BorrowDecode<'de> + Sized + 'static,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            let ptr = &mut buf as *mut _ as *mut [T; N];
            let res = unsafe { ptr.read() };
            Ok(res)
        } else {
            let result = super::impl_core::collect_into_array(
                &mut (0..N)
                    .map(|_| {
                        decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                        T::borrow_decode(decoder)
                    }),
            );
            result.unwrap()
        }
    }
}
impl Decode for () {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(())
    }
}
impl_borrow_decode!(());
impl<T> Decode for core::marker::PhantomData<T> {
    fn decode<D: Decoder>(_: &mut D) -> Result<Self, DecodeError> {
        Ok(core::marker::PhantomData)
    }
}
impl_borrow_decode!(core::marker::PhantomData < T >, T);
impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match super::decode_option_variant(
            decoder,
            core::any::type_name::<Option<T>>(),
        )? {
            Some(_) => {
                let val = T::decode(decoder)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}
impl<'de, T> BorrowDecode<'de> for Option<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        match super::decode_option_variant(
            decoder,
            core::any::type_name::<Option<T>>(),
        )? {
            Some(_) => {
                let val = T::borrow_decode(decoder)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }
}
impl<T, U> Decode for Result<T, U>
where
    T: Decode,
    U: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let is_ok = u32::decode(decoder)?;
        match is_ok {
            0 => {
                let t = T::decode(decoder)?;
                Ok(Ok(t))
            }
            1 => {
                let u = U::decode(decoder)?;
                Ok(Err(u))
            }
            x => {
                Err(DecodeError::UnexpectedVariant {
                    found: x,
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        max: 1,
                        min: 0,
                    },
                    type_name: core::any::type_name::<Result<T, U>>(),
                })
            }
        }
    }
}
impl<'de, T, U> BorrowDecode<'de> for Result<T, U>
where
    T: BorrowDecode<'de>,
    U: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let is_ok = u32::decode(decoder)?;
        match is_ok {
            0 => {
                let t = T::borrow_decode(decoder)?;
                Ok(Ok(t))
            }
            1 => {
                let u = U::borrow_decode(decoder)?;
                Ok(Err(u))
            }
            x => {
                Err(DecodeError::UnexpectedVariant {
                    found: x,
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        max: 1,
                        min: 0,
                    },
                    type_name: core::any::type_name::<Result<T, U>>(),
                })
            }
        }
    }
}
impl<T> Decode for Cell<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Cell::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for Cell<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Cell::new(t))
    }
}
impl<T> Decode for RefCell<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(RefCell::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for RefCell<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(RefCell::new(t))
    }
}
impl Decode for Duration {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        const NANOS_PER_SEC: u64 = 1_000_000_000;
        let secs: u64 = Decode::decode(decoder)?;
        let nanos: u32 = Decode::decode(decoder)?;
        if secs.checked_add(u64::from(nanos) / NANOS_PER_SEC).is_none() {
            return Err(DecodeError::InvalidDuration {
                secs,
                nanos,
            });
        }
        Ok(Duration::new(secs, nanos))
    }
}
impl_borrow_decode!(Duration);
impl<T> Decode for Range<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::decode(decoder)?;
        let max = T::decode(decoder)?;
        Ok(min..max)
    }
}
impl<'de, T> BorrowDecode<'de> for Range<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let min = T::borrow_decode(decoder)?;
        let max = T::borrow_decode(decoder)?;
        Ok(min..max)
    }
}
impl<T> Decode for RangeInclusive<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let min = T::decode(decoder)?;
        let max = T::decode(decoder)?;
        Ok(RangeInclusive::new(min, max))
    }
}
impl<'de, T> BorrowDecode<'de> for RangeInclusive<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let min = T::borrow_decode(decoder)?;
        let max = T::borrow_decode(decoder)?;
        Ok(RangeInclusive::new(min, max))
    }
}
impl<T> Decode for Bound<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::decode(decoder)?)),
            x => {
                Err(DecodeError::UnexpectedVariant {
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        max: 2,
                        min: 0,
                    },
                    found: x,
                    type_name: core::any::type_name::<Bound<T>>(),
                })
            }
        }
    }
}
impl<'de, T> BorrowDecode<'de> for Bound<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        match u32::decode(decoder)? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::borrow_decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::borrow_decode(decoder)?)),
            x => {
                Err(DecodeError::UnexpectedVariant {
                    allowed: &crate::error::AllowedEnumVariants::Range {
                        max: 2,
                        min: 0,
                    },
                    found: x,
                    type_name: core::any::type_name::<Bound<T>>(),
                })
            }
        }
    }
}
const UTF8_CHAR_WIDTH: [u8; 256] = [
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    4,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
const fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
#[cfg(test)]
mod tests_llm_16_182_llm_16_182 {
    use crate::config::{
        BigEndian, Config, Configuration, Fixint, LittleEndian, Varint, NoLimit,
    };
    use crate::de::{Decode, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    use std::ops::Range;
    #[test]
    fn test_decode_range() -> Result<(), DecodeError> {
        let config = Configuration::<BigEndian, Varint, NoLimit>::default()
            .with_big_endian()
            .with_fixed_int_encoding();
        let encoded_range: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 10];
        let slice_reader = SliceReader::new(&encoded_range);
        let mut decoder = DecoderImpl::new(slice_reader, config);
        let decoded: Range<u32> = Range::decode(&mut decoder)?;
        assert_eq!(decoded, 0..10);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_184_llm_16_184 {
    use crate::de::{Decode, Decoder, DecoderImpl, Sealed};
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    use crate::config::Configuration;
    struct TestDecoderImpl<'a> {
        inner: DecoderImpl<SliceReader<'a>, Configuration>,
    }
    impl<'a> TestDecoderImpl<'a> {
        fn new(data: &'a [u8]) -> Self {
            let config = Configuration::default();
            TestDecoderImpl {
                inner: DecoderImpl::new(SliceReader::new(data), config),
            }
        }
    }
    impl<'a> Sealed for TestDecoderImpl<'a> {}
    impl<'a> Decoder for TestDecoderImpl<'a> {
        type R = <DecoderImpl<SliceReader<'a>, Configuration> as Decoder>::R;
        type C = <DecoderImpl<SliceReader<'a>, Configuration> as Decoder>::C;
        fn reader(&mut self) -> &mut Self::R {
            self.inner.reader()
        }
        fn config(&self) -> &Self::C {
            self.inner.config()
        }
        fn claim_bytes_read(
            &mut self,
            n: usize,
        ) -> core::result::Result<(), DecodeError> {
            self.inner.claim_bytes_read(n)
        }
        fn unclaim_bytes_read(&mut self, n: usize) {
            self.inner.unclaim_bytes_read(n)
        }
    }
    #[test]
    fn test_decode_some() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1];
        let mut decoder = TestDecoderImpl::new(&data);
        let result: core::result::Result<Option<u8>, DecodeError> = Option::decode(
            &mut decoder,
        );
        debug_assert_eq!(result.unwrap(), Some(5));
             }
});    }
    #[test]
    fn test_decode_none() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0];
        let mut decoder = TestDecoderImpl::new(&data);
        let result: core::result::Result<Option<u8>, DecodeError> = Option::decode(
            &mut decoder,
        );
        debug_assert_eq!(result.unwrap(), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_185_llm_16_185 {
    use crate::de::{Decode, Decoder};
    use crate::error::DecodeError;
    use crate::config;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use std::result::Result;
    #[test]
    fn test_decode_result_ok() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = vec![rug_fuzz_0, 0, 0, 0, 0];
        let config = config::standard().with_no_limit();
        let mut reader = SliceReader::new(&input);
        let mut decoder = DecoderImpl::new(reader, config);
        let result: Result<Result<u32, u32>, DecodeError> = Result::decode(&mut decoder);
        debug_assert!(matches!(result, Ok(Ok(0))));
             }
});    }
    #[test]
    fn test_decode_result_err() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = vec![rug_fuzz_0, 0, 0, 0, 0];
        let config = config::standard().with_no_limit();
        let mut reader = SliceReader::new(&input);
        let mut decoder = DecoderImpl::new(reader, config);
        let result: Result<Result<u32, u32>, DecodeError> = Result::decode(&mut decoder);
        debug_assert!(matches!(result, Ok(Err(0))));
             }
});    }
    #[test]
    fn test_decode_result_unexpected_variant() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = vec![rug_fuzz_0, 0, 0, 0, 0];
        let config = config::standard().with_no_limit();
        let mut reader = SliceReader::new(&input);
        let mut decoder = DecoderImpl::new(reader, config);
        let result: Result<Result<u32, u32>, DecodeError> = Result::decode(&mut decoder);
        debug_assert!(matches!(result, Err(DecodeError::UnexpectedVariant { .. })));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_186_llm_16_186 {
    use super::*;
    use crate::*;
    use crate::de::{Decode, DecoderImpl};
    use crate::error::DecodeError;
    use std::time::Duration;
    use crate::config::Configuration;
    use crate::de::read::SliceReader;
    use crate::utils::Sealed;
    impl<R: crate::de::read::Reader, C: crate::config::Config> Sealed
    for TestDecoder<R, C> {}
    #[derive(Default)]
    struct TestDecoder<R: crate::de::read::Reader, C: crate::config::Config> {
        reader: R,
        config: C,
        bytes_read: usize,
    }
    impl<R: crate::de::read::Reader, C: crate::config::Config> crate::de::Decoder
    for TestDecoder<R, C> {
        type R = R;
        type C = C;
        fn reader(&mut self) -> &mut Self::R {
            &mut self.reader
        }
        fn config(&self) -> &Self::C {
            &self.config
        }
        fn claim_bytes_read(
            &mut self,
            n: usize,
        ) -> Result<(), crate::error::DecodeError> {
            self.bytes_read = self.bytes_read.saturating_add(n);
            Ok(())
        }
        fn claim_container_read<T>(
            &mut self,
            len: usize,
        ) -> Result<(), crate::error::DecodeError> {
            let size = len.checked_mul(core::mem::size_of::<T>());
            if let Some(size) = size {
                self.claim_bytes_read(size)
            } else {
                Err(crate::error::DecodeError::LimitExceeded)
            }
        }
        fn unclaim_bytes_read(&mut self, n: usize) {
            self.bytes_read = self.bytes_read.saturating_sub(n);
        }
    }
    #[test]
    fn test_decode_duration() {
        let _rug_st_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration = 0;
        let _rug_ed_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration = 0;
    }
    #[test]
    fn test_decode_duration_overflow() {
        let _rug_st_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration_overflow = 0;
        let _rug_ed_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration_overflow = 0;
    }
    #[test]
    fn test_decode_duration_invalid_nanos() {
        let _rug_st_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration_invalid_nanos = 0;
        let _rug_ed_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_decode_duration_invalid_nanos = 0;
    }
}
#[cfg(test)]
mod tests_rug_107 {
    use super::*;
    #[test]
    fn test_utf8_char_width() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: u8 = rug_fuzz_0;
        let width = crate::de::impls::utf8_char_width(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_108 {
    use crate::de::{self, Decode, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    use crate::error::DecodeError;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        debug_assert!(matches!(< bool > ::decode(& mut decoder), Ok(true)));
        let data_false = [rug_fuzz_5];
        let reader_false = SliceReader::new(&data_false);
        let mut decoder_false = DecoderImpl::new(reader_false, config);
        debug_assert!(matches!(< bool > ::decode(& mut decoder_false), Ok(false)));
        let data_invalid = [rug_fuzz_6];
        let reader_invalid = SliceReader::new(&data_invalid);
        let mut decoder_invalid = DecoderImpl::new(reader_invalid, config);
        debug_assert!(
            matches!(< bool > ::decode(& mut decoder_invalid),
            Err(DecodeError::InvalidBooleanValue(2)))
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_109 {
    use super::*;
    use crate::de::Decode;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::io::Read;
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
        let mut p0 = DecoderImpl::new(&mut reader, config);
        debug_assert!(< u8 > ::decode(& mut p0).is_ok());
        let decoded_byte = <u8>::decode(&mut p0).unwrap();
        debug_assert_eq!(decoded_byte, 1u8);
             }
});    }
}
#[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, Decode, DecodeError};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode_non_zero_u8() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        debug_assert_eq!(
            < std::num::NonZeroU8 > ::decode(& mut decoder).unwrap(),
            std::num::NonZeroU8::new(1).unwrap()
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, Decode};
    use crate::config::{BigEndian, Fixint, Limit, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1];
        let mut sr = SliceReader::new(&data);
        let cfg = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(sr, cfg);
        <u16>::decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::Decode;
    use crate::de::Decoder;
    use crate::de::{DecodeError, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroU16;
    #[test]
    fn test_decode_non_zero_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u16) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        match NonZeroU16::decode(&mut decoder) {
            Ok(non_zero_u16) => debug_assert!(non_zero_u16.get() > rug_fuzz_2),
            Err(e) => panic!("Error occurred: {:?}", e),
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    use crate::de::{self, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{self, Configuration};
    #[test]
    fn test_decode() {

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
        debug_assert!(< u32 > ::decode(& mut decoder).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_114 {
    use super::*;
    use crate::Decode;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let cfg = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(&mut v11, cfg);
        let result = <std::num::NonZeroU32 as Decode>::decode(&mut p0);
        debug_assert!(result.is_ok());
        let data_zero = [rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9];
        let mut reader_zero = SliceReader::new(&data_zero);
        let mut decoder_zero = DecoderImpl::new(&mut reader_zero, cfg);
        let result_zero = <std::num::NonZeroU32 as Decode>::decode(&mut decoder_zero);
        debug_assert!(result_zero.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_rug_115 {
    use super::*;
    use crate::de::{self, Decoder, DecoderImpl, read::SliceReader};
    use crate::config::{self, Configuration};
    #[test]
    fn test_decode() {

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
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <u64>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), 1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use std::num::NonZeroU64;
    use crate::de::{self, read::SliceReader, DecoderImpl};
    use crate::config::{self, Configuration};
    use crate::Decode;
    use crate::error::DecodeError;
    #[test]
    fn test_rug() {

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
        let result = NonZeroU64::decode(&mut p0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap().get(), 16909060);
             }
});    }
}
#[cfg(test)]
mod tests_rug_117 {
    use crate::de::{DecoderImpl, Decode, DecodeError};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint};
    #[test]
    fn test_decode() -> Result<(), DecodeError> {
        let data = [0u8; 16];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        <u128>::decode(&mut p0)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests_rug_118 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
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
        let mut p0 = DecoderImpl::new(&mut v11, v9);
        debug_assert!(< std::num::NonZeroU128 > ::decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_119 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    use crate::de::{Decode, DecodeError};
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
        let result = <usize>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_120 {
    use crate::de::{DecoderImpl, read::SliceReader, Decode};
    use crate::config::{BigEndian, Fixint, Configuration, Limit};
    use crate::de::DecodeError;
    use std::num::NonZeroUsize;
    #[test]
    fn test_decode_non_zero_usize() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = <NonZeroUsize as Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        let zero_data = [rug_fuzz_5; 8];
        let mut zero_reader = SliceReader::new(&zero_data);
        let mut zero_decoder = DecoderImpl::new(&mut zero_reader, config);
        let zero_result = <NonZeroUsize as Decode>::decode(&mut zero_decoder);
        debug_assert!(
            matches!(zero_result.unwrap_err(), DecodeError::NonZeroTypeIsZero {
            non_zero_type })
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_121 {
    use crate::de::{self, Decoder, DecoderImpl, read::SliceReader};
    use crate::config::{self, Configuration};
    use super::*;
    use std::io::Read;
    #[test]
    fn test_decode() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let slice_reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(slice_reader, config);
        debug_assert_eq!(< i8 > ::decode(& mut decoder).unwrap(), 1i8);
             }
});    }
}
#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::de::{DecoderImpl, Decode, DecodeError};
    use crate::de::read::SliceReader;
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
        debug_assert!(
            matches!(< std::num::NonZeroI8 as Decode > ::decode(& mut decoder),
            Err(DecodeError::NonZeroTypeIsZero { non_zero_type : IntegerType::I8 }))
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_123 {
    use super::*;
    use crate::de::{self, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
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
        let result = <i16>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_124 {
    use super::*;
    use crate::Decode;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroI16;
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
        let result = NonZeroI16::decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::de::{self, Decoder};
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
        let result = <i32>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{BigEndian, Fixint, Limit, Configuration};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data: [u8; 5] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        debug_assert!(matches!(< std::num::NonZeroI32 > ::decode(& mut p0), Ok(_)));
             }
});    }
}
#[cfg(test)]
mod tests_rug_127 {
    use crate::de::{self, read::SliceReader, DecoderImpl, Decode};
    use crate::config::{self, Configuration};
    #[test]
    fn test_decode() {

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
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let _result = <i64>::decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_128 {
    use crate::de::{self, read::SliceReader, Decoder, DecoderImpl};
    use crate::config::{self, Configuration};
    use std::num::NonZeroI64;
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
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let result = <NonZeroI64 as de::Decode>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use crate::de::{self, Decode, Decoder};
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
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <i128>::decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_130 {
    use crate::de::{Decode, Decoder, decoder::DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, Fixint, Configuration, Limit};
    use std::num::NonZeroI128;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u8, u8, i128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        match NonZeroI128::decode(&mut p0) {
            Ok(non_zero) => debug_assert!(non_zero.get() != rug_fuzz_5),
            Err(e) => panic!("Decoding failed with error {:?}", e),
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration, BigEndian, Fixint, Limit};
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
        let _result = <isize>::decode(&mut p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_132 {
    use crate::de::{self, Decode, Decoder};
    use crate::de::impls::DecodeError;
    use crate::{config, de::read};
    use std::num::NonZeroIsize;
    #[test]
    fn test_decode_non_zero_isize() {

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
        let mut reader = read::SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = de::DecoderImpl::new(&mut reader, config);
        match NonZeroIsize::decode(&mut decoder) {
            Ok(non_zero) => debug_assert_eq!(non_zero.get(), isize::from_be_bytes(data)),
            Err(e) => panic!("Error decoding NonZeroIsize: {:?}", e),
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_133 {
    use crate::de::{self, Decode, Decoder};
    use crate::config::{BigEndian, Configuration, Endian};
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    #[test]
    fn test_decode_f32() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let config = Configuration::<BigEndian>::default();
        let mut reader = SliceReader::new(&data);
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = <f32 as Decode>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), 50.0f32);
             }
});    }
}
#[cfg(test)]
mod tests_rug_134 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, f64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let result = <f64 as Decode>::decode(&mut p0).unwrap();
        debug_assert!((result - rug_fuzz_8).abs() < f64::EPSILON);
             }
});    }
}
#[cfg(test)]
mod tests_rug_135 {
    use super::*;
    use crate::de::{self, read::SliceReader, Decode, DecoderImpl};
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
        let v9 = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let result = <char>::decode(&mut p0);
        debug_assert!(result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_rug_136 {
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecode, BorrowDecoder};
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
        let mut v12 = DecoderImpl::new(v11, v9);
        let mut p0 = &mut v12;
        <&[u8]>::borrow_decode(p0).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_138 {
    use super::*;
    use crate::de::{DecoderImpl, read::SliceReader, Decode};
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
        let result = <[u8; 5]>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), [1u8, 2u8, 3u8, 4u8, 5u8]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_140 {
    use super::*;
    use crate::de::{DecoderImpl, Decode};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(p0, config);
        debug_assert!(< () > ::decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::marker::PhantomData;
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
        debug_assert!(PhantomData:: < i32 > ::decode(& mut p0).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_143 {
    use super::*;
    use crate::config::{BigEndian, Fixint, Configuration, Limit};
    use crate::de::{self, DecoderImpl, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
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
        let _: Result<Result<u32, u32>, DecodeError> = <Result<
            u32,
            u32,
        > as BorrowDecode>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_144 {
    use super::*;
    use crate::de::{read::SliceReader, DecoderImpl};
    use crate::config::Configuration;
    use crate::Decode;
    use std::cell::Cell;
    #[test]
    fn test_decode() {

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
        let mut decoder = DecoderImpl::new(reader, config);
        Cell::<u32>::decode(&mut decoder).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder, DecodeError};
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use std::cell::Cell;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result: Result<Cell<i32>, DecodeError> = <Cell<
            i32,
        > as BorrowDecode>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{Decoder, DecoderImpl};
    use crate::config::Configuration;
    use std::cell::RefCell;
    #[test]
    fn test_refcell_decode() {

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
        let mut decoder_impl = DecoderImpl::new(reader, config);
        debug_assert!(RefCell:: < i32 > ::decode(& mut decoder_impl).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_147 {
    use crate::de::read::SliceReader;
    use crate::de::{BorrowDecoder, BorrowDecode, DecoderImpl};
    use crate::config::Configuration;
    use crate::error::DecodeError;
    use std::cell::RefCell;
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
        let result: Result<RefCell<u8>, DecodeError> = <std::cell::RefCell<
            u8,
        >>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::decoder::DecoderImpl;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let mut config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = <std::ops::Range<u8>>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_149 {
    use super::*;
    use crate::de::{self, Decoder, DecodeError};
    use crate::de::read::SliceReader;
    use crate::de::decoder::DecoderImpl;
    use crate::config::{self, Configuration, BigEndian, Fixint, Limit};
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
        let result: Result<std::ops::RangeInclusive<i32>, DecodeError> = <std::ops::RangeInclusive<
            i32,
        >>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_150 {
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use crate::de::{
        self, BorrowDecode, BorrowDecoder, DecodeError, DecoderImpl, read::SliceReader,
    };
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
        let _result: Result<std::ops::RangeInclusive<u8>, DecodeError> = <std::ops::RangeInclusive<
            u8,
        >>::borrow_decode(&mut p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_151 {
    use crate::de::{self, Decode, DecoderImpl, DecodeError};
    use crate::de::read::SliceReader;
    use std::collections::Bound;
    use crate::config::{Configuration, BigEndian, Fixint};
    #[test]
    fn test_decode_bound() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let v11 = SliceReader::new(&data);
        let v9 = Configuration::<
            BigEndian,
            Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, v9);
        let result = <Bound<u32>>::decode(&mut p0);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{BorrowDecoder, DecoderImpl};
    use crate::de::impls::BorrowDecode;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use std::collections::Bound;
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
        <Bound<i32>>::borrow_decode(&mut decoder).unwrap();
             }
});    }
}
