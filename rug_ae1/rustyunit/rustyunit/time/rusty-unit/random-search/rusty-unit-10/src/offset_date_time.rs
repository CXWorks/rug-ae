//! The [`OffsetDateTime`] struct and its associated `impl`s.

use core::cmp::Ordering;
#[cfg(feature = "std")]
use core::convert::From;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;
#[cfg(feature = "std")]
use std::time::SystemTime;

#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::{error, Date, Duration, Month, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// The Julian day of the Unix epoch.
const UNIX_EPOCH_JULIAN_DAY: i32 = Date::__from_ordinal_date_unchecked(1970, 1).to_julian_day();

/// A [`PrimitiveDateTime`] with a [`UtcOffset`].
///
/// All comparisons are performed using the UTC time.
// Internally, an `OffsetDateTime` is a thin wrapper around a [`PrimitiveDateTime`] coupled with a
// [`UtcOffset`]. This offset is added to the date, time, or datetime as necessary for presentation
// or returning from a function.
#[derive(Debug, Clone, Copy, Eq)]
pub struct OffsetDateTime {
    /// The [`PrimitiveDateTime`], which is _always_ UTC.
    pub(crate) utc_datetime: PrimitiveDateTime,
    /// The [`UtcOffset`], which will be added to the [`PrimitiveDateTime`] as necessary.
    pub(crate) offset: UtcOffset,
}

