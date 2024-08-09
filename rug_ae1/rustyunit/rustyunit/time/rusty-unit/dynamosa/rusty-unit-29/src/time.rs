//! The [`Time`] struct and its associated `impl`s.

use core::fmt;
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;

#[cfg(feature = "formatting")]
use crate::formatting::Formattable;
#[cfg(feature = "parsing")]
use crate::parsing::Parsable;
use crate::util::DateAdjustment;
use crate::{error, Duration};

/// By explicitly inserting this enum where padding is expected, the compiler is able to better
/// perform niche value optimization.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Padding {
    #[allow(clippy::missing_docs_in_private_items)]
    Optimize,
}

/// The clock time within a given date. Nanosecond precision.
///
/// All minutes are assumed to have exactly 60 seconds; no attempt is made to handle leap seconds
/// (either positive or negative).
///
/// When comparing two `Time`s, they are assumed to be in the same calendar date.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time {
    #[allow(clippy::missing_docs_in_private_items)]
    hour: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    minute: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    second: u8,
    #[allow(clippy::missing_docs_in_private_items)]
    nanosecond: u32,
    #[allow(clippy::missing_docs_in_private_items)]
    padding: Padding,
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Time")
            .field("hour", &self.hour)
            .field("minute", &self.minute)
            .field("second", &self.second)
            .field("nanosecond", &self.nanosecond)
            .finish()
    }
}

impl Time {
    /// Create a `Time` that is exactly midnight.
    ///
    /// ```rust
    /// # use time::{Time, macros::time};
    /// assert_eq!(Time::MIDNIGHT, time!(0:00));
    /// ```
    pub const MIDNIGHT: Self = Self::__from_hms_nanos_unchecked(0, 0, 0, 0);

    /// The smallest value that can be represented by `Time`.
    ///
    /// `00:00:00.0`
    pub(crate) const MIN: Self = Self::__from_hms_nanos_unchecked(0, 0, 0, 0);

    /// The largest value that can be represented by `Time`.
    ///
    /// `23:59:59.999_999_999`
    pub(crate) const MAX: Self = Self::__from_hms_nanos_unchecked(23, 59, 59, 999_999_999);

