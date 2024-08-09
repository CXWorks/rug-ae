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
#[timeout(30000)]fn rusty_test_2685() {
//    rusty_monitor::set_test_id(2685);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_0: &str = "DocType";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 4usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_0, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut u8_slice_0: &[u8] = std::option::Option::unwrap(option_1);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
//    panic!("From RustyUnit with love");
}
}