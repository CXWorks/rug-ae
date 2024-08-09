//! Error management module

use crate::escape::EscapeError;
use crate::events::attributes::AttrError;
use std::str::Utf8Error;

/// The error type used by this crate.
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(::std::io::Error),
    /// Utf8 error
    Utf8(Utf8Error),
    /// Unexpected End of File
    UnexpectedEof(String),
    /// End event mismatch
    EndEventMismatch {
        /// Expected end event
        expected: String,
        /// Found end event
        found: String,
    },
    /// Unexpected token
    UnexpectedToken(String),
    /// Unexpected <!>
    UnexpectedBang(u8),
    /// Text not found, expected `Event::Text`
    TextNotFound,
    /// `Event::XmlDecl` must start with *version* attribute
    XmlDeclWithoutVersion(Option<String>),
    /// Attribute parsing error
    InvalidAttr(AttrError),
    /// Escape error
    EscapeError(EscapeError),
}

impl From<::std::io::Error> for Error {
    /// Creates a new `Error::Io` from the given error

    fn from(error: ::std::io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<Utf8Error> for Error {
    /// Creates a new `Error::Utf8` from the given error

    fn from(error: Utf8Error) -> Error {
        Error::Utf8(error)
    }
}

impl From<EscapeError> for Error {
    /// Creates a new `Error::EscapeError` from the given error

    fn from(error: EscapeError) -> Error {
        Error::EscapeError(error)
    }
}

impl From<AttrError> for Error {

    fn from(error: AttrError) -> Self {
        Error::InvalidAttr(error)
    }
}

/// A specialized `Result` type where the error is hard-wired to [`Error`].
///
/// [`Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Utf8(e) => write!(f, "UTF8 error: {}", e),
            Error::UnexpectedEof(e) => write!(f, "Unexpected EOF during reading {}", e),
            Error::EndEventMismatch { expected, found } => {
                write!(f, "Expecting </{}> found </{}>", expected, found)
            }
            Error::UnexpectedToken(e) => write!(f, "Unexpected token '{}'", e),
            Error::UnexpectedBang(b) => write!(
                f,
                "Only Comment (`--`), CDATA (`[CDATA[`) and DOCTYPE (`DOCTYPE`) nodes can start with a '!', but symbol `{}` found",
                *b as char
            ),
            Error::TextNotFound => write!(f, "Cannot read text, expecting Event::Text"),
            Error::XmlDeclWithoutVersion(e) => write!(
                f,
                "XmlDecl must start with 'version' attribute, found {:?}",
                e
            ),
            Error::InvalidAttr(e) => write!(f, "error while parsing attribute: {}", e),
            Error::EscapeError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Utf8(e) => Some(e),
            Error::InvalidAttr(e) => Some(e),
            Error::EscapeError(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "serialize")]
pub mod serialize {
    //! A module to handle serde (de)serialization errors

    use super::*;
    use crate::utils::write_byte_string;
    use std::fmt;
    use std::num::{ParseFloatError, ParseIntError};

    /// (De)serialization error
    #[derive(Debug)]
    pub enum DeError {
        /// Serde custom error
        Custom(String),
        /// Xml parsing error
        InvalidXml(Error),
        /// Cannot parse to integer
        InvalidInt(ParseIntError),
        /// Cannot parse to float
        InvalidFloat(ParseFloatError),
        /// Cannot parse specified value to boolean
        InvalidBoolean(String),
        /// This error indicates an error in the [`Deserialize`](serde::Deserialize)
        /// implementation when read a map or a struct: `MapAccess::next_value[_seed]`
        /// was called before `MapAccess::next_key[_seed]`.
        ///
        /// You should check your types, that implements corresponding trait.
        KeyNotRead,
        /// Deserializer encounter a start tag with a specified name when it is
        /// not expecting. This happens when you try to deserialize a primitive
        /// value (numbers, strings, booleans) from an XML element.
        UnexpectedStart(Vec<u8>),
        /// Deserializer encounter an end tag with a specified name when it is
        /// not expecting. Usually that should not be possible, because XML reader
        /// is not able to produce such stream of events that lead to this error.
        ///
        /// If you get this error this likely indicates and error in the `quick_xml`.
        /// Please open an issue at <https://github.com/tafia/quick-xml>, provide
        /// your Rust code and XML input.
        UnexpectedEnd(Vec<u8>),
        /// Unexpected end of file
        UnexpectedEof,
        /// This error indicates that [`deserialize_struct`] was called, but there
        /// is no any XML element in the input. That means that you try to deserialize
        /// a struct not from an XML element.
        ///
        /// [`deserialize_struct`]: serde::de::Deserializer::deserialize_struct
        ExpectedStart,
        /// Unsupported operation
        Unsupported(&'static str),
    }

