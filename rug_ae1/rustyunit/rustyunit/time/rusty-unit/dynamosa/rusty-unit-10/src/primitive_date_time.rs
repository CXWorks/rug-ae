//! The [`PrimitiveDateTime`] struct and its associated `impl`s.

use core::fmt;
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;

#[cfg(any(feature = "formatting", feature = "parsing"))]
use crate::error;
#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::{util, Date, Duration, Month, OffsetDateTime, Time, UtcOffset, Weekday};

/// Combined date and time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrimitiveDateTime {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) date: Date,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) time: Time,
}

impl PrimitiveDateTime {
    /// The smallest value that can be represented by `PrimitiveDateTime`.
    ///
    /// Depending on `large-dates` feature flag, value of this constant may vary.
    ///
    /// 1. With `large-dates` disabled it is equal to `-9999 - 01 - 01 00:00:00.0`
    /// 2. With `large-dates` enabled it is equal to `-999999 - 01 - 01 00:00:00.0`
    ///
    /// ```rust
    /// # use time::{PrimitiveDateTime, macros::datetime};
    /// // Assuming `large-dates` feature is enabled.
    /// assert_eq!(PrimitiveDateTime::MIN, datetime!(-999999 - 01 - 01 0:00));
    /// ```
    pub const MIN: Self = Self::new(Date::MIN, Time::MIN);

    /// The largest value that can be represented by `PrimitiveDateTime`.
    ///
    /// Depending on `large-dates` feature flag, value of this constant may vary.
    ///
    /// 1. With `large-dates` disabled it is equal to `9999 - 12 - 31 23:59:59.999_999_999`
    /// 2. With `large-dates` enabled it is equal to `999999 - 12 - 31 23:59:59.999_999_999`
    ///
    /// ```rust
    /// # use time::{PrimitiveDateTime, macros::datetime};
    /// // Assuming `large-dates` feature is enabled.
    /// assert_eq!(PrimitiveDateTime::MAX, datetime!(+999999 - 12 - 31 23:59:59.999_999_999));
    /// ```
    pub const MAX: Self = Self::new(Date::MAX, Time::MAX);

    /// Create a new `PrimitiveDateTime` from the provided [`Date`] and [`Time`].
    ///
    /// ```rust
    /// # use time::{PrimitiveDateTime, macros::{date, datetime, time}};
    /// assert_eq!(
    ///     PrimitiveDateTime::new(date!(2019-01-01), time!(0:00)),
    ///     datetime!(2019-01-01 0:00),
    /// );
    /// ```
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    // region: component getters
    /// Get the [`Date`] component of the `PrimitiveDateTime`.
    ///
    /// ```rust
    /// # use time::macros::{date, datetime};
    /// assert_eq!(datetime!(2019-01-01 0:00).date(), date!(2019-01-01));
    /// ```
    pub const fn date(self) -> Date {
        self.date
    }

    /// Get the [`Time`] component of the `PrimitiveDateTime`.
    ///
    /// ```rust
    /// # use time::macros::{datetime, time};
    /// assert_eq!(datetime!(2019-01-01 0:00).time(), time!(0:00));
    pub const fn time(self) -> Time {
        self.time
    }
    // endregion component getters

