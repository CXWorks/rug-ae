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
#[timeout(30000)]fn rusty_test_590() {
//    rusty_monitor::set_test_id(590);
    let mut str_0: &str = "CData";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "IKHEjMqx";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "InvalidHexadecimal";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "Comment";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut usize_0: usize = 14usize;
    let mut usize_1: usize = 1usize;
    let mut usize_2: usize = 12usize;
    let mut usize_3: usize = 8usize;
    let mut usize_4: usize = 2424usize;
    let mut usize_5: usize = 1usize;
    let mut usize_6: usize = 0usize;
    let mut usize_7: usize = 3usize;
    let mut usize_8: usize = 5usize;
    let mut usize_9: usize = 7usize;
    let mut usize_10: usize = 0usize;
    let mut usize_11: usize = 8usize;
    let mut usize_12: usize = 3469usize;
    let mut usize_13: usize = 14usize;
    let mut usize_14: usize = 1usize;
    let mut usize_15: usize = 3usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_15);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_14);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_13);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_12);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_11);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_10);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_9);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_8);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_7);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_6);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_5);
    let mut attrerror_11: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_4);
    let mut attrerror_12: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_3);
    let mut attrerror_13: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_2);
    let mut attrerror_14: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_1);
    let mut attrerror_15: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_0);
    let mut error_0: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_3, found: string_2};
    let mut error_1: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_1, found: string_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_597() {
//    rusty_monitor::set_test_id(597);
    let mut str_0: &str = "l";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_1: &str = "ExpectedQuote";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut str_2: &str = "G";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ExpectedValue";
    let mut str_4: &str = "DfL2Tzp8MPaeaVfGM";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_5: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_6: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut str_5: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut str_6: &str = "ExpectedQuote";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_3);
    let mut str_7: &str = "bindings";
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytestext_5: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_4);
    let mut str_7_ref_0: &str = &mut str_7;
    let mut bytestext_6: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_5);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_5);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_6);
    let mut cow_4: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_4);
    let mut cow_5: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut cow_6: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
//    panic!("From RustyUnit with love");
}
}