    // region: constructors
    /// Create a `Time` from its components.
    #[doc(hidden)]
    pub const fn __from_hms_nanos_unchecked(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
            padding: Padding::Optimize,
        }
    }

    /// Attempt to create a `Time` from the hour, minute, and second.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms(1, 2, 3).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms(24, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms(0, 60, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms(0, 0, 60).is_err()); // 60 isn't a valid second.
    /// ```
    pub const fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        Ok(Self::__from_hms_nanos_unchecked(hour, minute, second, 0))
    }

    /// Attempt to create a `Time` from the hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_milli(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_milli(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_milli(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_milli(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_milli(0, 0, 0, 1_000).is_err()); // 1_000 isn't a valid millisecond.
    /// ```
    pub const fn from_hms_milli(
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(millisecond in 0 => 999);
        Ok(Self::__from_hms_nanos_unchecked(
            hour,
            minute,
            second,
            millisecond as u32 * 1_000_000,
        ))
    }

    /// Attempt to create a `Time` from the hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_micro(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_micro(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_micro(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_micro(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_micro(0, 0, 0, 1_000_000).is_err()); // 1_000_000 isn't a valid microsecond.
    /// ```
    pub const fn from_hms_micro(
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(microsecond in 0 => 999_999);
        Ok(Self::__from_hms_nanos_unchecked(
            hour,
            minute,
            second,
            microsecond * 1_000,
        ))
    }

    /// Attempt to create a `Time` from the hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_nano(1, 2, 3, 4).is_ok());
    /// ```
    ///
    /// ```rust
    /// # use time::Time;
    /// assert!(Time::from_hms_nano(24, 0, 0, 0).is_err()); // 24 isn't a valid hour.
    /// assert!(Time::from_hms_nano(0, 60, 0, 0).is_err()); // 60 isn't a valid minute.
    /// assert!(Time::from_hms_nano(0, 0, 60, 0).is_err()); // 60 isn't a valid second.
    /// assert!(Time::from_hms_nano(0, 0, 0, 1_000_000_000).is_err()); // 1_000_000_000 isn't a valid nanosecond.
    /// ```
    pub const fn from_hms_nano(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => 23);
        ensure_value_in_range!(minute in 0 => 59);
        ensure_value_in_range!(second in 0 => 59);
        ensure_value_in_range!(nanosecond in 0 => 999_999_999);
        Ok(Self::__from_hms_nanos_unchecked(
            hour, minute, second, nanosecond,
        ))
    }
    // endregion constructors

    // region: getters
    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).as_hms(), (0, 0, 0));
    /// assert_eq!(time!(23:59:59).as_hms(), (23, 59, 59));
    /// ```
    pub const fn as_hms(self) -> (u8, u8, u8) {
        (self.hour, self.minute, self.second)
    }

    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).as_hms_milli(), (0, 0, 0, 0));
    /// assert_eq!(time!(23:59:59.999).as_hms_milli(), (23, 59, 59, 999));
    /// ```
    pub const fn as_hms_milli(self) -> (u8, u8, u8, u16) {
        (
            self.hour,
            self.minute,
            self.second,
            (self.nanosecond / 1_000_000) as u16,
        )
    }

    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).as_hms_micro(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     time!(23:59:59.999_999).as_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn as_hms_micro(self) -> (u8, u8, u8, u32) {
        (self.hour, self.minute, self.second, self.nanosecond / 1_000)
    }

    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).as_hms_nano(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     time!(23:59:59.999_999_999).as_hms_nano(),
    ///     (23, 59, 59, 999_999_999)
    /// );
    /// ```
    pub const fn as_hms_nano(self) -> (u8, u8, u8, u32) {
        (self.hour, self.minute, self.second, self.nanosecond)
    }

    /// Get the clock hour.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).hour(), 0);
    /// assert_eq!(time!(23:59:59).hour(), 23);
    /// ```
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// Get the minute within the hour.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).minute(), 0);
    /// assert_eq!(time!(23:59:59).minute(), 59);
    /// ```
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Get the second within the minute.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00:00).second(), 0);
    /// assert_eq!(time!(23:59:59).second(), 59);
    /// ```
    pub const fn second(self) -> u8 {
        self.second
    }

    /// Get the milliseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00).millisecond(), 0);
    /// assert_eq!(time!(23:59:59.999).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        (self.nanosecond / 1_000_000) as _
    }

    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00).microsecond(), 0);
    /// assert_eq!(time!(23:59:59.999_999).microsecond(), 999_999);
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.nanosecond / 1_000
    }

    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::macros::time;
    /// assert_eq!(time!(0:00).nanosecond(), 0);
    /// assert_eq!(time!(23:59:59.999_999_999).nanosecond(), 999_999_999);
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.nanosecond
    }
    // endregion getters

    // region: arithmetic helpers
    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow, returning whether
    /// the date is different.
    pub(crate) const fn adjusting_add(self, duration: Duration) -> (DateAdjustment, Self) {
        let mut nanoseconds = self.nanosecond as i32 + duration.subsec_nanoseconds();
        let mut seconds = self.second as i8 + (duration.whole_seconds() % 60) as i8;
        let mut minutes = self.minute as i8 + (duration.whole_minutes() % 60) as i8;
        let mut hours = self.hour as i8 + (duration.whole_hours() % 24) as i8;
        let mut date_adjustment = DateAdjustment::None;

        cascade!(nanoseconds in 0..1_000_000_000 => seconds);
        cascade!(seconds in 0..60 => minutes);
        cascade!(minutes in 0..60 => hours);
        if hours >= 24 {
            hours -= 24;
            date_adjustment = DateAdjustment::Next;
        } else if hours < 0 {
            hours += 24;
            date_adjustment = DateAdjustment::Previous;
        }

        (
            date_adjustment,
            Self::__from_hms_nanos_unchecked(
                hours as _,
                minutes as _,
                seconds as _,
                nanoseconds as _,
            ),
        )
    }

    /// Subtract the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow, returning
    /// whether the date is different.
    pub(crate) const fn adjusting_sub(self, duration: Duration) -> (DateAdjustment, Self) {
        let mut nanoseconds = self.nanosecond as i32 - duration.subsec_nanoseconds();
        let mut seconds = self.second as i8 - (duration.whole_seconds() % 60) as i8;
        let mut minutes = self.minute as i8 - (duration.whole_minutes() % 60) as i8;
        let mut hours = self.hour as i8 - (duration.whole_hours() % 24) as i8;
        let mut date_adjustment = DateAdjustment::None;

        cascade!(nanoseconds in 0..1_000_000_000 => seconds);
        cascade!(seconds in 0..60 => minutes);
        cascade!(minutes in 0..60 => hours);
        if hours >= 24 {
            hours -= 24;
            date_adjustment = DateAdjustment::Next;
        } else if hours < 0 {
            hours += 24;
            date_adjustment = DateAdjustment::Previous;
        }

        (
            date_adjustment,
            Self::__from_hms_nanos_unchecked(
                hours as _,
                minutes as _,
                seconds as _,
                nanoseconds as _,
            ),
        )
    }

    /// Add the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow,
    /// returning whether the date is the previous date as the first element of the tuple.
    pub(crate) const fn adjusting_add_std(self, duration: StdDuration) -> (bool, Self) {
        let mut nanosecond = self.nanosecond + duration.subsec_nanos();
        let mut second = self.second + (duration.as_secs() % 60) as u8;
        let mut minute = self.minute + ((duration.as_secs() / 60) % 60) as u8;
        let mut hour = self.hour + ((duration.as_secs() / 3_600) % 24) as u8;
        let mut is_next_day = false;

        cascade!(nanosecond in 0..1_000_000_000 => second);
        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        if hour >= 24 {
            hour -= 24;
            is_next_day = true;
        }

        (
            is_next_day,
            Self::__from_hms_nanos_unchecked(hour, minute, second, nanosecond),
        )
    }

    /// Subtract the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow,
    /// returning whether the date is the previous date as the first element of the tuple.
    pub(crate) const fn adjusting_sub_std(self, duration: StdDuration) -> (bool, Self) {
        let mut nanosecond = self.nanosecond as i32 - duration.subsec_nanos() as i32;
        let mut second = self.second as i8 - (duration.as_secs() % 60) as i8;
        let mut minute = self.minute as i8 - ((duration.as_secs() / 60) % 60) as i8;
        let mut hour = self.hour as i8 - ((duration.as_secs() / 3_600) % 24) as i8;
        let mut is_previous_day = false;

        cascade!(nanosecond in 0..1_000_000_000 => second);
        cascade!(second in 0..60 => minute);
        cascade!(minute in 0..60 => hour);
        if hour < 0 {
            hour += 24;
            is_previous_day = true;
        }

        (
            is_previous_day,
            Self::__from_hms_nanos_unchecked(hour as _, minute as _, second as _, nanosecond as _),
        )
    }
    // endregion arithmetic helpers
}

// region: formatting & parsing
#[cfg(feature = "formatting")]
impl Time {
    /// Format the `Time` using the provided [format description](crate::format_description).
    pub fn format_into(
        self,
        output: &mut impl io::Write,
        format: &(impl Formattable + ?Sized),
    ) -> Result<usize, crate::error::Format> {
        format.format_into(output, None, Some(self), None)
    }

    /// Format the `Time` using the provided [format description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::time};
    /// let format = format_description::parse("[hour]:[minute]:[second]")?;
    /// assert_eq!(time!(12:00).format(&format)?, "12:00:00");
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn format(
        self,
        format: &(impl Formattable + ?Sized),
    ) -> Result<String, crate::error::Format> {
        format.format(None, Some(self), None)
    }
}

