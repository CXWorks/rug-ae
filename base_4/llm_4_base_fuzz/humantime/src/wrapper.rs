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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let human_duration = wrapper::Duration::from(std_duration);
        let as_ref_result = AsRef::<StdDuration>::as_ref(&human_duration);
        debug_assert_eq!(& std_duration, as_ref_result);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    use std::time::Duration as StdDuration;
    #[test]
    fn test_from_std_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let humantime_duration: wrapper::Duration = std_duration.into();
        debug_assert_eq!(humantime_duration.as_ref(), & std_duration);
             }
});    }
    #[test]
    fn test_from_std_duration_with_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let humantime_duration: wrapper::Duration = std_duration.into();
        debug_assert_eq!(* humantime_duration, std_duration);
             }
});    }
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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parsed_duration = Duration::from_str(rug_fuzz_0).expect(rug_fuzz_1);
        let std_duration: StdDuration = parsed_duration.into();
        debug_assert_eq!(std_duration, StdDuration::from_secs(60 * 60));
             }
});    }
    #[test]
    fn test_std_duration_into_duration_and_back() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::from_secs(rug_fuzz_0 * rug_fuzz_1 * rug_fuzz_2);
        let human_duration: Duration = Duration::from(std_duration);
        let converted_back: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, converted_back);
             }
});    }
    #[test]
    fn test_zero_duration_into() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let human_duration = Duration::from(StdDuration::new(rug_fuzz_0, rug_fuzz_1));
        let std_duration: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, StdDuration::new(0, 0));
             }
});    }
    #[test]
    fn test_max_duration_into() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let human_duration = Duration::from(StdDuration::new(u64::MAX, rug_fuzz_0));
        let std_duration: StdDuration = human_duration.into();
        debug_assert_eq!(std_duration, StdDuration::new(u64::MAX, 999_999_999));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use std::time::Duration as StdDuration;
    use std::ops::Deref;
    use crate::wrapper::Duration;
    #[test]
    fn deref_duration() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::new(rug_fuzz_0, rug_fuzz_1);
        let my_duration = Duration::from(std_duration);
        debug_assert_eq!(* my_duration.deref(), std_duration);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use std::time::{Duration, UNIX_EPOCH};
    #[test]
    fn test_as_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(time);
        let system_time_ref: &SystemTime = timestamp.as_ref();
        debug_assert_eq!(& time, system_time_ref);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use super::*;
    use crate::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::str::FromStr;
    #[test]
    fn test_from_system_time_to_timestamp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let system_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(system_time);
        debug_assert_eq!(* timestamp, system_time);
             }
});    }
    #[test]
    fn test_timestamp_display() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let system_time = UNIX_EPOCH + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(system_time);
        let formatted_time = format!("{}", timestamp);
        let expected_time = rug_fuzz_2;
        debug_assert_eq!(formatted_time, expected_time);
             }
});    }
    #[test]
    fn test_from_str_to_timestamp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let rfc3339_str = rug_fuzz_0;
        let timestamp = Timestamp::from_str(rfc3339_str).unwrap();
        let system_time: SystemTime = timestamp.into();
        let expected_time = UNIX_EPOCH
            + std::time::Duration::new(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(system_time, expected_time);
             }
});    }
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

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timestamp_str = rug_fuzz_0;
        let timestamp = Timestamp::from_str(timestamp_str).unwrap();
        let expected = UNIX_EPOCH + Duration::new(rug_fuzz_1, rug_fuzz_2);
        let result: SystemTime = timestamp.into();
        debug_assert_eq!(result, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;
    use crate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::ops::Deref;
    #[test]
    fn test_deref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = UNIX_EPOCH + Duration::new(rug_fuzz_0, rug_fuzz_1);
        let timestamp = Timestamp::from(time);
        debug_assert_eq!(& time, timestamp.deref());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_input() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let result = Timestamp::from_str(input);
        debug_assert!(result.is_ok());
        let timestamp = result.expect(rug_fuzz_1);
             }
});    }
    #[test]
    fn test_from_str_invalid_input() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let result = Timestamp::from_str(input);
        debug_assert!(result.is_err());
             }
});    }
}