    impl fmt::Display for DeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DeError::Custom(s) => write!(f, "{}", s),
                DeError::InvalidXml(e) => write!(f, "{}", e),
                DeError::InvalidInt(e) => write!(f, "{}", e),
                DeError::InvalidFloat(e) => write!(f, "{}", e),
                DeError::InvalidBoolean(v) => write!(f, "Invalid boolean value '{}'", v),
                DeError::KeyNotRead => write!(f, "Invalid `Deserialize` implementation: `MapAccess::next_value[_seed]` was called before `MapAccess::next_key[_seed]`"),
                DeError::UnexpectedStart(e) => {
                    f.write_str("Unexpected `Event::Start(")?;
                    write_byte_string(f, &e)?;
                    f.write_str(")`")
                }
                DeError::UnexpectedEnd(e) => {
                    f.write_str("Unexpected `Event::End(")?;
                    write_byte_string(f, &e)?;
                    f.write_str(")`")
                }
                DeError::UnexpectedEof => write!(f, "Unexpected `Event::Eof`"),
                DeError::ExpectedStart => write!(f, "Expecting `Event::Start`"),
                DeError::Unsupported(s) => write!(f, "Unsupported operation {}", s),
            }
        }
    }

    impl ::std::error::Error for DeError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                DeError::InvalidXml(e) => Some(e),
                DeError::InvalidInt(e) => Some(e),
                DeError::InvalidFloat(e) => Some(e),
                _ => None,
            }
        }
    }

    impl serde::de::Error for DeError {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            DeError::Custom(msg.to_string())
        }
    }

    impl serde::ser::Error for DeError {
        fn custom<T: fmt::Display>(msg: T) -> Self {
            DeError::Custom(msg.to_string())
        }
    }

    impl From<Error> for DeError {
        fn from(e: Error) -> Self {
            Self::InvalidXml(e)
        }
    }

    impl From<EscapeError> for DeError {

        fn from(e: EscapeError) -> Self {
            Self::InvalidXml(e.into())
        }
    }

    impl From<ParseIntError> for DeError {
        fn from(e: ParseIntError) -> Self {
            Self::InvalidInt(e)
        }
    }

    impl From<ParseFloatError> for DeError {
        fn from(e: ParseFloatError) -> Self {
            Self::InvalidFloat(e)
        }
    }

    impl From<AttrError> for DeError {

        fn from(e: AttrError) -> Self {
            Self::InvalidXml(e.into())
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_165() {
//    rusty_monitor::set_test_id(165);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_0: &str = "level";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_1: &str = "UnquotedValue";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut option_2: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_2: &str = "ecrS";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut option_3: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_3: &str = "CData";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut option_4: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_4: &str = "value_len";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_4: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut option_5: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_5: &str = "html";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut cow_5: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_474() {
//    rusty_monitor::set_test_id(474);
    let mut u32_0: u32 = 6026u32;
    let mut u32_1: u32 = 8727u32;
    let mut u32_2: u32 = 0u32;
    let mut u32_3: u32 = 10u32;
    let mut u32_4: u32 = 10u32;
    let mut u32_5: u32 = 0u32;
    let mut u32_6: u32 = 3551u32;
    let mut u32_7: u32 = 10u32;
    let mut u32_8: u32 = 3835u32;
    let mut u32_9: u32 = 8677u32;
    let mut u32_10: u32 = 5643u32;
    let mut u32_11: u32 = 0u32;
    let mut u32_12: u32 = 10u32;
    let mut u32_13: u32 = 9057u32;
    let mut u32_14: u32 = 0u32;
    let mut u32_15: u32 = 9960u32;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_15);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_14);
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_13);
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_12);
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_11);
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_10);
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_9);
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_8);
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_7);
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_6);
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_5);
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_4);
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_3);
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_2);
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_1);
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_361() {
//    rusty_monitor::set_test_id(361);
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_182() {
//    rusty_monitor::set_test_id(182);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 9411usize;
    let mut usize_2: usize = 2826usize;
    let mut usize_3: usize = 9451usize;
    let mut usize_4: usize = 8142usize;
    let mut usize_5: usize = 2962usize;
    let mut usize_6: usize = 4usize;
    let mut usize_7: usize = 12usize;
    let mut usize_8: usize = 6587usize;
    let mut usize_9: usize = 7347usize;
    let mut usize_10: usize = 933usize;
    let mut usize_11: usize = 4883usize;
    let mut usize_12: usize = 2usize;
    let mut usize_13: usize = 136usize;
    let mut usize_14: usize = 6usize;
    let mut usize_15: usize = 1313usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_15);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_14);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_13);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_12);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_11);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_10);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_9);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_8);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_7);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_6);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_5);
    let mut attrerror_11: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_4);
    let mut attrerror_12: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_3);
    let mut attrerror_13: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_2);
    let mut attrerror_14: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_1);
    let mut attrerror_15: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_413() {
//    rusty_monitor::set_test_id(413);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_5: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_6: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_7: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_8: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_9: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_10: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_11: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_12: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_13: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_14: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_15: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_15);
    let mut bytesend_1: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_14);
    let mut bytesend_2: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_13);
    let mut bytesend_3: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_12);
    let mut bytesend_4: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_11);
    let mut bytesend_5: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_10);
    let mut bytesend_6: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_9);
    let mut bytesend_7: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_8);
    let mut bytesend_8: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_7);
    let mut bytesend_9: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_6);
    let mut bytesend_10: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_5);
    let mut bytesend_11: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_4);
    let mut bytesend_12: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_3);
    let mut bytesend_13: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_2);
    let mut bytesend_14: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_1);
    let mut bytesend_15: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_520() {
//    rusty_monitor::set_test_id(520);
    let mut char_0: char = '|';
    let mut char_1: char = 'U';
    let mut char_2: char = 'U';
    let mut char_3: char = 'B';
    let mut char_4: char = 'f';
    let mut char_5: char = 'S';
    let mut char_6: char = 's';
    let mut char_7: char = '&';
    let mut char_8: char = 'w';
    let mut char_9: char = 'c';
    let mut char_10: char = 'o';
    let mut char_11: char = 'C';
    let mut char_12: char = 'O';
    let mut char_13: char = 'q';
    let mut char_14: char = 'F';
    let mut char_15: char = '7';
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_15);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_14);
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_13);
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_12);
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_11);
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_10);
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_9);
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_8);
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_7);
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_6);
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_5);
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_4);
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_3);
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_2);
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_1);
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
//    panic!("From RustyUnit with love");
}
}