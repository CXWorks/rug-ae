//! Parsers recognizing numbers, complete input version

use crate::branch::alt;
use crate::bytes::complete::tag;
use crate::character::complete::{char, digit1, sign};
use crate::combinator::{cut, map, opt, recognize};
use crate::error::ParseError;
use crate::error::{make_error, ErrorKind};
use crate::internal::*;
use crate::lib::std::ops::{Add, Shl};
use crate::sequence::pair;
use crate::traits::{AsBytes, AsChar, Compare, Input, Offset};

/// Recognizes an unsigned 1 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u8;
///
/// let parser = |s| {
///   be_u8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 1)
}

/// Recognizes a big endian unsigned 2 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u16;
///
/// let parser = |s| {
///   be_u16(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u16<I, E: ParseError<I>>(input: I) -> IResult<I, u16, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 2)
}

/// Recognizes a big endian unsigned 3 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u24;
///
/// let parser = |s| {
///   be_u24(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u24<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 3)
}

/// Recognizes a big endian unsigned 4 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u32;
///
/// let parser = |s| {
///   be_u32(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 4)
}

/// Recognizes a big endian unsigned 8 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u64;
///
/// let parser = |s| {
///   be_u64(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u64<I, E: ParseError<I>>(input: I) -> IResult<I, u64, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 8)
}

/// Recognizes a big endian unsigned 16 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_u128;
///
/// let parser = |s| {
///   be_u128(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_u128<I, E: ParseError<I>>(input: I) -> IResult<I, u128, E>
where
  I: Input<Item = u8>,
{
  be_uint(input, 16)
}

#[inline]
fn be_uint<I, Uint, E: ParseError<I>>(input: I, bound: usize) -> IResult<I, Uint, E>
where
  I: Input<Item = u8>,
  Uint: Default + Shl<u8, Output = Uint> + Add<Uint, Output = Uint> + From<u8>,
{
  if input.input_len() < bound {
    Err(Err::Error(make_error(input, ErrorKind::Eof)))
  } else {
    let mut res = Uint::default();

    // special case to avoid shift a byte with overflow
    if bound > 1 {
      for byte in input.iter_elements().take(bound) {
        res = (res << 8) + byte.into();
      }
    } else {
      for byte in input.iter_elements().take(bound) {
        res = byte.into();
      }
    }

    Ok((input.take_from(bound), res))
  }
}

/// Recognizes a signed 1 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i8;
///
/// let parser = |s| {
///   be_i8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i8<I, E: ParseError<I>>(input: I) -> IResult<I, i8, E>
where
  I: Input<Item = u8>,
{
  be_u8.map(|x| x as i8).parse(input)
}

/// Recognizes a big endian signed 2 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i16;
///
/// let parser = |s| {
///   be_i16(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i16<I, E: ParseError<I>>(input: I) -> IResult<I, i16, E>
where
  I: Input<Item = u8>,
{
  be_u16.map(|x| x as i16).parse(input)
}

/// Recognizes a big endian signed 3 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i24;
///
/// let parser = |s| {
///   be_i24(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i24<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  // Same as the unsigned version but we need to sign-extend manually here
  be_u24
    .map(|x| {
      if x & 0x80_00_00 != 0 {
        (x | 0xff_00_00_00) as i32
      } else {
        x as i32
      }
    })
    .parse(input)
}

/// Recognizes a big endian signed 4 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i32;
///
/// let parser = |s| {
///   be_i32(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i32<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  be_u32.map(|x| x as i32).parse(input)
}

/// Recognizes a big endian signed 8 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i64;
///
/// let parser = |s| {
///   be_i64(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i64<I, E: ParseError<I>>(input: I) -> IResult<I, i64, E>
where
  I: Input<Item = u8>,
{
  be_u64.map(|x| x as i64).parse(input)
}

/// Recognizes a big endian signed 16 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_i128;
///
/// let parser = |s| {
///   be_i128(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_i128<I, E: ParseError<I>>(input: I) -> IResult<I, i128, E>
where
  I: Input<Item = u8>,
{
  be_u128.map(|x| x as i128).parse(input)
}

/// Recognizes an unsigned 1 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u8;
///
/// let parser = |s| {
///   le_u8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 1)
}

/// Recognizes a little endian unsigned 2 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u16;
///
/// let parser = |s| {
///   le_u16(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u16<I, E: ParseError<I>>(input: I) -> IResult<I, u16, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 2)
}

/// Recognizes a little endian unsigned 3 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u24;
///
/// let parser = |s| {
///   le_u24(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u24<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 3)
}

/// Recognizes a little endian unsigned 4 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u32;
///
/// let parser = |s| {
///   le_u32(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 4)
}

/// Recognizes a little endian unsigned 8 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u64;
///
/// let parser = |s| {
///   le_u64(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u64<I, E: ParseError<I>>(input: I) -> IResult<I, u64, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 8)
}

/// Recognizes a little endian unsigned 16 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_u128;
///
/// let parser = |s| {
///   le_u128(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_u128<I, E: ParseError<I>>(input: I) -> IResult<I, u128, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 16)
}

#[inline]
fn le_uint<I, Uint, E: ParseError<I>>(input: I, bound: usize) -> IResult<I, Uint, E>
where
  I: Input<Item = u8>,
  Uint: Default + Shl<u8, Output = Uint> + Add<Uint, Output = Uint> + From<u8>,
{
  if input.input_len() < bound {
    Err(Err::Error(make_error(input, ErrorKind::Eof)))
  } else {
    let mut res = Uint::default();
    for (index, byte) in input.iter_elements().take(bound).enumerate() {
      res = res + (Uint::from(byte) << (8 * index as u8));
    }

    Ok((input.take_from(bound), res))
  }
}

/// Recognizes a signed 1 byte integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i8;
///
/// let parser = |s| {
///   le_i8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i8<I, E: ParseError<I>>(input: I) -> IResult<I, i8, E>
where
  I: Input<Item = u8>,
{
  be_u8.map(|x| x as i8).parse(input)
}

/// Recognizes a little endian signed 2 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i16;
///
/// let parser = |s| {
///   le_i16(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i16<I, E: ParseError<I>>(input: I) -> IResult<I, i16, E>
where
  I: Input<Item = u8>,
{
  le_u16.map(|x| x as i16).parse(input)
}

/// Recognizes a little endian signed 3 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i24;
///
/// let parser = |s| {
///   le_i24(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i24<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  // Same as the unsigned version but we need to sign-extend manually here
  le_u24
    .map(|x| {
      if x & 0x80_00_00 != 0 {
        (x | 0xff_00_00_00) as i32
      } else {
        x as i32
      }
    })
    .parse(input)
}

/// Recognizes a little endian signed 4 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i32;
///
/// let parser = |s| {
///   le_i32(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i32<I, E: ParseError<I>>(input: I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  le_u32.map(|x| x as i32).parse(input)
}

/// Recognizes a little endian signed 8 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i64;
///
/// let parser = |s| {
///   le_i64(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i64<I, E: ParseError<I>>(input: I) -> IResult<I, i64, E>
where
  I: Input<Item = u8>,
{
  le_u64.map(|x| x as i64).parse(input)
}

/// Recognizes a little endian signed 16 bytes integer.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_i128;
///
/// let parser = |s| {
///   le_i128(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_i128<I, E: ParseError<I>>(input: I) -> IResult<I, i128, E>
where
  I: Input<Item = u8>,
{
  le_u128.map(|x| x as i128).parse(input)
}

/// Recognizes an unsigned 1 byte integer
///
/// Note that endianness does not apply to 1 byte numbers.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u8;
///
/// let parser = |s| {
///   u8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Input<Item = u8>,
{
  let bound: usize = 1;
  if input.input_len() < bound {
    Err(Err::Error(make_error(input, ErrorKind::Eof)))
  } else {
    let res = input.iter_elements().next().unwrap();

    Ok((input.take_from(bound), res))
  }
}

/// Recognizes an unsigned 2 bytes integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u16 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u16 integer.
/// *complete version*: returns an error if there is not enough input data
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u16;
///
/// let be_u16 = |s| {
///   u16(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(be_u16(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_u16 = |s| {
///   u16(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(le_u16(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u16<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, u16, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_u16,
    crate::number::Endianness::Little => le_u16,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_u16,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_u16,
  }
}

/// Recognizes an unsigned 3 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u24 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u24 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u24;
///
/// let be_u24 = |s| {
///   u24(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(be_u24(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_u24 = |s| {
///   u24(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(le_u24(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u24<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_u24,
    crate::number::Endianness::Little => le_u24,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_u24,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_u24,
  }
}

/// Recognizes an unsigned 4 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u32 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u32 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u32;
///
/// let be_u32 = |s| {
///   u32(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(be_u32(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_u32 = |s| {
///   u32(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(le_u32(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u32<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, u32, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_u32,
    crate::number::Endianness::Little => le_u32,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_u32,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_u32,
  }
}

/// Recognizes an unsigned 8 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u64 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u64 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u64;
///
/// let be_u64 = |s| {
///   u64(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(be_u64(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_u64 = |s| {
///   u64(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(le_u64(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u64<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, u64, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_u64,
    crate::number::Endianness::Little => le_u64,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_u64,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_u64,
  }
}

