//! Parsers recognizing numbers, streaming version

use crate::branch::alt;
use crate::bytes::streaming::tag;
use crate::character::streaming::{char, digit1, sign};
use crate::combinator::{cut, map, opt, recognize};
use crate::error::{ErrorKind, ParseError};
use crate::lib::std::ops::{Add, Shl};
use crate::sequence::pair;
use crate::traits::{AsBytes, AsChar, Compare, Offset};
use crate::{internal::*, Input};

/// Recognizes an unsigned 1 byte integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u8;
///
/// let parser = |s| {
///   be_u8::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u16;
///
/// let parser = |s| {
///   be_u16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0001)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u24;
///
/// let parser = |s| {
///   be_u24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x000102)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u32;
///
/// let parser = |s| {
///   be_u32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x00010203)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u64;
///
/// let parser = |s| {
///   be_u64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_u128;
///
/// let parser = |s| {
///   be_u128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x00010203040506070809101112131415)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i8;
///
/// let parser = be_i8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i16;
///
/// let parser = be_i16::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0001)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i24;
///
/// let parser = be_i24::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x000102)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i32;
///
/// let parser = be_i32::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x00010203)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(4))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i64;
///
/// let parser = be_i64::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0001020304050607)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_i128;
///
/// let parser = be_i128::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x00010203040506070809101112131415)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u8;
///
/// let parser = le_u8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u16;
///
/// let parser = |s| {
///   le_u16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_u16<I, E: ParseError<I>>(input: I) -> IResult<I, u16, E>
where
  I: Input<Item = u8>,
{
  le_uint(input, 2)
}

/// Recognizes a little endian unsigned 3 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u24;
///
/// let parser = |s| {
///   le_u24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u32;
///
/// let parser = |s| {
///   le_u32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x03020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u64;
///
/// let parser = |s| {
///   le_u64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_u128;
///
/// let parser = |s| {
///   le_u128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x15141312111009080706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
    Err(Err::Incomplete(Needed::new(bound - input.input_len())))
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i8;
///
/// let parser = le_i8::<_, (_, ErrorKind)>;
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"\x01abcd"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn le_i8<I, E: ParseError<I>>(input: I) -> IResult<I, i8, E>
where
  I: Input<Item = u8>,
{
  le_u8.map(|x| x as i8).parse(input)
}

/// Recognizes a little endian signed 2 bytes integer.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i16;
///
/// let parser = |s| {
///   le_i16::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01abcd"[..]), Ok((&b"abcd"[..], 0x0100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i24;
///
/// let parser = |s| {
///   le_i24::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02abcd"[..]), Ok((&b"abcd"[..], 0x020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i32;
///
/// let parser = |s| {
///   le_i32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03abcd"[..]), Ok((&b"abcd"[..], 0x03020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i64;
///
/// let parser = |s| {
///   le_i64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_i128;
///
/// let parser = |s| {
///   le_i128::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..]), Ok((&b"abcd"[..], 0x15141312111009080706050403020100)));
/// assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u8;
///
/// let parser = |s| {
///   u8::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[inline]
pub fn u8<I, E: ParseError<I>>(input: I) -> IResult<I, u8, E>
where
  I: Input<Item = u8>,
{
  let bound: usize = 1;
  if input.input_len() < bound {
    Err(Err::Incomplete(Needed::new(1)))
  } else {
    let res = input.iter_elements().next().unwrap();

    Ok((input.take_from(bound), res))
  }
}

/// Recognizes an unsigned 2 bytes integer
///
/// If the parameter is `nom::number::Endianness::Big`, parse a big endian u16 integer,
/// otherwise if `nom::number::Endianness::Little` parse a little endian u16 integer.
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u16;
///
/// let be_u16 = |s| {
///   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(be_u16(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
///
/// let le_u16 = |s| {
///   u16::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(le_u16(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u24;
///
/// let be_u24 = |s| {
///   u24::<_,(_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(be_u24(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
///
/// let le_u24 = |s| {
///   u24::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(le_u24(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u32;
///
/// let be_u32 = |s| {
///   u32::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(be_u32(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
///
/// let le_u32 = |s| {
///   u32::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(le_u32(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u64;
///
/// let be_u64 = |s| {
///   u64::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(be_u64(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
///
/// let le_u64 = |s| {
///   u64::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(le_u64(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::u128;
///
/// let be_u128 = |s| {
///   u128::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_u128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(be_u128(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
///
/// let le_u128 = |s| {
///   u128::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_u128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(le_u128(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i8;
///
/// let parser = |s| {
///   i8::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&b"\x00\x03abcefg"[..]), Ok((&b"\x03abcefg"[..], 0x00)));
/// assert_eq!(parser(&b""[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i16;
///
/// let be_i16 = |s| {
///   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
/// assert_eq!(be_i16(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
///
/// let le_i16 = |s| {
///   i16::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i16(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
/// assert_eq!(le_i16(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i24;
///
/// let be_i24 = |s| {
///   i24::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x000305)));
/// assert_eq!(be_i24(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
///
/// let le_i24 = |s| {
///   i24::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i24(&b"\x00\x03\x05abcefg"[..]), Ok((&b"abcefg"[..], 0x050300)));
/// assert_eq!(le_i24(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i32;
///
/// let be_i32 = |s| {
///   i32::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00030507)));
/// assert_eq!(be_i32(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
///
/// let le_i32 = |s| {
///   i32::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i32(&b"\x00\x03\x05\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07050300)));
/// assert_eq!(le_i32(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i64;
///
/// let be_i64 = |s| {
///   i64::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0001020304050607)));
/// assert_eq!(be_i64(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
///
/// let le_i64 = |s| {
///   i64::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i64(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x0706050403020100)));
/// assert_eq!(le_i64(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::i128;
///
/// let be_i128 = |s| {
///   i128::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_i128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x00010203040506070001020304050607)));
/// assert_eq!(be_i128(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
///
/// let le_i128 = |s| {
///   i128::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_i128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]), Ok((&b"abcefg"[..], 0x07060504030201000706050403020100)));
/// assert_eq!(le_i128(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(15))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_f32;
///
/// let parser = |s| {
///   be_f32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00][..]), Ok((&b""[..], 2.640625)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::be_f64;
///
/// let parser = |s| {
///   be_f64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_f32;
///
/// let parser = |s| {
///   le_f32::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(3))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::le_f64;
///
/// let parser = |s| {
///   le_f64::<_, (_, ErrorKind)>(s)
/// };
///
/// assert_eq!(parser(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 3145728.0)));
/// assert_eq!(parser(&[0x01][..]), Err(Err::Incomplete(Needed::new(7))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::f32;
///
/// let be_f32 = |s| {
///   f32::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_f32(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(be_f32(&b"abc"[..]), Err(Err::Incomplete(Needed::new(1))));
///
/// let le_f32 = |s| {
///   f32::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_f32(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(le_f32(&b"abc"[..]), Err(Err::Incomplete(Needed::new(1))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// # use nom::Needed::Size;
/// use nom::number::streaming::f64;
///
/// let be_f64 = |s| {
///   f64::<_, (_, ErrorKind)>(nom::number::Endianness::Big)(s)
/// };
///
/// assert_eq!(be_f64(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(be_f64(&b"abc"[..]), Err(Err::Incomplete(Needed::new(5))));
///
/// let le_f64 = |s| {
///   f64::<_, (_, ErrorKind)>(nom::number::Endianness::Little)(s)
/// };
///
/// assert_eq!(le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]), Ok((&b""[..], 12.5)));
/// assert_eq!(le_f64(&b"abc"[..]), Err(Err::Incomplete(Needed::new(5))));
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::hex_u32;
///
/// let parser = |s| {
///   hex_u32(s)
/// };
///
/// assert_eq!(parser(&b"01AE;"[..]), Ok((&b";"[..], 0x01AE)));
/// assert_eq!(parser(&b"abc"[..]), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(parser(&b"ggg"[..]), Err(Err::Error((&b"ggg"[..], ErrorKind::IsA))));
/// ```
#[inline]
pub fn hex_u32<I, E: ParseError<I>>(input: I) -> IResult<I, u32, E>
where
  I: Input + AsBytes,
  <I as Input>::Item: AsChar,
{
  let e: ErrorKind = ErrorKind::IsA;
  let (i, o) = input.split_at_position1(
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

/// Recognizes a floating point number in text format and returns the corresponding part of the input.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed};
/// use nom::number::streaming::recognize_float;
///
/// let parser = |s| {
///   recognize_float(s)
/// };
///
/// assert_eq!(parser("11e-1;"), Ok((";", "11e-1")));
/// assert_eq!(parser("123E-02;"), Ok((";", "123E-02")));
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
      crate::bytes::streaming::tag_no_case::<_, _, E>("nan")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
    |i: T| {
      crate::bytes::streaming::tag_no_case::<_, _, E>("inf")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
    |i: T| {
      crate::bytes::streaming::tag_no_case::<_, _, E>("infinity")(i.clone())
        .map_err(|_| crate::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
    },
  ))(input)
}

/// Recognizes a floating point number in text format
///
/// It returns a tuple of (`sign`, `integer part`, `fraction part` and `exponent`) of the input
/// data.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
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

  //let (i, zeroes) = take_while(|c: <T as InputTakeAtPosition>::Item| c.as_char() == '0')(i)?;
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

    let position = match position {
      Some(p) => p,
      None => return Err(Err::Incomplete(Needed::new(1))),
    };

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
    cut(crate::character::streaming::i32)(i)?
  } else {
    (i2, 0)
  };

  Ok((i, (sign, integer, fraction, exp)))
}

