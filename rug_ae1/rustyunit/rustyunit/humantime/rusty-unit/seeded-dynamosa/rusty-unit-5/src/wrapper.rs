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
#[timeout(30000)]fn rusty_test_8602() {
//    rusty_monitor::set_test_id(8602);
    let mut usize_0: usize = 4437usize;
    let mut str_0: &str = "2018-02-14T00:28:07Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2018-02-14T00:28:07";
    let mut str_2: &str = "GZsI9OfaYNIxHk";
    let mut str_3: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 6814usize;
    let mut str_4: &str = "0s";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_6: &str = "2cvI1aR2MuGuh";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut duration_2: std::time::Duration = std::result::Result::unwrap(result_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8610() {
//    rusty_monitor::set_test_id(8610);
    let mut usize_0: usize = 12usize;
    let mut str_0: &str = "Duration";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2018-02-14T00:28:07Z";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "2018-02-14T00:28:07";
    let mut str_3: &str = "GZsI9OfaYNIxHk";
    let mut str_4: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_1: usize = 6814usize;
    let mut str_5: &str = "2cvI1aR2MuGuh";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_2: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_3: duration::Error = crate::duration::Error::Empty;
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut duration_2: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut error_7: duration::Error = crate::duration::Error::NumberExpected(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8614() {
//    rusty_monitor::set_test_id(8614);
    let mut usize_0: usize = 4437usize;
    let mut str_0: &str = "2018-02-14T00:28:07Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2018-02-14T00:28:07";
    let mut str_2: &str = "GZsI9OfaYNIxHk";
    let mut str_3: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 6814usize;
    let mut str_4: &str = "0s";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_6: &str = "2cvI1aR2MuGuh";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_6: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
    let mut error_8: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8622() {
//    rusty_monitor::set_test_id(8622);
    let mut u64_0: u64 = 3u64;
    let mut u64_1: u64 = 31557600u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "BfWCFW5E7csdFO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "cz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_2: &str = "JfKDvW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07";
    let mut str_5: &str = "GZsI9OfaYNIxHk";
    let mut str_6: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut usize_0: usize = 6814usize;
    let mut str_7: &str = "0s";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "2018-02-14T00:28:07";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_9: &str = "2cvI1aR2MuGuh";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_4_ref_0);
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut duration_2: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_1: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8630() {
//    rusty_monitor::set_test_id(8630);
    let mut str_0: &str = "2018-02-14T00:28:07Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2018-02-14T00:28:07";
    let mut str_2: &str = "GZsI9OfaYNIxHk";
    let mut str_3: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_0: usize = 6814usize;
    let mut str_4: &str = "0s";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_6: &str = "2cvI1aR2MuGuh";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_7: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8648() {
//    rusty_monitor::set_test_id(8648);
    let mut u64_0: u64 = 10u64;
    let mut u64_1: u64 = 304u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "Duration";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "2018-02-14T00:28:07Z";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "2018-02-14T00:28:07";
    let mut str_3: &str = "GZsI9OfaYNIxHk";
    let mut str_4: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut usize_0: usize = 6814usize;
    let mut str_5: &str = "0s";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2018-02-14T00:28:07";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_7: &str = "2cvI1aR2MuGuh";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_2_ref_0: &str = &mut str_2;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_2_ref_0);
    let mut error_7: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_8: date::Error = crate::date::Error::OutOfRange;
    let mut error_9: date::Error = crate::date::Error::OutOfRange;
    let mut error_10: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8649() {
//    rusty_monitor::set_test_id(8649);
    let mut str_0: &str = "NumberExpected";
    let mut str_1: &str = "kPpQVypW4OJywi3ylrM";
    let mut usize_0: usize = 2915usize;
    let mut str_2: &str = "end";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_4: &str = "0s";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_6: &str = "2cvI1aR2MuGuh";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut error_7: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_8: date::Error = crate::date::Error::OutOfRange;
    let mut error_9: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8656() {
//    rusty_monitor::set_test_id(8656);
    let mut str_0: &str = " ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut usize_0: usize = 20usize;
    let mut str_1: &str = "OutOfRange";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut usize_1: usize = 15usize;
    let mut str_2: &str = "oSHB5kNQi7PKgncs";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "InvalidDigit";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "day";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07Z";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "GZsI9OfaYNIxHk";
    let mut str_7: &str = "TgvoAEfBEhUT7BqlyO";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "0s";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "2018-02-14T00:28:07";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_10: &str = "2cvI1aR2MuGuh";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_10_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_3: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8661() {
//    rusty_monitor::set_test_id(8661);
    let mut str_0: &str = "2018-02-14T00:28:07Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "GZsI9OfaYNIxHk";
    let mut str_2: &str = "Duration";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_0: usize = 6814usize;
    let mut str_3: &str = "0s";
    let mut str_4: &str = "2018-02-14T00:28:07";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_4_ref_0: &str = &mut str_4;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut error_4: date::Error = crate::date::Error::OutOfRange;
    let mut error_5: duration::Error = crate::duration::Error::Empty;
    let mut error_6: date::Error = crate::date::Error::OutOfRange;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_1_ref_0);
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_4);
    let mut error_7: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8670() {
//    rusty_monitor::set_test_id(8670);
    let mut str_0: &str = "OutOfRange";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Duration";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut str_2: &str = "gf2d0D";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "m";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bool_0: bool = false;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "2018-02-14T00:28:07";
    let mut str_6: &str = "GZsI9OfaYNIxHk";
    let mut usize_0: usize = 6814usize;
    let mut str_7: &str = "2018-02-14T00:28:07";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_8: &str = "2cvI1aR2MuGuh";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::Empty;
    let mut duration_0: std::time::Duration = std::result::Result::unwrap(result_1);
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
    let mut result_3: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_7_ref_0);
    let mut error_4: duration::Error = crate::duration::Error::Empty;
    let mut duration_1: std::time::Duration = std::result::Result::unwrap(result_2);
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_3);
    let mut error_5: date::Error = crate::date::Error::OutOfRange;
    let mut duration_2: std::time::Duration = std::result::Result::unwrap(result_0);
    let mut result_4: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_0_ref_0);
//    panic!("From RustyUnit with love");
}
}