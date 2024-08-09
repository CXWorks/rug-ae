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
fn rusty_test_7086() {
    rusty_monitor::set_test_id(7086);
    let mut usize_0: usize = 2652usize;
    let mut usize_1: usize = 9591usize;
    let mut u8_0: u8 = 78u8;
    let mut str_0: &str = "cKwbUdjqIA0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut str_1: &str = "ZixnCIPOim2";
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut u32_0: u32 = 5975u32;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
    let mut str_2: &str = "kz60ta56DUKTcnc";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut option_2: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_3: std::option::Option<&[u8]> = std::option::Option::None;
    let mut u32_1: u32 = 9249u32;
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_1);
    let mut str_3: &str = "ymbKm13eN";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut str_4: &str = "2C4Oq";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut str_5: &str = "zWC0Ty2FcpLE3WvHm5k";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut cow_1_ref_0: &std::borrow::Cow<[u8]> = &mut cow_1;
    let mut usize_2: usize = 2665usize;
    let mut usize_3: usize = 5555usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_3, usize_2);
    let mut error_0: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_1: errors::Error = crate::errors::Error::TextNotFound;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut error_2: errors::Error = crate::errors::Error::EscapeError(escapeerror_1);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    crate::events::BytesText::unescaped(bytestext_1_ref_0);
    let mut error_3: errors::Error = crate::errors::Error::TextNotFound;
    let mut error_4: errors::Error = crate::errors::Error::EscapeError(escapeerror_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut u8_slice_0: &[u8] = std::option::Option::unwrap(option_2);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    panic!("From RustyUnit with love");
}
}