/// Recognizes floating point number in text format and returns a f32.
///
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
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
  T: Clone + Offset,
  T: Input + crate::traits::ParseTo<f32> + Compare<&'static str>,
  <T as Input>::Item: AsChar + Clone,
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
/// *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there is not enough data.
///
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
  T: Clone + Offset,
  T: Input + crate::traits::ParseTo<f64> + Compare<&'static str>,
  <T as Input>::Item: AsChar + Clone,
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
  use crate::internal::{Err, Needed};
  use proptest::prelude::*;

  macro_rules! assert_parse(
    ($left: expr, $right: expr) => {
      let res: $crate::IResult<_, _, (_, ErrorKind)> = $left;
      assert_eq!(res, $right);
    };
  );

  #[test]
  fn i8_tests() {
    assert_parse!(be_i8(&[0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i8(&[0x7f][..]), Ok((&b""[..], 127)));
    assert_parse!(be_i8(&[0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i8(&[0x80][..]), Ok((&b""[..], -128)));
    assert_parse!(be_i8(&[][..]), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn i16_tests() {
    assert_parse!(be_i16(&[0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_i16(&[0x7f, 0xff][..]), Ok((&b""[..], 32_767_i16)));
    assert_parse!(be_i16(&[0xff, 0xff][..]), Ok((&b""[..], -1)));
    assert_parse!(be_i16(&[0x80, 0x00][..]), Ok((&b""[..], -32_768_i16)));
    assert_parse!(be_i16(&[][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(be_i16(&[0x00][..]), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn u24_tests() {
    assert_parse!(be_u24(&[0x00, 0x00, 0x00][..]), Ok((&b""[..], 0)));
    assert_parse!(be_u24(&[0x00, 0xFF, 0xFF][..]), Ok((&b""[..], 65_535_u32)));
    assert_parse!(
      be_u24(&[0x12, 0x34, 0x56][..]),
      Ok((&b""[..], 1_193_046_u32))
    );
    assert_parse!(be_u24(&[][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(be_u24(&[0x00][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      be_u24(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn i24_tests() {
    assert_parse!(be_i24(&[0xFF, 0xFF, 0xFF][..]), Ok((&b""[..], -1_i32)));
    assert_parse!(be_i24(&[0xFF, 0x00, 0x00][..]), Ok((&b""[..], -65_536_i32)));
    assert_parse!(
      be_i24(&[0xED, 0xCB, 0xAA][..]),
      Ok((&b""[..], -1_193_046_i32))
    );
    assert_parse!(be_i24(&[][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(be_i24(&[0x00][..]), Err(Err::Incomplete(Needed::new(2))));
    assert_parse!(
      be_i24(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn i32_tests() {
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
    assert_parse!(be_i32(&[][..]), Err(Err::Incomplete(Needed::new(4))));
    assert_parse!(be_i32(&[0x00][..]), Err(Err::Incomplete(Needed::new(3))));
    assert_parse!(
      be_i32(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i32(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn i64_tests() {
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
    assert_parse!(be_i64(&[][..]), Err(Err::Incomplete(Needed::new(8))));
    assert_parse!(be_i64(&[0x00][..]), Err(Err::Incomplete(Needed::new(7))));
    assert_parse!(
      be_i64(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(6)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(5)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(4)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(3)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(1)))
    );
  }

  #[test]
  fn i128_tests() {
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
    assert_parse!(be_i128(&[][..]), Err(Err::Incomplete(Needed::new(16))));
    assert_parse!(be_i128(&[0x00][..]), Err(Err::Incomplete(Needed::new(15))));
    assert_parse!(
      be_i128(&[0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(14)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(13)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(12)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(11)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(10)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(9)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(8)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(7)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(6)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(5)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(4)))
    );
    assert_parse!(
      be_i128(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
      Err(Err::Incomplete(Needed::new(3)))
    );
    assert_parse!(
      be_i128(
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]
      ),
      Err(Err::Incomplete(Needed::new(2)))
    );
    assert_parse!(
      be_i128(
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
          [..]
      ),
      Err(Err::Incomplete(Needed::new(1)))
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
    assert_parse!(hex_u32(&b"12af"[..]), Err(Err::Incomplete(Needed::new(1))));
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

      let larger = format!("{};", test);
      assert_parse!(recognize_float(&larger[..]), Ok((";", test)));

      assert_parse!(float(larger.as_bytes()), Ok((&b";"[..], expected32)));
      assert_parse!(float(&larger[..]), Ok((";", expected32)));

      assert_parse!(double(larger.as_bytes()), Ok((&b";"[..], expected64)));
      assert_parse!(double(&larger[..]), Ok((";", expected64)));
    }

    let remaining_exponent = "-1.234E-";
    assert_parse!(
      recognize_float(remaining_exponent),
      Err(Err::Incomplete(Needed::new(1)))
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
    use crate::traits::ParseTo;
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
mod tests_llm_16_510 {
    use crate::{Err, Needed};
    use crate::number::streaming::be_f32;
    use crate::error::{ErrorKind, Error};

    #[test]
    fn test_be_f32() {
        let test_cases = vec![
            (&[0x40, 0x49, 0x0F, 0xDB][..], Ok((&b""[..], 3.1415927))),
            (&[0x41, 0x45, 0x85, 0x1F][..], Ok((&b""[..], 12.34567))),
            (&[0x00, 0x00, 0x00, 0x00][..], Ok((&b""[..], 0.0))),
            (&[0xFF, 0x80, 0x00, 0x00][..], Ok((&b""[..], -0.0))),
            (&[0x7F, 0x80, 0x00, 0x00][..], Ok((&b""[..], f32::INFINITY))),
            (&[0xFF, 0x80, 0x00, 0x00][..], Ok((&b""[..], f32::NEG_INFINITY))),
            (&[0x7F, 0xC0, 0x00, 0x00][..], Ok((&b""[..], f32::NAN))),
            (&[0x40, 0x49, 0x0F][..], Err(Err::Incomplete(Needed::new(1)))),
            (&[0x40, 0x49][..], Err(Err::Incomplete(Needed::new(2)))),
            (&[0x40][..], Err(Err::Incomplete(Needed::new(3)))),
            (&[][..], Err(Err::Incomplete(Needed::new(4)))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(be_f32::<_, (_, ErrorKind)>(input), expected);
        }
    }
}#[cfg(test)]
mod tests_llm_16_511 {
    use crate::{
        error::{ErrorKind, ParseError},
        number::streaming::be_f64,
        Err, IResult, Needed,
    };

    #[test]
    fn test_be_f64() {
        fn parse_be_f64(input: &[u8]) -> IResult<&[u8], f64, crate::error::Error<&[u8]>> {
            be_f64(input)
        }

        let input_full = &[0x40, 0x09, 0x21, 0xFB, 0x54, 0x44, 0x2D, 0x18];
        let input_incomplete = &[0x40, 0x09, 0x21];
        let expected_value = 3.141592653589793;

        // Test complete input
        match parse_be_f64(input_full) {
            Ok((remaining, value)) => {
                assert!(remaining.is_empty(), "Expected no remaining input, got {:?}", remaining);
                assert!(
                    (value - expected_value).abs() < f64::EPSILON,
                    "Expected value {:?}, got {:?}",
                    expected_value,
                    value
                );
            },
            Err(e) => panic!("Expected successful parse, got error {:?}", e),
        }

        // Test incomplete input
        match parse_be_f64(input_incomplete) {
            Err(Err::Incomplete(Needed::Size(n))) => assert_eq!(n.get(), 5, "Expected needed size 5, got {:?}", n),
            Err(e) => panic!("Expected incomplete parse, got error {:?}", e),
            Ok(_) => panic!("Expected error, got successful parse"),
        }
    }
}#[cfg(test)]
mod tests_llm_16_513 {
    use crate::{
        error::{ErrorKind, ParseError},
        Err, Needed,
    };
    use crate::number::streaming::be_i16;

    #[test]
    fn test_be_i16_complete() {
        let empty: &[u8] = b"";
        let short: &[u8] = b"\x01";
        let valid: &[u8] = b"\x01\x02test";
        let valid_negative: &[u8] = b"\xFF\xFEtest";
        let extra: &[u8] = b"\x01\x02\x03\x04\x05\x06";

        assert_eq!(be_i16::<_, (_, ErrorKind)>(valid), Ok((&b"test"[..], 0x0102)));
        assert_eq!(be_i16::<_, (_, ErrorKind)>(valid_negative), Ok((&b"test"[..], -2)));
        assert_eq!(be_i16::<_, (_, ErrorKind)>(short), Err(Err::Incomplete(Needed::new(2))));
        assert_eq!(be_i16::<_, (_, ErrorKind)>(empty), Err(Err::Incomplete(Needed::new(2))));
        assert_eq!(be_i16::<_, (_, ErrorKind)>(extra), Ok((&extra[2..], 0x0102)));
    }
}#[cfg(test)]
mod tests_llm_16_514 {
    use crate::{
        error::{Error, ErrorKind},
        number::streaming::be_i24,
        Err, IResult, Needed,
    };

    #[test]
    fn test_be_i24() {
        fn test_parser(input: &[u8]) -> IResult<&[u8], i32, Error<&[u8]>> {
            be_i24::<_, Error<&[u8]>>(input)
        }

        // Test successful parsing
        let res = test_parser(&b"\x00\x01\x02abcd"[..]);
        assert_eq!(res, Ok((&b"abcd"[..], 0x000102)));

        // Test incomplete input
        let res = test_parser(&b""[..]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(3))));

        // Test error
        let res = test_parser(&b"\xff\xfe"[..]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(3))));

        // Test successful parsing for the maximum positive value
        let res = test_parser(&b"\x7f\xff\xffabcd"[..]);
        assert_eq!(res, Ok((&b"abcd"[..], 0x7f_ff_ff)));

        // Test successful parsing for the minimum negative value
        let res = test_parser(&b"\x80\x00\x00abcd"[..]);
        assert_eq!(res, Ok((&b"abcd"[..], -0x800000)));

        // Test successful parsing for negative value
        let res = test_parser(&b"\xff\xff\xfefoobar"[..]);
        assert_eq!(res, Ok((&b"foobar"[..], -0x000102)));

        // Test incomplete error at the very end of the input
        let res = test_parser(&b"\x01\x02"[..]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(3))));
    }
}#[cfg(test)]
mod tests_llm_16_515_llm_16_515 {
    use crate::{
        error::{Error, ErrorKind},
        number::streaming::be_i32,
        Err, Needed,
    };
    use std::num::NonZeroUsize;

    #[test]
    fn test_be_i32_complete() {
        let empty: &[u8] = &[];
        let incomplete: &[u8] = &[0x00, 0x01, 0x02];
        let complete: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let negative: &[u8] = &[0xff, 0xff, 0xff, 0xff];

        assert_eq!(be_i32::<_, Error<_>>(empty), Err(Err::Incomplete(Needed::new(4))));
        assert_eq!(be_i32::<_, Error<_>>(incomplete), Err(Err::Incomplete(Needed::new(4))));
        assert_eq!(be_i32::<_, Error<_>>(complete), Ok((&complete[4..], 0x00010203)));
        assert_eq!(be_i32::<_, Error<_>>(negative), Ok((&negative[4..], -1)));
    }

    #[test]
    fn test_be_i32_error() {
        let empty: &[u8] = &[];
        let incomplete: &[u8] = &[0x00, 0x01, 0x02];
        let error = be_i32::<_, Error<_>>(empty);
        let incomplete_error = be_i32::<_, Error<_>>(incomplete);

        assert!(matches!(error, Err(Err::Incomplete(Needed::Size(size))) if size == NonZeroUsize::new(4).unwrap()));
        assert!(matches!(incomplete_error, Err(Err::Incomplete(Needed::Size(size))) if size == NonZeroUsize::new(4).unwrap()));
    }
}#[cfg(test)]
mod tests_llm_16_516 {
    use crate::{
        Err,
        Needed,
        error::{Error, ErrorKind},
        number::streaming::be_i64,
    };

    #[test]
    fn test_be_i64_complete() {
        let parser = be_i64::<_, Error<&[u8]>>;
        assert_eq!(parser(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07][..]), Ok((&[][..], 0x0001020304050607)));
        assert_eq!(parser(&[0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88][..]), Ok((&[][..], -0x112233445566778)));
    }

    #[test]
    fn test_be_i64_incomplete() {
        let parser = be_i64::<_, Error<&[u8]>>;
        let rem = &[0x01][..];
        assert_eq!(parser(rem), Err(Err::Incomplete(Needed::new(7))));
    }

    #[test]
    fn test_be_i64_error() {
        let parser = be_i64::<_, Error<&[u8]>>;
        let rem = &[0x00, 0x01, 0x02, 0x03][..];
        assert_eq!(parser(rem), Err(Err::Incomplete(Needed::new(4))));
    }

    #[test]
    fn test_be_i64_remaining() {
        let parser = be_i64::<_, Error<&[u8]>>;
        let rem = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09][..];
        assert_eq!(parser(rem), Ok((&[0x08, 0x09][..], 0x0001020304050607)));
    }
}#[cfg(test)]
mod tests_llm_16_517 {
    use super::*;

use crate::*;
    use crate::{error::ErrorKind, number::streaming::be_i8, Err, Needed};

    #[test]
    fn test_be_i8_successful() {
        let data = &b"\x02abc"[..];
        let result = be_i8::<_, (&[u8], ErrorKind)>(data);
        assert_eq!(result, Ok((&b"abc"[..], 0x02)));
    }

    #[test]
    fn test_be_i8_incomplete() {
        let data = &b""[..];
        let result = be_i8::<_, (&[u8], ErrorKind)>(data);
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_be_i8_negative() {
        let data = &b"\xFFrest"[..];
        let result = be_i8::<_, (&[u8], ErrorKind)>(data);
        assert_eq!(result, Ok((&b"rest"[..], -1)));
    }

    #[test]
    fn test_be_i8_at_eof() {
        let data = &b"\x10"[..];
        let result = be_i8::<_, (&[u8], ErrorKind)>(data);
        assert_eq!(result, Ok((&b""[..], 0x10)));
    }

    #[test]
    fn test_be_i8_not_enough_data() {
        let data = &b""[..];
        let result = be_i8::<_, (&[u8], ErrorKind)>(data);
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }
}#[cfg(test)]
mod tests_llm_16_518 {
  use super::*;

use crate::*;
  use crate::{
    error::{Error, ErrorKind},
    Err, IResult, Needed,
  };

  #[test]
  fn test_be_u128() {
    let parse_be_u128 = |s| be_u128::<_, Error<&[u8]>>(s);

    assert_eq!(
      parse_be_u128(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F][..]),
      Ok((&[][..], 0x000102030405060708090A0B0C0D0E0F))
    );

    assert_eq!(
      parse_be_u128(&[0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00][..]),
      Ok((&[][..], 0xFFEEDDCCBBAA99887766554433221100))
    );

    assert_eq!(
      parse_be_u128(&[0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00, 0x01, 0x23, 0x45, 0x67, 0x89, 0x10, 0x11, 0x12, 0x13][..]),
      Ok((&[0x13][..], 0x12345678ABCDEF000123456789101112))
    );

    assert_eq!(
      parse_be_u128(&[0x01][..]),
      Err(Err::Incomplete(Needed::new(15)))
    );

    assert_eq!(
      parse_be_u128(&[]),
      Err(Err::Incomplete(Needed::new(16)))
    );
  }
}#[cfg(test)]
mod tests_llm_16_522_llm_16_522 {
    use crate::{Err, Needed, error::ErrorKind};
    use crate::number::streaming::be_u64;

    #[test]
    fn test_be_u64() {
        assert_eq!(be_u64::<_, (&[u8], ErrorKind)>(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]), Ok((&[][..], 0x0001020304050607)));
        assert_eq!(be_u64::<_, (&[u8], ErrorKind)>(&[0x01]), Err(Err::Incomplete(Needed::new(7))));
        
        let input = &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x00, 0x01, 0x02, 0x03];
        let (remaining, value) = be_u64::<_, (&[u8], ErrorKind)>(input).expect("Failed to parse be_u64");
        assert_eq!(remaining, &[0x00, 0x01, 0x02, 0x03]);
        assert_eq!(value, 0x123456789ABCDEF0);

        let incomplete_input = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        assert!(be_u64::<_, (&[u8], ErrorKind)>(incomplete_input).is_err());
    }
}#[cfg(test)]
mod tests_llm_16_526_llm_16_526 {
    use super::*;

use crate::*;
    use crate::*;
    use crate::{
        error::{ErrorKind, ParseError},
        number::Endianness,
        IResult, Needed,
    };
    use std::num::NonZeroUsize;

    fn needed(n: usize) -> Needed {
        Needed::Size(NonZeroUsize::new(n).unwrap())
    }

    #[test]
    fn test_f32_big_endian() {
        let be_f32 = |s| {
            f32::<_, (_, ErrorKind)>(Endianness::Big)(s)
        };

        assert_eq!(be_f32(&[0x41, 0x48, 0x00, 0x00][..]), Ok((&[] as &[u8], 12.5)));
        assert_eq!(be_f32(&[0x41, 0x48, 0x00][..]), Err(crate::Err::Incomplete(needed(1))));
        assert_eq!(be_f32(&[0x41, 0x48][..]), Err(crate::Err::Incomplete(needed(2))));
        assert_eq!(be_f32(&[0x41][..]), Err(crate::Err::Incomplete(needed(3))));
        assert_eq!(be_f32(&[][..]), Err(crate::Err::Incomplete(needed(4))));
    }

    #[test]
    fn test_f32_little_endian() {
        let le_f32 = |s| {
            f32::<_, (_, ErrorKind)>(Endianness::Little)(s)
        };

        assert_eq!(le_f32(&[0x00, 0x00, 0x48, 0x41][..]), Ok((&[] as &[u8], 12.5)));
        assert_eq!(le_f32(&[0x00, 0x00, 0x48][..]), Err(crate::Err::Incomplete(needed(1))));
        assert_eq!(le_f32(&[0x00, 0x00][..]), Err(crate::Err::Incomplete(needed(2))));
        assert_eq!(le_f32(&[0x00][..]), Err(crate::Err::Incomplete(needed(3))));
        assert_eq!(le_f32(&[][..]), Err(crate::Err::Incomplete(needed(4))));
    }
}#[cfg(test)]
mod tests_llm_16_527 {
    use crate::{
        Err,
        error::{ErrorKind, ParseError},
        Needed,
        number::streaming::f64 as nom_f64,
        number::Endianness,
    };

    #[test]
    fn test_f64() {
        let be_f64 = |s| {
            nom_f64::<_, (_, ErrorKind)>(Endianness::Big)(s)
        };

        assert_eq!(
            be_f64(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
            Ok((&b""[..], 12.5))
        );
        assert_eq!(
            be_f64(&b"abc"[..]),
            Err(Err::Incomplete(Needed::new(5)))
        );

        let le_f64 = |s| {
            nom_f64::<_, (_, ErrorKind)>(Endianness::Little)(s)
        };

        assert_eq!(
            le_f64(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..]),
            Ok((&b""[..], 12.5))
        );
        assert_eq!(
            le_f64(&b"abc"[..]),
            Err(Err::Incomplete(Needed::new(5)))
        );
    }
}#[cfg(test)]
mod tests_llm_16_528 {
    use crate::{
        error::{ErrorKind, ParseError},
        number::streaming::float,
        Err,
    };

    #[test]
    fn test_float() {
        let successful_tests = vec![
            ("11e-1", 1.1f32),
            ("123E-02", 1.23f32),
            ("+123.456", 123.456f32),
            ("0.0", 0.0f32),
            ("-0.0", -0.0f32),
            ("-123.456", -123.456f32),
        ];

        let incomplete_tests = vec![
            ("", 0.0f32),
            ("-", 0.0f32),
            ("+", 0.0f32),
            ("123e", 0.0f32),
        ];

        let error_tests = vec![
            ("abc", "abc"),
            ("123K-01", "K-01"),
            ("123.45.6", ".45.6"),
        ];

        for (input, output) in successful_tests {
            let result = float::<&str, crate::error::Error<&str>>(input);
            assert_eq!(result, Ok(("", output)));
        }

        for (input, _output) in incomplete_tests {
            let result = float::<&str, crate::error::Error<&str>>(input);
            assert!(matches!(result, Err(Err::Incomplete(_))));
        }

        for (input, remaining) in error_tests {
            let result = float::<&str, crate::error::Error<&str>>(input);
            assert!(matches!(result, Err(Err::Error(err)) if err.input == remaining && err.code == ErrorKind::Float));
        }
    }
}#[cfg(test)]
mod tests_llm_16_529 {
    use crate::{
        error::{Error, ErrorKind},
        number::streaming::hex_u32,
        Err, IResult, Needed,
    };

    fn parse_hex_u32(input: &[u8]) -> IResult<&[u8], u32, Error<&[u8]>> {
        hex_u32(input)
    }

    #[test]
    fn test_hex_u32_complete() {
        assert_eq!(parse_hex_u32(b"01AE;"), Ok((&b";"[..], 0x01AE)));
    }

    #[test]
    fn test_hex_u32_incomplete() {
        assert_eq!(
            parse_hex_u32(&b"abc"[..]),
            Err(Err::Incomplete(Needed::new(1)))
        );
    }

    #[test]
    fn test_hex_u32_non_hex() {
        assert_eq!(
            parse_hex_u32(&b"ggg"[..]),
            Err(Err::Error(Error::new(&b"ggg"[..], ErrorKind::IsA)))
        );
    }

    #[test]
    fn test_hex_u32_too_long() {
        // The test value is more than 8 characters, so only the first 8 are considered
        assert_eq!(parse_hex_u32(b"123456789"), Ok((&b"9"[..], 0x12345678)));
    }

    #[test]
    fn test_hex_u32_empty() {
        assert_eq!(
            parse_hex_u32(b""),
            Err(Err::Incomplete(Needed::new(1)))
        );
    }

    #[test]
    fn test_hex_u32_only_semicolon() {
        assert_eq!(
            parse_hex_u32(&b";"[..]),
            Err(Err::Error(Error::new(&b";"[..], ErrorKind::IsA)))
        );
    }

    #[test]
    fn test_hex_u32_no_semicolon() {
        // Input without a semicolon (;) at the end
        assert_eq!(parse_hex_u32(&b"01AE"[..]), Ok((&b""[..], 0x01AE)));
    }
}#[cfg(test)]
mod tests_llm_16_531 {
    use crate::{Err, Needed};
    use crate::number::streaming::i16;
    use crate::number::Endianness;
    use crate::error::ErrorKind;

    #[test]
    fn test_i16_be() {
        let be_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Big);
        assert_eq!(be_i16_parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0003)));
        assert_eq!(be_i16_parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_i16_le() {
        let le_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Little);
        assert_eq!(le_i16_parser(&b"\x00\x03abcefg"[..]), Ok((&b"abcefg"[..], 0x0300)));
        assert_eq!(le_i16_parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_i16_invalid_length() {
        let be_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Big);
        assert_eq!(be_i16_parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_i16_negative_number() {
        let be_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Big);
        assert_eq!(be_i16_parser(&b"\xff\xfd"[..]), Ok((&b""[..], -3)));
        let le_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Little);
        assert_eq!(le_i16_parser(&b"\xfd\xff"[..]), Ok((&b""[..], -3)));
    }

    #[test]
    fn test_i16_empty() {
        let be_i16_parser = i16::<_, (_, ErrorKind)>(Endianness::Big);
        assert_eq!(be_i16_parser(&b""[..]), Err(Err::Incomplete(Needed::new(2))));
    }
}#[cfg(test)]
mod tests_llm_16_537_llm_16_537 {
    use crate::{
        number::streaming::le_f64,
        error::{Error, ErrorKind, ParseError},
        IResult, Err, Needed,
    };

    #[test]
    fn le_f64_incomplete() {
        let bytes = &[0x00];
        let res = le_f64::<_, Error<&[u8]>>(bytes);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(7))));
    }

    #[test]
    fn le_f64_complete() {
        let bytes = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f];
        let res = le_f64::<_, Error<&[u8]>>(bytes);
        assert_eq!(res, Ok((&bytes[8..], 1.0)));
    }

    #[test]
    fn le_f64_error() {
        let bytes = &[0x00, 0x00, 0x00, 0x00];
        let res = le_f64::<_, Error<&[u8]>>(bytes);
        assert!(matches!(
            res,
            Err(Err::Error(Error {
                input: i,
                code: ErrorKind::Eof,
            })) if i == bytes
        ));
    }
}#[cfg(test)]
mod tests_llm_16_538_llm_16_538 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        number::streaming::le_i128,
        Err, Needed,
    };

    #[test]
    fn test_le_i128_complete() {
        let parser = |s| {
            le_i128::<_, Error<&[u8]>>(s)
        };

        let input = &b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..];
        assert_eq!(
            parser(input),
            Ok((&b"abcd"[..], 0x15141312111009080706050403020100i128))
        );
    }

    #[test]
    fn test_le_i128_incomplete() {
        let parser = |s| {
            le_i128::<_, Error<&[u8]>>(s)
        };

        let input = &b"\x01"[..];
        assert_eq!(
            parser(input),
            Err(Err::Incomplete(Needed::new(15)))
        );
    }

    #[test]
    fn test_le_i128_incomplete_with_error() {
        let parser = |s| {
            le_i128::<_, Error<&[u8]>>(s)
        };

        let input = &b"\x01\x02\x03"[..];
        assert_eq!(
            parser(input),
            Err(Err::Incomplete(Needed::new(12)))
        );
    }

    #[test]
    fn test_le_i128_error() {
        let parser = |s| {
            le_i128::<_, Error<&[u8]>>(s)
        };

        let input = &b""[..]; // Empty input
        assert_eq!(
            parser(input),
            Err(Err::Incomplete(Needed::new(16)))
        );
    }
}#[cfg(test)]
mod tests_llm_16_539 {
  use super::*;

use crate::*;
  use crate::{Err, error::{ErrorKind, Error}, Needed};

  #[test]
  fn test_le_i16_complete() {
    let data_complete = &[0xFF, 0xFF];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_complete), Ok((&[][..], -1)));
  }

  #[test]
  fn test_le_i16_incomplete() {
    let data_incomplete = &[0xFF];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_incomplete), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn test_le_i16_with_following_data() {
    let data_with_following = &[0x34, 0x12, 0x00, 0x00];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_with_following), Ok((&[0x00, 0x00][..], 0x1234)));
  }

  #[test]
  fn test_le_i16_zero() {
    let data_zero = &[0x00, 0x00];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_zero), Ok((&[][..], 0)));
  }

  #[test]
  fn test_le_i16_positive() {
    let data_positive = &[0x70, 0x00];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_positive), Ok((&[][..], 0x0070)));
  }

  #[test]
  fn test_le_i16_negative() {
    let data_negative = &[0x00, 0xFF];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_negative), Ok((&[][..], -256)));
  }

  #[test]
  fn test_le_i16_max() {
    let data_max = &[0xFF, 0x7F];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_max), Ok((&[][..], 0x7FFF)));
  }

  #[test]
  fn test_le_i16_min() {
    let data_min = &[0x00, 0x80];
    assert_eq!(le_i16::<_, Error<&[u8]>>(data_min), Ok((&[][..], -32768)));
  }
}#[cfg(test)]
mod tests_llm_16_540 {
  use super::*;

use crate::*;
  use crate::{
    error::{ErrorKind, ParseError},
    Err, IResult, Needed,
  };

  fn parse_le_i24(input: &[u8]) -> IResult<&[u8], i32, crate::error::Error<&[u8]>> {
    le_i24(input)
  }

  #[test]
  fn test_le_i24() {
    assert_eq!(parse_le_i24(&[0x00, 0x01, 0x02]), Ok((&[][..], 0x020100)));
    assert_eq!(parse_le_i24(&[0xFF, 0xFF, 0xFF]), Ok((&[][..], -1)));
    assert_eq!(parse_le_i24(&[0x80, 0x00, 0x00]), Ok((&[][..], -0x800000)));
    assert_eq!(parse_le_i24(&[0x7F, 0xFF, 0xFF]), Ok((&[][..], 0x7FFFFF)));
    assert_eq!(parse_le_i24(&[0x00]), Err(Err::Incomplete(Needed::new(2))));
    assert_eq!(parse_le_i24(&[0x00, 0x01]), Err(Err::Incomplete(Needed::new(1))));
    assert_eq!(parse_le_i24(&[]), Err(Err::Incomplete(Needed::new(3))));
  }
}#[cfg(test)]
mod tests_llm_16_542 {
    use crate::number::streaming::le_i64;
    use crate::{
        error::{Error, ErrorKind, ParseError},
        Err, IResult, Needed,
    };

    #[test]
    fn test_le_i64_complete() {
        let data = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let result: IResult<&[u8], i64> = le_i64(data);
        assert_eq!(result, Ok((&data[8..], 0x0706050403020100)));
    }

    #[test]
    fn test_le_i64_incomplete() {
        let data = &[0x00];
        let result: IResult<&[u8], i64> = le_i64(data);
        assert_eq!(result, Err(Err::Incomplete(Needed::new(7))));
    }

    #[test]
    fn test_le_i64_negative() {
        let data = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let result: IResult<&[u8], i64> = le_i64(data);
        assert_eq!(result, Ok((&data[8..], -1)));
    }

    #[test]
    fn test_le_i64_overflow() {
        let data = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let result: IResult<&[u8], i64> = le_i64(data);
        assert_eq!(result, Err(Err::Incomplete(Needed::new(1))));
    }
}#[cfg(test)]
mod tests_llm_16_543 {
  use crate::{
    error::{Error, ErrorKind},
    number::streaming::le_i8,
    Err::Incomplete,
    IResult,
    Needed,
  };

  #[test]
  fn test_le_i8_success() {
    let input = &[0x02, 0xFF, 0x7F, 0x80][..];
    let expected = Ok((&[0xFF, 0x7F, 0x80][..], 0x02 as i8));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_le_i8_incomplete() {
    let input = &[0x7F][..];
    let expected = Ok((&[][..], 0x7F as i8));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_le_i8_negative() {
    let input = &[0xFF][..];
    let expected = Ok((&[][..], -1i8));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_le_i8_incomplete_zero() {
    let input = &[][..];
    let expected = Err(Incomplete(Needed::new(1)));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_le_i8_zero() {
    let input = &[0x00][..];
    let expected = Ok((&[][..], 0x00 as i8));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_le_i8_boundary() {
    let input = &[0x80, 0x7F][..];
    let expected = Ok((&[0x7F][..], -128i8));
    let result = le_i8::<_, Error<&[u8]>>(input);
    assert_eq!(result, expected);
  }
}#[cfg(test)]
mod tests_llm_16_544 {
  use crate::{Err, Needed};
  use crate::number::streaming::le_u128;
  use crate::error::{ErrorKind, ParseError};
    
  #[test]
  fn test_le_u128() {
    fn test_parser(input: &[u8]) -> crate::IResult<&[u8], u128, crate::error::Error<&[u8]>> {
      le_u128(input)
    }

    let full_input = &b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15abcd"[..];
    let incomplete_input = &b"\x01"[..];
    let expected_value: u128 = 0x15141312111009080706050403020100;

    // Test with complete input
    assert_eq!(
      test_parser(full_input),
      Ok((&b"abcd"[..], expected_value))
    );
    
    // Test with incomplete input
    assert_eq!(
      test_parser(incomplete_input),
      Err(Err::Incomplete(Needed::new(15)))
    );
  }
}#[cfg(test)]
mod tests_llm_16_545 {
    use crate::{Err, Needed, error::ErrorKind, number::streaming::le_u16};

    #[test]
    fn test_le_u16() {
        let incomplete_input = &b"\x01"[..];
        let valid_input = &b"\x00\x01abcd"[..];
        let expected_remainder = &b"abcd"[..];
        let expected_value = 0x0100;

        assert_eq!(le_u16::<_, crate::error::Error<&[u8]>>(incomplete_input), Err(Err::Incomplete(Needed::new(1))));
        assert_eq!(le_u16::<_, crate::error::Error<&[u8]>>(valid_input), Ok((expected_remainder, expected_value)));
    }
}#[cfg(test)]
mod tests_llm_16_546 {
    use crate::{
        error::{Error, ErrorKind},
        number::streaming::le_u24,
        Err, IResult, Needed,
    };

    #[test]
    fn test_le_u24() {
        fn parse_le_u24(input: &[u8]) -> IResult<&[u8], u32, Error<&[u8]>> {
            le_u24(input)
        }

        let res1 = parse_le_u24(&b"\x00\x01\x02abcd"[..]);
        assert_eq!(res1, Ok((&b"abcd"[..], 0x020100)));

        let res2 = parse_le_u24(&b"\x01"[..]);
        assert_eq!(res2, Err(Err::Incomplete(Needed::new(2))));

        let res3 = parse_le_u24(&b"\x00\x01"[..]);
        assert_eq!(res3, Err(Err::Incomplete(Needed::new(1))));

        let res4 = parse_le_u24(&b"\x00\x01\x02"[..]);
        assert_eq!(res4, Ok((&b""[..], 0x020100)));

        let res5 = parse_le_u24(&b"\x78\x56\x34\x12"[..]);
        assert_eq!(res5, Ok((&b"\x12"[..], 0x345678)));

        let res6 = parse_le_u24(&b""[..]);
        assert_eq!(res6, Err(Err::Incomplete(Needed::new(3))));

        let res7 = parse_le_u24(&b"\xFF\xFF\xFF"[..]);
        assert_eq!(res7, Ok((&b""[..], 0xFFFFFF)));

        let res8 = parse_le_u24(&b"\xFF\xFF"[..]);
        assert_eq!(res8, Err(Err::Incomplete(Needed::new(1))));
    }
}#[cfg(test)]
mod tests_llm_16_547 {
    use crate::{
        error::{ErrorKind, ParseError},
        number::streaming::le_u32,
        Err, IResult, Needed,
    };

    #[test]
    fn test_le_u32_complete() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let full_input = &b"\x78\x56\x34\x12"[..];
        let expected_output = (&b""[..], 0x12345678u32);
        assert_eq!(parser(full_input), Ok(expected_output));
    }

    #[test]
    fn test_le_u32_partial() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let partial_input = &b"\x56\x34\x12"[..];
        assert_eq!(
            parser(partial_input),
            Err(Err::Incomplete(Needed::new(1)))
        );
    }

    #[test]
    fn test_le_u32_incomplete() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let incomplete_input = &b"\x34\x12"[..];
        assert_eq!(
            parser(incomplete_input),
            Err(Err::Incomplete(Needed::new(2)))
        );
    }

    #[test]
    fn test_le_u32_incomplete_one_byte() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let incomplete_input_one_byte = &b"\x12"[..];
        assert_eq!(
            parser(incomplete_input_one_byte),
            Err(Err::Incomplete(Needed::new(3)))
        );
    }

    #[test]
    fn test_le_u32_empty_input() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let empty_input = &b""[..];
        assert_eq!(parser(empty_input), Err(Err::Incomplete(Needed::new(4))));
    }

    #[test]
    fn test_le_u32_additional_data() {
        fn parser(s: &[u8]) -> IResult<&[u8], u32, crate::error::Error<&[u8]>> {
            le_u32(s)
        }

        let additional_data_input = &b"\x78\x56\x34\x12extra"[..];
        let expected_output = (&b"extra"[..], 0x12345678u32);
        assert_eq!(parser(additional_data_input), Ok(expected_output));
    }
}#[cfg(test)]
mod tests_llm_16_548 {
    use super::*;

use crate::*;
    use crate::{Err, Needed, error::ErrorKind};

    #[test]
    fn test_le_u64_complete() {
        let parser = |s| {
            le_u64::<_, (_, ErrorKind)>(s)
        };

        // complete buffer
        assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07abcd"[..]), Ok((&b"abcd"[..], 0x0706050403020100)));

        // buffer too small
        assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(7))));

        // exactly 8 bytes
        assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07"[..]), Ok((&b""[..], 0x0706050403020100)));

        // buffer too big
        assert_eq!(parser(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x08"[..]), Ok((&b"\x08"[..], 0x0706050403020100)));
    }

    #[test]
    #[should_panic]
    fn test_le_u64_incomplete() {
        let parser = |s| {
            le_u64::<_, (_, ErrorKind)>(s)
        };

        // incorrect buffer length that will panic due to too few bytes
        // a parser would normally need to handle this without panic, but this
        // is an explicit test to demonstrate what happens on incorrect input length
        let _ = parser(&b"\x00"[..]);
    }
}#[cfg(test)]
mod tests_llm_16_549_llm_16_549 {
    use crate::number::streaming::le_u8; // Correct the import path
    use crate::{
        error::{ErrorKind, ParseError},
        Err, IResult, Needed,
    };

    // Helper function to generate test input
    fn input_with_size(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    #[test]
    fn test_le_u8_complete() {
        let data = input_with_size(1);
        let res: IResult<&[u8], u8, crate::error::Error<&[u8]>> = le_u8(&data);
        assert_eq!(res, Ok((&b""[..], 0)));
    }

    #[test]
    fn test_le_u8_incomplete() {
        let data = input_with_size(0);
        let res: IResult<&[u8], u8, crate::error::Error<&[u8]>> = le_u8(&data[..]);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn test_le_u8_streaming() {
        let data = input_with_size(2);
        // Fix the reference to data to ensure it's a slice
        let res: IResult<&[u8], u8, crate::error::Error<&[u8]>> = le_u8(&data[..]);
        assert_eq!(res, Ok((&data[1..], 0)));
    }

    #[test]
    fn test_le_u8_overflow() {
        let data = input_with_size(256);
        // Fix the reference to data to ensure it's a slice
        let res: IResult<&[u8], u8, crate::error::Error<&[u8]>> = le_u8(&data[..]);
        assert_eq!(res, Ok((&data[1..], 0)));
    }

    #[test]
    fn test_le_u8_custom_error() {
        #[derive(Debug, Clone)]
        struct CustomError<'a>(&'a [u8], ErrorKind);

        impl<'a> ParseError<&'a [u8]> for CustomError<'a> {
            fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
                CustomError(input, kind)
            }

            fn append(_: &'a [u8], _: ErrorKind, other: Self) -> Self {
                other
            }
        }

        // Implement PartialEq to use assert_eq! for the error
        impl<'a> PartialEq for CustomError<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.1 == other.1
            }
        }

        let data = input_with_size(0);
        // Fix the reference to data to ensure it's a slice
        let res: IResult<&[u8], u8, CustomError> = le_u8(&data[..]);
        assert!(res.is_err());
    }
}#[cfg(test)]
mod tests_llm_16_551_llm_16_551 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::number::streaming::recognize_float;

    #[test]
    fn recognize_valid_floats() {
        let tests = vec![
            ("11e-1;", "11e-1"),
            ("123E-02;", "123E-02"),
            ("123.", "123."),
            ("0.0;", "0.0"),
            ("-.5;", "-.5"),
            ("0.12345e+02;", "0.12345e+02"),
        ];

        for (input, expected) in tests {
            assert_eq!(recognize_float::<_, Error<_>>(input), Ok((";", expected)));
        }
    }

    #[test]
    fn recognize_incomplete_floats() {
        let tests = vec![
            "11e-",
            "123E",
            "123E-",
            "0.",
            "-.",
            ".",
            ".e-1",
        ];

        for input in tests {
            let res = recognize_float::<_, Error<_>>(input);
            assert!(matches!(res,
                Err(Err::Error(Error {
                    input: i,
                    code: ErrorKind::Char
                })) if i == input
            ) || matches!(res, Err(Err::Incomplete(Needed::Size(_)))));
        }
    }

    #[test]
    fn recognize_with_trailing_non_digit() {
        assert_eq!(
            recognize_float::<_, Error<_>>("123K-01"),
            Ok(("K-01", "123"))
        );
    }

    #[test]
    fn recognize_invalid_floats() {
        let tests = vec![
            ("abc", ErrorKind::Char),
            ("", ErrorKind::Char),
            ("--12", ErrorKind::Char),
            ("E-12", ErrorKind::Char),
        ];

        for (input, error_kind) in tests {
            assert_eq!(
                recognize_float::<_, Error<_>>(input),
                Err(Err::Error(Error::new(input, error_kind)))
            );
        }
    }
}#[cfg(test)]
mod tests_llm_16_554_llm_16_554 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        number::streaming::u128 as parse_u128,
        number::Endianness,
        Err, IResult, Needed,
    };

    fn parse_be_u128(input: &[u8]) -> IResult<&[u8], u128, Error<&[u8]>> {
        parse_u128(Endianness::Big)(input)
    }

    fn parse_le_u128(input: &[u8]) -> IResult<&[u8], u128, Error<&[u8]>> {
        parse_u128(Endianness::Little)(input)
    }

    #[test]
    fn test_u128_be_parser() {
        assert_eq!(
            parse_be_u128(&b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07abcefg"[..]),
            Ok((&b"abcefg"[..], 0x00010203040506070001020304050607u128))
        );
        assert_eq!(
            parse_be_u128(&b"\x01"[..]),
            Err(Err::Incomplete(Needed::new(15)))
        );
    }

    #[test]
    fn test_u128_le_parser() {
        assert_eq!(
            parse_le_u128(&b"\x07\x06\x05\x04\x03\x02\x01\x00\x07\x06\x05\x04\x03\x02\x01\x00abcefg"[..]),
            Ok((&b"abcefg"[..], 0x00010203040506070001020304050607u128))
        );
        assert_eq!(
            parse_le_u128(&b"\x01"[..]),
            Err(Err::Incomplete(Needed::new(15)))
        );
    }
}#[cfg(test)]
mod tests_llm_16_555 {
    use crate::{IResult, Err, error::ErrorKind, Needed};
    use crate::number::streaming::u16;
    use crate::number::Endianness;
    use crate::error::Error;
    