/// Recognizes an unsigned 16 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u128 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u128 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::u128;
///
/// let be_u128 = |s| {
///   u128(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(be_u128(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_u128 = |s| {
///   u128(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(le_u128(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn u128<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, u128, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_u128,
    crate::number::Endianness::Little => le_u128,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_u128,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_u128,
  }
}

/// Recognizes a signed 1 byte integer
///
/// Note that endianness does not apply to 1 byte numbers.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i8;
///
/// let parser = |s| {
///   i8(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Error((&[][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i8<I, E: ParseError<I>>(i: I) -> IResult<I, i8, E>
where
  I: Input<Item = u8>,
{
  u8.map(|x| x as i8).parse(i)
}

/// Recognizes a signed 2 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian i16 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian i16 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i16;
///
/// let be_i16 = |s| {
///   i16(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(be_i16(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_i16 = |s| {
///   i16(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(le_i16(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i16<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, i16, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_i16,
    crate::number::Endianness::Little => le_i16,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_i16,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_i16,
  }
}

/// Recognizes a signed 3 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian i24 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian i24 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i24;
///
/// let be_i24 = |s| {
///   i24(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(be_i24(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_i24 = |s| {
///   i24(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(le_i24(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i24<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_i24,
    crate::number::Endianness::Little => le_i24,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_i24,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_i24,
  }
}

/// Recognizes a signed 4 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian i32 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian i32 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i32;
///
/// let be_i32 = |s| {
///   i32(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(be_i32(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_i32 = |s| {
///   i32(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(le_i32(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i32<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, i32, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_i32,
    crate::number::Endianness::Little => le_i32,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_i32,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_i32,
  }
}

/// Recognizes a signed 8 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian i64 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian i64 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i64;
///
/// let be_i64 = |s| {
///   i64(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(be_i64(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_i64 = |s| {
///   i64(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(le_i64(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i64<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, i64, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_i64,
    crate::number::Endianness::Little => le_i64,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_i64,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_i64,
  }
}

/// Recognizes a signed 16 byte integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian i128 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian i128 integer.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::i128;
///
/// let be_i128 = |s| {
///   i128(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(be_i128(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
///
/// let le_i128 = |s| {
///   i128(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(le_i128(&b"\x01"[..]), Err(Err::Error((&[0x01][..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn i128<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, i128, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_i128,
    crate::number::Endianness::Little => le_i128,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_i128,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_i128,
  }
}

/// Recognizes a big endian 4 bytes floating point number.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_f32;
///
/// let parser = |s| {
///   be_f32(s)
/// };
///
/// assert_eq!(parser(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_f32<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
  I: Input<Item = u8>,
{
  match be_u32(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f32::from_bits(o))),
  }
}

/// Recognizes a big endian 8 bytes floating point number.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::be_f64;
///
/// let parser = |s| {
///   be_f64(s)
/// };
///
/// assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn be_f64<I, E: ParseError<I>>(input: I) -> IResult<I, f64, E>
where
  I: Input<Item = u8>,
{
  match be_u64(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f64::from_bits(o))),
  }
}

/// Recognizes a little endian 4 bytes floating point number.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_f32;
///
/// let parser = |s| {
///   le_f32(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_f32<I, E: ParseError<I>>(input: I) -> IResult<I, f32, E>
where
  I: Input<Item = u8>,
{
  match le_u32(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f32::from_bits(o))),
  }
}

/// Recognizes a little endian 8 bytes floating point number.
///
/// *Complete version*: Returns an error if there is not enough input data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::le_f64;
///
/// let parser = |s| {
///   le_f64(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn le_f64<I, E: ParseError<I>>(input: I) -> IResult<I, f64, E>
where
  I: Input<Item = u8>,
{
  match le_u64(input) {
    Err(e) => Err(e),
    Ok((i, o)) => Ok((i, f64::from_bits(o))),
  }
}

/// Recognizes a 4 byte floating point number
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian f32 float,
/// otherwise if `nom::number::Endianness::Little` parse a little endian f32 float.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::f32;
///
/// let be_f32 = |s| {
///   f32(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_f32(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(be_f32(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
///
/// let le_f32 = |s| {
///   f32(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_f32(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(le_f32(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn f32<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, f32, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_f32,
    crate::number::Endianness::Little => le_f32,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_f32,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_f32,
  }
}

/// Recognizes an 8 byte floating point number
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian f64 float,
/// otherwise if `nom::number::Endianness::Little` parse a little endian f64 float.
/// *complete version*: returns an error if there is not enough input data
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::f64;
///
/// let be_f64 = |s| {
///   f64(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_f64(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(be_f64(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
///
/// let le_f64 = |s| {
///   f64(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(le_f64(&b"abc"[..]), Err(Err::Error((&b"abc"[..], ErrorKind::Eof))));
/// ```
#[inline]
pub fn f64<I, E: ParseError<I>>(endian: crate::number::Endianness) -> fn(I) -> IResult<I, f64, E>
where
  I: Input<Item = u8>,
{
  match endian {
    crate::number::Endianness::Big => be_f64,
    crate::number::Endianness::Little => le_f64,
    #[cfg(target_endian = "big")]
    crate::number::Endianness::Native => be_f64,
    #[cfg(target_endian = "little")]
    crate::number::Endianness::Native => le_f64,
  }
}

/// Recognizes a hex-encoded integer.
///
/// *Complete version*: Will parse until the end of input if it has less than 8 bytes.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::hex_u32;
///
/// let parser = |s| {
///   hex_u32(s)
/// };
///
/// assert_eq!(parser(&b"01AE"[..]), Ok((&b""[..], 0x01AE)));
/// assert_eq!(parser(&b"abc"[..]), Ok((&b""[..], 0x0ABC)));
/// assert_eq!(parser(&b"ggg"[..]), Err(Err::Error((&b"ggg"[..], ErrorKind::IsA))));
/// ```
#[inline]
pub fn hex_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input,
  <I as Input>::Item: AsChar,
  I: AsBytes,
{
  let e: ErrorKind = ErrorKind::IsA;
  let (i, o) = input.split_at_position1_complete(
    |c| {
      let c = c.as_char();
      !"0123456789abcdefABCDEF".contains(c)
    },
    e,
  )?;

  // Do not parse more than 8 characters for a u32
  let (remaining, parsed) = if o.input_len() <= 8 {
    (i, o)
  } else {
    input.take_split(8)
  };

  let res = parsed
    .as_bytes()
    .iter()
    .rev()
    .enumerate()
    .map(|(k, &v)| {
      let digit = v as char;
      digit.to_digit(16).unwrap_or(0) << (k * 4)
    })
    .sum();

  Ok((remaining, res))
}

/// Recognizes floating point number in a byte string and returns the corresponding slice.
///
/// *Complete version*: Can parse until the end of input.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::recognize_float;
///
/// let parser = |s| {
///   recognize_float(s)
/// };
///
/// assert_eq!(parser("11e-1"), Ok(("", "11e-1")));
/// assert_eq!(parser("123E-02"), Ok(("", "123E-02")));
/// assert_eq!(parser("123K-01"), Ok(("K-01", "123")));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Char))));
/// ```
#[rustfmt::skip]
pub fn recognize_float<T, E:ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Clone + Offset,
  T: Input,
  <T as Input>::Item: AsChar,
{
  recognize((
    opt(alt((char('+'), char('-')))),
      alt((
        map((digit1, opt(pair(char('.'), opt(digit1)))), |_| ()),
        map((char('.'), digit1), |_| ())
      )),
      opt((
        alt((char('e'), char('E'))),
        opt(alt((char('+'), char('-')))),
        cut(digit1)
      ))
  ))(input)
}

