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
#[timeout(30000)]fn rusty_test_9536() {
//    rusty_monitor::set_test_id(9536);
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut str_0: &str = "&";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut isize_0: isize = 9isize;
    let mut isize_1: isize = 1929isize;
    let mut str_1: &str = "InvalidAttr";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = 5isize;
    let mut isize_3: isize = -12763isize;
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_3, isize_2);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}
}