    // region: date getters
    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).year(), 2019);
    /// assert_eq!(datetime!(2019-12-31 0:00).year(), 2019);
    /// assert_eq!(datetime!(2020-01-01 0:00).year(), 2020);
    /// ```
    pub const fn year(self) -> i32 {
        self.date.year()
    }

    /// Get the month of the date.
    ///
    /// ```rust
    /// # use time::{macros::datetime, Month};
    /// assert_eq!(datetime!(2019-01-01 0:00).month(), Month::January);
    /// assert_eq!(datetime!(2019-12-31 0:00).month(), Month::December);
    /// ```
    pub const fn month(self) -> Month {
        self.date.month()
    }

    /// Get the day of the date.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).day(), 1);
    /// assert_eq!(datetime!(2019-12-31 0:00).day(), 31);
    /// ```
    pub const fn day(self) -> u8 {
        self.date.day()
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for common years).
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).ordinal(), 1);
    /// assert_eq!(datetime!(2019-12-31 0:00).ordinal(), 365);
    /// ```
    pub const fn ordinal(self) -> u16 {
        self.date.ordinal()
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).iso_week(), 1);
    /// assert_eq!(datetime!(2019-10-04 0:00).iso_week(), 40);
    /// assert_eq!(datetime!(2020-01-01 0:00).iso_week(), 1);
    /// assert_eq!(datetime!(2020-12-31 0:00).iso_week(), 53);
    /// assert_eq!(datetime!(2021-01-01 0:00).iso_week(), 53);
    /// ```
    pub const fn iso_week(self) -> u8 {
        self.date.iso_week()
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00).sunday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00).sunday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00).sunday_based_week(), 0);
    /// ```
    pub const fn sunday_based_week(self) -> u8 {
        self.date.sunday_based_week()
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-01-01 0:00).monday_based_week(), 0);
    /// assert_eq!(datetime!(2020-12-31 0:00).monday_based_week(), 52);
    /// assert_eq!(datetime!(2021-01-01 0:00).monday_based_week(), 0);
    /// ```
    pub const fn monday_based_week(self) -> u8 {
        self.date.monday_based_week()
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::{macros::datetime, Month};
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00).to_calendar_date(),
    ///     (2019, Month::January, 1)
    /// );
    /// ```
    pub const fn to_calendar_date(self) -> (i32, Month, u8) {
        self.date.to_calendar_date()
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).to_ordinal_date(), (2019, 1));
    /// ```
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        self.date.to_ordinal_date()
    }

    /// Get the ISO 8601 year, week number, and weekday.
    ///
    /// ```rust
    /// # use time::{Weekday::*, macros::datetime};
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00).to_iso_week_date(),
    ///     (2019, 1, Tuesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2019-10-04 0:00).to_iso_week_date(),
    ///     (2019, 40, Friday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-01-01 0:00).to_iso_week_date(),
    ///     (2020, 1, Wednesday)
    /// );
    /// assert_eq!(
    ///     datetime!(2020-12-31 0:00).to_iso_week_date(),
    ///     (2020, 53, Thursday)
    /// );
    /// assert_eq!(
    ///     datetime!(2021-01-01 0:00).to_iso_week_date(),
    ///     (2020, 53, Friday)
    /// );
    /// ```
    pub const fn to_iso_week_date(self) -> (i32, u8, Weekday) {
        self.date.to_iso_week_date()
    }

    /// Get the weekday.
    ///
    /// ```rust
    /// # use time::{Weekday::*, macros::datetime};
    /// assert_eq!(datetime!(2019-01-01 0:00).weekday(), Tuesday);
    /// assert_eq!(datetime!(2019-02-01 0:00).weekday(), Friday);
    /// assert_eq!(datetime!(2019-03-01 0:00).weekday(), Friday);
    /// assert_eq!(datetime!(2019-04-01 0:00).weekday(), Monday);
    /// assert_eq!(datetime!(2019-05-01 0:00).weekday(), Wednesday);
    /// assert_eq!(datetime!(2019-06-01 0:00).weekday(), Saturday);
    /// assert_eq!(datetime!(2019-07-01 0:00).weekday(), Monday);
    /// assert_eq!(datetime!(2019-08-01 0:00).weekday(), Thursday);
    /// assert_eq!(datetime!(2019-09-01 0:00).weekday(), Sunday);
    /// assert_eq!(datetime!(2019-10-01 0:00).weekday(), Tuesday);
    /// assert_eq!(datetime!(2019-11-01 0:00).weekday(), Friday);
    /// assert_eq!(datetime!(2019-12-01 0:00).weekday(), Sunday);
    /// ```
    pub const fn weekday(self) -> Weekday {
        self.date.weekday()
    }

    /// Get the Julian day for the date. The time is not taken into account for this calculation.
    ///
    /// The algorithm to perform this conversion is derived from one provided by Peter Baum; it is
    /// freely available [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(-4713-11-24 0:00).to_julian_day(), 0);
    /// assert_eq!(datetime!(2000-01-01 0:00).to_julian_day(), 2_451_545);
    /// assert_eq!(datetime!(2019-01-01 0:00).to_julian_day(), 2_458_485);
    /// assert_eq!(datetime!(2019-12-31 0:00).to_julian_day(), 2_458_849);
    /// ```
    pub const fn to_julian_day(self) -> i32 {
        self.date.to_julian_day()
    }
    // endregion date getters

    // region: time getters
    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00).as_hms(), (0, 0, 0));
    /// assert_eq!(datetime!(2020-01-01 23:59:59).as_hms(), (23, 59, 59));
    /// ```
    pub const fn as_hms(self) -> (u8, u8, u8) {
        self.time.as_hms()
    }

    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00).as_hms_milli(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999).as_hms_milli(),
    ///     (23, 59, 59, 999)
    /// );
    /// ```
    pub const fn as_hms_milli(self) -> (u8, u8, u8, u16) {
        self.time.as_hms_milli()
    }

    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00).as_hms_micro(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999).as_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn as_hms_micro(self) -> (u8, u8, u8, u32) {
        self.time.as_hms_micro()
    }

    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2020-01-01 0:00:00).as_hms_nano(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     datetime!(2020-01-01 23:59:59.999_999_999).as_hms_nano(),
    ///     (23, 59, 59, 999_999_999)
    /// );
    /// ```
    pub const fn as_hms_nano(self) -> (u8, u8, u8, u32) {
        self.time.as_hms_nano()
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).hour(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59).hour(), 23);
    /// ```
    pub const fn hour(self) -> u8 {
        self.time.hour()
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).minute(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59).minute(), 59);
    /// ```
    pub const fn minute(self) -> u8 {
        self.time.minute()
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).second(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59).second(), 59);
    /// ```
    pub const fn second(self) -> u8 {
        self.time.second()
    }

    /// Get the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).millisecond(), 0);
    /// assert_eq!(datetime!(2019-01-01 23:59:59.999).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        self.time.millisecond()
    }

    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).microsecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999).microsecond(),
    ///     999_999
    /// );
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.time.microsecond()
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(datetime!(2019-01-01 0:00).nanosecond(), 0);
    /// assert_eq!(
    ///     datetime!(2019-01-01 23:59:59.999_999_999).nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.time.nanosecond()
    }
    // endregion time getters

    // region: attach offset
    /// Assuming that the existing `PrimitiveDateTime` represents a moment in the provided
    /// [`UtcOffset`], return an [`OffsetDateTime`].
    ///
    /// ```rust
    /// # use time::macros::{datetime, offset};
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00)
    ///         .assume_offset(offset!(UTC))
    ///         .unix_timestamp(),
    ///     1_546_300_800,
    /// );
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00)
    ///         .assume_offset(offset!(-1))
    ///         .unix_timestamp(),
    ///     1_546_304_400,
    /// );
    /// ```
    pub const fn assume_offset(self, offset: UtcOffset) -> OffsetDateTime {
        OffsetDateTime {
            utc_datetime: self.offset_to_utc(offset),
            offset,
        }
    }

    /// Assuming that the existing `PrimitiveDateTime` represents a moment in UTC, return an
    /// [`OffsetDateTime`].
    ///
    /// ```rust
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     datetime!(2019-01-01 0:00).assume_utc().unix_timestamp(),
    ///     1_546_300_800,
    /// );
    /// ```
    pub const fn assume_utc(self) -> OffsetDateTime {
        OffsetDateTime {
            utc_datetime: self,
            offset: UtcOffset::UTC,
        }
    }
    // endregion attach offset

    // region: checked arithmetic
    /// Computes `self + duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time::macros::datetime;
    /// let datetime = Date::MIN.midnight();
    /// assert_eq!(datetime.checked_add((-2).days()), None);
    ///
    /// let datetime = Date::MAX.midnight();
    /// assert_eq!(datetime.checked_add(1.days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30).checked_add(27.hours()),
    ///     Some(datetime!(2019 - 11 - 26 18:30))
    /// );
    /// ```
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        let (date_adjustment, time) = self.time.adjusting_add(duration);
        let date = const_try_opt!(self.date.checked_add(duration));

        Some(Self {
            date: match date_adjustment {
                util::DateAdjustment::Previous => const_try_opt!(date.previous_day()),
                util::DateAdjustment::Next => const_try_opt!(date.next_day()),
                util::DateAdjustment::None => date,
            },
            time,
        })
    }

    /// Computes `self - duration`, returning `None` if an overflow occurred.
    ///
    /// ```
    /// # use time::{Date, ext::NumericalDuration};
    /// # use time::macros::datetime;
    /// let datetime = Date::MIN.midnight();
    /// assert_eq!(datetime.checked_sub(2.days()), None);
    ///
    /// let datetime = Date::MAX.midnight();
    /// assert_eq!(datetime.checked_sub((-1).days()), None);
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30).checked_sub(27.hours()),
    ///     Some(datetime!(2019 - 11 - 24 12:30))
    /// );
    /// ```
    pub const fn checked_sub(self, duration: Duration) -> Option<Self> {
        let (date_adjustment, time) = self.time.adjusting_sub(duration);
        let date = const_try_opt!(self.date.checked_sub(duration));

        Some(Self {
            date: match date_adjustment {
                util::DateAdjustment::Previous => const_try_opt!(date.previous_day()),
                util::DateAdjustment::Next => const_try_opt!(date.next_day()),
                util::DateAdjustment::None => date,
            },
            time,
        })
    }
    // endregion: checked arithmetic

    // region: saturating arithmetic
    /// Computes `self + duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::{PrimitiveDateTime, ext::NumericalDuration};
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     PrimitiveDateTime::MIN.saturating_add((-2).days()),
    ///     PrimitiveDateTime::MIN
    /// );
    ///
    /// assert_eq!(
    ///     PrimitiveDateTime::MAX.saturating_add(2.days()),
    ///     PrimitiveDateTime::MAX
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30).saturating_add(27.hours()),
    ///     datetime!(2019 - 11 - 26 18:30)
    /// );
    /// ```
    pub const fn saturating_add(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_add(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MIN
        } else {
            Self::MAX
        }
    }

    /// Computes `self - duration`, saturating value on overflow.
    ///
    /// ```
    /// # use time::{PrimitiveDateTime, ext::NumericalDuration};
    /// # use time::macros::datetime;
    /// assert_eq!(
    ///     PrimitiveDateTime::MIN.saturating_sub(2.days()),
    ///     PrimitiveDateTime::MIN
    /// );
    ///
    /// assert_eq!(
    ///     PrimitiveDateTime::MAX.saturating_sub((-2).days()),
    ///     PrimitiveDateTime::MAX
    /// );
    ///
    /// assert_eq!(
    ///     datetime!(2019 - 11 - 25 15:30).saturating_sub(27.hours()),
    ///     datetime!(2019 - 11 - 24 12:30)
    /// );
    /// ```
    pub const fn saturating_sub(self, duration: Duration) -> Self {
        if let Some(datetime) = self.checked_sub(duration) {
            datetime
        } else if duration.is_negative() {
            Self::MAX
        } else {
            Self::MIN
        }
    }
    // endregion: saturating arithmetic
}

