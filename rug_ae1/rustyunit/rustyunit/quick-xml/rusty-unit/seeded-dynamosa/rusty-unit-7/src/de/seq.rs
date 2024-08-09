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
#[timeout(30000)]fn rusty_test_624() {
//    rusty_monitor::set_test_id(624);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_16: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_17: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_18: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_19: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_20: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_21: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_22: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_23: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_24: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_25: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_26: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_27: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_28: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_29: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
//    panic!("From RustyUnit with love");
}
}