impl OffsetDateTime {
    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, macros::datetime};
    /// assert_eq!(OffsetDateTime::UNIX_EPOCH, datetime!(1970-01-01 0:00 UTC),);
    /// ```
    pub const UNIX_EPOCH: Self = Date::__from_ordinal_date_unchecked(1970, 1)
        .midnight()
        .assume_utc();

    // region: now
    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, macros::offset};
    /// assert!(OffsetDateTime::now_utc().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now_utc().offset(), offset!(UTC));
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
    pub fn now_utc() -> Self {
        SystemTime::now().into()
    }

    /// Attempt to create a new `OffsetDateTime` with the current date and time in the local offset.
    /// If the offset cannot be determined, an error is returned.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # if false {
    /// assert!(OffsetDateTime::now_local().is_ok());
    /// # }
    /// ```
    #[cfg(feature = "local-offset")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "local-offset")))]
    pub fn now_local() -> Result<Self, error::IndeterminateOffset> {
        let t = Self::now_utc();
        Ok(t.to_offset(UtcOffset::local_offset_at(t)?))
    }
    // endregion now

    /// Convert the `OffsetDateTime` from the current [`UtcOffset`] to the provided [`UtcOffset`].
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!(2000-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .year(),
    ///     1999,
    /// );
    ///
    /// // Let's see what time Sydney's new year's celebration is in New York // and Los Angeles.
    ///
    /// // Construct midnight on new year's in Sydney.
    /// let sydney = datetime!(2000-01-01 0:00 +11);
    /// let new_york = sydney.to_offset(offset!(-5));
    /// let los_angeles = sydney.to_offset(offset!(-8));
    /// assert_eq!(sydney.hour(), 0);
    /// assert_eq!(new_york.hour(), 8);
    /// assert_eq!(los_angeles.hour(), 5);
    /// ```
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        Self {
            utc_datetime: self.utc_datetime,
            offset,
        }
    }

    // region: constructors
    /// Create an `OffsetDateTime` from the provided Unix timestamp. Calling `.offset()` on the
    /// resulting value is guaranteed to return UTC.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, macros::datetime};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     Ok(OffsetDateTime::UNIX_EPOCH),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     Ok(datetime!(2019-01-01 0:00 UTC)),
    /// );
    /// ```
    ///
    /// If you have a timestamp-nanosecond pair, you can use something along the lines of the
    /// following:
    ///
    /// ```rust
    /// # use time::{Duration, OffsetDateTime, ext::NumericalDuration};
    /// let (timestamp, nanos) = (1, 500_000_000);
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(timestamp)? + Duration::nanoseconds(nanos),
    ///     OffsetDateTime::UNIX_EPOCH + 1.5.seconds()
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub const fn from_unix_timestamp(timestamp: i64) -> Result<Self, error::ComponentRange> {
        #[allow(clippy::missing_docs_in_private_items)]
        const MIN_TIMESTAMP: i64 = Date::MIN.midnight().assume_utc().unix_timestamp();
        #[allow(clippy::missing_docs_in_private_items)]
        const MAX_TIMESTAMP: i64 = Date::MAX
            .with_time(Time::__from_hms_nanos_unchecked(23, 59, 59, 999_999_999))
            .assume_utc()
            .unix_timestamp();

        ensure_value_in_range!(timestamp in MIN_TIMESTAMP => MAX_TIMESTAMP);

        // Use the unchecked method here, as the input validity has already been verified.
        let date = Date::from_julian_day_unchecked(
            UNIX_EPOCH_JULIAN_DAY + div_floor!(timestamp, 86_400) as i32,
        );

        let seconds_within_day = timestamp.rem_euclid(86_400);
        let time = Time::__from_hms_nanos_unchecked(
            (seconds_within_day / 3_600) as _,
            ((seconds_within_day % 3_600) / 60) as _,
            (seconds_within_day % 60) as _,
            0,
        );

        Ok(PrimitiveDateTime::new(date, time).assume_utc())
    }

    /// Construct an `OffsetDateTime` from the provided Unix timestamp (in nanoseconds). Calling
    /// `.offset()` on the resulting value is guaranteed to return UTC.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, macros::datetime};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(0),
    ///     Ok(OffsetDateTime::UNIX_EPOCH),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
    ///     Ok(datetime!(2019-01-01 0:00 UTC)),
    /// );
    /// ```
    pub const fn from_unix_timestamp_nanos(timestamp: i128) -> Result<Self, error::ComponentRange> {
        let datetime = const_try!(Self::from_unix_timestamp(
            div_floor!(timestamp, 1_000_000_000) as i64
        ));

        Ok(datetime
            .utc_datetime
            .replace_time(Time::__from_hms_nanos_unchecked(
                datetime.utc_datetime.hour(),
                datetime.utc_datetime.minute(),
                datetime.utc_datetime.second(),
                timestamp.rem_euclid(1_000_000_000) as u32,
            ))
            .assume_utc())
    }
    // endregion constructors

    // region: getters
    /// Get the [`UtcOffset`].
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).offset(), offset!(UTC));
    /// assert_eq!(datetime!(2019-01-01 0:00 +1).offset(), offset!(+1));
    /// ```
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(1970-01-01 0:00 UTC).unix_timestamp(), 0);
    /// assert_eq!(datetime!(1970-01-01 0:00 -1).unix_timestamp(), 3_600);
    /// ```
    pub const fn unix_timestamp(self) -> i64 {
        let days =
            (self.utc_datetime.to_julian_day() as i64 - UNIX_EPOCH_JULIAN_DAY as i64) * 86_400;
        let hours = self.utc_datetime.hour() as i64 * 3_600;
        let minutes = self.utc_datetime.minute() as i64 * 60;
        let seconds = self.utc_datetime.second() as i64;
        days + hours + minutes + seconds
    }

    /// Get the Unix timestamp in nanoseconds.
    ///
    /// ```rust
    /// use time::macros::datetime;
    /// assert_eq!(datetime!(1970-01-01 0:00 UTC).unix_timestamp_nanos(), 0);
    /// assert_eq!(
    ///     datetime!(1970-01-01 0:00 -1).unix_timestamp_nanos(),
    ///     3_600_000_000_000,
    /// );
    /// ```
    pub const fn unix_timestamp_nanos(self) -> i128 {
        self.unix_timestamp() as i128 * 1_000_000_000 + self.utc_datetime.nanosecond() as i128
    }

    /// Get the [`Date`] in the stored offset.
    ///
    /// ```rust
    /// # use time::macros::{date, datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).date(), date!(2019-01-01));
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .date(),
    ///     date!(2018-12-31),
    /// );
    /// ```
    pub const fn date(self) -> Date {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();
        let mut hour = self.utc_datetime.hour() as i8 + self.offset.whole_hours();
        let (mut year, mut ordinal) = self.utc_datetime.date.to_ordinal_date();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        cascade!(hour in 0..24 => ordinal);
        cascade!(ordinal => year);

        Date::__from_ordinal_date_unchecked(year, ordinal)
    }

    /// Get the [`Time`] in the stored offset.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset, time};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).time(), time!(0:00));
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC)
    ///         .to_offset(offset!(-1))
    ///         .time(),
    ///     time!(23:00)
    /// );
    /// ```
    pub const fn time(self) -> Time {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();
        let mut hour = self.utc_datetime.hour() as i8 + self.offset.whole_hours();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);

        Time::__from_hms_nanos_unchecked(
            hour.rem_euclid(24) as _,
            minute as _,
            second as _,
            self.utc_datetime.nanosecond(),
        )
    }

    // region: date getters
    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).year(), 2019);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).year(), 2020);
    /// ```
    pub const fn year(self) -> i32 {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();
        let mut hour = self.utc_datetime.hour() as i8 + self.offset.whole_hours();
        let (mut year, mut ordinal) = self.utc_datetime.date.to_ordinal_date();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        cascade!(hour in 0..24 => ordinal);
        cascade!(ordinal => year);

        year
    }

    /// Get the month of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::Month;
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).month(), Month::January);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .month(),
    ///     Month::January,
    /// );
    /// ```
    pub const fn month(self) -> Month {
        self.date().month()
    }

    /// Get the day of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).day(), 1);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .day(),
    ///     1,
    /// );
    /// ```
    pub const fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).ordinal(), 1);
    /// assert_eq!(
    ///     datetime!(2019-12-31 23:00 UTC)
    ///         .to_offset(offset!(+1))
    ///         .ordinal(),
    ///     1,
    /// );
    /// ```
    pub const fn ordinal(self) -> u16 {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();
        let mut hour = self.utc_datetime.hour() as i8 + self.offset.whole_hours();
        let (mut year, mut ordinal) = self.utc_datetime.date.to_ordinal_date();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        cascade!(hour in 0..24 => ordinal);
        cascade!(ordinal => year);

        ordinal
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).iso_week(), 1);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).iso_week(), 1);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).iso_week(), 53);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).iso_week(), 53);
    /// ```
    pub const fn iso_week(self) -> u8 {
        self.date().iso_week()
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).sunday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).sunday_based_week(), 0);
    /// ```
    pub const fn sunday_based_week(self) -> u8 {
        self.date().sunday_based_week()
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00 UTC).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00 UTC).monday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00 UTC).monday_based_week(), 0);
    /// ```
    pub const fn monday_based_week(self) -> u8 {
        self.date().monday_based_week()
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::{macros::datetime, Month};
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_calendar_date(),
    ///     (2019, Month::January, 1)
    /// );
    /// ```
    pub const fn to_calendar_date(self) -> (i32, Month, u8) {
        self.date().to_calendar_date()
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_ordinal_date(),
    ///     (2019, 1)
    /// );
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        self.date().to_ordinal_date()
    }

    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::{Weekday::*, macros::datetime};
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2019, 1, Tuesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2019-10-04 0:00 UTC).to_iso_week_date(),
    ///     (2019, 40, Friday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2020, 1, Wednesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-12-31 0:00 UTC).to_iso_week_date(),
    ///     (2020, 53, Thursday)
    /// );
    /// assert_eq!(
    ///     datetime!(2021-01-01 0:00 UTC).to_iso_week_date(),
    ///     (2020, 53, Friday)
    /// );
    /// ```
    pub const fn to_iso_week_date(self) -> (i32, u8, Weekday) {
        self.date().to_iso_week_date()
    }

    /// Get the weekday of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::{Weekday::*, macros::datetime};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).weekday(), Tuesday);
    /// assert_eq!(datetime!(2019-02-01 0:00 UTC).weekday(), Friday);
    /// assert_eq!(datetime!(2019-03-01 0:00 UTC).weekday(), Friday);
    /// ```
    pub const fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Get the Julian day for the date. The time is not taken into account for this calculation.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(-4713-11-24 0:00 UTC).to_julian_day(), 0);
    /// assert_eq!(datetime!(2000-01-01 0:00 UTC).to_julian_day(), 2_451_545);
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).to_julian_day(), 2_458_485);
    /// assert_eq!(datetime!(2019-12-31 0:00 UTC).to_julian_day(), 2_458_849);
    /// ```
    pub const fn to_julian_day(self) -> i32 {
        self.date().to_julian_day()
    }
    // endregion date getters

    // region: time getters
    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00 UTC).to_hms(), (0, 0, 0));
    /// assert_eq!(datetime!(2020-01-01 23:59:59 UTC).to_hms(), (23, 59, 59));
    /// ```
    pub const fn to_hms(self) -> (u8, u8, u8) {
        self.time().as_hms()
    }

    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_milli(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999 UTC).to_hms_milli(),
    ///     (23, 59, 59, 999)
    /// );
    /// ```
    pub const fn to_hms_milli(self) -> (u8, u8, u8, u16) {
        self.time().as_hms_milli()
    }

    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_micro(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999 UTC).to_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn to_hms_micro(self) -> (u8, u8, u8, u32) {
        self.time().as_hms_micro()
    }

    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00:00 UTC).to_hms_nano(),
    ///     (0, 0, 0, 0)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999_999 UTC).to_hms_nano(),
    ///     (23, 59, 59, 999_999_999)
    /// );
    /// ```
    pub const fn to_hms_nano(self) -> (u8, u8, u8, u32) {
        self.time().as_hms_nano()
    }

    /// Get the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).hour(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(-2))
    ///         .hour(),
    ///     21,
    /// );
    /// ```
    pub const fn hour(self) -> u8 {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();
        let mut hour = self.utc_datetime.hour() as i8 + self.offset.whole_hours();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        hour.rem_euclid(24) as _
    }

    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).minute(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(+0:30))
    ///         .minute(),
    ///     29,
    /// );
    /// ```
    pub const fn minute(self) -> u8 {
        let mut second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        let mut minute = self.utc_datetime.minute() as i8 + self.offset.minutes_past_hour();

        cascade!(second in 0..60 => minute);
        minute.rem_euclid(60) as _
    }

    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).second(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59 UTC)
    ///         .to_offset(offset!(+0:00:30))
    ///         .second(),
    ///     29,
    /// );
    /// ```
    pub const fn second(self) -> u8 {
        let second = self.utc_datetime.second() as i8 + self.offset.seconds_past_minute();
        second.rem_euclid(60) as _
    }

    // Because a `UtcOffset` is limited in resolution to one second, any subsecond value will not
    // change when adjusting for the offset.

    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).millisecond(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59.999 UTC).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.utc_datetime.millisecond()
    }

    /// Get the microseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).microsecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999 UTC).microsecond(),
    ///     999_999,
    /// );
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.utc_datetime.microsecond()
    }

    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00 UTC).nanosecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999_999 UTC).nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.utc_datetime.nanosecond()
    }
    // endregion time getters
    // endregion getters

    // region: checked arithmetic
    /// Computes `self + duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time::macros::{datetime, offset};
    /// let datetime = Date::MIN.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_add((-2).days()), None);
    ///
    /// let datetime = Date::MAX.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_add(2.days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).checked_add(27.hours()),
    ///     Some(datetime!(2019 - 11 - 26 18:30 +10))
    /// );
    /// ```
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        let offset_datetime = self.utc_datetime.utc_to_offset(self.offset);
        Some(const_try_opt!(offset_datetime.checked_add(duration)).assume_offset(self.offset))
    }

    /// Computes `self - duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time::macros::{datetime, offset};
    /// let datetime = Date::MIN.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_sub(2.days()), None);
    ///
    /// let datetime = Date::MAX.midnight().assume_offset(offset!(+10));
    /// assert_eq!(datetime.checked_sub((-2).days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).checked_sub(27.hours()),
    ///     Some(datetime!(2019 - 11 - 24 12:30 +10))
    /// );
    /// ```
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        let offset_datetime = self.utc_datetime.utc_to_offset(self.offset);
        Some(const_try_opt!(offset_datetime.checked_sub(duration)).assume_offset(self.offset))
    }
    // endregion: checked arithmetic

    // region: saturating arithmetic
    /// Computes `self + duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::ext::NumericalDuration;
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(-999999 - 01 - 01 0:00 +10).saturating_add((-2).days()),
    ///     datetime!(-999999 - 01 - 01 0:00 +10)
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(+999999 - 12 - 31 23:59:59.999_999_999 +10).saturating_add(2.days()),
    ///     datetime!(+999999 - 12 - 31 23:59:59.999_999_999 +10)
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).saturating_add(27.hours()),
    ///     datetime!(2019 - 11 - 26 18:30 +10)
    /// );
    /// ```
    pub const fn saturating_add(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_add(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MIN
                .assume_utc()
                .replace_offset(self.offset)
        } else {
            PrimitiveDateTime::MAX
                .assume_utc()
                .replace_offset(self.offset)
        }
    }

    /// Computes `self - duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::ext::NumericalDuration;
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(-999999 - 01 - 01 0:00 +10).saturating_sub(2.days()),
    ///     datetime!(-999999 - 01 - 01 0:00 +10)
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(+999999 - 12 - 31 23:59:59.999_999_999 +10).saturating_sub((-2).days()),
    ///     datetime!(+999999 - 12 - 31 23:59:59.999_999_999 +10)
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30 +10).saturating_sub(27.hours()),
    ///     datetime!(2019 - 11 - 24 12:30 +10)
    /// );
    /// ```
    pub const fn saturating_sub(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_sub(duration) {
            datetime
        } else if duration.is_negative() {
            PrimitiveDateTime::MAX
                .assume_utc()
                .replace_offset(self.offset)
        } else {
            PrimitiveDateTime::MIN
                .assume_utc()
                .replace_offset(self.offset)
        }
    }
    // endregion: saturating arithmetic
}

