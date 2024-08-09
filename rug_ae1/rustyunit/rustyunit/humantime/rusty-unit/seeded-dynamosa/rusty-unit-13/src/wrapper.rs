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
#[timeout(30000)]fn rusty_test_8482() {
//    rusty_monitor::set_test_id(8482);
    let mut usize_0: usize = 27usize;
    let mut str_0: &str = "InvalidFormat";
    let mut str_1: &str = "ns";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u64_0: u64 = 524u64;
    let mut u64_1: u64 = 31u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_2: &str = "g6P";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 4449usize;
    let mut str_3: &str = "Duration";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "FP1";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "Micros";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_6: &str = "2018-02-14T00:28:07Z";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "WVOnxAz1";
    let mut str_8: &str = "Rfc3339Timestamp";
    let mut str_9: &str = "ms";
    let mut str_10: &str = " ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_11: &str = "OxfkF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_12: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut str_13: &str = "ck2kyUE9";
    let mut str_14: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_15: &str = "Empty";
    let mut str_16: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_17: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_16;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut str_13: &str = "Sfkg2ZkYpl9";
    let mut str_14: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_7_ref_0: &str = &mut str_12;
    let mut str_15: &str = "Empty";
    let mut str_16: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_14;
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_18: &str = "2018-02-14T00:28:07Z";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_19: &str = "Smart";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut str_13_ref_0: &str = &mut str_13;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_15_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_13_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_19_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_1);
    let mut error_3: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8496() {
//    rusty_monitor::set_test_id(8496);
    let mut usize_0: usize = 12usize;
    let mut str_0: &str = "Duration";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "FP1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "pCsxwu3W";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Micros";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "WVOnxAz1";
    let mut str_6: &str = "Rfc3339Timestamp";
    let mut str_7: &str = "ms";
    let mut str_8: &str = "0AvfRg";
    let mut str_9: &str = " ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "OxfkF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut str_12: &str = "ck2kyUE9";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_15;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut str_12: &str = "Sfkg2ZkYpl9";
    let mut str_13: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_13_ref_0: &str = &mut str_16;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_8;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_17: &str = "2018-02-14T00:28:07Z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_18: &str = "Smart";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19: &str = "year";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_13_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_18_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_17_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_19_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_1_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::InvalidCharacter(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8514() {
//    rusty_monitor::set_test_id(8514);
    let mut str_0: &str = "VInwPls4o";
    let mut u64_0: u64 = 4207u64;
    let mut u64_1: u64 = 3u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "end";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_2: &str = "Duration";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "FP1";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_4: &str = "pCsxwu3W";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "Micros";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_6: &str = "2018-02-14T00:28:07Z";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "WVOnxAz1";
    let mut str_8: &str = "Rfc3339Timestamp";
    let mut str_9: &str = "ms";
    let mut str_10: &str = "0AvfRg";
    let mut str_11: &str = " ";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_12: &str = "OxfkF";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_13: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_4_ref_0);
    let mut str_14: &str = "ck2kyUE9";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_15: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_18: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_12;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut str_14: &str = "Sfkg2ZkYpl9";
    let mut str_15: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_14_ref_0: &str = &mut str_13;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_6_ref_0: &str = &mut str_18;
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_19: &str = "2018-02-14T00:28:07Z";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_20: &str = "Smart";
    let mut str_19_ref_0: &str = &mut str_19;
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_20_ref_0: &str = &mut str_20;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_3_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_19_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_8_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_16_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_17_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_20_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_9);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_15_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8520() {
//    rusty_monitor::set_test_id(8520);
    let mut u64_0: u64 = 7026u64;
    let mut u64_1: u64 = 334u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "g6P";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 4449usize;
    let mut str_1: &str = "Duration";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "FP1";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Micros";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "WVOnxAz1";
    let mut str_6: &str = "Rfc3339Timestamp";
    let mut str_7: &str = "ms";
    let mut str_8: &str = "0AvfRg";
    let mut str_9: &str = " ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "OxfkF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut str_12: &str = "ck2kyUE9";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_15;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut str_12: &str = "Sfkg2ZkYpl9";
    let mut str_13: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_13_ref_0: &str = &mut str_16;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_8;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_17: &str = "2018-02-14T00:28:07Z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_18: &str = "Smart";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19: &str = "year";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_18_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_7_ref_0);
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_17_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_19_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
    let mut error_3: date::Error = crate::date::Error::InvalidFormat;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8521() {
//    rusty_monitor::set_test_id(8521);
    let mut str_0: &str = "Rpw65";
    let mut str_1: &str = " ";
    let mut usize_0: usize = 29usize;
    let mut str_2: &str = "all times should be after the epoch";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_3: &str = "Duration";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_4: &str = "FP1";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_5: &str = "Micros";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_6: &str = "2018-02-14T00:28:07Z";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_7: &str = "WVOnxAz1";
    let mut str_8: &str = "Rfc3339Timestamp";
    let mut str_9: &str = "ms";
    let mut str_10: &str = "0AvfRg";
    let mut str_11: &str = " ";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_12: &str = "OxfkF";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_13: &str = "0s";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut str_14: &str = "ck2kyUE9";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_15: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_18: &str = "ms";
    let mut str_12_ref_0: &str = &mut str_8;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_16_ref_0);
    let mut str_14: &str = "Sfkg2ZkYpl9";
    let mut str_15: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_14_ref_0: &str = &mut str_11;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_16_ref_0: &str = &mut str_15;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_19: &str = "2018-02-14T00:28:07Z";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_20: &str = "Smart";
    let mut str_19_ref_0: &str = &mut str_19;
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_20_ref_0: &str = &mut str_20;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_10_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_5_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_19_ref_0);
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_17_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_13_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_18_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut error_2: duration::Error = crate::duration::Error::NumberExpected(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8525() {
//    rusty_monitor::set_test_id(8525);
    let mut str_0: &str = "Seconds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u64_0: u64 = 524u64;
    let mut u64_1: u64 = 31u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_1: &str = "end";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Duration";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "FP1";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "pCsxwu3W";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Micros";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2018-02-14T00:28:07Z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "WVOnxAz1";
    let mut str_8: &str = "Rfc3339Timestamp";
    let mut str_9: &str = "ms";
    let mut str_10: &str = "0AvfRg";
    let mut str_11: &str = " ";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_12: &str = "OxfkF";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "0s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_13_ref_0);
    let mut str_14: &str = "ck2kyUE9";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_15: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_18: &str = "ms";
    let mut str_13_ref_0: &str = &mut str_17;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut str_14: &str = "Sfkg2ZkYpl9";
    let mut str_15: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_15_ref_0: &str = &mut str_18;
    let mut str_16: &str = "Empty";
    let mut str_17: &str = "Smart";
    let mut str_16_ref_0: &str = &mut str_10;
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_19: &str = "2018-02-14T00:28:07Z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_20: &str = "Smart";
    let mut str_19_ref_0: &str = &mut str_19;
    let mut str_20_ref_0: &str = &mut str_20;
    let mut str_21: &str = "year";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_21_ref_0: &str = &mut str_21;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_2_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_15_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_16_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_20_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_19_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_11_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_21_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_12_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
    let mut systemtime_0: std::time::SystemTime = std::result::Result::unwrap(result_8);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8531() {
//    rusty_monitor::set_test_id(8531);
    let mut str_0: &str = "month";
    let mut str_1: &str = "FP1";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "pCsxwu3W";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Micros";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "WVOnxAz1";
    let mut str_6: &str = "Rfc3339Timestamp";
    let mut str_7: &str = "ms";
    let mut str_8: &str = "0AvfRg";
    let mut str_9: &str = " ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "OxfkF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut str_12: &str = "ck2kyUE9";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_6;
    let mut str_16: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_7;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut str_12: &str = "Sfkg2ZkYpl9";
    let mut str_13: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_13_ref_0: &str = &mut str_16;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_15;
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_17: &str = "2018-02-14T00:28:07Z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_18: &str = "Smart";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19: &str = "year";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_18_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_0_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_17_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_13_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_19_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_12_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8555() {
//    rusty_monitor::set_test_id(8555);
    let mut str_0: &str = "FP1";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "pCsxwu3W";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Micros";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2018-02-14T00:28:07Z";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "WVOnxAz1";
    let mut str_5: &str = "Rfc3339Timestamp";
    let mut str_6: &str = "ms";
    let mut str_7: &str = "0AvfRg";
    let mut str_8: &str = " ";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "OxfkF";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "0s";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_10_ref_0);
    let mut str_11: &str = "ck2kyUE9";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_12: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "Empty";
    let mut str_14: &str = "Smart";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_15: &str = "ms";
    let mut str_10_ref_0: &str = &mut str_14;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut str_11: &str = "ck2kyUE9";
    let mut str_12: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_12_ref_0: &str = &mut str_15;
    let mut str_13: &str = "Empty";
    let mut str_14: &str = "Smart";
    let mut str_13_ref_0: &str = &mut str_7;
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_16: &str = "2018-02-14T00:28:07Z";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_17: &str = "Smart";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18: &str = "year";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_13_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_9_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_16_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_8_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_17_ref_0);
    let mut result_9: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_18_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8556() {
//    rusty_monitor::set_test_id(8556);
    let mut u64_0: u64 = 7026u64;
    let mut u64_1: u64 = 334u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut str_0: &str = "g6P";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Duration";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "gP";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Micros";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "2018-02-14T00:28:07Z";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "WVOnxAz1";
    let mut str_6: &str = "Rfc3339Timestamp";
    let mut str_7: &str = "ms";
    let mut str_8: &str = "0AvfRg";
    let mut str_9: &str = " ";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "OxfkF";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "0s";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut str_12: &str = "ck2kyUE9";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "I5ZVQEhenRReca0Z1s";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_16: &str = "ms";
    let mut str_11_ref_0: &str = &mut str_15;
    let mut str_12: &str = "Sfkg2ZkYpl9";
    let mut str_13: &str = "glZiQLweMFvpcUHZ1U";
    let mut str_13_ref_0: &str = &mut str_16;
    let mut str_14: &str = "Empty";
    let mut str_15: &str = "Smart";
    let mut str_14_ref_0: &str = &mut str_8;
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_17: &str = "2018-02-14T00:28:07Z";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_18: &str = "Smart";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19: &str = "year";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_19_ref_0: &str = &mut str_19;
    let mut result_1: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_1_ref_0);
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_6_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_18_ref_0);
    let mut result_5: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut result_6: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_7: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_8: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_13_ref_0);
    let mut error_0: date::Error = crate::date::Error::OutOfRange;
    let mut error_1: date::Error = crate::date::Error::OutOfRange;
//    panic!("From RustyUnit with love");
}
}