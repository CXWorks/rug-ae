use crate::de::{DeError, DeEvent, Deserializer, XmlRead};
use crate::events::BytesStart;
use serde::de::{self, DeserializeSeed};

#[derive(Debug)]
enum Names {
    Unknown,
    Peek(Vec<u8>),
}

impl Names {
    fn is_valid(&self, start: &BytesStart) -> bool {
        match self {
            Names::Unknown => true,
            Names::Peek(n) => n == start.name(),
        }
    }
}

/// A SeqAccess
pub struct SeqAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    de: &'a mut Deserializer<'de, R>,
    names: Names,
}

impl<'a, 'de, R> SeqAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    /// Get a new SeqAccess
    pub fn new(de: &'a mut Deserializer<'de, R>) -> Result<Self, DeError> {
        let names = if de.has_value_field {
            Names::Unknown
        } else {
            if let DeEvent::Start(e) = de.peek()? {
                Names::Peek(e.name().to_vec())
            } else {
                Names::Unknown
            }
        };
        Ok(SeqAccess { de, names })
    }
}

impl<'de, 'a, R> de::SeqAccess<'de> for SeqAccess<'de, 'a, R>
where
    R: XmlRead<'de>,
{
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, DeError>
    where
        T: DeserializeSeed<'de>,
    {
        match self.de.peek()? {
            DeEvent::Eof | DeEvent::End(_) => Ok(None),
            DeEvent::Start(e) if !self.names.is_valid(e) => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8273() {
    rusty_monitor::set_test_id(8273);
    let mut str_0: &str = "kSm7";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut str_1: &str = "puRYBEX8l5zWiLeWZas";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut event_0: events::Event = crate::events::Event::CData(bytescdata_0);
    let mut str_2: &str = "8g";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "AfQaJRishad5os";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut usize_0: usize = 1484usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    let mut str_4: &str = "0n0zBB";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_5: &str = "dAjz9wmd8fU0nd";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut str_6: &str = "ziINb1DvdBYLz5";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_3);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_7: &str = "NswYH2XnNQe2Bx1s";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_7_ref_0);
    let mut error_0: errors::Error = crate::errors::Error::TextNotFound;
    let mut event_1: events::Event = crate::events::Event::Comment(bytestext_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_1);
    let mut error_1: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    panic!("From RustyUnit with love");
}
}