// region: replacement
/// Methods that replace part of the `OffsetDateTime`.
impl OffsetDateTime {
    /// Replace the time, which is assumed to be in the stored offset. The date and offset
    /// components are unchanged.
    ///
    /// ```rust
    /// # use time::macros::{datetime, time};
    /// assert_eq!(
    ///     datetime!(2020-01-01 5:00 UTC).replace_time(time!(12:00)),
    ///     datetime!(2020-01-01 12:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 -5).replace_time(time!(7:00)),
    ///     datetime!(2020-01-01 7:00 -5)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 +1).replace_time(time!(12:00)),
    ///     datetime!(2020-01-01 12:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_time(self, time: Time) -> Self {
        self.utc_datetime
            .utc_to_offset(self.offset)
            .replace_time(time)
            .assume_offset(self.offset)
    }

    /// Replace the date, which is assumed to be in the stored offset. The time and offset
    /// components are unchanged.
    ///
    /// ```rust
    /// # use time::macros::{datetime, date};
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 UTC).replace_date(date!(2020-01-30)),
    ///     datetime!(2020-01-30 12:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 +1).replace_date(date!(2020-01-30)),
    ///     datetime!(2020-01-30 0:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_date(self, date: Date) -> Self {
        self.utc_datetime
            .utc_to_offset(self.offset)
            .replace_date(date)
            .assume_offset(self.offset)
    }

    /// Replace the date and time, which are assumed to be in the stored offset. The offset
    /// component remains unchanged.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 UTC).replace_date_time(datetime!(2020-01-30 16:00)),
    ///     datetime!(2020-01-30 16:00 UTC)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00 +1).replace_date_time(datetime!(2020-01-30 0:00)),
    ///     datetime!(2020-01-30 0:00 +1)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_date_time(self, date_time: PrimitiveDateTime) -> Self {
        date_time.assume_offset(self.offset)
    }

    /// Replace the offset. The date and time components remain unchanged.
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00 UTC).replace_offset(offset!(-5)),
    ///     datetime!(2020-01-01 0:00 -5)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `OffsetDateTime`."]
    pub const fn replace_offset(self, offset: UtcOffset) -> Self {
        self.utc_datetime.assume_offset(offset)
    }
}
// endregion replacement

