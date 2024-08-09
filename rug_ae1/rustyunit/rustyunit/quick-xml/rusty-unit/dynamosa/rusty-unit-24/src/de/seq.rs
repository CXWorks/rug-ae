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
fn rusty_test_7724() {
    rusty_monitor::set_test_id(7724);
    let mut str_0: &str = "7SLFgmN4MPz34ay";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "7zoKhFaOij7AdCMkMv";
    let mut str_2: &str = "yL8hoIl0f17OgBHI";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut str_3: &str = "B7";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut str_4: &str = "Qb1V2ExLv7lDbl";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_1);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut str_5: &str = "uQPcaXVLTnMBCzvfjRW";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_5_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut u8_0: u8 = 9u8;
    let mut usize_0: usize = 3343usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_0, u8_0);
    let mut str_6: &str = "cZwnAfxvsad1OiQzP";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_6_ref_0);
    let mut reader_1_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_1;
    let mut str_7: &str = "sYOaY";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_7_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut str_8: &str = "hNcVayd";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_8_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut str_9: &str = "LtJB4D";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_9_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut str_9: &str = "LtJB4D";
    let mut str_9_ref_0: &str = &mut str_1;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    crate::events::BytesText::unescaped(bytestext_2_ref_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    panic!("From RustyUnit with love");
}
}