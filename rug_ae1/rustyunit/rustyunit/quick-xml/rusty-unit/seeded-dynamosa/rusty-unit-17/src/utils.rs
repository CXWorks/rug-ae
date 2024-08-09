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
#[timeout(30000)]fn rusty_test_322() {
//    rusty_monitor::set_test_id(322);
    let mut isize_0: isize = 0isize;
    let mut isize_1: isize = 7isize;
    let mut isize_2: isize = 4isize;
    let mut isize_3: isize = -11658isize;
    let mut isize_4: isize = 4isize;
    let mut isize_5: isize = 3isize;
    let mut isize_6: isize = -9222isize;
    let mut isize_7: isize = 6800isize;
    let mut isize_8: isize = -5688isize;
    let mut isize_9: isize = -6436isize;
    let mut isize_10: isize = 3289isize;
    let mut isize_11: isize = 5isize;
    let mut isize_12: isize = 6isize;
    let mut isize_13: isize = 2isize;
    let mut isize_14: isize = 6isize;
    let mut isize_15: isize = -1757isize;
    let mut isize_16: isize = 7127isize;
    let mut isize_17: isize = 3isize;
    let mut isize_18: isize = 3707isize;
    let mut isize_19: isize = 8isize;
    let mut isize_20: isize = 2isize;
    let mut isize_21: isize = 7isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_21, isize_20);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_19, isize_18);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_17, isize_16);
    let mut attr_3: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_15, isize_14);
    let mut attr_4: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_13, isize_12);
    let mut attr_5: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_11, isize_10);
    let mut attr_6: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_9, isize_8);
    let mut attr_7: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_7, isize_6);
    let mut attr_8: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_5, isize_4);
    let mut attr_9: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_3, isize_2);
    let mut attr_10: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_173() {
//    rusty_monitor::set_test_id(173);
    let mut str_0: &str = "End";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "TooLongHexadecimal";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "BytesDecl";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Decl";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "DOCTYPE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "element";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2mLL3PxWl3ILKXcy";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "ExpectedQuote";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "Attributes";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "ExpectedValue";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "ExpectedEq";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_10_ref_0);
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_9_ref_0);
    let mut reader_2: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_8_ref_0);
    let mut reader_3: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_7_ref_0);
    let mut reader_4: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_6_ref_0);
    let mut reader_5: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_5_ref_0);
    let mut reader_6: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_7: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut reader_8: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut reader_9: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut reader_10: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_351() {
//    rusty_monitor::set_test_id(351);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut event_1: events::Event = crate::events::Event::Eof;
    let mut event_2: events::Event = crate::events::Event::Eof;
    let mut event_3: events::Event = crate::events::Event::Eof;
    let mut event_4: events::Event = crate::events::Event::Eof;
    let mut event_5: events::Event = crate::events::Event::Eof;
    let mut event_6: events::Event = crate::events::Event::Eof;
    let mut event_7: events::Event = crate::events::Event::Eof;
    let mut event_8: events::Event = crate::events::Event::Eof;
    let mut event_9: events::Event = crate::events::Event::Eof;
    let mut event_10: events::Event = crate::events::Event::Eof;
    let mut event_11: events::Event = crate::events::Event::Eof;
    let mut event_12: events::Event = crate::events::Event::Eof;
    let mut event_13: events::Event = crate::events::Event::Eof;
    let mut event_14: events::Event = crate::events::Event::Eof;
    let mut event_15: events::Event = crate::events::Event::Eof;
    let mut event_16: events::Event = crate::events::Event::Eof;
    let mut event_17: events::Event = crate::events::Event::Eof;
    let mut event_18: events::Event = crate::events::Event::Eof;
    let mut event_19: events::Event = crate::events::Event::Eof;
    let mut event_20: events::Event = crate::events::Event::Eof;
    let mut event_21: events::Event = crate::events::Event::Eof;
    let mut event_22: events::Event = crate::events::Event::Eof;
    let mut event_23: events::Event = crate::events::Event::Eof;
    let mut event_24: events::Event = crate::events::Event::Eof;
    let mut event_25: events::Event = crate::events::Event::Eof;
    let mut event_26: events::Event = crate::events::Event::Eof;
    let mut event_27: events::Event = crate::events::Event::Eof;
    let mut event_28: events::Event = crate::events::Event::Eof;
    let mut event_29: events::Event = crate::events::Event::Eof;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_200() {
//    rusty_monitor::set_test_id(200);
    let mut str_0: &str = "UdC9dKGDIuH1g";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_2: &str = "<";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut str_3: &str = "<";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut str_4: &str = "Attr::SingleQ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_3);
    let mut cow_4: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut str_5: &str = "check_duplicates";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_4);
    let mut cow_5: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut str_6: &str = "CData";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut cow_6: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_5);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_445() {
//    rusty_monitor::set_test_id(445);
    let mut str_0: &str = "InvalidDecimal";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "UnexpectedEof";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "ctqWE";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "WwSfw9N";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "UnrecognizedSymbol";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "keys";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "d8AKQK0JLfPaQmUXnFV";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "Io";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "Attributes";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut str_11: &str = "Io";
    let mut string_11: std::string::String = std::string::String::from(str_11);
    let mut str_12: &str = "3bY8Y";
    let mut string_12: std::string::String = std::string::String::from(str_12);
    let mut str_13: &str = "SkipValue";
    let mut string_13: std::string::String = std::string::String::from(str_13);
    let mut error_0: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_13, found: string_12};
    let mut error_1: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_11, found: string_10};
    let mut error_2: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_9, found: string_8};
    let mut error_3: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_7, found: string_6};
    let mut error_4: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_5, found: string_4};
    let mut error_5: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_3, found: string_2};
    let mut error_6: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_1, found: string_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_406() {
//    rusty_monitor::set_test_id(406);
    let mut u8_0: u8 = 103u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 29u8;
    let mut u8_3: u8 = 65u8;
    let mut u8_4: u8 = 109u8;
    let mut u8_5: u8 = 62u8;
    let mut u8_6: u8 = 45u8;
    let mut u8_7: u8 = 62u8;
    let mut u8_8: u8 = 68u8;
    let mut u8_9: u8 = 38u8;
    let mut u8_10: u8 = 45u8;
    let mut u8_11: u8 = 33u8;
    let mut u8_12: u8 = 39u8;
    let mut u8_13: u8 = 30u8;
    let mut u8_14: u8 = 108u8;
    let mut u8_15: u8 = 16u8;
    let mut error_0: errors::Error = crate::errors::Error::UnexpectedBang(u8_15);
    let mut error_1: errors::Error = crate::errors::Error::UnexpectedBang(u8_14);
    let mut error_2: errors::Error = crate::errors::Error::UnexpectedBang(u8_13);
    let mut error_3: errors::Error = crate::errors::Error::UnexpectedBang(u8_12);
    let mut error_4: errors::Error = crate::errors::Error::UnexpectedBang(u8_11);
    let mut error_5: errors::Error = crate::errors::Error::UnexpectedBang(u8_10);
    let mut error_6: errors::Error = crate::errors::Error::UnexpectedBang(u8_9);
    let mut error_7: errors::Error = crate::errors::Error::UnexpectedBang(u8_8);
    let mut error_8: errors::Error = crate::errors::Error::UnexpectedBang(u8_7);
    let mut error_9: errors::Error = crate::errors::Error::UnexpectedBang(u8_6);
    let mut error_10: errors::Error = crate::errors::Error::UnexpectedBang(u8_5);
    let mut error_11: errors::Error = crate::errors::Error::UnexpectedBang(u8_4);
    let mut error_12: errors::Error = crate::errors::Error::UnexpectedBang(u8_3);
    let mut error_13: errors::Error = crate::errors::Error::UnexpectedBang(u8_2);
    let mut error_14: errors::Error = crate::errors::Error::UnexpectedBang(u8_1);
    let mut error_15: errors::Error = crate::errors::Error::UnexpectedBang(u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_430() {
//    rusty_monitor::set_test_id(430);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_5: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_6: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_7: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_8: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_9: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_10: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_11: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_12: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_13: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_14: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_15: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_16: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_17: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_18: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_19: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_20: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_21: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_22: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_23: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_24: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_25: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_26: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_27: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_28: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_29: std::vec::Vec<u8> = std::vec::Vec::new();
//    panic!("From RustyUnit with love");
}
}