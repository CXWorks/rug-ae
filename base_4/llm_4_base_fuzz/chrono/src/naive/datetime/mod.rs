//! ISO 8601 date and time without timezone.
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Write;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::{fmt, str};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::format::DelayedFormat;
use crate::format::{parse, ParseError, ParseResult, Parsed, StrftimeItems};
use crate::format::{Fixed, Item, Numeric, Pad};
use crate::naive::{Days, IsoWeek, NaiveDate, NaiveTime};
use crate::{
    DateTime, Datelike, LocalResult, Months, TimeDelta, TimeZone, Timelike, Weekday,
};
/// Tools to help serializing/deserializing `NaiveDateTime`s
#[cfg(feature = "serde")]
pub(crate) mod serde;
#[cfg(test)]
mod tests;
/// The tight upper bound guarantees that a duration with `|TimeDelta| >= 2^MAX_SECS_BITS`
/// will always overflow the addition with any date and time type.
///
/// So why is this needed? `TimeDelta::seconds(rhs)` may overflow, and we don't have
/// an alternative returning `Option` or `Result`. Thus we need some early bound to avoid
/// touching that call when we are already sure that it WILL overflow...
const MAX_SECS_BITS: usize = 44;
/// The minimum possible `NaiveDateTime`.
#[deprecated(since = "0.4.20", note = "Use NaiveDateTime::MIN instead")]
pub const MIN_DATETIME: NaiveDateTime = NaiveDateTime::MIN;
/// The maximum possible `NaiveDateTime`.
#[deprecated(since = "0.4.20", note = "Use NaiveDateTime::MAX instead")]
pub const MAX_DATETIME: NaiveDateTime = NaiveDateTime::MAX;
/// ISO 8601 combined date and time without timezone.
///
/// # Example
///
/// `NaiveDateTime` is commonly created from [`NaiveDate`](./struct.NaiveDate.html).
///
/// ```
/// use chrono::{NaiveDate, NaiveDateTime};
///
/// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
/// # let _ = dt;
/// ```
///
/// You can use typical [date-like](../trait.Datelike.html) and
/// [time-like](../trait.Timelike.html) methods,
/// provided that relevant traits are in the scope.
///
/// ```
/// # use chrono::{NaiveDate, NaiveDateTime};
/// # let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
/// use chrono::{Datelike, Timelike, Weekday};
///
/// assert_eq!(dt.weekday(), Weekday::Fri);
/// assert_eq!(dt.num_seconds_from_midnight(), 33011);
/// ```
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NaiveDateTime {
    date: NaiveDate,
    time: NaiveTime,
}
impl NaiveDateTime {
    /// Makes a new `NaiveDateTime` from date and time components.
    /// Equivalent to [`date.and_time(time)`](./struct.NaiveDate.html#method.and_time)
    /// and many other helper constructors on `NaiveDate`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    ///
    /// let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    /// let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    ///
    /// let dt = NaiveDateTime::new(d, t);
    /// assert_eq!(dt.date(), d);
    /// assert_eq!(dt.time(), t);
    /// ```
    #[inline]
    pub const fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime { date, time }
    }
    /// Makes a new `NaiveDateTime` corresponding to a UTC date and time,
    /// from the number of non-leap seconds
    /// since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// For a non-naive version of this function see
    /// [`TimeZone::timestamp`](../offset/trait.TimeZone.html#method.timestamp).
    ///
    /// The nanosecond part can exceed 1,000,000,000 in order to represent the
    /// [leap second](./struct.NaiveTime.html#leap-second-handling). (The true "UNIX
    /// timestamp" cannot represent a leap second unambiguously.)
    ///
    /// Panics on the out-of-range number of seconds and/or invalid nanosecond.
    #[deprecated(since = "0.4.23", note = "use `from_timestamp_opt()` instead")]
    #[inline]
    #[must_use]
    pub fn from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime {
        let datetime = NaiveDateTime::from_timestamp_opt(secs, nsecs);
        datetime.expect("invalid or out-of-range datetime")
    }
    /// Creates a new [NaiveDateTime] from milliseconds since the UNIX epoch.
    ///
    /// The UNIX epoch starts on midnight, January 1, 1970, UTC.
    ///
    /// Returns `None` on an out-of-range number of milliseconds.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDateTime;
    /// let timestamp_millis: i64 = 1662921288000; //Sunday, September 11, 2022 6:34:48 PM
    /// let naive_datetime = NaiveDateTime::from_timestamp_millis(timestamp_millis);
    /// assert!(naive_datetime.is_some());
    /// assert_eq!(timestamp_millis, naive_datetime.unwrap().timestamp_millis());
    ///
    /// // Negative timestamps (before the UNIX epoch) are supported as well.
    /// let timestamp_millis: i64 = -2208936075000; //Mon Jan 01 1900 14:38:45 GMT+0000
    /// let naive_datetime = NaiveDateTime::from_timestamp_millis(timestamp_millis);
    /// assert!(naive_datetime.is_some());
    /// assert_eq!(timestamp_millis, naive_datetime.unwrap().timestamp_millis());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_timestamp_millis(millis: i64) -> Option<NaiveDateTime> {
        let secs = millis.div_euclid(1000);
        let nsecs = millis.rem_euclid(1000) as u32 * 1_000_000;
        NaiveDateTime::from_timestamp_opt(secs, nsecs)
    }
    /// Creates a new [NaiveDateTime] from microseconds since the UNIX epoch.
    ///
    /// The UNIX epoch starts on midnight, January 1, 1970, UTC.
    ///
    /// Returns `None` on an out-of-range number of microseconds.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDateTime;
    /// let timestamp_micros: i64 = 1662921288000000; //Sunday, September 11, 2022 6:34:48 PM
    /// let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
    /// assert!(naive_datetime.is_some());
    /// assert_eq!(timestamp_micros, naive_datetime.unwrap().timestamp_micros());
    ///
    /// // Negative timestamps (before the UNIX epoch) are supported as well.
    /// let timestamp_micros: i64 = -2208936075000000; //Mon Jan 01 1900 14:38:45 GMT+0000
    /// let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
    /// assert!(naive_datetime.is_some());
    /// assert_eq!(timestamp_micros, naive_datetime.unwrap().timestamp_micros());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_timestamp_micros(micros: i64) -> Option<NaiveDateTime> {
        let secs = micros.div_euclid(1_000_000);
        let nsecs = micros.rem_euclid(1_000_000) as u32 * 1000;
        NaiveDateTime::from_timestamp_opt(secs, nsecs)
    }
    /// Makes a new `NaiveDateTime` corresponding to a UTC date and time,
    /// from the number of non-leap seconds
    /// since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// The nanosecond part can exceed 1,000,000,000
    /// in order to represent the [leap second](./struct.NaiveTime.html#leap-second-handling).
    /// (The true "UNIX timestamp" cannot represent a leap second unambiguously.)
    ///
    /// Returns `None` on the out-of-range number of seconds (more than 262 000 years away
    /// from common era) and/or invalid nanosecond (2 seconds or more).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDateTime, NaiveDate};
    /// use std::i64;
    ///
    /// let from_timestamp_opt = NaiveDateTime::from_timestamp_opt;
    ///
    /// assert!(from_timestamp_opt(0, 0).is_some());
    /// assert!(from_timestamp_opt(0, 999_999_999).is_some());
    /// assert!(from_timestamp_opt(0, 1_500_000_000).is_some()); // leap second
    /// assert!(from_timestamp_opt(0, 2_000_000_000).is_none());
    /// assert!(from_timestamp_opt(i64::MAX, 0).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<NaiveDateTime> {
        let days = secs.div_euclid(86_400);
        let secs = secs.rem_euclid(86_400);
        let date = i32::try_from(days)
            .ok()
            .and_then(|days| days.checked_add(719_163))
            .and_then(NaiveDate::from_num_days_from_ce_opt);
        let time = NaiveTime::from_num_seconds_from_midnight_opt(secs as u32, nsecs);
        match (date, time) {
            (Some(date), Some(time)) => Some(NaiveDateTime { date, time }),
            (_, _) => None,
        }
    }
    /// Parses a string with the specified format string and returns a new `NaiveDateTime`.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDateTime, NaiveDate};
    ///
    /// let parse_from_str = NaiveDateTime::parse_from_str;
    ///
    /// assert_eq!(parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S"),
    ///            Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap()));
    /// assert_eq!(parse_from_str("5sep2015pm012345.6789", "%d%b%Y%p%I%M%S%.f"),
    ///            Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_micro_opt(13, 23, 45, 678_900).unwrap()));
    /// ```
    ///
    /// Offset is ignored for the purpose of parsing.
    ///
    /// ```
    /// # use chrono::{NaiveDateTime, NaiveDate};
    /// # let parse_from_str = NaiveDateTime::parse_from_str;
    /// assert_eq!(parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),
    ///            Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// ```
    ///
    /// [Leap seconds](./struct.NaiveTime.html#leap-second-handling) are correctly handled by
    /// treating any time of the form `hh:mm:60` as a leap second.
    /// (This equally applies to the formatting, so the round trip is possible.)
    ///
    /// ```
    /// # use chrono::{NaiveDateTime, NaiveDate};
    /// # let parse_from_str = NaiveDateTime::parse_from_str;
    /// assert_eq!(parse_from_str("2015-07-01 08:59:60.123", "%Y-%m-%d %H:%M:%S%.f"),
    ///            Ok(NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_milli_opt(8, 59, 59, 1_123).unwrap()));
    /// ```
    ///
    /// Missing seconds are assumed to be zero,
    /// but out-of-bound times or insufficient fields are errors otherwise.
    ///
    /// ```
    /// # use chrono::{NaiveDateTime, NaiveDate};
    /// # let parse_from_str = NaiveDateTime::parse_from_str;
    /// assert_eq!(parse_from_str("94/9/4 7:15", "%y/%m/%d %H:%M"),
    ///            Ok(NaiveDate::from_ymd_opt(1994, 9, 4).unwrap().and_hms_opt(7, 15, 0).unwrap()));
    ///
    /// assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());
    /// assert!(parse_from_str("94/9/4 12", "%y/%m/%d %H").is_err());
    /// assert!(parse_from_str("94/9/4 17:60", "%y/%m/%d %H:%M").is_err());
    /// assert!(parse_from_str("94/9/4 24:00:00", "%y/%m/%d %H:%M:%S").is_err());
    /// ```
    ///
    /// All parsed fields should be consistent to each other, otherwise it's an error.
    ///
    /// ```
    /// # use chrono::NaiveDateTime;
    /// # let parse_from_str = NaiveDateTime::parse_from_str;
    /// let fmt = "%Y-%m-%d %H:%M:%S = UNIX timestamp %s";
    /// assert!(parse_from_str("2001-09-09 01:46:39 = UNIX timestamp 999999999", fmt).is_ok());
    /// assert!(parse_from_str("1970-01-01 00:00:00 = UNIX timestamp 1", fmt).is_err());
    /// ```
    ///
    /// Years before 1 BCE or after 9999 CE, require an initial sign
    ///
    ///```
    /// # use chrono::{NaiveDate, NaiveDateTime};
    /// # let parse_from_str = NaiveDateTime::parse_from_str;
    /// let fmt = "%Y-%m-%d %H:%M:%S";
    /// assert!(parse_from_str("10000-09-09 01:46:39", fmt).is_err());
    /// assert!(parse_from_str("+10000-09-09 01:46:39", fmt).is_ok());
    ///```
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDateTime> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_naive_datetime_with_offset(0)
    }
    /// Retrieves a date component.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
    /// assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2016, 7, 8).unwrap());
    /// ```
    #[inline]
    pub const fn date(&self) -> NaiveDate {
        self.date
    }
    /// Retrieves a time component.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveTime};
    ///
    /// let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
    /// assert_eq!(dt.time(), NaiveTime::from_hms_opt(9, 10, 11).unwrap());
    /// ```
    #[inline]
    pub const fn time(&self) -> NaiveTime {
        self.time
    }
    /// Returns the number of non-leap seconds since the midnight on January 1, 1970.
    ///
    /// Note that this does *not* account for the timezone!
    /// The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(0, 0, 1, 980).unwrap();
    /// assert_eq!(dt.timestamp(), 1);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_opt(1, 46, 40).unwrap();
    /// assert_eq!(dt.timestamp(), 1_000_000_000);
    ///
    /// let dt = NaiveDate::from_ymd_opt(1969, 12, 31).unwrap().and_hms_opt(23, 59, 59).unwrap();
    /// assert_eq!(dt.timestamp(), -1);
    ///
    /// let dt = NaiveDate::from_ymd_opt(-1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    /// assert_eq!(dt.timestamp(), -62198755200);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        const UNIX_EPOCH_DAY: i64 = 719_163;
        let gregorian_day = i64::from(self.date.num_days_from_ce());
        let seconds_from_midnight = i64::from(self.time.num_seconds_from_midnight());
        (gregorian_day - UNIX_EPOCH_DAY) * 86_400 + seconds_from_midnight
    }
    /// Returns the number of non-leap *milliseconds* since midnight on January 1, 1970.
    ///
    /// Note that this does *not* account for the timezone!
    /// The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.
    ///
    /// Note also that this does reduce the number of years that can be
    /// represented from ~584 Billion to ~584 Million. (If this is a problem,
    /// please file an issue to let me know what domain needs millisecond
    /// precision over billions of years, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(0, 0, 1, 444).unwrap();
    /// assert_eq!(dt.timestamp_millis(), 1_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_milli_opt(1, 46, 40, 555).unwrap();
    /// assert_eq!(dt.timestamp_millis(), 1_000_000_000_555);
    ///
    /// let dt = NaiveDate::from_ymd_opt(1969, 12, 31).unwrap().and_hms_milli_opt(23, 59, 59, 100).unwrap();
    /// assert_eq!(dt.timestamp_millis(), -900);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_millis(&self) -> i64 {
        let as_ms = self.timestamp() * 1000;
        as_ms + i64::from(self.timestamp_subsec_millis())
    }
    /// Returns the number of non-leap *microseconds* since midnight on January 1, 1970.
    ///
    /// Note that this does *not* account for the timezone!
    /// The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.
    ///
    /// Note also that this does reduce the number of years that can be
    /// represented from ~584 Billion to ~584 Thousand. (If this is a problem,
    /// please file an issue to let me know what domain needs microsecond
    /// precision over millennia, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_micro_opt(0, 0, 1, 444).unwrap();
    /// assert_eq!(dt.timestamp_micros(), 1_000_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_micro_opt(1, 46, 40, 555).unwrap();
    /// assert_eq!(dt.timestamp_micros(), 1_000_000_000_000_555);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_micros(&self) -> i64 {
        let as_us = self.timestamp() * 1_000_000;
        as_us + i64::from(self.timestamp_subsec_micros())
    }
    /// Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.
    ///
    /// Note that this does *not* account for the timezone!
    /// The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.
    ///
    /// # Panics
    ///
    /// Note also that this does reduce the number of years that can be
    /// represented from ~584 Billion to ~584 years. The dates that can be
    /// represented as nanoseconds are between 1677-09-21T00:12:44.0 and
    /// 2262-04-11T23:47:16.854775804.
    ///
    /// (If this is a problem, please file an issue to let me know what domain
    /// needs nanosecond precision over millennia, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime};
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_nano_opt(0, 0, 1, 444).unwrap();
    /// assert_eq!(dt.timestamp_nanos(), 1_000_000_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_nano_opt(1, 46, 40, 555).unwrap();
    ///
    /// const A_BILLION: i64 = 1_000_000_000;
    /// let nanos = dt.timestamp_nanos();
    /// assert_eq!(nanos, 1_000_000_000_000_000_555);
    /// assert_eq!(
    ///     dt,
    ///     NaiveDateTime::from_timestamp(nanos / A_BILLION, (nanos % A_BILLION) as u32)
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_nanos(&self) -> i64 {
        let as_ns = self.timestamp() * 1_000_000_000;
        as_ns + i64::from(self.timestamp_subsec_nanos())
    }
    /// Returns the number of milliseconds since the last whole non-leap second.
    ///
    /// The return value ranges from 0 to 999,
    /// or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 123_456_789).unwrap();
    /// assert_eq!(dt.timestamp_subsec_millis(), 123);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_nano_opt(8, 59, 59, 1_234_567_890).unwrap();
    /// assert_eq!(dt.timestamp_subsec_millis(), 1_234);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_millis(&self) -> u32 {
        self.timestamp_subsec_nanos() / 1_000_000
    }
    /// Returns the number of microseconds since the last whole non-leap second.
    ///
    /// The return value ranges from 0 to 999,999,
    /// or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 123_456_789).unwrap();
    /// assert_eq!(dt.timestamp_subsec_micros(), 123_456);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_nano_opt(8, 59, 59, 1_234_567_890).unwrap();
    /// assert_eq!(dt.timestamp_subsec_micros(), 1_234_567);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_micros(&self) -> u32 {
        self.timestamp_subsec_nanos() / 1_000
    }
    /// Returns the number of nanoseconds since the last whole non-leap second.
    ///
    /// The return value ranges from 0 to 999,999,999,
    /// or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999,999.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 123_456_789).unwrap();
    /// assert_eq!(dt.timestamp_subsec_nanos(), 123_456_789);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_nano_opt(8, 59, 59, 1_234_567_890).unwrap();
    /// assert_eq!(dt.timestamp_subsec_nanos(), 1_234_567_890);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_nanos(&self) -> u32 {
        self.time.nanosecond()
    }
    /// Adds given `TimeDelta` to the current date and time.
    ///
    /// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
    /// the addition assumes that **there is no leap second ever**,
    /// except when the `NaiveDateTime` itself represents a leap second
    /// in which case the assumption becomes that **there is exactly a single leap second ever**.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// let d = from_ymd(2016, 7, 8);
    /// let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::zero()),
    ///            Some(hms(3, 5, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::seconds(1)),
    ///            Some(hms(3, 5, 8)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::seconds(-1)),
    ///            Some(hms(3, 5, 6)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::seconds(3600 + 60)),
    ///            Some(hms(4, 6, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::seconds(86_400)),
    ///            Some(from_ymd(2016, 7, 9).and_hms_opt(3, 5, 7).unwrap()));
    ///
    /// let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();
    /// assert_eq!(hmsm(3, 5, 7, 980).checked_add_signed(TimeDelta::milliseconds(450)),
    ///            Some(hmsm(3, 5, 8, 430)));
    /// ```
    ///
    /// Overflow returns `None`.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveDate};
    /// # let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::days(1_000_000_000)), None);
    /// ```
    ///
    /// Leap seconds are handled,
    /// but the addition assumes that it is the only leap second happened.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveDate};
    /// # let from_ymd = NaiveDate::from_ymd;
    /// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();
    /// let leap = hmsm(3, 5, 59, 1_300);
    /// assert_eq!(leap.checked_add_signed(TimeDelta::zero()),
    ///            Some(hmsm(3, 5, 59, 1_300)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::milliseconds(-500)),
    ///            Some(hmsm(3, 5, 59, 800)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::milliseconds(500)),
    ///            Some(hmsm(3, 5, 59, 1_800)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::milliseconds(800)),
    ///            Some(hmsm(3, 6, 0, 100)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::seconds(10)),
    ///            Some(hmsm(3, 6, 9, 300)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::seconds(-10)),
    ///            Some(hmsm(3, 5, 50, 300)));
    /// assert_eq!(leap.checked_add_signed(TimeDelta::days(1)),
    ///            Some(from_ymd(2016, 7, 9).and_hms_milli_opt(3, 5, 59, 300).unwrap()));
    /// ```
    #[must_use]
    pub fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime> {
        let (time, rhs) = self.time.overflowing_add_signed(rhs);
        if rhs <= (-1 << MAX_SECS_BITS) || rhs >= (1 << MAX_SECS_BITS) {
            return None;
        }
        let date = self.date.checked_add_signed(TimeDelta::seconds(rhs))?;
        Some(NaiveDateTime { date, time })
    }
    /// Adds given `Months` to the current date and time.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// Overflow returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// use chrono::{Months, NaiveDate, NaiveDateTime};
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()
    ///         .checked_add_months(Months::new(1)),
    ///     Some(NaiveDate::from_ymd_opt(2014, 2, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())
    /// );
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()
    ///         .checked_add_months(Months::new(core::i32::MAX as u32 + 1)),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn checked_add_months(self, rhs: Months) -> Option<NaiveDateTime> {
        Some(Self {
            date: self.date.checked_add_months(rhs)?,
            time: self.time,
        })
    }
    /// Subtracts given `TimeDelta` from the current date and time.
    ///
    /// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
    /// the subtraction assumes that **there is no leap second ever**,
    /// except when the `NaiveDateTime` itself represents a leap second
    /// in which case the assumption becomes that **there is exactly a single leap second ever**.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// let d = from_ymd(2016, 7, 8);
    /// let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::zero()),
    ///            Some(hms(3, 5, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::seconds(1)),
    ///            Some(hms(3, 5, 6)));
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::seconds(-1)),
    ///            Some(hms(3, 5, 8)));
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::seconds(3600 + 60)),
    ///            Some(hms(2, 4, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::seconds(86_400)),
    ///            Some(from_ymd(2016, 7, 7).and_hms_opt(3, 5, 7).unwrap()));
    ///
    /// let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();
    /// assert_eq!(hmsm(3, 5, 7, 450).checked_sub_signed(TimeDelta::milliseconds(670)),
    ///            Some(hmsm(3, 5, 6, 780)));
    /// ```
    ///
    /// Overflow returns `None`.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveDate};
    /// # let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();
    /// assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::days(1_000_000_000)), None);
    /// ```
    ///
    /// Leap seconds are handled,
    /// but the subtraction assumes that it is the only leap second happened.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveDate};
    /// # let from_ymd = NaiveDate::from_ymd;
    /// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();
    /// let leap = hmsm(3, 5, 59, 1_300);
    /// assert_eq!(leap.checked_sub_signed(TimeDelta::zero()),
    ///            Some(hmsm(3, 5, 59, 1_300)));
    /// assert_eq!(leap.checked_sub_signed(TimeDelta::milliseconds(200)),
    ///            Some(hmsm(3, 5, 59, 1_100)));
    /// assert_eq!(leap.checked_sub_signed(TimeDelta::milliseconds(500)),
    ///            Some(hmsm(3, 5, 59, 800)));
    /// assert_eq!(leap.checked_sub_signed(TimeDelta::seconds(60)),
    ///            Some(hmsm(3, 5, 0, 300)));
    /// assert_eq!(leap.checked_sub_signed(TimeDelta::days(1)),
    ///            Some(from_ymd(2016, 7, 7).and_hms_milli_opt(3, 6, 0, 300).unwrap()));
    /// ```
    #[must_use]
    pub fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime> {
        let (time, rhs) = self.time.overflowing_sub_signed(rhs);
        if rhs <= (-1 << MAX_SECS_BITS) || rhs >= (1 << MAX_SECS_BITS) {
            return None;
        }
        let date = self.date.checked_sub_signed(TimeDelta::seconds(rhs))?;
        Some(NaiveDateTime { date, time })
    }
    /// Subtracts given `Months` from the current date and time.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// Overflow returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// use chrono::{Months, NaiveDate, NaiveDateTime};
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()
    ///         .checked_sub_months(Months::new(1)),
    ///     Some(NaiveDate::from_ymd_opt(2013, 12, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())
    /// );
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()
    ///         .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),
    ///     None
    /// );
    /// ```
    #[must_use]
    pub fn checked_sub_months(self, rhs: Months) -> Option<NaiveDateTime> {
        Some(Self {
            date: self.date.checked_sub_months(rhs)?,
            time: self.time,
        })
    }
    /// Add a duration in [`Days`] to the date part of the `NaiveDateTime`
    ///
    /// Returns `None` if the resulting date would be out of range.
    #[must_use]
    pub fn checked_add_days(self, days: Days) -> Option<Self> {
        Some(Self {
            date: self.date.checked_add_days(days)?,
            ..self
        })
    }
    /// Subtract a duration in [`Days`] from the date part of the `NaiveDateTime`
    ///
    /// Returns `None` if the resulting date would be out of range.
    #[must_use]
    pub fn checked_sub_days(self, days: Days) -> Option<Self> {
        Some(Self {
            date: self.date.checked_sub_days(days)?,
            ..self
        })
    }
    /// Subtracts another `NaiveDateTime` from the current date and time.
    /// This does not overflow or underflow at all.
    ///
    /// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
    /// the subtraction assumes that **there is no leap second ever**,
    /// except when any of the `NaiveDateTime`s themselves represents a leap second
    /// in which case the assumption becomes that
    /// **there are exactly one (or two) leap second(s) ever**.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDate};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// let d = from_ymd(2016, 7, 8);
    /// assert_eq!(d.and_hms_opt(3, 5, 7).unwrap().signed_duration_since(d.and_hms_opt(2, 4, 6).unwrap()),
    ///            TimeDelta::seconds(3600 + 60 + 1));
    ///
    /// // July 8 is 190th day in the year 2016
    /// let d0 = from_ymd(2016, 1, 1);
    /// assert_eq!(d.and_hms_milli_opt(0, 7, 6, 500).unwrap().signed_duration_since(d0.and_hms_opt(0, 0, 0).unwrap()),
    ///            TimeDelta::seconds(189 * 86_400 + 7 * 60 + 6) + TimeDelta::milliseconds(500));
    /// ```
    ///
    /// Leap seconds are handled, but the subtraction assumes that
    /// there were no other leap seconds happened.
    ///
    /// ```
    /// # use chrono::{TimeDelta, NaiveDate};
    /// # let from_ymd = NaiveDate::from_ymd;
    /// let leap = from_ymd(2015, 6, 30).and_hms_milli_opt(23, 59, 59, 1_500).unwrap();
    /// assert_eq!(leap.signed_duration_since(from_ymd(2015, 6, 30).and_hms_opt(23, 0, 0).unwrap()),
    ///            TimeDelta::seconds(3600) + TimeDelta::milliseconds(500));
    /// assert_eq!(from_ymd(2015, 7, 1).and_hms_opt(1, 0, 0).unwrap().signed_duration_since(leap),
    ///            TimeDelta::seconds(3600) - TimeDelta::milliseconds(500));
    /// ```
    #[must_use]
    pub fn signed_duration_since(self, rhs: NaiveDateTime) -> TimeDelta {
        self.date.signed_duration_since(rhs.date)
            + self.time.signed_duration_since(rhs.time)
    }
    /// Formats the combined date and time with the specified formatting items.
    /// Otherwise it is the same as the ordinary [`format`](#method.format) method.
    ///
    /// The `Iterator` of items should be `Clone`able,
    /// since the resulting `DelayedFormat` value may be formatted multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use chrono::format::strftime::StrftimeItems;
    ///
    /// let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    /// let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(dt.format_with_items(fmt.clone()).to_string(), "2015-09-05 23:56:04");
    /// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(),    "2015-09-05 23:56:04");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # use chrono::format::strftime::StrftimeItems;
    /// # let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S").clone();
    /// # let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(format!("{}", dt.format_with_items(fmt)), "2015-09-05 23:56:04");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        DelayedFormat::new(Some(self.date), Some(self.time), items)
    }
    /// Formats the combined date and time with the specified format string.
    /// See the [`format::strftime` module](../format/strftime/index.html)
    /// on the supported escape sequences.
    ///
    /// This returns a `DelayedFormat`,
    /// which gets converted to a string only when actual formatting happens.
    /// You may use the `to_string` method to get a `String`,
    /// or just feed it into `print!` and other formatting macros.
    /// (In this way it avoids the redundant memory allocation.)
    ///
    /// A wrong format string does *not* issue an error immediately.
    /// Rather, converting or formatting the `DelayedFormat` fails.
    /// You are recommended to immediately use `DelayedFormat` for this reason.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2015-09-05 23:56:04");
    /// assert_eq!(dt.format("around %l %p on %b %-d").to_string(), "around 11 PM on Sep 5");
    /// ```
    ///
    /// The resulting `DelayedFormat` can be formatted directly via the `Display` trait.
    ///
    /// ```
    /// # use chrono::NaiveDate;
    /// # let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
    /// assert_eq!(format!("{}", dt.format("%Y-%m-%d %H:%M:%S")), "2015-09-05 23:56:04");
    /// assert_eq!(format!("{}", dt.format("around %l %p on %b %-d")), "around 11 PM on Sep 5");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
    /// Converts the `NaiveDateTime` into the timezone-aware `DateTime<Tz>`
    /// with the provided timezone, if possible.
    ///
    /// This can fail in cases where the local time represented by the `NaiveDateTime`
    /// is not a valid local timestamp in the target timezone due to an offset transition
    /// for example if the target timezone had a change from +00:00 to +01:00
    /// occuring at 2015-09-05 22:59:59, then a local time of 2015-09-05 23:56:04
    /// could never occur. Similarly, if the offset transitioned in the opposite direction
    /// then there would be two local times of 2015-09-05 23:56:04, one at +00:00 and one
    /// at +01:00.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, Utc};
    /// let dt = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timezone(), Utc);
    #[must_use]
    pub fn and_local_timezone<Tz: TimeZone>(&self, tz: Tz) -> LocalResult<DateTime<Tz>> {
        tz.from_local_datetime(self)
    }
    /// The minimum possible `NaiveDateTime`.
    pub const MIN: Self = Self {
        date: NaiveDate::MIN,
        time: NaiveTime::MIN,
    };
    /// The maximum possible `NaiveDateTime`.
    pub const MAX: Self = Self {
        date: NaiveDate::MAX,
        time: NaiveTime::MAX,
    };
}
impl Datelike for NaiveDateTime {
    /// Returns the year number in the [calendar date](./struct.NaiveDate.html#calendar-date).
    ///
    /// See also the [`NaiveDate::year`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.year(), 2015);
    /// ```
    #[inline]
    fn year(&self) -> i32 {
        self.date.year()
    }
    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    ///
    /// See also the [`NaiveDate::month`](./struct.NaiveDate.html#method.month) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.month(), 9);
    /// ```
    #[inline]
    fn month(&self) -> u32 {
        self.date.month()
    }
    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    ///
    /// See also the [`NaiveDate::month0`](./struct.NaiveDate.html#method.month0) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.month0(), 8);
    /// ```
    #[inline]
    fn month0(&self) -> u32 {
        self.date.month0()
    }
    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    ///
    /// See also the [`NaiveDate::day`](./struct.NaiveDate.html#method.day) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.day(), 25);
    /// ```
    #[inline]
    fn day(&self) -> u32 {
        self.date.day()
    }
    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    ///
    /// See also the [`NaiveDate::day0`](./struct.NaiveDate.html#method.day0) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.day0(), 24);
    /// ```
    #[inline]
    fn day0(&self) -> u32 {
        self.date.day0()
    }
    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    ///
    /// See also the [`NaiveDate::ordinal`](./struct.NaiveDate.html#method.ordinal) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.ordinal(), 268);
    /// ```
    #[inline]
    fn ordinal(&self) -> u32 {
        self.date.ordinal()
    }
    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    ///
    /// See also the [`NaiveDate::ordinal0`](./struct.NaiveDate.html#method.ordinal0) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.ordinal0(), 267);
    /// ```
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.date.ordinal0()
    }
    /// Returns the day of week.
    ///
    /// See also the [`NaiveDate::weekday`](./struct.NaiveDate.html#method.weekday) method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike, Weekday};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.weekday(), Weekday::Fri);
    /// ```
    #[inline]
    fn weekday(&self) -> Weekday {
        self.date.weekday()
    }
    #[inline]
    fn iso_week(&self) -> IsoWeek {
        self.date.iso_week()
    }
    /// Makes a new `NaiveDateTime` with the year number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_year`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_year(2016), Some(NaiveDate::from_ymd_opt(2016, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_year(-308), Some(NaiveDate::from_ymd_opt(-308, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// ```
    #[inline]
    fn with_year(&self, year: i32) -> Option<NaiveDateTime> {
        self.date.with_year(year).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the month number (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_month`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_month(10), Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_month(13), None); // no month 13
    /// assert_eq!(dt.with_month(2), None); // no February 30
    /// ```
    #[inline]
    fn with_month(&self, month: u32) -> Option<NaiveDateTime> {
        self.date.with_month(month).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the month number (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_month0`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_month0(9), Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_month0(12), None); // no month 13
    /// assert_eq!(dt.with_month0(1), None); // no February 30
    /// ```
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<NaiveDateTime> {
        self.date.with_month0(month0).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the day of month (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_day`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_day(30), Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_day(31), None); // no September 31
    /// ```
    #[inline]
    fn with_day(&self, day: u32) -> Option<NaiveDateTime> {
        self.date.with_day(day).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the day of month (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_day0`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_day0(29), Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_day0(30), None); // no September 31
    /// ```
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<NaiveDateTime> {
        self.date.with_day0(day0).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the day of year (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_ordinal`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_ordinal(60),
    ///            Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_ordinal(366), None); // 2015 had only 365 days
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_ordinal(60),
    ///            Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_ordinal(366),
    ///            Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// ```
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime> {
        self.date.with_ordinal(ordinal).map(|d| NaiveDateTime { date: d, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the day of year (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveDate::with_ordinal0`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_ordinal0(365), None); // 2015 had only 365 days
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();
    /// assert_eq!(dt.with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// assert_eq!(dt.with_ordinal0(365),
    ///            Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap()));
    /// ```
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime> {
        self.date.with_ordinal0(ordinal0).map(|d| NaiveDateTime { date: d, ..*self })
    }
}
impl Timelike for NaiveDateTime {
    /// Returns the hour number from 0 to 23.
    ///
    /// See also the [`NaiveTime::hour`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.hour(), 12);
    /// ```
    #[inline]
    fn hour(&self) -> u32 {
        self.time.hour()
    }
    /// Returns the minute number from 0 to 59.
    ///
    /// See also the [`NaiveTime::minute`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.minute(), 34);
    /// ```
    #[inline]
    fn minute(&self) -> u32 {
        self.time.minute()
    }
    /// Returns the second number from 0 to 59.
    ///
    /// See also the [`NaiveTime::second`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.second(), 56);
    /// ```
    #[inline]
    fn second(&self) -> u32 {
        self.time.second()
    }
    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// See also the [`NaiveTime::nanosecond`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.nanosecond(), 789_000_000);
    /// ```
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.time.nanosecond()
    }
    /// Makes a new `NaiveDateTime` with the hour number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the [`NaiveTime::with_hour`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.with_hour(7),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(7, 34, 56, 789).unwrap()));
    /// assert_eq!(dt.with_hour(24), None);
    /// ```
    #[inline]
    fn with_hour(&self, hour: u32) -> Option<NaiveDateTime> {
        self.time.with_hour(hour).map(|t| NaiveDateTime { time: t, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the minute number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveTime::with_minute`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.with_minute(45),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 45, 56, 789).unwrap()));
    /// assert_eq!(dt.with_minute(60), None);
    /// ```
    #[inline]
    fn with_minute(&self, min: u32) -> Option<NaiveDateTime> {
        self.time.with_minute(min).map(|t| NaiveDateTime { time: t, ..*self })
    }
    /// Makes a new `NaiveDateTime` with the second number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid. As
    /// with the [`NaiveDateTime::second`] method, the input range is
    /// restricted to 0 through 59.
    ///
    /// See also the [`NaiveTime::with_second`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.with_second(17),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 17, 789).unwrap()));
    /// assert_eq!(dt.with_second(60), None);
    /// ```
    #[inline]
    fn with_second(&self, sec: u32) -> Option<NaiveDateTime> {
        self.time.with_second(sec).map(|t| NaiveDateTime { time: t, ..*self })
    }
    /// Makes a new `NaiveDateTime` with nanoseconds since the whole non-leap second changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    /// As with the [`NaiveDateTime::nanosecond`] method,
    /// the input range can exceed 1,000,000,000 for leap seconds.
    ///
    /// See also the [`NaiveTime::with_nanosecond`] method.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();
    /// assert_eq!(dt.with_nanosecond(333_333_333),
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_nano_opt(12, 34, 56, 333_333_333).unwrap()));
    /// assert_eq!(dt.with_nanosecond(1_333_333_333), // leap second
    ///            Some(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_nano_opt(12, 34, 56, 1_333_333_333).unwrap()));
    /// assert_eq!(dt.with_nanosecond(2_000_000_000), None);
    /// ```
    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime> {
        self.time.with_nanosecond(nano).map(|t| NaiveDateTime { time: t, ..*self })
    }
}
/// An addition of `TimeDelta` to `NaiveDateTime` yields another `NaiveDateTime`.
///
/// As a part of Chrono's [leap second handling], the addition assumes that **there is no leap
/// second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case
/// the assumption becomes that **there is exactly a single leap second ever**.
///
/// Panics on underflow or overflow. Use [`NaiveDateTime::checked_add_signed`]
/// to detect that.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// let d = from_ymd(2016, 7, 8);
/// let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();
/// assert_eq!(hms(3, 5, 7) + TimeDelta::zero(),             hms(3, 5, 7));
/// assert_eq!(hms(3, 5, 7) + TimeDelta::seconds(1),         hms(3, 5, 8));
/// assert_eq!(hms(3, 5, 7) + TimeDelta::seconds(-1),        hms(3, 5, 6));
/// assert_eq!(hms(3, 5, 7) + TimeDelta::seconds(3600 + 60), hms(4, 6, 7));
/// assert_eq!(hms(3, 5, 7) + TimeDelta::seconds(86_400),
///            from_ymd(2016, 7, 9).and_hms_opt(3, 5, 7).unwrap());
/// assert_eq!(hms(3, 5, 7) + TimeDelta::days(365),
///            from_ymd(2017, 7, 8).and_hms_opt(3, 5, 7).unwrap());
///
/// let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();
/// assert_eq!(hmsm(3, 5, 7, 980) + TimeDelta::milliseconds(450), hmsm(3, 5, 8, 430));
/// ```
///
/// Leap seconds are handled,
/// but the addition assumes that it is the only leap second happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveDate};
/// # let from_ymd = NaiveDate::from_ymd;
/// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();
/// let leap = hmsm(3, 5, 59, 1_300);
/// assert_eq!(leap + TimeDelta::zero(),             hmsm(3, 5, 59, 1_300));
/// assert_eq!(leap + TimeDelta::milliseconds(-500), hmsm(3, 5, 59, 800));
/// assert_eq!(leap + TimeDelta::milliseconds(500),  hmsm(3, 5, 59, 1_800));
/// assert_eq!(leap + TimeDelta::milliseconds(800),  hmsm(3, 6, 0, 100));
/// assert_eq!(leap + TimeDelta::seconds(10),        hmsm(3, 6, 9, 300));
/// assert_eq!(leap + TimeDelta::seconds(-10),       hmsm(3, 5, 50, 300));
/// assert_eq!(leap + TimeDelta::days(1),
///            from_ymd(2016, 7, 9).and_hms_milli_opt(3, 5, 59, 300).unwrap());
/// ```
///
/// [leap second handling]: crate::NaiveTime#leap-second-handling
impl Add<TimeDelta> for NaiveDateTime {
    type Output = NaiveDateTime;
    #[inline]
    fn add(self, rhs: TimeDelta) -> NaiveDateTime {
        self.checked_add_signed(rhs).expect("`NaiveDateTime + TimeDelta` overflowed")
    }
}
impl AddAssign<TimeDelta> for NaiveDateTime {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = self.add(rhs);
    }
}
impl Add<Months> for NaiveDateTime {
    type Output = NaiveDateTime;
    /// An addition of months to `NaiveDateTime` clamped to valid days in resulting month.
    ///
    /// # Panics
    ///
    /// Panics if the resulting date would be out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{TimeDelta, NaiveDateTime, Months, NaiveDate};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap() + Months::new(1),
    ///     NaiveDate::from_ymd_opt(2014, 2, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 2, 0).unwrap() + Months::new(11),
    ///     NaiveDate::from_ymd_opt(2014, 12, 1).unwrap().and_hms_opt(0, 2, 0).unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 0, 3).unwrap() + Months::new(12),
    ///     NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().and_hms_opt(0, 0, 3).unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 0, 4).unwrap() + Months::new(13),
    ///     NaiveDate::from_ymd_opt(2015, 2, 1).unwrap().and_hms_opt(0, 0, 4).unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2014, 1, 31).unwrap().and_hms_opt(0, 5, 0).unwrap() + Months::new(1),
    ///     NaiveDate::from_ymd_opt(2014, 2, 28).unwrap().and_hms_opt(0, 5, 0).unwrap()
    /// );
    /// assert_eq!(
    ///     NaiveDate::from_ymd_opt(2020, 1, 31).unwrap().and_hms_opt(6, 0, 0).unwrap() + Months::new(1),
    ///     NaiveDate::from_ymd_opt(2020, 2, 29).unwrap().and_hms_opt(6, 0, 0).unwrap()
    /// );
    /// ```
    fn add(self, rhs: Months) -> Self::Output {
        Self {
            date: self.date.checked_add_months(rhs).unwrap(),
            time: self.time,
        }
    }
}
/// A subtraction of `TimeDelta` from `NaiveDateTime` yields another `NaiveDateTime`.
/// It is the same as the addition with a negated `TimeDelta`.
///
/// As a part of Chrono's [leap second handling] the subtraction assumes that **there is no leap
/// second ever**, except when the `NaiveDateTime` itself represents a leap second in which case
/// the assumption becomes that **there is exactly a single leap second ever**.
///
/// Panics on underflow or overflow. Use [`NaiveDateTime::checked_sub_signed`]
/// to detect that.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// let d = from_ymd(2016, 7, 8);
/// let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();
/// assert_eq!(hms(3, 5, 7) - TimeDelta::zero(),             hms(3, 5, 7));
/// assert_eq!(hms(3, 5, 7) - TimeDelta::seconds(1),         hms(3, 5, 6));
/// assert_eq!(hms(3, 5, 7) - TimeDelta::seconds(-1),        hms(3, 5, 8));
/// assert_eq!(hms(3, 5, 7) - TimeDelta::seconds(3600 + 60), hms(2, 4, 7));
/// assert_eq!(hms(3, 5, 7) - TimeDelta::seconds(86_400),
///            from_ymd(2016, 7, 7).and_hms_opt(3, 5, 7).unwrap());
/// assert_eq!(hms(3, 5, 7) - TimeDelta::days(365),
///            from_ymd(2015, 7, 9).and_hms_opt(3, 5, 7).unwrap());
///
/// let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();
/// assert_eq!(hmsm(3, 5, 7, 450) - TimeDelta::milliseconds(670), hmsm(3, 5, 6, 780));
/// ```
///
/// Leap seconds are handled,
/// but the subtraction assumes that it is the only leap second happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveDate};
/// # let from_ymd = NaiveDate::from_ymd;
/// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();
/// let leap = hmsm(3, 5, 59, 1_300);
/// assert_eq!(leap - TimeDelta::zero(),            hmsm(3, 5, 59, 1_300));
/// assert_eq!(leap - TimeDelta::milliseconds(200), hmsm(3, 5, 59, 1_100));
/// assert_eq!(leap - TimeDelta::milliseconds(500), hmsm(3, 5, 59, 800));
/// assert_eq!(leap - TimeDelta::seconds(60),       hmsm(3, 5, 0, 300));
/// assert_eq!(leap - TimeDelta::days(1),
///            from_ymd(2016, 7, 7).and_hms_milli_opt(3, 6, 0, 300).unwrap());
/// ```
///
/// [leap second handling]: crate::NaiveTime#leap-second-handling
impl Sub<TimeDelta> for NaiveDateTime {
    type Output = NaiveDateTime;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> NaiveDateTime {
        self.checked_sub_signed(rhs).expect("`NaiveDateTime - TimeDelta` overflowed")
    }
}
impl SubAssign<TimeDelta> for NaiveDateTime {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = self.sub(rhs);
    }
}
/// A subtraction of Months from `NaiveDateTime` clamped to valid days in resulting month.
///
/// # Panics
///
/// Panics if the resulting date would be out of range.
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDateTime, Months, NaiveDate};
/// use std::str::FromStr;
///
/// assert_eq!(
///     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(01, 00, 00).unwrap() - Months::new(11),
///     NaiveDate::from_ymd_opt(2013, 02, 01).unwrap().and_hms_opt(01, 00, 00).unwrap()
/// );
/// assert_eq!(
///     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(00, 02, 00).unwrap() - Months::new(12),
///     NaiveDate::from_ymd_opt(2013, 01, 01).unwrap().and_hms_opt(00, 02, 00).unwrap()
/// );
/// assert_eq!(
///     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(00, 00, 03).unwrap() - Months::new(13),
///     NaiveDate::from_ymd_opt(2012, 12, 01).unwrap().and_hms_opt(00, 00, 03).unwrap()
/// );
/// ```
impl Sub<Months> for NaiveDateTime {
    type Output = NaiveDateTime;
    fn sub(self, rhs: Months) -> Self::Output {
        Self {
            date: self.date.checked_sub_months(rhs).unwrap(),
            time: self.time,
        }
    }
}
/// Subtracts another `NaiveDateTime` from the current date and time.
/// This does not overflow or underflow at all.
///
/// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
/// the subtraction assumes that **there is no leap second ever**,
/// except when any of the `NaiveDateTime`s themselves represents a leap second
/// in which case the assumption becomes that
/// **there are exactly one (or two) leap second(s) ever**.
///
/// The implementation is a wrapper around [`NaiveDateTime::signed_duration_since`].
///
/// # Example
///
/// ```
/// use chrono::{TimeDelta, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// let d = from_ymd(2016, 7, 8);
/// assert_eq!(d.and_hms_opt(3, 5, 7).unwrap() - d.and_hms_opt(2, 4, 6).unwrap(), TimeDelta::seconds(3600 + 60 + 1));
///
/// // July 8 is 190th day in the year 2016
/// let d0 = from_ymd(2016, 1, 1);
/// assert_eq!(d.and_hms_milli_opt(0, 7, 6, 500).unwrap() - d0.and_hms_opt(0, 0, 0).unwrap(),
///            TimeDelta::seconds(189 * 86_400 + 7 * 60 + 6) + TimeDelta::milliseconds(500));
/// ```
///
/// Leap seconds are handled, but the subtraction assumes that no other leap
/// seconds happened.
///
/// ```
/// # use chrono::{TimeDelta, NaiveDate};
/// # let from_ymd = NaiveDate::from_ymd;
/// let leap = from_ymd(2015, 6, 30).and_hms_milli_opt(23, 59, 59, 1_500).unwrap();
/// assert_eq!(leap - from_ymd(2015, 6, 30).and_hms_opt(23, 0, 0).unwrap(),
///            TimeDelta::seconds(3600) + TimeDelta::milliseconds(500));
/// assert_eq!(from_ymd(2015, 7, 1).and_hms_opt(1, 0, 0).unwrap() - leap,
///            TimeDelta::seconds(3600) - TimeDelta::milliseconds(500));
/// ```
impl Sub<NaiveDateTime> for NaiveDateTime {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: NaiveDateTime) -> TimeDelta {
        self.signed_duration_since(rhs)
    }
}
impl Add<Days> for NaiveDateTime {
    type Output = NaiveDateTime;
    fn add(self, days: Days) -> Self::Output {
        self.checked_add_days(days).unwrap()
    }
}
impl Sub<Days> for NaiveDateTime {
    type Output = NaiveDateTime;
    fn sub(self, days: Days) -> Self::Output {
        self.checked_sub_days(days).unwrap()
    }
}
/// The `Debug` output of the naive date and time `dt` is the same as
/// [`dt.format("%Y-%m-%dT%H:%M:%S%.f")`](crate::format::strftime).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// let dt = NaiveDate::from_ymd_opt(2016, 11, 15).unwrap().and_hms_opt(7, 39, 24).unwrap();
/// assert_eq!(format!("{:?}", dt), "2016-11-15T07:39:24");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveDate;
/// let dt = NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_500).unwrap();
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60.500");
/// ```
impl fmt::Debug for NaiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.date.fmt(f)?;
        f.write_char('T')?;
        self.time.fmt(f)
    }
}
/// The `Display` output of the naive date and time `dt` is the same as
/// [`dt.format("%Y-%m-%d %H:%M:%S%.f")`](crate::format::strftime).
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
///
/// let dt = NaiveDate::from_ymd_opt(2016, 11, 15).unwrap().and_hms_opt(7, 39, 24).unwrap();
/// assert_eq!(format!("{}", dt), "2016-11-15 07:39:24");
/// ```
///
/// Leap seconds may also be used.
///
/// ```
/// # use chrono::NaiveDate;
/// let dt = NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_500).unwrap();
/// assert_eq!(format!("{}", dt), "2015-06-30 23:59:60.500");
/// ```
impl fmt::Display for NaiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.date.fmt(f)?;
        f.write_char(' ')?;
        self.time.fmt(f)
    }
}
/// Parsing a `str` into a `NaiveDateTime` uses the same format,
/// [`%Y-%m-%dT%H:%M:%S%.f`](crate::format::strftime), as in `Debug`.
///
/// # Example
///
/// ```
/// use chrono::{NaiveDateTime, NaiveDate};
///
/// let dt = NaiveDate::from_ymd_opt(2015, 9, 18).unwrap().and_hms_opt(23, 56, 4).unwrap();
/// assert_eq!("2015-09-18T23:56:04".parse::<NaiveDateTime>(), Ok(dt));
///
/// let dt = NaiveDate::from_ymd_opt(12345, 6, 7).unwrap().and_hms_milli_opt(7, 59, 59, 1_500).unwrap(); // leap second
/// assert_eq!("+12345-6-7T7:59:60.5".parse::<NaiveDateTime>(), Ok(dt));
///
/// assert!("foo".parse::<NaiveDateTime>().is_err());
/// ```
impl str::FromStr for NaiveDateTime {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<NaiveDateTime> {
        const ITEMS: &[Item<'static>] = &[
            Item::Numeric(Numeric::Year, Pad::Zero),
            Item::Space(""),
            Item::Literal("-"),
            Item::Numeric(Numeric::Month, Pad::Zero),
            Item::Space(""),
            Item::Literal("-"),
            Item::Numeric(Numeric::Day, Pad::Zero),
            Item::Space(""),
            Item::Literal("T"),
            Item::Numeric(Numeric::Hour, Pad::Zero),
            Item::Space(""),
            Item::Literal(":"),
            Item::Numeric(Numeric::Minute, Pad::Zero),
            Item::Space(""),
            Item::Literal(":"),
            Item::Numeric(Numeric::Second, Pad::Zero),
            Item::Fixed(Fixed::Nanosecond),
            Item::Space(""),
        ];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.to_naive_datetime_with_offset(0)
    }
}
/// The default value for a NaiveDateTime is one with epoch 0
/// that is, 1st of January 1970 at 00:00:00.
///
/// # Example
///
/// ```rust
/// use chrono::NaiveDateTime;
///
/// let default_date = NaiveDateTime::default();
/// assert_eq!(default_date, NaiveDateTime::from_timestamp(0, 0));
/// ```
impl Default for NaiveDateTime {
    fn default() -> Self {
        NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
    }
}
#[cfg(all(test, feature = "serde"))]
fn test_encodable_json<F, E>(to_string: F)
where
    F: Fn(&NaiveDateTime) -> Result<String, E>,
    E: ::std::fmt::Debug,
{
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_milli_opt(9, 10,
        48, 90).unwrap()).ok(), Some(r#""2016-07-08T09:10:48.090""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(2014, 7, 24).unwrap().and_hms_opt(12, 34, 6)
        .unwrap()).ok(), Some(r#""2014-07-24T12:34:06""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(0, 1, 1).unwrap().and_hms_milli_opt(0, 0, 59,
        1_000).unwrap()).ok(), Some(r#""0000-01-01T00:00:60""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::from_ymd_opt(- 1, 12, 31).unwrap().and_hms_nano_opt(23,
        59, 59, 7).unwrap()).ok(), Some(r#""-0001-12-31T23:59:59.000000007""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::MIN.and_hms_opt(0, 0, 0).unwrap()).ok(),
        Some(r#""-262144-01-01T00:00:00""#.into())
    );
    assert_eq!(
        to_string(& NaiveDate::MAX.and_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap())
        .ok(), Some(r#""+262143-12-31T23:59:60.999999999""#.into())
    );
}
#[cfg(all(test, feature = "serde"))]
fn test_decodable_json<F, E>(from_str: F)
where
    F: Fn(&str) -> Result<NaiveDateTime, E>,
    E: ::std::fmt::Debug,
{
    assert_eq!(
        from_str(r#""2016-07-08T09:10:48.090""#).ok(), Some(NaiveDate::from_ymd_opt(2016,
        7, 8).unwrap().and_hms_milli_opt(9, 10, 48, 90).unwrap())
    );
    assert_eq!(
        from_str(r#""2016-7-8T9:10:48.09""#).ok(), Some(NaiveDate::from_ymd_opt(2016, 7,
        8).unwrap().and_hms_milli_opt(9, 10, 48, 90).unwrap())
    );
    assert_eq!(
        from_str(r#""2014-07-24T12:34:06""#).ok(), Some(NaiveDate::from_ymd_opt(2014, 7,
        24).unwrap().and_hms_opt(12, 34, 6).unwrap())
    );
    assert_eq!(
        from_str(r#""0000-01-01T00:00:60""#).ok(), Some(NaiveDate::from_ymd_opt(0, 1, 1)
        .unwrap().and_hms_milli_opt(0, 0, 59, 1_000).unwrap())
    );
    assert_eq!(
        from_str(r#""0-1-1T0:0:60""#).ok(), Some(NaiveDate::from_ymd_opt(0, 1, 1)
        .unwrap().and_hms_milli_opt(0, 0, 59, 1_000).unwrap())
    );
    assert_eq!(
        from_str(r#""-0001-12-31T23:59:59.000000007""#).ok(),
        Some(NaiveDate::from_ymd_opt(- 1, 12, 31).unwrap().and_hms_nano_opt(23, 59, 59,
        7).unwrap())
    );
    assert_eq!(
        from_str(r#""-262144-01-01T00:00:00""#).ok(), Some(NaiveDate::MIN.and_hms_opt(0,
        0, 0).unwrap())
    );
    assert_eq!(
        from_str(r#""+262143-12-31T23:59:60.999999999""#).ok(), Some(NaiveDate::MAX
        .and_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap())
    );
    assert_eq!(
        from_str(r#""+262143-12-31T23:59:60.9999999999997""#).ok(), Some(NaiveDate::MAX
        .and_hms_nano_opt(23, 59, 59, 1_999_999_999).unwrap())
    );
    assert!(from_str(r#""""#).is_err());
    assert!(from_str(r#""2016-07-08""#).is_err());
    assert!(from_str(r#""09:10:48.090""#).is_err());
    assert!(from_str(r#""20160708T091048.090""#).is_err());
    assert!(from_str(r#""2000-00-00T00:00:00""#).is_err());
    assert!(from_str(r#""2000-02-30T00:00:00""#).is_err());
    assert!(from_str(r#""2001-02-29T00:00:00""#).is_err());
    assert!(from_str(r#""2002-02-28T24:00:00""#).is_err());
    assert!(from_str(r#""2002-02-28T23:60:00""#).is_err());
    assert!(from_str(r#""2002-02-28T23:59:61""#).is_err());
    assert!(from_str(r#""2016-07-08T09:10:48,090""#).is_err());
    assert!(from_str(r#""2016-07-08 09:10:48.090""#).is_err());
    assert!(from_str(r#""2016-007-08T09:10:48.090""#).is_err());
    assert!(from_str(r#""yyyy-mm-ddThh:mm:ss.fffffffff""#).is_err());
    assert!(from_str(r#"20160708000000"#).is_err());
    assert!(from_str(r#"{}"#).is_err());
    assert!(from_str(r#"{"date":{"ymdf":20},"time":{"secs":0,"frac":0}}"#).is_err());
    assert!(from_str(r#"null"#).is_err());
}
#[cfg(test)]
mod tests_llm_16_119 {
    use super::*;
    use crate::*;
    use crate::{NaiveDateTime, NaiveDate, NaiveTime};
    #[test]
    fn test_naive_date_time_default() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let default = NaiveDateTime::default();
        let expected = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(default, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_120 {
    use crate::naive::datetime::NaiveDateTime;
    use crate::naive::time::NaiveTime;
    use crate::naive::date::NaiveDate;
    use crate::month::Months;
    use std::ops::Add;
    #[test]
    fn test_add_months_to_naive_datetime() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_datetime = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let added_months = Months::new(rug_fuzz_6);
        let result_datetime = initial_datetime.add(added_months);
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(), result_datetime
        );
             }
});    }
    #[test]
    fn test_add_months_to_naive_datetime_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_datetime = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let added_months = Months::new(rug_fuzz_6);
        let result_datetime = initial_datetime.add(added_months);
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(), result_datetime
        );
             }
});    }
    #[test]
    fn test_add_months_to_naive_datetime_overflow_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_datetime = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let added_months = Months::new(rug_fuzz_6);
        let result_datetime = initial_datetime.add(added_months);
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(), result_datetime
        );
             }
});    }
    #[test]
    fn test_add_months_to_naive_datetime_change_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_datetime = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let added_months = Months::new(rug_fuzz_6);
        let result_datetime = initial_datetime.add(added_months);
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(), result_datetime
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_121 {
    use super::*;
    use crate::*;
    use crate::naive::NaiveDate;
    #[test]
    fn test_add_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date_time_plus_5_days = NaiveDate::from_ymd(
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
            )
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date_time + Days::new(rug_fuzz_12), date_time_plus_5_days);
             }
});    }
    #[test]
    #[should_panic(expected = "out-of-range date")]
    fn test_add_days_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let _ = date_time + Days::new(rug_fuzz_6);
             }
});    }
    #[test]
    fn test_add_days_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_time + Days::new(rug_fuzz_6), date_time);
             }
});    }
    #[test]
    fn test_add_days_negative() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date_time_minus_5_days = NaiveDate::from_ymd(
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
            )
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(date_time + Days::new(u64::MAX), date_time_minus_5_days);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_122 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::datetime::NaiveDateTime;
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_add_positive_timedelta() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i64, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap(),
        );
        let delta = TimeDelta::seconds(rug_fuzz_6);
        let result = date_time.add(delta);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(),
        );
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_add_negative_timedelta() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i64, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap(),
        );
        let delta = TimeDelta::seconds(-rug_fuzz_6);
        let result = date_time.add(delta);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap(),
        );
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_add_timedelta_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap(),
        );
        let large_positive_delta = TimeDelta::seconds(i64::MAX);
        let large_negative_delta = TimeDelta::seconds(i64::MIN);
        debug_assert!(date_time.checked_add_signed(large_positive_delta).is_none());
        debug_assert!(date_time.checked_add_signed(large_negative_delta).is_none());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_123 {
    use crate::naive::datetime::NaiveDateTime;
    use crate::naive::NaiveDate;
    use crate::time_delta::TimeDelta;
    use std::ops::AddAssign;
    #[test]
    fn test_add_assign() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23, mut rug_fuzz_24, mut rug_fuzz_25)) = <(i32, u32, u32, u32, u32, u32, i64, i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i64, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut dt1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let td = TimeDelta::milliseconds(rug_fuzz_6);
        dt1.add_assign(td);
        let dt2 = NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(
            dt1, dt2, "NaiveDateTime::add_assign failed to add 10 seconds correctly"
        );
        let mut dt3 = NaiveDate::from_ymd(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .and_hms(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18);
        let td2 = TimeDelta::milliseconds(rug_fuzz_19);
        dt3.add_assign(td2);
        let dt4 = NaiveDate::from_ymd(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .and_hms(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25);
        debug_assert_eq!(
            dt3, dt4, "NaiveDateTime::add_assign failed to wrap to next day correctly"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_124 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::time::NaiveTime;
    use crate::naive::datetime::NaiveDateTime;
    use crate::month::Months;
    #[test]
    fn test_sub_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let rhs = Months::new(rug_fuzz_6);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            NaiveTime::from_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12),
        );
        debug_assert_eq!(datetime.sub(rhs), expected);
             }
});    }
    #[test]
    fn test_sub_months_rollover() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let rhs = Months::new(rug_fuzz_6);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            NaiveTime::from_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12),
        );
        debug_assert_eq!(datetime.sub(rhs), expected);
             }
});    }
    #[test]
    fn test_sub_months_with_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let rhs = Months::new(rug_fuzz_6);
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            NaiveTime::from_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12),
        );
        debug_assert_eq!(datetime.sub(rhs), expected);
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range datetime")]
    fn test_sub_months_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let rhs = Months::new(rug_fuzz_6);
        let _result = datetime.sub(rhs);
             }
});    }
}
#[cfg(test)]
mod test_naive_date_sub {
    use crate::naive::{NaiveDate, NaiveDateTime, NaiveTime};
    use crate::Datelike;
    use crate::{Days, Weekday};
    #[test]
    fn test_sub_days() {
        let initial_date = NaiveDate::from_ymd(2022, 4, 5);
        let days_to_sub = Days::new(5);
        let expected_date = NaiveDate::from_ymd(2022, 3, 31);
        assert_eq!(initial_date - days_to_sub, expected_date);
        let initial_date = NaiveDate::from_ymd(2022, 1, 1);
        let days_to_sub = Days::new(1);
        let expected_date = NaiveDate::from_ymd(2021, 12, 31);
        assert_eq!(initial_date - days_to_sub, expected_date);
        let initial_date = NaiveDate::from_ymd(2021, 3, 1);
        let days_to_sub = Days::new(1);
        let expected_date = NaiveDate::from_ymd(2021, 2, 28);
        assert_eq!(initial_date - days_to_sub, expected_date);
        let initial_date = NaiveDate::from_ymd(2024, 3, 1);
        let days_to_sub = Days::new(1);
        let expected_date = NaiveDate::from_ymd(2024, 2, 29);
        assert_eq!(initial_date - days_to_sub, expected_date);
    }
}
#[cfg(test)]
mod tests_llm_16_127 {
    use crate::{NaiveDate, NaiveDateTime};
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_sub() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date2 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let expected = TimeDelta::days(rug_fuzz_12);
        debug_assert_eq!(NaiveDateTime::signed_duration_since(date1, date2), expected);
             }
});    }
    #[test]
    fn test_sub_with_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time1 = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let date_time2 = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let expected = TimeDelta::days(rug_fuzz_12) + TimeDelta::hours(rug_fuzz_13)
            + TimeDelta::minutes(rug_fuzz_14) + TimeDelta::seconds(rug_fuzz_15);
        debug_assert_eq!(
            NaiveDateTime::signed_duration_since(date_time1, date_time2), expected
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_129 {
    use crate::naive::NaiveDateTime;
    use std::str::FromStr;
    #[test]
    fn test_naive_date_time_from_str_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = NaiveDateTime::new(
            crate::naive::NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            crate::naive::NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        let result = NaiveDateTime::from_str(input);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), expected);
             }
});    }
    #[test]
    fn test_naive_date_time_from_str_with_nanoseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = NaiveDateTime::new(
            crate::naive::NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            crate::naive::NaiveTime::from_hms_nano(
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
            ),
        );
        let result = NaiveDateTime::from_str(input);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), expected);
             }
});    }
    #[test]
    fn test_naive_date_time_from_str_invalid_format() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let result = NaiveDateTime::from_str(input);
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_naive_date_time_from_str_invalid_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let result = NaiveDateTime::from_str(input);
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_naive_date_time_from_str_invalid_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let result = NaiveDateTime::from_str(input);
        debug_assert!(result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_130 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, NaiveTime, Datelike, Timelike};
    #[test]
    fn test_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        debug_assert_eq!(dt.day(), 14);
             }
});    }
    #[test]
    fn test_day_last_day_of_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        debug_assert_eq!(dt.day(), 28);
             }
});    }
    #[test]
    fn test_day_first_day_of_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        debug_assert_eq!(dt.day(), 1);
             }
});    }
    #[test]
    fn test_day_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        debug_assert_eq!(dt.day(), 29);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_131 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_day0() {
        let _rug_st_tests_llm_16_131_rrrruuuugggg_test_day0 = 0;
        let rug_fuzz_0 = 2015;
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 25;
        let rug_fuzz_3 = 24;
        let rug_fuzz_4 = 2019;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2019;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 28;
        let rug_fuzz_11 = 27;
        let rug_fuzz_12 = 2020;
        let rug_fuzz_13 = 2;
        let rug_fuzz_14 = 29;
        let rug_fuzz_15 = 28;
        let rug_fuzz_16 = 2019;
        let rug_fuzz_17 = 12;
        let rug_fuzz_18 = 31;
        let rug_fuzz_19 = 30;
        let rug_fuzz_20 = 2020;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 2020;
        let rug_fuzz_25 = 12;
        let rug_fuzz_26 = 1;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 2020;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 2;
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 2020;
        let rug_fuzz_33 = 11;
        let rug_fuzz_34 = 30;
        let rug_fuzz_35 = 29;
        let rug_fuzz_36 = 2020;
        let rug_fuzz_37 = 6;
        let rug_fuzz_38 = 15;
        let rug_fuzz_39 = 14;
        let rug_fuzz_40 = 12;
        let rug_fuzz_41 = 34;
        let rug_fuzz_42 = 56;
        let times = &[
            (NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), rug_fuzz_3),
            (NaiveDate::from_ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6), rug_fuzz_7),
            (NaiveDate::from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10), rug_fuzz_11),
            (NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14), rug_fuzz_15),
            (NaiveDate::from_ymd(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18), rug_fuzz_19),
            (NaiveDate::from_ymd(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22), rug_fuzz_23),
            (NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26), rug_fuzz_27),
            (NaiveDate::from_ymd(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30), rug_fuzz_31),
            (NaiveDate::from_ymd(rug_fuzz_32, rug_fuzz_33, rug_fuzz_34), rug_fuzz_35),
            (NaiveDate::from_ymd(rug_fuzz_36, rug_fuzz_37, rug_fuzz_38), rug_fuzz_39),
        ];
        for &(date, expected) in times {
            let datetime = NaiveDateTime::new(
                date,
                NaiveTime::from_hms(rug_fuzz_40, rug_fuzz_41, rug_fuzz_42),
            );
            let actual = datetime.day0();
            debug_assert_eq!(actual, expected, "Failed at {}", date);
        }
        let _rug_ed_tests_llm_16_131_rrrruuuugggg_test_day0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_132 {
    use crate::naive::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_iso_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = date.iso_week();
        debug_assert_eq!(iso_week, date.iso_week());
             }
});    }
    #[test]
    fn test_iso_week_first_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = date.iso_week();
        debug_assert_eq!(iso_week.week(), 52);
        debug_assert_eq!(iso_week.year(), 2022);
             }
});    }
    #[test]
    fn test_iso_week_last_week() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = date.iso_week();
        debug_assert_eq!(iso_week.week(), 52);
        debug_assert_eq!(iso_week.year(), 2023);
             }
});    }
    #[test]
    fn test_iso_week_leap_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let iso_week = date.iso_week();
        debug_assert_eq!(iso_week.week(), 1);
        debug_assert_eq!(iso_week.year(), 2024);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_133 {
    use crate::naive::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.month(), 5);
             }
});    }
    #[test]
    #[should_panic]
    fn test_invalid_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _ = date.month();
             }
});    }
    #[test]
    fn test_month_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u32, i32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        for month in rug_fuzz_0..=rug_fuzz_1 {
            let date = NaiveDate::from_ymd(rug_fuzz_2, month, rug_fuzz_3);
            debug_assert_eq!(date.month(), month);
        }
             }
});    }
    #[test]
    fn test_month_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        date = date.with_month(rug_fuzz_3).unwrap();
        debug_assert_eq!(date.month(), 2);
        debug_assert_eq!(date.day(), 28);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_134 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, Datelike};
    /// Test `NaiveDateTime::month0` with typical values.
    #[test]
    fn test_month0_typical() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(dt.month0(), 0);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap()
            .and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        debug_assert_eq!(dt.month0(), 5);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .unwrap()
            .and_hms_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .unwrap();
        debug_assert_eq!(dt.month0(), 11);
             }
});    }
    /// Test `NaiveDateTime::month0` with edge case values.
    #[test]
    fn test_month0_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(dt.month0(), 11);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap()
            .and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        debug_assert_eq!(dt.month0(), 11);
             }
});    }
    /// Test `NaiveDateTime::month0` with leap seconds.
    #[test]
    fn test_month0_leap_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_milli_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(dt.month0(), 5);
             }
});    }
    /// Test `NaiveDateTime::month0` with invalid date values.
    #[test]
    #[should_panic(expected = "invalid date")]
    fn test_month0_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_135 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_ordinal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cases = vec![
            (rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3), (2016, 2, 29, 60), (2019,
            12, 31, 365), (2020, 12, 31, 366), (2021, 1, 1, 1), (2021, 12, 31, 365),
            (2022, 6, 1, 152)
        ];
        for (year, month, day, expected_ordinal) in test_cases {
            let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let naive_datetime = naive_date
                .and_hms_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
                .unwrap();
            debug_assert_eq!(
                naive_datetime.ordinal(), expected_ordinal,
                "NaiveDateTime::ordinal({}-{:02}-{:02}) did not match expected value",
                year, month, day
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_136 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveTime};
    #[test]
    fn test_ordinal0() {
        let _rug_st_tests_llm_16_136_rrrruuuugggg_test_ordinal0 = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2019;
        let rug_fuzz_7 = 12;
        let rug_fuzz_8 = 31;
        let rug_fuzz_9 = 23;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 2020;
        let rug_fuzz_13 = 12;
        let rug_fuzz_14 = 31;
        let rug_fuzz_15 = 23;
        let rug_fuzz_16 = 59;
        let rug_fuzz_17 = 59;
        let rug_fuzz_18 = 2020;
        let rug_fuzz_19 = 2;
        let rug_fuzz_20 = 29;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 0;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 2020;
        let rug_fuzz_25 = 9;
        let rug_fuzz_26 = 15;
        let rug_fuzz_27 = 12;
        let rug_fuzz_28 = 30;
        let rug_fuzz_29 = 30;
        let new_year = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let last_day_prev_year = NaiveDate::from_ymd_opt(
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        let last_day_curr_year = NaiveDate::from_ymd_opt(
                rug_fuzz_12,
                rug_fuzz_13,
                rug_fuzz_14,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .unwrap();
        let leap_day = NaiveDate::from_ymd_opt(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap()
            .and_hms_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap();
        let random_day = NaiveDate::from_ymd_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .unwrap()
            .and_hms_opt(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29)
            .unwrap();
        debug_assert_eq!(new_year.ordinal0(), 0);
        debug_assert_eq!(last_day_prev_year.ordinal0(), 364);
        debug_assert_eq!(last_day_curr_year.ordinal0(), 365);
        debug_assert_eq!(leap_day.ordinal0(), 59);
        debug_assert_eq!(random_day.ordinal0(), 258);
        let _rug_ed_tests_llm_16_136_rrrruuuugggg_test_ordinal0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_138 {
    use crate::NaiveDate;
    use crate::NaiveDateTime;
    use crate::traits::Datelike;
    #[test]
    fn test_with_day() {
        let _rug_st_tests_llm_16_138_rrrruuuugggg_test_with_day = 0;
        let rug_fuzz_0 = 2015;
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 2015;
        let rug_fuzz_7 = 9;
        let rug_fuzz_8 = 30;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 34;
        let rug_fuzz_11 = 56;
        let rug_fuzz_12 = 30;
        let rug_fuzz_13 = 2015;
        let rug_fuzz_14 = 9;
        let rug_fuzz_15 = 30;
        let rug_fuzz_16 = 12;
        let rug_fuzz_17 = 34;
        let rug_fuzz_18 = 56;
        let rug_fuzz_19 = 30;
        let rug_fuzz_20 = 2015;
        let rug_fuzz_21 = 9;
        let rug_fuzz_22 = 30;
        let rug_fuzz_23 = 12;
        let rug_fuzz_24 = 34;
        let rug_fuzz_25 = 56;
        let rug_fuzz_26 = 31;
        let original = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let expected = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap()
            .and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        let with_day = original.with_day(rug_fuzz_12);
        debug_assert!(with_day.is_some());
        debug_assert_eq!(with_day.unwrap(), expected);
        let original = NaiveDate::from_ymd_opt(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .unwrap()
            .and_hms_opt(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .unwrap();
        let end_of_month = original.with_day(rug_fuzz_19);
        debug_assert!(end_of_month.is_some());
        debug_assert_eq!(end_of_month.unwrap(), original);
        let original = NaiveDate::from_ymd_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .unwrap()
            .and_hms_opt(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25)
            .unwrap();
        let invalid_day = original.with_day(rug_fuzz_26);
        debug_assert!(invalid_day.is_none());
        let _rug_ed_tests_llm_16_138_rrrruuuugggg_test_with_day = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_139 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveDateTime, Datelike};
    #[test]
    fn test_with_day0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        for day in rug_fuzz_6..rug_fuzz_7 {
            let result_dt = base_dt.with_day0(day as u32);
            debug_assert_eq!(
                result_dt.map(| dt | dt.day0()), Some(day),
                "with_day0({}) should return day0 as {}", day, day
            );
            debug_assert!(
                result_dt.map(| dt | dt.time()).unwrap() == base_dt.time(),
                "with_day0 should not change the time component"
            );
        }
        debug_assert_eq!(
            base_dt.with_day0(rug_fuzz_8), None, "with_day0(31) should return None"
        );
        debug_assert_eq!(
            base_dt.with_day0(rug_fuzz_9), None, "with_day0(32) should return None"
        );
        debug_assert_eq!(
            base_dt.with_day0(rug_fuzz_10), None, "with_day0(45) should return None"
        );
        debug_assert_eq!(
            base_dt.with_day0(u32::max_value()), None,
            "with_day0(u32::max_value()) should return None"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_140 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_with_month() {
        let _rug_st_tests_llm_16_140_rrrruuuugggg_test_with_month = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 13;
        let rug_fuzz_9 = 2023;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 31;
        let rug_fuzz_12 = 10;
        let rug_fuzz_13 = 30;
        let rug_fuzz_14 = 45;
        let rug_fuzz_15 = 4;
        let rug_fuzz_16 = 2;
        let rug_fuzz_17 = 2024;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 31;
        let rug_fuzz_20 = 10;
        let rug_fuzz_21 = 30;
        let rug_fuzz_22 = 45;
        let rug_fuzz_23 = 2;
        let rug_fuzz_24 = 2023;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 31;
        let rug_fuzz_27 = 23;
        let rug_fuzz_28 = 59;
        let rug_fuzz_29 = 59;
        let rug_fuzz_30 = 1_500_000_000;
        let rug_fuzz_31 = 2;
        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(
            dt.with_month(rug_fuzz_6), Some(NaiveDate::from_ymd(2023, 5, 15).and_hms(10,
            30, 45))
        );
        debug_assert_eq!(dt.with_month(rug_fuzz_7), None);
        debug_assert_eq!(dt.with_month(rug_fuzz_8), None);
        let dt = NaiveDate::from_ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .and_hms(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        debug_assert_eq!(
            dt.with_month(rug_fuzz_15), Some(NaiveDate::from_ymd(2023, 4, 30).and_hms(10,
            30, 45))
        );
        debug_assert_eq!(
            dt.with_month(rug_fuzz_16), Some(NaiveDate::from_ymd(2023, 2, 28).and_hms(10,
            30, 45))
        );
        let dt = NaiveDate::from_ymd(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19)
            .and_hms(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22);
        debug_assert_eq!(
            dt.with_month(rug_fuzz_23), Some(NaiveDate::from_ymd(2024, 2, 29).and_hms(10,
            30, 45))
        );
        let dt = NaiveDate::from_ymd(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .and_hms_nano(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29, rug_fuzz_30);
        debug_assert_eq!(
            dt.with_month(rug_fuzz_31), Some(NaiveDate::from_ymd(2023, 2, 28)
            .and_hms_nano(23, 59, 59, 1_500_000_000))
        );
        let _rug_ed_tests_llm_16_140_rrrruuuugggg_test_with_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_141 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::NaiveDateTime;
    use crate::traits::Datelike;
    #[test]
    fn test_with_month0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let new_month0 = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8),
            NaiveTime::from_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
        );
        debug_assert_eq!(dt.with_month0(rug_fuzz_12), Some(new_month0));
        debug_assert_eq!(dt.with_month0(rug_fuzz_13), None);
        debug_assert_eq!(dt.with_month0(rug_fuzz_14), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_142 {
    use super::*;
    use crate::*;
    use crate::NaiveTime;
    use crate::NaiveDate;
    use crate::NaiveDateTime;
    #[test]
    fn test_with_ordinal() {
        let _rug_st_tests_llm_16_142_rrrruuuugggg_test_with_ordinal = 0;
        let rug_fuzz_0 = 2015;
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 60;
        let rug_fuzz_7 = 2015;
        let rug_fuzz_8 = 9;
        let rug_fuzz_9 = 8;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 34;
        let rug_fuzz_12 = 56;
        let rug_fuzz_13 = 366;
        let rug_fuzz_14 = 2016;
        let rug_fuzz_15 = 9;
        let rug_fuzz_16 = 8;
        let rug_fuzz_17 = 12;
        let rug_fuzz_18 = 34;
        let rug_fuzz_19 = 56;
        let rug_fuzz_20 = 60;
        let rug_fuzz_21 = 2016;
        let rug_fuzz_22 = 9;
        let rug_fuzz_23 = 8;
        let rug_fuzz_24 = 12;
        let rug_fuzz_25 = 34;
        let rug_fuzz_26 = 56;
        let rug_fuzz_27 = 366;
        let rug_fuzz_28 = 2016;
        let rug_fuzz_29 = 9;
        let rug_fuzz_30 = 8;
        let rug_fuzz_31 = 12;
        let rug_fuzz_32 = 34;
        let rug_fuzz_33 = 56;
        let rug_fuzz_34 = 367;
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5))
            .with_ordinal(rug_fuzz_6), Some(NaiveDateTime::new(NaiveDate::from_ymd(2015,
            3, 1), NaiveTime::from_hms(12, 34, 56)))
        );
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9),
            NaiveTime::from_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12))
            .with_ordinal(rug_fuzz_13), None
        );
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_14, rug_fuzz_15,
            rug_fuzz_16), NaiveTime::from_hms(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19))
            .with_ordinal(rug_fuzz_20), Some(NaiveDateTime::new(NaiveDate::from_ymd(2016,
            2, 29), NaiveTime::from_hms(12, 34, 56)))
        );
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22,
            rug_fuzz_23), NaiveTime::from_hms(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26))
            .with_ordinal(rug_fuzz_27), Some(NaiveDateTime::new(NaiveDate::from_ymd(2016,
            12, 31), NaiveTime::from_hms(12, 34, 56)))
        );
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_28, rug_fuzz_29,
            rug_fuzz_30), NaiveTime::from_hms(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33))
            .with_ordinal(rug_fuzz_34), None
        );
        let _rug_ed_tests_llm_16_142_rrrruuuugggg_test_with_ordinal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_143 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_with_ordinal0() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let new_dt = dt.with_ordinal0(rug_fuzz_6);
        debug_assert_eq!(
            new_dt, Some(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().and_hms_opt(12, 34,
            56).unwrap())
        );
        let new_dt = dt.with_ordinal0(rug_fuzz_7);
        debug_assert_eq!(
            new_dt, Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34,
            56).unwrap())
        );
        let new_dt = dt.with_ordinal0(rug_fuzz_8);
        debug_assert_eq!(
            new_dt, Some(NaiveDate::from_ymd_opt(2015, 12, 31).unwrap().and_hms_opt(12,
            34, 56).unwrap())
        );
        let new_dt = dt.with_ordinal0(rug_fuzz_9);
        debug_assert_eq!(new_dt, None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_144 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    use crate::NaiveTime;
    #[test]
    fn test_with_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, i32, i32, i32, i32, u32, u32, u32, u32, u32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            dt.with_year(rug_fuzz_6), Some(NaiveDate::from_ymd_opt(2016, 9, 8).unwrap()
            .and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(
            dt.with_year(- rug_fuzz_7), Some(NaiveDate::from_ymd_opt(- 308, 9, 8)
            .unwrap().and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(dt.with_year(rug_fuzz_8), None, "Year is out of range");
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).unwrap()
            .and_hms_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap()
            .with_year(rug_fuzz_15), None, "2015 is not a leap year"
        );
             }
});    }
    #[test]
    fn test_with_month() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            dt.with_month(rug_fuzz_6), Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap()
            .and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(dt.with_month(rug_fuzz_7), None, "Month is out of range");
        debug_assert_eq!(dt.with_month(rug_fuzz_8), None, "Month is out of range");
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).unwrap()
            .and_hms_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap()
            .with_month(rug_fuzz_15), None, "No February 31"
        );
             }
});    }
    #[test]
    fn test_with_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            dt.with_day(rug_fuzz_6), Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap()
            .and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(
            NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .and_hms_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12).unwrap()
            .with_day(rug_fuzz_13), Some(NaiveDate::from_ymd_opt(2015, 2, 29).unwrap()
            .and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(dt.with_day(rug_fuzz_14), None, "September has only 30 days");
        debug_assert_eq!(dt.with_day(rug_fuzz_15), None, "Day is out of range");
             }
});    }
    #[test]
    fn test_with_ordinal() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            dt.with_ordinal(rug_fuzz_6), Some(NaiveDate::from_ymd_opt(2015, 1, 1)
            .unwrap().and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(
            dt.with_ordinal(rug_fuzz_7), Some(NaiveDate::from_ymd_opt(2015, 12, 31)
            .unwrap().and_hms_opt(12, 34, 56).unwrap())
        );
        debug_assert_eq!(dt.with_ordinal(rug_fuzz_8), None, "2015 is not a leap year");
             }
});    }
    #[test]
    fn test_too_large() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(dt.with_year(rug_fuzz_6), None, "Year is out of range");
        debug_assert_eq!(dt.with_month(rug_fuzz_7), None, "Month is out of range");
        debug_assert_eq!(dt.with_day(rug_fuzz_8), None, "Day is out of range");
        debug_assert_eq!(
            dt.with_ordinal(rug_fuzz_9), None, "Day of year is out of range"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_145 {
    use crate::{NaiveDate, NaiveDateTime, NaiveTime, Datelike, Timelike};
    #[test]
    fn test_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let datetime = NaiveDateTime::new(date, time);
        debug_assert_eq!(datetime.year(), 2023);
             }
});    }
    #[test]
    fn test_year_with_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms_milli(
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
        );
        let datetime = NaiveDateTime::new(date, time);
        debug_assert_eq!(datetime.year(), 2023);
             }
});    }
    #[test]
    fn test_year_before_common_era() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(-rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let datetime = NaiveDateTime::new(date, time);
        debug_assert_eq!(datetime.year(), - 4);
             }
});    }
    #[test]
    fn test_year_on_dst_transition() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_before_transition = NaiveDate::from_ymd(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
        );
        let time_before_transition = NaiveTime::from_hms(
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
        );
        let datetime_before = NaiveDateTime::new(
            date_before_transition,
            time_before_transition,
        );
        let date_after_transition = NaiveDate::from_ymd(
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
        );
        let time_after_transition = NaiveTime::from_hms(
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
        );
        let datetime_after = NaiveDateTime::new(
            date_after_transition,
            time_after_transition,
        );
        debug_assert_eq!(datetime_before.year(), 2023);
        debug_assert_eq!(datetime_after.year(), 2023);
             }
});    }
    #[test]
    fn test_year_at_precisely_new_year() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let datetime = NaiveDateTime::new(date, time);
        debug_assert_eq!(datetime.year(), 2024);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_146 {
    use crate::{NaiveDate, Timelike};
    #[test]
    fn test_hour() {
        let _rug_st_tests_llm_16_146_rrrruuuugggg_test_hour = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 14;
        let rug_fuzz_9 = 23;
        let rug_fuzz_10 = 30;
        let rug_fuzz_11 = 45;
        let rug_fuzz_12 = 2023;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 14;
        let rug_fuzz_15 = 0;
        let rug_fuzz_16 = 30;
        let rug_fuzz_17 = 45;
        let rug_fuzz_18 = 2023;
        let rug_fuzz_19 = 3;
        let rug_fuzz_20 = 14;
        let rug_fuzz_21 = 1;
        let rug_fuzz_22 = 30;
        let rug_fuzz_23 = 45;
        let rug_fuzz_24 = 2023;
        let rug_fuzz_25 = 3;
        let rug_fuzz_26 = 14;
        let rug_fuzz_27 = 12;
        let rug_fuzz_28 = 30;
        let rug_fuzz_29 = 45;
        let rug_fuzz_30 = 2023;
        let rug_fuzz_31 = 3;
        let rug_fuzz_32 = 14;
        let rug_fuzz_33 = 13;
        let rug_fuzz_34 = 30;
        let rug_fuzz_35 = 45;
        let rug_fuzz_36 = 2023;
        let rug_fuzz_37 = 3;
        let rug_fuzz_38 = 14;
        let rug_fuzz_39 = 13;
        let rug_fuzz_40 = 30;
        let rug_fuzz_41 = 45;
        let rug_fuzz_42 = 500;
        let rug_fuzz_43 = 2023;
        let rug_fuzz_44 = 3;
        let rug_fuzz_45 = 14;
        let rug_fuzz_46 = 13;
        let rug_fuzz_47 = 30;
        let rug_fuzz_48 = 45;
        let rug_fuzz_49 = 500_000;
        let rug_fuzz_50 = 2023;
        let rug_fuzz_51 = 3;
        let rug_fuzz_52 = 14;
        let rug_fuzz_53 = 13;
        let rug_fuzz_54 = 30;
        let rug_fuzz_55 = 45;
        let rug_fuzz_56 = 500_000_000;
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(dt.hour(), 10);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap()
            .and_hms_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap();
        debug_assert_eq!(dt.hour(), 23);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .unwrap()
            .and_hms_opt(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .unwrap();
        debug_assert_eq!(dt.hour(), 0);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap()
            .and_hms_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap();
        debug_assert_eq!(dt.hour(), 1);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26)
            .unwrap()
            .and_hms_opt(rug_fuzz_27, rug_fuzz_28, rug_fuzz_29)
            .unwrap();
        debug_assert_eq!(dt.hour(), 12);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32)
            .unwrap()
            .and_hms_opt(rug_fuzz_33, rug_fuzz_34, rug_fuzz_35)
            .unwrap();
        debug_assert_eq!(dt.hour(), 13);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_36, rug_fuzz_37, rug_fuzz_38)
            .unwrap()
            .and_hms_milli_opt(rug_fuzz_39, rug_fuzz_40, rug_fuzz_41, rug_fuzz_42)
            .unwrap();
        debug_assert_eq!(dt.hour(), 13);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_43, rug_fuzz_44, rug_fuzz_45)
            .unwrap()
            .and_hms_micro_opt(rug_fuzz_46, rug_fuzz_47, rug_fuzz_48, rug_fuzz_49)
            .unwrap();
        debug_assert_eq!(dt.hour(), 13);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_50, rug_fuzz_51, rug_fuzz_52)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_53, rug_fuzz_54, rug_fuzz_55, rug_fuzz_56)
            .unwrap();
        debug_assert_eq!(dt.hour(), 13);
        let _rug_ed_tests_llm_16_146_rrrruuuugggg_test_hour = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_147 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Timelike};
    #[test]
    fn test_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(rug_fuzz_6, dt.minute());
             }
});    }
    #[test]
    fn test_minute_before_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(rug_fuzz_6, dt.minute());
             }
});    }
    #[test]
    fn test_minute_after_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(rug_fuzz_6, dt.minute());
             }
});    }
    #[test]
    fn test_minute_with_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(rug_fuzz_7, dt.minute());
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range datetime")]
    fn test_minute_invalid_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_148 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    use crate::NaiveDateTime;
    use crate::Timelike;
    #[test]
    fn test_nanosecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        debug_assert_eq!(dt.nanosecond(), 123_456_789);
             }
});    }
    #[test]
    fn test_nanosecond_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        debug_assert_eq!(dt.nanosecond(), 1_123_456_789);
             }
});    }
    #[test]
    fn test_nanosecond_next_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        debug_assert_eq!(dt.nanosecond(), 0);
             }
});    }
    #[test]
    fn test_nanosecond_end_of_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        debug_assert_eq!(dt.nanosecond(), 999_999_999);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_149 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    use crate::Timelike;
    #[test]
    fn test_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(< NaiveDateTime as Timelike > ::second(& time), 58);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_150 {
    use crate::{NaiveDate, NaiveDateTime, Timelike};
    #[test]
    fn test_with_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            dt.with_hour(rug_fuzz_6).unwrap(), NaiveDate::from_ymd_opt(2023, 3, 14)
            .unwrap().and_hms_opt(5, 30, 45).unwrap()
        );
        let dt_with_leap_second = NaiveDate::from_ymd_opt(
                rug_fuzz_7,
                rug_fuzz_8,
                rug_fuzz_9,
            )
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(
            dt_with_leap_second.with_hour(rug_fuzz_14).unwrap(), dt_with_leap_second
        );
        debug_assert_eq!(dt.with_hour(rug_fuzz_15), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_151 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_with_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let new_min1 = rug_fuzz_6;
        let new_dt1 = dt.with_minute(new_min1);
        debug_assert_eq!(
            new_dt1, Some(NaiveDate::from_ymd(2022, 4, 1).and_hms(18, new_min1, 22))
        );
        let new_min2 = rug_fuzz_7;
        let new_dt2 = dt.with_minute(new_min2);
        debug_assert_eq!(new_dt2, None);
        let dt_leap = NaiveDate::from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .with_nanosecond(rug_fuzz_14);
        let new_dt_leap = dt_leap.unwrap().with_minute(new_min1);
        debug_assert_eq!(new_dt_leap, None);
        let new_min4 = rug_fuzz_15;
        let new_dt4 = dt.with_minute(new_min4);
        debug_assert_eq!(new_dt4, Some(dt));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_with_nanosecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(
            base_dt.with_nanosecond(rug_fuzz_6), Some(NaiveDate::from_ymd(2015, 9, 8)
            .and_hms_nano(12, 34, 56, 333_333_333))
        );
        debug_assert_eq!(
            base_dt.with_nanosecond(rug_fuzz_7), Some(NaiveDate::from_ymd(2015, 9, 8)
            .and_hms_nano(12, 34, 56, 1_333_333_333))
        );
        debug_assert_eq!(base_dt.with_nanosecond(rug_fuzz_8), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_153 {
    use super::*;
    use crate::*;
    use crate::naive::time::NaiveTime;
    #[test]
    fn test_with_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_milli(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(
            NaiveTime::with_second(& time, rug_fuzz_4),
            Some(NaiveTime::from_hms_milli(12, 34, 45, 789))
        );
        debug_assert_eq!(NaiveTime::with_second(& time, rug_fuzz_5), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_405 {
    use super::*;
    use crate::*;
    use crate::offset::{LocalResult, TimeZone, Utc, FixedOffset};
    use crate::datetime::DateTime;
    use crate::naive::{NaiveDateTime, NaiveDate, NaiveTime};
    #[test]
    fn test_and_local_timezone() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let result = naive_datetime.and_local_timezone(Utc);
        let expected_datetime: DateTime<Utc> = Utc
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(result, LocalResult::Single(expected_datetime));
             }
});    }
    #[test]
    fn test_and_local_timezone_with_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let result = naive_datetime.and_local_timezone(Utc);
        debug_assert!(matches!(result, LocalResult::Single(_)));
        if let LocalResult::Single(dt) = result {
            debug_assert_eq!(dt.time().nanosecond(), 0);
        }
             }
});    }
    #[test]
    fn test_and_local_timezone_invalid_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let result = naive_datetime.and_local_timezone(Utc);
        debug_assert_eq!(result, LocalResult::None);
             }
});    }
    #[test]
    fn test_and_local_timezone_ambiguous() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u32, u32, u32, u32, u32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let naive_date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let naive_time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let timezone = FixedOffset::west_opt(rug_fuzz_6 * rug_fuzz_7).unwrap();
        let result = naive_datetime.and_local_timezone(timezone);
        debug_assert!(matches!(result, LocalResult::Single(_)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_406_llm_16_406 {
    use crate::naive::{NaiveDateTime, NaiveDate, NaiveTime, Days};
    #[test]
    fn test_checked_add_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(i32, u32, u32, u32, u32, u32, u64, u64, u64, i32, u32, u32, u32, u32, u32, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let base_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap(),
        );
        debug_assert_eq!(
            base_time.checked_add_days(Days::new(rug_fuzz_6)),
            Some(NaiveDateTime::new(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
        );
        debug_assert_eq!(
            base_time.checked_add_days(Days::new(rug_fuzz_7)),
            Some(NaiveDateTime::new(NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
        );
        debug_assert_eq!(
            base_time.checked_add_days(Days::new(rug_fuzz_8)), Some(base_time)
        );
        let max_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11).unwrap(),
            NaiveTime::from_hms_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14).unwrap(),
        );
        debug_assert_eq!(max_time.checked_add_days(Days::new(rug_fuzz_15)), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_407 {
    use crate::NaiveDateTime;
    use crate::naive::date::NaiveDate;
    use crate::naive::time::NaiveTime;
    use crate::Months;
    #[test]
    fn test_checked_add_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(
            date_time.checked_add_months(Months::new(rug_fuzz_6)),
            Some(NaiveDate::from_ymd_opt(2020, 2, 29).unwrap().and_hms_opt(12, 0, 0)
            .unwrap())
        );
        debug_assert_eq!(
            date_time.checked_add_months(Months::new(rug_fuzz_7)),
            Some(NaiveDate::from_ymd_opt(2021, 1, 31).unwrap().and_hms_opt(12, 0, 0)
            .unwrap())
        );
        debug_assert_eq!(
            date_time.checked_add_months(Months::new(i32::MAX as u32 + rug_fuzz_8)), None
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_408 {
    use super::*;
    use crate::*;
    use crate::naive::date::NaiveDate;
    use crate::naive::time::NaiveTime;
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_checked_add_signed() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19)) = <(i32, u32, u32, u32, u32, u32, i64, i64, i64, i64, i64, i32, u32, u32, u32, u32, u32, u32, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let from_ymd = NaiveDate::from_ymd;
        let d = from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        debug_assert_eq!(d.checked_add_signed(TimeDelta::zero()), Some(d));
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::seconds(rug_fuzz_6)), Some(d +
            TimeDelta::seconds(1))
        );
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::seconds(- rug_fuzz_7)), Some(d -
            TimeDelta::seconds(1))
        );
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::seconds(rug_fuzz_8 + rug_fuzz_9)), Some(d +
            TimeDelta::seconds(3600 + 60))
        );
        debug_assert_eq!(
            d.checked_add_signed(TimeDelta::seconds(rug_fuzz_10)), Some(d +
            TimeDelta::days(1))
        );
        let hmsm = |h, m, s, milli| {
            from_ymd(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
                .and_hms_milli_opt(h, m, s, milli)
                .unwrap()
        };
        let d_hmsm = hmsm(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(
            d_hmsm.checked_add_signed(TimeDelta::milliseconds(rug_fuzz_18)), Some(hmsm(3,
            5, 8, 430))
        );
        let d_overflow = d.checked_add_signed(TimeDelta::days(rug_fuzz_19));
        debug_assert_eq!(d_overflow, None);
             }
});    }
    #[test]
    fn test_checked_add_signed_leap_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(i32, u32, u32, u32, u32, u32, u32, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let from_ymd = NaiveDate::from_ymd;
        let hmsm = |h, m, s, milli| {
            from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
                .and_hms_milli_opt(h, m, s, milli)
                .unwrap()
        };
        let leap = hmsm(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::zero()), Some(hmsm(3, 5, 59, 1_300))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::milliseconds(- rug_fuzz_7)), Some(hmsm(3,
            5, 59, 800))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::milliseconds(rug_fuzz_8)), Some(hmsm(3, 5,
            59, 1_800))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::milliseconds(rug_fuzz_9)), Some(hmsm(3, 6,
            0, 100))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::seconds(rug_fuzz_10)), Some(hmsm(3, 6, 9,
            300))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::seconds(- rug_fuzz_11)), Some(hmsm(3, 5,
            50, 300))
        );
        debug_assert_eq!(
            leap.checked_add_signed(TimeDelta::days(rug_fuzz_12)), Some(from_ymd(2016, 7,
            9).and_hms_milli_opt(3, 5, 59, 300).unwrap())
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_409 {
    use crate::NaiveDateTime;
    use crate::naive::NaiveDate;
    use crate::naive::date::Days;
    use crate::naive::time::NaiveTime;
    #[test]
    fn test_checked_sub_days() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(i32, u32, u32, u32, u32, u32, u64, u64, u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let days = Days::new(rug_fuzz_6);
        debug_assert_eq!(
            dt.checked_sub_days(days), Some(NaiveDateTime::new(NaiveDate::from_ymd(2023,
            4, 5), NaiveTime::from_hms(12, 30, 45)))
        );
        let days = Days::new(rug_fuzz_7);
        debug_assert_eq!(
            dt.checked_sub_days(days), Some(NaiveDateTime::new(NaiveDate::from_ymd(2023,
            3, 31), NaiveTime::from_hms(12, 30, 45)))
        );
        let days = Days::new(rug_fuzz_8);
        debug_assert_eq!(
            dt.checked_sub_days(days), Some(NaiveDateTime::new(NaiveDate::from_ymd(2023,
            4, 10), NaiveTime::from_hms(12, 30, 45)))
        );
        let days = Days::new(u64::MAX);
        debug_assert_eq!(dt.checked_sub_days(days), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_410 {
    use crate::NaiveDateTime;
    use crate::naive::date::NaiveDate;
    use crate::naive::time::NaiveTime;
    use crate::Months;
    #[test]
    fn test_checked_sub_months() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        debug_assert_eq!(
            dt.checked_sub_months(Months::new(rug_fuzz_6)),
            Some(NaiveDateTime::new(NaiveDate::from_ymd(2019, 12, 31),
            NaiveTime::from_hms(1, 0, 0)))
        );
        debug_assert_eq!(
            dt.checked_sub_months(Months::new(rug_fuzz_7)),
            Some(NaiveDateTime::new(NaiveDate::from_ymd(2020, 2, 29),
            NaiveTime::from_hms(1, 0, 0)))
        );
        debug_assert_eq!(
            dt.checked_sub_months(Months::new(rug_fuzz_8)),
            Some(NaiveDateTime::new(NaiveDate::from_ymd(2019, 2, 28),
            NaiveTime::from_hms(1, 0, 0)))
        );
        debug_assert_eq!(dt.checked_sub_months(Months::new(rug_fuzz_9)), None);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_411 {
    use crate::NaiveDateTime;
    use crate::time_delta::TimeDelta;
    use crate::naive::date::NaiveDate;
    #[test]
    fn test_checked_sub_signed() {
        let _rug_st_tests_llm_16_411_rrrruuuugggg_test_checked_sub_signed = 0;
        let rug_fuzz_0 = 2016;
        let rug_fuzz_1 = 7;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 2016;
        let rug_fuzz_4 = 7;
        let rug_fuzz_5 = 8;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 7;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 5;
        let rug_fuzz_15 = 7;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 3;
        let rug_fuzz_18 = 5;
        let rug_fuzz_19 = 7;
        let rug_fuzz_20 = 3600;
        let rug_fuzz_21 = 60;
        let rug_fuzz_22 = 3;
        let rug_fuzz_23 = 5;
        let rug_fuzz_24 = 7;
        let rug_fuzz_25 = 86_400;
        let rug_fuzz_26 = 3;
        let rug_fuzz_27 = 5;
        let rug_fuzz_28 = 7;
        let rug_fuzz_29 = 450;
        let rug_fuzz_30 = 670;
        let rug_fuzz_31 = 3;
        let rug_fuzz_32 = 5;
        let rug_fuzz_33 = 7;
        let rug_fuzz_34 = 1_000_000_000;
        let rug_fuzz_35 = 3;
        let rug_fuzz_36 = 5;
        let rug_fuzz_37 = 59;
        let rug_fuzz_38 = 1_300;
        let rug_fuzz_39 = 200;
        let rug_fuzz_40 = 500;
        let rug_fuzz_41 = 60;
        let rug_fuzz_42 = 1;
        let from_ymd = NaiveDate::from_ymd;
        let hms = |h, m, s| {
            from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).and_hms_opt(h, m, s).unwrap()
        };
        let hmsm = |h, m, s, milli| {
            from_ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
                .and_hms_milli_opt(h, m, s, milli)
                .unwrap()
        };
        debug_assert_eq!(
            hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .checked_sub_signed(TimeDelta::zero()), Some(hms(3, 5, 7))
        );
        debug_assert_eq!(
            hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .checked_sub_signed(TimeDelta::seconds(rug_fuzz_12)), Some(hms(3, 5, 6))
        );
        debug_assert_eq!(
            hms(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .checked_sub_signed(TimeDelta::seconds(- rug_fuzz_16)), Some(hms(3, 5, 8))
        );
        debug_assert_eq!(
            hms(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19)
            .checked_sub_signed(TimeDelta::seconds(rug_fuzz_20 + rug_fuzz_21)),
            Some(hms(2, 4, 7))
        );
        debug_assert_eq!(
            hms(rug_fuzz_22, rug_fuzz_23, rug_fuzz_24)
            .checked_sub_signed(TimeDelta::seconds(rug_fuzz_25)), Some(from_ymd(2016, 7,
            7).and_hms_opt(3, 5, 7).unwrap())
        );
        debug_assert_eq!(
            hmsm(rug_fuzz_26, rug_fuzz_27, rug_fuzz_28, rug_fuzz_29)
            .checked_sub_signed(TimeDelta::milliseconds(rug_fuzz_30)), Some(hmsm(3, 5, 6,
            780))
        );
        debug_assert_eq!(
            hms(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33)
            .checked_sub_signed(TimeDelta::days(rug_fuzz_34)), None
        );
        let leap = hmsm(rug_fuzz_35, rug_fuzz_36, rug_fuzz_37, rug_fuzz_38);
        debug_assert_eq!(
            leap.checked_sub_signed(TimeDelta::zero()), Some(hmsm(3, 5, 59, 1_300))
        );
        debug_assert_eq!(
            leap.checked_sub_signed(TimeDelta::milliseconds(rug_fuzz_39)), Some(hmsm(3,
            5, 59, 1_100))
        );
        debug_assert_eq!(
            leap.checked_sub_signed(TimeDelta::milliseconds(rug_fuzz_40)), Some(hmsm(3,
            5, 59, 800))
        );
        debug_assert_eq!(
            leap.checked_sub_signed(TimeDelta::seconds(rug_fuzz_41)), Some(hmsm(3, 5, 0,
            300))
        );
        debug_assert_eq!(
            leap.checked_sub_signed(TimeDelta::days(rug_fuzz_42)), Some(from_ymd(2016, 7,
            7).and_hms_milli_opt(3, 6, 0, 300).unwrap())
        );
        let _rug_ed_tests_llm_16_411_rrrruuuugggg_test_checked_sub_signed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_412 {
    use crate::NaiveDateTime;
    use crate::naive::NaiveDate;
    #[test]
    fn test_date() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let datetime = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(datetime.date(), NaiveDate::from_ymd(2023, 4, 30));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_413 {
    use crate::NaiveTime;
    use crate::NaiveDateTime;
    use crate::NaiveDate;
    use crate::format::strftime::StrftimeItems;
    use crate::format::DelayedFormat;
    fn format<'a>(
        datetime: &NaiveDateTime,
        fmt: &'a str,
    ) -> DelayedFormat<StrftimeItems<'a>> {
        datetime.format(fmt)
    }
    #[test]
    fn test_naive_datetime_format() {
        let dt = NaiveDate::from_ymd(2021, 4, 11).and_hms(10, 30, 45);
        assert_eq!(format(& dt, "%Y-%m-%d %H:%M:%S").to_string(), "2021-04-11 10:30:45");
        assert_eq!(
            format(& dt, "%a %b %d %T %Y").to_string(), "Sun Apr 11 10:30:45 2021"
        );
        assert_eq!(format(& dt, "%Y-%m-%dT%H:%M:%S").to_string(), "2021-04-11T10:30:45");
        assert_eq!(format(& dt, "%H:%M").to_string(), "10:30");
        assert_eq!(format(& dt, "%I:%M %p").to_string(), "10:30 AM");
        assert_eq!(format(& dt, "%S.%f").to_string(), "45.000000000");
        assert_eq!(format(& dt, "%v").to_string(), "11-APR-2021");
    }
    #[test]
    fn test_naive_datetime_format_with_fractional_seconds() {
        let dt = NaiveDate::from_ymd(2021, 4, 11).and_hms_nano(10, 30, 45, 123_456_789);
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S.%f").to_string(),
            "2021-04-11 10:30:45.123456789"
        );
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S.%3f").to_string(), "2021-04-11 10:30:45.123"
        );
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S.%6f").to_string(),
            "2021-04-11 10:30:45.123456"
        );
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S.%9f").to_string(),
            "2021-04-11 10:30:45.123456789"
        );
    }
    #[test]
    fn test_naive_datetime_format_with_leap_second() {
        let dt = NaiveDate::from_ymd(2015, 7, 1).and_hms_milli(8, 59, 59, 1_000);
        assert_eq!(format(& dt, "%Y-%m-%d %H:%M:%S").to_string(), "2015-07-01 08:59:60");
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S.%f").to_string(),
            "2015-07-01 08:59:60.000000000"
        );
        assert_eq!(format(& dt, "%Y-%m-%dT%H:%M:%S").to_string(), "2015-07-01T08:59:60");
    }
    #[test]
    fn test_naive_datetime_format_with_offset() {
        let dt = NaiveDate::from_ymd(2014, 5, 17).and_hms_milli(12, 34, 56, 789);
        assert_eq!(
            format(& dt, "%Y-%m-%d %H:%M:%S%z").to_string(), "2014-05-17 12:34:56"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_414 {
    use super::*;
    use crate::*;
    use crate::format::strftime::StrftimeItems;
    use crate::format::Item;
    use crate::format::Numeric::*;
    use crate::format::Pad::Zero;
    #[test]
    fn test_format_with_items() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, u32, u32, u32, u32, u32, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        let datetime = date.and_time(time);
        let format_items = StrftimeItems::new(rug_fuzz_6);
        let formatted = datetime.format_with_items(format_items).to_string();
        debug_assert_eq!(formatted, "2023-04-15 12:30:45");
             }
});    }
    #[test]
    fn test_custom_format_with_items() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(i32, u32, u32, u32, u32, u32, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let time = NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        let datetime = date.and_time(time);
        let custom_format_items = [
            Item::Numeric(Year, Zero),
            Item::Literal(rug_fuzz_6),
            Item::Numeric(Month, Zero),
            Item::Literal(rug_fuzz_7),
            Item::Numeric(Day, Zero),
            Item::Literal(rug_fuzz_8),
            Item::Numeric(Hour, Zero),
            Item::Literal(rug_fuzz_9),
            Item::Numeric(Minute, Zero),
            Item::Literal(rug_fuzz_10),
            Item::Numeric(Second, Zero),
        ];
        let formatted = datetime
            .format_with_items(custom_format_items.iter().cloned())
            .to_string();
        debug_assert_eq!(formatted, "2023 04 15 -- 12:30:45");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_415 {
    use crate::NaiveDateTime;
    use crate::{Datelike, Timelike};
    #[test]
    fn test_from_timestamp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt1 = NaiveDateTime::from_timestamp(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(dt1.timestamp(), 1_000_000_000);
        debug_assert_eq!(dt1.date().year(), 2001);
        debug_assert_eq!(dt1.date().month(), 9);
        debug_assert_eq!(dt1.date().day(), 9);
        debug_assert_eq!(dt1.time().hour(), 1);
        debug_assert_eq!(dt1.time().minute(), 46);
        debug_assert_eq!(dt1.time().second(), 40);
        let dt2 = NaiveDateTime::from_timestamp(-rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(dt2.timestamp(), - 1_000_000_000);
        debug_assert_eq!(dt2.date().year(), 1938);
        debug_assert_eq!(dt2.date().month(), 4);
        debug_assert_eq!(dt2.date().day(), 24);
        debug_assert_eq!(dt2.time().hour(), 22);
        debug_assert_eq!(dt2.time().minute(), 13);
        debug_assert_eq!(dt2.time().second(), 20);
        let dt3 = NaiveDateTime::from_timestamp(rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(dt3.timestamp_subsec_nanos(), 1_000_000_000);
        debug_assert_eq!(dt3.timestamp(), 0);
        debug_assert_eq!(dt3.date().year(), 1970);
        debug_assert_eq!(dt3.date().month(), 1);
        debug_assert_eq!(dt3.date().day(), 1);
        debug_assert_eq!(dt3.time().hour(), 0);
        debug_assert_eq!(dt3.time().minute(), 0);
        debug_assert_eq!(dt3.time().second(), 0);
        let dt4 = NaiveDateTime::from_timestamp(-rug_fuzz_6, rug_fuzz_7);
        debug_assert_eq!(dt4.timestamp_subsec_nanos(), 1_000_000_000);
        debug_assert_eq!(dt4.timestamp(), - 1);
        debug_assert_eq!(dt4.date().year(), 1969);
        debug_assert_eq!(dt4.date().month(), 12);
        debug_assert_eq!(dt4.date().day(), 31);
        debug_assert_eq!(dt4.time().hour(), 23);
        debug_assert_eq!(dt4.time().minute(), 59);
        debug_assert_eq!(dt4.time().second(), 59);
        let dt5 = NaiveDateTime::from_timestamp(-rug_fuzz_8, rug_fuzz_9);
        debug_assert_eq!(dt5.timestamp(), - 62_167_219_200);
        debug_assert_eq!(dt5.date().year(), 0);
        debug_assert_eq!(dt5.time().hour(), 0);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_416 {
    use super::*;
    use crate::*;
    use crate::naive::{NaiveDate, NaiveTime};
    #[test]
    fn test_from_timestamp_micros_with_valid_microseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timestamp_micros: i64 = rug_fuzz_0;
        let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
        debug_assert!(naive_datetime.is_some());
        debug_assert_eq!(timestamp_micros, naive_datetime.unwrap().timestamp_micros());
             }
});    }
    #[test]
    fn test_from_timestamp_micros_with_valid_negative_microseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timestamp_micros: i64 = -rug_fuzz_0;
        let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
        debug_assert!(naive_datetime.is_some());
        debug_assert_eq!(timestamp_micros, naive_datetime.unwrap().timestamp_micros());
             }
});    }
    #[test]
    fn test_from_timestamp_micros_with_out_of_range_microseconds() {
        let _rug_st_tests_llm_16_416_rrrruuuugggg_test_from_timestamp_micros_with_out_of_range_microseconds = 0;
        let timestamp_micros: i64 = i64::MAX;
        let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
        debug_assert!(naive_datetime.is_none());
        let _rug_ed_tests_llm_16_416_rrrruuuugggg_test_from_timestamp_micros_with_out_of_range_microseconds = 0;
    }
    #[test]
    fn test_from_timestamp_micros_with_zero_microseconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timestamp_micros: i64 = rug_fuzz_0;
        let naive_datetime = NaiveDateTime::from_timestamp_micros(timestamp_micros);
        debug_assert!(naive_datetime.is_some());
        debug_assert_eq!(
            NaiveDateTime::new(NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms_micro(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)),
            naive_datetime.unwrap()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_417 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_timestamp_millis_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let timestamp_millis_positive: i64 = rug_fuzz_0;
        let naive_datetime_positive = NaiveDateTime::from_timestamp_millis(
            timestamp_millis_positive,
        );
        debug_assert!(naive_datetime_positive.is_some());
        debug_assert_eq!(
            timestamp_millis_positive, naive_datetime_positive.unwrap()
            .timestamp_millis()
        );
        let timestamp_millis_negative: i64 = -rug_fuzz_1;
        let naive_datetime_negative = NaiveDateTime::from_timestamp_millis(
            timestamp_millis_negative,
        );
        debug_assert!(naive_datetime_negative.is_some());
        debug_assert_eq!(
            timestamp_millis_negative, naive_datetime_negative.unwrap()
            .timestamp_millis()
        );
             }
});    }
    #[test]
    fn test_from_timestamp_millis_none() {
        let _rug_st_tests_llm_16_417_rrrruuuugggg_test_from_timestamp_millis_none = 0;
        let out_of_range_timestamp_millis: i64 = i64::MAX;
        let naive_datetime_out_of_range = NaiveDateTime::from_timestamp_millis(
            out_of_range_timestamp_millis,
        );
        debug_assert!(naive_datetime_out_of_range.is_none());
        let _rug_ed_tests_llm_16_417_rrrruuuugggg_test_from_timestamp_millis_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_418 {
    use crate::naive::datetime::NaiveDateTime;
    #[test]
    fn from_timestamp_opt_valid_timestamps() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(i64, u32, i64, u32, i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_2, rug_fuzz_3).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_4, rug_fuzz_5).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_6, rug_fuzz_7).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_8, rug_fuzz_9).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(- rug_fuzz_10, rug_fuzz_11).is_some()
        );
             }
});    }
    #[test]
    fn from_timestamp_opt_invalid_timestamps() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, u32, u32, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1).is_none()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(std::i64::MAX, rug_fuzz_2).is_none()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(std::i64::MAX, rug_fuzz_3).is_none()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(- rug_fuzz_4, rug_fuzz_5).is_none()
        );
             }
});    }
    #[test]
    fn from_timestamp_opt_edge_cases() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, u32, i64, u32, i64, u32, i64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(- rug_fuzz_2, rug_fuzz_3).is_some()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(rug_fuzz_4, rug_fuzz_5).is_none()
        );
        debug_assert!(
            NaiveDateTime::from_timestamp_opt(- rug_fuzz_6, rug_fuzz_7).is_none()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_419 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    use crate::NaiveTime;
    use crate::NaiveDateTime;
    #[test]
    fn test_naive_date_time_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let time = NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let datetime = NaiveDateTime::new(date, time);
        debug_assert_eq!(datetime, NaiveDate::from_ymd(2023, 4, 1).and_hms(12, 30, 45));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_420 {
    use crate::{NaiveDateTime, NaiveDate, ParseError};
    #[test]
    fn test_parse_from_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let parse_from_str = NaiveDateTime::parse_from_str;
        debug_assert_eq!(
            parse_from_str(rug_fuzz_0, rug_fuzz_1), Ok(NaiveDate::from_ymd(2015, 9, 5)
            .and_hms(23, 56, 4))
        );
        debug_assert_eq!(
            parse_from_str(rug_fuzz_2, rug_fuzz_3), Ok(NaiveDate::from_ymd(2015, 9, 5)
            .and_hms_micro(13, 23, 45, 678_900))
        );
        debug_assert_eq!(
            parse_from_str(rug_fuzz_4, rug_fuzz_5), Ok(NaiveDate::from_ymd(2014, 5, 17)
            .and_hms(12, 34, 56))
        );
        debug_assert_eq!(
            parse_from_str(rug_fuzz_6, rug_fuzz_7), Ok(NaiveDate::from_ymd(2015, 7, 1)
            .and_hms_milli(8, 59, 59, 1_123))
        );
        debug_assert_eq!(
            parse_from_str(rug_fuzz_8, rug_fuzz_9), Ok(NaiveDate::from_ymd(1994, 9, 4)
            .and_hms(7, 15, 0))
        );
        debug_assert!(parse_from_str(rug_fuzz_10, rug_fuzz_11).is_err());
        debug_assert!(parse_from_str(rug_fuzz_12, rug_fuzz_13).is_err());
        debug_assert!(parse_from_str(rug_fuzz_14, rug_fuzz_15).is_err());
        debug_assert!(parse_from_str(rug_fuzz_16, rug_fuzz_17).is_err());
        let fmt = rug_fuzz_18;
        debug_assert!(parse_from_str(rug_fuzz_19, fmt).is_ok());
        debug_assert!(parse_from_str(rug_fuzz_20, fmt).is_err());
        let fmt = rug_fuzz_21;
        debug_assert!(parse_from_str(rug_fuzz_22, fmt).is_err());
        debug_assert!(parse_from_str(rug_fuzz_23, fmt).is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_421 {
    use super::*;
    use crate::*;
    use crate::NaiveTime;
    #[test]
    fn test_signed_duration_since_for_leap_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let leap = NaiveTime::from_hms_milli_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        debug_assert_eq!(
            leap.signed_duration_since(NaiveTime::from_hms_opt(rug_fuzz_4, rug_fuzz_5,
            rug_fuzz_6).unwrap()), TimeDelta::seconds(3600) +
            TimeDelta::milliseconds(500)
        );
        debug_assert_eq!(
            NaiveTime::from_hms_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).unwrap()
            .signed_duration_since(leap), TimeDelta::seconds(3600) -
            TimeDelta::milliseconds(500)
        );
             }
});    }
    #[test]
    fn test_signed_duration_since_for_non_leap_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = NaiveTime::from_hms_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(
            time.signed_duration_since(NaiveTime::from_hms_opt(rug_fuzz_3, rug_fuzz_4,
            rug_fuzz_5).unwrap()), TimeDelta::seconds(3600 + 60 + 1)
        );
             }
});    }
    #[test]
    fn test_signed_duration_since_with_fractional_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(u32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time1 = NaiveTime::from_hms_micro_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let time2 = NaiveTime::from_hms_micro_opt(
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
            )
            .unwrap();
        debug_assert_eq!(
            time1.signed_duration_since(time2), TimeDelta::seconds(3600 + 60 + 1) +
            TimeDelta::microseconds(100)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_422 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn time_returns_correct_naive_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_time.time(), NaiveTime::from_hms(12, 34, 56));
             }
});    }
    #[test]
    fn time_works_for_leap_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, u32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert!(date_time.time().nanosecond() >= rug_fuzz_7);
             }
});    }
    #[test]
    fn time_works_for_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_time.time(), NaiveTime::from_hms(0, 0, 0));
             }
});    }
    #[test]
    fn time_works_for_end_of_day() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(date_time.time(), NaiveTime::from_hms(23, 59, 59));
             }
});    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range datetime")]
    fn time_panics_on_out_of_range() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_423 {
    use super::*;
    use crate::*;
    #[test]
    fn test_timestamp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(dt.timestamp(), 0);
        let dt = NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(dt.timestamp(), 1_000_000_000);
        let dt = NaiveDate::from_ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .and_hms(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert_eq!(dt.timestamp(), - 1);
        let dt = NaiveDate::from_ymd(-rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .and_hms(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23);
        debug_assert_eq!(dt.timestamp(), - 62198755200);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_424 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_timestamp_micros() {
        let _rug_st_tests_llm_16_424_rrrruuuugggg_test_timestamp_micros = 0;
        let rug_fuzz_0 = 1969;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 999999;
        let rug_fuzz_7 = 1970;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 1970;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 444;
        let rug_fuzz_21 = 2001;
        let rug_fuzz_22 = 9;
        let rug_fuzz_23 = 9;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 46;
        let rug_fuzz_26 = 40;
        let rug_fuzz_27 = 555;
        let rug_fuzz_28 = 262_143;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 31;
        let rug_fuzz_31 = 23;
        let rug_fuzz_32 = 59;
        let rug_fuzz_33 = 59;
        let rug_fuzz_34 = 999_999;
        let rug_fuzz_35 = 262_143;
        let rug_fuzz_36 = 1;
        let rug_fuzz_37 = 1;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 0;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 1;
        let before_epoch = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_micro(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(before_epoch.timestamp_micros(), - 1);
        let epoch = NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms_micro(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(epoch.timestamp_micros(), 0);
        let after_epoch = NaiveDate::from_ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .and_hms_micro(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20);
        debug_assert_eq!(after_epoch.timestamp_micros(), 1_000_444);
        let far_after_epoch = NaiveDate::from_ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .and_hms_micro(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26, rug_fuzz_27);
        debug_assert_eq!(far_after_epoch.timestamp_micros(), 1_000_000_000_000_555);
        let near_max = NaiveDate::from_ymd(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30)
            .and_hms_micro(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33, rug_fuzz_34);
        debug_assert_eq!(near_max.timestamp_micros(), 9223372036854775807);
        let near_min = NaiveDate::from_ymd(-rug_fuzz_35, rug_fuzz_36, rug_fuzz_37)
            .and_hms_micro(rug_fuzz_38, rug_fuzz_39, rug_fuzz_40, rug_fuzz_41);
        debug_assert_eq!(near_min.timestamp_micros(), - 9223372036854775807);
        let _rug_ed_tests_llm_16_424_rrrruuuugggg_test_timestamp_micros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_425 {
    use crate::NaiveDate;
    use crate::Datelike;
    #[test]
    fn test_timestamp_millis() {
        let _rug_st_tests_llm_16_425_rrrruuuugggg_test_timestamp_millis = 0;
        let rug_fuzz_0 = 1970;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2000;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 946_684_800_000i64;
        let rug_fuzz_15 = 1960;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 315_619_200_000i64;
        let rug_fuzz_23 = 1970;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 1;
        let rug_fuzz_29 = 444;
        let rug_fuzz_30 = 1970;
        let rug_fuzz_31 = 1;
        let rug_fuzz_32 = 1;
        let rug_fuzz_33 = 23;
        let rug_fuzz_34 = 59;
        let rug_fuzz_35 = 59;
        let rug_fuzz_36 = 1_500;
        let rug_fuzz_37 = 1970;
        let rug_fuzz_38 = 1;
        let rug_fuzz_39 = 1;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 1_000;
        let rug_fuzz_44 = 23;
        let rug_fuzz_45 = 59;
        let rug_fuzz_46 = 59;
        let rug_fuzz_47 = 999;
        let rug_fuzz_48 = 0;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 0;
        let rug_fuzz_51 = 0;
        let rug_fuzz_52 = 0;
        let rug_fuzz_53 = 0;
        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_millis(), 0);
        let dt = NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms_milli(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        let expected = rug_fuzz_14;
        debug_assert_eq!(dt.timestamp_millis(), expected);
        let dt = NaiveDate::from_ymd(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17)
            .and_hms_milli(rug_fuzz_18, rug_fuzz_19, rug_fuzz_20, rug_fuzz_21);
        let expected = -rug_fuzz_22;
        debug_assert_eq!(dt.timestamp_millis(), expected);
        let dt = NaiveDate::from_ymd(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25)
            .and_hms_milli(rug_fuzz_26, rug_fuzz_27, rug_fuzz_28, rug_fuzz_29);
        debug_assert_eq!(dt.timestamp_millis(), 1_444);
        let dt = NaiveDate::from_ymd(rug_fuzz_30, rug_fuzz_31, rug_fuzz_32)
            .and_hms_milli(rug_fuzz_33, rug_fuzz_34, rug_fuzz_35, rug_fuzz_36);
        debug_assert_eq!(dt.timestamp_millis(), 86_399_500);
        let dt = NaiveDate::from_ymd(rug_fuzz_37, rug_fuzz_38, rug_fuzz_39)
            .and_hms_milli(rug_fuzz_40, rug_fuzz_41, rug_fuzz_42, rug_fuzz_43);
        debug_assert_eq!(dt.timestamp_millis(), 1_000);
        let dt = NaiveDate::MAX
            .and_hms_milli(rug_fuzz_44, rug_fuzz_45, rug_fuzz_46, rug_fuzz_47);
        debug_assert!(dt.timestamp_millis() > rug_fuzz_48);
        let dt = NaiveDate::MIN
            .and_hms_milli(rug_fuzz_49, rug_fuzz_50, rug_fuzz_51, rug_fuzz_52);
        debug_assert!(dt.timestamp_millis() < rug_fuzz_53);
        let _rug_ed_tests_llm_16_425_rrrruuuugggg_test_timestamp_millis = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_426 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, NaiveTime, NaiveDateTime};
    #[test]
    fn test_timestamp_nanos() {
        let _rug_st_tests_llm_16_426_rrrruuuugggg_test_timestamp_nanos = 0;
        let rug_fuzz_0 = 1970;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 444;
        let rug_fuzz_7 = 2001;
        let rug_fuzz_8 = 9;
        let rug_fuzz_9 = 9;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 46;
        let rug_fuzz_12 = 40;
        let rug_fuzz_13 = 555;
        let rug_fuzz_14 = 1970;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 1969;
        let rug_fuzz_22 = 12;
        let rug_fuzz_23 = 31;
        let rug_fuzz_24 = 23;
        let rug_fuzz_25 = 59;
        let rug_fuzz_26 = 59;
        let rug_fuzz_27 = 999_999_999;
        let rug_fuzz_28 = 2020;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 31;
        let rug_fuzz_31 = 23;
        let rug_fuzz_32 = 59;
        let rug_fuzz_33 = 60;
        let rug_fuzz_34 = 500_000_000;
        let rug_fuzz_35 = 1_600_000_000_000_000_000;
        let dt1 = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(dt1.timestamp_nanos(), 1_000_000_444);
        let dt2 = NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(dt2.timestamp_nanos(), 1_000_000_000_000_000_555);
        let dt3 = NaiveDate::from_ymd_opt(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap();
        debug_assert_eq!(dt3.timestamp_nanos(), 0);
        let dt4 = NaiveDate::from_ymd_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26, rug_fuzz_27)
            .unwrap();
        debug_assert_eq!(dt4.timestamp_nanos(), - 1_000_000_001);
        let dt5 = NaiveDate::from_ymd_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33, rug_fuzz_34)
            .unwrap();
        debug_assert!(dt5.timestamp_nanos() > rug_fuzz_35);
        let _rug_ed_tests_llm_16_426_rrrruuuugggg_test_timestamp_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_427 {
    use crate::NaiveDate;
    #[test]
    fn test_timestamp_subsec_micros() {
        let _rug_st_tests_llm_16_427_rrrruuuugggg_test_timestamp_subsec_micros = 0;
        let rug_fuzz_0 = 2017;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 123_456_789;
        let rug_fuzz_7 = 2017;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 23;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 999_999_999;
        let rug_fuzz_14 = 2017;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 23;
        let rug_fuzz_18 = 59;
        let rug_fuzz_19 = 59;
        let rug_fuzz_20 = 1_123_456_789;
        let rug_fuzz_21 = 2017;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 123_456;
        let rug_fuzz_28 = 2017;
        let rug_fuzz_29 = 1;
        let rug_fuzz_30 = 1;
        let rug_fuzz_31 = 12;
        let rug_fuzz_32 = 34;
        let rug_fuzz_33 = 56;
        let rug_fuzz_34 = 789;
        let dt1 = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(dt1.timestamp_subsec_micros(), 123_456);
        let dt2 = NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(dt2.timestamp_subsec_micros(), 999_999);
        let dt3 = NaiveDate::from_ymd_opt(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap();
        debug_assert_eq!(dt3.timestamp_subsec_micros(), 1_123_456);
        let dt4 = NaiveDate::from_ymd_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26, rug_fuzz_27)
            .unwrap();
        debug_assert_eq!(dt4.timestamp_subsec_micros(), 123);
        let dt5 = NaiveDate::from_ymd_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33, rug_fuzz_34)
            .unwrap();
        debug_assert_eq!(dt5.timestamp_subsec_micros(), 0);
        let _rug_ed_tests_llm_16_427_rrrruuuugggg_test_timestamp_subsec_micros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_428 {
    use super::*;
    use crate::*;
    use crate::NaiveDate;
    #[test]
    fn test_timestamp_subsec_millis() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20)) = <(i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32, i32, u32, u32, u32, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_millis(), 345);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_429 {
    use crate::NaiveDate;
    #[test]
    fn test_timestamp_subsec_nanos() {
        let _rug_st_tests_llm_16_429_rrrruuuugggg_test_timestamp_subsec_nanos = 0;
        let rug_fuzz_0 = 2016;
        let rug_fuzz_1 = 7;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 9;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 11;
        let rug_fuzz_6 = 123_456_789;
        let rug_fuzz_7 = 2015;
        let rug_fuzz_8 = 7;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 8;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 1_234_567_890;
        let rug_fuzz_14 = 2016;
        let rug_fuzz_15 = 7;
        let rug_fuzz_16 = 8;
        let rug_fuzz_17 = 9;
        let rug_fuzz_18 = 10;
        let rug_fuzz_19 = 11;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 2016;
        let rug_fuzz_22 = 7;
        let rug_fuzz_23 = 8;
        let rug_fuzz_24 = 9;
        let rug_fuzz_25 = 10;
        let rug_fuzz_26 = 11;
        let rug_fuzz_27 = 999_999_999;
        let rug_fuzz_28 = 2016;
        let rug_fuzz_29 = 7;
        let rug_fuzz_30 = 8;
        let rug_fuzz_31 = 9;
        let rug_fuzz_32 = 10;
        let rug_fuzz_33 = 11;
        let rug_fuzz_34 = 1_000_000_000;
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_nanos(), 123_456_789);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_nanos(), 1_234_567_890);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19, rug_fuzz_20)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_nanos(), 0);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26, rug_fuzz_27)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_nanos(), 999_999_999);
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33, rug_fuzz_34)
            .unwrap();
        debug_assert_eq!(dt.timestamp_subsec_nanos(), 1_000_000_000);
        let _rug_ed_tests_llm_16_429_rrrruuuugggg_test_timestamp_subsec_nanos = 0;
    }
}