// region: formatting & parsing
#[cfg(feature = "formatting")]
impl OffsetDateTime {
    /// Format the `OffsetDateTime` using the provided [format
    /// description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        let local = self.utc_datetime.utc_to_offset(self.offset);
        format.format_into(
            output,
            Some(local.date),
            Some(local.time),
            Some(self.offset),
        )
    }

    /// Format the `OffsetDateTime` using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::datetime};
    /// let format = format_description::parse(
    ///     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
    ///          sign:mandatory]:[offset_minute]:[offset_second]",
    /// )?;
    /// assert_eq!(
    ///     datetime!(2020-01-02 03:04:05 +06:07:08).format(&format)?,
    ///     "2020-01-02 03:04:05 +06:07:08"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(self, format: &(impl Formattable + ?Sized)) -> Result<String, error::Format> {
        let local = self.utc_datetime.utc_to_offset(self.offset);
        format.format(Some(local.date), Some(local.time), Some(self.offset))
    }
}

#[cfg(feature = "parsing")]
impl OffsetDateTime {
    /// Parse an `OffsetDateTime` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::datetime, OffsetDateTime};
    /// let format = format_description::parse(
    ///     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
    ///          sign:mandatory]:[offset_minute]:[offset_second]",
    /// )?;
    /// assert_eq!(
    ///     OffsetDateTime::parse("2020-01-02 03:04:05 +06:07:08", &format)?,
    ///     datetime!(2020-01-02 03:04:05 +06:07:08)
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_offset_date_time(input.as_bytes())
    }
}