    #[test]
    fn u16_big_endian_test() {
        fn be_u16(i: &[u8]) -> IResult<&[u8], u16, Error<&[u8]>> {
            u16::<_, Error<&[u8]>>(Endianness::Big)(i)
        }
        let be_tests: Vec<(&[u8], IResult<&[u8], u16, Error<&[u8]>>)> = vec![
            (&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'], Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0003))),
            (&[0x01], Err(Err::Incomplete(Needed::new(1)))),
        ];
        for (input, expected) in be_tests {
            assert_eq!(be_u16(input), expected);
        }
    }
    
    #[test]
    fn u16_little_endian_test() {
        fn le_u16(i: &[u8]) -> IResult<&[u8], u16, Error<&[u8]>> {
            u16::<_, Error<&[u8]>>(Endianness::Little)(i)
        }
        let le_tests: Vec<(&[u8], IResult<&[u8], u16, Error<&[u8]>>)> = vec![
            (&[0x00, 0x03, b'a', b'b', b'c', b'e', b'f', b'g'], Ok((&[b'a', b'b', b'c', b'e', b'f', b'g'][..], 0x0300))),
            (&[0x01], Err(Err::Incomplete(Needed::new(1)))),
        ];
        for (input, expected) in le_tests {
            assert_eq!(le_u16(input), expected);
        }
    }
}#[cfg(test)]
mod tests_llm_16_556 {
  use crate::{
    error::{Error, ErrorKind},
    number::streaming::u24,
    number::Endianness,
    Err,
    Needed,
  };

