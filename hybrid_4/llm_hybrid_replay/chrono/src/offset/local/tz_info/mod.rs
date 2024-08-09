#![deny(missing_docs)]
#![allow(dead_code)]
#![warn(unreachable_pub)]
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::time::SystemTimeError;
use std::{error, fmt, io};
mod timezone;
pub(crate) use timezone::TimeZone;
mod parser;
mod rule;
/// Unified error type for everything in the crate
#[derive(Debug)]
pub(crate) enum Error {
    /// Date time error
    DateTime(&'static str),
    /// Local time type search error
    FindLocalTimeType(&'static str),
    /// Local time type error
    LocalTimeType(&'static str),
    /// Invalid slice for integer conversion
    InvalidSlice(&'static str),
    /// Invalid Tzif file
    InvalidTzFile(&'static str),
    /// Invalid TZ string
    InvalidTzString(&'static str),
    /// I/O error
    Io(io::Error),
    /// Out of range error
    OutOfRange(&'static str),
    /// Integer parsing error
    ParseInt(ParseIntError),
    /// Date time projection error
    ProjectDateTime(&'static str),
    /// System time error
    SystemTime(SystemTimeError),
    /// Time zone error
    TimeZone(&'static str),
    /// Transition rule error
    TransitionRule(&'static str),
    /// Unsupported Tzif file
    UnsupportedTzFile(&'static str),
    /// Unsupported TZ string
    UnsupportedTzString(&'static str),
    /// UTF-8 error
    Utf8(Utf8Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            DateTime(error) => write!(f, "invalid date time: {}", error),
            FindLocalTimeType(error) => error.fmt(f),
            LocalTimeType(error) => write!(f, "invalid local time type: {}", error),
            InvalidSlice(error) => error.fmt(f),
            InvalidTzString(error) => write!(f, "invalid TZ string: {}", error),
            InvalidTzFile(error) => error.fmt(f),
            Io(error) => error.fmt(f),
            OutOfRange(error) => error.fmt(f),
            ParseInt(error) => error.fmt(f),
            ProjectDateTime(error) => error.fmt(f),
            SystemTime(error) => error.fmt(f),
            TransitionRule(error) => write!(f, "invalid transition rule: {}", error),
            TimeZone(error) => write!(f, "invalid time zone: {}", error),
            UnsupportedTzFile(error) => error.fmt(f),
            UnsupportedTzString(error) => write!(f, "unsupported TZ string: {}", error),
            Utf8(error) => error.fmt(f),
        }
    }
}
impl error::Error for Error {}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}
impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Error::SystemTime(error)
    }
}
impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8(error)
    }
}
/// Number of hours in one day
const HOURS_PER_DAY: i64 = 24;
/// Number of seconds in one hour
const SECONDS_PER_HOUR: i64 = 3600;
/// Number of seconds in one day
const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * HOURS_PER_DAY;
/// Number of days in one week
const DAYS_PER_WEEK: i64 = 7;
/// Month days in a normal year
const DAY_IN_MONTHS_NORMAL_YEAR: [i64; 12] = [
    31,
    28,
    31,
    30,
    31,
    30,
    31,
    31,
    30,
    31,
    30,
    31,
];
/// Cumulated month days in a normal year
const CUMUL_DAY_IN_MONTHS_NORMAL_YEAR: [i64; 12] = [
    0,
    31,
    59,
    90,
    120,
    151,
    181,
    212,
    243,
    273,
    304,
    334,
];
#[cfg(test)]
mod tests_llm_16_186 {
    use std::io::{self, ErrorKind};
    use super::*;
    use crate::*;
    #[test]
    fn test_from_io_error_to_tz_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let io_error = io::Error::new(ErrorKind::NotFound, rug_fuzz_0);
        let tz_error: Error = Error::from(io_error);
        match tz_error {
            Error::Io(e) => debug_assert_eq!(e.kind(), ErrorKind::NotFound),
            _ => panic!("Expected Error::Io variant"),
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_188_llm_16_188 {
    use super::*;
    use crate::*;
    use std::str::Utf8Error;
    use std::string::FromUtf8Error;
    #[test]
    fn test_from_utf8_error() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let invalid_utf8: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let result = std::str::from_utf8(invalid_utf8);
        debug_assert!(result.is_err());
        if let Err(utf8_error) = result {
            let error: Error = Error::from(utf8_error);
            match error {
                Error::Utf8(e) => debug_assert_eq!(e, utf8_error),
                _ => panic!("Error type does not match the expected Utf8 error variant"),
            }
        } else {
            panic!("Failed to create Utf8Error");
        }
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_187 {
    use super::*;
    use std::num::ParseIntError;
    use crate::offset::local::tz_info::Error;
    use std::convert::From;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: ParseIntError = rug_fuzz_0.parse::<i32>().unwrap_err();
        let _result: Error = Error::from(p0);
             }
}
}
}    }
}
