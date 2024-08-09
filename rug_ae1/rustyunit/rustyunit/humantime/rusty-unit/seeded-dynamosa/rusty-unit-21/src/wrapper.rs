use std::str::FromStr;
use std::ops::Deref;
use std::fmt;
use std::time::{Duration as StdDuration, SystemTime};

use crate::duration::{self, parse_duration, format_duration};
use crate::date::{self, parse_rfc3339_weak, format_rfc3339};

/// A wrapper for duration that has `FromStr` implementation
///
/// This is useful if you want to use it somewhere where `FromStr` is
/// expected.
///
/// See `parse_duration` for the description of the format.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// let x: Duration;
/// x = "12h 5min 2ns".parse::<humantime::Duration>().unwrap().into();
/// assert_eq!(x, Duration::new(12*3600 + 5*60, 2))
/// ```
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Duration(StdDuration);

/// A wrapper for SystemTime that has `FromStr` implementation
///
/// This is useful if you want to use it somewhere where `FromStr` is
/// expected.
///
/// See `parse_rfc3339_weak` for the description of the format. The "weak"
/// format is used as it's more pemissive for human input as this is the
/// expected use of the type (e.g. command-line parsing).
///
/// # Example
///
/// ```
/// use std::time::SystemTime;
/// let x: SystemTime;
/// x = "2018-02-16T00:31:37Z".parse::<humantime::Timestamp>().unwrap().into();
/// assert_eq!(humantime::format_rfc3339(x).to_string(), "2018-02-16T00:31:37Z");
/// ```
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Timestamp(SystemTime);

impl AsRef<StdDuration> for Duration {
    fn as_ref(&self) -> &StdDuration { &self.0 }
}

impl Deref for Duration {
    type Target = StdDuration;
    fn deref(&self) -> &StdDuration { &self.0 }
}

impl Into<StdDuration> for Duration {
    fn into(self) -> StdDuration { self.0 }
}

impl From<StdDuration> for Duration {
    fn from(dur: StdDuration) -> Duration { Duration(dur) }
}

impl FromStr for Duration {
    type Err = duration::Error;
    fn from_str(s: &str) -> Result<Duration, Self::Err> {
        parse_duration(s).map(Duration)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_duration(self.0).fmt(f)
    }
}

impl AsRef<SystemTime> for Timestamp {
    fn as_ref(&self) -> &SystemTime { &self.0 }
}

impl Deref for Timestamp {
    type Target = SystemTime;
    fn deref(&self) -> &SystemTime { &self.0 }
}

impl Into<SystemTime> for Timestamp {
    fn into(self) -> SystemTime { self.0 }
}

impl From<SystemTime> for Timestamp {
    fn from(dur: SystemTime) -> Timestamp { Timestamp(dur) }
}

