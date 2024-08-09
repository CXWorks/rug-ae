//! Serde `Deserializer` module

use crate::de::deserialize_bool;
use crate::{errors::serialize::DeError, errors::Error, escape::unescape, reader::Decoder};
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::{self, forward_to_deserialize_any, serde_if_integer128};
use std::borrow::Cow;

/// A deserializer for a xml escaped and encoded value
///
/// # Note
///
/// Escaping the value is actually not always necessary, for instance
/// when converting to float, we don't expect any escapable character
/// anyway
#[derive(Clone, Debug)]
pub struct EscapedDeserializer<'a> {
    decoder: Decoder,
    /// Possible escaped value of text/CDATA or attribute value
    escaped_value: Cow<'a, [u8]>,
    /// If `true`, value requires unescaping before using
    escaped: bool,
}

impl<'a> EscapedDeserializer<'a> {
    pub fn new(escaped_value: Cow<'a, [u8]>, decoder: Decoder, escaped: bool) -> Self {
        EscapedDeserializer {
            decoder,
            escaped_value,
            escaped,
        }
    }
    fn unescaped(&self) -> Result<Cow<[u8]>, DeError> {
        if self.escaped {
            unescape(&self.escaped_value).map_err(|e| DeError::InvalidXml(Error::EscapeError(e)))
        } else {
            Ok(Cow::Borrowed(&self.escaped_value))
        }
    }
}

macro_rules! deserialize_num {
    ($method:ident, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            #[cfg(not(feature = "encoding"))]
            let value = self.decoder.decode(self.escaped_value.as_ref())?.parse()?;

            #[cfg(feature = "encoding")]
            let value = self.decoder.decode(self.escaped_value.as_ref()).parse()?;

            visitor.$visit(value)
        }
    };
}

impl<'de, 'a> serde::Deserializer<'de> for EscapedDeserializer<'a> {
    type Error = DeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let unescaped = self.unescaped()?;
        #[cfg(not(feature = "encoding"))]
        let value = self.decoder.decode(&unescaped)?;

        #[cfg(feature = "encoding")]
        let value = self.decoder.decode(&unescaped);
        visitor.visit_str(&value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let v = self.unescaped()?;
        visitor.visit_bytes(&v)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        deserialize_bool(self.escaped_value.as_ref(), self.decoder, visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.escaped_value.as_ref().is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    deserialize_num!(deserialize_i64, visit_i64);
    deserialize_num!(deserialize_i32, visit_i32);
    deserialize_num!(deserialize_i16, visit_i16);
    deserialize_num!(deserialize_i8, visit_i8);
    deserialize_num!(deserialize_u64, visit_u64);
    deserialize_num!(deserialize_u32, visit_u32);
    deserialize_num!(deserialize_u16, visit_u16);
    deserialize_num!(deserialize_u8, visit_u8);
    deserialize_num!(deserialize_f64, visit_f64);
    deserialize_num!(deserialize_f32, visit_f32);

    serde_if_integer128! {
        deserialize_num!(deserialize_i128, visit_i128);
        deserialize_num!(deserialize_u128, visit_u128);
    }

    forward_to_deserialize_any! {
        unit_struct seq tuple tuple_struct map struct identifier ignored_any
    }
}

impl<'de, 'a> EnumAccess<'de> for EscapedDeserializer<'a> {
    type Error = DeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let name = seed.deserialize(self.clone())?;
        Ok((name, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for EscapedDeserializer<'a> {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1378() {
//    rusty_monitor::set_test_id(1378);
    let mut str_0: &str = "Utf8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "UnterminatedEntity";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut str_2: &str = "bindings";
    let mut str_3: &str = "UnexpectedBang";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut str_4: &str = "8Icjoh";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "t0crCF";
    let mut str_6: &str = "EntityWithNull";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut str_5_ref_0: &str = &mut str_5;
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_7: &str = "Attr::Empty";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_8: &str = "SkipEqValue";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_9: &str = ">";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    let mut u8_0: u8 = 45u8;
    let mut str_10: &str = "upUoX1";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData::from_str(str_8_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_4);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_10_ref_0: &str = &mut str_10;
    let mut bytescdata_6: crate::events::BytesCData = crate::events::BytesCData::from_str(str_7_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_5);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
    let mut bytestext_5: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_4);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesText::escaped(bytestext_2_ref_0);
    let mut bytestext_5_ref_0: &crate::events::BytesText = &mut bytestext_5;
    let mut u8_slice_1: &[u8] = crate::events::BytesText::escaped(bytestext_0_ref_0);
//    panic!("From RustyUnit with love");
}
}