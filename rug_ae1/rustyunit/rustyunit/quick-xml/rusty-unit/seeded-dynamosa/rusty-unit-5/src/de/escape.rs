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
#[timeout(30000)]fn rusty_test_290() {
//    rusty_monitor::set_test_id(290);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 5896usize;
    let mut usize_2: usize = 12usize;
    let mut usize_3: usize = 2363usize;
    let mut usize_4: usize = 3usize;
    let mut usize_5: usize = 3739usize;
    let mut usize_6: usize = 7usize;
    let mut usize_7: usize = 7usize;
    let mut usize_8: usize = 1usize;
    let mut usize_9: usize = 8usize;
    let mut usize_10: usize = 2usize;
    let mut usize_11: usize = 14usize;
    let mut usize_12: usize = 2usize;
    let mut usize_13: usize = 7631usize;
    let mut usize_14: usize = 6usize;
    let mut usize_15: usize = 180usize;
    let mut usize_16: usize = 2673usize;
    let mut usize_17: usize = 3usize;
    let mut usize_18: usize = 8076usize;
    let mut usize_19: usize = 2usize;
    let mut usize_20: usize = 2765usize;
    let mut usize_21: usize = 4usize;
    let mut usize_22: usize = 1usize;
    let mut usize_23: usize = 6usize;
    let mut usize_24: usize = 5usize;
    let mut usize_25: usize = 1usize;
    let mut usize_26: usize = 8104usize;
    let mut usize_27: usize = 7434usize;
    let mut usize_28: usize = 4713usize;
    let mut usize_29: usize = 4usize;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_310() {
//    rusty_monitor::set_test_id(310);
    let mut isize_0: isize = 0isize;
    let mut isize_1: isize = 4isize;
    let mut isize_2: isize = 1isize;
    let mut isize_3: isize = 8847isize;
    let mut isize_4: isize = 7isize;
    let mut isize_5: isize = 8isize;
    let mut isize_6: isize = 3isize;
    let mut isize_7: isize = 3025isize;
    let mut isize_8: isize = 8isize;
    let mut isize_9: isize = -5551isize;
    let mut isize_10: isize = 2isize;
    let mut isize_11: isize = 8isize;
    let mut isize_12: isize = -13413isize;
    let mut isize_13: isize = 2020isize;
    let mut isize_14: isize = 6888isize;
    let mut isize_15: isize = -2952isize;
    let mut isize_16: isize = 9isize;
    let mut isize_17: isize = 14286isize;
    let mut isize_18: isize = 9isize;
    let mut isize_19: isize = 5isize;
    let mut isize_20: isize = -3946isize;
    let mut isize_21: isize = 5isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_21, isize_20);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_19, isize_18);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_17, isize_16);
    let mut attr_3: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_15, isize_14);
    let mut attr_4: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_13, isize_12);
    let mut attr_5: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_11, isize_10);
    let mut attr_6: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_9, isize_8);
    let mut attr_7: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_7, isize_6);
    let mut attr_8: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_5, isize_4);
    let mut attr_9: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_3, isize_2);
    let mut attr_10: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_296() {
//    rusty_monitor::set_test_id(296);
    let mut char_0: char = '?';
    let mut char_1: char = 'o';
    let mut char_2: char = '`';
    let mut char_3: char = 'A';
    let mut char_4: char = 'i';
    let mut char_5: char = '_';
    let mut char_6: char = 'k';
    let mut char_7: char = '<';
    let mut char_8: char = 'W';
    let mut char_9: char = '*';
    let mut char_10: char = 'H';
    let mut char_11: char = 'F';
    let mut char_12: char = 'j';
    let mut char_13: char = '-';
    let mut char_14: char = 'l';
    let mut char_15: char = '<';
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
#[timeout(30000)]fn rusty_test_312() {
//    rusty_monitor::set_test_id(312);
    let mut usize_0: usize = 2318usize;
    let mut usize_1: usize = 5usize;
    let mut usize_2: usize = 0usize;
    let mut usize_3: usize = 1usize;
    let mut usize_4: usize = 4445usize;
    let mut usize_5: usize = 7836usize;
    let mut usize_6: usize = 2usize;
    let mut usize_7: usize = 1usize;
    let mut usize_8: usize = 9521usize;
    let mut usize_9: usize = 2usize;
    let mut usize_10: usize = 0usize;
    let mut usize_11: usize = 403usize;
    let mut usize_12: usize = 8903usize;
    let mut usize_13: usize = 12usize;
    let mut usize_14: usize = 2277usize;
    let mut usize_15: usize = 12usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_15);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_14);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_13);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_12);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_11);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_10);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_9);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_8);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_7);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_6);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_5);
    let mut attrerror_11: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_4);
    let mut attrerror_12: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_3);
    let mut attrerror_13: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_2);
    let mut attrerror_14: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_1);
    let mut attrerror_15: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_362() {
//    rusty_monitor::set_test_id(362);
    let mut error_0: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_1: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_2: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_3: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_4: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_5: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_6: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_7: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_8: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_9: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_10: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_11: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_12: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_13: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_14: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_15: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_16: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_17: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_18: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_19: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_20: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_21: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_22: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_23: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_24: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_25: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_26: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_27: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_28: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_29: errors::Error = crate::errors::Error::TextNotFound;
//    panic!("From RustyUnit with love");
}
}