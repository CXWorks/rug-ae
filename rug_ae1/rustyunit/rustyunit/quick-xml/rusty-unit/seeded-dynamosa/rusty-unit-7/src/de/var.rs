use crate::{
    de::{escape::EscapedDeserializer, DeEvent, Deserializer, XmlRead},
    errors::serialize::DeError,
};
use serde::de::{self, DeserializeSeed, Deserializer as SerdeDeserializer, Visitor};
use std::borrow::Cow;

/// An enum access
pub struct EnumAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    de: &'a mut Deserializer<'de, R>,
}

impl<'de, 'a, R> EnumAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    pub fn new(de: &'a mut Deserializer<'de, R>) -> Self {
        EnumAccess { de }
    }
}

impl<'de, 'a, R> de::EnumAccess<'de> for EnumAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    type Error = DeError;
    type Variant = VariantAccess<'de, 'a, R>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantAccess<'de, 'a, R>), DeError>
    where
        V: DeserializeSeed<'de>,
    {
        let decoder = self.de.reader.decoder();
        let de = match self.de.peek()? {
            DeEvent::Text(t) => EscapedDeserializer::new(Cow::Borrowed(t), decoder, true),
            // Escape sequences does not processed inside CDATA section
            DeEvent::CData(t) => EscapedDeserializer::new(Cow::Borrowed(t), decoder, false),
            DeEvent::Start(e) => EscapedDeserializer::new(Cow::Borrowed(e.name()), decoder, false),
            _ => {
                return Err(DeError::Unsupported(
                    "Invalid event for Enum, expecting `Text` or `Start`",
                ))
            }
        };
        let name = seed.deserialize(de)?;
        Ok((name, VariantAccess { de: self.de }))
    }
}

pub struct VariantAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    de: &'a mut Deserializer<'de, R>,
}