#[cfg(feature = "parsing")]
impl Time {
    /// Parse a `Time` from the input using the provided [format
    /// description](crate::format_description).
    ///
    /// ```rust
    /// # use time::{format_description, macros::time, Time};
    /// let format = format_description::parse("[hour]:[minute]:[second]")?;
    /// assert_eq!(Time::parse("12:00:00", &format)?, time!(12:00));
    /// # Ok::<_, time::Error>(())
    /// ```
    pub fn parse(
        input: &str,
        description: &(impl Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        description.parse_time(input.as_bytes())
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, width) = match self.nanosecond() {
            nanos if nanos % 10 != 0 => (nanos, 9),
            nanos if (nanos / 10) % 10 != 0 => (nanos / 10, 8),
            nanos if (nanos / 100) % 10 != 0 => (nanos / 100, 7),
            nanos if (nanos / 1_000) % 10 != 0 => (nanos / 1_000, 6),
            nanos if (nanos / 10_000) % 10 != 0 => (nanos / 10_000, 5),
            nanos if (nanos / 100_000) % 10 != 0 => (nanos / 100_000, 4),
            nanos if (nanos / 1_000_000) % 10 != 0 => (nanos / 1_000_000, 3),
            nanos if (nanos / 10_000_000) % 10 != 0 => (nanos / 10_000_000, 2),
            nanos => (nanos / 100_000_000, 1),
        };
        write!(
            f,
            "{}:{:02}:{:02}.{:0width$}",
            self.hour,
            self.minute,
            self.second,
            value,
            width = width
        )
    }
}
// endregion formatting & parsing

// region: trait impls
impl Add<Duration> for Time {
    type Output = Self;

    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::{ext::NumericalDuration, macros::time};
    /// assert_eq!(time!(12:00) + 2.hours(), time!(14:00));
    /// assert_eq!(time!(0:00:01) + (-2).seconds(), time!(23:59:59));
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        self.adjusting_add(duration).1
    }
}

impl Add<StdDuration> for Time {
    type Output = Self;

    /// Add the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::{ext::NumericalStdDuration, macros::time};
    /// assert_eq!(time!(12:00) + 2.std_hours(), time!(14:00));
    /// assert_eq!(time!(23:59:59) + 2.std_seconds(), time!(0:00:01));
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        self.adjusting_add_std(duration).1
    }
}

impl_add_assign!(Time: Duration, StdDuration);

impl Sub<Duration> for Time {
    type Output = Self;

    /// Subtract the sub-day time of the [`Duration`] from the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::{ext::NumericalDuration, macros::time};
    /// assert_eq!(time!(14:00) - 2.hours(), time!(12:00));
    /// assert_eq!(time!(23:59:59) - (-2).seconds(), time!(0:00:01));
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        self.adjusting_sub(duration).1
    }
}

impl Sub<StdDuration> for Time {
    type Output = Self;

    /// Subtract the sub-day time of the [`std::time::Duration`] from the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::{ext::NumericalStdDuration, macros::time};
    /// assert_eq!(time!(14:00) - 2.std_hours(), time!(12:00));
    /// assert_eq!(time!(0:00:01) - 2.std_seconds(), time!(23:59:59));
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        self.adjusting_sub_std(duration).1
    }
}

impl_sub_assign!(Time: Duration, StdDuration);

impl Sub for Time {
    type Output = Duration;

    /// Subtract two `Time`s, returning the [`Duration`] between. This assumes both `Time`s are in
    /// the same calendar day.
    ///
    /// ```rust
    /// # use time::{ext::NumericalDuration, macros::time};
    /// assert_eq!(time!(0:00) - time!(0:00), 0.seconds());
    /// assert_eq!(time!(1:00) - time!(0:00), 1.hours());
    /// assert_eq!(time!(0:00) - time!(1:00), (-1).hours());
    /// assert_eq!(time!(0:00) - time!(23:00), (-23).hours());
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let hour_diff = (self.hour as i8) - (rhs.hour as i8);
        let minute_diff = (self.minute as i8) - (rhs.minute as i8);
        let mut second_diff = (self.second as i8) - (rhs.second as i8);
        let mut nanosecond_diff = (self.nanosecond as i32) - (rhs.nanosecond as i32);

        cascade!(nanosecond_diff in 0..1_000_000_000 => second_diff);

        Duration::new_unchecked(
            hour_diff as i64 * 3_600 + minute_diff as i64 * 60 + second_diff as i64,
            nanosecond_diff,
        )
    }
}

