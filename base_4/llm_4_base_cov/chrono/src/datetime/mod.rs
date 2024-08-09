//! ISO 8601 date and time with time zone.
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::string::{String, ToString};
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::Write;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::{fmt, hash, str};
#[cfg(feature = "std")]
use std::string::ToString;
#[cfg(any(feature = "std", test))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::format::DelayedFormat;
#[cfg(feature = "unstable-locales")]
use crate::format::Locale;
use crate::format::{parse, ParseError, ParseResult, Parsed, StrftimeItems};
use crate::format::{Fixed, Item};
use crate::naive::{Days, IsoWeek, NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "clock")]
use crate::offset::Local;
use crate::offset::{FixedOffset, Offset, TimeZone, Utc};
#[allow(deprecated)]
use crate::Date;
use crate::{Datelike, Months, TimeDelta, Timelike, Weekday};
/// documented at re-export site
#[cfg(feature = "serde")]
pub(super) mod serde;
#[cfg(test)]
mod tests;
/// Specific formatting options for seconds. This may be extended in the
/// future, so exhaustive matching in external code is not recommended.
///
/// See the `TimeZone::to_rfc3339_opts` function for usage.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SecondsFormat {
    /// Format whole seconds only, with no decimal point nor subseconds.
    Secs,
    /// Use fixed 3 subsecond digits. This corresponds to
    /// [Fixed::Nanosecond3](format/enum.Fixed.html#variant.Nanosecond3).
    Millis,
    /// Use fixed 6 subsecond digits. This corresponds to
    /// [Fixed::Nanosecond6](format/enum.Fixed.html#variant.Nanosecond6).
    Micros,
    /// Use fixed 9 subsecond digits. This corresponds to
    /// [Fixed::Nanosecond9](format/enum.Fixed.html#variant.Nanosecond9).
    Nanos,
    /// Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to
    /// display all available non-zero sub-second digits.  This corresponds to
    /// [Fixed::Nanosecond](format/enum.Fixed.html#variant.Nanosecond).
    AutoSi,
}
/// ISO 8601 combined date and time with time zone.
///
/// There are some constructors implemented here (the `from_*` methods), but
/// the general-purpose constructors are all via the methods on the
/// [`TimeZone`](./offset/trait.TimeZone.html) implementations.
#[derive(Clone)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct DateTime<Tz: TimeZone> {
    datetime: NaiveDateTime,
    offset: Tz::Offset,
}
/// The minimum possible `DateTime<Utc>`.
#[deprecated(since = "0.4.20", note = "Use DateTime::MIN_UTC instead")]
pub const MIN_DATETIME: DateTime<Utc> = DateTime::<Utc>::MIN_UTC;
/// The maximum possible `DateTime<Utc>`.
#[deprecated(since = "0.4.20", note = "Use DateTime::MAX_UTC instead")]
pub const MAX_DATETIME: DateTime<Utc> = DateTime::<Utc>::MAX_UTC;
impl<Tz: TimeZone> DateTime<Tz> {
    /// Makes a new `DateTime` with given *UTC* datetime and offset.
    /// The local datetime should be constructed via the `TimeZone` trait.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);
    /// assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_utc(datetime: NaiveDateTime, offset: Tz::Offset) -> DateTime<Tz> {
        DateTime { datetime, offset }
    }
    /// Makes a new `DateTime` with given **local** datetime and offset that
    /// presents local timezone.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::DateTime;
    /// use chrono::naive::NaiveDate;
    /// use chrono::offset::{Utc, FixedOffset};
    ///
    /// let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(2, 0, 0).unwrap();
    /// let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);
    ///
    /// let timezone_east = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    /// let naivedatetime_east = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(10, 0, 0).unwrap();
    /// let datetime_east = DateTime::<FixedOffset>::from_local(naivedatetime_east, timezone_east);
    ///
    /// let timezone_west = FixedOffset::west_opt(7 * 60 * 60).unwrap();
    /// let naivedatetime_west = NaiveDate::from_ymd_opt(2000, 1, 11).unwrap().and_hms_opt(19, 0, 0).unwrap();
    /// let datetime_west = DateTime::<FixedOffset>::from_local(naivedatetime_west, timezone_west);
    /// assert_eq!(datetime_east, datetime_utc.with_timezone(&timezone_east));
    /// assert_eq!(datetime_west, datetime_utc.with_timezone(&timezone_west));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_local(datetime: NaiveDateTime, offset: Tz::Offset) -> DateTime<Tz> {
        let datetime_utc = datetime - offset.fix();
        DateTime {
            datetime: datetime_utc,
            offset,
        }
    }
    /// Retrieves a date component
    ///
    /// Unless you are immediately planning on turning this into a `DateTime`
    /// with the same Timezone you should use the
    /// [`date_naive`](DateTime::date_naive) method.
    #[inline]
    #[deprecated(since = "0.4.23", note = "Use `date_naive()` instead")]
    #[allow(deprecated)]
    #[must_use]
    pub fn date(&self) -> Date<Tz> {
        Date::from_utc(self.naive_local().date(), self.offset.clone())
    }
    /// Retrieves the Date without an associated timezone
    ///
    /// [`NaiveDate`] is a more well-defined type, and has more traits implemented on it,
    /// so should be preferred to [`Date`] any time you truly want to operate on Dates.
    ///
    /// ```
    /// use chrono::prelude::*;
    ///
    /// let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    /// let other: DateTime<FixedOffset> = FixedOffset::east_opt(23).unwrap().with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    /// assert_eq!(date.date_naive(), other.date_naive());
    /// ```
    #[inline]
    #[must_use]
    pub fn date_naive(&self) -> NaiveDate {
        let local = self.naive_local();
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap()
    }
    /// Retrieves a time component.
    /// Unlike `date`, this is not associated to the time zone.
    #[inline]
    #[must_use]
    pub fn time(&self) -> NaiveTime {
        self.datetime.time() + self.offset.fix()
    }
    /// Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC
    /// (aka "UNIX timestamp").
    #[inline]
    #[must_use]
    pub fn timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }
    /// Returns the number of non-leap-milliseconds since January 1, 1970 UTC
    ///
    /// Note that this does reduce the number of years that can be represented
    /// from ~584 Billion to ~584 Million. (If this is a problem, please file
    /// an issue to let me know what domain needs millisecond precision over
    /// billions of years, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, NaiveDate};
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(0, 0, 1, 444).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_millis(), 1_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_milli_opt(1, 46, 40, 555).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_millis(), 1_000_000_000_555);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_millis(&self) -> i64 {
        self.datetime.timestamp_millis()
    }
    /// Returns the number of non-leap-microseconds since January 1, 1970 UTC
    ///
    /// Note that this does reduce the number of years that can be represented
    /// from ~584 Billion to ~584 Thousand. (If this is a problem, please file
    /// an issue to let me know what domain needs microsecond precision over
    /// millennia, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, NaiveDate};
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_micro_opt(0, 0, 1, 444).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_micros(), 1_000_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_micro_opt(1, 46, 40, 555).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_micros(), 1_000_000_000_000_555);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_micros(&self) -> i64 {
        self.datetime.timestamp_micros()
    }
    /// Returns the number of non-leap-nanoseconds since January 1, 1970 UTC
    ///
    /// Note that this does reduce the number of years that can be represented
    /// from ~584 Billion to ~584. (If this is a problem, please file
    /// an issue to let me know what domain needs nanosecond precision over
    /// millennia, I'm curious.)
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, NaiveDate};
    ///
    /// let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_nano_opt(0, 0, 1, 444).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_nanos(), 1_000_000_444);
    ///
    /// let dt = NaiveDate::from_ymd_opt(2001, 9, 9).unwrap().and_hms_nano_opt(1, 46, 40, 555).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.timestamp_nanos(), 1_000_000_000_000_000_555);
    /// ```
    #[inline]
    #[must_use]
    pub fn timestamp_nanos(&self) -> i64 {
        self.datetime.timestamp_nanos()
    }
    /// Returns the number of milliseconds since the last second boundary
    ///
    /// warning: in event of a leap second, this may exceed 999
    ///
    /// note: this is not the number of milliseconds since January 1, 1970 0:00:00 UTC
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_millis(&self) -> u32 {
        self.datetime.timestamp_subsec_millis()
    }
    /// Returns the number of microseconds since the last second boundary
    ///
    /// warning: in event of a leap second, this may exceed 999_999
    ///
    /// note: this is not the number of microseconds since January 1, 1970 0:00:00 UTC
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_micros(&self) -> u32 {
        self.datetime.timestamp_subsec_micros()
    }
    /// Returns the number of nanoseconds since the last second boundary
    ///
    /// warning: in event of a leap second, this may exceed 999_999_999
    ///
    /// note: this is not the number of nanoseconds since January 1, 1970 0:00:00 UTC
    #[inline]
    #[must_use]
    pub fn timestamp_subsec_nanos(&self) -> u32 {
        self.datetime.timestamp_subsec_nanos()
    }
    /// Retrieves an associated offset from UTC.
    #[inline]
    #[must_use]
    pub fn offset(&self) -> &Tz::Offset {
        &self.offset
    }
    /// Retrieves an associated time zone.
    #[inline]
    #[must_use]
    pub fn timezone(&self) -> Tz {
        TimeZone::from_offset(&self.offset)
    }
    /// Changes the associated time zone.
    /// The returned `DateTime` references the same instant of time from the perspective of the provided time zone.
    #[inline]
    #[must_use]
    pub fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> DateTime<Tz2> {
        tz.from_utc_datetime(&self.datetime)
    }
    /// Fix the offset from UTC to its current value, dropping the associated timezone information.
    /// This it useful for converting a generic `DateTime<Tz: Timezone>` to `DateTime<FixedOffset>`.
    #[inline]
    #[must_use]
    pub fn fixed_offset(&self) -> DateTime<FixedOffset> {
        self.with_timezone(&self.offset().fix())
    }
    /// Adds given `Duration` to the current date and time.
    ///
    /// Returns `None` when it will result in overflow.
    #[inline]
    #[must_use]
    pub fn checked_add_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>> {
        let datetime = self.datetime.checked_add_signed(rhs)?;
        let tz = self.timezone();
        Some(tz.from_utc_datetime(&datetime))
    }
    /// Adds given `Months` to the current date and time.
    ///
    /// Returns `None` when it will result in overflow, or if the
    /// local time is not valid on the newly calculated date.
    ///
    /// See [`NaiveDate::checked_add_months`] for more details on behavior
    #[must_use]
    pub fn checked_add_months(self, rhs: Months) -> Option<DateTime<Tz>> {
        self.naive_local()
            .checked_add_months(rhs)?
            .and_local_timezone(Tz::from_offset(&self.offset))
            .single()
    }
    /// Subtracts given `Duration` from the current date and time.
    ///
    /// Returns `None` when it will result in overflow.
    #[inline]
    #[must_use]
    pub fn checked_sub_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>> {
        let datetime = self.datetime.checked_sub_signed(rhs)?;
        let tz = self.timezone();
        Some(tz.from_utc_datetime(&datetime))
    }
    /// Subtracts given `Months` from the current date and time.
    ///
    /// Returns `None` when it will result in overflow, or if the
    /// local time is not valid on the newly calculated date.
    ///
    /// See [`NaiveDate::checked_sub_months`] for more details on behavior
    #[must_use]
    pub fn checked_sub_months(self, rhs: Months) -> Option<DateTime<Tz>> {
        self.naive_local()
            .checked_sub_months(rhs)?
            .and_local_timezone(Tz::from_offset(&self.offset))
            .single()
    }
    /// Add a duration in [`Days`] to the date part of the `DateTime`
    ///
    /// Returns `None` if the resulting date would be out of range.
    #[must_use]
    pub fn checked_add_days(self, days: Days) -> Option<Self> {
        self.naive_local()
            .checked_add_days(days)?
            .and_local_timezone(TimeZone::from_offset(&self.offset))
            .single()
    }
    /// Subtract a duration in [`Days`] from the date part of the `DateTime`
    ///
    /// Returns `None` if the resulting date would be out of range.
    #[must_use]
    pub fn checked_sub_days(self, days: Days) -> Option<Self> {
        self.naive_local()
            .checked_sub_days(days)?
            .and_local_timezone(TimeZone::from_offset(&self.offset))
            .single()
    }
    /// Subtracts another `DateTime` from the current date and time.
    /// This does not overflow or underflow at all.
    #[inline]
    #[must_use]
    pub fn signed_duration_since<Tz2: TimeZone>(self, rhs: DateTime<Tz2>) -> TimeDelta {
        self.datetime.signed_duration_since(rhs.datetime)
    }
    /// Returns a view to the naive UTC datetime.
    #[inline]
    #[must_use]
    pub fn naive_utc(&self) -> NaiveDateTime {
        self.datetime
    }
    /// Returns a view to the naive local datetime.
    #[inline]
    #[must_use]
    pub fn naive_local(&self) -> NaiveDateTime {
        self.datetime + self.offset.fix()
    }
    /// Retrieve the elapsed years from now to the given [`DateTime`].
    #[must_use]
    pub fn years_since(&self, base: Self) -> Option<u32> {
        let mut years = self.year() - base.year();
        let earlier_time = (self.month(), self.day(), self.time())
            < (base.month(), base.day(), base.time());
        years
            -= match earlier_time {
                true => 1,
                false => 0,
            };
        match years >= 0 {
            true => Some(years as u32),
            false => None,
        }
    }
    /// The minimum possible `DateTime<Utc>`.
    pub const MIN_UTC: DateTime<Utc> = DateTime {
        datetime: NaiveDateTime::MIN,
        offset: Utc,
    };
    /// The maximum possible `DateTime<Utc>`.
    pub const MAX_UTC: DateTime<Utc> = DateTime {
        datetime: NaiveDateTime::MAX,
        offset: Utc,
    };
}
impl Default for DateTime<Utc> {
    fn default() -> Self {
        Utc.from_utc_datetime(&NaiveDateTime::default())
    }
}
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl Default for DateTime<Local> {
    fn default() -> Self {
        Local.from_utc_datetime(&NaiveDateTime::default())
    }
}
impl Default for DateTime<FixedOffset> {
    fn default() -> Self {
        FixedOffset::west_opt(0).unwrap().from_utc_datetime(&NaiveDateTime::default())
    }
}
/// Convert a `DateTime<Utc>` instance into a `DateTime<FixedOffset>` instance.
impl From<DateTime<Utc>> for DateTime<FixedOffset> {
    /// Convert this `DateTime<Utc>` instance into a `DateTime<FixedOffset>` instance.
    ///
    /// Conversion is done via [`DateTime::with_timezone`]. Note that the converted value returned by
    /// this will be created with a fixed timezone offset of 0.
    fn from(src: DateTime<Utc>) -> Self {
        src.with_timezone(&FixedOffset::east_opt(0).unwrap())
    }
}
/// Convert a `DateTime<Utc>` instance into a `DateTime<Local>` instance.
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl From<DateTime<Utc>> for DateTime<Local> {
    /// Convert this `DateTime<Utc>` instance into a `DateTime<Local>` instance.
    ///
    /// Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in timezones.
    fn from(src: DateTime<Utc>) -> Self {
        src.with_timezone(&Local)
    }
}
/// Convert a `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance.
impl From<DateTime<FixedOffset>> for DateTime<Utc> {
    /// Convert this `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance.
    ///
    /// Conversion is performed via [`DateTime::with_timezone`], accounting for the timezone
    /// difference.
    fn from(src: DateTime<FixedOffset>) -> Self {
        src.with_timezone(&Utc)
    }
}
/// Convert a `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance.
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl From<DateTime<FixedOffset>> for DateTime<Local> {
    /// Convert this `DateTime<FixedOffset>` instance into a `DateTime<Local>` instance.
    ///
    /// Conversion is performed via [`DateTime::with_timezone`]. Returns the equivalent value in local
    /// time.
    fn from(src: DateTime<FixedOffset>) -> Self {
        src.with_timezone(&Local)
    }
}
/// Convert a `DateTime<Local>` instance into a `DateTime<Utc>` instance.
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl From<DateTime<Local>> for DateTime<Utc> {
    /// Convert this `DateTime<Local>` instance into a `DateTime<Utc>` instance.
    ///
    /// Conversion is performed via [`DateTime::with_timezone`], accounting for the difference in
    /// timezones.
    fn from(src: DateTime<Local>) -> Self {
        src.with_timezone(&Utc)
    }
}
/// Convert a `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance.
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl From<DateTime<Local>> for DateTime<FixedOffset> {
    /// Convert this `DateTime<Local>` instance into a `DateTime<FixedOffset>` instance.
    ///
    /// Conversion is performed via [`DateTime::with_timezone`].
    fn from(src: DateTime<Local>) -> Self {
        src.with_timezone(&src.offset().fix())
    }
}
/// Maps the local datetime to other datetime with given conversion function.
fn map_local<Tz: TimeZone, F>(dt: &DateTime<Tz>, mut f: F) -> Option<DateTime<Tz>>
where
    F: FnMut(NaiveDateTime) -> Option<NaiveDateTime>,
{
    f(dt.naive_local())
        .and_then(|datetime| dt.timezone().from_local_datetime(&datetime).single())
}
impl DateTime<FixedOffset> {
    /// Parses an RFC 2822 date-and-time string into a `DateTime<FixedOffset>` value.
    ///
    /// This parses valid RFC 2822 datetime strings (such as `Tue, 1 Jul 2003 10:52:37 +0200`)
    /// and returns a new [`DateTime`] instance with the parsed timezone as the [`FixedOffset`].
    ///
    /// RFC 2822 is the internet message standard that specifies the representation of times in HTTP
    /// and email headers.
    ///
    /// The RFC 2822 standard allows arbitrary intermixed whitespace.
    /// See [RFC 2822 Appendix A.5]
    ///
    /// The RFC 2822 standard allows arbitrary intermixed whitespace.
    /// See [RFC 2822 Appendix A.5]
    ///
    /// ```
    /// # use chrono::{DateTime, FixedOffset, TimeZone, NaiveDate};
    /// assert_eq!(
    ///     DateTime::<FixedOffset>::parse_from_rfc2822("Wed, 18 Feb 2015 23:16:09 GMT").unwrap(),
    ///     FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(2015, 2, 18, 23, 16, 9).unwrap()
    /// );
    /// ```
    ///
    /// [RFC 2822 Appendix A.5]: https://www.rfc-editor.org/rfc/rfc2822#appendix-A.5
    pub fn parse_from_rfc2822(s: &str) -> ParseResult<DateTime<FixedOffset>> {
        const ITEMS: &[Item<'static>] = &[Item::Fixed(Fixed::RFC2822)];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.to_datetime()
    }
    /// Parses an RFC 3339 date-and-time string into a `DateTime<FixedOffset>` value.
    ///
    /// Parses all valid RFC 3339 values (as well as the subset of valid ISO 8601 values that are
    /// also valid RFC 3339 date-and-time values) and returns a new [`DateTime`] with a
    /// [`FixedOffset`] corresponding to the parsed timezone. While RFC 3339 values come in a wide
    /// variety of shapes and sizes, `1996-12-19T16:39:57-08:00` is an example of the most commonly
    /// encountered variety of RFC 3339 formats.
    ///
    /// Why isn't this named `parse_from_iso8601`? That's because ISO 8601 allows representing
    /// values in a wide range of formats, only some of which represent actual date-and-time
    /// instances (rather than periods, ranges, dates, or times). Some valid ISO 8601 values are
    /// also simultaneously valid RFC 3339 values, but not all RFC 3339 values are valid ISO 8601
    /// values (or the other way around).
    pub fn parse_from_rfc3339(s: &str) -> ParseResult<DateTime<FixedOffset>> {
        const ITEMS: &[Item<'static>] = &[Item::Fixed(Fixed::RFC3339)];
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, ITEMS.iter())?;
        parsed.to_datetime()
    }
    /// Parses a string from a user-specified format into a `DateTime<FixedOffset>` value.
    ///
    /// Note that this method *requires a timezone* in the input string. See
    /// [`NaiveDateTime::parse_from_str`](./naive/struct.NaiveDateTime.html#method.parse_from_str)
    /// for a version that does not require a timezone in the to-be-parsed str. The returned
    /// [`DateTime`] value will have a [`FixedOffset`] reflecting the parsed timezone.
    ///
    /// See the [`format::strftime` module](./format/strftime/index.html) for supported format
    /// sequences.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::{DateTime, FixedOffset, TimeZone, NaiveDate};
    ///
    /// let dt = DateTime::<FixedOffset>::parse_from_str(
    ///     "1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z");
    /// assert_eq!(dt, Ok(FixedOffset::east_opt(0).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(1983, 4, 13).unwrap().and_hms_milli_opt(12, 9, 14, 274).unwrap()).unwrap()));
    /// ```
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<DateTime<FixedOffset>> {
        let mut parsed = Parsed::new();
        parse(&mut parsed, s, StrftimeItems::new(fmt))?;
        parsed.to_datetime()
    }
}
impl DateTime<Utc> {
    /// Parses an RFC 2822 date-and-time string into a `DateTime<Utc>` value.
    ///
    /// This parses valid RFC 2822 datetime values (such as `Tue, 1 Jul 2003 10:52:37 +0200`)
    /// and returns a new `DateTime<Utc>` instance corresponding to the UTC date/time, accounting
    /// for the difference between UTC and the parsed timezone, should they differ.
    ///
    /// RFC 2822 is the internet message standard that specifies the representation of times in HTTP
    /// and email headers.
    pub fn parse_from_rfc2822(s: &str) -> ParseResult<DateTime<Utc>> {
        DateTime::<FixedOffset>::parse_from_rfc2822(s).map(|result| result.into())
    }
    /// Parses an RFC 3339 date-and-time string into a `DateTime<Utc>` value.
    ///
    /// Parses all valid RFC 3339 values (as well as the subset of valid ISO 8601 values that are
    /// also valid RFC 3339 date-and-time values) and returns a new `DateTime<Utc>` instance
    /// corresponding to the matching UTC date/time, accounting for the difference between UTC and
    /// the parsed input's timezone, should they differ. While RFC 3339 values come in a wide
    /// variety of shapes and sizes, `1996-12-19T16:39:57-08:00` is an example of the most commonly
    /// encountered variety of RFC 3339 formats.
    ///
    /// Why isn't this named `parse_from_iso8601`? That's because ISO 8601 allows representing
    /// values in a wide range of formats, only some of which represent actual date-and-time
    /// instances (rather than periods, ranges, dates, or times). Some valid ISO 8601 values are
    /// also simultaneously valid RFC 3339 values, but not all RFC 3339 values are valid ISO 8601
    /// values (or the other way around).
    pub fn parse_from_rfc3339(s: &str) -> ParseResult<DateTime<Utc>> {
        DateTime::<FixedOffset>::parse_from_rfc3339(s).map(|result| result.into())
    }
    /// Parses a string from a user-specified format into a `DateTime<Utc>` value.
    ///
    /// Note that this method *requires a timezone* in the input string. See
    /// [`NaiveDateTime::parse_from_str`](./naive/struct.NaiveDateTime.html#method.parse_from_str)
    /// for a version that does not require a timezone in the to-be-parsed str. The returned
    /// `DateTime<Utc>` value will reflect the difference in timezones between UTC and the parsed
    /// time zone, should they differ.
    ///
    /// See the [`format::strftime` module](./format/strftime/index.html) for supported format
    /// sequences.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::{DateTime, TimeZone, Utc};
    ///
    /// let dt = DateTime::<Utc>::parse_from_str(
    ///     "1983 Apr 13 12:09:14.274 +0100", "%Y %b %d %H:%M:%S%.3f %z");
    /// assert_eq!(dt, Ok(Utc.ymd(1983, 4, 13).and_hms_milli(11, 9, 14, 274)));
    /// ```
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<DateTime<Utc>> {
        DateTime::<FixedOffset>::parse_from_str(s, fmt).map(|result| result.into())
    }
}
impl<Tz: TimeZone> DateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    /// Returns an RFC 2822 date and time string such as `Tue, 1 Jul 2003 10:52:37 +0200`.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[must_use]
    pub fn to_rfc2822(&self) -> String {
        let mut result = String::with_capacity(32);
        crate::format::write_rfc2822(&mut result, self.naive_local(), self.offset.fix())
            .expect("writing rfc2822 datetime to string should never fail");
        result
    }
    /// Returns an RFC 3339 and ISO 8601 date and time string such as `1996-12-19T16:39:57-08:00`.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[must_use]
    pub fn to_rfc3339(&self) -> String {
        let mut result = String::with_capacity(32);
        crate::format::write_rfc3339(&mut result, self.naive_local(), self.offset.fix())
            .expect("writing rfc3339 datetime to string should never fail");
        result
    }
    /// Return an RFC 3339 and ISO 8601 date and time string with subseconds
    /// formatted as per `SecondsFormat`.
    ///
    /// If `use_z` is true and the timezone is UTC (offset 0), uses `Z` as
    /// per [`Fixed::TimezoneOffsetColonZ`]. If `use_z` is false, uses
    /// [`Fixed::TimezoneOffsetColon`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use chrono::{DateTime, FixedOffset, SecondsFormat, TimeZone, Utc, NaiveDate};
    /// let dt = NaiveDate::from_ymd_opt(2018, 1, 26).unwrap().and_hms_micro_opt(18, 30, 9, 453_829).unwrap().and_local_timezone(Utc).unwrap();
    /// assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Millis, false),
    ///            "2018-01-26T18:30:09.453+00:00");
    /// assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Millis, true),
    ///            "2018-01-26T18:30:09.453Z");
    /// assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Secs, true),
    ///            "2018-01-26T18:30:09Z");
    ///
    /// let pst = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    /// let dt = pst.from_local_datetime(&NaiveDate::from_ymd_opt(2018, 1, 26).unwrap().and_hms_micro_opt(10, 30, 9, 453_829).unwrap()).unwrap();
    /// assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Secs, true),
    ///            "2018-01-26T10:30:09+08:00");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[must_use]
    pub fn to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String {
        use crate::format::Numeric::*;
        use crate::format::Pad::Zero;
        use crate::SecondsFormat::*;
        const PREFIX: &[Item<'static>] = &[
            Item::Numeric(Year, Zero),
            Item::Literal("-"),
            Item::Numeric(Month, Zero),
            Item::Literal("-"),
            Item::Numeric(Day, Zero),
            Item::Literal("T"),
            Item::Numeric(Hour, Zero),
            Item::Literal(":"),
            Item::Numeric(Minute, Zero),
            Item::Literal(":"),
            Item::Numeric(Second, Zero),
        ];
        let ssitem = match secform {
            Secs => None,
            Millis => Some(Item::Fixed(Fixed::Nanosecond3)),
            Micros => Some(Item::Fixed(Fixed::Nanosecond6)),
            Nanos => Some(Item::Fixed(Fixed::Nanosecond9)),
            AutoSi => Some(Item::Fixed(Fixed::Nanosecond)),
        };
        let tzitem = Item::Fixed(
            if use_z { Fixed::TimezoneOffsetColonZ } else { Fixed::TimezoneOffsetColon },
        );
        match ssitem {
            None => {
                self.format_with_items(PREFIX.iter().chain([tzitem].iter())).to_string()
            }
            Some(s) => {
                self
                    .format_with_items(PREFIX.iter().chain([s, tzitem].iter()))
                    .to_string()
            }
        }
    }
    /// Formats the combined date and time with the specified formatting items.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format_with_items<'a, I, B>(&self, items: I) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        let local = self.naive_local();
        DelayedFormat::new_with_offset(
            Some(local.date()),
            Some(local.time()),
            &self.offset,
            items,
        )
    }
    /// Formats the combined date and time per the specified format string.
    ///
    /// See the [`crate::format::strftime`] module for the supported escape sequences.
    ///
    /// # Example
    /// ```rust
    /// use chrono::prelude::*;
    ///
    /// let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2017, 04, 02, 12, 50, 32).unwrap();
    /// let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));
    /// assert_eq!(formatted, "02/04/2017 12:50");
    /// ```
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    #[inline]
    #[must_use]
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_with_items(StrftimeItems::new(fmt))
    }
    /// Formats the combined date and time with the specified formatting items and locale.
    #[cfg(feature = "unstable-locales")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-locales")))]
    #[inline]
    #[must_use]
    pub fn format_localized_with_items<'a, I, B>(
        &self,
        items: I,
        locale: Locale,
    ) -> DelayedFormat<I>
    where
        I: Iterator<Item = B> + Clone,
        B: Borrow<Item<'a>>,
    {
        let local = self.naive_local();
        DelayedFormat::new_with_offset_and_locale(
            Some(local.date()),
            Some(local.time()),
            &self.offset,
            items,
            locale,
        )
    }
    /// Formats the combined date and time per the specified format string and
    /// locale.
    ///
    /// See the [`crate::format::strftime`] module on the supported escape
    /// sequences.
    #[cfg(feature = "unstable-locales")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-locales")))]
    #[inline]
    #[must_use]
    pub fn format_localized<'a>(
        &self,
        fmt: &'a str,
        locale: Locale,
    ) -> DelayedFormat<StrftimeItems<'a>> {
        self.format_localized_with_items(
            StrftimeItems::new_with_locale(fmt, locale),
            locale,
        )
    }
}
impl<Tz: TimeZone> Datelike for DateTime<Tz> {
    #[inline]
    fn year(&self) -> i32 {
        self.naive_local().year()
    }
    #[inline]
    fn month(&self) -> u32 {
        self.naive_local().month()
    }
    #[inline]
    fn month0(&self) -> u32 {
        self.naive_local().month0()
    }
    #[inline]
    fn day(&self) -> u32 {
        self.naive_local().day()
    }
    #[inline]
    fn day0(&self) -> u32 {
        self.naive_local().day0()
    }
    #[inline]
    fn ordinal(&self) -> u32 {
        self.naive_local().ordinal()
    }
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.naive_local().ordinal0()
    }
    #[inline]
    fn weekday(&self) -> Weekday {
        self.naive_local().weekday()
    }
    #[inline]
    fn iso_week(&self) -> IsoWeek {
        self.naive_local().iso_week()
    }
    #[inline]
    fn with_year(&self, year: i32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_year(year))
    }
    #[inline]
    fn with_month(&self, month: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_month(month))
    }
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_month0(month0))
    }
    #[inline]
    fn with_day(&self, day: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_day(day))
    }
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_day0(day0))
    }
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_ordinal(ordinal))
    }
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_ordinal0(ordinal0))
    }
}
impl<Tz: TimeZone> Timelike for DateTime<Tz> {
    #[inline]
    fn hour(&self) -> u32 {
        self.naive_local().hour()
    }
    #[inline]
    fn minute(&self) -> u32 {
        self.naive_local().minute()
    }
    #[inline]
    fn second(&self) -> u32 {
        self.naive_local().second()
    }
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.naive_local().nanosecond()
    }
    #[inline]
    fn with_hour(&self, hour: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_hour(hour))
    }
    #[inline]
    fn with_minute(&self, min: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_minute(min))
    }
    #[inline]
    fn with_second(&self, sec: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_second(sec))
    }
    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<DateTime<Tz>> {
        map_local(self, |datetime| datetime.with_nanosecond(nano))
    }
}
impl<Tz: TimeZone> Copy for DateTime<Tz>
where
    <Tz as TimeZone>::Offset: Copy,
{}
unsafe impl<Tz: TimeZone> Send for DateTime<Tz>
where
    <Tz as TimeZone>::Offset: Send,
{}
impl<Tz: TimeZone, Tz2: TimeZone> PartialEq<DateTime<Tz2>> for DateTime<Tz> {
    fn eq(&self, other: &DateTime<Tz2>) -> bool {
        self.datetime == other.datetime
    }
}
impl<Tz: TimeZone> Eq for DateTime<Tz> {}
impl<Tz: TimeZone, Tz2: TimeZone> PartialOrd<DateTime<Tz2>> for DateTime<Tz> {
    /// Compare two DateTimes based on their true time, ignoring time zones
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::prelude::*;
    ///
    /// let earlier = Utc.with_ymd_and_hms(2015, 5, 15, 2, 0, 0).unwrap().with_timezone(&FixedOffset::west_opt(1 * 3600).unwrap());
    /// let later   = Utc.with_ymd_and_hms(2015, 5, 15, 3, 0, 0).unwrap().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
    ///
    /// assert_eq!(earlier.to_string(), "2015-05-15 01:00:00 -01:00");
    /// assert_eq!(later.to_string(), "2015-05-14 22:00:00 -05:00");
    ///
    /// assert!(later > earlier);
    /// ```
    fn partial_cmp(&self, other: &DateTime<Tz2>) -> Option<Ordering> {
        self.datetime.partial_cmp(&other.datetime)
    }
}
impl<Tz: TimeZone> Ord for DateTime<Tz> {
    fn cmp(&self, other: &DateTime<Tz>) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}
