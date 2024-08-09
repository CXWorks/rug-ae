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
fn rusty_test_3524() {
    rusty_monitor::set_test_id(3524);
    let mut str_0: &str = "l6A9bB84JfdWDaBEiQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut str_1: &str = "yd3J";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 7958usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_0, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut str_2: &str = "oIRXTC5F1myL";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut bool_1: bool = true;
    let mut usize_1: usize = 4871usize;
    let mut usize_2: usize = 3921usize;
    let mut usize_3: usize = 345usize;
    let mut usize_4: usize = 4849usize;
    let mut bool_2: bool = false;
    let mut usize_5: usize = 7410usize;
    let mut iterstate_1: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_5, bool_2);
    let mut usize_6: usize = 2523usize;
    let mut bool_3: bool = false;
    let mut usize_7: usize = 2178usize;
    let mut iterstate_2: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_7, bool_3);
    let mut iterstate_2_ref_0: &crate::events::attributes::IterState = &mut iterstate_2;
    let mut event_0: events::Event = crate::events::Event::Text(bytestext_0);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    panic!("From RustyUnit with love");
}
}