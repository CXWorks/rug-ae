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
#[timeout(30000)]fn rusty_test_5576() {
//    rusty_monitor::set_test_id(5576);
    let mut str_0: &str = "Q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "5M6";
    let mut str_2: &str = "NumberOverflow";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "8jbSUFP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_4: &str = "Nanos";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "0s";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_6: &str = "Seconds";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "ZU1qwCBafhly4";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_8: &str = "day";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_9: &str = "2BZmewNDq2tKAF2r0n";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_10: &str = "Rfc3339Timestamp";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_11: &str = "Nanos";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_10_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_10: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}