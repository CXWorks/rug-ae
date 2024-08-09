//! The time zone, which calculates offsets from the local time to UTC.
//!
//! There are four operations provided by the `TimeZone` trait:
//!
//! 1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
//! 2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
//! 3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
//! 4. Constructing `DateTime<Tz>` objects from various offsets
//!
//! 1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
//! 3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
//! which implements `Offset` (which then passed to `TimeZone` for actual implementations).
//! Technically speaking `TimeZone` has a total knowledge about given timescale,
//! but `Offset` is used as a cache to avoid the repeated conversion
//! and provides implementations for 1 and 3.
//! An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.
use core::fmt;
use crate::format::{parse, ParseResult, Parsed, StrftimeItems};
use crate::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::Weekday;
#[allow(deprecated)]
use crate::{Date, DateTime};
mod fixed;
pub use self::fixed::FixedOffset;
#[cfg(feature = "clock")]
mod local;
#[cfg(feature = "clock")]
pub use self::local::Local;
mod utc;
pub use self::utc::Utc;
/// The conversion result from the local time to the timezone-aware datetime types.
#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum LocalResult<T> {
    /// Given local time representation is invalid.
    /// This can occur when, for example, the positive timezone transition.
    None,
    /// Given local time representation has a single unique result.
    Single(T),
    /// Given local time representation has multiple results and thus ambiguous.
    /// This can occur when, for example, the negative timezone transition.
    Ambiguous(T, T),
}
impl<T> LocalResult<T> {
    /// Returns `Some` only when the conversion result is unique, or `None` otherwise.
    #[must_use]
    pub fn single(self) -> Option<T> {
        match self {
            LocalResult::Single(t) => Some(t),
            _ => None,
        }
    }
    /// Returns `Some` for the earliest possible conversion result, or `None` if none.
    #[must_use]
    pub fn earliest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(t, _) => Some(t),
            _ => None,
        }
    }
    /// Returns `Some` for the latest possible conversion result, or `None` if none.
    #[must_use]
    pub fn latest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(_, t) => Some(t),
            _ => None,
        }
    }
    /// Maps a `LocalResult<T>` into `LocalResult<U>` with given function.
    #[must_use]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> LocalResult<U> {
        match self {
            LocalResult::None => LocalResult::None,
            LocalResult::Single(v) => LocalResult::Single(f(v)),
            LocalResult::Ambiguous(min, max) => LocalResult::Ambiguous(f(min), f(max)),
        }
    }
}
#[allow(deprecated)]
impl<Tz: TimeZone> LocalResult<Date<Tz>> {
    /// Makes a new `DateTime` from the current date and given `NaiveTime`.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_time(self, time: NaiveTime) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d.and_time(time).map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the current date, hour, minute and second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d
                    .and_hms_opt(hour, min, sec)
                    .map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
    /// The millisecond part can exceed 1,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_milli_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d
                    .and_hms_milli_opt(hour, min, sec, milli)
                    .map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
    /// The microsecond part can exceed 1,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_micro_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d
                    .and_hms_micro_opt(hour, min, sec, micro)
                    .map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
    /// The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.
    /// The offset in the current date is preserved.
    ///
    /// Propagates any error. Ambiguous result would be discarded.
    #[inline]
    #[must_use]
    pub fn and_hms_nano_opt(
        self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> LocalResult<DateTime<Tz>> {
        match self {
            LocalResult::Single(d) => {
                d
                    .and_hms_nano_opt(hour, min, sec, nano)
                    .map_or(LocalResult::None, LocalResult::Single)
            }
            _ => LocalResult::None,
        }
    }
}
impl<T: fmt::Debug> LocalResult<T> {
    /// Returns the single unique conversion result, or panics accordingly.
    #[must_use]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            LocalResult::None => panic!("No such local time"),
            LocalResult::Single(t) => t,
            LocalResult::Ambiguous(t1, t2) => {
                panic!("Ambiguous local time, ranging from {:?} to {:?}", t1, t2)
            }
        }
    }
}
/// The offset from the local time to UTC.
pub trait Offset: Sized + Clone + fmt::Debug {
    /// Returns the fixed offset from UTC to the local time stored.
    fn fix(&self) -> FixedOffset;
}
/// The time zone.
///
/// The methods here are the primarily constructors for [`Date`](../struct.Date.html) and
/// [`DateTime`](../struct.DateTime.html) types.
pub trait TimeZone: Sized + Clone {
    /// An associated offset type.
    /// This type is used to store the actual offset in date and time types.
    /// The original `TimeZone` value can be recovered via `TimeZone::from_offset`.
    type Offset: Offset;
    /// Make a new `DateTime` from year, month, day, time components and current time zone.
    ///
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// Returns `LocalResult::None` on invalid input data.
    fn with_ymd_and_hms(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> LocalResult<DateTime<Self>> {
        match NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|d| d.and_hms_opt(hour, min, sec))
        {
            Some(dt) => self.from_local_datetime(&dt),
            None => LocalResult::None,
        }
    }
    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date, invalid month and/or day.
    #[deprecated(since = "0.4.23", note = "use `with_ymd_and_hms()` instead")]
    #[allow(deprecated)]
    fn ymd(&self, year: i32, month: u32, day: u32) -> Date<Self> {
        self.ymd_opt(year, month, day).unwrap()
    }
    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date, invalid month and/or day.
    #[deprecated(since = "0.4.23", note = "use `with_ymd_and_hms()` instead")]
    #[allow(deprecated)]
    fn ymd_opt(&self, year: i32, month: u32, day: u32) -> LocalResult<Date<Self>> {
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }
    /// Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date and/or invalid DOY.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn yo(&self, year: i32, ordinal: u32) -> Date<Self> {
        self.yo_opt(year, ordinal).unwrap()
    }
    /// Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date and/or invalid DOY.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn yo_opt(&self, year: i32, ordinal: u32) -> LocalResult<Date<Self>> {
        match NaiveDate::from_yo_opt(year, ordinal) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }
    /// Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
    /// the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    /// The resulting `Date` may have a different year from the input year.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date and/or invalid week number.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn isoywd(&self, year: i32, week: u32, weekday: Weekday) -> Date<Self> {
        self.isoywd_opt(year, week, weekday).unwrap()
    }
    /// Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
    /// the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    /// The resulting `Date` may have a different year from the input year.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date and/or invalid week number.
    #[deprecated(
        since = "0.4.23",
        note = "use `from_local_datetime()` with a `NaiveDateTime` instead"
    )]
    #[allow(deprecated)]
    fn isoywd_opt(
        &self,
        year: i32,
        week: u32,
        weekday: Weekday,
    ) -> LocalResult<Date<Self>> {
        match NaiveDate::from_isoywd_opt(year, week, weekday) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Panics on the out-of-range number of seconds and/or invalid nanosecond,
    /// for a non-panicking version see [`timestamp_opt`](#method.timestamp_opt).
    #[deprecated(since = "0.4.23", note = "use `timestamp_opt()` instead")]
    fn timestamp(&self, secs: i64, nsecs: u32) -> DateTime<Self> {
        self.timestamp_opt(secs, nsecs).unwrap()
    }
    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Returns `LocalResult::None` on out-of-range number of seconds and/or
    /// invalid nanosecond, otherwise always returns `LocalResult::Single`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_opt(1431648000, 0).unwrap().to_string(), "2015-05-15 00:00:00 UTC");
    /// ```
    fn timestamp_opt(&self, secs: i64, nsecs: u32) -> LocalResult<DateTime<Self>> {
        match NaiveDateTime::from_timestamp_opt(secs, nsecs) {
            Some(dt) => LocalResult::Single(self.from_utc_datetime(&dt)),
            None => LocalResult::None,
        }
    }
    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Panics on out-of-range number of milliseconds for a non-panicking
    /// version see [`timestamp_millis_opt`](#method.timestamp_millis_opt).
    #[deprecated(since = "0.4.23", note = "use `timestamp_millis_opt()` instead")]
    fn timestamp_millis(&self, millis: i64) -> DateTime<Self> {
        self.timestamp_millis_opt(millis).unwrap()
    }
    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    ///
    /// Returns `LocalResult::None` on out-of-range number of milliseconds
    /// and/or invalid nanosecond, otherwise always returns
    /// `LocalResult::Single`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, LocalResult};
    /// match Utc.timestamp_millis_opt(1431648000) {
    ///     LocalResult::Single(dt) => assert_eq!(dt.timestamp(), 1431648),
    ///     _ => panic!("Incorrect timestamp_millis"),
    /// };
    /// ```
    fn timestamp_millis_opt(&self, millis: i64) -> LocalResult<DateTime<Self>> {
        let (mut secs, mut millis) = (millis / 1000, millis % 1000);
        if millis < 0 {
            secs -= 1;
            millis += 1000;
        }
        self.timestamp_opt(secs, millis as u32 * 1_000_000)
    }
    /// Makes a new `DateTime` from the number of non-leap nanoseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Unlike [`timestamp_millis`](#method.timestamp_millis), this never
    /// panics.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_nanos(1431648000000000).timestamp(), 1431648);
    /// ```
    fn timestamp_nanos(&self, nanos: i64) -> DateTime<Self> {
        let (mut secs, mut nanos) = (nanos / 1_000_000_000, nanos % 1_000_000_000);
        if nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
        self.timestamp_opt(secs, nanos as u32).unwrap()
    }
    /// Parses a string with the specified format string and returns a
    /// `DateTime` with the current offset.
    ///
    /// See the [`crate::format::strftime`] module on the
    /// supported escape sequences.
    ///
    /// If the to-be-parsed string includes an offset, it *must* match the
    /// offset of the TimeZone, otherwise an error will be returned.
    ///
    /// See also [`DateTime::parse_from_str`] which gives a [`DateTime`] with
    /// parsed [`FixedOffset`].
    fn datetime_from_str(&self, s: &str, fmt: &str) -> ParseResult<DateTime<Self>> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_datetime_with_timezone(self)
    }
    /// Reconstructs the time zone from the offset.
    fn from_offset(offset: &Self::Offset) -> Self;
    /// Creates the offset(s) for given local `NaiveDate` if possible.
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>;
    /// Creates the offset(s) for given local `NaiveDateTime` if possible.
    fn offset_from_local_datetime(
        &self,
        local: &NaiveDateTime,
    ) -> LocalResult<Self::Offset>;
    /// Converts the local `NaiveDate` to the timezone-aware `Date` if possible.
    #[allow(clippy::wrong_self_convention)]
    #[deprecated(since = "0.4.23", note = "use `from_local_datetime()` instead")]
    #[allow(deprecated)]
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Self>> {
        self.offset_from_local_date(local)
            .map(|offset| { Date::from_utc(*local, offset) })
    }
    /// Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.
    #[allow(clippy::wrong_self_convention)]
    fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<DateTime<Self>> {
        self.offset_from_local_datetime(local)
            .map(|offset| DateTime::from_utc(*local - offset.fix(), offset))
    }
    /// Creates the offset for given UTC `NaiveDate`. This cannot fail.
    fn offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset;
    /// Creates the offset for given UTC `NaiveDateTime`. This cannot fail.
    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset;
    /// Converts the UTC `NaiveDate` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    #[allow(clippy::wrong_self_convention)]
    #[deprecated(since = "0.4.23", note = "use `from_utc_datetime()` instead")]
    #[allow(deprecated)]
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Self> {
        Date::from_utc(*utc, self.offset_from_utc_date(utc))
    }
    /// Converts the UTC `NaiveDateTime` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    #[allow(clippy::wrong_self_convention)]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self> {
        DateTime::from_utc(*utc, self.offset_from_utc_datetime(utc))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative_millis() {
        let dt = Utc.timestamp_millis_opt(-1000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59 UTC");
        let dt = Utc.timestamp_millis_opt(-7000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:53 UTC");
        let dt = Utc.timestamp_millis_opt(-7001).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:52.999 UTC");
        let dt = Utc.timestamp_millis_opt(-7003).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:52.997 UTC");
        let dt = Utc.timestamp_millis_opt(-999).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.001 UTC");
        let dt = Utc.timestamp_millis_opt(-1).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.999 UTC");
        let dt = Utc.timestamp_millis_opt(-60000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:59:00 UTC");
        let dt = Utc.timestamp_millis_opt(-3600000).unwrap();
        assert_eq!(dt.to_string(), "1969-12-31 23:00:00 UTC");
        for (millis, expected) in &[
            (-7000, "1969-12-31 23:59:53 UTC"),
            (-7001, "1969-12-31 23:59:52.999 UTC"),
            (-7003, "1969-12-31 23:59:52.997 UTC"),
        ] {
            match Utc.timestamp_millis_opt(*millis) {
                LocalResult::Single(dt) => {
                    assert_eq!(dt.to_string(), * expected);
                }
                e => panic!("Got {:?} instead of an okay answer", e),
            }
        }
    }
    #[test]
    fn test_negative_nanos() {
        let dt = Utc.timestamp_nanos(-1_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59 UTC");
        let dt = Utc.timestamp_nanos(-999_999_999);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.000000001 UTC");
        let dt = Utc.timestamp_nanos(-1);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:59.999999999 UTC");
        let dt = Utc.timestamp_nanos(-60_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:59:00 UTC");
        let dt = Utc.timestamp_nanos(-3_600_000_000_000);
        assert_eq!(dt.to_string(), "1969-12-31 23:00:00 UTC");
    }
    #[test]
    fn test_nanos_never_panics() {
        Utc.timestamp_nanos(i64::max_value());
        Utc.timestamp_nanos(i64::default());
        Utc.timestamp_nanos(i64::min_value());
    }
}
#[cfg(test)]
mod tests_llm_16_477 {
    use crate::offset::LocalResult;
    #[test]
    fn earliest_with_single() {
        let _rug_st_tests_llm_16_477_rrrruuuugggg_earliest_with_single = 0;
        let rug_fuzz_0 = 10;
        let single = LocalResult::Single(rug_fuzz_0);
        debug_assert_eq!(single.earliest(), Some(10));
        let _rug_ed_tests_llm_16_477_rrrruuuugggg_earliest_with_single = 0;
    }
    #[test]
    fn earliest_with_ambiguous() {
        let _rug_st_tests_llm_16_477_rrrruuuugggg_earliest_with_ambiguous = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let ambiguous = LocalResult::Ambiguous(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(ambiguous.earliest(), Some(10));
        let _rug_ed_tests_llm_16_477_rrrruuuugggg_earliest_with_ambiguous = 0;
    }
    #[test]
    fn earliest_with_none() {
        let _rug_st_tests_llm_16_477_rrrruuuugggg_earliest_with_none = 0;
        let none: LocalResult<i32> = LocalResult::None;
        debug_assert_eq!(none.earliest(), None);
        let _rug_ed_tests_llm_16_477_rrrruuuugggg_earliest_with_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_478 {
    use crate::LocalResult;
    #[test]
    fn test_latest_none() {
        let _rug_st_tests_llm_16_478_rrrruuuugggg_test_latest_none = 0;
        let result: LocalResult<i32> = LocalResult::None;
        debug_assert_eq!(result.latest(), None);
        let _rug_ed_tests_llm_16_478_rrrruuuugggg_test_latest_none = 0;
    }
    #[test]
    fn test_latest_single() {
        let _rug_st_tests_llm_16_478_rrrruuuugggg_test_latest_single = 0;
        let rug_fuzz_0 = 42;
        let result: LocalResult<i32> = LocalResult::Single(rug_fuzz_0);
        debug_assert_eq!(result.latest(), Some(42));
        let _rug_ed_tests_llm_16_478_rrrruuuugggg_test_latest_single = 0;
    }
    #[test]
    fn test_latest_ambiguous() {
        let _rug_st_tests_llm_16_478_rrrruuuugggg_test_latest_ambiguous = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 42;
        let result: LocalResult<i32> = LocalResult::Ambiguous(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(result.latest(), Some(42));
        let _rug_ed_tests_llm_16_478_rrrruuuugggg_test_latest_ambiguous = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_480 {
    use super::*;
    use crate::*;
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct MockTime(i32);
    #[test]
    fn test_single_with_single_result() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_single_with_single_result = 0;
        let rug_fuzz_0 = 1;
        let result = LocalResult::Single(MockTime(rug_fuzz_0));
        debug_assert_eq!(result.single(), Some(MockTime(1)));
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_single_with_single_result = 0;
    }
    #[test]
    fn test_single_with_no_result() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_single_with_no_result = 0;
        let result: LocalResult<MockTime> = LocalResult::None;
        debug_assert_eq!(result.single(), None);
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_single_with_no_result = 0;
    }
    #[test]
    fn test_single_with_ambiguous_result() {
        let _rug_st_tests_llm_16_480_rrrruuuugggg_test_single_with_ambiguous_result = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let result = LocalResult::Ambiguous(MockTime(rug_fuzz_0), MockTime(rug_fuzz_1));
        debug_assert_eq!(result.single(), None);
        let _rug_ed_tests_llm_16_480_rrrruuuugggg_test_single_with_ambiguous_result = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_481 {
    use super::*;
    use crate::*;
    use crate::offset::{LocalResult, TimeZone};
    use crate::{NaiveDate, NaiveDateTime, Utc};
    #[test]
    #[should_panic(expected = "No such local time")]
    fn unwrap_none_should_panic() {
        let _rug_st_tests_llm_16_481_rrrruuuugggg_unwrap_none_should_panic = 0;
        let result: LocalResult<NaiveDateTime> = LocalResult::None;
        result.unwrap();
        let _rug_ed_tests_llm_16_481_rrrruuuugggg_unwrap_none_should_panic = 0;
    }
    #[test]
    fn unwrap_single_should_return_value() {
        let _rug_st_tests_llm_16_481_rrrruuuugggg_unwrap_single_should_return_value = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let expected = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let result: LocalResult<NaiveDateTime> = LocalResult::Single(expected);
        let actual = result.unwrap();
        debug_assert_eq!(actual, expected);
        let _rug_ed_tests_llm_16_481_rrrruuuugggg_unwrap_single_should_return_value = 0;
    }
    #[test]
    #[should_panic(expected = "Ambiguous local time, ranging from")]
    fn unwrap_ambiguous_should_panic() {
        let _rug_st_tests_llm_16_481_rrrruuuugggg_unwrap_ambiguous_should_panic = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 10;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let time_min = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let time_max = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let result: LocalResult<NaiveDateTime> = LocalResult::Ambiguous(
            time_min,
            time_max,
        );
        result.unwrap();
        let _rug_ed_tests_llm_16_481_rrrruuuugggg_unwrap_ambiguous_should_panic = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_482_llm_16_482 {
    use crate::{Date, DateTime, LocalResult, TimeZone, Utc};
    use crate::traits::Timelike;
    #[test]
    fn and_hms_micro_opt_single_valid() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_valid = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789123;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_local_result = LocalResult::Single(date_utc);
        let hour = rug_fuzz_3;
        let minute = rug_fuzz_4;
        let second = rug_fuzz_5;
        let microsecond = rug_fuzz_6;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::Single(datetime) => {
                debug_assert_eq!(datetime.hour(), hour);
                debug_assert_eq!(datetime.minute(), minute);
                debug_assert_eq!(datetime.second(), second);
                debug_assert_eq!(datetime.nanosecond(), microsecond * 1_000);
            }
            _ => panic!("Expected a single DateTime result, got {:?}", datetime_result),
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_valid = 0;
    }
    #[test]
    fn and_hms_micro_opt_single_invalid_hour() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_hour = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 25;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789123;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_local_result = LocalResult::Single(date_utc);
        let hour = rug_fuzz_3;
        let minute = rug_fuzz_4;
        let second = rug_fuzz_5;
        let microsecond = rug_fuzz_6;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::None => {}
            _ => {
                panic!(
                    "Expected LocalResult::None due to invalid hour, got {:?}",
                    datetime_result
                )
            }
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_hour = 0;
    }
    #[test]
    fn and_hms_micro_opt_single_invalid_minute() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_minute = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 60;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789123;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_local_result = LocalResult::Single(date_utc);
        let hour = rug_fuzz_3;
        let minute = rug_fuzz_4;
        let second = rug_fuzz_5;
        let microsecond = rug_fuzz_6;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::None => {}
            _ => {
                panic!(
                    "Expected LocalResult::None due to invalid minute, got {:?}",
                    datetime_result
                )
            }
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_minute = 0;
    }
    #[test]
    fn and_hms_micro_opt_single_invalid_second() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 61;
        let rug_fuzz_6 = 789123;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_local_result = LocalResult::Single(date_utc);
        let hour = rug_fuzz_3;
        let minute = rug_fuzz_4;
        let second = rug_fuzz_5;
        let microsecond = rug_fuzz_6;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::None => {}
            _ => {
                panic!(
                    "Expected LocalResult::None due to invalid second, got {:?}",
                    datetime_result
                )
            }
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_invalid_second = 0;
    }
    #[test]
    fn and_hms_micro_opt_single_leap_second() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_leap_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 60;
        let rug_fuzz_6 = 1_500_000;
        let date_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_local_result = LocalResult::Single(date_utc);
        let hour = rug_fuzz_3;
        let minute = rug_fuzz_4;
        let second = rug_fuzz_5;
        let microsecond = rug_fuzz_6;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::Single(datetime) => {
                debug_assert_eq!(datetime.hour(), hour);
                debug_assert_eq!(datetime.minute(), minute);
                debug_assert_eq!(datetime.second(), second - 1);
                debug_assert_eq!(
                    datetime.nanosecond(), (microsecond - 1_000_000) * 1_000
                );
            }
            _ => {
                panic!(
                    "Expected a single DateTime result including leap second, got {:?}",
                    datetime_result
                )
            }
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_single_leap_second = 0;
    }
    #[test]
    fn and_hms_micro_opt_ambiguous() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_ambiguous = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 4;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 12;
        let rug_fuzz_7 = 34;
        let rug_fuzz_8 = 56;
        let rug_fuzz_9 = 789123;
        let date1_utc: Date<Utc> = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date2_utc: Date<Utc> = Utc.ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date_local_result = LocalResult::Ambiguous(date1_utc, date2_utc);
        let hour = rug_fuzz_6;
        let minute = rug_fuzz_7;
        let second = rug_fuzz_8;
        let microsecond = rug_fuzz_9;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        match datetime_result {
            LocalResult::None => {}
            _ => {
                panic!(
                    "Expected LocalResult::None due to ambiguous date, got {:?}",
                    datetime_result
                )
            }
        }
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_ambiguous = 0;
    }
    #[test]
    fn and_hms_micro_opt_none() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_none = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789123;
        let date_local_result: LocalResult<Date<Utc>> = LocalResult::None;
        let hour = rug_fuzz_0;
        let minute = rug_fuzz_1;
        let second = rug_fuzz_2;
        let microsecond = rug_fuzz_3;
        let datetime_result = date_local_result
            .and_hms_micro_opt(hour, minute, second, microsecond);
        debug_assert!(
            matches!(datetime_result, LocalResult::None),
            "Expected LocalResult::None, got {:?}", datetime_result
        );
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_and_hms_micro_opt_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_485 {
    use crate::prelude::*;
    use crate::offset::{LocalResult, TimeZone};
    use crate::DateTime;
    #[test]
    fn test_and_hms_opt_none() {
        let _rug_st_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_none = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let local_none: LocalResult<Date<Utc>> = LocalResult::None;
        let result = local_none.and_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(matches!(result, LocalResult::None));
        let _rug_ed_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_none = 0;
    }
    #[test]
    fn test_and_hms_opt_single() {
        let _rug_st_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_single = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 25;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let local_single: LocalResult<Date<Utc>> = LocalResult::Single(date);
        let result = local_single.and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        match result {
            LocalResult::Single(date_time) => {
                debug_assert_eq!(date_time.hour(), 10);
                debug_assert_eq!(date_time.minute(), 10);
                debug_assert_eq!(date_time.second(), 10);
            }
            _ => panic!("Expected Single variant"),
        }
        let _rug_ed_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_single = 0;
    }
    #[test]
    fn test_and_hms_opt_ambiguous() {
        let _rug_st_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_ambiguous = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 25;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 26;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = 10;
        let date_early = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let date_late = Utc.ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_ambiguous: LocalResult<Date<Utc>> = LocalResult::Ambiguous(
            date_early,
            date_late,
        );
        let result = local_ambiguous.and_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert!(matches!(result, LocalResult::None));
        let _rug_ed_tests_llm_16_485_rrrruuuugggg_test_and_hms_opt_ambiguous = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_489 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, Utc};
    use crate::naive::{NaiveDate, NaiveTime};
    use crate::DateTime;
    #[test]
    fn test_from_local_datetime() {
        let _rug_st_tests_llm_16_489_rrrruuuugggg_test_from_local_datetime = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 30;
        let rug_fuzz_11 = 45;
        let naive_dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let utc_dt: DateTime<Utc> = Utc.from_local_datetime(&naive_dt).unwrap();
        let expected_utc_dt = Utc
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(utc_dt, expected_utc_dt);
        let _rug_ed_tests_llm_16_489_rrrruuuugggg_test_from_local_datetime = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_495 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc};
    #[test]
    fn test_fixed_timestamp_millis() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_test_fixed_timestamp_millis = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let offset = FixedOffset::east(rug_fuzz_0);
        let expected = DateTime::<
            Utc,
        >::from_utc(NaiveDateTime::from_timestamp(rug_fuzz_1, rug_fuzz_2), Utc);
        let result = offset.timestamp_millis(rug_fuzz_3);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_test_fixed_timestamp_millis = 0;
    }
    #[test]
    fn test_positive_fixed_timestamp_millis() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_test_positive_fixed_timestamp_millis = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_500;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1_500_000;
        let offset = FixedOffset::east(rug_fuzz_0);
        let expected = DateTime::<
            Utc,
        >::from_utc(NaiveDateTime::from_timestamp(rug_fuzz_1, rug_fuzz_2), Utc);
        let result = offset.timestamp_millis(rug_fuzz_3);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_test_positive_fixed_timestamp_millis = 0;
    }
    #[test]
    fn test_negative_fixed_timestamp_millis() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_test_negative_fixed_timestamp_millis = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1_500;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1_500_000;
        let offset = FixedOffset::west(rug_fuzz_0);
        let expected = DateTime::<
            Utc,
        >::from_utc(NaiveDateTime::from_timestamp(-rug_fuzz_1, rug_fuzz_2), Utc);
        let result = offset.timestamp_millis(-rug_fuzz_3);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_test_negative_fixed_timestamp_millis = 0;
    }
    #[test]
    fn test_fixed_timestamp_millis_with_nanoseconds() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_test_fixed_timestamp_millis_with_nanoseconds = 0;
        let rug_fuzz_0 = 5000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 500_000_000;
        let rug_fuzz_3 = 1_000_500;
        let offset = FixedOffset::east(rug_fuzz_0);
        let expected = DateTime::<
            Utc,
        >::from_utc(NaiveDateTime::from_timestamp(rug_fuzz_1, rug_fuzz_2), Utc);
        let result = offset.timestamp_millis(rug_fuzz_3);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_test_fixed_timestamp_millis_with_nanoseconds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_496 {
    use crate::{TimeZone, Utc, LocalResult};
    #[test]
    fn timestamp_millis_opt_valid() {
        let _rug_st_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_valid = 0;
        let rug_fuzz_0 = 1_575_448_051_987;
        let result = Utc.timestamp_millis_opt(rug_fuzz_0);
        match result {
            LocalResult::Single(dt) => {
                debug_assert_eq!(dt.timestamp(), 1_575_448_051);
                debug_assert_eq!(dt.timestamp_subsec_millis(), 987);
            }
            _ => panic!("Expected single result for valid timestamp"),
        }
        let _rug_ed_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_valid = 0;
    }
    #[test]
    fn timestamp_millis_opt_invalid_millis() {
        let _rug_st_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_invalid_millis = 0;
        let rug_fuzz_0 = true;
        let result = Utc.timestamp_millis_opt(i64::MAX);
        match result {
            LocalResult::None => {
                debug_assert!(
                    rug_fuzz_0, "Expected no result for out of range timestamp"
                )
            }
            _ => panic!("Expected no result for out of range timestamp"),
        }
        let _rug_ed_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_invalid_millis = 0;
    }
    #[test]
    fn timestamp_millis_opt_negative_millis() {
        let _rug_st_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_negative_millis = 0;
        let rug_fuzz_0 = 1;
        let result = Utc.timestamp_millis_opt(-rug_fuzz_0);
        match result {
            LocalResult::Single(dt) => {
                debug_assert_eq!(dt.timestamp(), - 1);
                debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
            }
            _ => panic!("Expected single result for valid negative timestamp"),
        }
        let _rug_ed_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_negative_millis = 0;
    }
    #[test]
    fn timestamp_millis_opt_zero() {
        let _rug_st_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_zero = 0;
        let rug_fuzz_0 = 0;
        let result = Utc.timestamp_millis_opt(rug_fuzz_0);
        debug_assert_eq!(result, LocalResult::Single(Utc.timestamp(0, 0)));
        let _rug_ed_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_zero = 0;
    }
    #[test]
    fn timestamp_millis_opt_boundary() {
        let _rug_st_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_boundary = 0;
        let rug_fuzz_0 = 1;
        let result = Utc.timestamp_millis_opt(-rug_fuzz_0);
        match result {
            LocalResult::Single(dt) => {
                debug_assert_eq!(dt.timestamp(), - 1);
                debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
            }
            _ => panic!("Expected single result for valid timestamp"),
        }
        let _rug_ed_tests_llm_16_496_rrrruuuugggg_timestamp_millis_opt_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_497 {
    use super::*;
    use crate::*;
    use crate::{TimeZone, Utc, FixedOffset};
    #[test]
    fn test_timestamp_nanos() {
        let _rug_st_tests_llm_16_497_rrrruuuugggg_test_timestamp_nanos = 0;
        let rug_fuzz_0 = 1_000_000_000_000_000_000;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 1_000_000_000_000_000_000;
        let utc = Utc.timestamp_nanos(rug_fuzz_0);
        debug_assert_eq!(utc, Utc.ymd(2001, 9, 9).and_hms(1, 46, 40));
        let fixed = FixedOffset::east(rug_fuzz_1).timestamp_nanos(rug_fuzz_2);
        debug_assert_eq!(
            fixed, FixedOffset::east(3600).ymd(2001, 9, 9).and_hms(2, 46, 40)
        );
        let _rug_ed_tests_llm_16_497_rrrruuuugggg_test_timestamp_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_498 {
    use crate::{DateTime, LocalResult, TimeZone, Utc};
    #[test]
    fn timestamp_opt_valid() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_valid = 0;
        let rug_fuzz_0 = "2015-05-15 00:00:00 UTC";
        let rug_fuzz_1 = 1431648000;
        let rug_fuzz_2 = 0;
        let expected_date = rug_fuzz_0;
        let timestamp = Utc.timestamp_opt(rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(timestamp.unwrap().to_string(), expected_date);
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_valid = 0;
    }
    #[test]
    fn timestamp_opt_out_of_range_seconds() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_out_of_range_seconds = 0;
        let rug_fuzz_0 = 0;
        let seconds = i64::MAX;
        let timestamp = Utc.timestamp_opt(seconds, rug_fuzz_0);
        debug_assert!(matches!(timestamp, LocalResult::None));
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_out_of_range_seconds = 0;
    }
    #[test]
    fn timestamp_opt_out_of_range_nanoseconds() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_out_of_range_nanoseconds = 0;
        let rug_fuzz_0 = 1_000_000_000;
        let rug_fuzz_1 = 1431648000;
        let nanoseconds = rug_fuzz_0;
        let timestamp = Utc.timestamp_opt(rug_fuzz_1, nanoseconds);
        debug_assert!(matches!(timestamp, LocalResult::None));
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_out_of_range_nanoseconds = 0;
    }
    #[test]
    fn timestamp_opt_edge_case_seconds() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_edge_case_seconds = 0;
        let rug_fuzz_0 = 0;
        let seconds = i64::MIN;
        let timestamp = Utc.timestamp_opt(seconds, rug_fuzz_0);
        debug_assert!(matches!(timestamp, LocalResult::None));
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_edge_case_seconds = 0;
    }
    #[test]
    fn timestamp_opt_edge_case_nanoseconds() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_edge_case_nanoseconds = 0;
        let rug_fuzz_0 = 999_999_999;
        let rug_fuzz_1 = 1431648000;
        let nanoseconds = rug_fuzz_0;
        let timestamp = Utc.timestamp_opt(rug_fuzz_1, nanoseconds);
        debug_assert!(matches!(timestamp, LocalResult::Single(_)));
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_edge_case_nanoseconds = 0;
    }
    #[test]
    fn timestamp_opt_ambiguous() {
        let _rug_st_tests_llm_16_498_rrrruuuugggg_timestamp_opt_ambiguous = 0;
        let _rug_ed_tests_llm_16_498_rrrruuuugggg_timestamp_opt_ambiguous = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_499_llm_16_499 {
    use crate::{DateTime, Local, LocalResult, TimeZone, Datelike, Timelike};
    #[test]
    fn test_with_ymd_and_hms_valid_input() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_valid_input = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let tz = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let hour = rug_fuzz_3;
        let min = rug_fuzz_4;
        let sec = rug_fuzz_5;
        let datetime = tz.with_ymd_and_hms(year, month, day, hour, min, sec);
        match datetime {
            LocalResult::Single(dt) => {
                debug_assert_eq!(dt.year(), year);
                debug_assert_eq!(dt.month(), month);
                debug_assert_eq!(dt.day(), day);
                debug_assert_eq!(dt.hour(), hour);
                debug_assert_eq!(dt.minute(), min);
                debug_assert_eq!(dt.second(), sec);
            }
            _ => panic!("Expected a single, valid DateTime."),
        }
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_valid_input = 0;
    }
    #[test]
    fn test_with_ymd_and_hms_invalid_month() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_month = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 13;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let tz = Local;
        let datetime = tz
            .with_ymd_and_hms(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
            );
        debug_assert!(matches!(datetime, LocalResult::None));
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_month = 0;
    }
    #[test]
    fn test_with_ymd_and_hms_invalid_day() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_day = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let tz = Local;
        let datetime = tz
            .with_ymd_and_hms(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
            );
        debug_assert!(matches!(datetime, LocalResult::None));
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_day = 0;
    }
    #[test]
    fn test_with_ymd_and_hms_invalid_hour() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_hour = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 24;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let tz = Local;
        let datetime = tz
            .with_ymd_and_hms(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
            );
        debug_assert!(matches!(datetime, LocalResult::None));
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_hour = 0;
    }
    #[test]
    fn test_with_ymd_and_hms_invalid_minute() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_minute = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 60;
        let rug_fuzz_5 = 59;
        let tz = Local;
        let datetime = tz
            .with_ymd_and_hms(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
            );
        debug_assert!(matches!(datetime, LocalResult::None));
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_minute = 0;
    }
    #[test]
    fn test_with_ymd_and_hms_invalid_second() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 60;
        let tz = Local;
        let datetime = tz
            .with_ymd_and_hms(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
            );
        debug_assert!(matches!(datetime, LocalResult::None));
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_with_ymd_and_hms_invalid_second = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_501_llm_16_501 {
    use crate::offset::{Local, TimeZone, LocalResult};
    use crate::naive::NaiveDate;
    use crate::Date;
    use crate::offset::local::Local as LocalImpl;
    use crate::offset::Offset;
    #[test]
    fn test_ymd_opt_valid_date() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_valid_date = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 17;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        let naive_date = NaiveDate::from_ymd(year, month, day);
        debug_assert_eq!(
            date_opt, LocalResult::Single(Date::from_utc(naive_date, timezone
            .offset_from_utc_date(& naive_date)))
        );
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_valid_date = 0;
    }
    #[test]
    fn test_ymd_opt_invalid_month() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_month = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 13;
        let rug_fuzz_2 = 17;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        debug_assert_eq!(date_opt, LocalResult::None);
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_month = 0;
    }
    #[test]
    fn test_ymd_opt_invalid_day() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_day = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 30;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        debug_assert_eq!(date_opt, LocalResult::None);
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_day = 0;
    }
    #[test]
    fn test_ymd_opt_invalid_date() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_date = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 30;
        let timezone = Local;
        let year = -rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        debug_assert_eq!(date_opt, LocalResult::None);
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_invalid_date = 0;
    }
    #[test]
    fn test_ymd_opt_leap_year() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_leap_year = 0;
        let rug_fuzz_0 = 2024;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        let naive_date = NaiveDate::from_ymd(year, month, day);
        debug_assert_eq!(
            date_opt, LocalResult::Single(Date::from_utc(naive_date, timezone
            .offset_from_utc_date(& naive_date)))
        );
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_leap_year = 0;
    }
    #[test]
    fn test_ymd_opt_non_leap_year() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_non_leap_year = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        debug_assert_eq!(date_opt, LocalResult::None);
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_non_leap_year = 0;
    }
    #[test]
    fn test_ymd_opt_ambiguous() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_ambiguous = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let timezone = Local;
        let year = rug_fuzz_0;
        let month = rug_fuzz_1;
        let day = rug_fuzz_2;
        let date_opt = timezone.ymd_opt(year, month, day);
        debug_assert!(matches!(date_opt, LocalResult::Ambiguous(_, _)));
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_ymd_opt_ambiguous = 0;
    }
}
