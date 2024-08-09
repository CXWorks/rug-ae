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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4869() {
    rusty_monitor::set_test_id(4869);
    let mut str_0: &str = "GdvUPI";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 6997u64;
    let mut u64_1: u64 = 7372u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "Un";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_2: u64 = 7487u64;
    let mut u64_3: u64 = 8273u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_2: &str = "XqzgrER";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 5223usize;
    let mut u64_4: u64 = 683u64;
    let mut str_3: &str = "FUB257ySj4p";
    let mut string_0: std::string::String = std::string::String::from(str_3);
    let mut usize_1: usize = 9011usize;
    let mut usize_2: usize = 6301usize;
    let mut str_4: &str = "4fNLhAtlNRs5Dj";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_3: usize = 2593usize;
    let mut usize_4: usize = 9744usize;
    let mut u64_5: u64 = 3179u64;
    let mut u64_6: u64 = 9078u64;
    let mut u64_7: u64 = 6599u64;
    let mut tuple_2: (u64, u64) = (u64_7, u64_6);
    let mut str_5: &str = "8Hxz";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_5: usize = 8349usize;
    let mut usize_6: usize = 5284usize;
    let mut u64_8: u64 = 8032u64;
    let mut u64_9: u64 = 5354u64;
    let mut u64_10: u64 = 5089u64;
    let mut tuple_3: (u64, u64) = (u64_10, u64_9);
    let mut str_6: &str = "8Oo2NuoXa658";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "QP";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut u64_11: u64 = 2656u64;
    let mut u64_12: u64 = 2074u64;
    let mut tuple_4: (u64, u64) = (u64_12, u64_11);
    let mut str_8: &str = "ptMT";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut u64_13: u64 = 6790u64;
    let mut str_9: &str = "j4IeGzPxhVVCHTG";
    let mut string_1: std::string::String = std::string::String::from(str_9);
    let mut usize_7: usize = 9874usize;
    let mut usize_8: usize = 4111usize;
    let mut str_10: &str = "IoFHQcSpyIFxB";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_8, end: usize_7, unit: string_1, value: u64_13};
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: date::Error = crate::date::Error::InvalidDigit;
    let mut error_3: date::Error = crate::date::Error::InvalidDigit;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_1);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::UnknownUnit {start: usize_2, end: usize_1, unit: string_0, value: u64_4};
    let mut error_5: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_2);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}