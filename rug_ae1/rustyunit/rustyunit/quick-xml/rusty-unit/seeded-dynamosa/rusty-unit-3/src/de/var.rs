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
#[timeout(30000)]fn rusty_test_700() {
//    rusty_monitor::set_test_id(700);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_0: &str = "UnquotedValue";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut u8_0: u8 = 111u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 35u8;
    let mut u8_3: u8 = 17u8;
    let mut u8_4: u8 = 65u8;
    let mut u8_5: u8 = 70u8;
    let mut u8_6: u8 = 102u8;
    let mut u8_7: u8 = 126u8;
    let mut u8_8: u8 = 57u8;
    let mut u8_9: u8 = 102u8;
    let mut u8_10: u8 = 63u8;
    let mut u8_11: u8 = 10u8;
    let mut u8_12: u8 = 47u8;
    let mut u8_13: u8 = 106u8;
    let mut u8_14: u8 = 117u8;
    let mut bool_0: bool = crate::reader::is_whitespace(u8_14);
    let mut bool_1: bool = crate::reader::is_whitespace(u8_13);
    let mut bool_2: bool = crate::reader::is_whitespace(u8_12);
    let mut bool_3: bool = crate::reader::is_whitespace(u8_11);
    let mut bool_4: bool = crate::reader::is_whitespace(u8_10);
    let mut bool_5: bool = crate::reader::is_whitespace(u8_9);
    let mut bool_6: bool = crate::reader::is_whitespace(u8_8);
    let mut bool_7: bool = crate::reader::is_whitespace(u8_7);
    let mut bool_8: bool = crate::reader::is_whitespace(u8_6);
    let mut bool_9: bool = crate::reader::is_whitespace(u8_5);
    let mut bool_10: bool = crate::reader::is_whitespace(u8_4);
    let mut bool_11: bool = crate::reader::is_whitespace(u8_3);
    let mut bool_12: bool = crate::reader::is_whitespace(u8_2);
    let mut bool_13: bool = crate::reader::is_whitespace(u8_1);
    let mut bool_14: bool = crate::reader::is_whitespace(u8_0);
//    panic!("From RustyUnit with love");
}
}