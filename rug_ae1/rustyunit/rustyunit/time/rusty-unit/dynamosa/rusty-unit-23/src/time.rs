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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_113() {
    rusty_monitor::set_test_id(113);
    let mut i8_0: i8 = -109i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = 13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -57i8;
    let mut i8_4: i8 = -48i8;
    let mut i8_5: i8 = 18i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 46.623727f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i64_0: i64 = 191i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 20i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i64_1: i64 = 122i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = -87i32;
    let mut i64_2: i64 = -294i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut i32_2: i32 = -71i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut i32_3: i32 = 54i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i8_6: i8 = 64i8;
    let mut i8_7: i8 = -14i8;
    let mut i8_8: i8 = -67i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -44i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_4: i32 = -101i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_4, utcoffset_2);
    let mut i64_4: i64 = -137i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = 163i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_1, duration_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = std::option::Option::unwrap(option_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut month_0: month::Month = crate::date::Date::month(date_3);
    let mut u32_1: u32 = crate::time::Time::microsecond(time_3);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7249() {
    rusty_monitor::set_test_id(7249);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut i64_0: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 57i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -62i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i32_1: i32 = 12i32;
    let mut i64_3: i64 = 96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut u16_0: u16 = 39u16;
    let mut i32_2: i32 = -62i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_2};
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_438() {
    rusty_monitor::set_test_id(438);
    let mut i128_0: i128 = 210i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_0);
    let mut i32_0: i32 = -25i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut f64_0: f64 = 107.517915f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = -28i8;
    let mut i8_1: i8 = -32i8;
    let mut i8_2: i8 = 32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -29i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 13i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_1: i128 = 11i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_1: i32 = -235i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut i128_2: i128 = -81i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2930() {
    rusty_monitor::set_test_id(2930);
    let mut i32_0: i32 = -137i32;
    let mut i64_0: i64 = 77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = -31i32;
    let mut i64_1: i64 = 83i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i128_0: i128 = -30i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i64_2: i64 = -25i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_3: i64 = 57i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_4: i64 = -62i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_8);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_7);
    let mut i32_3: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i32_4: i32 = 12i32;
    let mut i64_5: i64 = 96i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_4);
    let mut u16_0: u16 = 39u16;
    let mut i32_5: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_1};
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_2);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_5);
    let mut padding_2: time::Padding = std::clone::Clone::clone(padding_0_ref_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_3);
    let mut i32_6: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1022() {
    rusty_monitor::set_test_id(1022);
    let mut i64_0: i64 = 197i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 40i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i8_0: i8 = -112i8;
    let mut i8_1: i8 = -6i8;
    let mut i8_2: i8 = -117i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -84i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 85i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 21i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = 26i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f64_0: f64 = 93.245967f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 7u32;
    let mut u8_0: u8 = 33u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 1u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_2: crate::time::Time = std::ops::Sub::sub(time_1, duration_3);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i64_2: i64 = -25i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_3: i64 = 57i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_4: i64 = -62i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 26i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_8);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_5);
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_7);
    let mut i32_3: i32 = -94i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i32_4: i32 = 12i32;
    let mut i64_5: i64 = 96i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_4);
    let mut u16_0: u16 = 39u16;
    let mut i32_5: i32 = -62i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_9);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_6, date_4);
    let mut time_5: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_7);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 46u32;
    let mut u8_3: u8 = 58u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 36u8;
    let mut time_6: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_6_ref_0: &crate::time::Time = &mut time_6;
    let mut u32_2: u32 = crate::time::Time::nanosecond(time_5);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_4, duration_5);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(time_2_ref_0);
    let mut tuple_2: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1880() {
    rusty_monitor::set_test_id(1880);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 57i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -62i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i32_1: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 12i32;
    let mut i64_3: i64 = 96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut u16_0: u16 = 39u16;
    let mut i32_3: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_2);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_1);
    let mut tuple_1: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut i64_0: i64 = 129i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 15u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_3);
    let mut u32_1: u32 = 98u32;
    let mut i8_0: i8 = -4i8;
    let mut i64_2: i64 = 118i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 20i8;
    let mut i8_3: i8 = 41i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i32_0: i32 = -96i32;
    let mut i64_3: i64 = 32i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut i32_1: i32 = -125i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut u32_3: u32 = crate::time::Time::microsecond(time_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4342() {
    rusty_monitor::set_test_id(4342);
    let mut i64_0: i64 = -250i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f32_0: f32 = -43.294203f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -21.641354f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 75u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_1: f32 = -95.963377f32;
    let mut i64_1: i64 = -110i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_0: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_0: i8 = 39i8;
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_2: i32 = 32i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut u32_1: u32 = 89u32;
    let mut u8_3: u8 = 73u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 81u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_2);
    let mut i64_2: i64 = -61i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 79i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u32_2: u32 = 1u32;
    let mut u8_6: u8 = 27u8;
    let mut u8_7: u8 = 90u8;
    let mut u8_8: u8 = 2u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_3: i32 = -51i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut time_4: crate::time::Time = std::clone::Clone::clone(time_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_92() {
    rusty_monitor::set_test_id(92);
    let mut i8_0: i8 = -72i8;
    let mut i8_1: i8 = -64i8;
    let mut i8_2: i8 = -21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -93i8;
    let mut i8_4: i8 = 28i8;
    let mut i8_5: i8 = -1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = -49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 84u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 19u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_0);
    let mut i32_0: i32 = 35i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i64_1: i64 = 144i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_1: i32 = 116i32;
    let mut i64_2: i64 = -25i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 70u8;
    let mut u8_4: u8 = 46u8;
    let mut u8_5: u8 = 69u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_1};
    let mut time_3: crate::time::Time = std::ops::Sub::sub(time_2, duration_3);
    let mut i64_3: i64 = 129i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i32_2: i32 = -172i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3231() {
    rusty_monitor::set_test_id(3231);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut i64_0: i64 = -86i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = 170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i8_0: i8 = 77i8;
    let mut i8_1: i8 = 56i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 25u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 16u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 138i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 31i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut i64_3: i64 = 9i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_1: i32 = 165i32;
    let mut i64_4: i64 = 102i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_1);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_2: u32 = 18u32;
    let mut u8_6: u8 = 53u8;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 81u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_3: i8 = 70i8;
    let mut i8_4: i8 = -55i8;
    let mut i8_5: i8 = 26i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(time_2_ref_0, time_0_ref_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut i8_0: i8 = -60i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = -60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_0: i64 = 73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 40u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 62u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_1: crate::time::Time = std::ops::Add::add(time_0, duration_1);
    let mut i32_0: i32 = 84i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_0);
    let mut i64_1: i64 = 71i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 0i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 8u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 86u8;
    let mut u8_5: u8 = 58u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut u8_6: u8 = crate::time::Time::second(time_2);
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_3);
    let mut i64_3: i64 = crate::duration::Duration::whole_hours(duration_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_3);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 63u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 35u8;
    let mut time_0: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut i8_0: i8 = 25i8;
    let mut i8_1: i8 = -22i8;
    let mut i8_2: i8 = 71i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 41u16;
    let mut i32_0: i32 = 47i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(time_1, time_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i8_3: i8 = -6i8;
    let mut i8_4: i8 = 100i8;
    let mut i8_5: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 61u32;
    let mut u8_3: u8 = 47u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 25u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_0: i64 = 129i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -73i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_1: i32 = 38i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6387() {
    rusty_monitor::set_test_id(6387);
    let mut i64_0: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 62i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -62i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut i32_1: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u16_0: u16 = 39u16;
    let mut i32_2: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 4u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 37u8;
    let mut u8_5: u8 = 68u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = 97i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 161i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut duration_0: crate::duration::Duration = std::ops::Sub::sub(time_2, time_0);
    let mut i8_3: i8 = -23i8;
    let mut i8_4: i8 = -29i8;
    let mut i8_5: i8 = 52i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i64_0: i64 = 89i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_523() {
    rusty_monitor::set_test_id(523);
    let mut i128_0: i128 = -118i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = -69i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = 8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 77i8;
    let mut i8_4: i8 = -12i8;
    let mut i8_5: i8 = -28i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = 14i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_3);
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut time_2_ref_0: &crate::time::Time = &mut time_2;
    let mut i64_0: i64 = -35i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut u32_0: u32 = 72u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 63u8;
    let mut u8_2: u8 = 4u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut time_4: crate::time::Time = std::ops::Sub::sub(time_3, duration_5);
    let mut i32_1: i32 = -33i32;
    let mut i32_2: i32 = -1i32;
    let mut i64_1: i64 = 73i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_1);
    let mut i32_3: i32 = 4i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_7);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_4);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 78u8;
    let mut u8_4: u8 = 54u8;
    let mut u8_5: u8 = 40u8;
    let mut time_5: crate::time::Time = crate::time::Time {hour: u8_5, minute: u8_4, second: u8_3, nanosecond: u32_1, padding: padding_0};
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 93u32;
    let mut u8_6: u8 = 19u8;
    let mut u8_7: u8 = 63u8;
    let mut u8_8: u8 = 42u8;
    let mut time_6: crate::time::Time = crate::time::Time {hour: u8_8, minute: u8_7, second: u8_6, nanosecond: u32_2, padding: padding_1};
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(time_6, time_5);
    let mut u32_3: u32 = 80u32;
    let mut u8_9: u8 = 46u8;
    let mut u8_10: u8 = 29u8;
    let mut u8_11: u8 = 43u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u32_4: u32 = 66u32;
    let mut u8_12: u8 = 22u8;
    let mut u8_13: u8 = 13u8;
    let mut u8_14: u8 = 10u8;
    let mut time_8: crate::time::Time = crate::time::Time {hour: u8_14, minute: u8_13, second: u8_12, nanosecond: u32_4, padding: padding_2};
    let mut i32_4: i32 = -3i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_8);
    let mut i32_5: i32 = 113i32;
    let mut i64_2: i64 = 5i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_5);
    let mut u32_5: u32 = 70u32;
    let mut u8_15: u8 = 70u8;
    let mut u8_16: u8 = 93u8;
    let mut u8_17: u8 = 89u8;
    let mut time_9: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut time_10: crate::time::Time = std::ops::Add::add(time_9, duration_10);
    let mut i32_6: i32 = -66i32;
    let mut i64_3: i64 = 12i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_6);
    let mut u16_0: u16 = 77u16;
    let mut i32_7: i32 = -152i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_11);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_6, time_10);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_3, primitivedatetime_1);
    let mut i32_8: i32 = 54i32;
    let mut i64_4: i64 = 179i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut padding_3_ref_0: &time::Padding = &mut padding_3;
    let mut padding_4: time::Padding = crate::time::Padding::Optimize;
    let mut padding_4_ref_0: &time::Padding = &mut padding_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_4_ref_0, padding_3_ref_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_12, i32_8);
    let mut time_11: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut bool_1: bool = std::cmp::PartialEq::eq(time_2_ref_0, time_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3221() {
    rusty_monitor::set_test_id(3221);
    let mut f32_0: f32 = 136.981699f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut i64_0: i64 = 32i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -25i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i64_3: i64 = -62i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_6);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_5);
    let mut i32_1: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 12i32;
    let mut i64_4: i64 = 96i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut u16_0: u16 = 39u16;
    let mut i32_3: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_7);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_3: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_2);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4115() {
    rusty_monitor::set_test_id(4115);
    let mut i32_0: i32 = -69i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = -25i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_1: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 12i32;
    let mut i64_1: i64 = 96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u16_0: u16 = 39u16;
    let mut i32_3: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_1: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    let mut time_1_ref_0: &crate::time::Time = &mut time_1;
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_0);
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6270() {
    rusty_monitor::set_test_id(6270);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut i8_0: i8 = -43i8;
    let mut i8_1: i8 = 112i8;
    let mut i8_2: i8 = -21i8;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 34i8;
    let mut i8_5: i8 = -84i8;
    let mut i64_0: i64 = 69i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_6: i8 = 13i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 14i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u16_0: u16 = 9u16;
    let mut i32_0: i32 = -199i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7270() {
    rusty_monitor::set_test_id(7270);
    let mut i64_0: i64 = 57i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -62i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_1);
    let mut i32_1: i32 = -94i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u16_0: u16 = 39u16;
    let mut i32_2: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 36u8;
    let mut time_2: crate::time::Time = crate::time::Time {hour: u8_2, minute: u8_1, second: u8_0, nanosecond: u32_0, padding: padding_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u32_0: u32 = 65u32;
    let mut i64_0: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut f64_0: f64 = 105.662392f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -43i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = -64i8;
    let mut i8_1: i8 = -39i8;
    let mut i8_2: i8 = -110i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut time_1: crate::time::Time = std::ops::Sub::sub(time_0, duration_2);
    let mut i8_3: i8 = -11i8;
    let mut i8_4: i8 = 65i8;
    let mut i8_5: i8 = -58i8;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}
}