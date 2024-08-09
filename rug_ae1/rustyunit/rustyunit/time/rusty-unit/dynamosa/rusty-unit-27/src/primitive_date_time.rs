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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::ops::Sub;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2539() {
    rusty_monitor::set_test_id(2539);
    let mut f32_0: f32 = 84.241361f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -19i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 50u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 29u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i64_0: i64 = 88i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut u16_0: u16 = 36u16;
    let mut i32_1: i32 = -10i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_2);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i32_2: i32 = -119i32;
    let mut i64_1: i64 = 88i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_4);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_7, utcoffset_0);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_4, duration_3);
    let mut i64_3: i64 = 128i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_3: i32 = -91i32;
    let mut i64_4: i64 = -63i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_9, duration_7);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_10);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 78i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_11, utcoffset_2);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_12);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 2u16;
    let mut i32_4: i32 = -72i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut i32_5: i32 = 132i32;
    let mut i64_5: i64 = -48i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_5);
    let mut i64_6: i64 = 16i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_9, duration_8);
    let mut u32_2: u32 = 31u32;
    let mut u8_6: u8 = 58u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 88u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_6: i32 = 28i32;
    let mut i64_7: i64 = 99i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_6);
    let mut i8_6: i8 = 50i8;
    let mut i8_7: i8 = 104i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_13: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_14: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_13, utcoffset_3);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_14);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_11);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_7, time_6);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_6, duration_10);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_5);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4966() {
    rusty_monitor::set_test_id(4966);
    let mut i32_0: i32 = 28i32;
    let mut i64_0: i64 = -107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -119i32;
    let mut i64_1: i64 = 88i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = 7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_2);
    let mut i64_3: i64 = 128i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_2: i32 = -91i32;
    let mut i64_4: i64 = -63i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_6);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 78i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 2u16;
    let mut i32_3: i32 = -72i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i32_4: i32 = 132i32;
    let mut i64_5: i64 = -48i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut i64_6: i64 = 16i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_8, duration_7);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 88u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_5: i32 = 28i32;
    let mut i64_7: i64 = 99i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut i8_6: i8 = 50i8;
    let mut i8_7: i8 = 104i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_8, utcoffset_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_10);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_9);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_9);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6828() {
    rusty_monitor::set_test_id(6828);
    let mut i64_0: i64 = -80i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 36u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 70i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = -148i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_0);
    let mut i8_0: i8 = -10i8;
    let mut i8_1: i8 = -78i8;
    let mut i8_2: i8 = 85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i128_1: i128 = -78i128;
    let mut str_0: &str = "G2AE5UpLKu";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_0: i32 = 9i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_1: u32 = 15u32;
    let mut u8_3: u8 = 54u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 68u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 5i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6007() {
    rusty_monitor::set_test_id(6007);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 26i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = -89i8;
    let mut i8_1: i8 = 58i8;
    let mut i8_2: i8 = 30i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_0: i32 = -96i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut i64_1: i64 = -64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_2);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut i8_3: i8 = -116i8;
    let mut i8_4: i8 = -124i8;
    let mut i8_5: i8 = 68i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_6: i8 = 3i8;
    let mut i8_7: i8 = 43i8;
    let mut i8_8: i8 = -17i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_0: f32 = -46.556388f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 53i32;
    let mut i64_2: i64 = 61i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_90() {
    rusty_monitor::set_test_id(90);
    let mut i64_0: i64 = -23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -39i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_3);
    let mut u16_0: u16 = 93u16;
    let mut i32_0: i32 = -66i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = -244i32;
    let mut i64_2: i64 = -178i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut i32_2: i32 = -39i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = 34i8;
    let mut i8_2: i8 = -23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_3: i64 = 94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut u32_1: u32 = 95u32;
    let mut u8_3: u8 = 69u8;
    let mut u8_4: u8 = 71u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 70i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_3: i32 = -31i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_5, offset: utcoffset_0};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_3);
    let mut u8_7: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7147() {
    rusty_monitor::set_test_id(7147);
    let mut i64_0: i64 = 35i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i8_0: i8 = -69i8;
    let mut i8_1: i8 = -48i8;
    let mut i8_2: i8 = -49i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = -19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_0: u16 = 58u16;
    let mut i32_0: i32 = 34i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i8_3: i8 = -3i8;
    let mut i8_4: i8 = -55i8;
    let mut i8_5: i8 = 77i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = 132i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i32_2: i32 = -119i32;
    let mut i64_2: i64 = 88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_3: i64 = 7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_5);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i8_6: i8 = 32i8;
    let mut i8_7: i8 = 44i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_4, duration_4);
    let mut i64_4: i64 = 128i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_3: i32 = -91i32;
    let mut i64_5: i64 = -63i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_8);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i8_9: i8 = 73i8;
    let mut i8_10: i8 = 78i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 2u16;
    let mut i32_4: i32 = -72i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut i32_5: i32 = 132i32;
    let mut i64_6: i64 = -48i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_5);
    let mut i64_7: i64 = 16i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 88u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_6: i32 = 28i32;
    let mut i64_8: i64 = 99i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_8, i32_6);
    let mut i8_12: i8 = 50i8;
    let mut i8_13: i8 = 104i8;
    let mut i8_14: i8 = 88i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_10, utcoffset_5);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_11);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_12);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_6, time_3);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_6, duration_11);
    let mut primitivedatetime_7_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_7;
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_5);
    let mut bool_0: bool = std::cmp::PartialEq::eq(primitivedatetime_3_ref_0, primitivedatetime_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7630() {
    rusty_monitor::set_test_id(7630);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_1: i32 = -119i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut i64_2: i64 = 128i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_2: i32 = -91i32;
    let mut i64_3: i64 = -63i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 78i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 2u16;
    let mut i32_3: i32 = -72i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i32_4: i32 = 132i32;
    let mut i64_4: i64 = -48i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut i64_5: i64 = 16i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 88u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_5: i32 = 28i32;
    let mut i64_6: i64 = 99i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_5);
    let mut i8_6: i8 = 50i8;
    let mut i8_7: i8 = 104i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_10, utcoffset_4);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_11);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_8);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut month_1: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_252() {
    rusty_monitor::set_test_id(252);
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u16_0: u16 = 49u16;
    let mut i32_0: i32 = 160i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_2);
    let mut i64_1: i64 = -42i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = 161i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 54u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -182i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = 122i32;
    let mut i64_3: i64 = -96i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6847() {
    rusty_monitor::set_test_id(6847);
    let mut i64_0: i64 = 228i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 9u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut f32_0: f32 = 33.106668f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -21i32;
    let mut i64_1: i64 = 146i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = 114i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_2, duration_2);
    let mut i32_2: i32 = -119i32;
    let mut i64_2: i64 = 88i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_3: i64 = 7i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_5, utcoffset_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_4, duration_5);
    let mut i64_4: i64 = 128i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_3: i32 = -91i32;
    let mut i64_5: i64 = -63i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_8, duration_7);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 78i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_7, utcoffset_1);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 2u16;
    let mut i32_4: i32 = -72i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut i64_6: i64 = 16i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 88u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_5: i32 = 28i32;
    let mut i64_7: i64 = 99i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut i8_6: i8 = 50i8;
    let mut i8_7: i8 = 104i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_9, utcoffset_2);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_10);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_11);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_4);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_5);
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(primitivedatetime_3, primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5162() {
    rusty_monitor::set_test_id(5162);
    let mut i32_0: i32 = 31i32;
    let mut u16_0: u16 = 78u16;
    let mut i32_1: i32 = -112i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_0: i64 = 144i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_1: i64 = 18i64;
    let mut f32_0: f32 = -64.229658f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u32_0: u32 = 55u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_2: i32 = 135i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_2);
    let mut f32_1: f32 = -60.910676f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_2: i64 = -73i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_3: i32 = 107i32;
    let mut i64_3: i64 = -77i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut i32_4: i32 = 92i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_3, duration_10);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut i32_5: i32 = -119i32;
    let mut i64_4: i64 = 88i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_13);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = 132i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i32_6: i32 = -91i32;
    let mut i64_6: i64 = -63i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_15, duration_14);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_7, duration_16);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_8);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 78i8;
    let mut i8_2: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_9, utcoffset_2);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_10);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 2u16;
    let mut i32_7: i32 = -72i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut i32_8: i32 = 132i32;
    let mut i64_7: i64 = -48i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_8);
    let mut i64_8: i64 = 16i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_18, duration_17);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 88u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_9: i32 = 28i32;
    let mut i64_9: i64 = 99i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_9);
    let mut i8_3: i8 = 50i8;
    let mut i8_4: i8 = 104i8;
    let mut i8_5: i8 = 88i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_11, utcoffset_3);
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_12);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_7, duration_20);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_8, time_6);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_5, duration_19);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_4);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3825() {
    rusty_monitor::set_test_id(3825);
    let mut str_0: &str = "3";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -188i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = -14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_1: i32 = -119i32;
    let mut i64_1: i64 = 88i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = 7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_2, duration_2);
    let mut i64_3: i64 = 128i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_2: i32 = -91i32;
    let mut i64_4: i64 = -63i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3008() {
    rusty_monitor::set_test_id(3008);
    let mut i32_0: i32 = -46i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_2_ref_0: &std::time::Instant = &mut instant_2;
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut f32_0: f32 = -112.549153f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = -3i8;
    let mut i8_1: i8 = -35i8;
    let mut i8_2: i8 = 39i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 36u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_2: i32 = 107i32;
    let mut i64_1: i64 = -77i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_3: i32 = 92i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_2, duration_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3710() {
    rusty_monitor::set_test_id(3710);
    let mut f32_0: f32 = -19.324911f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -150i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut u16_0: u16 = 78u16;
    let mut i32_1: i32 = -112i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i64_0: i64 = 144i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_2: i32 = -86i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut f32_1: f32 = -64.229658f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u32_1: u32 = 55u32;
    let mut u8_3: u8 = 25u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 3u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = 1i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_3: i32 = 135i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_3};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1268() {
    rusty_monitor::set_test_id(1268);
    let mut i32_0: i32 = 14i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 292i32;
    let mut i64_0: i64 = 140i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = 20i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = -39i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_2: i32 = 68i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_2: i64 = 9i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_3: i32 = -64i32;
    let mut i64_3: i64 = -19i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i8_0: i8 = -49i8;
    let mut i8_1: i8 = -20i8;
    let mut i8_2: i8 = -67i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -116.924542f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_4: i32 = 51i32;
    let mut i64_4: i64 = 0i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_10);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_5: i64 = -56i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut u16_0: u16 = 71u16;
    let mut i32_5: i32 = -30i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_11);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_9);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = std::clone::Clone::clone(primitivedatetime_1_ref_0);
    let mut i32_6: i32 = crate::date::Date::year(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4935() {
    rusty_monitor::set_test_id(4935);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = -11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = -119i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_0: i8 = 32i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 88i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_1);
    let mut i64_2: i64 = 128i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_2: i32 = -91i32;
    let mut i64_3: i64 = -63i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_5);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 78i8;
    let mut i8_5: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_6, utcoffset_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 2u16;
    let mut i32_3: i32 = -72i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut i32_4: i32 = 132i32;
    let mut i64_4: i64 = -48i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut i64_5: i64 = 16i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 88u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_5: i32 = 28i32;
    let mut i64_6: i64 = 99i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_5);
    let mut i8_6: i8 = 50i8;
    let mut i8_7: i8 = 104i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_8, utcoffset_3);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_8);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut u8_3: u8 = crate::date::Date::iso_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_363() {
    rusty_monitor::set_test_id(363);
    let mut i32_0: i32 = -65i32;
    let mut i64_0: i64 = -125i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 79i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_1: i64 = -63i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 11u32;
    let mut u8_3: u8 = 16u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 80u8;
    let mut i32_2: i32 = 0i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_3: i32 = 16i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut u32_2: u32 = 80u32;
    let mut u8_6: u8 = 93u8;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 45u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u32_3: u32 = 82u32;
    let mut u8_9: u8 = 8u8;
    let mut u8_10: u8 = 92u8;
    let mut u8_11: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i64_2: i64 = -110i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_4: i32 = 67i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_2);
    let mut i32_5: i32 = 74i32;
    let mut i64_3: i64 = -149i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_5);
    let mut i32_6: i32 = -122i32;
    let mut i64_4: i64 = -32i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_6);
    let mut i32_7: i32 = -24i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_8);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_5);
    let mut i64_5: i64 = 110i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 22u32;
    let mut u8_12: u8 = 82u8;
    let mut u8_13: u8 = 38u8;
    let mut u8_14: u8 = 68u8;
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(primitivedatetime_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8414() {
    rusty_monitor::set_test_id(8414);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut f32_0: f32 = -160.879850f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = 16i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -25i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 18u16;
    let mut i32_0: i32 = -98i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_0, duration_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 107i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 39i32;
    let mut i64_2: i64 = -98i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i8_3: i8 = 20i8;
    let mut i8_4: i8 = -81i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut u32_0: u32 = 68u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 68u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 36u8;
    let mut u8_4: u8 = 25u8;
    let mut u8_5: u8 = 37u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 7u32;
    let mut u8_6: u8 = 51u8;
    let mut u8_7: u8 = 6u8;
    let mut u8_8: u8 = 65u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut weekday_1: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut month_2: month::Month = crate::month::Month::March;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_328() {
    rusty_monitor::set_test_id(328);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_0: u16 = 49u16;
    let mut i32_0: i32 = 160i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_0, duration_1);
    let mut i64_0: i64 = -42i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 161i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 54u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -185i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = std::ops::Add::add(primitivedatetime_2, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i32_2: i32 = 57i32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 128i32;
    let mut i64_2: i64 = -96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut i32_4: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = std::ops::Sub::sub(primitivedatetime_4, duration_5);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_5);
    let mut u32_1: u32 = crate::time::Time::microsecond(time_1);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut bool_0: bool = crate::util::is_leap_year(i32_2);
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}
}