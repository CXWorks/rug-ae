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
    fn as_ref(&self) -> &StdDuration {
        &self.0
    }
}
impl Deref for Duration {
    type Target = StdDuration;
    fn deref(&self) -> &StdDuration {
        &self.0
    }
}
impl Into<StdDuration> for Duration {
    fn into(self) -> StdDuration {
        self.0
    }
}
impl From<StdDuration> for Duration {
    fn from(dur: StdDuration) -> Duration {
        Duration(dur)
    }
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
    fn as_ref(&self) -> &SystemTime {
        &self.0
    }
}
impl Deref for Timestamp {
    type Target = SystemTime;
    fn deref(&self) -> &SystemTime {
        &self.0
    }
}
impl Into<SystemTime> for Timestamp {
    fn into(self) -> SystemTime {
        self.0
    }
}
impl From<SystemTime> for Timestamp {
    fn from(dur: SystemTime) -> Timestamp {
        Timestamp(dur)
    }
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
mod tests_llm_16_3 {
    use super::*;
    use crate::*;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_as_ref() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_as_ref = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 0;
        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let human_duration = wrapper::Duration::from(std_duration);
        let as_ref_result = AsRef::<StdDuration>::as_ref(&human_duration);
        debug_assert_eq!(& std_duration, as_ref_result);
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_as_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_from_std_duration() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_from_std_duration = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 0;
        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let humantime_duration: wrapper::Duration = std_duration.into();
        debug_assert_eq!(humantime_duration.as_ref(), & std_duration);
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_from_std_duration = 0;
    }
    #[test]
    fn test_from_std_duration_with_nanos() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_from_std_duration_with_nanos = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 500;
        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let humantime_duration: wrapper::Duration = std_duration.into();
        debug_assert_eq!(* humantime_duration, std_duration);
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_from_std_duration_with_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use crate::Duration;
    use std::time::Duration as StdDuration;
    use std::str::FromStr;
    use std::convert::From;
    use std::convert::Into;
    #[test]
    fn test_duration_into_std_duration() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_duration_into_std_duration = 0;
        let rug_fuzz_0 = "1h";
        let rug_fuzz_1 = "Failed to parse human-readable duration";
        let parsed_duration = Duration::from_str(rug_fuzz_0).expect(rug_fuzz_1);
        let std_duration: StdDuration = parsed_duration.into();
        debug_assert_eq!(std_duration, StdDuration::from_secs(60 * 60));
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_duration_into_std_duration = 0;
    }
    #[test]
    fn test_std_duration_into_duration_and_back() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_std_duration_into_duration_and_back = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 60;
        let std_duration = StdDuration::from_secs(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2);
        let human_duration: Duration = Duration::from(std_duration);
        let converted_back: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, converted_back);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_std_duration_into_duration_and_back = 0;
    }
    #[test]
    fn test_zero_duration_into() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_zero_duration_into = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let human_duration = Duration::from(StdDuration::new(rug_fuzz_0, rug_fuzz_1));
        let std_duration: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, StdDuration::new(0, 0));
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_zero_duration_into = 0;
    }
    #[test]
    fn test_max_duration_into() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_max_duration_into = 0;
        let rug_fuzz_0 = 999_999_999;
        let human_duration = Duration::from(StdDuration::new(u64::MAX, rug_fuzz_0));
        let std_duration: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, StdDuration::new(u64::MAX, 999_999_999));
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_max_duration_into = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use std::time::Duration as StdDuration;
    use std::ops::Deref;
    use crate::wrapper::Duration;
    #[test]
    fn deref_duration() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_deref_duration = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 0;
        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let my_duration = Duration::from(std_duration);
        debug_assert_eq!(* my_duration.deref(), std_duration);
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_deref_duration = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use std::time::{Duration, UNIX_EPOCH};
    #[test]
    fn test_as_ref() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_test_as_ref = 0;
        let rug_fuzz_0 = 1_500_000_000;
        let rug_fuzz_1 = 0;
        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(time);
        let system_time_ref: &SystemTime = timestamp.as_ref();
        debug_assert_eq!(& time, system_time_ref);
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_test_as_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::str::FromStr;
    #[test]
    fn test_from_system_time_to_timestamp() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_from_system_time_to_timestamp = 0;
        let rug_fuzz_0 = 1_000_000;
        let rug_fuzz_1 = 0;
        let system_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(system_time);
        debug_assert_eq!(* timestamp, system_time);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_from_system_time_to_timestamp = 0;
    }
    #[test]
    fn test_timestamp_display() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_timestamp_display = 0;
        let rug_fuzz_0 = 1_000_000;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "2001-09-09T01:46:40Z";
        let system_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(system_time);
        let formatted_time = format!("{}", timestamp);
        let expected_time = rug_fuzz_2;
        debug_assert_eq!(formatted_time, expected_time);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_timestamp_display = 0;
    }
    #[test]
    fn test_from_str_to_timestamp() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_from_str_to_timestamp = 0;
        let rug_fuzz_0 = "2001-09-09T01:46:40Z";
        let rug_fuzz_1 = 1_000_000;
        let rug_fuzz_2 = 0;
        let rfc3339_str = rug_fuzz_0;
        let timestamp = Timestamp::from_str(rfc3339_str).unwrap();
        let system_time: SystemTime = timestamp.into();
        let expected_time = UNIX_EPOCH
            + std::time::Duration::new(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(system_time, expected_time);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_from_str_to_timestamp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::str::FromStr;
    use std::convert::Into;
    #[test]
    fn timestamp_into_systemtime() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_timestamp_into_systemtime = 0;
        let rug_fuzz_0 = "2018-02-16T00:31:37Z";
        let rug_fuzz_1 = 1518743497;
        let rug_fuzz_2 = 0;
        let timestamp_str = rug_fuzz_0;
        let timestamp = Timestamp::from_str(timestamp_str).unwrap();
        let expected = UNIX_EPOCH + Duration::new(rug_fuzz_1, rug_fuzz_2);
        let result: SystemTime = timestamp.into();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_timestamp_into_systemtime = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::ops::Deref;
    #[test]
    fn test_deref() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_deref = 0;
        let rug_fuzz_0 = 1_234_567_890;
        let rug_fuzz_1 = 0;
        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(time);
        debug_assert_eq!(& time, timestamp.deref());
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_deref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_input() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_from_str_valid_input = 0;
        let rug_fuzz_0 = "2023-03-01T12:00:00Z";
        let rug_fuzz_1 = "Failed to parse RFC3339 timestamp";
        let input = rug_fuzz_0;
        let result = Timestamp::from_str(input);
        debug_assert!(result.is_ok());
        let timestamp = result.expect(rug_fuzz_1);
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_from_str_valid_input = 0;
    }
    #[test]
    fn test_from_str_invalid_input() {
        let _rug_st_tests_llm_16_12_rrrruuuugggg_test_from_str_invalid_input = 0;
        let rug_fuzz_0 = "invalid-timestamp";
        let input = rug_fuzz_0;
        let result = Timestamp::from_str(input);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_12_rrrruuuugggg_test_from_str_invalid_input = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::wrapper::Duration;
    use std::str::FromStr;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "15min";
        let mut p0: &str = rug_fuzz_0;
        let _ = Duration::from_str(&p0);
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = 0;
    }
}
