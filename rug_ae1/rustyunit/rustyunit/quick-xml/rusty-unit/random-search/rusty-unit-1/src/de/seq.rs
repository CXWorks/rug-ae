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
fn rusty_test_5153() {
    rusty_monitor::set_test_id(5153);
    let mut str_0: &str = "CqOIAoUmOz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_0: usize = 6325usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    let mut usize_1: usize = 2532usize;
    let mut str_1: &str = "crFALIRb";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut str_2: &str = "Qk4";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut i32_0: i32 = 13282i32;
    let mut usize_2: usize = 1258usize;
    let mut bool_2: bool = false;
    let mut usize_3: usize = 6425usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_3, bool_2);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut str_4: &str = "s13Szn0VhKDa4Fyp";
    let mut string_0: std::string::String = std::string::String::from(str_4);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut error_0: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    let mut event_0: events::Event = crate::events::Event::Text(bytestext_1);
    let mut error_1: errors::Error = crate::errors::Error::TextNotFound;
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_1);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut error_2: errors::Error = crate::errors::Error::EscapeError(escapeerror_0);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_1);
    let mut event_4: events::Event = crate::events::Event::into_owned(event_3);
    let mut event_5: events::Event = crate::events::Event::into_owned(event_4);
    let mut error_3: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
    panic!("From RustyUnit with love");
}
}