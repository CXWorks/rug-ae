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
#[timeout(30000)]fn rusty_test_436() {
//    rusty_monitor::set_test_id(436);
    let mut str_0: &str = "I4P";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "DocType";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "Comment";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "Element";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "InvalidAttr";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "ExpectedValue";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "NN";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "InvalidHexadecimal";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "Duplicated";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "mH8OwJOeV7ko8tW9a5o";
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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_440() {
//    rusty_monitor::set_test_id(440);
    let mut usize_0: usize = 4369usize;
    let mut usize_1: usize = 9254usize;
    let mut usize_2: usize = 2usize;
    let mut usize_3: usize = 8usize;
    let mut usize_4: usize = 12usize;
    let mut usize_5: usize = 2usize;
    let mut usize_6: usize = 7usize;
    let mut usize_7: usize = 8usize;
    let mut usize_8: usize = 4usize;
    let mut usize_9: usize = 952usize;
    let mut usize_10: usize = 8usize;
    let mut usize_11: usize = 0usize;
    let mut usize_12: usize = 5usize;
    let mut usize_13: usize = 14usize;
    let mut usize_14: usize = 3usize;
    let mut usize_15: usize = 3408usize;
    let mut usize_16: usize = 2788usize;
    let mut usize_17: usize = 2usize;
    let mut usize_18: usize = 7529usize;
    let mut usize_19: usize = 4451usize;
    let mut usize_20: usize = 4usize;
    let mut usize_21: usize = 4181usize;
    let mut usize_22: usize = 412usize;
    let mut usize_23: usize = 1618usize;
    let mut usize_24: usize = 8894usize;
    let mut usize_25: usize = 7usize;
    let mut usize_26: usize = 5222usize;
    let mut usize_27: usize = 4838usize;
    let mut usize_28: usize = 8843usize;
    let mut usize_29: usize = 3872usize;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_447() {
//    rusty_monitor::set_test_id(447);
    let mut isize_0: isize = 2isize;
    let mut isize_1: isize = 8isize;
    let mut isize_2: isize = 2isize;
    let mut isize_3: isize = 9isize;
    let mut isize_4: isize = 3isize;
    let mut isize_5: isize = 0isize;
    let mut isize_6: isize = 4isize;
    let mut isize_7: isize = 4isize;
    let mut isize_8: isize = 7576isize;
    let mut isize_9: isize = 417isize;
    let mut isize_10: isize = 3isize;
    let mut isize_11: isize = 6isize;
    let mut isize_12: isize = 8isize;
    let mut isize_13: isize = 7isize;
    let mut isize_14: isize = 6isize;
    let mut isize_15: isize = 7isize;
    let mut isize_16: isize = 8isize;
    let mut isize_17: isize = 7isize;
    let mut isize_18: isize = 8isize;
    let mut isize_19: isize = -10462isize;
    let mut isize_20: isize = 6isize;
    let mut isize_21: isize = 1isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_21, isize_20);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_19, isize_18);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_17, isize_16);
    let mut attr_3: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_15, isize_14);
    let mut attr_4: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_13, isize_12);
    let mut attr_5: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_11, isize_10);
    let mut attr_6: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_9, isize_8);
    let mut attr_7: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_7, isize_6);
    let mut attr_8: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_5, isize_4);
    let mut attr_9: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_3, isize_2);
    let mut attr_10: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_211() {
//    rusty_monitor::set_test_id(211);
    let mut usize_0: usize = 2usize;
    let mut usize_1: usize = 0usize;
    let mut usize_2: usize = 12usize;
    let mut usize_3: usize = 8usize;
    let mut usize_4: usize = 5usize;
    let mut usize_5: usize = 14usize;
    let mut usize_6: usize = 9475usize;
    let mut usize_7: usize = 1612usize;
    let mut usize_8: usize = 1052usize;
    let mut usize_9: usize = 2usize;
    let mut usize_10: usize = 5usize;
    let mut usize_11: usize = 4usize;
    let mut usize_12: usize = 14usize;
    let mut usize_13: usize = 5usize;
    let mut usize_14: usize = 6usize;
    let mut usize_15: usize = 2usize;
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
#[timeout(30000)]fn rusty_test_253() {
//    rusty_monitor::set_test_id(253);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_2: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_0: &str = "Utf8";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut option_3: std::option::Option<std::string::String> = std::option::Option::Some(string_0);
    let mut str_1: &str = "Attr::SingleQ";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut option_4: std::option::Option<std::string::String> = std::option::Option::Some(string_1);
    let mut str_2: &str = "InvalidAttr";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut option_5: std::option::Option<std::string::String> = std::option::Option::Some(string_2);
    let mut option_6: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_3: &str = "check_duplicates";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut option_7: std::option::Option<std::string::String> = std::option::Option::Some(string_3);
    let mut str_4: &str = "Start";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut option_8: std::option::Option<std::string::String> = std::option::Option::Some(string_4);
    let mut str_5: &str = "html";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut option_9: std::option::Option<std::string::String> = std::option::Option::Some(string_5);
    let mut error_0: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_9);
    let mut error_1: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_8);
    let mut error_2: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_7);
    let mut error_3: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_6);
    let mut error_4: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_5);
    let mut error_5: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_4);
    let mut error_6: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_3);
    let mut error_7: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_2);
    let mut error_8: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_1);
    let mut error_9: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
//    panic!("From RustyUnit with love");
}
}