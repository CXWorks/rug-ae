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
fn rusty_test_5501() {
    rusty_monitor::set_test_id(5501);
    let mut str_0: &str = "1BOni9wRBvJakM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut str_1: &str = "MhMMrb2yMetc5bdfa";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_2: &str = "M";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_3: &str = "hO7YaqRTRv";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut usize_0: usize = 8867usize;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut char_0: char = 'u';
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_0);
    let mut bytesend_1: crate::events::BytesEnd = crate::events::BytesEnd::into_owned(bytesend_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    let mut bytesend_2: crate::events::BytesEnd = crate::events::BytesEnd::into_owned(bytesend_1);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut bytesend_2_ref_0: &crate::events::BytesEnd = &mut bytesend_2;
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_2_ref_0);
    panic!("From RustyUnit with love");
}
}