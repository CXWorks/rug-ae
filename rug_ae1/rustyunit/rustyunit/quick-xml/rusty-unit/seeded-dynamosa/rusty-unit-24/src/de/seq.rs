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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8702() {
//    rusty_monitor::set_test_id(8702);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 4905usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_1);
    let mut str_0: &str = "TooLongDecimal";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1: &str = "fYF7dbDJHCIT18fEtN3";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_0);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut error_0: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_1);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
//    panic!("From RustyUnit with love");
}
}