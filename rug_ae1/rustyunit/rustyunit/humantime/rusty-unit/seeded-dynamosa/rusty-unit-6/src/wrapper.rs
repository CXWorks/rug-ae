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
#[timeout(30000)]fn rusty_test_298() {
//    rusty_monitor::set_test_id(298);
    let mut str_0: &str = "InvalidDigit";
    let mut bool_0: bool = true;
    let mut usize_0: usize = 8891usize;
    let mut u64_0: u64 = 5u64;
    let mut str_1: &str = "fNf2fwU5SHgS8zQWVy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "OutOfRange";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_1: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_3: &str = "all times should be after the epoch";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_1_ref_0: &mut bool = &mut bool_1;
    let mut str_5: &str = "Qp4TAHPNt";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut u64_1: u64 = 2978u64;
    let mut tuple_0: (u64, u64) = (u64_0, u64_1);
    let mut str_5_ref_0: &str = &mut str_5;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_2: duration::Error = crate::duration::Error::Empty;
    let mut error_3: date::Error = crate::date::Error::OutOfRange;
    let mut error_4: duration::Error = crate::duration::Error::NumberOverflow;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2169() {
//    rusty_monitor::set_test_id(2169);
    let mut str_0: &str = "44y";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "year";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Micros";
    let mut option_0: std::option::Option<u64> = std::option::Option::None;
    let mut str_3: &str = "InvalidFormat";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "0s";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "Seconds";
    let mut str_6: &str = "s3FdBiMJl2phxHFYXqx";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_7: &str = "2018-02-14T00:28:07";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_8: &str = "zuuUpb3Zl";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_9: &str = "ns";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5032() {
//    rusty_monitor::set_test_id(5032);
    let mut str_0: &str = "Micros";
    let mut str_1: &str = "Nanos";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "BTre56ARyS3zydGU";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_3: &str = "0s";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Seconds";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "s3FdBiMJl2phxHFYXqx";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2018-02-14T00:28:07";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "ns";
    let mut str_8: &str = "year";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
//    panic!("From RustyUnit with love");
}
}