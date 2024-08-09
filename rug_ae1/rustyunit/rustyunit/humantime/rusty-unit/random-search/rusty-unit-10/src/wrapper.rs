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
fn rusty_test_4953() {
    rusty_monitor::set_test_id(4953);
    let mut u64_0: u64 = 5066u64;
    let mut u64_1: u64 = 295u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "NbGcq0DFFA0A";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 169usize;
    let mut usize_1: usize = 6471usize;
    let mut u64_2: u64 = 2097u64;
    let mut u64_3: u64 = 2203u64;
    let mut u64_4: u64 = 3010u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_1: &str = "x";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_5: u64 = 1454u64;
    let mut u64_6: u64 = 8632u64;
    let mut tuple_2: (u64, u64) = (u64_6, u64_5);
    let mut str_2: &str = "mci1oi";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "KVivNUW2lbSYN5KExoy";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_7: u64 = 9843u64;
    let mut str_4: &str = "gBQNKUrz3IlAD";
    let mut string_0: std::string::String = std::string::String::from(str_4);
    let mut usize_2: usize = 6758usize;
    let mut usize_3: usize = 6263usize;
    let mut u64_8: u64 = 1376u64;
    let mut u64_9: u64 = 4539u64;
    let mut tuple_3: (u64, u64) = (u64_9, u64_8);
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_10: u64 = 2396u64;
    let mut str_6: &str = "Zv0NWaR3olVqghhW";
    let mut string_1: std::string::String = std::string::String::from(str_6);
    let mut usize_4: usize = 9908usize;
    let mut usize_5: usize = 3556usize;
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut error_1: duration::Error = crate::duration::Error::UnknownUnit {start: usize_5, end: usize_4, unit: string_1, value: u64_10};
    let mut error_2: duration::Error = crate::duration::Error::UnknownUnit {start: usize_3, end: usize_2, unit: string_0, value: u64_7};
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    panic!("From RustyUnit with love");
}
}