// workaround until issues with minimal-lexical are fixed
#[doc(hidden)]
pub fn recognize_float_or_exceptions<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Clone + Offset,
  T: Input + Compare<&'static str>,
  <T as Input>::Item: AsChar,
{
  alt((
    |i: T| {
      recognize_float::<_, E>(i.clone()).map_err(|e| match e {
        crate::Err::Error(_) => crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)),
        crate::Err::Failure(_) => crate::Err::Failure(E::from_error_kind(i, ErrorKind::Float)),
        crate::Err::Incomplete(needed) => crate::Err::Incomplete(needed),
      })
    },
    |i: T| {
      crate::bytes::complete::tag_no_case::<_, _, E>("nan")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
    |i: T| {
      crate::bytes::complete::tag_no_case::<_, _, E>("inf")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
    |i: T| {
      crate::bytes::complete::tag_no_case::<_, _, E>("infinity")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
  ))(input)
}

/// Recognizes a floating point number in text format
///
/// It returns a tuple of (`sign`, `integer part`, `fraction part` and `exponent`) of the input
/// data.
///
/// *Complete version*: Can parse until the end of input.
///
pub fn recognize_float_parts<T, E: ParseError<T>>(input: T) -> IResult<T, (bool, T, T, i32), E>
where
  T: Clone + Offset,
  T: Input,
  <T as Input>::Item: AsChar,
  T: for<'a> Compare<&'a [u8]>,
  T: AsBytes,
{
  let (i, sign) = sign(input.clone())?;

  //let (i, zeroes) = take_while(|c: <T as Input>::Item| c.as_char() == '0')(i)?;
  let (i, zeroes) = match i.as_bytes().iter().position(|c| *c != b'0') {
    Some(index) => i.take_split(index),
    None => i.take_split(i.input_len()),
  };
  //let (i, mut integer) = digit0(i)?;
  let (i, mut integer) = match i
    .as_bytes()
    .iter()
    .position(|c| !(*c >= b'0' && *c <= b'9'))
  {
    Some(index) => i.take_split(index),
    None => i.take_split(i.input_len()),
  };

  if integer.input_len() == 0 && zeroes.input_len() > 0 {
    // keep the last zero if integer is empty
    integer = zeroes.take_from(zeroes.input_len() - 1);
  }

  let (i, opt_dot) = opt(tag(&b"."[..]))(i)?;
  let (i, fraction) = if opt_dot.is_none() {
    let i2 = i.clone();
    (i2, i.take(0))
  } else {
    // match number, trim right zeroes
    let mut zero_count = 0usize;
    let mut position = None;
    for (pos, c) in i.as_bytes().iter().enumerate() {
      if *c >= b'0' && *c <= b'9' {
        if *c == b'0' {
          zero_count += 1;
        } else {
          zero_count = 0;
        }
      } else {
        position = Some(pos);
        break;
      }
    }

    #[allow(clippy::or_fun_call)]
    let position = position.unwrap_or(i.input_len());

    let index = if zero_count == 0 {
      position
    } else if zero_count == position {
      position - zero_count + 1
    } else {
      position - zero_count
    };

    (i.take_from(position), i.take(index))
  };

  if integer.input_len() == 0 && fraction.input_len() == 0 {
    return Err(Err::Error(E::from_error_kind(input, ErrorKind::Float)));
  }

  let i2 = i.clone();
  let (i, e) = match i.as_bytes().iter().next() {
    Some(b'e') => (i.take_from(1), true),
    Some(b'E') => (i.take_from(1), true),
    _ => (i, false),
  };

  let (i, exp) = if e {
    cut(crate::character::complete::i32)(i)?
  } else {
    (i2, 0)
  };

  Ok((i, (sign, integer, fraction, exp)))
}

use crate::traits::ParseTo;

/// Recognizes floating point number in text format and returns a f32.
///
/// *Complete version*: Can parse until the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::float;
///
/// let parser = |s| {
///   float(s)
/// };
///
/// assert_eq!(parser("11e-1"), Ok(("", 1.1)));
/// assert_eq!(parser("123E-02"), Ok(("", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
/// ```
pub fn float<T, E: ParseError<T>>(input: T) -> IResult<T, f32, E>
where
  T: Clone + Offset + ParseTo<f32> + Compare<&'static str>,
  T: Input,
  <T as Input>::Item: AsChar,
  <T as Input>::Iter: Clone,
  T: AsBytes,
  T: for<'a> Compare<&'a [u8]>,
{
  /*
  let (i, (sign, integer, fraction, exponent)) = recognize_float_parts(input)?;

  let mut float: f32 = minimal_lexical::parse_float(
    integer.as_bytes().iter(),
    fraction.as_bytes().iter(),
    exponent,
  );
  if !sign {
    float = -float;
  }

  Ok((i, float))
      */
  let (i, s) = recognize_float_or_exceptions(input)?;
  match s.parse_to() {
    Some(f) => Ok((i, f)),
    None => Err(crate::Err::Error(E::from_error_kind(
      i,
      crate::error::ErrorKind::Float,
    ))),
  }
}

/// Recognizes floating point number in text format and returns a f64.
///
/// *Complete version*: Can parse until the end of input.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::complete::double;
///
/// let parser = |s| {
///   double(s)
/// };
///
/// assert_eq!(parser("11e-1"), Ok(("", 1.1)));
/// assert_eq!(parser("123E-02"), Ok(("", 1.23)));
/// assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
/// ```
pub fn double<T, E: ParseError<T>>(input: T) -> IResult<T, f64, E>
where
  T: Clone + Offset + ParseTo<f64> + Compare<&'static str>,
  T: Input,
  <T as Input>::Item: AsChar,
  <T as Input>::Iter: Clone,
  T: AsBytes,
  T: for<'a> Compare<&'a [u8]>,
{
  /*
  let (i, (sign, integer, fraction, exponent)) = recognize_float_parts(input)?;

  let mut float: f64 = minimal_lexical::parse_float(
    integer.as_bytes().iter(),
    fraction.as_bytes().iter(),
    exponent,
  );
  if !sign {
    float = -float;
  }

  Ok((i, float))
      */
  let (i, s) = recognize_float_or_exceptions(input)?;
  match s.parse_to() {
    Some(f) => Ok((i, f)),
    None => Err(crate::Err::Error(E::from_error_kind(
      i,
      crate::error::ErrorKind::Float,
    ))),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::error::ErrorKind;
  use crate::internal::Err;
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn i8_tests() {
    assert_parse!(i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(i8(&[0x80][..]), Ok((&b""[..], -128)));
  }

  #[test]
  fn be_i8_tests() {
    assert_parse!(be_i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(be_i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i8(&[0x80][..]), Ok((&b""[..], -128)));
  }

  #[test]
  fn be_i16_tests() {
    assert_parse!(be_i16(&[0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i16(&[0x7f, 0xff][..]), Ok((&b""[..], 32_767_i16)));
    assert_parse!(be_i16(&[0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i16(&[0x80, 0x00][..]), Ok((&b""[..], -32_768_i16)));
  }

  #[test]
  fn be_u24_tests() {
    assert_parse!(be_u24(&[0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_u24(&[0x00, 0xFF, 0xFF][..]), Ok((&b""[..], 65_535_u32)));
    assert_parse!(
      be_u24(&[0x12, 0x34, 0x56][..]),
      Ok((&b""[..], 1_193_046_u32))
    );
  }

  #[test]
  fn be_i24_tests() {
    assert_parse!(be_i24(&[0xFF, 0xFF, 0xFF][..]), Ok((&b""[..], -1_i32)));
    assert_parse!(be_i24(&[0xFF, 0x00, 0x00][..]), Ok((&b""[..], -65_536_i32)));
    assert_parse!(
      be_i24(&[0xED, 0xCB, 0xAA][..]),
      Ok((&b""[..], -1_193_046_i32))
    );
  }

  #[test]
  fn be_i32_tests() {
    assert_parse!(be_i32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(
      be_i32(&[0x7f, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], 2_147_483_647_i32))
    );
    assert_parse!(be_i32(&[0xff, 0xff, 0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(
      be_i32(&[0x80, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], -2_147_483_648_i32))
    );
  }

  #[test]
  fn be_i64_tests() {
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      be_i64(&[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], 9_223_372_036_854_775_807_i64))
    );
    assert_parse!(
      be_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      be_i64(&[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], -9_223_372_036_854_775_808_i64))
    );
  }

  #[test]
  fn be_i128_tests() {
    assert_parse!(
      be_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      be_i128(
        &[
          0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((
        &b""[..],
        170_141_183_460_469_231_731_687_303_715_884_105_727_i128
      ))
    );
    assert_parse!(
      be_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      be_i128(
        &[
          0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((
        &b""[..],
        -170_141_183_460_469_231_731_687_303_715_884_105_728_i128
      ))
    );
  }

  #[test]
  fn le_i8_tests() {
    assert_parse!(le_i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(le_i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(le_i8(&[0x80][..]), Ok((&b""[..], -128)));
  }

  #[test]
  fn le_i16_tests() {
    assert_parse!(le_i16(&[0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_i16(&[0xff, 0x7f][..]), Ok((&b""[..], 32_767_i16)));
    assert_parse!(le_i16(&[0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(le_i16(&[0x00, 0x80][..]), Ok((&b""[..], -32_768_i16)));
  }

  #[test]
  fn le_u24_tests() {
    assert_parse!(le_u24(&[0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(le_u24(&[0xFF, 0xFF, 0x00][..]), Ok((&b""[..], 65_535_u32)));
    assert_parse!(
      le_u24(&[0x56, 0x34, 0x12][..]),
      Ok((&b""[..], 1_193_046_u32))
    );
  }

  #[test]
  fn le_i24_tests() {
    assert_parse!(le_i24(&[0xFF, 0xFF, 0xFF][..]), Ok((&b""[..], -1_i32)));
    assert_parse!(le_i24(&[0x00, 0x00, 0xFF][..]), Ok((&b""[..], -65_536_i32)));
    assert_parse!(
      le_i24(&[0xAA, 0xCB, 0xED][..]),
      Ok((&b""[..], -1_193_046_i32))
    );
  }

  #[test]
  fn le_i32_tests() {
    assert_parse!(le_i32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(
      le_i32(&[0xff, 0xff, 0xff, 0x7f][..]),
      Ok((&b""[..], 2_147_483_647_i32))
    );
    assert_parse!(le_i32(&[0xff, 0xff, 0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(
      le_i32(&[0x00, 0x00, 0x00, 0x80][..]),
      Ok((&b""[..], -2_147_483_648_i32))
    );
  }

  #[test]
  fn le_i64_tests() {
    assert_parse!(
      le_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      le_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f][..]),
      Ok((&b""[..], 9_223_372_036_854_775_807_i64))
    );
    assert_parse!(
      le_i64(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff][..]),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      le_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80][..]),
      Ok((&b""[..], -9_223_372_036_854_775_808_i64))
    );
  }

  #[test]
  fn le_i128_tests() {
    assert_parse!(
      le_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00
        ][..]
      ),
      Ok((&b""[..], 0))
    );
    assert_parse!(
      le_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0x7f
        ][..]
      ),
      Ok((
        &b""[..],
        170_141_183_460_469_231_731_687_303_715_884_105_727_i128
      ))
    );
    assert_parse!(
      le_i128(
        &[
          0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
          0xff
        ][..]
      ),
      Ok((&b""[..], -1))
    );
    assert_parse!(
      le_i128(
        &[
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x80
        ][..]
      ),
      Ok((
        &b""[..],
        -170_141_183_460_469_231_731_687_303_715_884_105_728_i128
      ))
    );
  }

  #[test]
  fn be_f32_tests() {
    assert_parse!(be_f32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0_f32)));
    assert_parse!(
      be_f32(&[0x4d, 0x31, 0x1f, 0xd8][..]),
      Ok((&b""[..], 185_728_392_f32))
    );
  }

  #[test]
  fn be_f64_tests() {
    assert_parse!(
      be_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0_f64))
    );
    assert_parse!(
      be_f64(&[0x41, 0xa6, 0x23, 0xfb, 0x10, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 185_728_392_f64))
    );
  }

  #[test]
  fn le_f32_tests() {
    assert_parse!(le_f32(&[0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 0_f32)));
    assert_parse!(
      le_f32(&[0xd8, 0x1f, 0x31, 0x4d][..]),
      Ok((&b""[..], 185_728_392_f32))
    );
  }

  #[test]
  fn le_f64_tests() {
    assert_parse!(
      le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Ok((&b""[..], 0_f64))
    );
    assert_parse!(
      le_f64(&[0x00, 0x00, 0x00, 0x10, 0xfb, 0x23, 0xa6, 0x41][..]),
      Ok((&b""[..], 185_728_392_f64))
    );
  }

  #[test]
  fn hex_u32_tests() {
    assert_parse!(
      hex_u32(&b";"[..]),
      Err(Err::Error(error_position!(&b";"[..], ErrorKind::IsA)))
    );
    assert_parse!(hex_u32(&b"ff;"[..]), Ok((&b";"[..], 255)));
    assert_parse!(hex_u32(&b"1be2;"[..]), Ok((&b";"[..], 7138)));
    assert_parse!(hex_u32(&b"c5a31be2;"[..]), Ok((&b";"[..], 3_315_801_058)));
    assert_parse!(hex_u32(&b"C5A31be2;"[..]), Ok((&b";"[..], 3_315_801_058)));
    assert_parse!(hex_u32(&b"00c5a31be2;"[..]), Ok((&b"e2;"[..], 12_952_347)));
    assert_parse!(
      hex_u32(&b"c5a31be201;"[..]),
      Ok((&b"01;"[..], 3_315_801_058))
    );
    assert_parse!(hex_u32(&b"ffffffff;"[..]), Ok((&b";"[..], 4_294_967_295)));
    assert_parse!(hex_u32(&b"0x1be2;"[..]), Ok((&b"x1be2;"[..], 0)));
    assert_parse!(hex_u32(&b"12af"[..]), Ok((&b""[..], 0x12af)));
  }

  #[test]
  #[cfg(feature = "std")]
  fn float_test() {
    let mut test_cases = vec![
      "+3.14",
      "3.14",
      "-3.14",
      "0",
      "0.0",
      "1.",
      ".789",
      "-.5",
      "1e7",
      "-1E-7",
      ".3e-2",
      "1.e4",
      "1.2e4",
      "12.34",
      "-1.234E-12",
      "-1.234e-12",
      "0.00000000000000000087",
    ];

    for test in test_cases.drain(..) {
      let expected32 = str::parse::<f32>(test).unwrap();
      let expected64 = str::parse::<f64>(test).unwrap();

      println!("now parsing: {} -> {}", test, expected32);

      assert_parse!(recognize_float(test), Ok(("", test)));

      assert_parse!(float(test.as_bytes()), Ok((&b""[..], expected32)));
      assert_parse!(float(test), Ok(("", expected32)));

      assert_parse!(double(test.as_bytes()), Ok((&b""[..], expected64)));
      assert_parse!(double(test), Ok(("", expected64)));
    }

    let remaining_exponent = "-1.234E-";
    assert_parse!(
      recognize_float(remaining_exponent),
      Err(Err::Failure(("", ErrorKind::Digit)))
    );

    let (_i, nan) = float::<_, ()>("NaN").unwrap();
    assert!(nan.is_nan());

    let (_i, inf) = float::<_, ()>("inf").unwrap();
    assert!(inf.is_infinite());
    let (_i, inf) = float::<_, ()>("infinite").unwrap();
    assert!(inf.is_infinite());
  }

  #[test]
  fn configurable_endianness() {
    use crate::number::Endianness;

    fn be_tst16(i: &[u8]) -> IResult<&[u8], u16> {
      u16(Endianness::Big)(i)
    }
    fn le_tst16(i: &[u8]) -> IResult<&[u8], u16> {
      u16(Endianness::Little)(i)
    }
    assert_eq!(be_tst16(&[0x80, 0x00]), Ok((&b""[..], 32_768_u16)));
    assert_eq!(le_tst16(&[0x80, 0x00]), Ok((&b""[..], 128_u16)));

    fn be_tst32(i: &[u8]) -> IResult<&[u8], u32> {
      u32(Endianness::Big)(i)
    }
    fn le_tst32(i: &[u8]) -> IResult<&[u8], u32> {
      u32(Endianness::Little)(i)
    }
    assert_eq!(
      be_tst32(&[0x12, 0x00, 0x60, 0x00]),
      Ok((&b""[..], 302_014_464_u32))
    );
    assert_eq!(
      le_tst32(&[0x12, 0x00, 0x60, 0x00]),
      Ok((&b""[..], 6_291_474_u32))
    );

    fn be_tst64(i: &[u8]) -> IResult<&[u8], u64> {
      u64(Endianness::Big)(i)
    }
    fn le_tst64(i: &[u8]) -> IResult<&[u8], u64> {
      u64(Endianness::Little)(i)
    }
    assert_eq!(
      be_tst64(&[0x12, 0x00, 0x60, 0x00, 0x12, 0x00, 0x80, 0x00]),
      Ok((&b""[..], 1_297_142_246_100_992_000_u64))
    );
    assert_eq!(
      le_tst64(&[0x12, 0x00, 0x60, 0x00, 0x12, 0x00, 0x80, 0x00]),
      Ok((&b""[..], 36_028_874_334_666_770_u64))
    );

    fn be_tsti16(i: &[u8]) -> IResult<&[u8], i16> {
      i16(Endianness::Big)(i)
    }
    fn le_tsti16(i: &[u8]) -> IResult<&[u8], i16> {
      i16(Endianness::Little)(i)
    }
    assert_eq!(be_tsti16(&[0x00, 0x80]), Ok((&b""[..], 128_i16)));
    assert_eq!(le_tsti16(&[0x00, 0x80]), Ok((&b""[..], -32_768_i16)));

    fn be_tsti32(i: &[u8]) -> IResult<&[u8], i32> {
      i32(Endianness::Big)(i)
    }
    fn le_tsti32(i: &[u8]) -> IResult<&[u8], i32> {
      i32(Endianness::Little)(i)
    }
    assert_eq!(
      be_tsti32(&[0x00, 0x12, 0x60, 0x00]),
      Ok((&b""[..], 1_204_224_i32))
    );
    assert_eq!(
      le_tsti32(&[0x00, 0x12, 0x60, 0x00]),
      Ok((&b""[..], 6_296_064_i32))
    );

    fn be_tsti64(i: &[u8]) -> IResult<&[u8], i64> {
      i64(Endianness::Big)(i)
    }
    fn le_tsti64(i: &[u8]) -> IResult<&[u8], i64> {
      i64(Endianness::Little)(i)
    }
    assert_eq!(
      be_tsti64(&[0x00, 0xFF, 0x60, 0x00, 0x12, 0x00, 0x80, 0x00]),
      Ok((&b""[..], 71_881_672_479_506_432_i64))
    );
    assert_eq!(
      le_tsti64(&[0x00, 0xFF, 0x60, 0x00, 0x12, 0x00, 0x80, 0x00]),
      Ok((&b""[..], 36_028_874_334_732_032_i64))
    );
  }

  #[cfg(feature = "std")]
  fn parse_f64(i: &str) -> IResult<&str, f64, ()> {
    match recognize_float_or_exceptions(i) {
      Err(e) => Err(e),
      Ok((i, s)) => {
        if s.is_empty() {
          return Err(Err::Error(()));
        }
        match s.parse_to() {
          Some(n) => Ok((i, n)),
          None => Err(Err::Error(())),
        }
      }
    }
  }

  proptest! {
    #[test]
    #[cfg(feature = "std")]
    fn floats(s in "\\PC*") {
        println!("testing {}", s);
        let res1 = parse_f64(&s);
        let res2 = double::<_, ()>(s.as_str());
        assert_eq!(res1, res2);
    }
  }
}
#[cfg(test)]
mod tests_llm_16_460 {
  use crate::{
    number::complete::be_f32,
    IResult,
    error::{Error, ErrorKind},
  };

  #[test]
  fn test_be_f32() {
    fn parser(input: &[u8]) -> IResult<&[u8], f32, Error<&[u8]>> {
      be_f32(input)
    }

    let f32_data = &[0x41, 0x48, 0x00, 0x00];
    let f32_value: f32 = 12.5;
    let incomplete_data = &[0x00, 0x00, 0x00];
    let empty_data = &[];

    // Successful parsing
    assert_eq!(parser(&f32_data[..]), Ok((&empty_data[..], f32_value)));

    // Incomplete data
    assert_eq!(
      parser(&incomplete_data[..]),
      Err(crate::Err::Error(Error::new(&incomplete_data[..], ErrorKind::Eof)))
    );

    // No data
    assert_eq!(
      parser(&empty_data[..]),
      Err(crate::Err::Error(Error::new(&empty_data[..], ErrorKind::Eof)))
    );
  }
}#[cfg(test)]
mod tests_llm_16_463 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        Err, Needed,
    };

    #[test]
    fn test_be_i16() {
        let parser = |s| be_i16::<_, Error<_>>(s);

        assert_eq!(parser(&[0x00, 0x03][..]), Ok((&[][..], 0x0003)));
        assert_eq!(parser(&[0xFF, 0xFF][..]), Ok((&[][..], -1)));
        assert_eq!(parser(&[0x80, 0x00][..]), Ok((&[][..], -32768)));
        assert_eq!(parser(&[0x7F, 0xFF][..]), Ok((&[][..], 32767)));

        assert_eq!(
            parser(&[0x01][..]),
            Err(Err::Error(Error {
                input: &[0x01][..],
                code: ErrorKind::Eof
            }))
        );

        assert_eq!(parser(&[][..]), Err(Err::Incomplete(Needed::new(2))));
    }
}#[cfg(test)]
mod tests_llm_16_464 {
    use crate::{Err, error::ErrorKind, IResult, Needed};
    use crate::number::complete::be_i24;
    use crate::error::Error;
    use crate::error::ParseError;

    #[test]
    fn test_be_i24_positive() {
        let res: IResult<&[u8], i32, Error<&[u8]>> = be_i24(&[0x00, 0x03, 0x05]);
        assert_eq!(res, Ok((&[][..], 0x000305)));
    }

    #[test]
    fn test_be_i24_negative() {
        let res: IResult<&[u8], i32, Error<&[u8]>> = be_i24(&[0xFF, 0xAC, 0x15]);
        assert_eq!(res, Ok((&[][..], -21515)));
    }

    #[test]
    fn test_be_i24_incomplete() {
        let res: IResult<&[u8], i32, Error<&[u8]>> = be_i24(&[0x00, 0x03]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_be_i24_remaining_input() {
        let res: IResult<&[u8], i32, Error<&[u8]>> = be_i24(&[0x00, 0x03, 0x05, 0x06, 0x07]);
        assert_eq!(res, Ok((&[0x06, 0x07][..], 0x000305)));
    }

    #[test]
    fn test_be_i24_incorrect_input() {
        let res: IResult<&[u8], i32, Error<&[u8]>> = be_i24(&[]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(3))));
    }
}#[cfg(test)]
mod tests_llm_16_467 {
    use crate::{Err, error::{ErrorKind, ParseError, Error}, number::complete::be_i8, IResult};

    #[test]
    fn test_be_i8() {
        fn test_parser(input: &[u8]) -> IResult<&[u8], i8, Error<&[u8]>> {
            be_i8(input)
        }

        let test_cases: Vec<(&[u8], IResult<&[u8], i8, Error<&[u8]>>)> = vec![
            // Successful parsing
            (&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'], Ok((&[0x03, b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x00))),
            (&[0x7F, 0x00], Ok((&[0x00][..], 0x7F))),
            (&[0xFF], Ok((&[][..], -0x01))),
            (&[0x80], Ok((&[][..], -0x80))),
            // Incomplete parsing
            (&[][..], Err(Err::Error(Error::new(&[][..], ErrorKind::Eof)))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(test_parser(input), expected);
        }
    }
}#[cfg(test)]
mod tests_llm_16_468 {
    use crate::{
        error::{Error, ErrorKind},
        number::complete::be_u128,
        Err, IResult,
    };

    fn parser(input: &[u8]) -> IResult<&[u8], u128> {
        be_u128(input)
    }

    #[test]
    fn test_be_u128() {
        let data = &b"\x12\x34\x56\x78\x9a\xbc\xde\xf0\x12\x34\x56\x78\x9a\xbc\xde\xf0"[..];
        assert_eq!(
            parser(data),
            Ok((
                &b""[..],
                0x123456789abcdef0123456789abcdef0
            ))
        );

        let incomplete_data = &b"\x12\x34"[..];
        assert_eq!(
            parser(incomplete_data),
            Err(Err::Error(Error {
                input: incomplete_data,
                code: ErrorKind::Eof
            }))
        );

        let extra_data = &b"\x12\x34\x56\x78\x9a\xbc\xde\xf0\x12\x34\x56\x78\x9a\xbc\xde\xf0extra"[..];
        assert_eq!(
            parser(extra_data),
            Ok((
                &b"extra"[..],
                0x123456789abcdef0123456789abcdef0
            ))
        );

        let empty_data = &b""[..];
        assert_eq!(
            parser(empty_data),
            Err(Err::Error(Error {
                input: empty_data,
                code: ErrorKind::Eof
            }))
        );

        let data_with_error = &b"\x12\x34\x56\x78\x9a\xbc\xde\xf0\x12\x34\x56\x78\x9a\xbc\xde"[..];
        assert_eq!(
            parser(data_with_error),
            Err(Err::Error(Error {
                input: data_with_error,
                code: ErrorKind::Eof
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_469_llm_16_469 {
    use crate::number::complete::be_u16;
    use crate::{Err, IResult, Needed, error::{ErrorKind, Error}};

    #[test]
    fn test_be_u16_complete() {
        fn parse_be_u16(input: &[u8]) -> IResult<&[u8], u16, Error<&[u8]>> {
            be_u16(input)
        }

        let res = parse_be_u16(&[0x00, 0x03, 0x61, 0x62, 0x63, 0x65, 0x66, 0x67]);
        assert_eq!(res, Ok((&[0x61, 0x62, 0x63, 0x65, 0x66, 0x67][..], 0x0003)));

        let res = parse_be_u16(&[0x01]);
        assert_eq!(res, Err(Err::Error(Error::new(&[0x01][..], ErrorKind::Eof))));
       
        let res = parse_be_u16(&[]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(2))));
    }
}#[cfg(test)]
mod tests_llm_16_470 {
    use crate::{
        Err,
        error::{ErrorKind, ParseError},
        number::complete::be_u24,
        Needed,
    };

    #[test]
    fn test_be_u24() {
        fn test_parser(input: &[u8]) -> crate::IResult<&[u8], u32> {
            be_u24(input)
        }

        // Successful parsing
        let result = test_parser(&[0x00, 0x03, 0x05, b'a', b'b', b'c', b'e', b'f', b'g']);
        assert_eq!(result, Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x000305)));

        // Incomplete input
        let result = test_parser(&[0x01]);
        assert_eq!(result, Err(Err::Error(crate::error::Error::new(&[0x01][..], ErrorKind::Eof))));

        let result = test_parser(&[0x01, 0x02]);
        assert_eq!(result, Err(Err::Error(crate::error::Error::new(&[0x01, 0x02][..], ErrorKind::Eof))));

        // Complete input, but shorter than 3 bytes
        let result = test_parser(&[]);
        assert_eq!(result, Err(Err::Error(crate::error::Error::new(&[][..], ErrorKind::Eof))));

        // Complete input, exactly 3 bytes
        let result = test_parser(&[0xFF, 0xFF, 0xFF]);
        assert_eq!(result, Ok((&[][..], 0xFFFFFF)));

        // Input longer than 3 bytes, only first 3 considered
        let result = test_parser(&[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(result, Ok((&[0x78][..], 0x123456)));
    }
}#[cfg(test)]
mod tests_llm_16_471 {
    use crate::{
        error::{Error, ErrorKind},
        number::complete::be_u32,
        Err, IResult, Needed,
    };

    #[test]
    fn test_be_u32() {
        // Test for successful parsing
        let result: IResult<&[u8], u32> = be_u32(&[0x00, 0x03, 0x05, 0x07]);
        assert_eq!(result, Ok((&[][..], 0x00030507)));

        // Test for incomplete input
        let incomplete_result: IResult<&[u8], u32> = be_u32(&[0x00, 0x03]);
        assert_eq!(
            incomplete_result,
            Err(Err::Incomplete(Needed::new(2)))
        );

        // Test for error handling
        let error_result: IResult<&[u8], u32> = be_u32(&[]);
        assert_eq!(
            error_result,
            Err(Err::Error(Error {
                input: &[][..],
                code: ErrorKind::Eof,
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_472 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::number::complete::be_u64;

    #[test]
    fn test_be_u64_complete() {
        fn parse_be_u64(i: &[u8]) -> IResult<&[u8], u64, Error<&[u8]>> {
            be_u64(i)
        }

        // Positive case
        assert_eq!(
            parse_be_u64(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07][..]),
            Ok((&[][..], 0x0001020304050607u64))
        );

        // Input not long enough
        assert_eq!(
            parse_be_u64(&[0x01, 0x02, 0x03][..]),
            Err(Err::Error(Error::new(&[0x01, 0x02, 0x03][..], ErrorKind::Eof)))
        );

        // No input
        assert_eq!(
            parse_be_u64(&[][..]),
            Err(Err::Error(Error::new(&[][..], ErrorKind::Eof)))
        );

        // Input exactly 8 bytes long
        assert_eq!(
            parse_be_u64(&[0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88][..]),
            Ok((&[][..], 0xFFEEDDCCBBAA9988u64))
        );

        // Input longer than 8 bytes
        assert_eq!(
            parse_be_u64(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33][..]),
            Ok((&[0x11, 0x22, 0x33][..], 0x123456789ABCDEu64))
        );
    }
}#[cfg(test)]
mod tests_llm_16_473 {
    use super::*;

use crate::*;
    use crate::{Err, error::{ErrorKind, ParseError, Error}, IResult, Needed};

    #[test]
    fn be_u8_test() {
        fn parser(s: &[u8]) -> IResult<&[u8], u8, Error<&[u8]>> {
            be_u8(s)
        }

        assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
        assert_eq!(parser(&b"\xFF\x80"[..]), Ok((&b"\x80"[..], 0xFF)));
        assert_eq!(parser(&b"\x7F"[..]), Ok((&b""[..], 0x7F)));
        assert_eq!(parser(&b""[..]), Err(Err::Error(Error::new(&b""[..], ErrorKind::Eof))));
        assert_eq!(parser(&b"\x00\x01\x02\x03"[..]), Ok((&b"\x01\x02\x03"[..], 0x00)));
    }
}#[cfg(test)]
mod tests_llm_16_475 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::number::complete::double;

    #[test]
    fn test_double() {
        fn test_parser(input: &str) -> IResult<&str, f64> {
            double::<&str, Error<&str>>(input)
        }

        assert_eq!(test_parser("11e-1"), Ok(("", 1.1f64)));
        assert_eq!(test_parser("123E-02"), Ok(("", 1.23f64)));
        assert_eq!(test_parser("123K-01"), Ok(("K-01", 123.0f64)));
        assert_eq!(test_parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::Float))));
        assert_eq!(test_parser(""), Err(Err::Error(Error::new("", ErrorKind::Float))));
        assert_eq!(test_parser("12.34"), Ok(("", 12.34f64)));
        assert_eq!(test_parser("0x1.921fb54442d18p+1"), Ok(("", 3.141592653589793f64)));
        assert_eq!(test_parser("inf"), Ok(("", f64::INFINITY)));
        assert_eq!(test_parser("-inf"), Ok(("", f64::NEG_INFINITY)));
        assert_eq!(test_parser("nan"), Ok(("", f64::NAN)));
        assert!(test_parser("1.23.45").is_err());
    }
}#[cfg(test)]
mod tests_llm_16_477_llm_16_477 {
    use super::*;

use crate::*;
    use crate::error::ParseError;
    use crate::error::ErrorKind;
    use crate::number::complete::f64;
    use crate::number::Endianness;
    use crate::IResult;

    #[test]
    fn test_f64_big_endian() {
        let be_f64 = |s| {
            f64::<&[u8], (&[u8], ErrorKind)>(Endianness::Big)(s)
        };

        let input = &[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
        let expected = 12.5f64;
        match be_f64(input) {
            Ok((rest, value)) => {
                assert!(rest.is_empty());
                assert_eq!(value, expected);
            }
            Err(_) => assert!(false, "Failed to parse big-endian f64"),
        }

        let incomplete_input = &b"abc"[..];
        assert!(matches!(be_f64(incomplete_input), Err(Err::Error(_))));
    }

    #[test]
    fn test_f64_little_endian() {
        let le_f64 = |s| {
            f64::<&[u8], (&[u8], ErrorKind)>(Endianness::Little)(s)
        };

        let input = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..];
        let expected = 12.5f64;
        match le_f64(input) {
            Ok((rest, value)) => {
                assert!(rest.is_empty());
                assert_eq!(value, expected);
            }
            Err(_) => assert!(false, "Failed to parse little-endian f64"),
        }

        let incomplete_input = &b"abc"[..];
        assert!(matches!(le_f64(incomplete_input), Err(Err::Error(_))));
    }
}#[cfg(test)]
mod tests_llm_16_478 {
    use crate::number::complete::float;
    use crate::{Err, error::ErrorKind, error::Error};

    #[test]
    fn test_float() {
        fn test_parser(input: &str) -> crate::IResult<&str, f32, Error<&str>> {
            float(input)
        }

        let res = test_parser("11e-1");
        assert_eq!(res, Ok(("", 1.1)));

        let res = test_parser("123E-02");
        assert_eq!(res, Ok(("", 1.23)));

        let res = test_parser("123.45");
        assert_eq!(res, Ok(("", 123.45)));

        let res = test_parser("0.123");
        assert_eq!(res, Ok(("", 0.123)));

        let res = test_parser("123K-01");
        assert_eq!(res, Ok(("K-01", 123.0)));

        let res = test_parser("abc");
        assert_eq!(res, Err(Err::Error(Error::new("abc", ErrorKind::Float))));

        let res = test_parser("-12.34");
        assert_eq!(res, Ok(("", -12.34)));
    }
}#[cfg(test)]
mod tests_llm_16_479_llm_16_479 {
    use crate::{
        error::{Error, ErrorKind},
        number::complete::hex_u32,
        Err, IResult,
    };

    #[test]
    fn test_hex_u32() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, Error<&[u8]>> {
            hex_u32(s)
        }

        let empty: &[u8] = &[];
        
        assert_eq!(parser(b"01AE"), Ok((empty, 0x01AE)), "Regular hex value");
        assert_eq!(parser(b"abc"), Ok((empty, 0x0ABC)), "Hex with lowercase");
        assert_eq!(parser(b"ggg"), Err(Err::Error(Error { input: b"ggg" as &[u8], code: ErrorKind::IsA })), "Invalid hex digit");
        assert_eq!(parser(b"1"), Ok((empty, 0x1)), "Single digit");
        assert_eq!(parser(b"00000001"), Ok((empty, 0x1)), "Leading zeros");
        assert_eq!(parser(b"FFFFFFFF"), Ok((empty, 0xFFFFFFFF)), "Max value");
        assert_eq!(parser(b"FFFFFFFFF"), Ok((b"F" as &[u8], 0xFFFFFFFF)), "More than 8 digits");
    }
}#[cfg(test)]
mod tests_llm_16_485_llm_16_485 {
  use crate::{Err, error::{Error, ErrorKind}, IResult, number::complete::i8, Needed::Size};

  #[test]
  fn test_i8() {
    fn parse_i8(input: &[u8]) -> IResult<&[u8], i8, Error<&[u8]>> {
      i8(input)
    }

    // Test cases
    let inputs_outputs = vec![
      // Test case: Valid 1-byte input
      (&b"\x00\x03abcefg"[..], Ok((&b"\x03abcefg"[..], 0x00))),
      (&b"\x7Fother"[..], Ok((&b"other"[..], 0x7F))),
      (&b"\xFFrest"[..], Ok((&b"rest"[..], -1i8))),

      // Test case: Incomplete input
      // Empty input should yield an error
      (&b""[..], Err(Err::Error(Error::new(&[][..], ErrorKind::Eof)))),

      // Test case: Longer input
      (&b"\x2A\xFF"[..], Ok((&b"\xFF"[..], 0x2A))),

      // Test case: Input with only one byte
      (&b"\x2A"[..], Ok((&b""[..], 0x2A))),
    ];

    for (input, expected) in inputs_outputs {
      assert_eq!(parse_i8(input), expected);
    }
  }
}#[cfg(test)]
mod tests_llm_16_486_llm_16_486 {
  use super::*;

use crate::*;
  use crate::error::{Error, ErrorKind, ParseError};
  use crate::number::complete::le_f32;
  use crate::IResult;

  #[test]
  fn test_le_f32() {
    fn test_parser(input: &[u8]) -> IResult<&[u8], f32, Error<&[u8]>> {
      le_f32(input)
    }

    let expected = 12.5f32.to_le_bytes();
    assert_eq!(test_parser(&expected), Ok((&[] as &[u8], 12.5f32)));

    let incomplete_input = &[0x00, 0x00, 0x48];
    assert!(matches!(test_parser(incomplete_input), Err(Err::Error(Error { input, code: ErrorKind::Eof })) if input == incomplete_input));

    let non_f32_input = &[0x00];
    assert!(matches!(test_parser(non_f32_input), Err(Err::Error(_))));

    let extra_input = &[0x00, 0x00, 0x80, 0x3f, 0x00];
    assert_eq!(test_parser(extra_input), Ok((&extra_input[4..], 1.0f32)));

    let negative_input = (-12.5f32).to_le_bytes();
    assert_eq!(test_parser(&negative_input), Ok((&[] as &[u8], -12.5f32)));

    let max_input = f32::MAX.to_le_bytes();
    assert_eq!(test_parser(&max_input), Ok((&[] as &[u8], f32::MAX)));

    let min_input = f32::MIN.to_le_bytes();
    assert_eq!(test_parser(&min_input), Ok((&[] as &[u8], f32::MIN)));

    // Tests with incorrect endian input to ensure it's parsing little endian
    let big_endian_input = 12.5f32.to_be_bytes();
    assert_ne!(test_parser(&big_endian_input), Ok((&[] as &[u8], 12.5f32)));
  }
}#[cfg(test)]
mod tests_llm_16_487_llm_16_487 {
    use super::*; 

use crate::*;
    use crate::error::{Error, ErrorKind};
    use crate::Err;
    use crate::IResult;

    #[test]
    fn test_le_f64() {
        fn parser(input: &[u8]) -> IResult<&[u8], f64, Error<&[u8]>> {
            le_f64(input)
        }
        
        let endianness_input = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..];
        let incomplete_input = &[0x00, 0x00, 0x00][..];
        let excess_input = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40, 0x01, 0x02][..];
        
        assert_eq!(parser(endianness_input), Ok((&[][..], 12.5)));
        assert_eq!(parser(incomplete_input), Err(Err::Error(Error::from_error_kind(incomplete_input, ErrorKind::Eof))));
        assert_eq!(parser(excess_input), Ok((&[0x01, 0x02][..], 12.5)));
    }
}#[cfg(test)]
mod tests_llm_16_488 {
    use crate::{
        error::{Error, ErrorKind},
        number::complete::le_i128, Err, IResult, Needed,
    };

    #[test]
    fn test_le_i128() {
        let parse_le_i128 = |s| le_i128::<_, Error<&[u8]>>(s);

        let inputs = vec![
            (&b"\x12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"[..], 0x12_i128),
            (&b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x00\x00\x00\x00\x00\x00\x00\x80"[..], -1_i128),
            (&b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x06\x12"[..], 0x12060000000000000000000000000000_i128),
        ];

        for (input, expected) in inputs {
            assert_eq!(parse_le_i128(input), Ok((&[] as &[u8], expected)));
        }

        let incomplete_inputs = vec![
            &b"\x00"[..],
            &b"\x00\x01\x02\x03"[..],
            &b"\x00\x01\x02\x03\x04\x05\x06\x07"[..],
            &b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C"[..],
        ];

        for input in incomplete_inputs {
            assert_eq!(
                parse_le_i128(input),
                Err(Err::Error(Error {
                    input,
                    code: ErrorKind::Eof
                }))
            );
        }

        let remaining_inputs = vec![
            (
                &b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..],
                0x07060504030201000706050403020100_i128,
                &b"abcefg"[..],
            ),
            (
                &b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07"[..],
                0x07060504030201000706050403020100_i128,
                &[] as &[u8],
            ),
        ];

        for (input, expected_value, expected_remaining) in remaining_inputs {
            assert_eq!(
                parse_le_i128(input),
                Ok((expected_remaining, expected_value))
            );
        }
    }
}#[cfg(test)]
mod tests_llm_16_489_llm_16_489 {
    use super::*;

use crate::*;

    #[test]
    fn test_le_i16_success() {
        let parser = |s| le_i16::<&[u8], crate::error::Error<&[u8]>>(s);
        
        assert_eq!(parser(&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g']), Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0300i16)));
    }

    #[test]
    fn test_le_i16_incomplete() {
        let parser = |s| le_i16::<&[u8], crate::error::Error<&[u8]>>(s);
        use std::num::NonZeroUsize;
        
        assert_eq!(parser(&[0x01]), Err(Err::Incomplete(Needed::Size(NonZeroUsize::new(2).unwrap()))));
    }

    #[test]
    fn test_le_i16_error() {
        let parser = |s| le_i16::<&[u8], crate::error::Error<&[u8]>>(s);

        assert!(parser(&[]).is_err());
        if let Err(Err::Error(crate::error::Error { input, code: ErrorKind::Eof })) = parser(&[]) {
            assert!(input.is_empty());
        } else {
            panic!("Expected Err::Error with ErrorKind::Eof");
        }
    }
}#[cfg(test)]
mod tests_llm_16_490_llm_16_490 {
  use crate::{
    error::{Error as NomError, ErrorKind, ParseError},
    number::complete::le_i24, 
    Err, 
    Needed,
    IResult
  };

  #[test]
  fn test_le_i24() {
    fn run_le_i24(input: &[u8]) -> IResult<&[u8], i32, NomError<&[u8]>> {
      le_i24(input)
    }

    let positive_input = &[0x78, 0x56, 0x34]; // 3418472 in little-endian
    let negative_input = &[0x88, 0x99, 0xFF]; // -1672600 in little-endian (0xFF9988)
    let incomplete_input = &[0x01, 0x02]; // not enough bytes
    let remaining_input = &[0x78, 0x56, 0x34, 0x12]; // 3418472 in little-endian, with remaining 0x12
    let max_int_input = &[0xFF, 0xFF, 0x7F]; // 0x7FFFFF (8388607)
    let min_int_input = &[0x00, 0x00, 0x80]; // 0x800000 (-8388608)

    assert_eq!(run_le_i24(positive_input), Ok((&[][..], 3418472)));
    assert_eq!(run_le_i24(negative_input), Ok((&[][..], -1672600)));
    assert_eq!(run_le_i24(incomplete_input), Err(Err::Incomplete(Needed::new(1))));
    assert_eq!(run_le_i24(remaining_input), Ok((&[0x12][..], 3418472)));
    assert_eq!(run_le_i24(max_int_input), Ok((&[][..], 8388607)));
    assert_eq!(run_le_i24(min_int_input), Ok((&[][..], -8388608)));
  }
}#[cfg(test)]
mod tests_llm_16_491_llm_16_491 {
    use super::*;

use crate::*;
    use crate::{
        error::{Error, ErrorKind, ParseError},
        Err, IResult
    };

    #[test]
    fn le_i32_complete() {
        let empty: &[u8] = &[];
        let half: &[u8] = &[0x00, 0x03];
        let full: &[u8] = &[0x00, 0x03, 0x05, 0x07];
        let extra: &[u8] = &[0x00, 0x03, 0x05, 0x07, 0xAB, 0xCD, 0xEF, 0x01];
        
        let expected_full = 0x07050300_i32;
        fn parse_le_i32(input: &[u8]) -> IResult<&[u8], i32, Error<&[u8]>> {
            le_i32(input)
        }

        assert_eq!(parse_le_i32(full), Ok((empty, expected_full)));
        assert_eq!(parse_le_i32(extra), Ok((&extra[4..], expected_full)));
        assert_eq!(parse_le_i32(half), Err(Err::Error(Error::new(half, ErrorKind::Eof))));
    }
}#[cfg(test)]
mod tests_llm_16_492 {
    use crate::{
        number::complete::le_i64,
        error::{ErrorKind, ParseError, Error},
        Err,
        Needed,
        IResult,
    };

    #[test]
    fn test_le_i64() {
        fn test_parser(input: &[u8]) -> IResult<&[u8], i64, Error<&[u8]>> {
            le_i64(input)
        }

        let input_bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67];
        assert_eq!(
            test_parser(input_bytes),
            Ok((&b"abcdefg"[..], 0x0706050403020100))
        );

        let incomplete_bytes: &[u8] = &[0x01];
        assert_eq!(
            test_parser(incomplete_bytes),
            Err(Err::Incomplete(Needed::new(7)))
        );

        let input_bytes_with_error: &[u8] = &[0x00, 0x01, 0x02];
        assert_eq!(
            test_parser(input_bytes_with_error),
            Err(Err::Incomplete(Needed::new(5)))
        );

        let input_bytes_negative: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        assert_eq!(
            test_parser(input_bytes_negative),
            Ok((&[][..], -1))
        );

        let input_bytes_max: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        assert_eq!(
            test_parser(input_bytes_max),
            Ok((&[][..], i64::MAX))
        );

        let input_bytes_min: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        assert_eq!(
            test_parser(input_bytes_min),
            Ok((&[][..], i64::MIN))
        );
    }
}#[cfg(test)]
mod tests_llm_16_493_llm_16_493 {
    use crate::{Err, IResult, Needed, error::{ErrorKind, ParseError}, number::complete::le_i8};

    #[test]
    fn test_le_i8_success() {
        let res: IResult<&[u8], i8> = le_i8(&[0x01, 0x02, 0x03]);
        assert_eq!(res, Ok((&[0x02, 0x03][..], 0x01i8)));
    }

    #[test]
    fn test_le_i8_incomplete() {
        let res: IResult<&[u8], i8> = le_i8(&[]);
        assert_eq!(res, Err(Err::Error(crate::error::Error::new(&[][..], ErrorKind::Eof))));
    }

    #[test]
    fn test_le_i8_negative() {
        let res: IResult<&[u8], i8> = le_i8(&[0xff]);
        assert_eq!(res, Ok((&[][..], -1i8)));
    }

    #[test]
    fn test_le_i8_at_eof() {
        let res: IResult<&[u8], i8> = le_i8(&[0x02]);
        assert_eq!(res, Ok((&[][..], 0x02i8)));
    }
}#[cfg(test)]
mod tests_llm_16_494_llm_16_494 {
    use crate::{number::complete::le_u128, IResult, Err, Needed, error::{Error, ErrorKind, ParseError}};

    #[test]
    fn test_le_u128() {
        let parser = |s| le_u128::<_, Error<_>>(s);

        let result = parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]);
        assert_eq!(result, Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));

        let incomplete = parser(&b"\x01"[..]);
        assert_eq!(incomplete, Err(Err::Error(Error::new(&b"\x01"[..], ErrorKind::Eof))));

        let not_enough_input = parser(&b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"[..]);
        assert_eq!(not_enough_input, Err(Err::Incomplete(Needed::new(1))));

        let just_enough_input = parser(&b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10"[..]);
        assert_eq!(just_enough_input, Ok((&b""[..], 0x100F0E0D0C0B0A090807060504030201)));

        let too_much_input = parser(&b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10abcdef"[..]);
        assert_eq!(too_much_input, Ok((&b"abcdef"[..], 0x100F0E0D0C0B0A090807060504030201)));
    }
}#[cfg(test)]
mod tests_llm_16_496 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        number::complete::le_u24, IResult,
    };

    #[test]
    fn test_le_u24() {
        fn parser(input: &[u8]) -> IResult<&[u8], u32, Error<&[u8]>> {
            le_u24(input)
        }

        // Test parsing a valid 3-byte input
        assert_eq!(parser(&[0x00, 0x03, 0x05, 0x61, 0x62, 0x63, 0x65, 0x66, 0x67]), Ok((&[0x61, 0x62, 0x63, 0x65, 0x66, 0x67][..], 0x050300)));
        
        // Test input which is too short
        assert_eq!(parser(&[0x01]), Err(crate::Err::Error(Error::new(&[0x01][..], ErrorKind::Eof))));
        
        // Test complete input
        assert_eq!(parser(&[0x00, 0x03, 0x05]), Ok((&[][..], 0x050300)));
        
        // Test empty input
        assert_eq!(parser(&[]), Err(crate::Err::Error(Error::new(&[][..], ErrorKind::Eof))));
        
        // Test input longer than 3 bytes
        assert_eq!(parser(&[0xff, 0xff, 0xff, 0x01]), Ok((&[0x01][..], 0xffffff)));
    }
}#[cfg(test)]
mod tests_llm_16_498 {
    use crate::{
        error::{Error, ErrorKind},
        number::complete::le_u64,
        Err, IResult, Needed,
    };

    #[test]
    fn test_le_u64() {
        fn test_parser(input: &[u8]) -> IResult<&[u8], u64, Error<&[u8]>> {
            le_u64(input)
        }

        // Successful case
        assert_eq!(
            test_parser(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x61, 0x62][..]),
            Ok((&[0x61, 0x62][..], 0x0706050403020100u64))
        );

        // Not enough data to parse
        assert_eq!(
            test_parser(&[0x01]),
            Err(Err::Error(Error {
                input: &[0x01][..],
                code: ErrorKind::Eof
            }))
        );

        // Exactly 8 bytes, no remaining input
        assert_eq!(
            test_parser(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
            Ok((&[][..], 0x0807060504030201u64))
        );

        // More than 8 bytes, with remaining input
        assert_eq!(
            test_parser(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A][..]),
            Ok((&[0x09, 0x0A][..], 0x0807060504030201u64))
        );

        // Not enough data, but with incomplete input
        assert_eq!(
            test_parser(&[0x01, 0x02, 0x03]),
            Err(Err::Error(Error {
                input: &[0x01, 0x02, 0x03][..],
                code: ErrorKind::Eof
            }))
        );

        // Test with the complete error input (empty input should return an error, not incomplete)
        assert_eq!(
            test_parser(&[]),
            Err(Err::Error(Error {
                input: &[][..],
                code: ErrorKind::Eof
            }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_499 {
    use crate::{Err, error::{Error, ErrorKind}, Needed};
    use crate::number::complete::le_u8;
    use crate::IResult;

    #[test]
    fn test_le_u8() {
        fn test_parser(input: &[u8]) -> IResult<&[u8], u8, Error<&[u8]>> {
            le_u8(input)
        }

        let empty_input: &[u8] = &[];
        let incomplete_input: &[u8] = &[0x05];
        let valid_input: &[u8] = &[0x12, 0x34, 0x56];
        let valid_input_expected_remainder: &[u8] = &[0x34, 0x56];
        let invalid_input: &[u8] = &[];

        // Test a valid input
        assert_eq!(test_parser(valid_input), Ok((valid_input_expected_remainder, 0x12)));

        // Test an incomplete input
        assert_eq!(
            test_parser(incomplete_input),
            Ok((&[][..], 0x05))
        );

        // Test an empty input, which should result in an error.
        assert_eq!(
            test_parser(invalid_input),
            Err(Err::Error(Error { input: empty_input, code: ErrorKind::Eof }))
        );
    }
}#[cfg(test)]
mod tests_llm_16_505_llm_16_505 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        number::complete::u16,
        number::Endianness,
        Err, IResult,
    };

    #[test]
    fn test_u16_big_endian() {
        let parse_u16_big_endian = |s| u16::<_, Error<&[u8]>>(Endianness::Big)(s);

        assert_eq!(
            parse_u16_big_endian(&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'][..]),
            Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0003))
        );
        assert_eq!(
            parse_u16_big_endian(&[0x01][..]),
            Err(Err::Error(Error::from_error_kind(&[0x01][..], ErrorKind::Eof)))
        );
    }

    #[test]
    fn test_u16_little_endian() {
        let parse_u16_little_endian = |s| u16::<_, Error<&[u8]>>(Endianness::Little)(s);

        assert_eq!(
            parse_u16_little_endian(&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'][..]),
            Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0300))
        );
        assert_eq!(
            parse_u16_little_endian(&[0x01][..]),
            Err(Err::Error(Error::from_error_kind(&[0x01][..], ErrorKind::Eof)))
        );
    }

    #[test]
    fn test_u16_native_endian() {
        let parse_u16_native_endian = |s| u16::<_, Error<&[u8]>>(Endianness::Native)(s);

        #[cfg(target_endian = "big")]
        let expected = Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0003));
        #[cfg(target_endian = "little")]
        let expected = Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0300));

        assert_eq!(
            parse_u16_native_endian(&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'][..]),
            expected
        );
    }
}#[cfg(test)]
mod tests_llm_16_506 {
    use crate::number::complete::u24;
    use crate::number::Endianness;
    use crate::error::ErrorKind;
    use crate::error::ParseError;
    use crate::{Err, IResult, Needed};

    #[test]
    fn test_u24_big_endian_complete() {
        let be_u24 = |s| u24(Endianness::Big)(s);
        assert_eq!(be_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305u32)));
        assert_eq!(be_u24(&b"\x01\x02\x03"[..]), Ok((&b""[..], 0x010203u32)));
        assert_eq!(be_u24(&b"\x01"[..]), Err(Err::Error((&b"\x01"[..], ErrorKind::Eof))));
    }

    #[test]
    fn test_u24_little_endian_complete() {
        let le_u24 = |s| u24(Endianness::Little)(s);
        assert_eq!(le_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300u32)));
        assert_eq!(le_u24(&b"\x01\x02\x03"[..]), Ok((&b""[..], 0x030201u32)));
        assert_eq!(le_u24(&b"\x01"[..]), Err(Err::Error((&b"\x01"[..], ErrorKind::Eof))));
    }

    #[test]
    fn test_u24_incomplete() {
        let be_u24 = |s| u24(Endianness::Big)(s);
        // The case with 2 bytes missing
        assert_eq!(be_u24(&b"\x01"[..]), Err(Err::Error((&b"\x01"[..], ErrorKind::Eof))));
        // The case with 1 byte missing
        assert_eq!(be_u24(&b"\x01\x02"[..]), Err(Err::Error((&b"\x01\x02"[..], ErrorKind::Eof))));
    }

    #[test]
    fn test_u24_big_endian_at_eof() {
        let be_u24 = |s| u24(Endianness::Big)(s);
        assert_eq!(be_u24(&b""[..]), Err(Err::Error((&b""[..], ErrorKind::Eof))));
    }
}#[cfg(test)]
mod tests_llm_16_507 {
    use crate::{Err, IResult, error::ErrorKind, number::complete::u32, number::Endianness};

    #[test]
    fn test_u32_be() {
        let be_parser = |s| u32(Endianness::Big)(s);
        let input = &b"\x00\x03\x05\x07rest"[..];
        assert_eq!(be_parser(input), Ok((&b"rest"[..], 0x00030507u32)));

        let incomplete_input = &b"\x00\x03\x05"[..];
        assert_eq!(be_parser(incomplete_input), Err(Err::Error((&b"\x00\x03\x05"[..], ErrorKind::Eof))));
    }

    #[test]
    fn test_u32_le() {
        let le_parser = |s| u32(Endianness::Little)(s);
        let input = &b"\x07\x05\x03\x00rest"[..];
        assert_eq!(le_parser(input), Ok((&b"rest"[..], 0x00030507u32)));

        let incomplete_input = &b"\x07\x05\x03"[..];
        assert_eq!(le_parser(incomplete_input), Err(Err::Error((&b"\x07\x05\x03"[..], ErrorKind::Eof))));
    }

    #[test]
    #[cfg(target_endian = "big")]
    fn test_u32_native_big_endian() {
        let native_parser = |s| u32(Endianness::Native)(s);
        let input = &b"\x00\x03\x05\x07rest"[..];
        assert_eq!(native_parser(input), Ok((&b"rest"[..], 0x00030507u32)));

        let incomplete_input = &b"\x00\x03\x05"[..];
        assert_eq!(native_parser(incomplete_input), Err(Err::Error((&b"\x00\x03\x05"[..], ErrorKind::Eof))));
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn test_u32_native_little_endian() {
        let native_parser = |s| u32(Endianness::Native)(s);
        let input = &b"\x07\x05\x03\x00rest"[..];
        assert_eq!(native_parser(input), Ok((&b"rest"[..], 0x00030507u32)));

        let incomplete_input = &b"\x07\x05\x03"[..];
        assert_eq!(native_parser(incomplete_input), Err(Err::Error((&b"\x07\x05\x03"[..], ErrorKind::Eof))));
    }
}