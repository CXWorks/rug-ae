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
fn rusty_test_4641() {
    rusty_monitor::set_test_id(4641);
    let mut u32_0: u32 = 8494u32;
    let mut str_0: &str = "zKyt";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut usize_0: usize = 9242usize;
    let mut usize_1: usize = 5840usize;
    let mut usize_2: usize = 4599usize;
    let mut usize_3: usize = 2331usize;
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut u8_0: u8 = 90u8;
    let mut usize_4: usize = 1118usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_4, u8_0);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_3);
    let mut u8_slice_0: &[u8] = std::option::Option::unwrap(option_0);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_2, usize_1);
    let mut u8_slice_1: &[u8] = std::option::Option::unwrap(option_1);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
    panic!("From RustyUnit with love");
}
}