// endregion trait impls

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::ops::Sub;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6947() {
    rusty_monitor::set_test_id(6947);
    let mut f32_0: f32 = 46.884041f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut i32_0: i32 = 75i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u8_0: u8 = 87u8;
    let mut i32_1: i32 = -186i32;
    let mut i8_0: i8 = -55i8;
    let mut i8_1: i8 = 105i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_2: i32 = -81i32;
    let mut f32_1: f32 = -82.453524f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_0: i64 = 82i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u32_0: u32 = 97u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 35u8;
    let mut u8_3: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_6);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_1);
    let mut i64_1: i64 = -52i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_3: i32 = -105i32;
    let mut i64_2: i64 = -22i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_2: f32 = 152.714349f32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut u32_1: u32 = 37u32;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 38u8;
    let mut u8_6: u8 = 24u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_10);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_167() {
    rusty_monitor::set_test_id(167);
    let mut i64_0: i64 = -155i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 54u16;
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f32_0: f32 = 66.814052f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut f64_0: f64 = -33.471806f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = -14.695302f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2263() {
    rusty_monitor::set_test_id(2263);
    let mut i64_0: i64 = 50i64;
    let mut i64_1: i64 = -52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -105i32;
    let mut i64_2: i64 = -22i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut u16_0: u16 = 52u16;
    let mut i32_1: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut f32_0: f32 = 152.714349f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 38u8;
    let mut u8_2: u8 = 24u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_3);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i64_3: i64 = 78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 52i8;
    let mut i8_1: i8 = -6i8;
    let mut i8_2: i8 = 33i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 125i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i32_2: i32 = 113i32;
    let mut i64_5: i64 = 19i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 66u8;
    let mut u8_5: u8 = 10u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_8);
    let mut i8_3: i8 = 80i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -35i8;
    let mut i8_7: i8 = -71i8;
    let mut i8_8: i8 = -35i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut f64_0: f64 = 73.906470f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut i8_9: i8 = -99i8;
    let mut i8_10: i8 = -87i8;
    let mut i8_11: i8 = 30i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut f32_1: f32 = 57.560709f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 75u32;
    let mut u8_6: u8 = 14u8;
    let mut u8_7: u8 = 74u8;
    let mut u8_8: u8 = 53u8;
    let mut time_6: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_0};
    let mut time_7: crate::time::Time = std::ops::Sub::sub(time_6, duration_13);
    let mut i32_3: i32 = 18i32;
    let mut i64_6: i64 = 63i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut f64_1: f64 = -37.836691f64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_3: u32 = 80u32;
    let mut u8_9: u8 = 62u8;
    let mut u8_10: u8 = 12u8;
    let mut u8_11: u8 = 73u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_9: crate::time::Time = std::ops::Add::add(time_8, duration_15);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_9);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(time_3_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7034() {
    rusty_monitor::set_test_id(7034);
    let mut i32_0: i32 = 42i32;
    let mut i64_0: i64 = -120i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = -87i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut i32_2: i32 = 75i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_2);
    let mut u8_0: u8 = 87u8;
    let mut i32_3: i32 = -186i32;
    let mut i8_0: i8 = -55i8;
    let mut i8_1: i8 = 105i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_4: i32 = -81i32;
    let mut f32_0: f32 = -82.453524f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_4);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 84u32;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 5u8;
    let mut u8_3: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i64_1: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut u32_1: u32 = 97u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 35u8;
    let mut u8_6: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_3);
    let mut i64_2: i64 = -52i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_5: i32 = -105i32;
    let mut i64_3: i64 = -22i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_5);
    let mut u16_0: u16 = 52u16;
    let mut i32_6: i32 = 16i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_4, primitivedatetime_0);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_7);
    let mut time_5_ref_0: &crate::time::Time = &mut time_5;
    let mut f32_1: f32 = 152.714349f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u32_2: u32 = 37u32;
    let mut u8_7: u8 = 61u8;
    let mut u8_8: u8 = 38u8;
    let mut u8_9: u8 = 24u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut time_7: crate::time::Time = std::ops::Add::add(time_6, duration_9);
    let mut u8_10: u8 = crate::time::Time::second(time_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_0, weekday_0);
    let mut u32_3: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_102() {
    rusty_monitor::set_test_id(102);
    let mut i32_0: i32 = 2i32;
    let mut i32_1: i32 = -76i32;
    let mut i64_0: i64 = 186i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut i64_1: i64 = -61i64;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut i64_2: i64 = 48i64;
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 18u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 36u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 40u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 99u32;
    let mut u8_6: u8 = 35u8;
    let mut u8_7: u8 = 96u8;
    let mut u8_8: u8 = 96u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_412() {
    rusty_monitor::set_test_id(412);
    let mut i64_0: i64 = -10i64;
    let mut i64_1: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 183i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_1: i32 = -15i32;
    let mut i64_2: i64 = 2i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 75u8;
    let mut u8_1: u8 = 93u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_108() {
    rusty_monitor::set_test_id(108);
    let mut i64_0: i64 = -155i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 54u16;
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f32_0: f32 = 66.814052f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut f64_0: f64 = -33.471806f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f32_1: f32 = -14.695302f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6727() {
    rusty_monitor::set_test_id(6727);
    let mut i64_0: i64 = -168i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i64_1: i64 = 2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut f64_0: f64 = -193.116964f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i64_2: i64 = -227i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u32_0: u32 = 33u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 30u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut i32_0: i32 = 75i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2700() {
    rusty_monitor::set_test_id(2700);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 47i32;
    let mut i64_1: i64 = -60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut i32_1: i32 = -58i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 1u8;
    let mut i32_2: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = -10i64;
    let mut i64_3: i64 = 17i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 183i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i32_4: i32 = -15i32;
    let mut i64_4: i64 = 2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_4);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_5);
    let mut i32_5: i32 = 101i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_0);
    let mut primitivedatetime_3_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_2, u8_1, u8_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_2);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_1, duration_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4771() {
    rusty_monitor::set_test_id(4771);
    let mut i32_0: i32 = -66i32;
    let mut i64_0: i64 = -106i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_1: i32 = -107i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = 43i8;
    let mut i8_2: i8 = -85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 35i8;
    let mut i8_5: i8 = 82i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -135i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -11i32;
    let mut i64_2: i64 = -144i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_6: i8 = -13i8;
    let mut i8_7: i8 = 89i8;
    let mut i8_8: i8 = -32i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 22i8;
    let mut i8_10: i8 = -88i8;
    let mut i8_11: i8 = 1i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 56u32;
    let mut u8_3: u8 = 96u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 75u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 40i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(time_2_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_486() {
    rusty_monitor::set_test_id(486);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i64_0: i64 = -52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -105i32;
    let mut i64_1: i64 = -22i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u16_0: u16 = 52u16;
    let mut i32_1: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut f32_0: f32 = 152.714349f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 38u8;
    let mut u8_2: u8 = 24u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_3);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i64_2: i64 = 78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2253() {
    rusty_monitor::set_test_id(2253);
    let mut i8_0: i8 = 82i8;
    let mut i8_1: i8 = -39i8;
    let mut i8_2: i8 = -15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 75u8;
    let mut i32_0: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = -10i64;
    let mut i64_1: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = 183i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_2: i32 = -15i32;
    let mut i64_2: i64 = 2i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i32_3: i32 = 101i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7329() {
    rusty_monitor::set_test_id(7329);
    let mut i32_0: i32 = 47i32;
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut i32_1: i32 = -58i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 1u8;
    let mut i32_2: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_1: i64 = -10i64;
    let mut i64_2: i64 = 17i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_3: i32 = -15i32;
    let mut i64_3: i64 = 2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_4);
    let mut i32_4: i32 = 101i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_2, u8_1, u8_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7628() {
    rusty_monitor::set_test_id(7628);
    let mut f32_0: f32 = 41.596790f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 93u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = -54i8;
    let mut i8_2: i8 = 18i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i64_0: i64 = -116i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 84u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 5u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_3: i8 = -40i8;
    let mut i8_4: i8 = -45i8;
    let mut i8_5: i8 = -32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 116.257497f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = 154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_2: u32 = 90u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 9u8;
    let mut u8_8: u8 = 99u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_5: crate::time::Time = std::ops::Sub::sub(time_4, duration_4);
    let mut i64_2: i64 = 28i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut i8_6: i8 = 80i8;
    let mut i8_7: i8 = 77i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = 130i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3103() {
    rusty_monitor::set_test_id(3103);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = 179i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 46i8;
    let mut i8_1: i8 = -44i8;
    let mut i8_2: i8 = -24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 23i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i32_1: i32 = 52i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut u16_0: u16 = 8u16;
    let mut i32_2: i32 = 88i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i64_1: i64 = 27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_3: i32 = -134i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 67u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_4: i32 = -116i32;
    let mut i64_2: i64 = -19i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut i32_5: i32 = -8i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_7, time_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i8_3: i8 = 37i8;
    let mut i8_4: i8 = -18i8;
    let mut i8_5: i8 = 50i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_6: i32 = 24i32;
    let mut i64_3: i64 = 144i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_6);
    let mut u32_1: u32 = 6u32;
    let mut u8_3: u8 = 90u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 86u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_5);
    let mut i32_7: i32 = -28i32;
    let mut date_8: crate::date::Date = crate::date::Date {value: i32_7};
    let mut i8_6: i8 = 7i8;
    let mut i8_7: i8 = -82i8;
    let mut i8_8: i8 = 34i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = -50.300370f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 54u32;
    let mut u8_6: u8 = 85u8;
    let mut u8_7: u8 = 23u8;
    let mut u8_8: u8 = 14u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_6: crate::time::Time = std::ops::Sub::sub(time_5, duration_7);
    let mut i32_8: i32 = -68i32;
    let mut date_9: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_9, time_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_6, offset: utcoffset_3};
    let mut i32_9: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_4);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1007() {
    rusty_monitor::set_test_id(1007);
    let mut i64_0: i64 = 21i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 70u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut i64_1: i64 = -104i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u16_0: u16 = 73u16;
    let mut i32_0: i32 = -1i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut u32_1: u32 = 56u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 43u8;
    let mut u8_5: u8 = 74u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = -121i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 242i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_2: i64 = -106i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_2: u32 = 59u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 60u8;
    let mut u8_8: u8 = 4u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_5);
    let mut i64_3: i64 = -46i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i128_1: i128 = -10i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_9);
    let mut time_5: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_6: crate::time::Time = std::ops::Sub::sub(time_5, duration_7);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = -24i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_1);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_4);
    let mut time_7: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2533() {
    rusty_monitor::set_test_id(2533);
    let mut i32_0: i32 = -10i32;
    let mut i64_0: i64 = 68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 72i8;
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut f64_0: f64 = -122.959890f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = -25i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut i32_1: i32 = 75i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut u8_0: u8 = 87u8;
    let mut i32_2: i32 = -186i32;
    let mut i8_3: i8 = -55i8;
    let mut i8_4: i8 = 105i8;
    let mut i8_5: i8 = -20i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
    let mut i32_3: i32 = -81i32;
    let mut f32_0: f32 = -82.453524f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u32_0: u32 = 84u32;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 5u8;
    let mut u8_3: u8 = 90u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_7);
    let mut i64_2: i64 = 82i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut u32_1: u32 = 97u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 35u8;
    let mut u8_6: u8 = 10u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_9);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_5);
    let mut i64_3: i64 = -52i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i32_4: i32 = -105i32;
    let mut i64_4: i64 = -22i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut u16_0: u16 = 52u16;
    let mut i32_5: i32 = 16i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_12);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_6, primitivedatetime_1);
    let mut time_6: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_7);
    let mut time_7: crate::time::Time = std::ops::Add::add(time_6, duration_11);
    let mut time_7_ref_0: &crate::time::Time = &mut time_7;
    let mut f32_1: f32 = 152.714349f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u32_2: u32 = 37u32;
    let mut u8_7: u8 = 61u8;
    let mut u8_8: u8 = 38u8;
    let mut u8_9: u8 = 24u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut time_9: crate::time::Time = std::ops::Add::add(time_8, duration_13);
    let mut u8_10: u8 = crate::time::Time::second(time_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_0, weekday_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3232() {
    rusty_monitor::set_test_id(3232);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 47i32;
    let mut i64_1: i64 = -60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut i32_1: i32 = -58i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 1u8;
    let mut i32_2: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = -10i64;
    let mut i64_3: i64 = 17i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 183i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut i32_4: i32 = -15i32;
    let mut i64_4: i64 = 2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_4);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_2};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_5);
    let mut i32_5: i32 = 101i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_2, u8_1, u8_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_2);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_1, duration_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_328() {
    rusty_monitor::set_test_id(328);
    let mut i32_0: i32 = -55i32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 75u8;
    let mut i32_1: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = -10i64;
    let mut i64_1: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 183i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -15i32;
    let mut i64_2: i64 = 2i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i32_4: i32 = 101i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_367() {
    rusty_monitor::set_test_id(367);
    let mut i64_0: i64 = -106i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -46i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i128_0: i128 = -10i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_6);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_4);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = -24i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_942() {
    rusty_monitor::set_test_id(942);
    let mut i32_0: i32 = 157i32;
    let mut i64_0: i64 = -64i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut f32_0: f32 = -22.683526f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 33u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 76u8;
    let mut u8_4: u8 = 43u8;
    let mut u8_5: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 3u32;
    let mut u8_6: u8 = 48u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 48u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(time_3, time_2);
    let mut u32_3: u32 = 80u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 5u8;
    let mut u8_11: u8 = 61u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 2u32;
    let mut u8_12: u8 = 93u8;
    let mut u8_13: u8 = 47u8;
    let mut u8_14: u8 = 88u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_0};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u32_5: u32 = 16u32;
    let mut u8_15: u8 = 21u8;
    let mut u8_16: u8 = 30u8;
    let mut u8_17: u8 = 99u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_1: i64 = 125i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_7);
    let mut u16_0: u16 = 4u16;
    let mut i32_2: i32 = -56i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 107i8;
    let mut i8_2: i8 = -44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_0);
    let mut time_7: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut u16_1: u16 = 38u16;
    let mut i32_3: i32 = -71i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_7);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut u32_6: u32 = 87u32;
    let mut u8_18: u8 = 35u8;
    let mut u8_19: u8 = 54u8;
    let mut u8_20: u8 = 7u8;
    let mut time_8: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i32_4: i32 = -150i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_8);
    let mut primitivedatetime_2_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_2;
    let mut i64_3: i64 = -84i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = -132i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -16i8;
    let mut i8_5: i8 = 115i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_5: i32 = 80i32;
    let mut i32_6: i32 = 75i32;
    let mut i64_5: i64 = -58i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_5);
    let mut u32_7: u32 = 12u32;
    let mut u8_21: u8 = 1u8;
    let mut u8_22: u8 = 92u8;
    let mut u8_23: u8 = 62u8;
    let mut time_9: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_23, u8_22, u8_21, u32_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5594() {
    rusty_monitor::set_test_id(5594);
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = -62i8;
    let mut i8_2: i8 = 86i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -134i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i32_0: i32 = -13i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 74u32;
    let mut u8_3: u8 = 51u8;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 66u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = -31i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut i64_2: i64 = -116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u32_2: u32 = 88u32;
    let mut u8_6: u8 = 84u8;
    let mut u8_7: u8 = 75u8;
    let mut u8_8: u8 = 5u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_3: i8 = -40i8;
    let mut i8_4: i8 = -45i8;
    let mut i8_5: i8 = -32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 116.257497f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_6: i8 = 25i8;
    let mut i8_7: i8 = -47i8;
    let mut i8_8: i8 = -10i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = 154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_3: u32 = 90u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 9u8;
    let mut u8_11: u8 = 99u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_6: crate::time::Time = std::ops::Sub::sub(time_5, duration_5);
    let mut i64_4: i64 = 28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut i8_9: i8 = 80i8;
    let mut i8_10: i8 = 77i8;
    let mut i8_11: i8 = 88i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_5: i64 = 130i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 33u32;
    let mut u8_12: u8 = 7u8;
    let mut u8_13: u8 = 47u8;
    let mut u8_14: u8 = 94u8;
    let mut time_7: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_1};
    let mut u8_15: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2733() {
    rusty_monitor::set_test_id(2733);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i64_0: i64 = 9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 75i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i8_0: i8 = -55i8;
    let mut i8_1: i8 = 105i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_1: i32 = -81i32;
    let mut f32_0: f32 = -82.453524f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 84u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i64_1: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut u32_1: u32 = 97u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_3);
    let mut i64_2: i64 = -52i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_2: i32 = -105i32;
    let mut i64_3: i64 = -22i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut u16_0: u16 = 52u16;
    let mut i32_3: i32 = 16i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u8_6: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1881() {
    rusty_monitor::set_test_id(1881);
    let mut i64_0: i64 = 134i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 91i8;
    let mut i8_2: i8 = 35i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 138i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_0: i32 = -204i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_2);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i8_3: i8 = -23i8;
    let mut i8_4: i8 = 73i8;
    let mut i8_5: i8 = 121i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_0: i128 = -126i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = -22i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut f32_0: f32 = 267.940437f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_2: i64 = 86i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_2);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut bool_1: bool = std::cmp::PartialEq::ne(time_3_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1647() {
    rusty_monitor::set_test_id(1647);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = -52i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -105i32;
    let mut i64_1: i64 = -22i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u16_0: u16 = 52u16;
    let mut i32_1: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut f32_0: f32 = 152.714349f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 38u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_2: i64 = 78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Sunday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8077() {
    rusty_monitor::set_test_id(8077);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 5u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 17u32;
    let mut u8_3: u8 = 33u8;
    let mut u8_4: u8 = 62u8;
    let mut u8_5: u8 = 70u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_3_ref_0, padding_2_ref_0);
    let mut i32_0: i32 = -15i32;
    let mut i64_0: i64 = -26i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i128_0: i128 = -34i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = -86i8;
    let mut i8_1: i8 = 65i8;
    let mut i8_2: i8 = -89i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -9i8;
    let mut i8_4: i8 = -85i8;
    let mut i8_5: i8 = 67i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_2: u32 = 63u32;
    let mut u8_6: u8 = 78u8;
    let mut u8_7: u8 = 5u8;
    let mut u8_8: u8 = 84u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_1: i32 = 20i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_2: i32 = 22i32;
    let mut i64_1: i64 = 151i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_2);
    let mut u32_3: u32 = 0u32;
    let mut u8_9: u8 = 73u8;
    let mut u8_10: u8 = 28u8;
    let mut u8_11: u8 = 98u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut u16_0: u16 = 64u16;
    let mut i32_3: i32 = 44i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_5);
    let mut i32_4: i32 = -50i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i64_2: i64 = -79i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_6);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut f32_0: f32 = 117.012122f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_3: i64 = -32i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_3);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_9);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7590() {
    rusty_monitor::set_test_id(7590);
    let mut i64_0: i64 = -18i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = -16i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 48i32;
    let mut f64_0: f64 = 57.444488f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut i32_2: i32 = 52i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut f64_1: f64 = -109.679649f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_3: i32 = -64i32;
    let mut i64_1: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i64_2: i64 = -97i64;
    let mut u32_0: u32 = 27u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = -82i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_3: i64 = -105i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut u16_0: u16 = 25u16;
    let mut i32_4: i32 = -85i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut time_1: crate::time::Time = std::clone::Clone::clone(time_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut i64_0: i64 = -155i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 54u16;
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f32_0: f32 = 66.814052f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut f64_0: f64 = -163.858066f64;
    let mut f64_1: f64 = -33.471806f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut f32_1: f32 = -14.695302f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_650() {
    rusty_monitor::set_test_id(650);
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -71i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 75u8;
    let mut i32_1: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_1: i64 = -10i64;
    let mut i64_2: i64 = 17i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 183i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -15i32;
    let mut i64_3: i64 = 2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 37u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 93u8;
    let mut u8_5: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i32_4: i32 = 101i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_2, u8_1, u8_0);
    let mut u8_6: u8 = crate::date::Date::sunday_based_week(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_974() {
    rusty_monitor::set_test_id(974);
    let mut i32_0: i32 = -134i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 67u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 50u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_1: i32 = -116i32;
    let mut i64_0: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i32_2: i32 = -8i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i8_0: i8 = 37i8;
    let mut i8_1: i8 = -18i8;
    let mut i8_2: i8 = 50i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = 24i32;
    let mut i64_1: i64 = 144i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut u32_1: u32 = 6u32;
    let mut u8_3: u8 = 90u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 86u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_1);
    let mut i32_4: i32 = -28i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut i8_3: i8 = 7i8;
    let mut i8_4: i8 = -82i8;
    let mut i8_5: i8 = 34i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = -50.300370f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 54u32;
    let mut u8_6: u8 = 85u8;
    let mut u8_7: u8 = 23u8;
    let mut u8_8: u8 = 14u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_3);
    let mut i32_5: i32 = -68i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_1};
    let mut i32_6: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_67() {
    rusty_monitor::set_test_id(67);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_0: i64 = 141i64;
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 63u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 61u8;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 55u32;
    let mut u8_3: u8 = 39u8;
    let mut u8_4: u8 = 19u8;
    let mut u8_5: u8 = 68u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut i64_1: i64 = 19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -28i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 2u32;
    let mut u8_6: u8 = 95u8;
    let mut u8_7: u8 = 15u8;
    let mut u8_8: u8 = 48u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut i32_0: i32 = 147i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i64_3: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_1: i32 = -27i32;
    let mut i64_4: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 27u32;
    let mut u8_9: u8 = 11u8;
    let mut u8_10: u8 = 28u8;
    let mut u8_11: u8 = 25u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_3, padding: padding_2};
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_4);
    let mut u16_0: u16 = 2u16;
    let mut i32_2: i32 = -82i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_3);
    let mut i64_5: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3609() {
    rusty_monitor::set_test_id(3609);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = 43i8;
    let mut i8_2: i8 = -85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 35i8;
    let mut i8_5: i8 = 82i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = -135i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -11i32;
    let mut i64_1: i64 = -144i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = -88i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 40i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    let mut padding_1: time::Padding = std::clone::Clone::clone(padding_0_ref_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_849() {
    rusty_monitor::set_test_id(849);
    let mut i64_0: i64 = -106i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 59u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_2);
    let mut i64_1: i64 = -46i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i128_0: i128 = -10i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = -24i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut month_0: month::Month = crate::month::Month::December;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_207() {
    rusty_monitor::set_test_id(207);
    let mut i64_0: i64 = 147i64;
    let mut u16_0: u16 = 54u16;
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut f64_0: f64 = -33.471806f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4485() {
    rusty_monitor::set_test_id(4485);
    let mut f64_0: f64 = -63.973649f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 25u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i32_0: i32 = -42i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_1: i32 = 194i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = 28i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_1: u32 = 64u32;
    let mut u8_3: u8 = 22u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 5u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -136i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut f64_1: f64 = -60.224623f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = -111.961155f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_2: u32 = 45u32;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 22u8;
    let mut u8_8: u8 = 31u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_1: i64 = -12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u16_1: u16 = 66u16;
    let mut i32_3: i32 = -6i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_4: i32 = -10i32;
    let mut u16_2: u16 = 56u16;
    let mut u8_9: u8 = 41u8;
    let mut u8_10: u8 = 89u8;
    let mut u8_11: u8 = 83u8;
    let mut i8_0: i8 = -32i8;
    let mut i8_1: i8 = -103i8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut i8_2: i8 = -14i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_1, i8_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_9, u8_10, u8_11, u16_2);
    let mut u8_12: u8 = crate::util::days_in_year_month(i32_4, month_0);
    let mut u8_13: u8 = crate::date::Date::monday_based_week(date_5);
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_4);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_14: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 55u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 19u8;
    let mut u8_2: u8 = 68u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = 19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = -28i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 2u32;
    let mut u8_3: u8 = 95u8;
    let mut u8_4: u8 = 15u8;
    let mut u8_5: u8 = 48u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut i32_0: i32 = 147i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i64_2: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_1: i32 = -27i32;
    let mut i64_3: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 27u32;
    let mut u8_6: u8 = 11u8;
    let mut u8_7: u8 = 28u8;
    let mut u8_8: u8 = 25u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_2};
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_4);
    let mut u16_0: u16 = 2u16;
    let mut i32_2: i32 = -82i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_3);
    let mut i64_4: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4493() {
    rusty_monitor::set_test_id(4493);
    let mut i64_0: i64 = 20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 30u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 93u8;
    let mut u8_2: u8 = 2u8;
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = 43i8;
    let mut i8_2: i8 = -85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = 35i8;
    let mut i8_5: i8 = 82i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -135i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = -11i32;
    let mut i64_2: i64 = -144i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_6: i8 = -13i8;
    let mut i8_7: i8 = 89i8;
    let mut i8_8: i8 = -32i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 23i8;
    let mut i8_10: i8 = -88i8;
    let mut i8_11: i8 = 1i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 56u32;
    let mut u8_3: u8 = 96u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 40i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_2);
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut u8_6: u8 = crate::date::Date::sunday_based_week(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4993() {
    rusty_monitor::set_test_id(4993);
    let mut i32_0: i32 = -57i32;
    let mut i64_0: i64 = 29i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = 75i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u8_0: u8 = 8u8;
    let mut i32_2: i32 = -186i32;
    let mut i8_0: i8 = -55i8;
    let mut i8_1: i8 = 105i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i32_3: i32 = -81i32;
    let mut f32_0: f32 = -82.453524f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_3);
    let mut u32_0: u32 = 84u32;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 5u8;
    let mut u8_3: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i64_1: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut u32_1: u32 = 97u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 35u8;
    let mut u8_6: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_2);
    let mut i64_2: i64 = -52i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i32_4: i32 = -105i32;
    let mut i64_3: i64 = -22i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut u16_0: u16 = 52u16;
    let mut i32_5: i32 = 16i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_0);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_7);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut f32_1: f32 = 152.714349f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut u32_2: u32 = 37u32;
    let mut u8_7: u8 = 61u8;
    let mut u8_8: u8 = 38u8;
    let mut u8_9: u8 = 24u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut time_6: crate::time::Time = std::ops::Add::add(time_5, duration_9);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_0, weekday_0);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1498() {
    rusty_monitor::set_test_id(1498);
    let mut i8_0: i8 = 81i8;
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = -53i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = -47i8;
    let mut i8_4: i8 = 105i8;
    let mut i8_5: i8 = -13i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -131i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_1};
    let mut i8_6: i8 = -71i8;
    let mut i8_7: i8 = -119i8;
    let mut i8_8: i8 = 3i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_0: i64 = -145i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u32_1: u32 = 26u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 79u8;
    let mut u8_5: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -50i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_1: i32 = 3i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_3);
    let mut u8_6: u8 = crate::time::Time::second(time_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6352() {
    rusty_monitor::set_test_id(6352);
    let mut i64_0: i64 = -58i64;
    let mut u32_0: u32 = 79u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 3u8;
    let mut i64_1: i64 = -116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 84u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = -45i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 116.257497f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_3: i8 = 25i8;
    let mut i8_4: i8 = -47i8;
    let mut i8_5: i8 = -10i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = 154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_2: u32 = 90u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 9u8;
    let mut u8_8: u8 = 99u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut i64_3: i64 = 28i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i8_6: i8 = 80i8;
    let mut i8_7: i8 = 77i8;
    let mut i8_8: i8 = 88i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_4: i64 = 130i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 33u32;
    let mut u8_9: u8 = 7u8;
    let mut u8_10: u8 = 47u8;
    let mut u8_11: u8 = 94u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_3, padding: padding_0};
    let mut i8_9: i8 = 49i8;
    let mut i8_10: i8 = 118i8;
    let mut i8_11: i8 = -26i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 97i8;
    let mut i8_13: i8 = 66i8;
    let mut i8_14: i8 = -121i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut f64_1: f64 = -121.705496f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i8_15: i8 = 47i8;
    let mut i8_16: i8 = -39i8;
    let mut i8_17: i8 = 91i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut f32_0: f32 = -188.289072f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 67u32;
    let mut u8_12: u8 = 16u8;
    let mut u8_13: u8 = 81u8;
    let mut u8_14: u8 = 25u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_1};
    let mut time_6: crate::time::Time = std::ops::Add::add(time_5, duration_10);
    let mut i8_18: i8 = -31i8;
    let mut i8_19: i8 = 57i8;
    let mut i8_20: i8 = 0i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_21: i8 = 10i8;
    let mut i8_22: i8 = -99i8;
    let mut i8_23: i8 = -33i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i128_0: i128 = -18i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_24: i8 = -76i8;
    let mut i8_25: i8 = 24i8;
    let mut i8_26: i8 = 18i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 28i8;
    let mut i8_28: i8 = 84i8;
    let mut i8_29: i8 = 4i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = -55i8;
    let mut i8_31: i8 = -48i8;
    let mut i8_32: i8 = 43i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i64_5: i64 = 105i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i64_6: i64 = 111i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::abs(duration_14);
    let mut u32_5: u32 = 80u32;
    let mut u8_15: u8 = 2u8;
    let mut u8_16: u8 = 76u8;
    let mut u8_17: u8 = 72u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut time_8: crate::time::Time = std::ops::Sub::sub(time_7, duration_15);
    let mut i64_7: i64 = 160i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_17, duration_16);
    let mut i8_33: i8 = 70i8;
    let mut i8_34: i8 = -120i8;
    let mut i8_35: i8 = -77i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut f64_2: f64 = -47.774042f64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut u32_6: u32 = 50u32;
    let mut u8_18: u8 = 0u8;
    let mut u8_19: u8 = 70u8;
    let mut u8_20: u8 = 9u8;
    let mut time_9: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut time_10: crate::time::Time = std::ops::Add::add(time_9, duration_19);
    let mut f64_3: f64 = 29.520390f64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_3);
    let mut i64_8: i64 = 100i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut i8_36: i8 = 9i8;
    let mut i8_37: i8 = -59i8;
    let mut i8_38: i8 = 32i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i64_9: i64 = 27i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut i32_0: i32 = -134i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_22);
    let mut time_11: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_7: u32 = 67u32;
    let mut u8_21: u8 = 37u8;
    let mut u8_22: u8 = 37u8;
    let mut u8_23: u8 = 50u8;
    let mut time_12: crate::time::Time = crate::time::Time {hour: u8_23, minute: u8_22, second: u8_21, nanosecond: u32_7, padding: padding_2};
    let mut i32_1: i32 = -116i32;
    let mut i64_10: i64 = -19i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_1);
    let mut i32_2: i32 = -8i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_23);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_12);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_11);
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i8_39: i8 = 37i8;
    let mut i8_40: i8 = -18i8;
    let mut i8_41: i8 = 50i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i32_3: i32 = 24i32;
    let mut i64_11: i64 = 144i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_11, i32_3);
    let mut u32_8: u32 = 6u32;
    let mut u8_24: u8 = 90u8;
    let mut u8_25: u8 = 51u8;
    let mut u8_26: u8 = 86u8;
    let mut time_13: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_26, u8_25, u8_24, u32_8);
    let mut time_14: crate::time::Time = std::ops::Add::add(time_13, duration_24);
    let mut i32_4: i32 = -28i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_14};
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_14};
    let mut i8_42: i8 = 7i8;
    let mut i8_43: i8 = -82i8;
    let mut i8_44: i8 = 34i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut f64_4: f64 = -50.300370f64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_4);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_25);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut u32_9: u32 = 54u32;
    let mut u8_27: u8 = 85u8;
    let mut u8_28: u8 = 23u8;
    let mut u8_29: u8 = 14u8;
    let mut time_15: crate::time::Time = crate::time::Time {hour: u8_29, minute: u8_28, second: u8_27, nanosecond: u32_9, padding: padding_3};
    let mut time_16: crate::time::Time = std::ops::Sub::sub(time_15, duration_26);
    let mut i32_5: i32 = -68i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_16);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_4, offset: utcoffset_15};
    let mut i32_6: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_5);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_13);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}