  #[test]
  fn test_u24_big_endian() {
    let parser = |s| u24::<_, (_, ErrorKind)>(Endianness::Big)(s);
    assert_eq!(parser(&b"\x00\x03\x05abc"[..]), Ok((&b"abc"[..], 0x000305)));
    assert_eq!(parser(&b"\x01\x00\xFFabc"[..]), Ok((&b"abc"[..], 0x0100FF)));
    assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
  }

  #[test]
  fn test_u24_little_endian() {
    let parser = |s| u24::<_, (_, ErrorKind)>(Endianness::Little)(s);
    assert_eq!(parser(&b"\x00\x03\x05abc"[..]), Ok((&b"abc"[..], 0x050300)));
    assert_eq!(parser(&b"\x01\x00\xFFabc"[..]), Ok((&b"abc"[..], 0xFF0001)));
    assert_eq!(parser(&b"\x01"[..]), Err(Err::Incomplete(Needed::new(2))));
  }

  #[test]
  fn test_u24_incomplete() {
    let parser_be = |s| u24::<_, (_, ErrorKind)>(Endianness::Big)(s);
    let parser_le = |s| u24::<_, (_, ErrorKind)>(Endianness::Little)(s);
    let input = &b"\x01\x02"[..];
    assert_eq!(parser_be(input), Err(Err::Incomplete(Needed::new(1))));
    assert_eq!(parser_le(input), Err(Err::Incomplete(Needed::new(1))));
  }