impl<'de, 'a, R> de::VariantAccess<'de> for VariantAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    type Error = DeError;

    fn unit_variant(self) -> Result<(), DeError> {
        match self.de.next()? {
            DeEvent::Start(e) => self.de.read_to_end(e.name()),
            DeEvent::Text(_) | DeEvent::CData(_) => Ok(()),
            _ => unreachable!(),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, DeError>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, DeError>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DeError>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_struct("", fields, visitor)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_169() {
//    rusty_monitor::set_test_id(169);
    let mut str_0: &str = "Attr::Empty";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "UsWHJ9hf";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "iua8qaawi";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "'";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "NamespaceEntry";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "<";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "level";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "SfyvqhkFXHEv1e";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "TooLongDecimal";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "Bang";
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
#[timeout(30000)]fn rusty_test_261() {
//    rusty_monitor::set_test_id(261);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_16: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_17: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_18: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_19: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_20: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_21: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_22: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_23: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_24: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_25: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_26: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_27: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_28: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_29: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_345() {
//    rusty_monitor::set_test_id(345);
    let mut str_0: &str = "S81g5";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "CData";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "TooLongDecimal";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "UnrecognizedSymbol";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "NamespaceResolver";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "fBSxwtvBm";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "ygsq4XdVOhju6EpwS1";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "Attributes";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "prefix_len";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "'";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut error_0: errors::Error = crate::errors::Error::UnexpectedToken(string_10);
    let mut error_1: errors::Error = crate::errors::Error::UnexpectedToken(string_9);
    let mut error_2: errors::Error = crate::errors::Error::UnexpectedToken(string_8);
    let mut error_3: errors::Error = crate::errors::Error::UnexpectedToken(string_7);
    let mut error_4: errors::Error = crate::errors::Error::UnexpectedToken(string_6);
    let mut error_5: errors::Error = crate::errors::Error::UnexpectedToken(string_5);
    let mut error_6: errors::Error = crate::errors::Error::UnexpectedToken(string_4);
    let mut error_7: errors::Error = crate::errors::Error::UnexpectedToken(string_3);
    let mut error_8: errors::Error = crate::errors::Error::UnexpectedToken(string_2);
    let mut error_9: errors::Error = crate::errors::Error::UnexpectedToken(string_1);
    let mut error_10: errors::Error = crate::errors::Error::UnexpectedToken(string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_247() {
//    rusty_monitor::set_test_id(247);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    let mut bool_0: bool = true;
    let mut i32_0: i32 = -8701i32;
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1_ref_0: &mut std::vec::Vec<u8> = &mut vec_1;
    let mut bool_1: bool = true;
    let mut i32_1: i32 = 0i32;
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_2_ref_0: &mut std::vec::Vec<u8> = &mut vec_2;
    let mut bool_2: bool = false;
    let mut i32_2: i32 = 1i32;
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_3_ref_0: &mut std::vec::Vec<u8> = &mut vec_3;
    let mut bool_3: bool = false;
    let mut i32_3: i32 = 0i32;
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_4_ref_0: &mut std::vec::Vec<u8> = &mut vec_4;
    let mut bool_4: bool = true;
    let mut i32_4: i32 = 0i32;
    let mut vec_5: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_5_ref_0: &mut std::vec::Vec<u8> = &mut vec_5;
    let mut bool_5: bool = false;
    let mut i32_5: i32 = 7434i32;
    let mut vec_6: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_6_ref_0: &mut std::vec::Vec<u8> = &mut vec_6;
    let mut bool_6: bool = true;
    let mut i32_6: i32 = 4i32;
    let mut vec_7: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_7_ref_0: &mut std::vec::Vec<u8> = &mut vec_7;
    let mut bool_7: bool = false;
    let mut i32_7: i32 = 1i32;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_372() {
//    rusty_monitor::set_test_id(372);
    let mut char_0: char = 'z';
    let mut char_1: char = 'D';
    let mut char_2: char = '@';
    let mut char_3: char = 'I';
    let mut char_4: char = '\n';
    let mut char_5: char = '9';
    let mut char_6: char = 'i';
    let mut char_7: char = '?';
    let mut char_8: char = ' ';
    let mut char_9: char = 'a';
    let mut char_10: char = 'Y';
    let mut char_11: char = 'e';
    let mut char_12: char = 'U';
    let mut char_13: char = 'u';
    let mut char_14: char = '3';
    let mut char_15: char = 'Y';
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_15);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_14);
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_13);
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_12);
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_11);
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_10);
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_9);
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_8);
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_7);
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_6);
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_5);
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_4);
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_3);
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_2);
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_1);
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_273() {
//    rusty_monitor::set_test_id(273);
    let mut u8_0: u8 = 108u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 13u8;
    let mut u8_3: u8 = 34u8;
    let mut u8_4: u8 = 60u8;
    let mut u8_5: u8 = 9u8;
    let mut u8_6: u8 = 45u8;
    let mut u8_7: u8 = 10u8;
    let mut u8_8: u8 = 39u8;
    let mut u8_9: u8 = 10u8;
    let mut u8_10: u8 = 60u8;
    let mut u8_11: u8 = 38u8;
    let mut u8_12: u8 = 116u8;
    let mut u8_13: u8 = 54u8;
    let mut u8_14: u8 = 104u8;
    let mut u8_15: u8 = 45u8;
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
#[timeout(30000)]fn rusty_test_401() {
//    rusty_monitor::set_test_id(401);
    let mut str_0: &str = "found";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "InvalidAttr";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "\"";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "FY";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "Start";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "Io";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "Empty";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "EndEventMismatch";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "InvalidHexadecimal";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "DOCTYPE";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "Next";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut error_0: errors::Error = crate::errors::Error::UnexpectedEof(string_10);
    let mut error_1: errors::Error = crate::errors::Error::UnexpectedEof(string_9);
    let mut error_2: errors::Error = crate::errors::Error::UnexpectedEof(string_8);
    let mut error_3: errors::Error = crate::errors::Error::UnexpectedEof(string_7);
    let mut error_4: errors::Error = crate::errors::Error::UnexpectedEof(string_6);
    let mut error_5: errors::Error = crate::errors::Error::UnexpectedEof(string_5);
    let mut error_6: errors::Error = crate::errors::Error::UnexpectedEof(string_4);
    let mut error_7: errors::Error = crate::errors::Error::UnexpectedEof(string_3);
    let mut error_8: errors::Error = crate::errors::Error::UnexpectedEof(string_2);
    let mut error_9: errors::Error = crate::errors::Error::UnexpectedEof(string_1);
    let mut error_10: errors::Error = crate::errors::Error::UnexpectedEof(string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_522() {
//    rusty_monitor::set_test_id(522);
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut i32_0: i32 = 4852i32;
    let mut bool_2: bool = false;
    let mut bool_3: bool = true;
    let mut i32_1: i32 = 1i32;
    let mut bool_4: bool = true;
    let mut bool_5: bool = false;
    let mut i32_2: i32 = 4i32;
    let mut bool_6: bool = false;
    let mut bool_7: bool = false;
    let mut i32_3: i32 = 0i32;
    let mut bool_8: bool = false;
    let mut bool_9: bool = true;
    let mut i32_4: i32 = 4i32;
    let mut bool_10: bool = true;
    let mut bool_11: bool = false;
    let mut i32_5: i32 = 4i32;
    let mut bool_12: bool = true;
    let mut bool_13: bool = false;
    let mut i32_6: i32 = -1574i32;
    let mut bool_14: bool = false;
    let mut bool_15: bool = false;
    let mut i32_7: i32 = 0i32;
    let mut bool_16: bool = false;
    let mut bool_17: bool = true;
    let mut i32_8: i32 = 1i32;
    let mut bool_18: bool = false;
    let mut bool_19: bool = false;
    let mut i32_9: i32 = -2965i32;
    let mut bool_20: bool = true;
    let mut bool_21: bool = true;
    let mut i32_10: i32 = 4i32;
//    panic!("From RustyUnit with love");
}
}