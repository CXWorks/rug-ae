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
#[timeout(30000)]fn rusty_test_8698() {
//    rusty_monitor::set_test_id(8698);
    let mut str_0: &str = "Ag";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "y50eFj5Qh2So7";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "InvalidCodepoint";
    let mut str_3: &str = "Io";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut str_4: &str = "value_len";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_6: &str = "'";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_6_ref_0);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut str_7: &str = "";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_7_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_8: &str = "'";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut bytestext_3_ref_0: &crate::events::BytesText = &mut bytestext_3;
    let mut str_9: &str = "PI";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_5: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    let mut bytestext_4_ref_0: &crate::events::BytesText = &mut bytestext_4;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut bytestext_6: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_8_ref_0);
    let mut bytestext_5_ref_0: &crate::events::BytesText = &mut bytestext_5;
    crate::events::BytesText::unescaped(bytestext_2_ref_0);
    crate::events::BytesText::unescaped(bytestext_1_ref_0);
    crate::events::BytesText::unescaped(bytestext_4_ref_0);
    crate::events::BytesText::unescaped(bytestext_5_ref_0);
    crate::events::BytesText::unescaped(bytestext_3_ref_0);
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
//    panic!("From RustyUnit with love");
}
}