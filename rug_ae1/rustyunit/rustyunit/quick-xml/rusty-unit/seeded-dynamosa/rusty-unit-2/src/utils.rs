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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_562() {
//    rusty_monitor::set_test_id(562);
    let mut str_0: &str = "UnrecognizedSymbol";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "WRNkq6E7ncT";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 42u8;
    let mut u8_3: u8 = 108u8;
    let mut u8_4: u8 = 91u8;
    let mut u8_5: u8 = 1u8;
    let mut u8_6: u8 = 107u8;
    let mut u8_7: u8 = 48u8;
    let mut u8_8: u8 = 68u8;
    let mut u8_9: u8 = 100u8;
    let mut u8_10: u8 = 33u8;
    let mut u8_11: u8 = 32u8;
    let mut u8_12: u8 = 100u8;
    let mut u8_13: u8 = 49u8;
    let mut u8_14: u8 = 5u8;
    let mut u8_15: u8 = 58u8;
    let mut bool_0: bool = crate::reader::is_whitespace(u8_6);
    let mut bool_1: bool = crate::reader::is_whitespace(u8_4);
    let mut bool_2: bool = crate::reader::is_whitespace(u8_5);
    let mut bool_3: bool = crate::reader::is_whitespace(u8_3);
    let mut bool_4: bool = crate::reader::is_whitespace(u8_12);
    let mut bool_5: bool = crate::reader::is_whitespace(u8_14);
    let mut bool_6: bool = crate::reader::is_whitespace(u8_11);
    let mut bool_7: bool = crate::reader::is_whitespace(u8_2);
    let mut bool_8: bool = crate::reader::is_whitespace(u8_10);
    let mut bool_9: bool = crate::reader::is_whitespace(u8_13);
    let mut bool_10: bool = crate::reader::is_whitespace(u8_9);
    let mut bool_11: bool = crate::reader::is_whitespace(u8_7);
    let mut bool_12: bool = crate::reader::is_whitespace(u8_8);
    let mut bool_13: bool = crate::reader::is_whitespace(u8_15);
    let mut bool_14: bool = crate::reader::is_whitespace(u8_1);
    let mut bool_15: bool = crate::reader::is_whitespace(u8_0);
    let mut error_0: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_1, found: string_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_573() {
//    rusty_monitor::set_test_id(573);
    let mut str_0: &str = "NamespaceEntry";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "keys";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "5y1KuHNcl8mQRCSoR";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Decoder";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Done";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "TNyK9tFDpnZl06ac";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "DocType";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "6e";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "bm0JPC6F5Lq";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "PI";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "84CpUIgMuk4nWBjFhG";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_10_ref_0);
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_9_ref_0);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_8_ref_0);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_7_ref_0);
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytescdata_6: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytescdata_7: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytescdata_8: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytescdata_9: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytescdata_10: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}