  #[test]
  fn test_u24_error() {
    let parser_be = |s| u24::<_, Error<&[u8]>>(Endianness::Big)(s);
    let parser_le = |s| u24::<_, Error<&[u8]>>(Endianness::Little)(s);
    let input = &b"\x01\x02\x03"[..];
    let error_kind = ErrorKind::Tag;
    assert_eq!(parser_be(input), Ok((&b""[..], 0x010203)));
    assert_eq!(parser_le(input), Ok((&b""[..], 0x030201)));
    assert!(matches!(parser_be(&b""[..]), Err(Err::Incomplete(_))));
    assert!(matches!(parser_le(&b""[..]), Err(Err::Incomplete(_))));
  }
}#[cfg(test)]
mod tests_llm_16_557_llm_16_557 {
    use crate::number::streaming::u32;
    use crate::number::Endianness;
    use crate::error::{Error, ErrorKind};
    use crate::IResult;
    use crate::Err;
    use crate::Needed;
    use crate::AsBytes;

    #[test]
    fn test_u32_big_endian_success() {
        let parser = u32::<_, Error<&[u8]>>(Endianness::Big);
        let input = b"\x00\x03\x05\x07abcefg";
        let expected = Ok((&b"abcefg"[..], 0x00030507_u32));
        assert_eq!(parser(input.as_bytes()), expected);
    }

    #[test]
    fn test_u32_big_endian_incomplete() {
        let parser = u32::<_, Error<&[u8]>>(Endianness::Big);
        let input = b"\x01";
        let expected = Err(Err::Incomplete(Needed::new(3)));
        assert_eq!(parser(input.as_bytes()), expected);
    }

    #[test]
    fn test_u32_little_endian_success() {
        let parser = u32::<_, Error<&[u8]>>(Endianness::Little);
        let input = b"\x00\x03\x05\x07abcefg";
        let expected = Ok((&b"abcefg"[..], 0x07050300_u32));
        assert_eq!(parser(input.as_bytes()), expected);
    }

    #[test]
    fn test_u32_little_endian_incomplete() {
        let parser = u32::<_, Error<&[u8]>>(Endianness::Little);
        let input = b"\x01";
        let expected = Err(Err::Incomplete(Needed::new(3)));
        assert_eq!(parser(input.as_bytes()), expected);
    }
}