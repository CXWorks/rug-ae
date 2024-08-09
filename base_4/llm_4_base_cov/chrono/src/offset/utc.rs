//! The UTC (Coordinated Universal Time) time zone.
use core::fmt;
#[cfg(
    all(
        feature = "clock",
        not(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )
)]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
use super::{FixedOffset, LocalResult, Offset, TimeZone};
use crate::naive::{NaiveDate, NaiveDateTime};
#[cfg(feature = "clock")]
#[allow(deprecated)]
use crate::{Date, DateTime};
/// The UTC time zone. This is the most efficient time zone when you don't need the local time.
/// It is also used as an offset (which is also a dummy type).
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the UTC struct is the preferred way to construct `DateTime<Utc>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
///
/// let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
///
/// assert_eq!(Utc.timestamp(61, 0), dt);
/// assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Utc;
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl Utc {
    /// Returns a `Date` which corresponds to the current date.
    #[deprecated(
        since = "0.4.23",
        note = "use `Utc::now()` instead, potentially with `.date_naive()`"
    )]
    #[allow(deprecated)]
    #[must_use]
    pub fn today() -> Date<Utc> {
        Utc::now().date()
    }
    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(
        not(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )]
    #[must_use]
    pub fn now() -> DateTime<Utc> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");
        let naive = NaiveDateTime::from_timestamp_opt(
                now.as_secs() as i64,
                now.subsec_nanos(),
            )
            .unwrap();
        DateTime::from_utc(naive, Utc)
    }
    /// Returns a `DateTime` which corresponds to the current date and time.
    #[cfg(
        all(
            target_arch = "wasm32",
            feature = "wasmbind",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    #[must_use]
    pub fn now() -> DateTime<Utc> {
        let now = js_sys::Date::new_0();
        DateTime::<Utc>::from(now)
    }
}
impl TimeZone for Utc {
    type Offset = Utc;
    fn from_offset(_state: &Utc) -> Utc {
        Utc
    }
    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Utc> {
        LocalResult::Single(Utc)
    }
    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Utc> {
        LocalResult::Single(Utc)
    }
    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc {
        Utc
    }
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc {
        Utc
    }
}
impl Offset for Utc {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east_opt(0).unwrap()
    }
}
impl fmt::Debug for Utc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Z")
    }
}
impl fmt::Display for Utc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UTC")
    }
}
#[cfg(test)]
mod tests_llm_16_193 {
    use crate::Offset;
    use crate::offset::fixed::FixedOffset;
    use crate::offset::utc::Utc;
    #[test]
    fn test_utc_fix() {
        let _rug_st_tests_llm_16_193_rrrruuuugggg_test_utc_fix = 0;
        let utc = Utc;
        let fixed_offset = utc.fix();
        debug_assert_eq!(fixed_offset, FixedOffset::east(0));
        let _rug_ed_tests_llm_16_193_rrrruuuugggg_test_utc_fix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_194 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, Utc};
    #[test]
    fn from_offset_returns_utc() {
        let _rug_st_tests_llm_16_194_rrrruuuugggg_from_offset_returns_utc = 0;
        let utc = Utc;
        let result = Utc::from_offset(&utc);
        debug_assert_eq!(result, Utc);
        let _rug_ed_tests_llm_16_194_rrrruuuugggg_from_offset_returns_utc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_195 {
    use crate::{NaiveDate, TimeZone, Utc};
    #[test]
    fn test_offset_from_local_date() {
        let _rug_st_tests_llm_16_195_rrrruuuugggg_test_offset_from_local_date = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 31;
        let utc = Utc;
        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let result = utc.offset_from_local_date(&date);
        debug_assert_eq!(result, crate ::offset::LocalResult::Single(Utc));
        let date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result = utc.offset_from_local_date(&date);
        debug_assert_eq!(result, crate ::offset::LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_195_rrrruuuugggg_test_offset_from_local_date = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_196 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime};
    #[test]
    fn test_offset_from_local_datetime() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let utc = Utc;
        let naive_datetime = NaiveDateTime::from_timestamp(rug_fuzz_0, rug_fuzz_1);
        let result = utc.offset_from_local_datetime(&naive_datetime);
        debug_assert_eq!(result, LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime = 0;
    }
    #[test]
    fn test_offset_from_local_datetime_before_epoch() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_before_epoch = 0;
        let rug_fuzz_0 = 1969;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let utc = Utc;
        let naive_datetime = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result = utc.offset_from_local_datetime(&naive_datetime);
        debug_assert_eq!(result, LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_before_epoch = 0;
    }
    #[test]
    fn test_offset_from_local_datetime_distant_future() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_distant_future = 0;
        let rug_fuzz_0 = 9999;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let utc = Utc;
        let naive_datetime = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result = utc.offset_from_local_datetime(&naive_datetime);
        debug_assert_eq!(result, LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_distant_future = 0;
    }
    #[test]
    fn test_offset_from_local_datetime_distant_past() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_distant_past = 0;
        let rug_fuzz_0 = 9999;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let utc = Utc;
        let naive_datetime = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result = utc.offset_from_local_datetime(&naive_datetime);
        debug_assert_eq!(result, LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_distant_past = 0;
    }
    #[test]
    fn test_offset_from_local_datetime_with_leap_second() {
        let _rug_st_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_with_leap_second = 0;
        let rug_fuzz_0 = 9999;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 1_000;
        let utc = Utc;
        let naive_datetime = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let result = utc.offset_from_local_datetime(&naive_datetime);
        debug_assert_eq!(result, LocalResult::Single(Utc));
        let _rug_ed_tests_llm_16_196_rrrruuuugggg_test_offset_from_local_datetime_with_leap_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_197 {
    use crate::{NaiveDate, TimeZone, Utc};
    #[test]
    fn test_offset_from_utc_date() {
        let _rug_st_tests_llm_16_197_rrrruuuugggg_test_offset_from_utc_date = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let utc = Utc;
        let naive_date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let offset = utc.offset_from_utc_date(&naive_date);
        debug_assert_eq!(offset, Utc);
        let _rug_ed_tests_llm_16_197_rrrruuuugggg_test_offset_from_utc_date = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_198 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, TimeZone, Utc};
    #[test]
    fn test_offset_from_utc_datetime() {
        let _rug_st_tests_llm_16_198_rrrruuuugggg_test_offset_from_utc_datetime = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 18;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let utc = Utc;
        let naive_date_time = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .naive_utc();
        let utc_offset = utc.offset_from_utc_datetime(&naive_date_time);
        debug_assert_eq!(utc_offset, Utc);
        let offset_secs = utc_offset.fix().local_minus_utc();
        debug_assert_eq!(offset_secs, 0);
        let _rug_ed_tests_llm_16_198_rrrruuuugggg_test_offset_from_utc_datetime = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_591 {
    use super::*;
    use crate::*;
    use crate::prelude::*;
    #[test]
    fn test_utc_now() {
        let _rug_st_tests_llm_16_591_rrrruuuugggg_test_utc_now = 0;
        let rug_fuzz_0 = 1000;
        let utc_now = Utc::now();
        let now = Utc::now();
        let diff = now.signed_duration_since(utc_now).num_milliseconds().abs();
        debug_assert!(
            diff < rug_fuzz_0,
            "The 'now' function did not return the current UTC datetime within an acceptable range."
        );
        let _rug_ed_tests_llm_16_591_rrrruuuugggg_test_utc_now = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_592 {
    use crate::{Date, Utc, TimeZone, NaiveDate};
    #[test]
    fn test_today() {
        let _rug_st_tests_llm_16_592_rrrruuuugggg_test_today = 0;
        let today_utc: Date<Utc> = Utc::today();
        let today_utc_naive: NaiveDate = Utc::today().naive_utc();
        let now_utc: NaiveDate = Utc::now().date().naive_utc();
        debug_assert_eq!(today_utc_naive, now_utc);
        let plus_one_day = today_utc.succ();
        let minus_one_day = today_utc.pred();
        debug_assert!(plus_one_day > today_utc);
        debug_assert!(minus_one_day < today_utc);
        let _rug_ed_tests_llm_16_592_rrrruuuugggg_test_today = 0;
    }
}
