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
#[timeout(30000)]fn rusty_test_8436() {
//    rusty_monitor::set_test_id(8436);
    let mut str_0: &str = "rZLojQBzu4E8";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "Smart";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "XTLzfo0v";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = " ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "day";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "Micros";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "2018-02-14T00:28:07";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "year";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "2018-02-14T00:28:07Z";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "0s";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "day";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut str_11: &str = "day";
    let mut str_11_ref_0: &str = &mut str_11;
    let mut str_12: &str = "ZISH8Pu1VXsJN";
    let mut str_12_ref_0: &str = &mut str_12;
    let mut str_13: &str = "yzUx";
    let mut str_13_ref_0: &str = &mut str_13;
    let mut str_14: &str = "InvalidFormat";
    let mut str_14_ref_0: &str = &mut str_14;
    let mut str_15: &str = "s";
    let mut str_15_ref_0: &str = &mut str_15;
    let mut str_16: &str = " ";
    let mut str_16_ref_0: &str = &mut str_16;
    let mut str_17: &str = "VgbUdHjccBxOoA7ofma";
    let mut str_17_ref_0: &str = &mut str_17;
    let mut str_18: &str = "pW";
    let mut str_18_ref_0: &str = &mut str_18;
    let mut str_19: &str = "oWa0S0JYw0kHW";
    let mut str_20: &str = "Duration";
    let mut str_21: &str = "NumberExpected";
    let mut str_22: &str = "Conversion to utf8 failed";
    let mut str_23: &str = "nN0IQnRilp6";
    let mut str_24: &str = "start";
    let mut str_25: &str = "Millis";
    let mut str_26: &str = "Nanos";
    let mut str_27: &str = "cnr";
    let mut str_28: &str = "2018-02-14T00:28:07Z";
    let mut str_29: &str = "28w";
    let mut str_30: &str = "9QV";
    let mut str_31: &str = "Seconds";
    let mut str_32: &str = "IBFPAk4zj";
    let mut str_33: &str = "end";
    let mut str_34: &str = "2018-02-14T00:28:07";
    let mut str_35: &str = "us";
    let mut str_36: &str = "2018-02-14T00:28:07Z";
    let mut str_37: &str = "end";
    let mut str_38: &str = "rspmuWonm15ZPaOT";
    let mut str_39: &str = "nJo";
    let mut str_20_ref_0: &str = &mut str_20;
    let mut bool_0: bool = true;
    let mut bool_0_ref_0: &mut bool = &mut bool_0;
    let mut str_40: &str = "rJUiDI";
    let mut str_40_ref_0: &str = &mut str_40;
    let mut str_41: &str = "GLpZU3sLvV9o";
    let mut str_41_ref_0: &str = &mut str_41;
    let mut str_42: &str = "InvalidCharacter";
    let mut str_42_ref_0: &str = &mut str_42;
    let mut str_43: &str = "value";
    let mut str_44: &str = "Conversion to utf8 failed";
    let mut str_44_ref_0: &str = &mut str_44;
    let mut str_45: &str = "Timestamp";
    let mut str_45_ref_0: &str = &mut str_45;
    let mut str_46: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_46_ref_0: &str = &mut str_46;
    let mut str_47: &str = "NumberOverflow";
    let mut str_47_ref_0: &str = &mut str_47;
    let mut str_48: &str = "SI4HWXXC9";
    let mut str_48_ref_0: &str = &mut str_48;
    let mut str_49: &str = "2018-02-14T00:28:07Z";
    let mut str_50: &str = "end";
    let mut str_51: &str = "4cV";
    let mut str_51_ref_0: &str = &mut str_51;
    let mut str_52: &str = "ns";
    let mut str_52_ref_0: &str = &mut str_52;
    let mut str_53: &str = "0s";
    let mut str_53_ref_0: &str = &mut str_53;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_53_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_52_ref_0);
    let mut str_50_ref_0: &str = &mut str_50;
    let mut str_54: &str = "b8SH7WFzHoH82t6oikT";
    let mut str_49_ref_0: &str = &mut str_49;
    let mut str_54_ref_0: &str = &mut str_54;
    let mut str_55: &str = "nrYzENBrrdI";
    let mut str_43_ref_0: &str = &mut str_43;
    let mut str_56: &str = "97h3I3ywD3VUuxqfp";
    let mut str_55_ref_0: &str = &mut str_55;
    let mut str_57: &str = "us";
    let mut str_56_ref_0: &str = &mut str_56;
    let mut str_58: &str = "ns";
    let mut str_57_ref_0: &str = &mut str_57;
    let mut str_59: &str = "rHqK";
    let mut str_58_ref_0: &str = &mut str_58;
    let mut str_60: &str = "year";
    let mut str_59_ref_0: &str = &mut str_59;
    let mut str_60_ref_0: &str = &mut str_60;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_56_ref_0);
    let mut str_61: &str = "Empty";
    let mut str_39_ref_0: &str = &mut str_39;
    let mut str_62: &str = "fXe6ftXCaDYSUy";
    let mut str_63: &str = "2018-02-14T00:28:07Z";
    let mut str_64: &str = "Seconds";
    let mut str_61_ref_0: &str = &mut str_61;
    let mut str_65: &str = "Micros";
    let mut str_62_ref_0: &str = &mut str_62;
    let mut str_66: &str = "Wy0cXQH";
    let mut str_64_ref_0: &str = &mut str_64;
    let mut str_40: &str = "rJUiDI";
    let mut str_40_ref_0: &str = &mut str_66;
    let mut str_41: &str = "GLpZU3sLvV9o";
    let mut str_41_ref_0: &str = &mut str_65;
    let mut str_42: &str = "InvalidCharacter";
    let mut str_42_ref_0: &str = &mut str_63;
    let mut str_44_ref_0: &str = &mut str_38;
    let mut str_45: &str = "Timestamp";
    let mut str_45_ref_0: &str = &mut str_37;
    let mut str_46: &str = "tJM8ac1reQk0hGv3eC2";
    let mut str_46_ref_0: &str = &mut str_36;
    let mut str_47_ref_0: &str = &mut str_35;
    let mut str_48: &str = "SI4HWXXC9";
    let mut str_48_ref_0: &str = &mut str_34;
    let mut str_49: &str = "2018-02-14T00:28:07Z";
    let mut str_50: &str = "end";
    let mut str_51: &str = "4cV";
    let mut str_51_ref_0: &str = &mut str_33;
    let mut str_52_ref_0: &str = &mut str_32;
    let mut str_53_ref_0: &str = &mut str_31;
    let mut result_0: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_40_ref_0);
    let mut result_1: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339_weak(str_45_ref_0);
    let mut str_50_ref_0: &str = &mut str_30;
    let mut str_54: &str = "Conversion to utf8 failed";
    let mut str_49_ref_0: &str = &mut str_29;
    let mut str_54_ref_0: &str = &mut str_28;
    let mut str_55: &str = "nrYzENBrrdI";
    let mut str_43_ref_0: &str = &mut str_27;
    let mut str_56: &str = "Rfc3339Timestamp";
    let mut str_55_ref_0: &str = &mut str_26;
    let mut str_56_ref_0: &str = &mut str_25;
    let mut str_58: &str = "ns";
    let mut str_57_ref_0: &str = &mut str_24;
    let mut str_59: &str = "rHqK";
    let mut str_58_ref_0: &str = &mut str_23;
    let mut str_60: &str = "year";
    let mut str_59_ref_0: &str = &mut str_22;
    let mut str_60_ref_0: &str = &mut str_21;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_48_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_54_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_47_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_42_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_58_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_49_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_50_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_51_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_44_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_57_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_20_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_43_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_61_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_41_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_60_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_46_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_55_ref_0);
    let mut result_5: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_39_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_62_ref_0);
    let mut str_60: &str = "year";
    let mut str_60_ref_0: &str = &mut str_19;
    let mut result_2: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_64_ref_0);
    let mut result_3: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_18_ref_0);
    let mut result_4: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_17_ref_0);
    let mut result_6: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_16_ref_0);
    let mut result_19: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_15_ref_0);
    let mut result_7: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_14_ref_0);
    let mut result_20: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_13_ref_0);
    let mut result_8: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_12_ref_0);
    let mut result_9: std::result::Result<std::time::Duration, duration::Error> = crate::duration::parse_duration(str_11_ref_0);
    let mut result_10: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_10_ref_0);
    let mut result_11: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_9_ref_0);
    let mut result_12: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_3_ref_0);
    let mut result_13: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_0_ref_0);
    let mut result_14: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_2_ref_0);
    let mut result_15: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_7_ref_0);
    let mut result_16: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_6_ref_0);
    let mut result_17: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_5_ref_0);
    let mut result_21: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_8_ref_0);
    let mut result_18: std::result::Result<std::time::SystemTime, date::Error> = crate::date::parse_rfc3339(str_4_ref_0);
    let mut error_0: duration::Error = crate::duration::Error::NumberOverflow;
//    panic!("From RustyUnit with love");
}
}