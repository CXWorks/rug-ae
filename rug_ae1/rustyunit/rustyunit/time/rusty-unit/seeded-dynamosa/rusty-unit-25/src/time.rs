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
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_514() {
//    rusty_monitor::set_test_id(514);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut padding_4_ref_0: &time::Padding = &mut padding_4;
    let mut padding_5: time::Padding = crate::time::Padding::Optimize;
    let mut padding_5_ref_0: &time::Padding = &mut padding_5;
    let mut padding_6: time::Padding = crate::time::Padding::Optimize;
    let mut padding_6_ref_0: &time::Padding = &mut padding_6;
    let mut padding_7: time::Padding = crate::time::Padding::Optimize;
    let mut padding_7_ref_0: &time::Padding = &mut padding_7;
    let mut padding_8: time::Padding = crate::time::Padding::Optimize;
    let mut padding_8_ref_0: &time::Padding = &mut padding_8;
    let mut padding_9: time::Padding = crate::time::Padding::Optimize;
    let mut padding_9_ref_0: &time::Padding = &mut padding_9;
    let mut padding_10: time::Padding = crate::time::Padding::Optimize;
    let mut padding_10_ref_0: &time::Padding = &mut padding_10;
    let mut padding_11: time::Padding = crate::time::Padding::Optimize;
    let mut padding_11_ref_0: &time::Padding = &mut padding_11;
    let mut padding_12: time::Padding = crate::time::Padding::Optimize;
    let mut padding_12_ref_0: &time::Padding = &mut padding_12;
    let mut padding_13: time::Padding = crate::time::Padding::Optimize;
    let mut padding_13_ref_0: &time::Padding = &mut padding_13;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_13_ref_0, padding_12_ref_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_11_ref_0, padding_10_ref_0);
    let mut option_2: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_9_ref_0, padding_8_ref_0);
    let mut option_3: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_7_ref_0, padding_6_ref_0);
    let mut option_4: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_5_ref_0, padding_4_ref_0);
    let mut option_5: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_3_ref_0, padding_2_ref_0);
    let mut option_6: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8585() {
//    rusty_monitor::set_test_id(8585);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 77u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 5u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 28u8;
    let mut i64_1: i64 = 24i64;
    let mut i128_0: i128 = 1i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 1000000u32;
    let mut u8_6: u8 = 23u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i64_3: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_4);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i64_4: i64 = 24i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut u32_3: u32 = 10u32;
    let mut u8_9: u8 = 1u8;
    let mut u8_10: u8 = 7u8;
    let mut u8_11: u8 = 29u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_5: crate::time::Time = std::ops::Sub::sub(time_4, duration_6);
    let mut time_5_ref_0: &crate::time::Time = &mut time_5;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_1);
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut time_6_ref_0: &crate::time::Time = &mut time_6;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(time_6_ref_0, time_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4713() {
//    rusty_monitor::set_test_id(4713);
    let mut i8_0: i8 = 16i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 381i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i32_2: i32 = -115i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_3: i32 = 43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_4: i32 = 97i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_4);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_5);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_3);
    let mut u8_5: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3506() {
//    rusty_monitor::set_test_id(3506);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_1);
    let mut time_2_ref_0: &mut crate::time::Time = &mut time_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6897() {
//    rusty_monitor::set_test_id(6897);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_0: i64 = 144i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 2u8;
    let mut u8_5: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_3);
    let mut i32_1: i32 = 82i32;
    let mut i64_2: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_3: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut u16_1: u16 = 93u16;
    let mut i32_2: i32 = 365i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_4);
    let mut u32_2: u32 = 96u32;
    let mut u8_6: u8 = 12u8;
    let mut u8_7: u8 = 53u8;
    let mut u8_8: u8 = 6u8;
    let mut i64_4: i64 = 604800i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_3: i32 = 3652425i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_5, u8_8, u8_7, u8_6, u32_2);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(time_1_ref_0, time_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3861() {
//    rusty_monitor::set_test_id(3861);
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 4u8;
    let mut i32_0: i32 = 0i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 142i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 999999u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut i64_2: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 285i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_6);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_4);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = 48i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(time_3, time_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4096() {
//    rusty_monitor::set_test_id(4096);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 2u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 511i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2517() {
//    rusty_monitor::set_test_id(2517);
    let mut i64_0: i64 = 59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 3600i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_5);
    let mut i32_1: i32 = 240i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_2: i32 = 229i32;
    let mut i64_3: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_2);
    let mut i64_4: i64 = 1000i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_8);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_7);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 7u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_4);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_3);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2549() {
//    rusty_monitor::set_test_id(2549);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 0u8;
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 999999999u32;
    let mut u8_6: u8 = 12u8;
    let mut u8_7: u8 = 3u8;
    let mut u8_8: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_2);
    let mut time_3_ref_0: &mut crate::time::Time = &mut time_3;
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(time_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5608() {
//    rusty_monitor::set_test_id(5608);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = -128i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i8_6: i8 = -122i8;
    let mut i8_7: i8 = 16i8;
    let mut i8_8: i8 = 3i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u16_1: u16 = 999u16;
    let mut i32_1: i32 = 116i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_3);
    let mut i8_9: i8 = 24i8;
    let mut i8_10: i8 = 4i8;
    let mut i8_11: i8 = 2i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_3: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_6);
    let mut i32_2: i32 = 240i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_4};
    let mut i32_3: i32 = 229i32;
    let mut i64_4: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_3);
    let mut i64_5: i64 = 1000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_9);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut time_5: crate::time::Time = std::ops::Sub::sub(time_4, duration_8);
    let mut f64_1: f64 = 4794699203894837248.000000f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 7u8;
    let mut time_6: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_7, time_6);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_8);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_10);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_5);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_2);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = crate::date::Date::iso_week(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7085() {
//    rusty_monitor::set_test_id(7085);
    let mut i128_0: i128 = 82i128;
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i32_1: i32 = 240i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_2: i32 = 229i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 7u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_5);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_3);
    let mut month_1: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3646() {
//    rusty_monitor::set_test_id(3646);
    let mut i32_0: i32 = -191i32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 320i32;
    let mut i64_0: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_2: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 44i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_6: i8 = 1i8;
    let mut i8_7: i8 = 1i8;
    let mut i8_8: i8 = 23i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_4: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_3: i32 = 5119853i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut u16_0: u16 = 1u16;
    let mut i32_4: i32 = 86399i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_3);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut u32_0: u32 = crate::time::Time::microsecond(time_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_589() {
//    rusty_monitor::set_test_id(589);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = 99i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(time_1, time_0);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1320() {
//    rusty_monitor::set_test_id(1320);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = 1000i32;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_1);
    let mut time_2_ref_0: &mut crate::time::Time = &mut time_2;
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_0, month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_981() {
//    rusty_monitor::set_test_id(981);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 87i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 252i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut i64_0: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = 331i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_78() {
//    rusty_monitor::set_test_id(78);
    let mut i8_0: i8 = -22i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i8_6: i8 = 24i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 156i32;
    let mut i128_0: i128 = -152i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_1: u32 = 1000u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_3);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_2: i32 = 365i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_2};
    let mut offsetdatetime_1_ref_0: &crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_218() {
//    rusty_monitor::set_test_id(218);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 71i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 105i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_0: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 127i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = 359i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_2);
    let mut tuple_1: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3087() {
//    rusty_monitor::set_test_id(3087);
    let mut i32_0: i32 = 336i32;
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_1: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7645() {
//    rusty_monitor::set_test_id(7645);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i32_0: i32 = 100i32;
    let mut i64_2: i64 = 604800i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i32_1: i32 = 3600i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut f32_0: f32 = 4.940469f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut f64_0: f64 = -196.209922f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 6i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 60u16;
    let mut i32_2: i32 = 133i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_9);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_6);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut f64_1: f64 = 4815374002031689728.000000f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 7u8;
    let mut u8_4: u8 = 10u8;
    let mut u8_5: u8 = 1u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut i64_3: i64 = 1000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 100000000u32;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 24u8;
    let mut u8_8: u8 = 37u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut time_6: crate::time::Time = std::ops::Sub::sub(time_5, duration_13);
    let mut i32_3: i32 = 3600i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_4);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_10);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8442() {
//    rusty_monitor::set_test_id(8442);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 52u8;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut i32_0: i32 = -115i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 94u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut i32_2: i32 = 25i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_0);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_3);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_221() {
//    rusty_monitor::set_test_id(221);
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 75u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 53u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 5i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 10000u32;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 24u8;
    let mut u8_8: u8 = 29u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_0};
    let mut i32_2: i32 = 3652425i32;
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_3: u32 = 21u32;
    let mut u8_9: u8 = 6u8;
    let mut u8_10: u8 = 28u8;
    let mut u8_11: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = 1000000i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_3};
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4131() {
//    rusty_monitor::set_test_id(4131);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i64_0: i64 = 2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut i32_0: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut i32_1: i32 = -9i32;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 11u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_4);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8125() {
//    rusty_monitor::set_test_id(8125);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i64_0: i64 = 2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_0: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = -9i32;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4434() {
//    rusty_monitor::set_test_id(4434);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 23u16;
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut i32_1: i32 = 291i32;
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 88u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_2);
    let mut i32_2: i32 = 47i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i32_3: i32 = 65i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 4u8;
    let mut u8_5: u8 = 2u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_4);
    let mut u16_1: u16 = 365u16;
    let mut i32_4: i32 = 511i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_3, date_3);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_2_ref_0, padding_1_ref_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3132() {
//    rusty_monitor::set_test_id(3132);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 87i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 252i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i64_0: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_2: i32 = 331i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4603() {
//    rusty_monitor::set_test_id(4603);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_2);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i64_2: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1810() {
//    rusty_monitor::set_test_id(1810);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 142i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut i64_2: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 285i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 48i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_340() {
//    rusty_monitor::set_test_id(340);
    let mut i32_0: i32 = 161i32;
    let mut i64_0: i64 = 38i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut time_1_ref_0: &mut crate::time::Time = &mut time_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4818() {
//    rusty_monitor::set_test_id(4818);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 87i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 252i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_0);
    let mut i32_2: i32 = 331i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_419() {
//    rusty_monitor::set_test_id(419);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 83u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 73u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 252i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u16_1: u16 = 0u16;
    let mut i32_1: i32 = 353i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i32_2: i32 = 376i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(time_3, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5340() {
//    rusty_monitor::set_test_id(5340);
    let mut i32_0: i32 = 43i32;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = 1000000i32;
    let mut i64_1: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_2);
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 52u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_3);
    let mut u16_0: u16 = 367u16;
    let mut i32_2: i32 = 221i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_3: i32 = -115i32;
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut i64_4: i64 = 12i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut i32_4: i32 = 48i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 100000u32;
    let mut u8_6: u8 = 94u8;
    let mut u8_7: u8 = 24u8;
    let mut u8_8: u8 = 30u8;
    let mut time_4: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_2};
    let mut i32_5: i32 = 25i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_3, primitivedatetime_4);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_5, utcoffset_1);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_6);
    let mut u8_10: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_4);
    let mut u8_11: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_2);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7218() {
//    rusty_monitor::set_test_id(7218);
    let mut i64_0: i64 = 144i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u16_0: u16 = 365u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_1: i32 = 82i32;
    let mut i64_2: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_3: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut u32_1: u32 = 96u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 6u8;
    let mut i64_4: i64 = 604800i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i32_2: i32 = 3652425i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4228() {
//    rusty_monitor::set_test_id(4228);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_1: i32 = 229i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_531() {
//    rusty_monitor::set_test_id(531);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut padding_4_ref_0: &time::Padding = &mut padding_4;
    let mut padding_5: time::Padding = crate::time::Padding::Optimize;
    let mut padding_5_ref_0: &time::Padding = &mut padding_5;
    let mut padding_6: time::Padding = crate::time::Padding::Optimize;
    let mut padding_6_ref_0: &time::Padding = &mut padding_6;
    let mut padding_7: time::Padding = crate::time::Padding::Optimize;
    let mut padding_7_ref_0: &time::Padding = &mut padding_7;
    let mut padding_8: time::Padding = crate::time::Padding::Optimize;
    let mut padding_8_ref_0: &time::Padding = &mut padding_8;
    let mut padding_9: time::Padding = crate::time::Padding::Optimize;
    let mut padding_9_ref_0: &time::Padding = &mut padding_9;
    let mut padding_10: time::Padding = crate::time::Padding::Optimize;
    let mut padding_10_ref_0: &time::Padding = &mut padding_10;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_10_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_9_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_8_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_7_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_6_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_5_ref_0);
    let mut tuple_6: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_4_ref_0);
    let mut tuple_7: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_3_ref_0);
    let mut tuple_8: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_2_ref_0);
    let mut tuple_9: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_1_ref_0);
    let mut tuple_10: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8579() {
//    rusty_monitor::set_test_id(8579);
    let mut i64_0: i64 = 12i64;
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 28u8;
    let mut i64_1: i64 = 24i64;
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_1);
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2969() {
//    rusty_monitor::set_test_id(2969);
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut f32_0: f32 = 4.940469f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut f64_1: f64 = 4815374002031689728.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 1u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = 1000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 37u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_8);
    let mut i32_1: i32 = 3600i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_112() {
//    rusty_monitor::set_test_id(112);
    let mut i64_0: i64 = 208i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = -65i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_2_ref_0, padding_1_ref_0);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6955() {
//    rusty_monitor::set_test_id(6955);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_0: f32 = 4.940469f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f64_0: f64 = -196.209922f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut f64_1: f64 = 4815374002031689728.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 1u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = 1000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 37u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_8);
    let mut i32_1: i32 = 3600i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_5);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2814() {
//    rusty_monitor::set_test_id(2814);
    let mut f32_0: f32 = 4.940469f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f64_0: f64 = -196.209922f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_0: i64 = 1000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 37u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_1};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_5);
    let mut i32_1: i32 = 3600i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2942() {
//    rusty_monitor::set_test_id(2942);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut u16_1: u16 = 365u16;
    let mut i32_1: i32 = 88i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 3u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_2: crate::time::Time = std::ops::Add::add(time_1, duration_1);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_2: i32 = 240i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_1: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_1);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut time_4: crate::time::Time = std::ops::Add::add(time_3, duration_3);
    let mut time_4_ref_0: &crate::time::Time = &mut time_4;
    let mut i64_2: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 29u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_6: crate::time::Time = std::ops::Sub::sub(time_5, duration_5);
    let mut time_6_ref_0: &crate::time::Time = &mut time_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(time_6_ref_0, time_4_ref_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1797() {
//    rusty_monitor::set_test_id(1797);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 604800i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = -9i32;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i64_3: i64 = 604800i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3119() {
//    rusty_monitor::set_test_id(3119);
    let mut i128_0: i128 = 1i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i64_1: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i128_1: i128 = 1000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_2: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 29u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_5);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(time_3_ref_0, time_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1139() {
//    rusty_monitor::set_test_id(1139);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 229i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1861() {
//    rusty_monitor::set_test_id(1861);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut i128_1: i128 = 1i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_3);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_1: i32 = 240i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i64_2: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i128_2: i128 = 1000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_5);
    let mut time_5_ref_0: &crate::time::Time = &mut time_5;
    let mut i64_3: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 1u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 29u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_7: crate::time::Time = std::ops::Sub::sub(time_6, duration_7);
    let mut time_7_ref_0: &crate::time::Time = &mut time_7;
    let mut bool_0: bool = std::cmp::PartialEq::eq(time_7_ref_0, time_5_ref_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5399() {
//    rusty_monitor::set_test_id(5399);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut u8_3: u8 = 55u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 37i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 6u8;
    let mut u8_6: u8 = 64u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut bool_0: bool = std::cmp::PartialEq::ne(time_1_ref_0, time_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1029() {
//    rusty_monitor::set_test_id(1029);
    let mut i128_0: i128 = 9223372036854775807i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = -22i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 15i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_2);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut i64_1: i64 = 144i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 7u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_3: crate::time::Time = std::ops::Add::add(time_2, duration_3);
    let mut i64_2: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 37i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_3);
    let mut i32_2: i32 = 82i32;
    let mut i64_3: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_4: i64 = 0i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut u16_1: u16 = 93u16;
    let mut i32_3: i32 = 365i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_7);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_4);
    let mut u32_1: u32 = 96u32;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 53u8;
    let mut u8_5: u8 = 6u8;
    let mut i64_5: i64 = 604800i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i32_4: i32 = 3652425i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_8);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_6, u8_5, u8_4, u8_3, u32_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut time_5: crate::time::Time = std::clone::Clone::clone(time_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1219() {
//    rusty_monitor::set_test_id(1219);
    let mut i32_0: i32 = 320i32;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_1: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_3: i8 = 1i8;
    let mut i8_4: i8 = 1i8;
    let mut i8_5: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 5119853i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut u32_0: u32 = crate::time::Time::microsecond(time_1);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8234() {
//    rusty_monitor::set_test_id(8234);
    let mut i32_0: i32 = -115i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_2: i32 = 25i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_1, primitivedatetime_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_4);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_2);
    let mut u8_5: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_696() {
//    rusty_monitor::set_test_id(696);
    let mut f64_0: f64 = -86.648912f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 59u16;
    let mut i32_0: i32 = 392i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i64_0: i64 = 141i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i32_1: i32 = 348i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i64_1: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 17u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut u16_1: u16 = 1u16;
    let mut i32_2: i32 = 400i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_3);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_2);
    let mut tuple_1: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_498() {
//    rusty_monitor::set_test_id(498);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut padding_2_ref_0: &time::Padding = &mut padding_2;
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut padding_4_ref_0: &time::Padding = &mut padding_4;
    let mut padding_5: time::Padding = crate::time::Padding::Optimize;
    let mut padding_5_ref_0: &time::Padding = &mut padding_5;
    let mut padding_6: time::Padding = crate::time::Padding::Optimize;
    let mut padding_6_ref_0: &time::Padding = &mut padding_6;
    let mut padding_7: time::Padding = crate::time::Padding::Optimize;
    let mut padding_7_ref_0: &time::Padding = &mut padding_7;
    let mut padding_8: time::Padding = crate::time::Padding::Optimize;
    let mut padding_8_ref_0: &time::Padding = &mut padding_8;
    let mut padding_9: time::Padding = crate::time::Padding::Optimize;
    let mut padding_9_ref_0: &time::Padding = &mut padding_9;
    let mut padding_10: time::Padding = crate::time::Padding::Optimize;
    let mut padding_10_ref_0: &time::Padding = &mut padding_10;
    let mut padding_11: time::Padding = std::clone::Clone::clone(padding_10_ref_0);
    let mut padding_12: time::Padding = std::clone::Clone::clone(padding_9_ref_0);
    let mut padding_13: time::Padding = std::clone::Clone::clone(padding_8_ref_0);
    let mut padding_14: time::Padding = std::clone::Clone::clone(padding_7_ref_0);
    let mut padding_15: time::Padding = std::clone::Clone::clone(padding_6_ref_0);
    let mut padding_16: time::Padding = std::clone::Clone::clone(padding_5_ref_0);
    let mut padding_17: time::Padding = std::clone::Clone::clone(padding_4_ref_0);
    let mut padding_18: time::Padding = std::clone::Clone::clone(padding_3_ref_0);
    let mut padding_19: time::Padding = std::clone::Clone::clone(padding_2_ref_0);
    let mut padding_20: time::Padding = std::clone::Clone::clone(padding_1_ref_0);
    let mut padding_21: time::Padding = std::clone::Clone::clone(padding_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_182() {
//    rusty_monitor::set_test_id(182);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 87u8;
    let mut u8_5: u8 = 12u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 10u32;
    let mut u8_6: u8 = 31u8;
    let mut u8_7: u8 = 29u8;
    let mut u8_8: u8 = 10u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut i64_2: i64 = 604800i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1696() {
//    rusty_monitor::set_test_id(1696);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i32_0: i32 = 240i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_1: i32 = 229i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i64_2: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7870() {
//    rusty_monitor::set_test_id(7870);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 9u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -115i32;
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_1: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 94u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut i32_2: i32 = 25i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_1, primitivedatetime_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_2);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_0, duration_2);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2038() {
//    rusty_monitor::set_test_id(2038);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 31u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_0);
    let mut i32_0: i32 = -115i32;
    let mut i64_0: i64 = 15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_1: i32 = 38i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 94u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 30u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut i32_2: i32 = 25i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_6: u8 = crate::time::Time::second(time_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8885() {
//    rusty_monitor::set_test_id(8885);
    let mut i32_0: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i32_1: i32 = 25i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 127i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_3);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3746() {
//    rusty_monitor::set_test_id(3746);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = 103i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 60i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u16_1: u16 = 365u16;
    let mut i32_2: i32 = 48i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i32_3: i32 = 348i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 1u8;
    let mut i8_0: i8 = 127i8;
    let mut i8_1: i8 = -28i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_2: u16 = 59u16;
    let mut i32_4: i32 = 133i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_3);
    let mut u16_3: u16 = 367u16;
    let mut i32_5: i32 = 66i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_4, date_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_5, offset: utcoffset_0};
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 29u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 76u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut i64_3: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_2: u32 = 999999u32;
    let mut u8_6: u8 = 31u8;
    let mut u8_7: u8 = 24u8;
    let mut u8_8: u8 = 4u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut time_5: crate::time::Time = std::ops::Add::add(time_4, duration_5);
    let mut time_5_ref_0: &crate::time::Time = &mut time_5;
    let mut i32_6: i32 = -63i32;
    let mut i64_4: i64 = 1000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_6);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u32_3: u32 = 100000000u32;
    let mut u8_9: u8 = 4u8;
    let mut u8_10: u8 = 28u8;
    let mut u8_11: u8 = 3u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut time_7: crate::time::Time = std::ops::Sub::sub(time_6, duration_7);
    let mut time_7_ref_0: &crate::time::Time = &mut time_7;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(time_7_ref_0, time_5_ref_0);
    let mut ordering_1: std::cmp::Ordering = std::cmp::Ord::cmp(time_3_ref_0, time_2_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(time_1_ref_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
//    panic!("From RustyUnit with love");
}
}