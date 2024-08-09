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
fn rusty_test_4925() {
    rusty_monitor::set_test_id(4925);
    let mut u64_0: u64 = 6999u64;
    let mut u64_1: u64 = 132u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "EIV2i";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_2: u64 = 5423u64;
    let mut u64_3: u64 = 4040u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_1: &str = "PQpcWoqRObn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_4: u64 = 3928u64;
    let mut u64_5: u64 = 4433u64;
    let mut tuple_2: (u64, u64) = (u64_5, u64_4);
    let mut str_2: &str = "OsNA6das0WuKyh";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 1544usize;
    let mut usize_1: usize = 9107usize;
    let mut u64_6: u64 = 621u64;
    let mut u64_7: u64 = 9286u64;
    let mut u64_8: u64 = 6418u64;
    let mut tuple_3: (u64, u64) = (u64_8, u64_7);
    let mut str_3: &str = "q8vBWEaho";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_2: usize = 5595usize;
    let mut u64_9: u64 = 6365u64;
    let mut u64_10: u64 = 6799u64;
    let mut tuple_4: (u64, u64) = (u64_10, u64_9);
    let mut str_4: &str = "N6af";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_11: u64 = 5660u64;
    let mut u64_12: u64 = 5233u64;
    let mut tuple_5: (u64, u64) = (u64_12, u64_11);
    let mut str_5: &str = "TwoKGWB4J";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut u64_13: u64 = 407u64;
    let mut str_6: &str = "7s4mMNIt90A3fANgSwj";
    let mut string_0: std::string::String = std::string::String::from(str_6);
    let mut usize_3: usize = 1857usize;
    let mut usize_4: usize = 1885usize;
    let mut u64_14: u64 = 5238u64;
    let mut u64_15: u64 = 1054u64;
    let mut tuple_6: (u64, u64) = (u64_15, u64_14);
    let mut str_7: &str = "JS5";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "yvDU1";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut u64_16: u64 = 9923u64;
    let mut u64_17: u64 = 7514u64;
    let mut tuple_7: (u64, u64) = (u64_17, u64_16);
    let mut str_9: &str = "kWdR9VmwtE";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut u64_18: u64 = 726u64;
    let mut str_10: &str = "7";
    let mut string_1: std::string::String = std::string::String::from(str_10);
    let mut usize_5: usize = 5304usize;
    let mut usize_6: usize = 8319usize;
    let mut str_11: &str = "";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_11_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::UnknownUnit {start: usize_6, end: usize_5, unit: string_1, value: u64_18};
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::UnknownUnit {start: usize_4, end: usize_3, unit: string_0, value: u64_13};
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_2);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    panic!("From RustyUnit with love");
}
}