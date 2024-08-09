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
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u64_0: u64 = 8344u64;
    let mut u64_1: u64 = 6299u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "oIglXGttpQQUrYkG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_2: u64 = 1169u64;
    let mut u64_3: u64 = 5946u64;
    let mut tuple_1: (u64, u64) = (u64_3, u64_2);
    let mut str_1: &str = "P";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "i";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2qlQVCEiX6YnT";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 6334usize;
    let mut u64_4: u64 = 1641u64;
    let mut str_4: &str = "6L";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut u64_5: u64 = 1823u64;
    let mut u64_6: u64 = 162u64;
    let mut tuple_2: (u64, u64) = (u64_6, u64_5);
    let mut str_5: &str = "jjnaSmcR2lyqUpzEVLf";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut result_0: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_1: duration::Error = crate::duration::Error::NumberOverflow;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_0);
    let mut error_2: date::Error = crate::date::Error::OutOfRange;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut usize_0: usize = 801usize;
    let mut usize_1: usize = 812usize;
    let mut u64_0: u64 = 7352u64;
    let mut u64_1: u64 = 4060u64;
    let mut u64_2: u64 = 3177u64;
    let mut tuple_0: (u64, u64) = (u64_2, u64_1);
    let mut str_0: &str = "EXJcyljZQDBYktvmM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_3: u64 = 6696u64;
    let mut u64_4: u64 = 558u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut str_1: &str = "2YQKOd8TnoNFtSGBYd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_2: usize = 2656usize;
    let mut usize_3: usize = 5170usize;
    let mut u64_5: u64 = 6455u64;
    let mut u64_6: u64 = 2389u64;
    let mut u64_7: u64 = 9659u64;
    let mut tuple_2: (u64, u64) = (u64_7, u64_6);
    let mut str_2: &str = "hVN0G1W6kff9azdn";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u64_8: u64 = 515u64;
    let mut str_3: &str = "OLvrCzF";
    let mut string_0: std::string::String = std::string::String::from(str_3);
    let mut usize_4: usize = 272usize;
    let mut usize_5: usize = 3924usize;
    let mut usize_6: usize = 2692usize;
    let mut usize_7: usize = 4805usize;
    let mut u64_9: u64 = 5888u64;
    let mut u64_10: u64 = 1161u64;
    let mut u64_11: u64 = 8849u64;
    let mut tuple_3: (u64, u64) = (u64_11, u64_10);
    let mut str_4: &str = "fjRChkKpL55Qq";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_8: usize = 7692usize;
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_8);
    let mut error_1: duration::Error = crate::duration::Error::UnknownUnit {start: usize_5, end: usize_4, unit: string_0, value: u64_8};
    panic!("From RustyUnit with love");
}
}