impl fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.date(), self.time(), self.offset)
    }
}
// endregion formatting & parsing

// region: trait impls
impl PartialEq for OffsetDateTime {
    fn eq(&self, rhs: &Self) -> bool {
        self.utc_datetime.eq(&rhs.utc_datetime)
    }
}

impl PartialOrd for OffsetDateTime {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for OffsetDateTime {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.utc_datetime.cmp(&rhs.utc_datetime)
    }
}

impl Hash for OffsetDateTime {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // We need to distinguish this from a `PrimitiveDateTime`, which would otherwise conflict.
        hasher.write(b"OffsetDateTime");
        self.utc_datetime.hash(hasher);
    }
}

impl<T> Add<T> for OffsetDateTime
where
    PrimitiveDateTime: Add<T, Output = PrimitiveDateTime>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        (self.utc_datetime + rhs)
            .assume_utc()
            .to_offset(self.offset)
    }
}

impl_add_assign!(OffsetDateTime: Duration, StdDuration);

impl<T> Sub<T> for OffsetDateTime
where
    PrimitiveDateTime: Sub<T, Output = PrimitiveDateTime>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        (self.utc_datetime - rhs)
            .assume_utc()
            .to_offset(self.offset)
    }
}

impl_sub_assign!(OffsetDateTime: Duration, StdDuration);

