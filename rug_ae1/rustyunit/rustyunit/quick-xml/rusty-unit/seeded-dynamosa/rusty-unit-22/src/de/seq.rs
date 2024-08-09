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
#[timeout(30000)]fn rusty_test_5405() {
//    rusty_monitor::set_test_id(5405);
    let mut str_0: &str = "sICoM8u";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1: &str = "Bang";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = 5isize;
    let mut isize_1: isize = 6isize;
    let mut str_2: &str = "Utf8";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "w8hrVZLL23VhJG9K";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_1, isize_0);
    let mut reader_2: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
//    panic!("From RustyUnit with love");
}
}