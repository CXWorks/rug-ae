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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6729() {
    rusty_monitor::set_test_id(6729);
    let mut isize_0: isize = -17602isize;
    let mut isize_1: isize = 879isize;
    let mut str_0: &str = "wFGRthHk9ZUM";
    let mut str_1: &str = "CF79fljgqxdecLc6XpC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_2: &str = "JlHfMzRMFimdexpXUh";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_3: &str = "nGx9SM6YHre73DmoZQ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "UBlKcgahGzRMTKkcz";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_0, isize_1);
    crate::events::BytesText::unescaped(bytestext_1_ref_0);
    panic!("From RustyUnit with love");
}
}