impl Sub for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.utc_datetime - rhs.utc_datetime
    }
}

#[cfg(feature = "std")]
impl Add<Duration> for SystemTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_zero() {
            self
        } else if duration.is_positive() {
            self + duration.abs_std()
        } else {
            debug_assert!(duration.is_negative());
            self - duration.abs_std()
        }
    }
}

impl_add_assign!(SystemTime: #[cfg(feature = "std")] Duration);

#[cfg(feature = "std")]
impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        (OffsetDateTime::from(self) - duration).into()
    }
}

impl_sub_assign!(SystemTime: #[cfg(feature = "std")] Duration);

#[cfg(feature = "std")]
impl Sub<SystemTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: SystemTime) -> Self::Output {
        self - Self::from(rhs)
    }
}

#[cfg(feature = "std")]
impl Sub<OffsetDateTime> for SystemTime {
    type Output = Duration;

    fn sub(self, rhs: OffsetDateTime) -> Self::Output {
        OffsetDateTime::from(self) - rhs
    }
}

#[cfg(feature = "std")]
impl PartialEq<SystemTime> for OffsetDateTime {
    fn eq(&self, rhs: &SystemTime) -> bool {
        self == &Self::from(*rhs)
    }
}

#[cfg(feature = "std")]
impl PartialEq<OffsetDateTime> for SystemTime {
    fn eq(&self, rhs: &OffsetDateTime) -> bool {
        &OffsetDateTime::from(*self) == rhs
    }
}

