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
fn rusty_test_4315() {
    rusty_monitor::set_test_id(4315);
    let mut str_0: &str = "OWBfvWPfJUX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 9133usize;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 9143usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_1, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut usize_2: usize = 2905usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_2);
    let mut str_1: &str = "mN7OFnbf1f9eIBDWYc6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut usize_3: usize = 9426usize;
    let mut usize_4: usize = 8605usize;
    let mut usize_5: usize = 479usize;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 7914usize;
    let mut iterstate_1: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_6, bool_1);
    let mut iterstate_1_ref_0: &mut crate::events::attributes::IterState = &mut iterstate_1;
    let mut str_2: &str = "KhoqE711kkwztUosb0q";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_5);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_4);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_3);
    let mut error_0: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}