impl<Tz: TimeZone> hash::Hash for DateTime<Tz> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.datetime.hash(state)
    }
}
impl<Tz: TimeZone> Add<TimeDelta> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    #[inline]
    fn add(self, rhs: TimeDelta) -> DateTime<Tz> {
        self.checked_add_signed(rhs).expect("`DateTime + Duration` overflowed")
    }
}
impl<Tz: TimeZone> AddAssign<TimeDelta> for DateTime<Tz> {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        let datetime = self
            .datetime
            .checked_add_signed(rhs)
            .expect("`DateTime + Duration` overflowed");
        let tz = self.timezone();
        *self = tz.from_utc_datetime(&datetime);
    }
}
impl<Tz: TimeZone> Add<Months> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    fn add(self, rhs: Months) -> Self::Output {
        self.checked_add_months(rhs).unwrap()
    }
}
impl<Tz: TimeZone> Sub<TimeDelta> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    #[inline]
    fn sub(self, rhs: TimeDelta) -> DateTime<Tz> {
        self.checked_sub_signed(rhs).expect("`DateTime - Duration` overflowed")
    }
}
impl<Tz: TimeZone> SubAssign<TimeDelta> for DateTime<Tz> {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        let datetime = self
            .datetime
            .checked_sub_signed(rhs)
            .expect("`DateTime - Duration` overflowed");
        let tz = self.timezone();
        *self = tz.from_utc_datetime(&datetime);
    }
}
impl<Tz: TimeZone> Sub<Months> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    fn sub(self, rhs: Months) -> Self::Output {
        self.checked_sub_months(rhs).unwrap()
    }
}
impl<Tz: TimeZone> Sub<DateTime<Tz>> for DateTime<Tz> {
    type Output = TimeDelta;
    #[inline]
    fn sub(self, rhs: DateTime<Tz>) -> TimeDelta {
        self.signed_duration_since(rhs)
    }
}
impl<Tz: TimeZone> Add<Days> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    fn add(self, days: Days) -> Self::Output {
        self.checked_add_days(days).unwrap()
    }
}
impl<Tz: TimeZone> Sub<Days> for DateTime<Tz> {
    type Output = DateTime<Tz>;
    fn sub(self, days: Days) -> Self::Output {
        self.checked_sub_days(days).unwrap()
    }
}
impl<Tz: TimeZone> fmt::Debug for DateTime<Tz> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.naive_local().fmt(f)?;
        self.offset.fmt(f)
    }
}
impl<Tz: TimeZone> fmt::Display for DateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.naive_local().fmt(f)?;
        f.write_char(' ')?;
        self.offset.fmt(f)
    }
}
/// Accepts a relaxed form of RFC3339.
/// A space or a 'T' are acepted as the separator between the date and time
/// parts. Additional spaces are allowed between each component.
///
/// All of these examples are equivalent:
/// ```
/// # use chrono::{DateTime, Utc};
/// "2012-12-12T12:12:12Z".parse::<DateTime<Utc>>();
/// "2012-12-12 12:12:12Z".parse::<DateTime<Utc>>();
/// "2012-  12-12T12:  12:12Z".parse::<DateTime<Utc>>();
/// ```
impl str::FromStr for DateTime<Utc> {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<DateTime<Utc>> {
        s.parse::<DateTime<FixedOffset>>().map(|dt| dt.with_timezone(&Utc))
    }
}
/// Accepts a relaxed form of RFC3339.
/// A space or a 'T' are acepted as the separator between the date and time
/// parts. Additional spaces are allowed between each component.
///
/// All of these examples are equivalent:
/// ```
/// # use chrono::{DateTime, Local};
/// "2012-12-12T12:12:12Z".parse::<DateTime<Local>>();
/// "2012-12-12 12:12:12Z".parse::<DateTime<Local>>();
/// "2012-  12-12T12:  12:12Z".parse::<DateTime<Local>>();
/// ```
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl str::FromStr for DateTime<Local> {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<DateTime<Local>> {
        s.parse::<DateTime<FixedOffset>>().map(|dt| dt.with_timezone(&Local))
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<SystemTime> for DateTime<Utc> {
    fn from(t: SystemTime) -> DateTime<Utc> {
        let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
            Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
            Err(e) => {
                let dur = e.duration();
                let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
                if nsec == 0 { (-sec, 0) } else { (-sec - 1, 1_000_000_000 - nsec) }
            }
        };
        Utc.timestamp_opt(sec, nsec).unwrap()
    }
}
#[cfg(feature = "clock")]
#[cfg_attr(docsrs, doc(cfg(feature = "clock")))]
impl From<SystemTime> for DateTime<Local> {
    fn from(t: SystemTime) -> DateTime<Local> {
        DateTime::<Utc>::from(t).with_timezone(&Local)
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<Tz: TimeZone> From<DateTime<Tz>> for SystemTime {
    fn from(dt: DateTime<Tz>) -> SystemTime {
        use std::time::Duration;
        let sec = dt.timestamp();
        let nsec = dt.timestamp_subsec_nanos();
        if sec < 0 {
            UNIX_EPOCH - Duration::new(-sec as u64, 0) + Duration::new(0, nsec)
        } else {
            UNIX_EPOCH + Duration::new(sec as u64, nsec)
        }
    }
}
#[cfg(
    all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )
)]
#[cfg_attr(
    docsrs,
    doc(
        cfg(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )
)]
impl From<js_sys::Date> for DateTime<Utc> {
    fn from(date: js_sys::Date) -> DateTime<Utc> {
        DateTime::<Utc>::from(&date)
    }
}
#[cfg(
    all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )
)]
#[cfg_attr(
    docsrs,
    doc(
        cfg(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )
)]
impl From<&js_sys::Date> for DateTime<Utc> {
    fn from(date: &js_sys::Date) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(date.get_time() as i64).unwrap()
    }
}
#[cfg(
    all(
        target_arch = "wasm32",
        feature = "wasmbind",
        not(any(target_os = "emscripten", target_os = "wasi"))
    )
)]
#[cfg_attr(
    docsrs,
    doc(
        cfg(
            all(
                target_arch = "wasm32",
                feature = "wasmbind",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )
    )
)]
impl From<DateTime<Utc>> for js_sys::Date {
    /// Converts a `DateTime<Utc>` to a JS `Date`. The resulting value may be lossy,
    /// any values that have a millisecond timestamp value greater/less than ±8,640,000,000,000,000
    /// (April 20, 271821 BCE ~ September 13, 275760 CE) will become invalid dates in JS.
    fn from(date: DateTime<Utc>) -> js_sys::Date {
        let js_millis = wasm_bindgen::JsValue::from_f64(date.timestamp_millis() as f64);
        js_sys::Date::new(&js_millis)
    }
}
#[cfg(feature = "arbitrary")]
impl<'a, Tz> arbitrary::Arbitrary<'a> for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(
        u: &mut arbitrary::Unstructured<'a>,
    ) -> arbitrary::Result<DateTime<Tz>> {
        let datetime = NaiveDateTime::arbitrary(u)?;
        let offset = <Tz as TimeZone>::Offset::arbitrary(u)?;
        Ok(DateTime::from_utc(datetime, offset))
    }
}
#[test]
fn test_add_sub_months() {
    let utc_dt = Utc.with_ymd_and_hms(2018, 9, 5, 23, 58, 0).unwrap();
    assert_eq!(
        utc_dt + Months::new(15), Utc.with_ymd_and_hms(2019, 12, 5, 23, 58, 0).unwrap()
    );
    let utc_dt = Utc.with_ymd_and_hms(2020, 1, 31, 23, 58, 0).unwrap();
    assert_eq!(
        utc_dt + Months::new(1), Utc.with_ymd_and_hms(2020, 2, 29, 23, 58, 0).unwrap()
    );
    assert_eq!(
        utc_dt + Months::new(2), Utc.with_ymd_and_hms(2020, 3, 31, 23, 58, 0).unwrap()
    );
    let utc_dt = Utc.with_ymd_and_hms(2018, 9, 5, 23, 58, 0).unwrap();
    assert_eq!(
        utc_dt - Months::new(15), Utc.with_ymd_and_hms(2017, 6, 5, 23, 58, 0).unwrap()
    );
    let utc_dt = Utc.with_ymd_and_hms(2020, 3, 31, 23, 58, 0).unwrap();
    assert_eq!(
        utc_dt - Months::new(1), Utc.with_ymd_and_hms(2020, 2, 29, 23, 58, 0).unwrap()
    );
    assert_eq!(
        utc_dt - Months::new(2), Utc.with_ymd_and_hms(2020, 1, 31, 23, 58, 0).unwrap()
    );
}
#[test]
fn test_auto_conversion() {
    let utc_dt = Utc.with_ymd_and_hms(2018, 9, 5, 23, 58, 0).unwrap();
    let cdt_dt = FixedOffset::west_opt(5 * 60 * 60)
        .unwrap()
        .with_ymd_and_hms(2018, 9, 5, 18, 58, 0)
        .unwrap();
    let utc_dt2: DateTime<Utc> = cdt_dt.into();
    assert_eq!(utc_dt, utc_dt2);
}
#[cfg(all(test, feature = "serde"))]
fn test_encodable_json<FUtc, FFixed, E>(to_string_utc: FUtc, to_string_fixed: FFixed)
where
    FUtc: Fn(&DateTime<Utc>) -> Result<String, E>,
    FFixed: Fn(&DateTime<FixedOffset>) -> Result<String, E>,
    E: ::core::fmt::Debug,
{
    assert_eq!(
        to_string_utc(& Utc.with_ymd_and_hms(2014, 7, 24, 12, 34, 6).unwrap()).ok(),
        Some(r#""2014-07-24T12:34:06Z""#.into())
    );
    assert_eq!(
        to_string_fixed(& FixedOffset::east_opt(3660).unwrap().with_ymd_and_hms(2014, 7,
        24, 12, 34, 6).unwrap()).ok(), Some(r#""2014-07-24T12:34:06+01:01""#.into())
    );
    assert_eq!(
        to_string_fixed(& FixedOffset::east_opt(3650).unwrap().with_ymd_and_hms(2014, 7,
        24, 12, 34, 6).unwrap()).ok(), Some(r#""2014-07-24T12:34:06+01:00:50""#.into())
    );
}
#[cfg(all(test, feature = "clock", feature = "serde"))]
fn test_decodable_json<FUtc, FFixed, FLocal, E>(
    utc_from_str: FUtc,
    fixed_from_str: FFixed,
    local_from_str: FLocal,
)
where
    FUtc: Fn(&str) -> Result<DateTime<Utc>, E>,
    FFixed: Fn(&str) -> Result<DateTime<FixedOffset>, E>,
    FLocal: Fn(&str) -> Result<DateTime<Local>, E>,
    E: ::core::fmt::Debug,
{
    fn norm<Tz: TimeZone>(
        dt: &Option<DateTime<Tz>>,
    ) -> Option<(&DateTime<Tz>, &Tz::Offset)> {
        dt.as_ref().map(|dt| (dt, dt.offset()))
    }
    assert_eq!(
        norm(& utc_from_str(r#""2014-07-24T12:34:06Z""#).ok()), norm(& Some(Utc
        .with_ymd_and_hms(2014, 7, 24, 12, 34, 6).unwrap()))
    );
    assert_eq!(
        norm(& utc_from_str(r#""2014-07-24T13:57:06+01:23""#).ok()), norm(& Some(Utc
        .with_ymd_and_hms(2014, 7, 24, 12, 34, 6).unwrap()))
    );
    assert_eq!(
        norm(& fixed_from_str(r#""2014-07-24T12:34:06Z""#).ok()), norm(&
        Some(FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(2014, 7, 24, 12, 34, 6)
        .unwrap()))
    );
    assert_eq!(
        norm(& fixed_from_str(r#""2014-07-24T13:57:06+01:23""#).ok()), norm(&
        Some(FixedOffset::east_opt(60 * 60 + 23 * 60).unwrap().with_ymd_and_hms(2014, 7,
        24, 13, 57, 6).unwrap()))
    );
    assert_eq!(
        local_from_str(r#""2014-07-24T12:34:06Z""#).expect("local shouuld parse"), Utc
        .with_ymd_and_hms(2014, 7, 24, 12, 34, 6).unwrap()
    );
    assert_eq!(
        local_from_str(r#""2014-07-24T13:57:06+01:23""#)
        .expect("local should parse with offset"), Utc.with_ymd_and_hms(2014, 7, 24, 12,
        34, 6).unwrap()
    );
    assert!(utc_from_str(r#""2014-07-32T12:34:06Z""#).is_err());
    assert!(fixed_from_str(r#""2014-07-32T12:34:06Z""#).is_err());
}
#[cfg(test)]
mod tests_llm_16_30 {
    use crate::{DateTime, FixedOffset, Utc, offset::TimeZone};
    #[test]
    fn test_cmp_utc() {
        let _rug_st_tests_llm_16_30_rrrruuuugggg_test_cmp_utc = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2022;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 2022;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 12;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 0;
        let dt1_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt2_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let dt3_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .and_hms(rug_fuzz_15, rug_fuzz_16, rug_fuzz_17);
        debug_assert!(dt1_utc < dt2_utc);
        debug_assert!(dt2_utc > dt1_utc);
        debug_assert_eq!(dt1_utc, dt3_utc);
        let _rug_ed_tests_llm_16_30_rrrruuuugggg_test_cmp_utc = 0;
    }
    #[test]
    fn test_cmp_fixed_offset() {
        let _rug_st_tests_llm_16_30_rrrruuuugggg_test_cmp_fixed_offset = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2022;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 13;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2022;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 13;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 2022;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 13;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let dt1_fixed: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let dt2_fixed: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        let dt3_fixed: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .and_hms(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18);
        debug_assert!(dt1_fixed < dt2_fixed);
        debug_assert!(dt2_fixed > dt1_fixed);
        debug_assert_eq!(dt1_fixed, dt3_fixed);
        let _rug_ed_tests_llm_16_30_rrrruuuugggg_test_cmp_fixed_offset = 0;
    }
    #[test]
    fn test_cmp_mixed() {
        let _rug_st_tests_llm_16_30_rrrruuuugggg_test_cmp_mixed = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 2022;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 13;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed_offset = FixedOffset::east(rug_fuzz_6);
        let dt_fixed: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(dt_utc, dt_fixed);
        let _rug_ed_tests_llm_16_30_rrrruuuugggg_test_cmp_mixed = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    #[cfg(feature = "with-chrono")]
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    use std::str::FromStr;
    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_equality_between_offsets() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_equality_between_offsets = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2016;
        let rug_fuzz_3 = 11;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2016;
        let rug_fuzz_9 = 11;
        let rug_fuzz_10 = 8;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let fixed_offset = FixedOffset::west_opt(rug_fuzz_0 * rug_fuzz_1).unwrap();
        let datetime_with_fixed_offset = fixed_offset
            .ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .unwrap()
            .and_hms_opt(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .unwrap();
        let datetime_with_utc = Utc
            .ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .unwrap()
            .and_hms_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert_eq!(datetime_with_fixed_offset, datetime_with_utc);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_equality_between_offsets = 0;
    }
    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_equality_across_different_timezones() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_equality_across_different_timezones = 0;
        let rug_fuzz_0 = "2015-05-15T17:00:00Z";
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 3600;
        let utc: DateTime<Utc> = DateTime::from_str(rug_fuzz_0).unwrap();
        let fixed_offset = FixedOffset::east_opt(rug_fuzz_1 * rug_fuzz_2).unwrap();
        let with_timezone: DateTime<FixedOffset> = utc.with_timezone(&fixed_offset);
        debug_assert_eq!(utc, with_timezone);
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_equality_across_different_timezones = 0;
    }
    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_fixed_offset_equality() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_fixed_offset_equality = 0;
        let rug_fuzz_0 = 9;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 9;
        let rug_fuzz_3 = 3600;
        let rug_fuzz_4 = 2000;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 2000;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 1;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let fixed_offset1 = FixedOffset::east_opt(rug_fuzz_0 * rug_fuzz_1).unwrap();
        let fixed_offset2 = FixedOffset::east(rug_fuzz_2 * rug_fuzz_3);
        debug_assert_eq!(fixed_offset1, fixed_offset2);
        let datetime_fixed1 = fixed_offset1
            .ymd(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .and_hms(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        let datetime_fixed2 = fixed_offset2
            .ymd(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12)
            .and_hms(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15);
        debug_assert_ne!(datetime_fixed1, datetime_fixed2);
        debug_assert_eq!(datetime_fixed1.timestamp(), datetime_fixed2.timestamp());
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_fixed_offset_equality = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::{DateTime, FixedOffset, TimeZone, NaiveDate, Utc};
    #[test]
    fn test_partial_cmp() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_partial_cmp = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 15;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 3;
        let rug_fuzz_10 = 15;
        let rug_fuzz_11 = 16;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 2023;
        let rug_fuzz_15 = 3;
        let rug_fuzz_16 = 15;
        let rug_fuzz_17 = 15;
        let rug_fuzz_18 = 30;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 2023;
        let rug_fuzz_21 = 3;
        let rug_fuzz_22 = 15;
        let rug_fuzz_23 = 10;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 0;
        let offset = FixedOffset::east_opt(rug_fuzz_0 * rug_fuzz_1).unwrap();
        let earlier_dt: DateTime<FixedOffset> = offset
            .ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .unwrap()
            .and_hms_opt(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .unwrap();
        let later_dt: DateTime<Utc> = Utc
            .ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .unwrap()
            .and_hms_opt(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13)
            .unwrap();
        debug_assert!(earlier_dt.partial_cmp(& later_dt).unwrap().is_lt());
        debug_assert!(later_dt.partial_cmp(& earlier_dt).unwrap().is_gt());
        let same_dt_with_offset: DateTime<FixedOffset> = offset
            .ymd_opt(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .unwrap()
            .and_hms_opt(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19)
            .unwrap();
        debug_assert!(same_dt_with_offset.partial_cmp(& later_dt).unwrap().is_eq());
        let naivedate = NaiveDate::from_ymd_opt(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .unwrap();
        let dt_from_naive: DateTime<FixedOffset> = offset
            .from_utc_date(&naivedate)
            .and_hms_opt(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25)
            .unwrap();
        debug_assert!(dt_from_naive.partial_cmp(& earlier_dt).unwrap().is_eq());
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_partial_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use crate::{FixedOffset, TimeZone, NaiveTime, Timelike, NaiveDate, NaiveDateTime};
    fn create_fixed_offset(offset_secs: i32) -> FixedOffset {
        FixedOffset::east_opt(offset_secs).unwrap()
    }
    #[test]
    fn test_add_positive_offset() {
        let ndt = NaiveDate::from_ymd_opt(2023, 3, 5)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();
        let offset = create_fixed_offset(2 * 3600);
        let result = ndt + offset;
        let new_time = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
        let expected = NaiveDateTime::new(ndt.date(), new_time);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_add_negative_offset() {
        let ndt = NaiveDate::from_ymd_opt(2023, 3, 5)
            .unwrap()
            .and_hms_opt(5, 45, 30)
            .unwrap();
        let offset = create_fixed_offset(-3 * 3600);
        let result = ndt + offset;
        let new_time = NaiveTime::from_hms_opt(2, 45, 30).unwrap();
        let expected = NaiveDateTime::new(ndt.date(), new_time);
        assert_eq!(result, expected);
    }
    #[test]
    #[should_panic(expected = "FixedOffset::east out of bounds")]
    fn test_add_offset_out_of_bounds() {
        let ndt = NaiveDate::from_ymd_opt(2023, 3, 5)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();
        let _ = create_fixed_offset(25 * 3600);
    }
}
#[cfg(test)]
mod tests_llm_16_36 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    #[test]
    fn test_add_positive_duration() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_positive_duration = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 2020;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.add(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_positive_duration = 0;
    }
    #[test]
    fn test_add_negative_duration() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_negative_duration = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 2020;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        let result = dt.add(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_negative_duration = 0;
    }
    #[test]
    fn test_add_duration_leap_second() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_duration_leap_second = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2020;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.add(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_duration_leap_second = 0;
    }
    #[test]
    fn test_add_duration_overflow() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_duration_overflow = 0;
        let rug_fuzz_0 = 9999;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 2;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.checked_add_signed(duration);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_duration_overflow = 0;
    }
    #[test]
    fn test_add_duration_underflow() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_duration_underflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        let result = dt.checked_add_signed(duration);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_duration_underflow = 0;
    }
    #[test]
    fn test_add_duration_day_boundary() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_duration_day_boundary = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 2020;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 30;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.add(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_duration_day_boundary = 0;
    }
    #[test]
    fn test_add_duration_year_boundary() {
        let _rug_st_tests_llm_16_36_rrrruuuugggg_test_add_duration_year_boundary = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 7200;
        let rug_fuzz_7 = 2021;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.add(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_36_rrrruuuugggg_test_add_duration_year_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_37 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_add_assign_for_fixed_offset() {
        let _rug_st_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_fixed_offset = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 60;
        let rug_fuzz_9 = 60;
        let mut dt: DateTime<FixedOffset> = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1)
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let delta = TimeDelta::seconds(rug_fuzz_8 * rug_fuzz_9);
        dt.add_assign(delta);
        debug_assert_eq!(
            dt, FixedOffset::east(5 * 3600).ymd(2023, 4, 1).and_hms(13, 0, 0)
        );
        let _rug_ed_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_fixed_offset = 0;
    }
    #[test]
    #[should_panic(expected = "`DateTime + Duration` overflowed")]
    fn test_add_assign_for_fixed_offset_overflow() {
        let _rug_st_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_fixed_offset_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 262143;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = 31;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 1;
        let mut dt: DateTime<FixedOffset> = FixedOffset::east(rug_fuzz_0)
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let delta = TimeDelta::seconds(rug_fuzz_7);
        dt.add_assign(delta);
        let _rug_ed_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_fixed_offset_overflow = 0;
    }
    #[test]
    fn test_add_assign_for_utc() {
        let _rug_st_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_utc = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 60;
        let rug_fuzz_7 = 60;
        let mut dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let delta = TimeDelta::seconds(rug_fuzz_6 * rug_fuzz_7);
        dt.add_assign(delta);
        debug_assert_eq!(dt, Utc.ymd(2023, 4, 1).and_hms(13, 0, 0));
        let _rug_ed_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_utc = 0;
    }
    #[test]
    #[should_panic(expected = "`DateTime + Duration` overflowed")]
    fn test_add_assign_for_utc_overflow() {
        let _rug_st_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_utc_overflow = 0;
        let rug_fuzz_0 = 262143;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 1;
        let mut dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let delta = TimeDelta::seconds(rug_fuzz_6);
        dt.add_assign(delta);
        let _rug_ed_tests_llm_16_37_rrrruuuugggg_test_add_assign_for_utc_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    use crate::prelude::*;
    use crate::offset::Utc;
    #[test]
    fn test_sub_duration() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 6;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 4;
        let rug_fuzz_9 = 6;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration = 0;
    }
    #[test]
    fn test_sub_duration_with_leap_second() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration_with_leap_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 6;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 6;
        let rug_fuzz_9 = 30;
        let rug_fuzz_10 = 23;
        let rug_fuzz_11 = 59;
        let rug_fuzz_12 = 57;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration_with_leap_second = 0;
    }
    #[test]
    fn test_sub_duration_overflow() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration_overflow = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 6;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 86_400;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 4;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration_overflow = 0;
    }
    #[test]
    fn test_sub_duration_underflow() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration_underflow = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 86_400;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration_underflow = 0;
    }
    #[test]
    fn test_sub_duration_leap_year() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration_leap_year = 0;
        let rug_fuzz_0 = 2024;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 2024;
        let rug_fuzz_8 = 2;
        let rug_fuzz_9 = 28;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::days(rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration_leap_year = 0;
    }
    #[test]
    fn test_sub_duration_across_dst() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_sub_duration_across_dst = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::hours(rug_fuzz_6);
        let result = dt.sub(duration);
        let expected = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_sub_duration_across_dst = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    use crate::{DurationRound, NaiveDate, NaiveDateTime, Offset, TimeZone};
    #[test]
    fn test_sub() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_test_sub = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 14;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 1;
        let dt1 = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let dt2 = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8),
            NaiveTime::from_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11),
        );
        let offset = FixedOffset::east(rug_fuzz_12);
        let date_time1 = offset.from_utc_datetime(&dt1);
        let date_time2 = offset.from_utc_datetime(&dt2);
        let time_delta = TimeDelta::days(rug_fuzz_13);
        debug_assert_eq!(date_time2, date_time1.sub(time_delta));
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_test_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_42_llm_16_42 {
    use super::*;
    use crate::*;
    use crate::DateTime;
    use crate::offset::TimeZone;
    use crate::offset::Utc;
    use crate::offset::FixedOffset;
    use crate::offset::Local;
    use crate::naive::{NaiveDateTime, NaiveDate, NaiveTime};
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_sub_assign_duration_for_naive_date_time() {
        let _rug_st_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_sub_assign_duration_for_naive_date_time = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 30;
        let mut dt = NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let duration = TimeDelta::seconds(rug_fuzz_6);
        dt.sub_assign(duration);
        debug_assert_eq!(
            dt, NaiveDate::from_ymd_opt(2023, 4, 10).unwrap().and_hms_opt(12, 30, 15)
            .unwrap()
        );
        let _rug_ed_tests_llm_16_42_llm_16_42_rrrruuuugggg_test_sub_assign_duration_for_naive_date_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use super::*;
    use crate::*;
    use crate::datetime::DateTime;
    use crate::offset::FixedOffset;
    use crate::naive::NaiveDate;
    use crate::naive::NaiveDateTime;
    use crate::naive::NaiveTime;
    use crate::offset::Utc;
    use crate::offset::TimeZone;
    use crate::offset::LocalResult;
    #[test]
    fn test_day() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3600;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 15;
        let rug_fuzz_12 = 30;
        let rug_fuzz_13 = 45;
        let rug_fuzz_14 = 2023;
        let rug_fuzz_15 = 4;
        let rug_fuzz_16 = 10;
        let rug_fuzz_17 = 10;
        let rug_fuzz_18 = 30;
        let rug_fuzz_19 = 45;
        let utc_dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(utc_dt.day(), 10);
        let fixed_dt = FixedOffset::east(rug_fuzz_6 * rug_fuzz_7)
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(fixed_dt.day(), 10);
        let naive_dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16),
            NaiveTime::from_hms(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19),
        );
        debug_assert_eq!(naive_dt.day(), 10);
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day = 0;
    }
    #[test]
    #[should_panic(expected = "invalid or out-of-range datetime")]
    fn test_day_out_of_range() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_out_of_range = 0;
        let rug_fuzz_0 = 262144;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_out_of_range = 0;
    }
    #[test]
    fn test_day_with_offset() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_offset = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = "2023-04-10T10:30:45+05:00";
        let rug_fuzz_3 = "%Y-%m-%dT%H:%M:%S%z";
        let offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let dt = offset.datetime_from_str(rug_fuzz_2, rug_fuzz_3).unwrap();
        debug_assert_eq!(dt.day(), 10);
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_offset = 0;
    }
    #[test]
    fn test_day_with_offset_result() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_offset_result = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 30;
        let rug_fuzz_7 = 45;
        let offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        match offset
            .from_local_datetime(
                &NaiveDate::from_ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
                    .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7),
            )
        {
            LocalResult::Single(dt) => debug_assert_eq!(dt.day(), 10),
            _ => panic!("DateTime conversion failed"),
        }
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_offset_result = 0;
    }
    #[test]
    fn test_day_with_negative_offset() {
        let _rug_st_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_negative_offset = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 30;
        let rug_fuzz_7 = 45;
        let fixed_dt = FixedOffset::west(rug_fuzz_0 * rug_fuzz_1)
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        debug_assert_eq!(fixed_dt.day(), 9);
        let _rug_ed_tests_llm_16_43_llm_16_43_rrrruuuugggg_test_day_with_negative_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::{
        DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
    };
    #[test]
    fn test_day0() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_day0 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 30;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 5;
        let rug_fuzz_6 = 0;
        let utc_plus_0 = FixedOffset::east(rug_fuzz_0);
        let naive_date_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        let utc_date_time: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);
        let fixed_offset_date_time: DateTime<FixedOffset> = utc_date_time
            .with_timezone(&utc_plus_0);
        use crate::Datelike;
        debug_assert_eq!(fixed_offset_date_time.day0(), 29);
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_day0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    use crate::{Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc};
    #[test]
    fn test_iso_week() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 4;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.iso_week().week(), 1);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week = 0;
    }
    #[test]
    fn test_iso_week_at_year_boundary() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week_at_year_boundary = 0;
        let rug_fuzz_0 = 2021;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 27;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.iso_week().week(), 1);
        debug_assert_eq!(date.iso_week().year(), 2022);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week_at_year_boundary = 0;
    }
    #[test]
    fn test_iso_week_before_first_week() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week_before_first_week = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.iso_week().week(), 52);
        debug_assert_eq!(date.iso_week().year(), 2021);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week_before_first_week = 0;
    }
    #[test]
    fn test_iso_week_on_leap_year() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week_on_leap_year = 0;
        let rug_fuzz_0 = 2024;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.iso_week().week(), 1);
        debug_assert_eq!(date.iso_week().year(), 2025);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week_on_leap_year = 0;
    }
    #[test]
    fn test_iso_week_on_common_year() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week_on_common_year = 0;
        let rug_fuzz_0 = 2025;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let date = Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert_eq!(date.iso_week().week(), 1);
        debug_assert_eq!(date.iso_week().year(), 2026);
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week_on_common_year = 0;
    }
    #[test]
    fn test_iso_week_failure() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_iso_week_failure = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        debug_assert!(
            NaiveDate::from_ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_none()
        );
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_iso_week_failure = 0;
    }
    #[test]
    fn test_date_with_time() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_date_with_time = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 10;
        let date = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let datetime = date.and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(
            datetime, NaiveDateTime::new(date, NaiveTime::from_hms(10, 10, 10))
        );
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_date_with_time = 0;
    }
    #[test]
    fn test_date_with_time_failure() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_date_with_time_failure = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 25;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        debug_assert!(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_none()
        );
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_date_with_time_failure = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_46_llm_16_46 {
    use crate::traits::Datelike;
    use crate::FixedOffset;
    use crate::TimeZone;
    #[test]
    fn test_month() {
        let _rug_st_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_month = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2020;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let dt = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.month(), 2);
        let _rug_ed_tests_llm_16_46_llm_16_46_rrrruuuugggg_test_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use super::*;
    use crate::*;
    use crate::DateTime;
    use crate::NaiveDateTime;
    use crate::offset::Utc;
    use crate::offset::TimeZone;
    #[test]
    fn test_month0() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_month0 = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt = dt_utc.naive_utc();
        debug_assert_eq!(dt.month0(), 2);
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_month0 = 0;
    }
    #[test]
    fn test_month0_invalid_month() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_month0_invalid_month = 0;
        let rug_fuzz_0 = 1_646_404_800;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 12;
        let dt = NaiveDateTime::from_timestamp(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(dt.month0() < rug_fuzz_2, "Month0 should always be less than 12");
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_month0_invalid_month = 0;
    }
    #[test]
    fn test_month_leap_year() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_month_leap_year = 0;
        let rug_fuzz_0 = 2024;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt = dt_utc.naive_utc();
        debug_assert_eq!(dt.month0(), 1, "February of 2024 is a leap year");
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_month_leap_year = 0;
    }
    #[test]
    fn test_month0_min_value() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_month0_min_value = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt = dt_utc.naive_utc();
        debug_assert_eq!(dt.month0(), 0, "January is month0 of 0");
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_month0_min_value = 0;
    }
    #[test]
    fn test_month0_max_value() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_month0_max_value = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt = dt_utc.naive_utc();
        debug_assert_eq!(dt.month0(), 11, "December is month0 of 11");
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_month0_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;
    use crate::*;
    use crate::{
        DateTime, Datelike, Timelike, TimeZone, Utc, NaiveDate, NaiveDateTime,
        FixedOffset,
    };
    #[test]
    fn test_ordinal0() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_test_ordinal0 = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3600;
        let rug_fuzz_8 = 2020;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 2020;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 1;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 5;
        let rug_fuzz_21 = 3600;
        let rug_fuzz_22 = 2020;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 5;
        let rug_fuzz_29 = 3600;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(dt_utc.ordinal0(), 0);
        let dt_fixed: DateTime<FixedOffset> = FixedOffset::east(rug_fuzz_6 * rug_fuzz_7)
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert_eq!(dt_fixed.ordinal0(), 0);
        let nd = NaiveDate::from_ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16);
        let ndt = NaiveDateTime::new(
            nd,
            NaiveTime::from_hms(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19),
        );
        let dt_fixed_from_naive: DateTime<FixedOffset> = ndt
            .and_local_timezone(FixedOffset::east(rug_fuzz_20 * rug_fuzz_21))
            .unwrap();
        debug_assert_eq!(dt_fixed_from_naive.ordinal0(), 0);
        let ndt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_22, rug_fuzz_23, rug_fuzz_24),
            NaiveTime::from_hms(rug_fuzz_25, rug_fuzz_26, rug_fuzz_27),
        );
        let dt_fixed_from_naive: DateTime<FixedOffset> = FixedOffset::east(
                rug_fuzz_28 * rug_fuzz_29,
            )
            .from_utc_datetime(&ndt);
        debug_assert_eq!(dt_fixed_from_naive.ordinal0(), 0);
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_test_ordinal0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_51 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
    #[test]
    fn test_with_day() {
        let _rug_st_tests_llm_16_51_rrrruuuugggg_test_with_day = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3600;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 15;
        let rug_fuzz_11 = 15;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 10;
        let rug_fuzz_15 = 10;
        let rug_fuzz_16 = 32;
        let rug_fuzz_17 = 20;
        let rug_fuzz_18 = 20;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 2023;
        let rug_fuzz_21 = 5;
        let rug_fuzz_22 = 15;
        let rug_fuzz_23 = 10;
        let rug_fuzz_24 = 0;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 5;
        let rug_fuzz_27 = 5;
        let rug_fuzz_28 = 31;
        let dt_utc = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt_fixed = FixedOffset::east(rug_fuzz_6 * rug_fuzz_7)
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        debug_assert!(dt_utc.with_day(rug_fuzz_14).is_some());
        debug_assert_eq!(
            dt_utc.with_day(rug_fuzz_15).unwrap(), Utc.ymd(2023, 5, 10).and_hms(10, 0, 0)
        );
        debug_assert!(dt_utc.with_day(rug_fuzz_16).is_none());
        debug_assert!(dt_fixed.with_day(rug_fuzz_17).is_some());
        debug_assert_eq!(
            dt_fixed.with_day(rug_fuzz_18).unwrap(), FixedOffset::east(5 * 3600)
            .ymd(2023, 5, 20).and_hms(15, 0, 0)
        );
        debug_assert!(dt_fixed.with_day(rug_fuzz_19).is_none());
        let naive_dt = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_20, rug_fuzz_21, rug_fuzz_22),
            NaiveTime::from_hms(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25),
        );
        debug_assert!(naive_dt.with_day(rug_fuzz_26).is_some());
        debug_assert_eq!(
            naive_dt.with_day(rug_fuzz_27).unwrap(), NaiveDate::from_ymd(2023, 5, 5)
            .and_hms(10, 0, 0)
        );
        debug_assert!(naive_dt.with_day(rug_fuzz_28).is_none());
        let _rug_ed_tests_llm_16_51_rrrruuuugggg_test_with_day = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_53 {
    use super::*;
    use crate::*;
    use crate::{
        DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
    };
    #[test]
    fn test_with_month() {
        let _rug_st_tests_llm_16_53_rrrruuuugggg_test_with_month = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 2023;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 2;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 13;
        let rug_fuzz_22 = 2;
        let rug_fuzz_23 = 0;
        let rug_fuzz_24 = 13;
        let rug_fuzz_25 = 2;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 13;
        let utc = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed = FixedOffset::east(rug_fuzz_6)
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        let local = Local
            .ymd(rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .and_hms(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18);
        debug_assert_eq!(
            utc.with_month(rug_fuzz_19), Some(Utc.ymd(2023, 2, 1).and_hms(0, 0, 0))
        );
        debug_assert_eq!(utc.with_month(rug_fuzz_20), None);
        debug_assert_eq!(utc.with_month(rug_fuzz_21), None);
        debug_assert_eq!(
            fixed.with_month(rug_fuzz_22), Some(FixedOffset::east(0).ymd(2023, 2, 1)
            .and_hms(0, 0, 0))
        );
        debug_assert_eq!(fixed.with_month(rug_fuzz_23), None);
        debug_assert_eq!(fixed.with_month(rug_fuzz_24), None);
        if let Some(local_with_month) = local.with_month(rug_fuzz_25) {
            debug_assert_eq!(local_with_month, Local.ymd(2023, 2, 1).and_hms(0, 0, 0));
        }
        debug_assert_eq!(local.with_month(rug_fuzz_26), None);
        debug_assert_eq!(local.with_month(rug_fuzz_27), None);
        let _rug_ed_tests_llm_16_53_rrrruuuugggg_test_with_month = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_54_llm_16_54 {
    use super::*;
    use crate::*;
    use crate::{NaiveDate, Utc};
    #[test]
    fn test_with_month0() {
        let _rug_st_tests_llm_16_54_llm_16_54_rrrruuuugggg_test_with_month0 = 0;
        let rug_fuzz_0 = 2022;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2022;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 31;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 2020;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 29;
        let rug_fuzz_17 = 12;
        let rug_fuzz_18 = 0;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 1;
        let rug_fuzz_21 = 2022;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 1;
        let rug_fuzz_24 = 12;
        let rug_fuzz_25 = 0;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 11;
        let rug_fuzz_28 = 2022;
        let rug_fuzz_29 = 2;
        let rug_fuzz_30 = 2;
        let rug_fuzz_31 = 12;
        let rug_fuzz_32 = 0;
        let rug_fuzz_33 = 0;
        let rug_fuzz_34 = 0;
        let rug_fuzz_35 = 2022;
        let rug_fuzz_36 = 1;
        let rug_fuzz_37 = 2;
        let rug_fuzz_38 = 12;
        let rug_fuzz_39 = 0;
        let rug_fuzz_40 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt_with_month0 = dt.with_month0(rug_fuzz_6);
        debug_assert_eq!(dt_with_month0, Some(Utc.ymd(2022, 1, 2).and_hms(12, 0, 0)));
        let dt = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        let dt_with_month0 = dt.with_month0(rug_fuzz_13);
        debug_assert_eq!(dt_with_month0, None);
        let dt = Utc
            .ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16)
            .and_hms(rug_fuzz_17, rug_fuzz_18, rug_fuzz_19);
        let dt_with_month0 = dt.with_month0(rug_fuzz_20);
        debug_assert_eq!(dt_with_month0, Some(Utc.ymd(2020, 3, 29).and_hms(12, 0, 0)));
        let dt = Utc
            .ymd(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .and_hms(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26);
        let dt_with_month0 = dt.with_month0(rug_fuzz_27);
        debug_assert_eq!(dt_with_month0, Some(Utc.ymd(2022, 12, 1).and_hms(12, 0, 0)));
        let dt = Local
            .ymd(rug_fuzz_28, rug_fuzz_29, rug_fuzz_30)
            .and_hms(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33);
        let dt_with_month0 = dt.with_month0(rug_fuzz_34);
        let expected_dt_with_month0 = Local
            .ymd(rug_fuzz_35, rug_fuzz_36, rug_fuzz_37)
            .and_hms(rug_fuzz_38, rug_fuzz_39, rug_fuzz_40);
        debug_assert_eq!(dt_with_month0, Some(expected_dt_with_month0));
        let _rug_ed_tests_llm_16_54_llm_16_54_rrrruuuugggg_test_with_month0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;
    use crate::*;
    #[test]
    fn test_with_ordinal() {
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_with_ordinal = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 60;
        let rug_fuzz_7 = 400;
        let rug_fuzz_8 = 365;
        let tz = Local;
        let dt = tz
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let valid_ordinal = rug_fuzz_6;
        let invalid_ordinal = rug_fuzz_7;
        let post_valid_ordinal = rug_fuzz_8;
        let valid_result = dt.with_ordinal(valid_ordinal);
        debug_assert!(valid_result.is_some());
        debug_assert_eq!(valid_result.unwrap().ordinal(), valid_ordinal);
        let invalid_result = dt.with_ordinal(invalid_ordinal);
        debug_assert!(invalid_result.is_none());
        let post_valid_result = dt.with_ordinal(post_valid_ordinal);
        debug_assert!(post_valid_result.is_some());
        debug_assert_eq!(post_valid_result.unwrap().ordinal(), post_valid_ordinal);
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_with_ordinal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, LocalResult, TimeZone};
    #[test]
    fn test_with_ordinal0() {
        let _rug_st_tests_llm_16_56_rrrruuuugggg_test_with_ordinal0 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 31;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 365;
        let rug_fuzz_14 = 1;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 0;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0;
        let tz = FixedOffset::east(rug_fuzz_0);
        let ordinal0 = rug_fuzz_1;
        let year = rug_fuzz_2;
        let expected_date = tz
            .ymd(year, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let res = tz
            .with_ymd_and_hms(
                year,
                rug_fuzz_8,
                ordinal0 + rug_fuzz_9,
                rug_fuzz_10,
                rug_fuzz_11,
                rug_fuzz_12,
            );
        debug_assert_eq!(res, LocalResult::Single(expected_date));
        let invalid_ordinal0 = rug_fuzz_13;
        let res = tz
            .with_ymd_and_hms(
                year,
                rug_fuzz_14,
                invalid_ordinal0 + rug_fuzz_15,
                rug_fuzz_16,
                rug_fuzz_17,
                rug_fuzz_18,
            );
        debug_assert_eq!(res, LocalResult::None);
        let _rug_ed_tests_llm_16_56_rrrruuuugggg_test_with_ordinal0 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_58 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc, FixedOffset};
    use crate::offset::LocalResult;
    /// Returns a `DateTime` object representing "2015-09-05T23:56:04 UTC"
    fn example_datetime() -> DateTime<Utc> {
        Utc.ymd(2015, 9, 5).and_hms(23, 56, 4)
    }
    /// Returns a `FixedOffset` object representing UTC+1
    fn example_offset() -> FixedOffset {
        FixedOffset::east(3600)
    }
    #[test]
    fn test_year_with_utc() {
        let dt = example_datetime();
        assert_eq!(dt.year(), 2015);
    }
    #[test]
    fn test_year_with_fixed_offset() {
        let dt = example_datetime().with_timezone(&example_offset());
        assert_eq!(dt.year(), 2015);
    }
    #[test]
    fn test_year_with_utc_opt() {
        let dt = Utc.ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
        assert_eq!(dt.year(), 2015);
    }
    #[test]
    fn test_year_with_fixed_offset_opt() {
        let dt = Utc.ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap();
        let dt_offset = dt.with_timezone(&example_offset());
        assert_eq!(dt_offset.year(), 2015);
    }
    #[test]
    fn test_year_with_timestamp() {
        let dt = Utc.timestamp(1441493764, 0);
        assert_eq!(dt.year(), 2015);
    }
    #[test]
    fn test_year_with_timestamp_opt() {
        let local_result = Utc.timestamp_opt(1441493764, 0);
        match local_result {
            LocalResult::Single(dt) => assert_eq!(dt.year(), 2015),
            _ => panic!("Timestamp opt should yield a valid result"),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_61 {
    use super::*;
    use crate::*;
    use crate::offset::{FixedOffset, TimeZone};
    use crate::naive::{NaiveDate, NaiveTime};
    #[test]
    fn test_nanosecond() {
        let _rug_st_tests_llm_16_61_rrrruuuugggg_test_nanosecond = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 30;
        let rug_fuzz_6 = 45;
        let rug_fuzz_7 = 123_456_789;
        let fixed_offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let naive_date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap();
        let naive_time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
            )
            .unwrap();
        let datetime = naive_date.and_time(naive_time);
        let datetime_with_offset: DateTime<FixedOffset> = fixed_offset
            .from_local_datetime(&datetime)
            .unwrap();
        debug_assert_eq!(datetime_with_offset.nanosecond(), 123_456_789);
        let _rug_ed_tests_llm_16_61_rrrruuuugggg_test_nanosecond = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_63 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc};
    #[test]
    fn test_with_hour() {
        let _rug_st_tests_llm_16_63_rrrruuuugggg_test_with_hour = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 24;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 0;
        let tz: Utc = Utc;
        let initial: DateTime<Utc> = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let with_hour = initial.with_hour(rug_fuzz_6).unwrap();
        debug_assert_eq!(with_hour.hour(), 10);
        debug_assert_eq!(with_hour.minute(), initial.minute());
        debug_assert_eq!(with_hour.second(), initial.second());
        debug_assert_eq!(with_hour.year(), initial.year());
        debug_assert_eq!(with_hour.month(), initial.month());
        debug_assert_eq!(with_hour.day(), initial.day());
        debug_assert!(initial.with_hour(rug_fuzz_7).is_none());
        let before_midnight: DateTime<Utc> = tz
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        let next_day = before_midnight.with_hour(rug_fuzz_14).unwrap();
        debug_assert_eq!(next_day.hour(), 0);
        debug_assert_eq!(next_day.day(), before_midnight.day() + 1);
        let _rug_ed_tests_llm_16_63_rrrruuuugggg_test_with_hour = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use crate::offset::FixedOffset;
    use crate::naive::NaiveTime;
    use crate::TimeZone;
    use crate::Timelike;
    use crate::traits::Datelike;
    use crate::naive::{NaiveDate, NaiveDateTime};
    #[test]
    fn test_with_minute() {
        let _rug_st_tests_llm_16_64_rrrruuuugggg_test_with_minute = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 3600;
        let rug_fuzz_3 = 86399;
        let rug_fuzz_4 = 86399;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = 34;
        let rug_fuzz_7 = 56;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 5;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
        let fixed_offset_0 = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let fixed_offset_plus_1 = FixedOffset::east_opt(rug_fuzz_1).unwrap();
        let fixed_offset_minus_1 = FixedOffset::west_opt(rug_fuzz_2).unwrap();
        let fixed_offset_max = FixedOffset::east_opt(rug_fuzz_3).unwrap();
        let fixed_offset_min = FixedOffset::west_opt(rug_fuzz_4).unwrap();
        let time = NaiveTime::from_hms_opt(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7).unwrap();
        let date = NaiveDate::from_ymd_opt(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10).unwrap();
        let naive_datetime = NaiveDateTime::new(date, time);
        let datetime = fixed_offset_0
            .from_local_datetime(&naive_datetime.with_minute(rug_fuzz_11).unwrap())
            .unwrap();
        debug_assert_eq!(datetime.minute(), 0);
        let datetime = fixed_offset_plus_1
            .from_local_datetime(&naive_datetime.with_minute(rug_fuzz_12).unwrap())
            .unwrap();
        debug_assert_eq!(datetime.minute(), 0);
        let datetime = fixed_offset_minus_1
            .from_local_datetime(&naive_datetime.with_minute(rug_fuzz_13).unwrap())
            .unwrap();
        debug_assert_eq!(datetime.minute(), 0);
        let datetime = fixed_offset_max
            .from_local_datetime(&naive_datetime.with_minute(rug_fuzz_14).unwrap())
            .unwrap();
        debug_assert_eq!(datetime.minute(), 0);
        let datetime = fixed_offset_min
            .from_local_datetime(&naive_datetime.with_minute(rug_fuzz_15).unwrap())
            .unwrap();
        debug_assert_eq!(datetime.minute(), 0);
        let _rug_ed_tests_llm_16_64_rrrruuuugggg_test_with_minute = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    use crate::prelude::*;
    #[test]
    fn test_with_nanosecond() {
        let _rug_st_tests_llm_16_65_rrrruuuugggg_test_with_nanosecond = 0;
        let rug_fuzz_0 = 2000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1_000;
        let rug_fuzz_8 = 999_999_999;
        let rug_fuzz_9 = 1_000_000_000;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.nanosecond(), 0);
        let with_nano = dt.with_nanosecond(rug_fuzz_7).unwrap();
        debug_assert_eq!(with_nano.nanosecond(), 1_000);
        let with_nano = dt.with_nanosecond(rug_fuzz_8).unwrap();
        debug_assert_eq!(with_nano.nanosecond(), 999_999_999);
        let with_nano = dt.with_nanosecond(rug_fuzz_9);
        debug_assert!(with_nano.is_none());
        let _rug_ed_tests_llm_16_65_rrrruuuugggg_test_with_nanosecond = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_66 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, TimeZone};
    #[test]
    fn test_with_second() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_with_second = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 30;
        let rug_fuzz_7 = 45;
        let rug_fuzz_8 = 60;
        let tz = FixedOffset::east(rug_fuzz_0);
        let datetime = tz
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.with_second(rug_fuzz_7).unwrap().second(), 45);
        debug_assert!(datetime.with_second(rug_fuzz_8).is_none());
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_with_second = 0;
    }
    #[test]
    fn test_with_second_leap_second() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_with_second_leap_second = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 60;
        let tz = FixedOffset::east(rug_fuzz_0);
        let leap_second = tz
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(leap_second.with_second(rug_fuzz_7).unwrap().second(), 60);
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_with_second_leap_second = 0;
    }
    #[test]
    fn test_with_second_invalid_seconds() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_with_second_invalid_seconds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 30;
        let rug_fuzz_7 = 61;
        let tz = FixedOffset::east(rug_fuzz_0);
        let datetime = tz
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert!(datetime.with_second(rug_fuzz_7).is_none());
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_with_second_invalid_seconds = 0;
    }
    #[test]
    fn test_with_second_edge_case() {
        let _rug_st_tests_llm_16_66_rrrruuuugggg_test_with_second_edge_case = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let tz = FixedOffset::east(rug_fuzz_0);
        let datetime = tz
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.with_second(rug_fuzz_7).unwrap().second(), 0);
        debug_assert_eq!(datetime.with_second(rug_fuzz_8).unwrap().minute(), 0);
        debug_assert_eq!(datetime.with_second(rug_fuzz_9).unwrap().hour(), 0);
        let _rug_ed_tests_llm_16_66_rrrruuuugggg_test_with_second_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_68 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, NaiveDateTime, TimeZone};
    #[test]
    fn test_from_utc() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_from_utc = 0;
        let rug_fuzz_0 = 1_609_439_600;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 3600;
        let utc_dt = NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1).unwrap();
        let fixed_offset = FixedOffset::east_opt(rug_fuzz_2).unwrap();
        let fixed_dt = fixed_offset.from_utc_datetime(&utc_dt);
        debug_assert_eq!(fixed_dt, fixed_offset.ymd(2020, 12, 12).and_hms(1, 0, 0));
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_from_utc = 0;
    }
    #[test]
    fn test_offset_from_utc() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_offset_from_utc = 0;
        let rug_fuzz_0 = 1_609_439_600;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 3600;
        let utc_dt = NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1).unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_2).unwrap();
        debug_assert_eq!(offset.offset_from_utc_datetime(& utc_dt), offset);
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_offset_from_utc = 0;
    }
    #[test]
    fn test_from_offset() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_from_offset = 0;
        let rug_fuzz_0 = 3600;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        debug_assert_eq!(FixedOffset::from_offset(& offset), offset);
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_from_offset = 0;
    }
    #[test]
    fn test_offset_from_local() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_offset_from_local = 0;
        let rug_fuzz_0 = 1_609_443_200;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 3600;
        let local_dt = NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let offset = FixedOffset::east_opt(rug_fuzz_2).unwrap();
        let local_result = offset.offset_from_local_datetime(&local_dt);
        debug_assert_eq!(local_result, LocalResult::Single(offset));
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_offset_from_local = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    use crate::DateTime;
    use crate::Utc;
    #[test]
    fn test_default_for_datetime_fixedoffset() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_default_for_datetime_fixedoffset = 0;
        let default_datetime: DateTime<FixedOffset> = DateTime::default();
        debug_assert_eq!(
            default_datetime, FixedOffset::west_opt(0).unwrap().from_utc_datetime(&
            NaiveDateTime::default())
        );
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_default_for_datetime_fixedoffset = 0;
    }
    #[test]
    fn test_default_for_datetime_utc() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_default_for_datetime_utc = 0;
        let default_datetime: DateTime<Utc> = DateTime::default();
        debug_assert_eq!(
            default_datetime, Utc.from_utc_datetime(& NaiveDateTime::default())
        );
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_default_for_datetime_utc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_70_llm_16_70 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, Local, NaiveDateTime, Utc, TimeZone};
    #[test]
    fn test_from_fixed_offset_to_local() {
        let _rug_st_tests_llm_16_70_llm_16_70_rrrruuuugggg_test_from_fixed_offset_to_local = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let fixed_time: DateTime<FixedOffset> = FixedOffset::east(
                rug_fuzz_0 * rug_fuzz_1,
            )
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let local_time: DateTime<Local> = DateTime::from(fixed_time);
        let naive_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10),
            NaiveTime::from_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13),
        );
        let fixed_timestamp = fixed_time.timestamp();
        let local_timestamp = local_time.timestamp();
        let naive_timestamp = naive_time.timestamp();
        let utc_time = Utc.from_utc_datetime(&naive_time);
        let utc_timestamp = utc_time.timestamp();
        debug_assert_eq!(fixed_timestamp, local_timestamp);
        debug_assert_eq!(fixed_timestamp, naive_timestamp);
        debug_assert_eq!(utc_timestamp, naive_timestamp);
        let _rug_ed_tests_llm_16_70_llm_16_70_rrrruuuugggg_test_from_fixed_offset_to_local = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_conversion_to_local_from_utc() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_conversion_to_local_from_utc = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let utc_datetime = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_datetime = DateTime::<Local>::from(utc_datetime);
        debug_assert_eq!(local_datetime, utc_datetime.with_timezone(& Local));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_conversion_to_local_from_utc = 0;
    }
    #[test]
    fn test_conversion_from_utc_with_fixed_offset() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_with_fixed_offset = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 5;
        let rug_fuzz_7 = 3600;
        let utc_datetime = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed_offset = FixedOffset::east(rug_fuzz_6 * rug_fuzz_7);
        let fixed_datetime = DateTime::<FixedOffset>::from(utc_datetime);
        debug_assert_eq!(fixed_datetime, utc_datetime.with_timezone(& fixed_offset));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_with_fixed_offset = 0;
    }
    #[test]
    fn test_conversion_from_utc_at_midnight() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_at_midnight = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let utc_datetime = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_datetime = DateTime::<Local>::from(utc_datetime);
        debug_assert_eq!(local_datetime, utc_datetime.with_timezone(& Local));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_at_midnight = 0;
    }
    #[test]
    fn test_conversion_from_utc_with_negative_offset() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_with_negative_offset = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 3600;
        let utc_datetime = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed_offset = FixedOffset::west(rug_fuzz_6 * rug_fuzz_7);
        let fixed_datetime = DateTime::<FixedOffset>::from(utc_datetime);
        debug_assert_eq!(fixed_datetime, utc_datetime.with_timezone(& fixed_offset));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_conversion_from_utc_with_negative_offset = 0;
    }
    #[test]
    fn test_conversion_during_dst_transition() {
        let _rug_st_tests_llm_16_71_rrrruuuugggg_test_conversion_during_dst_transition = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 29;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let utc_datetime = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let local_datetime = DateTime::<Local>::from(utc_datetime);
        debug_assert_eq!(local_datetime, utc_datetime.with_timezone(& Local));
        let _rug_ed_tests_llm_16_71_rrrruuuugggg_test_conversion_during_dst_transition = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;
    use crate::*;
    use crate::prelude::*;
    use crate::DateTime;
    #[test]
    fn test_systemtime_to_datetime_local() {
        let _rug_st_tests_llm_16_72_rrrruuuugggg_test_systemtime_to_datetime_local = 0;
        let rug_fuzz_0 = 1_000_000;
        let system_time = std::time::SystemTime::now();
        let datetime_local: DateTime<Local> = system_time.into();
        let system_time_converted: std::time::SystemTime = datetime_local.into();
        let datetime_local_converted: DateTime<Local> = system_time_converted.into();
        let duration_difference = datetime_local
            .signed_duration_since(datetime_local_converted)
            .num_nanoseconds()
            .unwrap();
        debug_assert!(duration_difference.abs() < rug_fuzz_0);
        let _rug_ed_tests_llm_16_72_rrrruuuugggg_test_systemtime_to_datetime_local = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc};
    #[test]
    fn test_default_date_time_local() {
        let _rug_st_tests_llm_16_73_rrrruuuugggg_test_default_date_time_local = 0;
        let default_dt: DateTime<Local> = Default::default();
        debug_assert_eq!(default_dt, Local::now());
        let _rug_ed_tests_llm_16_73_rrrruuuugggg_test_default_date_time_local = 0;
    }
    #[test]
    fn test_default_date_time_utc() {
        let _rug_st_tests_llm_16_73_rrrruuuugggg_test_default_date_time_utc = 0;
        let default_dt: DateTime<Utc> = Default::default();
        debug_assert_eq!(default_dt, Utc::now());
        let _rug_ed_tests_llm_16_73_rrrruuuugggg_test_default_date_time_utc = 0;
    }
    #[test]
    fn test_fixed_offset() {
        let _rug_st_tests_llm_16_73_rrrruuuugggg_test_fixed_offset = 0;
        let rug_fuzz_0 = 3600;
        let offset = FixedOffset::east(rug_fuzz_0);
        debug_assert_eq!(offset.local_minus_utc(), 3600);
        let _rug_ed_tests_llm_16_73_rrrruuuugggg_test_fixed_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_74 {
    use crate::{DateTime, Local, ParseResult, TimeZone};
    use std::str::FromStr;
    #[test]
    fn test_from_str_valid_input() {
        let _rug_st_tests_llm_16_74_rrrruuuugggg_test_from_str_valid_input = 0;
        let rug_fuzz_0 = "2020-01-01T12:00:00+00:00";
        let input = rug_fuzz_0;
        let parsed: ParseResult<DateTime<Local>> = <DateTime<
            Local,
        > as FromStr>::from_str(input);
        debug_assert!(parsed.is_ok());
        let parsed_date = parsed.unwrap();
        debug_assert_eq!(parsed_date, Local.timestamp(1577880000, 0));
        let _rug_ed_tests_llm_16_74_rrrruuuugggg_test_from_str_valid_input = 0;
    }
    #[test]
    fn test_from_str_invalid_input() {
        let _rug_st_tests_llm_16_74_rrrruuuugggg_test_from_str_invalid_input = 0;
        let rug_fuzz_0 = "not a valid datetime";
        let input = rug_fuzz_0;
        let parsed: ParseResult<DateTime<Local>> = <DateTime<
            Local,
        > as FromStr>::from_str(input);
        debug_assert!(parsed.is_err());
        let _rug_ed_tests_llm_16_74_rrrruuuugggg_test_from_str_invalid_input = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_75 {
    use super::*;
    use crate::*;
    use crate::{DateTime, NaiveDateTime, NaiveDate, FixedOffset};
    #[test]
    fn test_datetime_from_utc() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_datetime_from_utc = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 3600;
        let utc_dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed_dt: DateTime<FixedOffset> = utc_dt
            .with_timezone(&FixedOffset::east(rug_fuzz_6));
        debug_assert_eq!(
            fixed_dt, FixedOffset::east(3600).ymd(2023, 4, 1).and_hms(4, 30, 45)
        );
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_datetime_from_utc = 0;
    }
    #[test]
    fn test_datetime_from_local() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_datetime_from_local = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 3600;
        let local = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let fixed_dt: DateTime<FixedOffset> = FixedOffset::east(rug_fuzz_6)
            .from_local_datetime(&local)
            .unwrap();
        debug_assert_eq!(
            fixed_dt, FixedOffset::east(3600).ymd(2023, 4, 1).and_hms(4, 30, 45)
        );
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_datetime_from_local = 0;
    }
    #[test]
    #[should_panic]
    fn test_invalid_datetime_from_utc() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_invalid_datetime_from_utc = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 3600;
        let invalid_dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let _ = invalid_dt.with_timezone(&FixedOffset::east(rug_fuzz_6));
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_invalid_datetime_from_utc = 0;
    }
    #[test]
    #[should_panic]
    fn test_invalid_datetime_from_local() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_invalid_datetime_from_local = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 3600;
        let local = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let _fixed_dt: DateTime<FixedOffset> = FixedOffset::east(rug_fuzz_6)
            .from_local_datetime(&local)
            .unwrap();
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_invalid_datetime_from_local = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_77 {
    use super::*;
    use crate::*;
    use crate::TimeZone;
    #[test]
    fn test_system_time_to_utc_datetime() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime = 0;
        let sys_time = SystemTime::UNIX_EPOCH;
        let datetime_utc: DateTime<Utc> = From::from(sys_time);
        debug_assert_eq!(datetime_utc, Utc.timestamp(0, 0));
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime = 0;
    }
    #[test]
    fn test_system_time_from_utc_datetime() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let datetime_utc = Utc.timestamp(rug_fuzz_0, rug_fuzz_1);
        let sys_time: SystemTime = From::from(datetime_utc);
        debug_assert_eq!(sys_time, SystemTime::UNIX_EPOCH);
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime = 0;
    }
    #[test]
    fn test_system_time_to_utc_datetime_after_epoch() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime_after_epoch = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let sys_time = SystemTime::UNIX_EPOCH
            + std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let datetime_utc: DateTime<Utc> = From::from(sys_time);
        debug_assert_eq!(datetime_utc, Utc.timestamp(1, 0));
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime_after_epoch = 0;
    }
    #[test]
    fn test_system_time_from_utc_datetime_after_epoch() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime_after_epoch = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let datetime_utc = Utc.timestamp(rug_fuzz_0, rug_fuzz_1);
        let sys_time: SystemTime = From::from(datetime_utc);
        debug_assert_eq!(
            sys_time, SystemTime::UNIX_EPOCH + std::time::Duration::new(1, 0)
        );
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime_after_epoch = 0;
    }
    #[test]
    fn test_system_time_to_utc_datetime_before_epoch() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime_before_epoch = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let sys_time = SystemTime::UNIX_EPOCH
            - std::time::Duration::new(rug_fuzz_0, rug_fuzz_1);
        let datetime_utc: DateTime<Utc> = From::from(sys_time);
        debug_assert_eq!(datetime_utc, Utc.timestamp(- 1, 0));
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_to_utc_datetime_before_epoch = 0;
    }
    #[test]
    fn test_system_time_from_utc_datetime_before_epoch() {
        let _rug_st_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime_before_epoch = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let datetime_utc = Utc.timestamp(-rug_fuzz_0, rug_fuzz_1);
        let sys_time: SystemTime = From::from(datetime_utc);
        debug_assert_eq!(
            sys_time, SystemTime::UNIX_EPOCH - std::time::Duration::new(1, 0)
        );
        let _rug_ed_tests_llm_16_77_rrrruuuugggg_test_system_time_from_utc_datetime_before_epoch = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;
    use crate::*;
    use crate::{TimeZone, Utc, NaiveDate};
    #[test]
    fn test_default_naive_date_time_in_utc() {
        let _rug_st_tests_llm_16_78_rrrruuuugggg_test_default_naive_date_time_in_utc = 0;
        let rug_fuzz_0 = 1970;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let naive_utc: crate::NaiveDateTime = crate::NaiveDateTime::default();
        let expected = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .naive_utc();
        debug_assert_eq!(naive_utc, expected);
        let _rug_ed_tests_llm_16_78_rrrruuuugggg_test_default_naive_date_time_in_utc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use crate::{DateTime, Utc, TimeZone, ParseResult};
    use std::str::FromStr;
    #[test]
    fn test_valid_datetime_from_str() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_valid_datetime_from_str = 0;
        let rug_fuzz_0 = "2023-03-26T12:39:45Z";
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 26;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 39;
        let rug_fuzz_6 = 45;
        let input = rug_fuzz_0;
        let expected = Utc
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let result: ParseResult<DateTime<Utc>> = DateTime::from_str(input);
        debug_assert!(result.is_ok());
        let datetime = result.unwrap();
        debug_assert_eq!(datetime, expected);
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_valid_datetime_from_str = 0;
    }
    #[test]
    fn test_invalid_datetime_from_str() {
        let _rug_st_tests_llm_16_79_rrrruuuugggg_test_invalid_datetime_from_str = 0;
        let rug_fuzz_0 = "invalid-datetime";
        let input = rug_fuzz_0;
        let result: ParseResult<DateTime<Utc>> = DateTime::from_str(input);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_79_rrrruuuugggg_test_invalid_datetime_from_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_237 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, NaiveDate, NaiveTime, TimeZone, Utc};
    #[test]
    fn test_checked_add_days() {
        let _rug_st_tests_llm_16_237_rrrruuuugggg_test_checked_add_days = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2014;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 2014;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 1;
        let rug_fuzz_16 = 2014;
        let rug_fuzz_17 = 1;
        let rug_fuzz_18 = 1;
        let rug_fuzz_19 = 0;
        let rug_fuzz_20 = 0;
        let rug_fuzz_21 = 0;
        let rug_fuzz_22 = 1;
        let rug_fuzz_23 = 2014;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 1;
        let rug_fuzz_26 = 0;
        let rug_fuzz_27 = 0;
        let rug_fuzz_28 = 0;
        let rug_fuzz_29 = 1;
        let fixed_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let naive_datetime = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let datetime: DateTime<FixedOffset> = DateTime::from_utc(
            naive_datetime.naive_utc(),
            fixed_offset,
        );
        debug_assert_eq!(
            datetime.checked_add_days(Days::new(rug_fuzz_8)), Some(fixed_offset.ymd(2014,
            1, 2).and_hms(0, 0, 0))
        );
        let naive_datetime = Utc
            .ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .and_hms(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        let datetime: DateTime<Utc> = DateTime::from_utc(
            naive_datetime.naive_utc(),
            Utc,
        );
        debug_assert_eq!(
            datetime.checked_add_days(Days::new(rug_fuzz_15)), Some(Utc.ymd(2014, 1, 2)
            .and_hms(0, 0, 0))
        );
        let naive_datetime = NaiveDate::from_ymd(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .and_hms(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21);
        debug_assert_eq!(
            naive_datetime.checked_add_days(Days::new(rug_fuzz_22)),
            Some(NaiveDate::from_ymd(2014, 1, 2).and_hms(0, 0, 0))
        );
        let naive_datetime = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_23, rug_fuzz_24, rug_fuzz_25),
            NaiveTime::from_hms(rug_fuzz_26, rug_fuzz_27, rug_fuzz_28),
        );
        debug_assert_eq!(
            naive_datetime.checked_add_days(Days::new(rug_fuzz_29)),
            Some(NaiveDate::from_ymd(2014, 1, 2).and_hms(0, 0, 0))
        );
        let _rug_ed_tests_llm_16_237_rrrruuuugggg_test_checked_add_days = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_239_llm_16_239 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, Utc};
    use crate::naive::{NaiveDate, NaiveTime};
    #[test]
    fn test_checked_add_signed_duration() {
        let _rug_st_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_duration = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 4;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 35;
        let rug_fuzz_12 = 06;
        let initial_date_time = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        let expected_date_time = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(
            initial_date_time.checked_add_signed(duration), Some(expected_date_time)
        );
        let _rug_ed_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_duration = 0;
    }
    #[test]
    fn test_checked_add_signed_negative_duration() {
        let _rug_st_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_negative_duration = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 4;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 34;
        let rug_fuzz_12 = 46;
        let initial_date_time = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        let expected_date_time = Utc
            .ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        debug_assert_eq!(
            initial_date_time.checked_add_signed(duration), Some(expected_date_time)
        );
        let _rug_ed_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_negative_duration = 0;
    }
    #[test]
    fn test_checked_add_signed_overflow() {
        let _rug_st_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_overflow = 0;
        let rug_fuzz_0 = 9999;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 2;
        let initial_date_time = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(rug_fuzz_6);
        debug_assert_eq!(initial_date_time.checked_add_signed(duration), None);
        let _rug_ed_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_overflow = 0;
    }
    #[test]
    fn test_checked_add_signed_underflow() {
        let _rug_st_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_underflow = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2;
        let initial_date_time = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        debug_assert_eq!(initial_date_time.checked_add_signed(duration), None);
        let _rug_ed_tests_llm_16_239_llm_16_239_rrrruuuugggg_test_checked_add_signed_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_240 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
    #[test]
    fn test_checked_sub_days_utc() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_utc = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 5;
        let dt = Utc
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_6));
        debug_assert_eq!(
            result, Some(Utc.ymd_opt(2023, 1, 5).unwrap().and_hms_opt(0, 0, 0).unwrap())
        );
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_utc = 0;
    }
    #[test]
    fn test_checked_sub_days_fixed_offset() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_fixed_offset = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 10;
        let dt = FixedOffset::east_opt(rug_fuzz_0 * rug_fuzz_1)
            .unwrap()
            .ymd_opt(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .unwrap()
            .and_hms_opt(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_8));
        debug_assert_eq!(
            result, Some(FixedOffset::east_opt(5 * 3600).unwrap().ymd_opt(2023, 1, 1)
            .unwrap().and_hms_opt(0, 0, 0).unwrap())
        );
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_fixed_offset = 0;
    }
    #[test]
    fn test_checked_sub_days_local() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_local = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 15;
        let dt = Local
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_6));
        debug_assert_eq!(
            result, Some(Local.ymd_opt(2022, 12, 26).unwrap().and_hms_opt(0, 0, 0)
            .unwrap())
        );
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_local = 0;
    }
    #[test]
    fn test_checked_sub_days_invalid() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_invalid = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2;
        let dt = Utc
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_6));
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_invalid = 0;
    }
    #[test]
    fn test_checked_sub_days_with_leap_year() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_with_leap_year = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 365;
        let dt = Utc
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_6));
        debug_assert_eq!(
            result, Some(Utc.ymd_opt(2019, 3, 1).unwrap().and_hms_opt(0, 0, 0).unwrap())
        );
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_with_leap_year = 0;
    }
    #[test]
    fn test_checked_sub_days_with_time() {
        let _rug_st_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_with_time = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 1;
        let dt = Utc
            .ymd_opt(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let result = dt.checked_sub_days(Days::new(rug_fuzz_6));
        debug_assert_eq!(
            result, Some(Utc.ymd_opt(2023, 1, 9).unwrap().and_hms_opt(12, 30, 45)
            .unwrap())
        );
        let _rug_ed_tests_llm_16_240_rrrruuuugggg_test_checked_sub_days_with_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_242 {
    use super::*;
    use crate::*;
    use crate::{DateTime, TimeZone, Utc};
    #[test]
    fn test_checked_sub_signed() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2023;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 30;
        let rug_fuzz_12 = 30;
        let dt1: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt2: DateTime<Utc> = Utc
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        let duration = TimeDelta::seconds(rug_fuzz_12);
        let result = dt1.checked_sub_signed(duration);
        debug_assert_eq!(result, Some(dt2));
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed = 0;
    }
    #[test]
    fn test_checked_sub_signed_overflow() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_overflow = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(i64::MAX);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_overflow = 0;
    }
    #[test]
    fn test_checked_sub_signed_leap_second() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_leap_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 1_500_000_000;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 31;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 500_000_000;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let duration = TimeDelta::seconds(rug_fuzz_7);
        let expected: DateTime<Utc> = Utc
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms_nano(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, Some(expected));
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_leap_second = 0;
    }
    #[test]
    fn test_checked_sub_signed_negative() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_negative = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 30;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let duration = TimeDelta::seconds(-rug_fuzz_6);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_negative = 0;
    }
    #[test]
    fn test_checked_sub_signed_adjust_leap_second() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_adjust_leap_second = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 60;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 31;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 59;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let duration = TimeDelta::seconds(rug_fuzz_7);
        let expected: DateTime<Utc> = Utc
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, Some(expected));
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_adjust_leap_second = 0;
    }
    #[test]
    fn test_checked_sub_signed_leap_second_boundary() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_leap_second_boundary = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 6;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 60;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 6;
        let rug_fuzz_10 = 30;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 59;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let duration = TimeDelta::seconds(rug_fuzz_7);
        let expected: DateTime<Utc> = Utc
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, Some(expected));
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_leap_second_boundary = 0;
    }
    #[test]
    fn test_checked_sub_signed_multiple_leap_seconds() {
        let _rug_st_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_multiple_leap_seconds = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 60;
        let rug_fuzz_6 = 500_000_000;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 31;
        let rug_fuzz_11 = 23;
        let rug_fuzz_12 = 59;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 500_000_000;
        let dt: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        let duration = TimeDelta::seconds(rug_fuzz_7);
        let expected: DateTime<Utc> = Utc
            .ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10)
            .and_hms_nano(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13, rug_fuzz_14);
        let result = dt.checked_sub_signed(duration);
        debug_assert_eq!(result, Some(expected));
        let _rug_ed_tests_llm_16_242_rrrruuuugggg_test_checked_sub_signed_multiple_leap_seconds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_243 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_fixed_offset_date() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_fixed_offset_date = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let date = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date, fixed_offset.ymd(2023, 4, 10).and_hms(10, 0, 0));
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_fixed_offset_date = 0;
    }
    #[test]
    fn test_date_time_conversion() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_date_time_conversion = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 4;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let naive_date_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3),
            NaiveTime::from_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6),
        );
        let date_time: DateTime<FixedOffset> = fixed_offset
            .from_utc_datetime(&naive_date_time);
        debug_assert_eq!(date_time, fixed_offset.from_utc_datetime(& naive_date_time));
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_date_time_conversion = 0;
    }
    #[test]
    fn test_date_time_timestamp() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_date_time_timestamp = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 3600;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let dt_fixed: DateTime<FixedOffset> = dt_utc
            .with_timezone(&FixedOffset::east(rug_fuzz_6));
        debug_assert_eq!(dt_fixed.timestamp(), dt_utc.timestamp());
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_date_time_timestamp = 0;
    }
    #[test]
    fn test_date_time_formatting() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_date_time_formatting = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = "%Y-%m-%d %H:%M:%S";
        let date_time: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let formatted = date_time.format(rug_fuzz_6).to_string();
        debug_assert_eq!(formatted, "2023-04-10 10:00:00");
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_date_time_formatting = 0;
    }
    #[test]
    fn test_timestamp_nanos() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_timestamp_nanos = 0;
        let rug_fuzz_0 = 1679155200000000000;
        let dt: DateTime<Utc> = Utc.timestamp_nanos(rug_fuzz_0);
        debug_assert_eq!(dt, Utc.ymd(2023, 4, 10).and_hms(10, 0, 0));
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_timestamp_nanos = 0;
    }
    #[test]
    fn test_naive_datetime_from_str() {
        let _rug_st_tests_llm_16_243_rrrruuuugggg_test_naive_datetime_from_str = 0;
        let rug_fuzz_0 = "2023-04-10T10:00:00";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%S";
        let res = NaiveDateTime::parse_from_str(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(res.is_ok());
        let naive_date_time = res.unwrap();
        debug_assert_eq!(
            naive_date_time, NaiveDateTime::new(NaiveDate::from_ymd(2023, 4, 10),
            NaiveTime::from_hms(10, 0, 0),)
        );
        let _rug_ed_tests_llm_16_243_rrrruuuugggg_test_naive_datetime_from_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_244_llm_16_244 {
    use super::*;
    use crate::*;
    use crate::offset::FixedOffset;
    use crate::offset::TimeZone;
    use crate::DateTime;
    use crate::LocalResult;
    use crate::NaiveDate;
    use crate::NaiveDateTime;
    #[test]
    fn test_fixed_offset_east_opt() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_east_opt = 0;
        let rug_fuzz_0 = 3600;
        let offset = FixedOffset::east_opt(rug_fuzz_0);
        debug_assert!(offset.is_some());
        debug_assert_eq!(offset.unwrap().local_minus_utc(), 3600);
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_east_opt = 0;
    }
    #[test]
    fn test_fixed_offset_west_opt() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_west_opt = 0;
        let rug_fuzz_0 = 3600;
        let offset = FixedOffset::west_opt(rug_fuzz_0);
        debug_assert!(offset.is_some());
        debug_assert_eq!(offset.unwrap().local_minus_utc(), - 3600);
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_west_opt = 0;
    }
    #[test]
    fn test_fixed_offset_from_utc_date() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_utc_date = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2022;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let utc_date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap();
        let date = offset.from_utc_date(&utc_date);
        debug_assert_eq!(
            date.naive_utc(), NaiveDate::from_ymd_opt(2022, 3, 14).unwrap()
        );
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_utc_date = 0;
    }
    #[test]
    fn test_fixed_offset_from_utc_datetime() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_utc_datetime = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2022;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let utc_datetime = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap()
            .and_hms_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        let local_datetime: DateTime<FixedOffset> = offset
            .from_utc_datetime(&utc_datetime);
        debug_assert_eq!(
            local_datetime, offset.ymd_opt(2022, 3, 14).unwrap().and_hms_opt(13, 0, 0)
            .unwrap()
        );
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_utc_datetime = 0;
    }
    #[test]
    fn test_fixed_offset_from_local_date() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_local_date = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2022;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let local_date = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap();
        let offset_date = offset.from_local_date(&local_date);
        debug_assert_eq!(offset_date.unwrap().naive_utc(), local_date);
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_local_date = 0;
    }
    #[test]
    fn test_fixed_offset_from_local_datetime() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_local_datetime = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2022;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let local_datetime = NaiveDate::from_ymd_opt(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .unwrap()
            .and_hms_opt(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        let offset_datetime = offset.from_local_datetime(&local_datetime);
        debug_assert_eq!(
            offset_datetime, LocalResult::Single(DateTime::from_utc(local_datetime -
            offset.fix(), offset))
        );
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_from_local_datetime = 0;
    }
    #[test]
    fn test_fixed_offset_datetime_from_str() {
        let _rug_st_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_datetime_from_str = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = "2022-03-14 13:00:00";
        let rug_fuzz_2 = "%Y-%m-%d %H:%M:%S";
        let rug_fuzz_3 = 2022;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 14;
        let rug_fuzz_6 = 13;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let offset = FixedOffset::east_opt(rug_fuzz_0).unwrap();
        let datetime_str = rug_fuzz_1;
        let parsed_datetime = offset.datetime_from_str(datetime_str, rug_fuzz_2);
        debug_assert!(parsed_datetime.is_ok());
        let expected_datetime = offset
            .ymd_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap()
            .and_hms_opt(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .unwrap();
        debug_assert_eq!(parsed_datetime.unwrap(), expected_datetime);
        let _rug_ed_tests_llm_16_244_llm_16_244_rrrruuuugggg_test_fixed_offset_datetime_from_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_245 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, FixedOffset};
    use crate::naive::{NaiveDate, NaiveDateTime};
    use crate::DateTime;
    #[test]
    fn test_fixed_offset() {
        let _rug_st_tests_llm_16_245_rrrruuuugggg_test_fixed_offset = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 15;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 2023;
        let rug_fuzz_8 = 3;
        let rug_fuzz_9 = 15;
        let rug_fuzz_10 = 12;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 5;
        let rug_fuzz_14 = 3600;
        let naive_date_time = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let datetime = DateTime::<
            FixedOffset,
        >::from_utc(naive_date_time, FixedOffset::east(rug_fuzz_6));
        let fixed_datetime = datetime.fixed_offset();
        debug_assert_eq!(fixed_datetime, datetime);
        let naive_date_time = NaiveDate::from_ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9)
            .and_hms(rug_fuzz_10, rug_fuzz_11, rug_fuzz_12);
        let offsets = vec![- rug_fuzz_13, 0, 5];
        for &offset in &offsets {
            let datetime = DateTime::<
                FixedOffset,
            >::from_utc(naive_date_time, FixedOffset::east(offset * rug_fuzz_14));
            let fixed_datetime = datetime.fixed_offset();
            debug_assert_eq!(
                fixed_datetime, datetime.with_timezone(& FixedOffset::east(offset *
                3600))
            );
        }
        let _rug_ed_tests_llm_16_245_rrrruuuugggg_test_fixed_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_246 {
    use super::*;
    use crate::*;
    use crate::{TimeZone, Utc, FixedOffset};
    #[test]
    fn format_utc_date_time() {
        let _rug_st_tests_llm_16_246_rrrruuuugggg_format_utc_date_time = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 11;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = "%Y-%m-%d %H:%M:%S";
        let date_time: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let formatted = date_time.format(rug_fuzz_6).to_string();
        debug_assert_eq!(formatted, "2023-03-30 10:11:12");
        let _rug_ed_tests_llm_16_246_rrrruuuugggg_format_utc_date_time = 0;
    }
    #[test]
    fn format_fixed_offset_date_time() {
        let _rug_st_tests_llm_16_246_rrrruuuugggg_format_fixed_offset_date_time = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = 34;
        let rug_fuzz_7 = 56;
        let rug_fuzz_8 = "%Y-%m-%d %H:%M:%SZ";
        let fixed_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let date_time = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let formatted = date_time.format(rug_fuzz_8).to_string();
        debug_assert_eq!(formatted, "2023-03-30 12:34:56Z");
        let _rug_ed_tests_llm_16_246_rrrruuuugggg_format_fixed_offset_date_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_248 {
    use crate::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
    #[test]
    fn test_from_local_with_east_offset() {
        let _rug_st_tests_llm_16_248_rrrruuuugggg_test_from_local_with_east_offset = 0;
        let rug_fuzz_0 = 2000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 8;
        let rug_fuzz_7 = 60;
        let rug_fuzz_8 = 60;
        let rug_fuzz_9 = "2000-01-12T10:00:00+08:00";
        let naive_datetime_east = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let timezone_east = FixedOffset::east_opt(rug_fuzz_6 * rug_fuzz_7 * rug_fuzz_8)
            .unwrap();
        let datetime_east = DateTime::<
            FixedOffset,
        >::from_local(naive_datetime_east, timezone_east);
        let expected_datetime = DateTime::<FixedOffset>::parse_from_rfc3339(rug_fuzz_9)
            .unwrap();
        debug_assert_eq!(datetime_east, expected_datetime);
        let _rug_ed_tests_llm_16_248_rrrruuuugggg_test_from_local_with_east_offset = 0;
    }
    #[test]
    fn test_from_local_with_west_offset() {
        let _rug_st_tests_llm_16_248_rrrruuuugggg_test_from_local_with_west_offset = 0;
        let rug_fuzz_0 = 2000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 11;
        let rug_fuzz_3 = 19;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 7;
        let rug_fuzz_7 = 60;
        let rug_fuzz_8 = 60;
        let rug_fuzz_9 = 2000;
        let rug_fuzz_10 = 1;
        let rug_fuzz_11 = 12;
        let rug_fuzz_12 = 2;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let naive_datetime_west = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let timezone_west = FixedOffset::west_opt(rug_fuzz_6 * rug_fuzz_7 * rug_fuzz_8)
            .unwrap();
        let datetime_west = DateTime::<
            FixedOffset,
        >::from_local(naive_datetime_west, timezone_west);
        let naive_datetime_as_utc = NaiveDate::from_ymd_opt(
                rug_fuzz_9,
                rug_fuzz_10,
                rug_fuzz_11,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14)
            .unwrap();
        let datetime_as_utc = DateTime::<Utc>::from_utc(naive_datetime_as_utc, Utc);
        debug_assert_eq!(datetime_west, datetime_as_utc.with_timezone(& timezone_west));
        let _rug_ed_tests_llm_16_248_rrrruuuugggg_test_from_local_with_west_offset = 0;
    }
    #[test]
    fn test_from_local_with_utc_offset() {
        let _rug_st_tests_llm_16_248_rrrruuuugggg_test_from_local_with_utc_offset = 0;
        let rug_fuzz_0 = 2000;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 12;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let naive_datetime_utc = NaiveDate::from_ymd_opt(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .unwrap()
            .and_hms_opt(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .unwrap();
        let timezone_utc = FixedOffset::east_opt(rug_fuzz_6).unwrap();
        let datetime_utc = DateTime::<
            FixedOffset,
        >::from_local(naive_datetime_utc, timezone_utc);
        let expected_datetime_utc = DateTime::<Utc>::from_utc(naive_datetime_utc, Utc);
        debug_assert_eq!(
            datetime_utc, expected_datetime_utc.with_timezone(& timezone_utc)
        );
        let _rug_ed_tests_llm_16_248_rrrruuuugggg_test_from_local_with_utc_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_249 {
    use crate::{DateTime, NaiveDateTime, Utc, TimeZone};
    #[test]
    fn test_from_utc() {
        let _rug_st_tests_llm_16_249_rrrruuuugggg_test_from_utc = 0;
        let rug_fuzz_0 = 61;
        let rug_fuzz_1 = 0;
        let naive_dt = NaiveDateTime::from_timestamp_opt(rug_fuzz_0, rug_fuzz_1)
            .unwrap();
        let utc_dt: DateTime<Utc> = DateTime::from_utc(naive_dt, Utc);
        debug_assert_eq!(utc_dt, Utc.timestamp_opt(61, 0).unwrap());
        let direct_utc_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);
        debug_assert_eq!(direct_utc_dt, utc_dt);
        let _rug_ed_tests_llm_16_249_rrrruuuugggg_test_from_utc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_251 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, TimeZone, NaiveDateTime};
    #[test]
    fn test_naive_utc() {
        let _rug_st_tests_llm_16_251_rrrruuuugggg_test_naive_utc = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2023;
        let rug_fuzz_9 = 4;
        let rug_fuzz_10 = 9;
        let rug_fuzz_11 = 5;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let datetime_with_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1)
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let naive_utc = datetime_with_offset.naive_utc();
        let expected_naive_utc = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10),
            NaiveTime::from_hms(rug_fuzz_11, rug_fuzz_12, rug_fuzz_13),
        );
        debug_assert_eq!(
            naive_utc, expected_naive_utc,
            "naive_utc function did not return the expected NaiveDateTime"
        );
        let _rug_ed_tests_llm_16_251_rrrruuuugggg_test_naive_utc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_254 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_time_method_without_tz() {
        let _rug_st_tests_llm_16_254_rrrruuuugggg_test_time_method_without_tz = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 30;
        let rug_fuzz_5 = 45;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 3600;
        let naive_utc_time = NaiveDateTime::new(
            NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            NaiveTime::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5),
        );
        let utc_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_utc_time);
        let extracted_time_utc = utc_time.time();
        debug_assert_eq!(extracted_time_utc, naive_utc_time.time());
        let fixed_offset = FixedOffset::east(rug_fuzz_6 * rug_fuzz_7);
        let fixed_time: DateTime<FixedOffset> = fixed_offset
            .from_utc_datetime(&naive_utc_time);
        let extracted_time_fixed = fixed_time.time();
        debug_assert_eq!(extracted_time_fixed, NaiveTime::from_hms(9, 30, 45));
        let _rug_ed_tests_llm_16_254_rrrruuuugggg_test_time_method_without_tz = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_257 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_timestamp_millis_utc() {
        let _rug_st_tests_llm_16_257_rrrruuuugggg_test_timestamp_millis_utc = 0;
        let rug_fuzz_0 = 1970;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 234;
        let dt_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt_utc.timestamp_millis(), 1_234);
        let _rug_ed_tests_llm_16_257_rrrruuuugggg_test_timestamp_millis_utc = 0;
    }
    #[test]
    fn test_timestamp_millis_fixed_offset() {
        let _rug_st_tests_llm_16_257_rrrruuuugggg_test_timestamp_millis_fixed_offset = 0;
        let rug_fuzz_0 = 3;
        let rug_fuzz_1 = 3600;
        let rug_fuzz_2 = 1970;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 234;
        let fixed_offset = FixedOffset::east(rug_fuzz_0 * rug_fuzz_1);
        let dt_fixed: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms_milli(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        debug_assert_eq!(dt_fixed.timestamp_millis(), 1_234);
        let _rug_ed_tests_llm_16_257_rrrruuuugggg_test_timestamp_millis_fixed_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_258 {
    use crate::{NaiveDate, Utc, TimeZone};
    #[test]
    fn test_timestamp_nanos() {
        let _rug_st_tests_llm_16_258_rrrruuuugggg_test_timestamp_nanos = 0;
        let rug_fuzz_0 = 1970;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1970;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 1;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 100_000_000;
        let rug_fuzz_14 = 2001;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 28;
        let rug_fuzz_17 = 23;
        let rug_fuzz_18 = 59;
        let rug_fuzz_19 = 59;
        let rug_fuzz_20 = 999_999_999;
        let rug_fuzz_21 = 2001;
        let rug_fuzz_22 = 9;
        let rug_fuzz_23 = 9;
        let rug_fuzz_24 = 1;
        let rug_fuzz_25 = 46;
        let rug_fuzz_26 = 40;
        let rug_fuzz_27 = 555;
        let rug_fuzz_28 = 1969;
        let rug_fuzz_29 = 12;
        let rug_fuzz_30 = 31;
        let rug_fuzz_31 = 23;
        let rug_fuzz_32 = 59;
        let rug_fuzz_33 = 59;
        let rug_fuzz_34 = 1_000_000_000;
        let rug_fuzz_35 = 4000;
        let rug_fuzz_36 = 1;
        let rug_fuzz_37 = 1;
        let rug_fuzz_38 = 0;
        let rug_fuzz_39 = 0;
        let rug_fuzz_40 = 0;
        let rug_fuzz_41 = 0;
        let rug_fuzz_42 = 0;
        let rug_fuzz_43 = 2015;
        let rug_fuzz_44 = 7;
        let rug_fuzz_45 = 1;
        let rug_fuzz_46 = 0;
        let rug_fuzz_47 = 0;
        let rug_fuzz_48 = 0;
        let rug_fuzz_49 = 0;
        let rug_fuzz_50 = 143_570_879_999_999_999;
        debug_assert_eq!(
            Utc.ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).and_hms_nano(rug_fuzz_3,
            rug_fuzz_4, rug_fuzz_5, rug_fuzz_6).timestamp_nanos(), 0
        );
        debug_assert_eq!(
            Utc.ymd(rug_fuzz_7, rug_fuzz_8, rug_fuzz_9).and_hms_nano(rug_fuzz_10,
            rug_fuzz_11, rug_fuzz_12, rug_fuzz_13).timestamp_nanos(), 100_000_000
        );
        debug_assert_eq!(
            Utc.ymd(rug_fuzz_14, rug_fuzz_15, rug_fuzz_16).and_hms_nano(rug_fuzz_17,
            rug_fuzz_18, rug_fuzz_19, rug_fuzz_20).timestamp_nanos(),
            981_173_999_999_999_999
        );
        let dt = NaiveDate::from_ymd_opt(rug_fuzz_21, rug_fuzz_22, rug_fuzz_23)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_24, rug_fuzz_25, rug_fuzz_26, rug_fuzz_27)
            .unwrap();
        debug_assert_eq!(dt.timestamp_nanos(), 1_000_000_000_000_000_555);
        let dt_before_epoch = NaiveDate::from_ymd_opt(
                rug_fuzz_28,
                rug_fuzz_29,
                rug_fuzz_30,
            )
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_31, rug_fuzz_32, rug_fuzz_33, rug_fuzz_34)
            .unwrap();
        debug_assert_eq!(dt_before_epoch.timestamp_nanos(), - 1_000_000_000);
        let dt_far_future = NaiveDate::from_ymd_opt(
                rug_fuzz_35,
                rug_fuzz_36,
                rug_fuzz_37,
            )
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_38, rug_fuzz_39, rug_fuzz_40, rug_fuzz_41)
            .unwrap();
        debug_assert!(dt_far_future.timestamp_nanos() > rug_fuzz_42);
        let after_leap = NaiveDate::from_ymd_opt(rug_fuzz_43, rug_fuzz_44, rug_fuzz_45)
            .unwrap()
            .and_hms_nano_opt(rug_fuzz_46, rug_fuzz_47, rug_fuzz_48, rug_fuzz_49)
            .unwrap();
        debug_assert!(after_leap.timestamp_nanos() > rug_fuzz_50);
        let _rug_ed_tests_llm_16_258_rrrruuuugggg_test_timestamp_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_259 {
    use super::*;
    use crate::*;
    use crate::offset::{TimeZone, FixedOffset};
    use crate::DateTime;
    use crate::NaiveDateTime;
    #[test]
    fn test_timestamp_subsec_micros() {
        let _rug_st_tests_llm_16_259_rrrruuuugggg_test_timestamp_subsec_micros = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = "2023-01-01T01:01:01.123456";
        let rug_fuzz_2 = "%Y-%m-%dT%H:%M:%S%.f";
        let rug_fuzz_3 = "2023-01-01T01:01:01.000123";
        let rug_fuzz_4 = "%Y-%m-%dT%H:%M:%S%.f";
        let rug_fuzz_5 = "2023-01-01T01:01:01";
        let rug_fuzz_6 = "%Y-%m-%dT%H:%M:%S";
        let rug_fuzz_7 = "2023-01-01T01:01:01.999999";
        let rug_fuzz_8 = "%Y-%m-%dT%H:%M:%S%.f";
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let naive_date_time = NaiveDateTime::parse_from_str(rug_fuzz_1, rug_fuzz_2)
            .unwrap();
        let date_time: DateTime<FixedOffset> = DateTime::from_utc(
            naive_date_time,
            fixed_offset,
        );
        debug_assert_eq!(date_time.timestamp_subsec_micros(), 123456);
        let naive_date_time = NaiveDateTime::parse_from_str(rug_fuzz_3, rug_fuzz_4)
            .unwrap();
        let date_time: DateTime<FixedOffset> = DateTime::from_utc(
            naive_date_time,
            fixed_offset,
        );
        debug_assert_eq!(date_time.timestamp_subsec_micros(), 123);
        let naive_date_time = NaiveDateTime::parse_from_str(rug_fuzz_5, rug_fuzz_6)
            .unwrap();
        let date_time: DateTime<FixedOffset> = DateTime::from_utc(
            naive_date_time,
            fixed_offset,
        );
        debug_assert_eq!(date_time.timestamp_subsec_micros(), 0);
        let naive_date_time = NaiveDateTime::parse_from_str(rug_fuzz_7, rug_fuzz_8)
            .unwrap();
        let date_time: DateTime<FixedOffset> = DateTime::from_utc(
            naive_date_time,
            fixed_offset,
        );
        debug_assert_eq!(date_time.timestamp_subsec_micros(), 999999);
        let _rug_ed_tests_llm_16_259_rrrruuuugggg_test_timestamp_subsec_micros = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_260 {
    use super::*;
    use crate::*;
    use crate::{DateTime, NaiveDateTime, TimeZone, Utc};
    #[test]
    fn test_timestamp_subsec_millis_at_second_boundary() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_second_boundary = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 0);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_second_boundary = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_at_subsecond() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_subsecond = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 123;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 123);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_subsecond = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_at_leap_second() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_leap_second = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 1_123;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 1_123);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_leap_second = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_on_negative_subsecond() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_on_negative_subsecond = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 1_987_654_321;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 987);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_on_negative_subsecond = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_just_before_new_year() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_just_before_new_year = 0;
        let rug_fuzz_0 = 2019;
        let rug_fuzz_1 = 12;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 999;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_just_before_new_year = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_just_after_new_year() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_just_after_new_year = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 1);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_just_after_new_year = 0;
    }
    #[test]
    fn test_timestamp_subsec_millis_at_max_value() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_max_value = 0;
        let rug_fuzz_0 = 2020;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 999;
        let dt = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(dt.timestamp_subsec_millis(), 999);
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_at_max_value = 0;
    }
    #[test]
    #[should_panic]
    fn test_timestamp_subsec_millis_panics_on_overflow() {
        let _rug_st_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_panics_on_overflow = 0;
        let rug_fuzz_0 = 253_402_300_800;
        let rug_fuzz_1 = 0;
        let dt: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::from_timestamp(rug_fuzz_0, rug_fuzz_1),
            Utc,
        );
        let _ = dt.timestamp_subsec_millis();
        let _rug_ed_tests_llm_16_260_rrrruuuugggg_test_timestamp_subsec_millis_panics_on_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_261 {
    use super::*;
    use crate::*;
    use crate::{DateTime, Utc, FixedOffset, NaiveDate, NaiveTime};
    #[test]
    fn test_timestamp_subsec_nanos() {
        let _rug_st_tests_llm_16_261_rrrruuuugggg_test_timestamp_subsec_nanos = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 6;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let rug_fuzz_6 = 789_012_345;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = 3600;
        let rug_fuzz_9 = 2023;
        let rug_fuzz_10 = 4;
        let rug_fuzz_11 = 6;
        let rug_fuzz_12 = 17;
        let rug_fuzz_13 = 34;
        let rug_fuzz_14 = 56;
        let rug_fuzz_15 = 123_456_789;
        let rug_fuzz_16 = 2023;
        let rug_fuzz_17 = 4;
        let rug_fuzz_18 = 6;
        let rug_fuzz_19 = 12;
        let rug_fuzz_20 = 34;
        let rug_fuzz_21 = 56;
        let rug_fuzz_22 = 789_012_345;
        let rug_fuzz_23 = 12;
        let rug_fuzz_24 = 34;
        let rug_fuzz_25 = 56;
        let rug_fuzz_26 = 789_012_345;
        let date_time_utc: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_nano(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(date_time_utc.timestamp_subsec_nanos(), 789_012_345);
        let date_time_fixed: DateTime<FixedOffset> = FixedOffset::east(
                rug_fuzz_7 * rug_fuzz_8,
            )
            .ymd(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .and_hms_nano(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15);
        debug_assert_eq!(date_time_fixed.timestamp_subsec_nanos(), 123_456_789);
        let naive_date_time = NaiveDate::from_ymd(rug_fuzz_16, rug_fuzz_17, rug_fuzz_18)
            .and_hms_nano_opt(rug_fuzz_19, rug_fuzz_20, rug_fuzz_21, rug_fuzz_22)
            .unwrap();
        debug_assert_eq!(naive_date_time.timestamp_subsec_nanos(), 789_012_345);
        let naive_time = NaiveTime::from_hms_nano_opt(
                rug_fuzz_23,
                rug_fuzz_24,
                rug_fuzz_25,
                rug_fuzz_26,
            )
            .unwrap();
        debug_assert_eq!(naive_time.nanosecond(), 789_012_345);
        let _rug_ed_tests_llm_16_261_rrrruuuugggg_test_timestamp_subsec_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_264 {
    use super::*;
    use crate::*;
    use crate::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_to_rfc3339() {
        let _rug_st_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339 = 0;
        let rug_fuzz_0 = 3600;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let rug_fuzz_4 = 15;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = 26;
        let fixed_offset = FixedOffset::east(rug_fuzz_0);
        let datetime: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.to_rfc3339(), "2023-03-14T15:09:26+01:00");
        let _rug_ed_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339 = 0;
    }
    #[test]
    fn test_to_rfc3339_with_utc() {
        let _rug_st_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339_with_utc = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 14;
        let rug_fuzz_3 = 15;
        let rug_fuzz_4 = 9;
        let rug_fuzz_5 = 26;
        let datetime: DateTime<Utc> = Utc
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        debug_assert_eq!(datetime.to_rfc3339(), "2023-03-14T15:09:26Z");
        let _rug_ed_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339_with_utc = 0;
    }
    #[test]
    fn test_to_rfc3339_with_negative_offset() {
        let _rug_st_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339_with_negative_offset = 0;
        let rug_fuzz_0 = 5400;
        let rug_fuzz_1 = 2023;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 14;
        let rug_fuzz_4 = 15;
        let rug_fuzz_5 = 9;
        let rug_fuzz_6 = 26;
        let fixed_offset = FixedOffset::west(rug_fuzz_0);
        let datetime: DateTime<FixedOffset> = fixed_offset
            .ymd(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3)
            .and_hms(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        debug_assert_eq!(datetime.to_rfc3339(), "2023-03-14T15:09:26-01:30");
        let _rug_ed_tests_llm_16_264_rrrruuuugggg_test_to_rfc3339_with_negative_offset = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_265 {
    use super::*;
    use crate::*;
    use crate::{FixedOffset, NaiveDate, SecondsFormat, TimeZone};
    #[test]
    fn test_to_rfc3339_opts() {
        let _rug_st_tests_llm_16_265_rrrruuuugggg_test_to_rfc3339_opts = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 9;
        let rug_fuzz_2 = 8;
        let rug_fuzz_3 = 13;
        let rug_fuzz_4 = 5;
        let rug_fuzz_5 = 7;
        let rug_fuzz_6 = 890;
        let rug_fuzz_7 = 3600;
        let rug_fuzz_8 = true;
        let rug_fuzz_9 = false;
        let rug_fuzz_10 = true;
        let rug_fuzz_11 = false;
        let dt = NaiveDate::from_ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms_milli(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5, rug_fuzz_6)
            .and_local_timezone(FixedOffset::east(rug_fuzz_7))
            .unwrap();
        debug_assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::Secs, rug_fuzz_8), "2023-09-08T13:05:07Z"
        );
        debug_assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::Millis, rug_fuzz_9),
            "2023-09-08T13:05:07.890+01:00"
        );
        debug_assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::Nanos, rug_fuzz_10),
            "2023-09-08T13:05:07.890Z"
        );
        debug_assert_eq!(
            dt.to_rfc3339_opts(SecondsFormat::AutoSi, rug_fuzz_11),
            "2023-09-08T13:05:07.890+01:00"
        );
        let _rug_ed_tests_llm_16_265_rrrruuuugggg_test_to_rfc3339_opts = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_267 {
    use super::*;
    use crate::*;
    use crate::offset::TimeZone;
    #[test]
    fn test_years_since_past() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_past = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(23));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_past = 0;
    }
    #[test]
    fn test_years_since_future() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_future = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2040;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), None);
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_future = 0;
    }
    #[test]
    fn test_years_since_earlier_in_year() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_earlier_in_year = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 7;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(22));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_earlier_in_year = 0;
    }
    #[test]
    fn test_years_since_later_in_year() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_later_in_year = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 7;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 1;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(23));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_later_in_year = 0;
    }
    #[test]
    fn test_years_since_same_day() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(23));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day = 0;
    }
    #[test]
    fn test_years_since_same_day_different_time() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day_different_time = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 6;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(23));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day_different_time = 0;
    }
    #[test]
    fn test_years_since_same_day_time_earlier() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day_time_earlier = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 4;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 6;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2000;
        let rug_fuzz_7 = 4;
        let rug_fuzz_8 = 1;
        let rug_fuzz_9 = 12;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(22));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_same_day_time_earlier = 0;
    }
    #[test]
    fn test_years_since_leap_year() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_leap_year = 0;
        let rug_fuzz_0 = 2024;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 29;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2020;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 29;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(4));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_leap_year = 0;
    }
    #[test]
    fn test_years_since_leap_year_to_non_leap() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_years_since_leap_year_to_non_leap = 0;
        let rug_fuzz_0 = 2023;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 28;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2020;
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = 29;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let tz = crate::Utc;
        let now = tz
            .ymd(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2)
            .and_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5);
        let base = tz
            .ymd(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)
            .and_hms(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11);
        debug_assert_eq!(now.years_since(base), Some(2));
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_years_since_leap_year_to_non_leap = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_268_llm_16_268 {
    use crate::{DateTime, FixedOffset, TimeZone, offset};
    #[test]
    fn test_parse_from_rfc2822_valid_dates() {
        let _rug_st_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_parse_from_rfc2822_valid_dates = 0;
        let rug_fuzz_0 = "Tue, 1 Jul 2003 10:52:37 +0200";
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3600;
        let rug_fuzz_3 = 2003;
        let rug_fuzz_4 = 7;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 10;
        let rug_fuzz_7 = 52;
        let rug_fuzz_8 = 37;
        let pairs = vec![
            (rug_fuzz_0, FixedOffset::east(rug_fuzz_1 * rug_fuzz_2).ymd(rug_fuzz_3,
            rug_fuzz_4, rug_fuzz_5).and_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8)),
            ("Wed, 18 Feb 2015 23:16:09 GMT", FixedOffset::east(0).ymd(2015, 2, 18)
            .and_hms(23, 16, 9)), ("Mon, 22 Jul 2019 21:10:29 -0400", FixedOffset::west(4
            * 3600).ymd(2019, 7, 22).and_hms(21, 10, 29))
        ];
        for (input, expected) in pairs {
            let parsed = DateTime::<FixedOffset>::parse_from_rfc2822(input).unwrap();
            debug_assert_eq!(parsed, expected);
        }
        let _rug_ed_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_parse_from_rfc2822_valid_dates = 0;
    }
    #[test]
    fn test_parse_from_rfc2822_invalid_dates() {
        let _rug_st_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_parse_from_rfc2822_invalid_dates = 0;
        let rug_fuzz_0 = "Not a date";
        let inputs = vec![
            rug_fuzz_0, "Tue, 1 Jul 2003 99:99:99 +0200", "Wed, 18 Feb 2015"
        ];
        for input in inputs {
            debug_assert!(
                DateTime:: < FixedOffset > ::parse_from_rfc2822(input).is_err()
            );
        }
        let _rug_ed_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_parse_from_rfc2822_invalid_dates = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_270 {
    use crate::{DateTime, FixedOffset, TimeZone};
    #[test]
    fn test_parse_from_str_valid() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_valid = 0;
        let rug_fuzz_0 = "2023-03-14 11:00:00 +0100";
        let rug_fuzz_1 = "%Y-%m-%d %H:%M:%S %z";
        let rug_fuzz_2 = 3600;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 14;
        let rug_fuzz_6 = 11;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let expected = FixedOffset::east(rug_fuzz_2)
            .ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .and_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8);
        let result = DateTime::<FixedOffset>::parse_from_str(input, fmt);
        debug_assert_eq!(result.unwrap(), expected);
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_valid = 0;
    }
    #[test]
    fn test_parse_from_str_with_milliseconds() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_milliseconds = 0;
        let rug_fuzz_0 = "2023-03-14 11:00:00.123 +0100";
        let rug_fuzz_1 = "%Y-%m-%d %H:%M:%S%.3f %z";
        let rug_fuzz_2 = 3600;
        let rug_fuzz_3 = 2023;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 14;
        let rug_fuzz_6 = 11;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 123;
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let expected = FixedOffset::east(rug_fuzz_2)
            .ymd(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5)
            .and_hms_milli(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8, rug_fuzz_9);
        let result = DateTime::<FixedOffset>::parse_from_str(input, fmt);
        debug_assert_eq!(result.unwrap(), expected);
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_milliseconds = 0;
    }
    #[test]
    fn test_parse_from_str_with_invalid_format() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_invalid_format = 0;
        let rug_fuzz_0 = "An invalid date";
        let rug_fuzz_1 = "%Y-%m-%d %H:%M:%S %z";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let result = DateTime::<FixedOffset>::parse_from_str(input, fmt);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_invalid_format = 0;
    }
    #[test]
    fn test_parse_from_str_with_invalid_timezone() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_invalid_timezone = 0;
        let rug_fuzz_0 = "2023-03-14 11:00:00 +99:99";
        let rug_fuzz_1 = "%Y-%m-%d %H:%M:%S %z";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let result = DateTime::<FixedOffset>::parse_from_str(input, fmt);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_invalid_timezone = 0;
    }
    #[test]
    fn test_parse_from_str_with_no_timezone() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_no_timezone = 0;
        let rug_fuzz_0 = "2023-03-14 11:00:00";
        let rug_fuzz_1 = "%Y-%m-%d %H:%M:%S";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let result = DateTime::<FixedOffset>::parse_from_str(input, fmt);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_test_parse_from_str_with_no_timezone = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_271 {
    use super::*;
    use crate::*;
    use crate::{DateTime, Utc, TimeZone};
    #[test]
    fn test_parse_from_rfc2822_valid() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_valid = 0;
        let rug_fuzz_0 = "Tue, 1 Jul 2003 10:52:37 +0200";
        let rug_fuzz_1 = "2003-07-01T08:52:37Z";
        let rfc2822 = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let parsed = DateTime::<Utc>::parse_from_rfc2822(rfc2822).unwrap();
        debug_assert_eq!(parsed.to_rfc3339(), expected);
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_valid = 0;
    }
    #[test]
    fn test_parse_from_rfc2822_invalid() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_invalid = 0;
        let rug_fuzz_0 = "This is not a valid RFC 2822 date string";
        let rfc2822 = rug_fuzz_0;
        debug_assert!(DateTime:: < Utc > ::parse_from_rfc2822(rfc2822).is_err());
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_invalid = 0;
    }
    #[test]
    fn test_parse_from_rfc2822_with_wrong_day_name() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_with_wrong_day_name = 0;
        let rug_fuzz_0 = "Mon, 1 Jul 2003 10:52:37 +0200";
        let rfc2822 = rug_fuzz_0;
        debug_assert!(DateTime:: < Utc > ::parse_from_rfc2822(rfc2822).is_err());
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_with_wrong_day_name = 0;
    }
    #[test]
    fn test_parse_from_rfc2822_with_wrong_timezone() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_with_wrong_timezone = 0;
        let rug_fuzz_0 = "Tue, 1 Jul 2003 10:52:37 +9999";
        let rfc2822 = rug_fuzz_0;
        debug_assert!(DateTime:: < Utc > ::parse_from_rfc2822(rfc2822).is_err());
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_with_wrong_timezone = 0;
    }
    #[test]
    fn test_parse_from_rfc2822_timezone_conversion() {
        let _rug_st_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_timezone_conversion = 0;
        let rug_fuzz_0 = "Tue, 1 Jul 2003 10:52:37 -0700";
        let rug_fuzz_1 = "2003-07-01T17:52:37Z";
        let rfc2822 = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let parsed = DateTime::<Utc>::parse_from_rfc2822(rfc2822).unwrap();
        debug_assert_eq!(parsed.to_rfc3339(), expected);
        let _rug_ed_tests_llm_16_271_rrrruuuugggg_test_parse_from_rfc2822_timezone_conversion = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_272 {
    use crate::{DateTime, Utc, TimeZone, offset::FixedOffset};
    use crate::format::ParseResult;
    #[test]
    fn test_parse_from_rfc3339_valid() {
        let _rug_st_tests_llm_16_272_rrrruuuugggg_test_parse_from_rfc3339_valid = 0;
        let rug_fuzz_0 = "1996-12-19T16:39:57-08:00";
        let rug_fuzz_1 = "1996-12-19T16:39:57-08:00";
        let inputs_and_expected = vec![
            (rug_fuzz_0, rug_fuzz_1), ("1990-05-06T03:59:00+00:00",
            "1990-05-06T03:59:00+00:00"), ("2022-03-20T10:30:45Z",
            "2022-03-20T10:30:45Z")
        ];
        for (input, expected) in inputs_and_expected {
            let result = DateTime::<Utc>::parse_from_rfc3339(input);
            debug_assert!(result.is_ok());
            let datetime = result.unwrap();
            debug_assert_eq!(datetime.to_rfc3339(), expected);
        }
        let _rug_ed_tests_llm_16_272_rrrruuuugggg_test_parse_from_rfc3339_valid = 0;
    }
    #[test]
    fn test_parse_from_rfc3339_invalid() {
        let _rug_st_tests_llm_16_272_rrrruuuugggg_test_parse_from_rfc3339_invalid = 0;
        let rug_fuzz_0 = "not-a-date";
        let inputs = vec![
            rug_fuzz_0, "1996-12-19 16:39:57-08:00", "1996-12-19T25:39:57-08:00"
        ];
        for input in inputs {
            let result = DateTime::<Utc>::parse_from_rfc3339(input);
            debug_assert!(result.is_err());
        }
        let _rug_ed_tests_llm_16_272_rrrruuuugggg_test_parse_from_rfc3339_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_273 {
    use crate::{DateTime, TimeZone, Utc, FixedOffset, ParseResult};
    #[test]
    fn test_parse_from_str_with_correct_format_and_utc_timezone() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_correct_format_and_utc_timezone = 0;
        let rug_fuzz_0 = "2023-03-14T12:34:56Z";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%SZ";
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 14;
        let rug_fuzz_5 = 12;
        let rug_fuzz_6 = 34;
        let rug_fuzz_7 = 56;
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let expected = Utc
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert_eq!(actual, Ok(expected));
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_correct_format_and_utc_timezone = 0;
    }
    #[test]
    fn test_parse_from_str_with_correct_format_and_non_utc_timezone() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_correct_format_and_non_utc_timezone = 0;
        let rug_fuzz_0 = "2023-03-14T12:34:56+01:00";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%S%z";
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 14;
        let rug_fuzz_5 = 11;
        let rug_fuzz_6 = 34;
        let rug_fuzz_7 = 56;
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let expected = Utc
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert_eq!(actual, Ok(expected));
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_correct_format_and_non_utc_timezone = 0;
    }
    #[test]
    fn test_parse_from_str_with_incorrect_format() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_format = 0;
        let rug_fuzz_0 = "2023-03-14 12:34:56";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%SZ";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert!(actual.is_err());
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_format = 0;
    }
    #[test]
    fn test_parse_from_str_with_incorrect_date_values() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_date_values = 0;
        let rug_fuzz_0 = "2023-02-30T12:34:56Z";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%SZ";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert!(actual.is_err());
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_date_values = 0;
    }
    #[test]
    fn test_parse_from_str_with_time_zone_offset() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_time_zone_offset = 0;
        let rug_fuzz_0 = "2023-03-14T12:34:56+09:30";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%S%z";
        let rug_fuzz_2 = 2023;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 14;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 4;
        let rug_fuzz_7 = 56;
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let expected = Utc
            .ymd(rug_fuzz_2, rug_fuzz_3, rug_fuzz_4)
            .and_hms(rug_fuzz_5, rug_fuzz_6, rug_fuzz_7);
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert_eq!(actual, Ok(expected));
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_time_zone_offset = 0;
    }
    #[test]
    fn test_parse_from_str_with_incorrect_time_zone_format() {
        let _rug_st_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_time_zone_format = 0;
        let rug_fuzz_0 = "2023-03-14T12:34:56+0930";
        let rug_fuzz_1 = "%Y-%m-%dT%H:%M:%S%z";
        let input = rug_fuzz_0;
        let fmt = rug_fuzz_1;
        let actual: ParseResult<DateTime<Utc>> = DateTime::<
            Utc,
        >::parse_from_str(input, fmt);
        debug_assert!(actual.is_err());
        let _rug_ed_tests_llm_16_273_rrrruuuugggg_test_parse_from_str_with_incorrect_time_zone_format = 0;
    }
}
