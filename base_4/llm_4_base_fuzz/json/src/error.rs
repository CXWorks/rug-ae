//! When serializing or deserializing JSON goes wrong.
use crate::io;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
use core::result;
use core::str::FromStr;
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;
/// This type represents all possible errors that can occur when serializing or
/// deserializing JSON data.
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
/// Alias for a `Result` with the error type `serde_json::Error`.
pub type Result<T> = result::Result<T, Error>;
impl Error {
    /// One-based line number at which the error was detected.
    ///
    /// Characters in the first line of the input (before the first newline
    /// character) are in line 1.
    pub fn line(&self) -> usize {
        self.err.line
    }
    /// One-based column number at which the error was detected.
    ///
    /// The first character in the input and any characters immediately
    /// following a newline character are in column 1.
    ///
    /// Note that errors may occur in column 0, for example if a read from an IO
    /// stream fails immediately following a previously read newline character.
    pub fn column(&self) -> usize {
        self.err.column
    }
    /// Categorizes the cause of this error.
    ///
    /// - `Category::Io` - failure to read or write bytes on an IO stream
    /// - `Category::Syntax` - input that is not syntactically valid JSON
    /// - `Category::Data` - input data that is semantically incorrect
    /// - `Category::Eof` - unexpected end of the input data
    pub fn classify(&self) -> Category {
        match self.err.code {
            ErrorCode::Message(_) => Category::Data,
            ErrorCode::Io(_) => Category::Io,
            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue => Category::Eof,
            ErrorCode::ExpectedColon
            | ErrorCode::ExpectedListCommaOrEnd
            | ErrorCode::ExpectedObjectCommaOrEnd
            | ErrorCode::ExpectedSomeIdent
            | ErrorCode::ExpectedSomeValue
            | ErrorCode::InvalidEscape
            | ErrorCode::InvalidNumber
            | ErrorCode::NumberOutOfRange
            | ErrorCode::InvalidUnicodeCodePoint
            | ErrorCode::ControlCharacterWhileParsingString
            | ErrorCode::KeyMustBeAString
            | ErrorCode::LoneLeadingSurrogateInHexEscape
            | ErrorCode::TrailingComma
            | ErrorCode::TrailingCharacters
            | ErrorCode::UnexpectedEndOfHexEscape
            | ErrorCode::RecursionLimitExceeded => Category::Syntax,
        }
    }
    /// Returns true if this error was caused by a failure to read or write
    /// bytes on an IO stream.
    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }
    /// Returns true if this error was caused by input that was not
    /// syntactically valid JSON.
    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }
    /// Returns true if this error was caused by input data that was
    /// semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }
    /// Returns true if this error was caused by prematurely reaching the end of
    /// the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }
}
/// Categorizes the cause of a `serde_json::Error`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    /// The error was caused by a failure to read or write bytes on an IO
    /// stream.
    Io,
    /// The error was caused by input that was not syntactically valid JSON.
    Syntax,
    /// The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    Data,
    /// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    Eof,
}
#[cfg(feature = "std")]
#[allow(clippy::fallible_impl_from)]
impl From<Error> for io::Error {
    /// Convert a `serde_json::Error` into an `io::Error`.
    ///
    /// JSON syntax and data errors are turned into `InvalidData` IO errors.
    /// EOF errors are turned into `UnexpectedEof` IO errors.
    ///
    /// ```
    /// use std::io;
    ///
    /// enum MyError {
    ///     Io(io::Error),
    ///     Json(serde_json::Error),
    /// }
    ///
    /// impl From<serde_json::Error> for MyError {
    ///     fn from(err: serde_json::Error) -> MyError {
    ///         use serde_json::error::Category;
    ///         match err.classify() {
    ///             Category::Io => {
    ///                 MyError::Io(err.into())
    ///             }
    ///             Category::Syntax | Category::Data | Category::Eof => {
    ///                 MyError::Json(err)
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn from(j: Error) -> Self {
        if let ErrorCode::Io(err) = j.err.code {
            err
        } else {
            match j.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => {
                    io::Error::new(io::ErrorKind::InvalidData, j)
                }
                Category::Eof => io::Error::new(io::ErrorKind::UnexpectedEof, j),
            }
        }
    }
}
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),
    /// Some IO error occurred while serializing or deserializing.
    Io(io::Error),
    /// EOF while parsing a list.
    EofWhileParsingList,
    /// EOF while parsing an object.
    EofWhileParsingObject,
    /// EOF while parsing a string.
    EofWhileParsingString,
    /// EOF while parsing a JSON value.
    EofWhileParsingValue,
    /// Expected this character to be a `':'`.
    ExpectedColon,
    /// Expected this character to be either a `','` or a `']'`.
    ExpectedListCommaOrEnd,
    /// Expected this character to be either a `','` or a `'}'`.
    ExpectedObjectCommaOrEnd,
    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,
    /// Expected this character to start a JSON value.
    ExpectedSomeValue,
    /// Invalid hex escape code.
    InvalidEscape,
    /// Invalid number.
    InvalidNumber,
    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,
    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,
    /// Control character found while parsing a string.
    ControlCharacterWhileParsingString,
    /// Object key is not a string.
    KeyMustBeAString,
    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,
    /// JSON has a comma after the last value in an array or map.
    TrailingComma,
    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,
    /// Unexpected end of hex escape.
    UnexpectedEndOfHexEscape,
    /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    RecursionLimitExceeded,
}
impl Error {
    #[cold]
    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }
    #[doc(hidden)]
    #[cold]
    pub fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }
    #[cold]
    pub(crate) fn fix_position<F>(self, f: F) -> Self
    where
        F: FnOnce(ErrorCode) -> Error,
    {
        if self.err.line == 0 { f(self.err.code) } else { self }
    }
}
impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, f),
            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => {
                f.write_str("EOF while parsing an object")
            }
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => f.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => f.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => f.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => {
                f.write_str("invalid unicode code point")
            }
            ErrorCode::ControlCharacterWhileParsingString => {
                f
                    .write_str(
                        "control character (\\u0000-\\u001F) found while parsing a string",
                    )
            }
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                f.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => f.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => {
                f.write_str("unexpected end of hex escape")
            }
            ErrorCode::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
        }
    }
}
impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.err.code {
            ErrorCode::Io(err) => err.source(),
            _ => None,
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}
impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(f, "{} at line {} column {}", self.code, self.line, self.column)
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "Error({:?}, line: {}, column: {})", self.err.code.to_string(), self.err
            .line, self.err.column
        )
    }
}
impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }
    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            Error::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}
impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }
}
fn make_error(mut msg: String) -> Error {
    let (line, column) = parse_line_col(&mut msg).unwrap_or((0, 0));
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg.into_boxed_str()),
            line,
            column,
        }),
    }
}
fn parse_line_col(msg: &mut String) -> Option<(usize, usize)> {
    let start_of_suffix = match msg.rfind(" at line ") {
        Some(index) => index,
        None => return None,
    };
    let start_of_line = start_of_suffix + " at line ".len();
    let mut end_of_line = start_of_line;
    while starts_with_digit(&msg[end_of_line..]) {
        end_of_line += 1;
    }
    if !msg[end_of_line..].starts_with(" column ") {
        return None;
    }
    let start_of_column = end_of_line + " column ".len();
    let mut end_of_column = start_of_column;
    while starts_with_digit(&msg[end_of_column..]) {
        end_of_column += 1;
    }
    if end_of_column < msg.len() {
        return None;
    }
    let line = match usize::from_str(&msg[start_of_line..end_of_line]) {
        Ok(line) => line,
        Err(_) => return None,
    };
    let column = match usize::from_str(&msg[start_of_column..end_of_column]) {
        Ok(column) => column,
        Err(_) => return None,
    };
    msg.truncate(start_of_suffix);
    Some((line, column))
}
fn starts_with_digit(slice: &str) -> bool {
    match slice.as_bytes().first() {
        None => false,
        Some(&byte) => byte >= b'0' && byte <= b'9',
    }
}
#[cfg(test)]
mod tests_llm_16_133_llm_16_133 {
    use super::*;
    use crate::*;
    use serde::de::{self, Error as DeError, Expected};
    use std::fmt;
    struct ExpectedType(String);
    impl Expected for ExpectedType {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }
    #[test]
    fn test_invalid_type_null_expectation() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let unexp = de::Unexpected::Unit;
        let exp = ExpectedType(rug_fuzz_0.to_owned());
        let error = <Error as de::Error>::invalid_type(unexp, &exp);
        debug_assert_eq!(error.to_string(), "invalid type: null, expected integer");
             }
});    }
    #[test]
    fn test_invalid_type_non_null_expectation() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(bool, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let unexp = de::Unexpected::Bool(rug_fuzz_0);
        let exp = ExpectedType(rug_fuzz_1.to_owned());
        let error = <Error as de::Error>::invalid_type(unexp, &exp);
        debug_assert_eq!(
            error.to_string(), "invalid type: boolean `true`, expected string"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_134 {
    use crate::Error;
    use serde::ser::Error as SerError;
    use std::fmt::Display;
    #[test]
    fn custom_error_message_test() {
        let custom_msg = "custom error";
        let error = <Error as SerError>::custom(custom_msg);
        assert_eq!(error.to_string(), custom_msg);
    }
    #[test]
    fn custom_error_display_trait_test() {
        struct TestDisplay;
        impl Display for TestDisplay {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "display trait message")
            }
        }
        let test_display = TestDisplay {};
        let error = <Error as SerError>::custom(test_display);
        assert_eq!(error.to_string(), "display trait message");
    }
    fn make_error<T: Display>(msg: T) -> Error {
        Error::custom(msg.to_string())
    }
}
#[cfg(test)]
mod tests_llm_16_135 {
    use super::*;
    use crate::*;
    use crate::error::Error as SerdeError;
    use std::error::Error as StdError;
    use std::io;
    #[test]
    fn test_error_source_with_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io_error = io::Error::new(io::ErrorKind::Other, rug_fuzz_0);
        let error = SerdeError::io(io_error);
        let source = error.source();
        debug_assert!(source.is_some());
        let downcasted = source.unwrap().downcast_ref::<io::Error>().unwrap();
        debug_assert_eq!(downcasted.kind(), io::ErrorKind::Other);
        debug_assert_eq!(downcasted.to_string(), "simulate I/O failure");
             }
});    }
    #[test]
    fn test_error_source_with_non_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error = SerdeError::syntax(
            crate::error::ErrorCode::ExpectedSomeValue,
            rug_fuzz_0,
            rug_fuzz_1,
        );
        let source = error.source();
        debug_assert!(source.is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_483_llm_16_483 {
    use crate::error::{Category, ErrorCode, Error};
    use std::io::{self, ErrorKind};
    use std::error::Error as StdError;
    #[test]
    fn test_io_error_from_json_error_io() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io_error = io::Error::new(ErrorKind::Other, rug_fuzz_0);
        let json_error = Error::io(io_error);
        let converted_io_error: io::Error = json_error.into();
        debug_assert_eq!(converted_io_error.kind(), ErrorKind::Other);
        debug_assert_eq!(converted_io_error.to_string(), "io error");
             }
});    }
    #[test]
    fn test_io_error_from_json_error_syntax() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_error = Error::syntax(
            ErrorCode::ExpectedSomeValue,
            rug_fuzz_0,
            rug_fuzz_1,
        );
        let converted_io_error: io::Error = json_error.into();
        debug_assert_eq!(converted_io_error.kind(), ErrorKind::InvalidData);
        debug_assert!(converted_io_error.source().is_some());
             }
});    }
    #[test]
    fn test_io_error_from_json_error_data() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_error = Error::syntax(
            ErrorCode::Message(rug_fuzz_0.into()),
            rug_fuzz_1,
            rug_fuzz_2,
        );
        let converted_io_error: io::Error = json_error.into();
        debug_assert_eq!(converted_io_error.kind(), ErrorKind::InvalidData);
        debug_assert!(converted_io_error.source().is_some());
        debug_assert_eq!(converted_io_error.to_string(), "data error");
             }
});    }
    #[test]
    fn test_io_error_from_json_error_eof() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let json_error = Error::syntax(
            ErrorCode::EofWhileParsingValue,
            rug_fuzz_0,
            rug_fuzz_1,
        );
        let converted_io_error: io::Error = json_error.into();
        debug_assert_eq!(converted_io_error.kind(), ErrorKind::UnexpectedEof);
        debug_assert!(converted_io_error.source().is_some());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_484 {
    use crate::error::{Error, ErrorCode, Category};
    #[test]
    fn test_classify_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error");
        let error = Error::io(io_error);
        assert_eq!(error.classify(), Category::Io);
    }
    #[test]
    fn test_classify_syntax_error() {
        let syntax_error = Error::syntax(ErrorCode::ExpectedColon, 1, 10);
        assert_eq!(syntax_error.classify(), Category::Syntax);
    }
    #[test]
    fn test_classify_data_error() {
        let data_error = Error::syntax(ErrorCode::Message("data error".into()), 1, 10);
        assert_eq!(data_error.classify(), Category::Data);
    }
    #[test]
    fn test_classify_eof_error() {
        let eof_error = Error::syntax(ErrorCode::EofWhileParsingValue, 1, 10);
        assert_eq!(eof_error.classify(), Category::Eof);
    }
    #[test]
    fn test_classify_syntax_error_unexpected_end_of_hex_escape() {
        let error = Error::syntax(ErrorCode::UnexpectedEndOfHexEscape, 2, 4);
        assert_eq!(error.classify(), Category::Syntax);
    }
    fn make_error(message: String) -> Error {
        Error::syntax(ErrorCode::Message(message.into_boxed_str()), 1, 1)
    }
}
#[cfg(test)]
mod tests_llm_16_486 {
    use crate::error::{Error, ErrorCode};
    use std::io;
    #[test]
    fn test_fix_position_does_not_change_positioned_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error = Error::syntax(
            ErrorCode::Message(rug_fuzz_0.into()),
            rug_fuzz_1,
            rug_fuzz_2,
        );
        let fix = |code: ErrorCode| Error::syntax(code, rug_fuzz_3, rug_fuzz_4);
        let fixed_error = error.fix_position(fix);
        debug_assert_eq!(fixed_error.line(), 2);
        debug_assert_eq!(fixed_error.column(), 5);
             }
});    }
    #[test]
    fn test_fix_position_applies_fix_for_error_without_position() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error = Error::syntax(
            ErrorCode::Message(rug_fuzz_0.into()),
            rug_fuzz_1,
            rug_fuzz_2,
        );
        let fix = |code: ErrorCode| Error::syntax(code, rug_fuzz_3, rug_fuzz_4);
        let fixed_error = error.fix_position(fix);
        debug_assert_eq!(fixed_error.line(), 1);
        debug_assert_eq!(fixed_error.column(), 1);
             }
});    }
    #[test]
    fn test_fix_position_for_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io_error = io::Error::new(io::ErrorKind::Other, rug_fuzz_0);
        let error = Error::io(io_error);
        let fix = |code: ErrorCode| Error::syntax(code, rug_fuzz_1, rug_fuzz_2);
        let fixed_error = error.fix_position(fix);
        debug_assert_eq!(fixed_error.line(), 5);
        debug_assert_eq!(fixed_error.column(), 10);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_489_llm_16_489 {
    use crate::error::{Error, ErrorCode, Category};
    fn create_eof_error() -> Error {
        Error::syntax(ErrorCode::EofWhileParsingValue, 1, 1)
    }
    fn create_non_eof_error() -> Error {
        Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1)
    }
    #[test]
    fn test_is_eof_on_eof_error() {
        let eof_error = create_eof_error();
        assert!(eof_error.is_eof());
    }
    #[test]
    fn test_is_eof_on_non_eof_error() {
        let non_eof_error = create_non_eof_error();
        assert!(! non_eof_error.is_eof());
    }
}
#[cfg(test)]
mod tests_llm_16_490 {
    use super::*;
    use crate::*;
    use std::io;
    use crate::error::{Error, ErrorCode, Category};
    #[test]
    fn test_is_io_for_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io_error = io::Error::new(io::ErrorKind::Other, rug_fuzz_0);
        let error = Error::io(io_error);
        debug_assert!(error.is_io());
             }
});    }
    #[test]
    fn test_is_io_for_non_io_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error = Error::syntax(ErrorCode::ExpectedSomeValue, rug_fuzz_0, rug_fuzz_1);
        debug_assert!(! error.is_io());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_491 {
    use crate::error::{Category, ErrorCode, Error};
    #[test]
    fn test_is_syntax_true() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let syntax_error = Error::syntax(
            ErrorCode::TrailingCharacters,
            rug_fuzz_0,
            rug_fuzz_1,
        );
        debug_assert!(syntax_error.is_syntax());
             }
});    }
    #[test]
    fn test_is_syntax_false() {
        let _rug_st_tests_llm_16_491_rrrruuuugggg_test_is_syntax_false = 0;
        let io_error = Error::io(std::io::Error::from(std::io::ErrorKind::Other));
        debug_assert!(! io_error.is_syntax());
        let _rug_ed_tests_llm_16_491_rrrruuuugggg_test_is_syntax_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_492 {
    use super::*;
    use crate::*;
    use crate::error::{Error, ErrorCode};
    #[test]
    fn test_error_line() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let error = Error::syntax(ErrorCode::ExpectedSomeValue, rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(error.line(), 5);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_493_llm_16_493 {
    use super::*;
    use crate::*;
    use crate::error::{Error, ErrorCode};
    use crate::error::Category;
    use std::fmt;
    use std::io;
    #[test]
    fn test_syntax_error_creation() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let message = rug_fuzz_0;
        let line = rug_fuzz_1;
        let column = rug_fuzz_2;
        let error_code = ErrorCode::Message(message.into());
        let error = Error::syntax(error_code, line, column);
        debug_assert_eq!(error.line(), line);
        debug_assert_eq!(error.column(), column);
        debug_assert!(error.is_syntax(), "Error should be a syntax error");
             }
});    }
    #[test]
    fn test_syntax_error_display_impl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let message = rug_fuzz_0;
        let line = rug_fuzz_1;
        let column = rug_fuzz_2;
        let error_code = ErrorCode::Message(message.into());
        let error = Error::syntax(error_code, line, column);
        let error_message = format!("{}", error);
        debug_assert!(error_message.contains(message));
        debug_assert!(error_message.contains(& line.to_string()));
        debug_assert!(error_message.contains(& column.to_string()));
             }
});    }
    #[test]
    fn test_syntax_error_debug_impl() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let message = rug_fuzz_0;
        let line = rug_fuzz_1;
        let column = rug_fuzz_2;
        let error_code = ErrorCode::Message(message.into());
        let error = Error::syntax(error_code, line, column);
        let error_debug = format!("{:?}", error);
        debug_assert!(error_debug.contains(message));
        debug_assert!(error_debug.contains(& line.to_string()));
        debug_assert!(error_debug.contains(& column.to_string()));
             }
});    }
}
