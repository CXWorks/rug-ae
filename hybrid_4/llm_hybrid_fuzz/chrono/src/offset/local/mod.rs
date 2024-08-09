//! The local (system) time zone.
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
use super::fixed::FixedOffset;
use super::{LocalResult, TimeZone};
use crate::naive::{NaiveDate, NaiveDateTime};
#[allow(deprecated)]
use crate::{Date, DateTime};
#[cfg(
    all(
        not(unix),
        not(windows),
        not(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )
)]
#[path = "stub.rs"]
mod inner;
#[cfg(unix)]
#[path = "unix.rs"]
mod inner;
#[cfg(windows)]
#[path = "windows.rs"]
mod inner;
#[cfg(unix)]
mod tz_info;
/// The local timescale. This is implemented via the standard `time` crate.
///
/// Using the [`TimeZone`](./trait.TimeZone.html) methods
/// on the Local struct is the preferred way to construct `DateTime<Local>`
/// instances.
///
/// # Example
///
/// ```
/// use chrono::{Local, DateTime, TimeZone};
///
/// let dt: DateTime<Local> = Local::now();
/// let dt: DateTime<Local> = Local.timestamp(0, 0);
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Local;
impl Local {
    /// Returns a `Date` which corresponds to the current date.
    #[deprecated(since = "0.4.23", note = "use `Local::now()` instead")]
    #[allow(deprecated)]
    #[must_use]
    pub fn today() -> Date<Local> {
        Local::now().date()
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
    pub fn now() -> DateTime<Local> {
        inner::now()
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
    pub fn now() -> DateTime<Local> {
        use super::Utc;
        let now: DateTime<Utc> = super::Utc::now();
        let offset = FixedOffset::west_opt(
                (js_sys::Date::new_0().get_timezone_offset() as i32) * 60,
            )
            .unwrap();
        DateTime::from_utc(now.naive_utc(), offset)
    }
}
impl TimeZone for Local {
    type Offset = FixedOffset;
    fn from_offset(_offset: &FixedOffset) -> Local {
        Local
    }
    #[allow(deprecated)]
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<FixedOffset> {
        self.from_local_date(local).map(|date| *date.offset())
    }
    fn offset_from_local_datetime(
        &self,
        local: &NaiveDateTime,
    ) -> LocalResult<FixedOffset> {
        self.from_local_datetime(local).map(|datetime| *datetime.offset())
    }
    #[allow(deprecated)]
    fn offset_from_utc_date(&self, utc: &NaiveDate) -> FixedOffset {
        *self.from_utc_date(utc).offset()
    }
    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> FixedOffset {
        *self.from_utc_datetime(utc).offset()
    }
    #[allow(deprecated)]
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Local>> {
        let midnight = self.from_local_datetime(&local.and_hms_opt(0, 0, 0).unwrap());
        midnight.map(|datetime| Date::from_utc(*local, *datetime.offset()))
    }
    #[cfg(
        all(
            target_arch = "wasm32",
            feature = "wasmbind",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    fn from_local_datetime(
        &self,
        local: &NaiveDateTime,
    ) -> LocalResult<DateTime<Local>> {
        let mut local = local.clone();
        let offset = FixedOffset::west_opt(
                (js_sys::Date::new_0().get_timezone_offset() as i32) * 60,
            )
            .unwrap();
        local -= crate::TimeDelta::seconds(offset.local_minus_utc() as i64);
        LocalResult::Single(DateTime::from_utc(local, offset))
    }
    #[cfg(
        not(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )]
    fn from_local_datetime(
        &self,
        local: &NaiveDateTime,
    ) -> LocalResult<DateTime<Local>> {
        inner::naive_to_local(local, true)
    }
    #[allow(deprecated)]
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Local> {
        let midnight = self.from_utc_datetime(&utc.and_hms_opt(0, 0, 0).unwrap());
        Date::from_utc(*utc, *midnight.offset())
    }
    #[cfg(
        all(
            target_arch = "wasm32",
            feature = "wasmbind",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )
    )]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        let offset = FixedOffset::west_opt(
                (js_sys::Date::new_0().get_timezone_offset() as i32) * 60,
            )
            .unwrap();
        DateTime::from_utc(*utc, offset)
    }
    #[cfg(
        not(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Local> {
        inner::naive_to_local(utc, false).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::Local;
    use crate::offset::TimeZone;
    use crate::{Datelike, TimeDelta, Utc};
    #[test]
    fn verify_correct_offsets() {
        let now = Local::now();
        let from_local = Local.from_local_datetime(&now.naive_local()).unwrap();
        let from_utc = Local.from_utc_datetime(&now.naive_utc());
        assert_eq!(
            now.offset().local_minus_utc(), from_local.offset().local_minus_utc()
        );
        assert_eq!(now.offset().local_minus_utc(), from_utc.offset().local_minus_utc());
        assert_eq!(now, from_local);
        assert_eq!(now, from_utc);
    }
    #[test]
    fn verify_correct_offsets_distant_past() {
        let distant_past = Local::now() - TimeDelta::days(250 * 31);
        let from_local = Local.from_local_datetime(&distant_past.naive_local()).unwrap();
        let from_utc = Local.from_utc_datetime(&distant_past.naive_utc());
        assert_eq!(
            distant_past.offset().local_minus_utc(), from_local.offset()
            .local_minus_utc()
        );
        assert_eq!(
            distant_past.offset().local_minus_utc(), from_utc.offset().local_minus_utc()
        );
        assert_eq!(distant_past, from_local);
        assert_eq!(distant_past, from_utc);
    }
    #[test]
    fn verify_correct_offsets_distant_future() {
        let distant_future = Local::now() + TimeDelta::days(250 * 31);
        let from_local = Local
            .from_local_datetime(&distant_future.naive_local())
            .unwrap();
        let from_utc = Local.from_utc_datetime(&distant_future.naive_utc());
        assert_eq!(
            distant_future.offset().local_minus_utc(), from_local.offset()
            .local_minus_utc()
        );
        assert_eq!(
            distant_future.offset().local_minus_utc(), from_utc.offset()
            .local_minus_utc()
        );
        assert_eq!(distant_future, from_local);
        assert_eq!(distant_future, from_utc);
    }
    #[test]
    fn test_local_date_sanity_check() {
        assert_eq!(Local.with_ymd_and_hms(2999, 12, 28, 0, 0, 0).unwrap().day(), 28);
    }
    #[test]
    fn test_leap_second() {
        let today = Utc::now().date_naive();
        if let Some(dt) = today.and_hms_milli_opt(15, 2, 59, 1000) {
            let timestr = dt.time().to_string();
            assert!(
                timestr == "15:02:60" || timestr == "15:03:00",
                "unexpected timestr {:?}", timestr
            );
        }
        if let Some(dt) = today.and_hms_milli_opt(15, 2, 3, 1234) {
            let timestr = dt.time().to_string();
            assert!(
                timestr == "15:02:03.234" || timestr == "15:02:04.234",
                "unexpected timestr {:?}", timestr
            );
        }
    }
}
#[cfg(test)]
mod tests_llm_16_176 {
    use super::*;
    use crate::*;
    use crate::{Datelike, TimeZone, Weekday};
    fn make_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd(year, month, day)
    }
    #[test]
    fn test_from_local_date_for_existing_date() {
        let local_date = make_date(2023, 4, 1);
        let local = Local.from_local_date(&local_date);
        match local {
            LocalResult::Single(date) => {
                assert_eq!(date.year(), 2023);
                assert_eq!(date.month(), 4);
                assert_eq!(date.day(), 1);
                assert_eq!(date.weekday(), Weekday::Sat);
            }
            _ => panic!("Expected a single date result, found: {:?}", local),
        }
    }
    #[test]
    fn test_from_local_date_for_ambiguous_date() {
        let local_date = make_date(2023, 10, 29);
        let local = Local.from_local_date(&local_date);
        match local {
            LocalResult::Ambiguous(min, max) => {
                assert_eq!(min.year(), 2023);
                assert_eq!(min.month(), 10);
                assert_eq!(min.day(), 29);
                assert_eq!(min.weekday(), Weekday::Sun);
                assert_eq!(max.year(), 2023);
                assert_eq!(max.month(), 10);
                assert_eq!(max.day(), 29);
                assert_eq!(max.weekday(), Weekday::Sun);
                assert!(min < max, "Expected min date to be earlier than max date");
            }
            _ => panic!("Expected an ambiguous date result, found: {:?}", local),
        }
    }
    #[test]
    fn test_from_local_date_for_nonexistent_date() {
        let local_date = make_date(2023, 3, 32);
        let local = Local.from_local_date(&local_date);
        match local {
            LocalResult::None => {}
            _ => panic!("Expected no date result, found: {:?}", local),
        }
    }
    #[test]
    fn test_from_local_date_for_boundary_min() {
        let local_date = NaiveDate::MIN;
        let local = Local.from_local_date(&local_date);
        assert!(
            matches!(local, LocalResult::Single(_)),
            "Expected a single date result for NaiveDate::MIN, found: {:?}", local
        );
    }
    #[test]
    fn test_from_local_date_for_boundary_max() {
        let local_date = NaiveDate::MAX;
        let local = Local.from_local_date(&local_date);
        assert!(
            matches!(local, LocalResult::Single(_)),
            "Expected a single date result for NaiveDate::MAX, found: {:?}", local
        );
    }
}
#[cfg(test)]
mod tests_llm_16_177 {
    use super::*;
    use crate::*;
    use crate::offset::{Local, TimeZone};
    use crate::naive::datetime::NaiveDateTime;
    use crate::DateTime;
    #[test]
    fn test_from_local_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let local = Local;
        let naive_dt = NaiveDateTime::from_timestamp(rug_fuzz_0, rug_fuzz_1);
        let local_dt = local.from_local_datetime(&naive_dt);
        match local_dt {
            LocalResult::None => panic!("LocalResult::None: no corresponding local time"),
            LocalResult::Single(dt) => {
                let expected_dt: DateTime<Local> = Local
                    .timestamp(rug_fuzz_2, rug_fuzz_3);
                debug_assert_eq!(dt, expected_dt);
            }
            LocalResult::Ambiguous(min, max) => {
                panic!("LocalResult::Ambiguous: min = {:?}, max = {:?}", min, max);
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_178 {
    use crate::offset::{TimeZone, Local, FixedOffset};
    #[test]
    fn test_from_offset() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let offset = FixedOffset::east(rug_fuzz_0);
        let local_from_offset = Local::from_offset(&offset);
        debug_assert_eq!(format!("{:?}", local_from_offset), "Local");
        let offset_west = FixedOffset::west(rug_fuzz_1);
        let local_from_offset_west = Local::from_offset(&offset_west);
        debug_assert_eq!(format!("{:?}", local_from_offset_west), "Local");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_180 {
    use super::*;
    use crate::*;
    use crate::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};
    #[cfg(feature = "std")]
    #[test]
    fn test_from_utc_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u32, i32, i32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let fixed_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        let offset = FixedOffset::east(rug_fuzz_7 * rug_fuzz_8);
        let datetime = offset.from_utc_datetime(&fixed_time);
        debug_assert_eq!(datetime.year(), 2023);
        debug_assert_eq!(datetime.month(), 7);
        debug_assert_eq!(datetime.day(), 14);
        debug_assert_eq!(datetime.weekday(), Weekday::Fri);
        debug_assert_eq!(datetime.hour(), 19);
        debug_assert_eq!(datetime.minute(), 56);
        debug_assert_eq!(datetime.second(), 4);
        debug_assert_eq!(datetime.timestamp_subsec_millis(), 127);
        let leap_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
            NaiveTime::from_hms_milli(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15),
        );
        let datetime = offset.from_utc_datetime(&leap_time);
        debug_assert_eq!(datetime.year(), 2024);
        debug_assert_eq!(datetime.month(), 1);
        debug_assert_eq!(datetime.day(), 1);
        debug_assert_eq!(datetime.weekday(), Weekday::Mon);
        debug_assert_eq!(datetime.hour(), 7);
        debug_assert_eq!(datetime.minute(), 59);
        debug_assert_eq!(datetime.second(), 59);
        debug_assert_eq!(datetime.timestamp_subsec_millis(), 1000);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_181_llm_16_181 {
    use crate::{Local, NaiveDate, TimeZone};
    use crate::offset::LocalResult::Single;
    #[test]
    fn test_offset_from_local_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, i32, u32, u32, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let local = Local;
        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let res = local.offset_from_local_date(&date);
        debug_assert_eq!(res, Single(* local.from_local_date(& date).unwrap().offset()));
        let dst_date = NaiveDate::from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dst_res = local.offset_from_local_date(&dst_date);
        match dst_res {
            Single(offset) => {
                debug_assert_eq!(
                    offset, * local.from_local_date(& dst_date).unwrap().offset()
                )
            }
            _ => {
                debug_assert!(
                    rug_fuzz_6, "DST date should yield a single result in most timezones"
                )
            }
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_182 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, TimeZone};
    #[test]
    fn test_offset_from_local_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_datetime = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = <Local as TimeZone>::offset_from_local_datetime(
            &Local,
            &naive_datetime,
        );
        let expected_offset = FixedOffset::east(rug_fuzz_6);
        match result {
            LocalResult::Single(offset) => {
                debug_assert_eq!(offset, expected_offset);
            }
            _ => panic!("Expected single fixed offset result."),
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_184 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, Utc};
    use crate::naive::{NaiveDate, NaiveTime, NaiveDateTime};
    #[test]
    fn test_offset_from_utc_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let utc_dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_offset = Local.offset_from_utc_datetime(&utc_dt.naive_utc());
        let local_dt = utc_dt.with_timezone(&local_offset);
        let offset_seconds = local_offset.local_minus_utc();
        let utc_naive = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8),
            NaiveTime::from_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
        );
        let expected_naive = utc_naive + TimeDelta::seconds(i64::from(offset_seconds));
        let expected_dt = DateTime::<Utc>::from_utc(expected_naive, Utc)
            .with_timezone(&local_offset);
        debug_assert_eq!(local_dt, expected_dt);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_517 {
    use super::*;
    use crate::*;
    use crate::{DateTime, Local, TimeZone, Utc};
    #[test]
    fn test_now() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let local_now: DateTime<Local> = Local::now();
        let utc_now: DateTime<Utc> = Utc::now();
        debug_assert!((local_now.timestamp() - utc_now.timestamp()).abs() < rug_fuzz_0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::offset::local::Local;
    use crate::Date;
    #[test]
    fn test_today() {
        let _rug_st_tests_rug_189_rrrruuuugggg_test_today = 0;
        let today: Date<Local> = Local::today();
        let now = Local::now().date();
        debug_assert_eq!(today, now);
        let _rug_ed_tests_rug_189_rrrruuuugggg_test_today = 0;
    }
}
#[cfg(test)]
mod tests_rug_190 {
    use crate::{Local, NaiveDate, TimeZone, FixedOffset};
    #[test]
    fn test_offset_from_utc_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Local;
        let mut p1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let result: FixedOffset = <Local as TimeZone>::offset_from_utc_date(&p0, &p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_191 {
    use crate::{Local, NaiveDate, TimeZone, Date};
    use crate::offset;
    #[test]
    fn test_from_utc_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Local;
        let mut p1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let result = <offset::local::Local as offset::TimeZone>::from_utc_date(&p0, &p1);
        debug_assert_eq!(result.naive_utc(), p1);
             }
});    }
}
