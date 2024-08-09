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
#[timeout(30000)]fn rusty_test_337() {
//    rusty_monitor::set_test_id(337);
    let mut str_0: &str = "Utf8";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "EFDHEl6m2";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "PtLtW6YPKT1dUNVhz99";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "Eof";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "9LpP4vF6v";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "InvalidHexadecimal";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "InvalidDecimal";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "MN0jtU56pGKxI6L9FB";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "KCwiHABZvP9";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "ExpectedQuote";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "n";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut str_11: &str = "Attr::DoubleQ";
    let mut string_11: std::string::String = std::string::String::from(str_11);
    let mut str_12: &str = "u0s5DD5lT6lKcy8";
    let mut string_12: std::string::String = std::string::String::from(str_12);
    let mut str_13: &str = "cNGj8CvHbrjry";
    let mut string_13: std::string::String = std::string::String::from(str_13);
    let mut error_0: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_13, found: string_12};
    let mut error_1: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_11, found: string_10};
    let mut error_2: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_9, found: string_8};
    let mut error_3: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_7, found: string_6};
    let mut error_4: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_5, found: string_4};
    let mut error_5: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_3, found: string_2};
    let mut error_6: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_1, found: string_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_174() {
//    rusty_monitor::set_test_id(174);
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_481() {
//    rusty_monitor::set_test_id(481);
    let mut u8_0: u8 = 73u8;
    let mut usize_0: usize = 8usize;
    let mut u8_1: u8 = 60u8;
    let mut usize_1: usize = 0usize;
    let mut u8_2: u8 = 58u8;
    let mut usize_2: usize = 7usize;
    let mut u8_3: u8 = 10u8;
    let mut usize_3: usize = 7usize;
    let mut u8_4: u8 = 115u8;
    let mut usize_4: usize = 7usize;
    let mut u8_5: u8 = 45u8;
    let mut usize_5: usize = 1usize;
    let mut u8_6: u8 = 102u8;
    let mut usize_6: usize = 1usize;
    let mut u8_7: u8 = 117u8;
    let mut usize_7: usize = 0usize;
    let mut u8_8: u8 = 39u8;
    let mut usize_8: usize = 6usize;
    let mut u8_9: u8 = 24u8;
    let mut usize_9: usize = 2usize;
    let mut u8_10: u8 = 124u8;
    let mut usize_10: usize = 0usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_10, u8_10);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_9, u8_9);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_8, u8_8);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_7, u8_7);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_6, u8_6);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_5, u8_5);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_4, u8_4);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_3, u8_3);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_2, u8_2);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_1, u8_1);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_0, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_438() {
//    rusty_monitor::set_test_id(438);
    let mut str_0: &str = "9RYZss";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_1: &str = "Bang";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut str_2: &str = "A";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut str_3: &str = "YbH4joF7qB7hQ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut str_4: &str = "JO";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut str_5: &str = "UnexpectedEof";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_5: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_3);
    let mut str_6: &str = "KedF6o2q9";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytestext_6: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_4);
    let mut event_0: events::Event = crate::events::Event::PI(bytestext_6);
    let mut event_1: events::Event = crate::events::Event::PI(bytestext_5);
    let mut event_2: events::Event = crate::events::Event::PI(bytestext_4);
    let mut event_3: events::Event = crate::events::Event::PI(bytestext_3);
    let mut event_4: events::Event = crate::events::Event::PI(bytestext_2);
    let mut event_5: events::Event = crate::events::Event::PI(bytestext_1);
    let mut event_6: events::Event = crate::events::Event::PI(bytestext_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_504() {
//    rusty_monitor::set_test_id(504);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut event_1: events::Event = crate::events::Event::Eof;
    let mut event_2: events::Event = crate::events::Event::Eof;
    let mut event_3: events::Event = crate::events::Event::Eof;
    let mut event_4: events::Event = crate::events::Event::Eof;
    let mut event_5: events::Event = crate::events::Event::Eof;
    let mut event_6: events::Event = crate::events::Event::Eof;
    let mut event_7: events::Event = crate::events::Event::Eof;
    let mut event_8: events::Event = crate::events::Event::Eof;
    let mut event_9: events::Event = crate::events::Event::Eof;
    let mut event_10: events::Event = crate::events::Event::Eof;
    let mut event_11: events::Event = crate::events::Event::Eof;
    let mut event_12: events::Event = crate::events::Event::Eof;
    let mut event_13: events::Event = crate::events::Event::Eof;
    let mut event_14: events::Event = crate::events::Event::Eof;
    let mut event_15: events::Event = crate::events::Event::Eof;
    let mut event_16: events::Event = crate::events::Event::Eof;
    let mut event_17: events::Event = crate::events::Event::Eof;
    let mut event_18: events::Event = crate::events::Event::Eof;
    let mut event_19: events::Event = crate::events::Event::Eof;
    let mut event_20: events::Event = crate::events::Event::Eof;
    let mut event_21: events::Event = crate::events::Event::Eof;
    let mut event_22: events::Event = crate::events::Event::Eof;
    let mut event_23: events::Event = crate::events::Event::Eof;
    let mut event_24: events::Event = crate::events::Event::Eof;
    let mut event_25: events::Event = crate::events::Event::Eof;
    let mut event_26: events::Event = crate::events::Event::Eof;
    let mut event_27: events::Event = crate::events::Event::Eof;
    let mut event_28: events::Event = crate::events::Event::Eof;
    let mut event_29: events::Event = crate::events::Event::Eof;
//    panic!("From RustyUnit with love");
}
}