// region: replacement
/// Methods that replace part of the `PrimitiveDateTime`.
impl PrimitiveDateTime {
    /// Replace the time, preserving the date.
    ///
    /// ```rust
    /// # use time::macros::{datetime, time};
    /// assert_eq!(
    ///     datetime!(2020-01-01 17:00).replace_time(time!(5:00)),
    ///     datetime!(2020-01-01 5:00)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `PrimitiveDateTime`."]
    pub const fn replace_time(self, time: Time) -> Self {
        self.date.with_time(time)
    }

    /// Replace the date, preserving the time.
    ///
    /// ```rust
    /// # use time::macros::{datetime, date};
    /// assert_eq!(
    ///     datetime!(2020-01-01 12:00).replace_date(date!(2020-01-30)),
    ///     datetime!(2020-01-30 12:00)
    /// );
    /// ```
    #[must_use = "This method does not mutate the original `PrimitiveDateTime`."]
    pub const fn replace_date(self, date: Date) -> Self {
        date.with_time(self.time)
    }
}
// endregion replacement

// region: offset conversion helpers
/// Helper methods to adjust a [`PrimitiveDateTime`] to a given [`UtcOffset`].
impl PrimitiveDateTime {
    /// Assuming that the current [`PrimitiveDateTime`] is a value in the provided [`UtcOffset`],
    /// obtain the equivalent value in the UTC.
    pub(crate) const fn offset_to_utc(self, offset: UtcOffset) -> Self {
        let mut second = self.second() as i8 - offset.seconds_past_minute();
        let mut minute = self.minute() as i8 - offset.minutes_past_hour();
        let mut hour = self.hour() as i8 - offset.whole_hours();
        let (mut year, mut ordinal) = self.date.to_ordinal_date();

        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        cascade!(hour in 0..24 => ordinal);
        cascade!(ordinal => year);

        Self {
            date: Date::__from_ordinal_date_unchecked(year, ordinal),
            time: Time::__from_hms_nanos_unchecked(
                hour as _,
                minute as _,
                second as _,
                self.nanosecond(),
            ),
        }
    }

