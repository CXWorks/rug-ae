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
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut usize_0: usize = 5640usize;
    let mut usize_1: usize = 8513usize;
    let mut u64_0: u64 = 1072u64;
    let mut u64_1: u64 = 2106u64;
    let mut u64_2: u64 = 2273u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_0: &str = "e0hkzJmwmyX358";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_3: u64 = 5088u64;
    let mut str_1: &str = "VY";
    let mut string_0: std::string::String = std::string::String::from(str_1);
    let mut usize_2: usize = 495usize;
    let mut usize_3: usize = 6876usize;
    let mut str_2: &str = "dlWdTY9iYAi";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_4: usize = 7111usize;
    let mut u64_4: u64 = 9021u64;
    let mut u64_5: u64 = 8750u64;
    let mut tuple_1: (u64, u64) = (u64_5, u64_4);
    let mut str_3: &str = "d";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_6: u64 = 5576u64;
    let mut u64_7: u64 = 5917u64;
    let mut tuple_2: (u64, u64) = (u64_7, u64_6);
    let mut str_4: &str = "H1xyVyUfWg5IGT";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "U";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_0: date::Error = crate::date::Error::InvalidFormat;
    let mut error_1: duration::Error = crate::duration::Error::InvalidCharacter(usize_4);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::UnknownUnit {start: usize_3, end: usize_2, unit: string_0, value: u64_3};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut str_0: &str = "mA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 3201u32;
    let mut str_1: &str = "zyUc";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_0: u64 = 5512u64;
    let mut u64_1: u64 = 2164u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "THynKym72";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "hGg8yNXO4z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 7762usize;
    let mut usize_1: usize = 865usize;
    let mut u64_2: u64 = 2285u64;
    let mut u64_3: u64 = 6487u64;
    let mut u64_4: u64 = 476u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_4: &str = "P88JYY";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "7hpi";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "OT110LY";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::Empty;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_2: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    panic!("From RustyUnit with love");
}
}