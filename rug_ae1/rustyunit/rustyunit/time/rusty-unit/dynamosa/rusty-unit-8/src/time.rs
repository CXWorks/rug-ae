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
fn rusty_test_1320() {
    rusty_monitor::set_test_id(1320);
    let mut i64_0: i64 = 122i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 85u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -23i32;
    let mut i64_1: i64 = -50i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_2: i64 = -66i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_0: i8 = -126i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_3);
    let mut u16_0: u16 = 85u16;
    let mut i32_1: i32 = 35i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_3: i8 = -41i8;
    let mut i8_4: i8 = 18i8;
    let mut i8_5: i8 = 7i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_1);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 96u8;
    let mut u8_5: u8 = 0u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 164i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_3);
    let mut u32_2: u32 = 46u32;
    let mut u8_6: u8 = 54u8;
    let mut u8_7: u8 = 12u8;
    let mut u8_8: u8 = 52u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_1: u16 = 49u16;
    let mut i32_3: i32 = -225i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_5);
    let mut i32_4: i32 = 43i32;
    let mut i64_3: i64 = -51i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut i8_6: i8 = -78i8;
    let mut i8_7: i8 = -14i8;
    let mut i8_8: i8 = -21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_0: f32 = -41.754070f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_5: i32 = -78i32;
    let mut i64_4: i64 = 47i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut i32_6: i32 = 10i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_6);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_5, duration_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_6, offset: utcoffset_2};
    let mut time_6: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut time_7: crate::time::Time = std::ops::Add::add(time_6, duration_4);
    let mut u8_9: u8 = 63u8;
    let mut u8_10: u8 = 1u8;
    let mut u8_11: u8 = 62u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_11, u8_10, u8_9);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2473() {
    rusty_monitor::set_test_id(2473);
    let mut i64_0: i64 = -138i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 25i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_1: i32 = 78i32;
    let mut i64_2: i64 = 99i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_0: u32 = 91u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 49u32;
    let mut u8_3: u8 = 36u8;
    let mut u8_4: u8 = 69u8;
    let mut u8_5: u8 = 47u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3903() {
    rusty_monitor::set_test_id(3903);
    let mut i32_0: i32 = -58i32;
    let mut i64_0: i64 = 29i64;
    let mut i32_1: i32 = 25i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_2: i32 = 78i32;
    let mut i64_2: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 91u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_2);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 49u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 47u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_3_ref_0, time_1_ref_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4471() {
    rusty_monitor::set_test_id(4471);
    let mut i32_0: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_1: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_2: i32 = -149i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u16_0: u16 = 26u16;
    let mut i32_3: i32 = 241i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut u8_3: u8 = crate::date::Date::day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1104() {
    rusty_monitor::set_test_id(1104);
    let mut f64_0: f64 = 61.028539f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 61u32;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 66u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_0: i64 = 15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 42u32;
    let mut u8_3: u8 = 57u8;
    let mut u8_4: u8 = 16u8;
    let mut u8_5: u8 = 15u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_2);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i64_1: i64 = 13i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 221i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut i64_3: i64 = 175i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut f64_1: f64 = 200.626940f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = -60i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_7);
    let mut i32_0: i32 = 63i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut i64_5: i64 = -65i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i64_6: i64 = 47i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_3_ref_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_2);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(time_3_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6832() {
    rusty_monitor::set_test_id(6832);
    let mut u16_0: u16 = 78u16;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 54u8;
    let mut i32_0: i32 = 28i32;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 45i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_1: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_2: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_6: u8 = 38u8;
    let mut u8_7: u8 = 0u8;
    let mut u8_8: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_1, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i32_3: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_2: u32 = 91u32;
    let mut u8_9: u8 = 56u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_2);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_2);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 49u32;
    let mut u8_12: u8 = 36u8;
    let mut u8_13: u8 = 69u8;
    let mut u8_14: u8 = 47u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_3, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_4_ref_0, time_2_ref_0);
    let mut u8_15: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = std::result::Result::unwrap(result_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_87() {
    rusty_monitor::set_test_id(87);
    let mut i8_0: i8 = 7i8;
    let mut i8_1: i8 = 25i8;
    let mut i8_2: i8 = -22i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 89u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i32_0: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 2u8;
    let mut u8_5: u8 = 45u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut i32_1: i32 = -79i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4226() {
    rusty_monitor::set_test_id(4226);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -51i32;
    let mut i64_0: i64 = 46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_1: i32 = -52i32;
    let mut i64_1: i64 = 69i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 49u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i64_2: i64 = 98i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i32_2: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut i128_0: i128 = -5i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut i32_3: i32 = -102i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = -149i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_5: i32 = 241i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_2_ref_0);
    let mut month_1: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_3);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut month_3: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_0, month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3483() {
    rusty_monitor::set_test_id(3483);
    let mut i32_0: i32 = -14i32;
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_1: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 75u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut month_1: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_786() {
    rusty_monitor::set_test_id(786);
    let mut i64_0: i64 = -20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = -174i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_0: i8 = -75i8;
    let mut i8_1: i8 = 68i8;
    let mut i8_2: i8 = -28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_1: i64 = 46i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = -5i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_1: i32 = -102i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut i32_2: i32 = -149i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_3: i32 = 241i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_3);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_2);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_2);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8089() {
    rusty_monitor::set_test_id(8089);
    let mut u8_0: u8 = 86u8;
    let mut i64_0: i64 = 74i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u16_0: u16 = 47u16;
    let mut i32_0: i32 = 29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 9u32;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 23u8;
    let mut u8_3: u8 = 9u8;
    let mut i128_0: i128 = -5i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 92u32;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 26u8;
    let mut u8_6: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_6, minute: u8_5, second: u8_4, nanosecond: u32_1, padding: padding_0};
    let mut i32_1: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_2: i32 = -149i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_2: u32 = 75u32;
    let mut u8_7: u8 = 8u8;
    let mut u8_8: u8 = 32u8;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_8, u8_0, u8_7, u32_2);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_3, u8_2, u8_1, u32_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1551() {
    rusty_monitor::set_test_id(1551);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -97i32;
    let mut i64_1: i64 = -78i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_1: i32 = -23i32;
    let mut i64_2: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut i32_2: i32 = -54i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_3);
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 57u8;
    let mut i32_3: i32 = -109i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_3: i64 = 171i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut u32_0: u32 = 83u32;
    let mut u8_3: u8 = 19u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 43u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_6);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_3, u8_2, u8_1, u8_0);
    let mut date_4: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_3);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2245() {
    rusty_monitor::set_test_id(2245);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_3};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut padding_4_ref_0: &time::Padding = &mut padding_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_4_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut padding_5: time::Padding = std::clone::Clone::clone(padding_2_ref_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3318() {
    rusty_monitor::set_test_id(3318);
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = 55i8;
    let mut i8_2: i8 = -88i8;
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut month_2: month::Month = crate::month::Month::May;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_147() {
    rusty_monitor::set_test_id(147);
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_3: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5614() {
    rusty_monitor::set_test_id(5614);
    let mut i64_0: i64 = 62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = -5i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1224() {
    rusty_monitor::set_test_id(1224);
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_0: i32 = -54i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 57u8;
    let mut i32_1: i32 = -109i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_1: i64 = 171i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u32_0: u32 = 83u32;
    let mut u8_3: u8 = 19u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 43u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_668() {
    rusty_monitor::set_test_id(668);
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut i64_0: i64 = 175i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut f64_0: f64 = 200.626940f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = -60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_0: i32 = 63i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut u16_0: u16 = 41u16;
    let mut i64_2: i64 = -65i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = 47i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7142() {
    rusty_monitor::set_test_id(7142);
    let mut f32_0: f32 = 11.971652f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = -54i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_1: i64 = 79i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i128_0: i128 = -77i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -149i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_1: i32 = 241i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7914() {
    rusty_monitor::set_test_id(7914);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = 11i32;
    let mut i64_0: i64 = -95i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = -22i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_2: i32 = 28i32;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 45i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 28u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -41i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_2);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_4: i32 = 25i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 53u32;
    let mut u8_6: u8 = 38u8;
    let mut u8_7: u8 = 0u8;
    let mut u8_8: u8 = 7u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_2);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut i32_5: i32 = 78i32;
    let mut i64_2: i64 = 99i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_5);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_3: u32 = 91u32;
    let mut u8_9: u8 = 56u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 60u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_6: crate::time::Time = std::ops::Add::add(time_5, duration_4);
    let mut time_6_ref_0: &crate::time::Time = &mut time_6;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 49u32;
    let mut u8_12: u8 = 36u8;
    let mut u8_13: u8 = 69u8;
    let mut u8_14: u8 = 47u8;
    let mut time_7: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_2};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_6_ref_0, time_4_ref_0);
    let mut u8_15: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(time_1_ref_0, time_0_ref_0);
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_195() {
    rusty_monitor::set_test_id(195);
    let mut i128_0: i128 = 21i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u8_3: u8 = 54u8;
    let mut i64_0: i64 = -29i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -99i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_0: i32 = -77i32;
    let mut i64_2: i64 = 207i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i64_3: i64 = -53i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i8_0: i8 = -65i8;
    let mut i8_1: i8 = -98i8;
    let mut i8_2: i8 = 74i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 57u32;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 44u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_3, u8_4, u32_1);
    let mut i32_1: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = -70i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6328() {
    rusty_monitor::set_test_id(6328);
    let mut i32_0: i32 = 127i32;
    let mut i64_0: i64 = -26i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -57i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = -66i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_2: i32 = -211i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_3: i32 = 173i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut i32_4: i32 = -242i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = -97i32;
    let mut i64_2: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut i32_6: i32 = 118i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 117i8;
    let mut i8_2: i8 = -45i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 69u16;
    let mut i32_7: i32 = -81i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_4, utcoffset_0);
    let mut i64_3: i64 = -70i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_8: i32 = -108i32;
    let mut i64_4: i64 = 19i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_8);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_4);
    let mut time_2: crate::time::Time = std::clone::Clone::clone(time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8222() {
    rusty_monitor::set_test_id(8222);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 47u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u8_3: u8 = 88u8;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 45i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 13u32;
    let mut u8_4: u8 = 5u8;
    let mut u8_5: u8 = 4u8;
    let mut u8_6: u8 = 28u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_0: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_1: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 53u32;
    let mut u8_7: u8 = 0u8;
    let mut u8_8: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_3, nanosecond: u32_2, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i32_2: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_3: u32 = 91u32;
    let mut u8_9: u8 = 9u8;
    let mut u8_10: u8 = 60u8;
    let mut u8_11: u8 = 69u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_9, minute: u8_11, second: u8_10, nanosecond: u32_3, padding: padding_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_897() {
    rusty_monitor::set_test_id(897);
    let mut i32_0: i32 = 122i32;
    let mut i64_0: i64 = 25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 78i8;
    let mut i8_1: i8 = -91i8;
    let mut i8_2: i8 = -71i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_0: u8 = 58u8;
    let mut i32_1: i32 = 75i32;
    let mut f64_0: f64 = -32.307070f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 79u8;
    let mut u8_3: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i64_1: i64 = -56i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u16_0: u16 = 49u16;
    let mut i32_2: i32 = 21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut i64_2: i64 = 123i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_3: i32 = -194i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_3: i64 = 2i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = 26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut u32_1: u32 = 44u32;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 42u8;
    let mut u8_6: u8 = 44u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_7);
    let mut i32_4: i32 = -179i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_847() {
    rusty_monitor::set_test_id(847);
    let mut i128_0: i128 = 104i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 92u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 55u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut i32_0: i32 = 25i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut i32_1: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_1);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i32_2: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_2: u32 = 91u32;
    let mut u8_6: u8 = 56u8;
    let mut u8_7: u8 = 99u8;
    let mut u8_8: u8 = 60u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_3);
    let mut time_5_ref_0: &crate::time::Time = &mut time_5;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 49u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 47u8;
    let mut time_6: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_3, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_5_ref_0, time_3_ref_0);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3049() {
    rusty_monitor::set_test_id(3049);
    let mut i32_0: i32 = -63i32;
    let mut i32_1: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 91u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_2);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 49u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 47u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_3_ref_0, time_1_ref_0);
    let mut u32_3: u32 = crate::time::Time::microsecond(time_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3566() {
    rusty_monitor::set_test_id(3566);
    let mut i32_0: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_1: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 91u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_2);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 49u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 47u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_3_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4714() {
    rusty_monitor::set_test_id(4714);
    let mut u8_0: u8 = 63u8;
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_0: i32 = 80i32;
    let mut i32_1: i32 = 28i32;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 45i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 4u8;
    let mut u8_3: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_2: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 0u8;
    let mut u8_6: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_6, minute: u8_5, second: u8_4, nanosecond: u32_1, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i32_4: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 49u32;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 69u8;
    let mut u8_9: u8 = 47u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_9, minute: u8_8, second: u8_7, nanosecond: u32_2, padding: padding_1};
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8279() {
    rusty_monitor::set_test_id(8279);
    let mut u8_0: u8 = 76u8;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_0: i32 = 82i32;
    let mut i32_1: i32 = 28i32;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 45i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 4u8;
    let mut u8_3: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_2: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_3: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 0u8;
    let mut u8_6: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_6, minute: u8_5, second: u8_4, nanosecond: u32_1, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i32_4: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_2: u32 = 91u32;
    let mut u8_7: u8 = 56u8;
    let mut u8_8: u8 = 99u8;
    let mut u8_9: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_2);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 49u32;
    let mut u8_10: u8 = 36u8;
    let mut u8_11: u8 = 69u8;
    let mut u8_12: u8 = 47u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_12, minute: u8_11, second: u8_10, nanosecond: u32_3, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_4_ref_0, time_2_ref_0);
    let mut u8_13: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4669() {
    rusty_monitor::set_test_id(4669);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u32_0: u32 = 72u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 41u16;
    let mut i32_0: i32 = -100i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = -39i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut f64_0: f64 = 67.422465f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f64_1: f64 = 237.862882f64;
    let mut u32_1: u32 = 63u32;
    let mut i64_0: i64 = 125i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u16_1: u16 = 1u16;
    let mut f64_2: f64 = 78.975080f64;
    let mut i64_1: i64 = 61i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_1);
    let mut f64_3: f64 = crate::duration::Duration::as_seconds_f64(duration_3);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3773() {
    rusty_monitor::set_test_id(3773);
    let mut i64_0: i64 = 126i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 77u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 35u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i32_0: i32 = 25i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut u32_2: u32 = 91u32;
    let mut u8_6: u8 = 56u8;
    let mut u8_7: u8 = 99u8;
    let mut u8_8: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 49u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 47u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_3, padding: padding_1};
    let mut month_0: month::Month = crate::month::Month::August;
    let mut u32_4: u32 = crate::time::Time::nanosecond(time_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_265() {
    rusty_monitor::set_test_id(265);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 75u8;
    let mut u8_1: u8 = 13u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = -61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u32_1: u32 = 11u32;
    let mut u8_3: u8 = 88u8;
    let mut u8_4: u8 = 68u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut i128_0: i128 = 153i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 44u16;
    let mut i32_0: i32 = -45i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut u32_2: u32 = 94u32;
    let mut u8_6: u8 = 69u8;
    let mut u8_7: u8 = 47u8;
    let mut u8_8: u8 = 38u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_1: i32 = 63i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut i8_0: i8 = -71i8;
    let mut i8_1: i8 = 72i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -185i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut f32_0: f32 = -153.103913f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = -55i32;
    let mut i64_2: i64 = -33i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut u16_1: u16 = crate::time::Time::millisecond(time_0);
    let mut month_0: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3701() {
    rusty_monitor::set_test_id(3701);
    let mut i32_0: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 91u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_2);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 49u32;
    let mut u8_3: u8 = 36u8;
    let mut u8_4: u8 = 69u8;
    let mut u8_5: u8 = 47u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5774() {
    rusty_monitor::set_test_id(5774);
    let mut i32_0: i32 = 29i32;
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 6u8;
    let mut i32_1: i32 = 28i32;
    let mut i8_0: i8 = -58i8;
    let mut i8_1: i8 = 51i8;
    let mut i8_2: i8 = -29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -41i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_3: i32 = 25i32;
    let mut i64_0: i64 = -15i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 53u32;
    let mut u8_6: u8 = 38u8;
    let mut u8_7: u8 = 0u8;
    let mut u8_8: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i32_4: i32 = 78i32;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_3: u32 = 91u32;
    let mut u8_9: u8 = 56u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_2);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 49u32;
    let mut u8_12: u8 = 36u8;
    let mut u8_13: u8 = 69u8;
    let mut u8_14: u8 = 47u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_4_ref_0, time_2_ref_0);
    let mut u8_15: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2366() {
    rusty_monitor::set_test_id(2366);
    let mut i8_0: i8 = -15i8;
    let mut i8_1: i8 = -64i8;
    let mut i8_2: i8 = 126i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -23i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 106i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = -16.173945f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = 18i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u16_0: u16 = 69u16;
    let mut i32_0: i32 = -38i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_1: i32 = -19i32;
    let mut i64_1: i64 = 49i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut u16_1: u16 = 63u16;
    let mut i32_2: i32 = -19i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u16_2: u16 = 76u16;
    let mut i32_3: i32 = -33i32;
    let mut i32_4: i32 = -146i32;
    let mut i64_2: i64 = -3i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_5: i32 = 94i32;
    let mut f64_1: f64 = 61.634889f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_5);
    let mut i8_6: i8 = 19i8;
    let mut i8_7: i8 = -74i8;
    let mut i8_8: i8 = -106i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_3, utcoffset_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_6);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3172() {
    rusty_monitor::set_test_id(3172);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_1};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 242i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_2_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut padding_3: time::Padding = std::clone::Clone::clone(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6145() {
    rusty_monitor::set_test_id(6145);
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_0: i32 = -149i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 26u16;
    let mut i32_1: i32 = 241i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7480() {
    rusty_monitor::set_test_id(7480);
    let mut i8_0: i8 = -53i8;
    let mut i8_1: i8 = -78i8;
    let mut i8_2: i8 = 70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -7i32;
    let mut i64_0: i64 = -73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i32_1: i32 = 14i32;
    let mut i64_1: i64 = -90i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_1: u32 = 55u32;
    let mut u8_3: u8 = 37u8;
    let mut u8_4: u8 = 48u8;
    let mut u8_5: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_4);
    let mut i64_2: i64 = -107i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_2: i32 = -127i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i128_0: i128 = -5i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 92u32;
    let mut u8_6: u8 = 38u8;
    let mut u8_7: u8 = 26u8;
    let mut u8_8: u8 = 8u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_0};
    let mut i32_3: i32 = -102i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_4};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_6);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut i32_4: i32 = -149i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u32_3: u32 = 75u32;
    let mut u8_9: u8 = 8u8;
    let mut u8_10: u8 = 20u8;
    let mut u8_11: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_5: i32 = 241i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_11, u8_10, u8_9, u32_3);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_3);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_92() {
    rusty_monitor::set_test_id(92);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_0: i64 = 182i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 25u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 91u32;
    let mut u8_3: u8 = 40u8;
    let mut u8_4: u8 = 48u8;
    let mut u8_5: u8 = 20u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Sub::sub(time_1, time_0);
    let mut i64_1: i64 = 172i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_6: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3506() {
    rusty_monitor::set_test_id(3506);
    let mut u16_0: u16 = 25u16;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 49u8;
    let mut i32_0: i32 = -18i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 88.057758f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 43u32;
    let mut u8_3: u8 = 82u8;
    let mut u8_4: u8 = 87u8;
    let mut u8_5: u8 = 88u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_0: i64 = 158i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = -67i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut i8_0: i8 = -64i8;
    let mut i8_1: i8 = -16i8;
    let mut i8_2: i8 = -25i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 12u32;
    let mut u8_6: u8 = 4u8;
    let mut u8_7: u8 = 68u8;
    let mut u8_8: u8 = 76u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_1, padding: padding_1};
    let mut i128_0: i128 = -5i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 92u32;
    let mut u8_9: u8 = 38u8;
    let mut u8_10: u8 = 26u8;
    let mut u8_11: u8 = 8u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_2, padding: padding_2};
    let mut i32_1: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_2: i32 = -149i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_3: u32 = 75u32;
    let mut u8_12: u8 = 8u8;
    let mut u8_13: u8 = 20u8;
    let mut u8_14: u8 = 32u8;
    let mut u16_1: u16 = 26u16;
    let mut i32_3: i32 = 241i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_3_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_14, u8_13, u8_12, u32_3);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_2);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7300() {
    rusty_monitor::set_test_id(7300);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 31u16;
    let mut i32_0: i32 = -95i32;
    let mut i32_1: i32 = 59i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i128_0: i128 = -5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_2: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_3: i32 = -149i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_1: u16 = 26u16;
    let mut i32_4: i32 = 241i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut u8_6: u8 = crate::date::Date::iso_week(date_0);
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1346() {
    rusty_monitor::set_test_id(1346);
    let mut i128_0: i128 = -80i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_1: i128 = -5i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_1: i32 = -149i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_2: i32 = 241i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_1);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2907() {
    rusty_monitor::set_test_id(2907);
    let mut i64_0: i64 = -46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 154i32;
    let mut i64_1: i64 = -58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut u16_0: u16 = 4u16;
    let mut i32_1: i32 = -118i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 5u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = -156i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_2: i32 = -24i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 25i32;
    let mut i64_3: i64 = -15i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 53u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 7u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_5);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut i32_4: i32 = 78i32;
    let mut i64_4: i64 = 99i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u32_2: u32 = 91u32;
    let mut u8_6: u8 = 56u8;
    let mut u8_7: u8 = 99u8;
    let mut u8_8: u8 = 60u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_6: crate::time::Time = std::ops::Add::add(time_5, duration_7);
    let mut time_6_ref_0: &crate::time::Time = &mut time_6;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_3: u32 = 49u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 47u8;
    let mut time_7: crate::time::Time = crate::time::Time {hour: u8_11, minute: u8_10, second: u8_9, nanosecond: u32_3, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_6_ref_0, time_4_ref_0);
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_5: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2679() {
    rusty_monitor::set_test_id(2679);
    let mut i64_0: i64 = 123i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = 10i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i128_0: i128 = -5i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_1: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut i32_2: i32 = -149i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 75u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 32u8;
    let mut u16_0: u16 = 26u16;
    let mut i32_3: i32 = 241i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut tuple_1: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_2);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut u32_2: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_859() {
    rusty_monitor::set_test_id(859);
    let mut i32_0: i32 = -36i32;
    let mut i64_0: i64 = 28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut f32_0: f32 = -31.931124f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 25i32;
    let mut i64_1: i64 = -15i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 38u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i32_2: i32 = 78i32;
    let mut i64_2: i64 = 99i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_1: u32 = 91u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_5);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 49u32;
    let mut u8_6: u8 = 36u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 47u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_3_ref_0, time_1_ref_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_weeks(duration_2);
    panic!("From RustyUnit with love");
}
}