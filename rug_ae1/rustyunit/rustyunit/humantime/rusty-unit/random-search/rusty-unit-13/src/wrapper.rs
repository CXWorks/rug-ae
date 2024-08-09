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
fn rusty_test_4965() {
    rusty_monitor::set_test_id(4965);
    let mut u64_0: u64 = 5948u64;
    let mut str_0: &str = "f4";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut usize_0: usize = 5088usize;
    let mut usize_1: usize = 1745usize;
    let mut u64_1: u64 = 7893u64;
    let mut str_1: &str = "2fhEerF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_2: &str = "KRjqfi6kwgY";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_2: usize = 5755usize;
    let mut usize_3: usize = 651usize;
    let mut u64_2: u64 = 6026u64;
    let mut u64_3: u64 = 1323u64;
    let mut u64_4: u64 = 3168u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut str_3: &str = "9ZRJdJy";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_5: u64 = 5469u64;
    let mut u64_6: u64 = 3311u64;
    let mut tuple_1: (u64, u64) = (u64_6, u64_5);
    let mut str_4: &str = "5M7NffvCoE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_7: u64 = 2411u64;
    let mut u64_8: u64 = 603u64;
    let mut tuple_2: (u64, u64) = (u64_8, u64_7);
    let mut str_5: &str = "nEW4BXvwhqX";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_1, end: usize_0, unit: string_0, value: u64_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4991() {
    rusty_monitor::set_test_id(4991);
    let mut usize_0: usize = 1162usize;
    let mut usize_1: usize = 6946usize;
    let mut u64_0: u64 = 3820u64;
    let mut u64_1: u64 = 9527u64;
    let mut u64_2: u64 = 8913u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_0: &str = "LdLi6JV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 1852u32;
    let mut str_1: &str = "kMXVxbCP2hZSqTfR";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_3: u64 = 1357u64;
    let mut u64_4: u64 = 647u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_2: &str = "WKWRmwlzYFDdTxLH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_2: usize = 697usize;
    let mut usize_3: usize = 8782usize;
    let mut u64_5: u64 = 7446u64;
    let mut u64_6: u64 = 8217u64;
    let mut u64_7: u64 = 4030u64;
    let mut tuple_2: (u64, u64) = (u64_7, u64_6);
    let mut str_3: &str = "StDzgUABc7";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut u64_8: u64 = 8403u64;
    let mut u64_9: u64 = 2614u64;
    let mut tuple_3: (u64, u64) = (u64_9, u64_8);
    let mut str_4: &str = "y2pPqQqY1q23OifekKf";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_10: u64 = 9868u64;
    let mut u64_11: u64 = 3616u64;
    let mut tuple_4: (u64, u64) = (u64_11, u64_10);
    let mut str_5: &str = "9xsnoYmaKIZJhg";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut usize_4: usize = 84usize;
    let mut error_0: duration::Error = crate::duration::Error::NumberExpected(usize_4);
    panic!("From RustyUnit with love");
}
}