    /// Assuming that the current [`PrimitiveDateTime`] is a value in UTC, obtain the equivalent
    /// value in the provided [`UtcOffset`].
    pub(crate) const fn utc_to_offset(self, offset: UtcOffset) -> Self {
        self.offset_to_utc(UtcOffset::__from_hms_unchecked(
            -offset.whole_hours(),
            -offset.minutes_past_hour(),
            -offset.seconds_past_minute(),
        ))
    }
}
// endregion offset conversion helpers

// region: formatting & parsing
#[cfg(feature = "formatting")]
impl PrimitiveDateTime {
    /// Format the `PrimitiveDateTime` using the provided [format
    /// description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, error::Format> {
        format.format_into(output, Some(self.date), Some(self.time), None)
    }

    /// Format the `PrimitiveDateTime` using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::datetime};
    /// let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    /// assert_eq!(
    ///     datetime!(2020-01-02 03:04:05).format(&format)?,
    ///     "2020-01-02 03:04:05"
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(self, format: &(impl Formattable + ?Sized)) -> Result<String, error::Format> {
        format.format(Some(self.date), Some(self.time), None)
    }
}

#[cfg(feature = "parsing")]
impl PrimitiveDateTime {
    /// Parse a `PrimitiveDateTime` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::datetime, PrimitiveDateTime};
    /// let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    /// assert_eq!(
    ///     PrimitiveDateTime::parse("2020-01-02 03:04:05", &format)?,
    ///     datetime!(2020-01-02 03:04:05)
    /// );
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_date_time(input.as_bytes())
    }
}