#[cfg(feature = "std")]
impl PartialOrd<SystemTime> for OffsetDateTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*other))
    }
}

#[cfg(feature = "std")]
impl PartialOrd<OffsetDateTime> for SystemTime {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        OffsetDateTime::from(*self).partial_cmp(other)
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for OffsetDateTime {
    fn from(system_time: SystemTime) -> Self {
        match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Self::UNIX_EPOCH + duration,
            Err(err) => Self::UNIX_EPOCH - err.duration(),
        }
    }
}

#[allow(clippy::fallible_impl_from)] // caused by `debug_assert!`
#[cfg(feature = "std")]
impl From<OffsetDateTime> for SystemTime {
    fn from(datetime: OffsetDateTime) -> Self {
        let duration = datetime - OffsetDateTime::UNIX_EPOCH;

        if duration.is_zero() {
            Self::UNIX_EPOCH
        } else if duration.is_positive() {
            Self::UNIX_EPOCH + duration.abs_std()
        } else {
            debug_assert!(duration.is_negative());
            Self::UNIX_EPOCH - duration.abs_std()
        }
    }
}
// endregion trait impls

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialOrd;
	use std::cmp::Eq;
	use std::ops::Sub;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2188() {
    rusty_monitor::set_test_id(2188);
    let mut i64_0: i64 = -28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 122i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 4u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 53u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 44u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -61i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut month_0: month::Month = crate::month::Month::January;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_0_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(offsetdatetime_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut tuple_1: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u32_0: u32 = 53u32;
    let mut i8_0: i8 = 39i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 42u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i32_0: i32 = 148i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = -67i8;
    let mut i8_5: i8 = 19i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(offsetdatetime_3, offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_501() {
    rusty_monitor::set_test_id(501);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 17i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = 33i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i64_1: i64 = -34i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_2);
    let mut offsetdatetime_5_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_5;
    let mut i64_2: i64 = -241i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_1: i32 = -1i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_6, date_4);
    let mut offsetdatetime_7_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_7;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(offsetdatetime_7_ref_0, offsetdatetime_5_ref_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1192() {
    rusty_monitor::set_test_id(1192);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = 27i32;
    let mut i64_0: i64 = -286i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i8_0: i8 = 79i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = -84i8;
    let mut i8_3: i8 = 78i8;
    let mut i8_4: i8 = 16i8;
    let mut i8_5: i8 = 22i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_2_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = std::clone::Clone::clone(offsetdatetime_2_ref_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_3);
    panic!("From RustyUnit with love");
}
}