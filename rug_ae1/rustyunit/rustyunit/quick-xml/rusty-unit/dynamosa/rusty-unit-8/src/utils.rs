use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};

#[cfg(feature = "serialize")]
use serde::de::{Deserialize, Deserializer, Error, Visitor};

pub fn write_cow_string(f: &mut Formatter, cow_string: &Cow<[u8]>) -> fmt::Result {
    match cow_string {
        Cow::Owned(s) => {
            write!(f, "Owned(")?;
            write_byte_string(f, &s)?;
        }
        Cow::Borrowed(s) => {
            write!(f, "Borrowed(")?;
            write_byte_string(f, s)?;
        }
    }
    write!(f, ")")
}

pub fn write_byte_string(f: &mut Formatter, byte_string: &[u8]) -> fmt::Result {
    write!(f, "\"")?;
    for b in byte_string {
        match *b {
            32..=33 | 35..=126 => write!(f, "{}", *b as char)?,
            34 => write!(f, "\\\"")?,
            _ => write!(f, "{:#02X}", b)?,
        }
    }
    write!(f, "\"")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Wrapper around `Vec<u8>` that has a human-readable debug representation:
/// printable ASCII symbols output as is, all other output in HEX notation.
///
/// Also, when `serialize` feature is on, this type deserialized using
/// [`deserialize_byte_buf`](serde::Deserializer::deserialize_byte_buf) instead
/// of vector's generic [`deserialize_seq`](serde::Deserializer::deserialize_seq)
#[derive(PartialEq)]
pub struct ByteBuf(pub Vec<u8>);

impl<'de> Debug for ByteBuf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_byte_string(f, &self.0)
    }
}

#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for ByteBuf {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = ByteBuf;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("byte data")
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(ByteBuf(v.to_vec()))
            }

            fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(ByteBuf(v))
            }
        }

        Ok(d.deserialize_byte_buf(ValueVisitor)?)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Wrapper around `&[u8]` that has a human-readable debug representation:
/// printable ASCII symbols output as is, all other output in HEX notation.
///
/// Also, when `serialize` feature is on, this type deserialized using
/// [`deserialize_bytes`](serde::Deserializer::deserialize_bytes) instead
/// of vector's generic [`deserialize_seq`](serde::Deserializer::deserialize_seq)
#[derive(PartialEq)]
pub struct Bytes<'de>(pub &'de [u8]);

impl<'de> Debug for Bytes<'de> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_byte_string(f, &self.0)
    }
}

#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for Bytes<'de> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Bytes<'de>;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("borrowed bytes")
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                Ok(Bytes(v))
            }
        }

        Ok(d.deserialize_bytes(ValueVisitor)?)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn write_byte_string0() {
        let bytes = ByteBuf(vec![10, 32, 32, 32, 32, 32, 32, 32, 32]);
        assert_eq!(format!("{:?}", bytes), "\"0xA        \"".to_owned());
    }

    #[test]
    fn write_byte_string1() {
        let bytes = ByteBuf(vec![
            104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50,
            48, 48, 50, 47, 48, 55, 47, 111, 119, 108, 35,
        ]);
        assert_eq!(
            format!("{:?}", bytes),
            r##""http://www.w3.org/2002/07/owl#""##.to_owned()
        );
    }

    #[test]
    fn write_byte_string3() {
        let bytes = ByteBuf(vec![
            67, 108, 97, 115, 115, 32, 73, 82, 73, 61, 34, 35, 66, 34,
        ]);
        assert_eq!(format!("{:?}", bytes), r##""Class IRI=\"#B\"""##.to_owned());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8631() {
    rusty_monitor::set_test_id(8631);
    let mut str_0: &str = "0nI7";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut str_1: &str = "9Km8dKE49BUmuhu8";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut u8_slice_0: &[u8] = std::option::Option::unwrap(option_0);
    let mut u8_slice_1: &[u8] = std::option::Option::unwrap(option_1);
    panic!("From RustyUnit with love");
}
}