impl fmt::Display for PrimitiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
// endregion formatting & parsing

// region: trait impls
impl Add<Duration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration)
            .expect("resulting value is out of range")
    }
}

impl Add<StdDuration> for PrimitiveDateTime {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        let (is_next_day, time) = self.time.adjusting_add_std(duration);

        Self {
            date: if is_next_day {
                (self.date + duration)
                    .next_day()
                    .expect("resulting value is out of range")
            } else {
                self.date + duration
            },
            time,
        }
    }
}

impl_add_assign!(PrimitiveDateTime: Duration, StdDuration);

impl Sub<Duration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration)
            .expect("resulting value is out of range")
    }
}

impl Sub<StdDuration> for PrimitiveDateTime {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        let (is_previous_day, time) = self.time.adjusting_sub_std(duration);

        Self {
            date: if is_previous_day {
                (self.date - duration)
                    .previous_day()
                    .expect("resulting value is out of range")
            } else {
                self.date - duration
            },
            time,
        }
    }
}

impl_sub_assign!(PrimitiveDateTime: Duration, StdDuration);

impl Sub for PrimitiveDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.date - rhs.date) + (self.time - rhs.time)
    }
}
// endregion trait impls

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialEq;
	use std::ops::Add;
	use std::ops::Sub;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6012() {
    rusty_monitor::set_test_id(6012);
    let mut i8_0: i8 = 66i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 26i8;
    let mut i8_4: i8 = -122i8;
    let mut i8_5: i8 = -33i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = -17i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i64_0: i64 = -123i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 237i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = 254i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut i8_6: i8 = 30i8;
    let mut i8_7: i8 = -4i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 66i8;
    let mut i8_10: i8 = -107i8;
    let mut i8_11: i8 = 73i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = 154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = 22i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u16_0: u16 = 22u16;
    let mut i32_2: i32 = 81i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_4, duration_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_5, utcoffset_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_0: u8 = 46u8;
    let mut i32_3: i32 = 35i32;
    let mut i8_12: i8 = -96i8;
    let mut i8_13: i8 = 66i8;
    let mut i8_14: i8 = 55i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_6, utcoffset_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_7);
    let mut i8_15: i8 = -81i8;
    let mut i8_16: i8 = 64i8;
    let mut i8_17: i8 = 41i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 78u8;
    let mut u8_3: u8 = 11u8;
    let mut i64_4: i64 = -91i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_4: i32 = -100i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_8, duration_7);
    let mut primitivedatetime_9_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_9;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_5: i32 = -68i32;
    let mut i64_5: i64 = -104i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut u32_0: u32 = 76u32;
    let mut u8_4: u8 = 57u8;
    let mut u8_5: u8 = 36u8;
    let mut u8_6: u8 = 44u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_0);
    let mut i32_6: i32 = -52i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_10: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_2};
    let mut primitivedatetime_11: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_10, duration_11);
    let mut primitivedatetime_11_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_11;
    let mut i32_7: i32 = -21i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_7);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_11_ref_0, primitivedatetime_9_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_3, u8_2, u8_1);
    let mut i8_18: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_5);
    let mut u32_1: u32 = crate::time::Time::microsecond(time_1);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_0, weekday_0);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    let mut primitivedatetime_12: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2054() {
    rusty_monitor::set_test_id(2054);
    let mut i64_0: i64 = -14i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 113i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut i64_1: i64 = -14i64;
    let mut u32_0: u32 = 92u32;
    let mut i32_1: i32 = -123i32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 0u8;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 54u8;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 78u8;
    let mut u8_8: u8 = 11u8;
    let mut i64_2: i64 = -91i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = -100i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_2, duration_3);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -68i32;
    let mut i64_3: i64 = -104i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u32_1: u32 = 76u32;
    let mut u8_9: u8 = 57u8;
    let mut u8_10: u8 = 36u8;
    let mut u8_11: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_1);
    let mut i32_4: i32 = -52i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_4, duration_7);
    let mut primitivedatetime_5_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_5;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_5_ref_0, primitivedatetime_3_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u8_12: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut i64_0: i64 = 73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_0: i32 = -14i32;
    let mut i32_1: i32 = -78i32;
    let mut i64_1: i64 = -229i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_2: i64 = -51i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = -15i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = 35i8;
    let mut i8_1: i8 = -48i8;
    let mut i8_2: i8 = 29i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_4);
    let mut bool_0: bool = true;
    let mut i64_3: i64 = -15i64;
    let mut i64_4: i64 = -148i64;
    let mut i64_5: i64 = -105i64;
    let mut str_0: &str = "Esk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_0};
    let mut f32_0: f32 = 129.785333f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_3);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7295() {
    rusty_monitor::set_test_id(7295);
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 41u16;
    let mut i32_0: i32 = 40i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 16u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 3u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut u32_1: u32 = 92u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 92u8;
    let mut u8_5: u8 = 35u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 102i8;
    let mut i8_1: i8 = 61i8;
    let mut i8_2: i8 = 8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(primitivedatetime_1, primitivedatetime_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 3i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u32_2: u32 = 6u32;
    let mut u8_6: u8 = 45u8;
    let mut u8_7: u8 = 92u8;
    let mut u8_8: u8 = 86u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_1: u16 = 39u16;
    let mut i32_2: i32 = 176i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_3};
    let mut i8_3: i8 = 51i8;
    let mut i8_4: i8 = -96i8;
    let mut i8_5: i8 = 26i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 70i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_4);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut i64_2: i64 = 107i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_8, duration_5);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_1);
    let mut i64_3: i64 = -91i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_3: i32 = -100i32;
    let mut date_7: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_5, duration_7);
    let mut primitivedatetime_6_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_6;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_4: i32 = -68i32;
    let mut i64_4: i64 = -104i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut u8_9: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_4);
    let mut u8_10: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_2);
    let mut u8_11: u8 = crate::date::Date::iso_week(date_3);
    let mut i64_5: i64 = crate::duration::Duration::whole_minutes(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8751() {
    rusty_monitor::set_test_id(8751);
    let mut i64_0: i64 = 359i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f64_0: f64 = 156.344580f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 74u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 85u16;
    let mut i32_1: i32 = -59i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut u32_1: u32 = 92u32;
    let mut i32_2: i32 = -123i32;
    let mut u8_3: u8 = 26u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 0u8;
    let mut u8_6: u8 = 72u8;
    let mut u8_7: u8 = 75u8;
    let mut u8_8: u8 = 54u8;
    let mut u8_9: u8 = 1u8;
    let mut u8_10: u8 = 78u8;
    let mut u8_11: u8 = 11u8;
    let mut i64_1: i64 = -91i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_3: i32 = -100i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_2, duration_3);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_4: i32 = -68i32;
    let mut i64_2: i64 = -104i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_5: i32 = -52i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_11, u8_10, u8_9);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3912() {
    rusty_monitor::set_test_id(3912);
    let mut i32_0: i32 = -83i32;
    let mut i64_0: i64 = -65i64;
    let mut i64_1: i64 = 168i64;
    let mut u32_0: u32 = 92u32;
    let mut i32_1: i32 = -123i32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 0u8;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 54u8;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 78u8;
    let mut u8_8: u8 = 11u8;
    let mut i64_2: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_2: i32 = -100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -68i32;
    let mut i64_3: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_1: u32 = 76u32;
    let mut u8_9: u8 = 57u8;
    let mut u8_10: u8 = 36u8;
    let mut u8_11: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_1);
    let mut i32_4: i32 = -52i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_5);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_5: i32 = crate::duration::Duration::subsec_microseconds(duration_6);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_593() {
    rusty_monitor::set_test_id(593);
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 11u8;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_1: i32 = -68i32;
    let mut i64_1: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut u32_0: u32 = 76u32;
    let mut u8_3: u8 = 57u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_2: i32 = -52i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i32_3: i32 = -21i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7565() {
    rusty_monitor::set_test_id(7565);
    let mut u32_0: u32 = 92u32;
    let mut i32_0: i32 = -123i32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 0u8;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 54u8;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 78u8;
    let mut u8_8: u8 = 11u8;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = -100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = -68i32;
    let mut i64_1: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = -52i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7780() {
    rusty_monitor::set_test_id(7780);
    let mut i64_0: i64 = -167i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 50u16;
    let mut i32_0: i32 = -9i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut u32_1: u32 = 71u32;
    let mut u8_3: u8 = 69u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 33u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 38u32;
    let mut u8_6: u8 = 30u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 32u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_1: i64 = 171i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_1: i32 = 128i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut u32_3: u32 = 92u32;
    let mut i32_2: i32 = -123i32;
    let mut u8_9: u8 = 26u8;
    let mut u8_10: u8 = 28u8;
    let mut u8_11: u8 = 0u8;
    let mut u8_12: u8 = 72u8;
    let mut u8_13: u8 = 75u8;
    let mut u8_14: u8 = 54u8;
    let mut u8_15: u8 = 1u8;
    let mut u8_16: u8 = 78u8;
    let mut u8_17: u8 = 11u8;
    let mut i64_2: i64 = -91i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i32_3: i32 = -100i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_4, duration_4);
    let mut primitivedatetime_5_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_5;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_4: i32 = -68i32;
    let mut i64_3: i64 = -104i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_4: u32 = 76u32;
    let mut u8_18: u8 = 57u8;
    let mut u8_19: u8 = 36u8;
    let mut u8_20: u8 = 44u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_4);
    let mut i32_5: i32 = -52i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_3};
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_6, duration_8);
    let mut primitivedatetime_7_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_7;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_7_ref_0, primitivedatetime_5_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_17, u8_16, u8_15);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_14, u8_13, u8_12);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_11, u8_10, u8_9, u32_3);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_533() {
    rusty_monitor::set_test_id(533);
    let mut i32_0: i32 = 14i32;
    let mut i64_0: i64 = 180i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 49u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 12i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_0);
    let mut i64_1: i64 = -108i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i8_0: i8 = -23i8;
    let mut i8_1: i8 = -10i8;
    let mut i8_2: i8 = -53i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 62u32;
    let mut u8_3: u8 = 98u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 54u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 81u32;
    let mut u8_6: u8 = 63u8;
    let mut u8_7: u8 = 23u8;
    let mut u8_8: u8 = 25u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = -119i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i64_2: i64 = -51i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = 112i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1278() {
    rusty_monitor::set_test_id(1278);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = -71i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut i32_1: i32 = 33i32;
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 0u8;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 54u8;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 78u8;
    let mut u8_8: u8 = 11u8;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_2: i32 = -100i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -68i32;
    let mut i64_1: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_1: u32 = 76u32;
    let mut u8_9: u8 = 57u8;
    let mut u8_10: u8 = 36u8;
    let mut u8_11: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_1);
    let mut i32_4: i32 = -52i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_5);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1343() {
    rusty_monitor::set_test_id(1343);
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -12i32;
    let mut i64_0: i64 = 113i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -113i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 26u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 0u8;
    let mut u8_6: u8 = 72u8;
    let mut u8_7: u8 = 75u8;
    let mut u8_8: u8 = 54u8;
    let mut u8_9: u8 = 1u8;
    let mut u8_10: u8 = 78u8;
    let mut u8_11: u8 = 11u8;
    let mut i64_1: i64 = -91i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_2: i32 = -100i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_1, duration_2);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -68i32;
    let mut i64_2: i64 = -104i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut u32_2: u32 = 76u32;
    let mut u8_12: u8 = 57u8;
    let mut u8_13: u8 = 36u8;
    let mut u8_14: u8 = 44u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_2);
    let mut i32_4: i32 = -52i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_3, duration_6);
    let mut primitivedatetime_4_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_4;
    let mut i32_5: i32 = -21i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_5);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_4_ref_0, primitivedatetime_2_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_11, u8_10, u8_9);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut i32_6: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut i64_0: i64 = -30i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_1: i64 = -10i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 37i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = -109i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_1: i32 = -57i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8638() {
    rusty_monitor::set_test_id(8638);
    let mut i64_0: i64 = -157i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = -93.234751f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_0: f32 = -17.867808f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i32_0: i32 = -159i32;
    let mut i64_1: i64 = -100i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_5);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = 74i8;
    let mut i8_2: i8 = -18i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -221i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 145i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut u16_0: u16 = 85u16;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 38u8;
    let mut i64_3: i64 = -37i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -163i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_8, duration_7);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_3: u8 = 46u8;
    let mut i32_3: i32 = 35i32;
    let mut i8_3: i8 = -96i8;
    let mut i8_4: i8 = 66i8;
    let mut i8_5: i8 = 55i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_5, utcoffset_1);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_6);
    let mut i8_6: i8 = -81i8;
    let mut i8_7: i8 = 64i8;
    let mut i8_8: i8 = 41i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 78u8;
    let mut u8_6: u8 = 8u8;
    let mut i64_5: i64 = -91i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i32_4: i32 = -100i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_7, duration_11);
    let mut primitivedatetime_8_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_5: i32 = -68i32;
    let mut i64_6: i64 = -104i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_5);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut i32_6: i32 = -52i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut i32_7: i32 = -21i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_7);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_6, u8_5, u8_4);
    let mut i8_9: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_2);
    let mut u32_0: u32 = crate::time::Time::microsecond(time_1);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_3, weekday_0);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7652() {
    rusty_monitor::set_test_id(7652);
    let mut i128_0: i128 = 144i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -114i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = 58i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut u32_0: u32 = 56u32;
    let mut u8_0: u8 = 91u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -110i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i32_1: i32 = -117i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i64_2: i64 = 97i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_2: i32 = 49i32;
    let mut f64_0: f64 = -36.752722f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_2);
    let mut i32_3: i32 = -169i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_7);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7359() {
    rusty_monitor::set_test_id(7359);
    let mut i8_0: i8 = -33i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = -34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 3i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_0: u32 = 92u32;
    let mut i32_1: i32 = -123i32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 0u8;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 54u8;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 78u8;
    let mut u8_8: u8 = 11u8;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_2: i32 = -100i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_1, duration_1);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -68i32;
    let mut i64_1: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_1: u32 = 76u32;
    let mut u8_9: u8 = 57u8;
    let mut u8_10: u8 = 36u8;
    let mut u8_11: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_1);
    let mut i32_4: i32 = -52i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_3, duration_5);
    let mut primitivedatetime_4_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_4;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_4_ref_0, primitivedatetime_2_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_8, u8_7, u8_6);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_913() {
    rusty_monitor::set_test_id(913);
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 11u8;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_1: i32 = -68i32;
    let mut i64_1: i64 = -104i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_0: u32 = 76u32;
    let mut u8_3: u8 = 57u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_2: i32 = -52i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_5);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i32_3: i32 = -21i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}
}