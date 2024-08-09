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
#[timeout(30000)]fn rusty_test_365() {
//    rusty_monitor::set_test_id(365);
    let mut bool_0: bool = false;
    let mut i32_0: i32 = 9114i32;
    let mut bool_1: bool = false;
    let mut i32_1: i32 = 1i32;
    let mut bool_2: bool = false;
    let mut i32_2: i32 = 4i32;
    let mut bool_3: bool = true;
    let mut i32_3: i32 = -884i32;
    let mut bool_4: bool = false;
    let mut i32_4: i32 = 1i32;
    let mut bool_5: bool = true;
    let mut i32_5: i32 = 152i32;
    let mut bool_6: bool = true;
    let mut i32_6: i32 = 1i32;
    let mut bool_7: bool = true;
    let mut i32_7: i32 = 7907i32;
    let mut bool_8: bool = false;
    let mut i32_8: i32 = 0i32;
    let mut bool_9: bool = false;
    let mut i32_9: i32 = 1i32;
    let mut bool_10: bool = false;
    let mut i32_10: i32 = 4i32;
    let mut bool_11: bool = true;
    let mut i32_11: i32 = 1i32;
    let mut bool_12: bool = true;
    let mut i32_12: i32 = 12902i32;
    let mut bool_13: bool = true;
    let mut i32_13: i32 = 4i32;
    let mut bool_14: bool = false;
    let mut i32_14: i32 = 4266i32;
    let mut bool_15: bool = false;
    let mut i32_15: i32 = 1i32;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_487() {
//    rusty_monitor::set_test_id(487);
    let mut u32_0: u32 = 10u32;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
    let mut str_0: &str = "Decoder";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut result_0: std::result::Result<std::string::String, std::string::FromUtf8Error> = std::result::Result::Ok(string_0);
    let mut char_0: char = 'L';
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut char_1: char = 'b';
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_1);
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut char_2: char = '0';
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_2);
    let mut char_3: char = '/';
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_3);
    let mut char_4: char = 'k';
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_4);
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut str_1: &str = "Eof";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut result_1: std::result::Result<std::string::String, std::string::FromUtf8Error> = std::result::Result::Ok(string_1);
    let mut error_0: errors::Error = crate::errors::Error::EscapeError(escapeerror_9);
    let mut error_1: errors::Error = crate::errors::Error::EscapeError(escapeerror_8);
    let mut error_2: errors::Error = crate::errors::Error::EscapeError(escapeerror_7);
    let mut error_3: errors::Error = crate::errors::Error::EscapeError(escapeerror_6);
    let mut error_4: errors::Error = crate::errors::Error::EscapeError(escapeerror_5);
    let mut error_5: errors::Error = crate::errors::Error::EscapeError(escapeerror_4);
    let mut error_6: errors::Error = crate::errors::Error::EscapeError(escapeerror_3);
    let mut error_7: errors::Error = crate::errors::Error::EscapeError(escapeerror_2);
    let mut error_8: errors::Error = crate::errors::Error::EscapeError(escapeerror_1);
    let mut error_9: errors::Error = crate::errors::Error::EscapeError(escapeerror_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_304() {
//    rusty_monitor::set_test_id(304);
    let mut str_0: &str = "Fr4MlxS2Fb";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "EndEventMismatch";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "'";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "SkipEqValue";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "XmlDecl";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "CData";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "EntityWithNull";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "UnexpectedBang";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "vdGwUyKyC";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "UnexpectedToken";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "nv09yDqCAuBncSd4q";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut str_11: &str = "xTF0Gln";
    let mut string_11: std::string::String = std::string::String::from(str_11);
    let mut str_12: &str = "pzKcId";
    let mut string_12: std::string::String = std::string::String::from(str_12);
    let mut str_13: &str = "rgW8Hzh";
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
#[timeout(30000)]fn rusty_test_531() {
//    rusty_monitor::set_test_id(531);
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 771usize;
    let mut usize_2: usize = 8usize;
    let mut usize_3: usize = 8120usize;
    let mut usize_4: usize = 8128usize;
    let mut usize_5: usize = 2650usize;
    let mut usize_6: usize = 4924usize;
    let mut usize_7: usize = 5688usize;
    let mut usize_8: usize = 0usize;
    let mut usize_9: usize = 5usize;
    let mut usize_10: usize = 1usize;
    let mut usize_11: usize = 0usize;
    let mut usize_12: usize = 5024usize;
    let mut usize_13: usize = 9406usize;
    let mut usize_14: usize = 14usize;
    let mut usize_15: usize = 6usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_15);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_14);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_13);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_12);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_11);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_10);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_9);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_8);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_7);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_6);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_5);
    let mut attrerror_11: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_4);
    let mut attrerror_12: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_3);
    let mut attrerror_13: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_2);
    let mut attrerror_14: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_1);
    let mut attrerror_15: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_360() {
//    rusty_monitor::set_test_id(360);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    let mut bool_0: bool = true;
    let mut i32_0: i32 = 4i32;
    let mut vec_1: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_1_ref_0: &mut std::vec::Vec<u8> = &mut vec_1;
    let mut bool_1: bool = true;
    let mut i32_1: i32 = 4i32;
    let mut vec_2: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_2_ref_0: &mut std::vec::Vec<u8> = &mut vec_2;
    let mut bool_2: bool = true;
    let mut i32_2: i32 = -19718i32;
    let mut vec_3: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_3_ref_0: &mut std::vec::Vec<u8> = &mut vec_3;
    let mut bool_3: bool = false;
    let mut i32_3: i32 = 1i32;
    let mut vec_4: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_4_ref_0: &mut std::vec::Vec<u8> = &mut vec_4;
    let mut bool_4: bool = true;
    let mut i32_4: i32 = 4i32;
    let mut vec_5: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_5_ref_0: &mut std::vec::Vec<u8> = &mut vec_5;
    let mut bool_5: bool = false;
    let mut i32_5: i32 = 0i32;
    let mut vec_6: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_6_ref_0: &mut std::vec::Vec<u8> = &mut vec_6;
    let mut bool_6: bool = true;
    let mut i32_6: i32 = -596i32;
    let mut vec_7: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_7_ref_0: &mut std::vec::Vec<u8> = &mut vec_7;
    let mut bool_7: bool = true;
    let mut i32_7: i32 = 4i32;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_534() {
//    rusty_monitor::set_test_id(534);
    let mut str_0: &str = "Io";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "Element";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "Sx3CpNDUItjtc8yQ";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "h6J1xg";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "IterState";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "SkipEqValue";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "2KAxDy8";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "prefix_len";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "--";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "Decl";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "xcl2";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut error_0: errors::Error = crate::errors::Error::UnexpectedEof(string_10);
    let mut error_1: errors::Error = crate::errors::Error::UnexpectedEof(string_9);
    let mut error_2: errors::Error = crate::errors::Error::UnexpectedEof(string_8);
    let mut error_3: errors::Error = crate::errors::Error::UnexpectedEof(string_7);
    let mut error_4: errors::Error = crate::errors::Error::UnexpectedEof(string_6);
    let mut error_5: errors::Error = crate::errors::Error::UnexpectedEof(string_5);
    let mut error_6: errors::Error = crate::errors::Error::UnexpectedEof(string_4);
    let mut error_7: errors::Error = crate::errors::Error::UnexpectedEof(string_3);
    let mut error_8: errors::Error = crate::errors::Error::UnexpectedEof(string_2);
    let mut error_9: errors::Error = crate::errors::Error::UnexpectedEof(string_1);
    let mut error_10: errors::Error = crate::errors::Error::UnexpectedEof(string_0);
//    panic!("From RustyUnit with love");
}
}