impl FromStr for Timestamp {
    type Err = date::Error;
    fn from_str(s: &str) -> Result<Timestamp, Self::Err> {
        parse_rfc3339_weak(s).map(Timestamp)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_rfc3339(self.0).fmt(f)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8558() {
//    rusty_monitor::set_test_id(8558);
    let mut str_0: &str = "Micros";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 24usize;
    let mut str_2: &str = "NumberExpected";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 14usize;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = " ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Smart";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_1: bool = false;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut str_6: &str = "NbpfqFaOokB52yyoGs";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "0XMKXbWu7z";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut usize_2: usize = 11usize;
    let mut str_8: &str = "0s";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut usize_3: usize = 29usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8568() {
//    rusty_monitor::set_test_id(8568);
    let mut str_0: &str = "LC";
    let mut str_1: &str = "7zu4B37zzFn";
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 726u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_0: usize = 24usize;
    let mut str_2: &str = "NumberExpected";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = " ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Smart";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_6: &str = "NbpfqFaOokB52yyoGs";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_1: date::Error = crate::date::Error::InvalidFormat;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8580() {
//    rusty_monitor::set_test_id(8580);
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 24usize;
    let mut str_1: &str = "NumberExpected";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 14usize;
    let mut str_2: &str = "2018-02-14T00:28:07Z";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = " ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Smart";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_5: &str = "NbpfqFaOokB52yyoGs";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "0XMKXbWu7z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut usize_2: usize = 11usize;
    let mut str_7: &str = "0s";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut usize_3: usize = 29usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8585() {
//    rusty_monitor::set_test_id(8585);
    let mut str_0: &str = "start";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut usize_0: usize = 2usize;
    let mut bool_0: bool = false;
    let mut str_1: &str = "us";
    let mut u64_0: u64 = 7263u64;
    let mut str_2: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 24usize;
    let mut usize_2: usize = 14usize;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = " ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "Smart";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_6: &str = "NbpfqFaOokB52yyoGs";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "0XMKXbWu7z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut usize_3: usize = 11usize;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_2);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut error_3: duration::Error = crate::duration::Error::UnknownUnit {start: usize_0, end: usize_0, unit: string_0, value: u64_0};
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8589() {
//    rusty_monitor::set_test_id(8589);
    let mut str_0: &str = "UnknownUnit";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "Micros";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "NumberExpected";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = " ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bool_1: bool = false;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut str_7: &str = "NbpfqFaOokB52yyoGs";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "0XMKXbWu7z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_8_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut error_2: duration::Error = crate::duration::Error::Empty;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8592() {
//    rusty_monitor::set_test_id(8592);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 3usize;
    let mut usize_1: usize = 0usize;
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "NumberExpected";
    let mut str_2: &str = " ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "Smart";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_4: &str = "NbpfqFaOokB52yyoGs";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "0XMKXbWu7z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_2: usize = 11usize;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_3: usize = 29usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8614() {
//    rusty_monitor::set_test_id(8614);
    let mut u64_0: u64 = 31u64;
    let mut u64_1: u64 = 2195u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "mUK2F7irTI5cgMe4V";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_2: u64 = 120u64;
    let mut str_1: &str = "rZe4ppqs";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut usize_0: usize = 28usize;
    let mut usize_1: usize = 11usize;
    let mut u64_3: u64 = 7263u64;
    let mut u64_4: u64 = 1000000000u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_2: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "NumberExpected";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_2: usize = 14usize;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = " ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_7: &str = "NbpfqFaOokB52yyoGs";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "0XMKXbWu7z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut usize_3: usize = 11usize;
    let mut str_9: &str = "0s";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut usize_4: usize = 29usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_4);
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_5: duration::Error = crate::duration::Error::UnknownUnit {start: usize_1, end: usize_0, unit: string_0, value: u64_2};
    let mut error_6: duration::Error = crate::duration::Error::NumberOverflow;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8623() {
//    rusty_monitor::set_test_id(8623);
    let mut str_0: &str = "day";
    let mut bool_0: bool = false;
    let mut str_1: &str = "2018-02-14T00:28:07";
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 14usize;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = " ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_5: &str = "Smart";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "0XMKXbWu7z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 11usize;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8626() {
//    rusty_monitor::set_test_id(8626);
    let mut str_0: &str = "InvalidFormat";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut bool_1: bool = true;
    let mut str_1: &str = "0w1jq0R4XTU0fP8e";
    let mut str_2: &str = "Duration";
    let mut u64_0: u64 = 7263u64;
    let mut u64_1: u64 = 1000000000u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_3: &str = "Of2uJKxYJ4BpIjHQ0";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 24usize;
    let mut str_4: &str = "NumberExpected";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_1: usize = 14usize;
    let mut str_4_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 14usize;
    let mut str_5: &str = "2018-02-14T00:28:07Z";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_6: &str = " ";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut str_8: &str = "NbpfqFaOokB52yyoGs";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_9: &str = "0XMKXbWu7z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut usize_2: usize = 11usize;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut usize_3: usize = 29usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_1);
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_3);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut error_4: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}
}