//! The [`Time`] struct and its associated `impl`s.
use core::fmt;
use core::ops::{Add, Sub};
use core::time::Duration as StdDuration;
#[cfg(feature = "formatting")]
use std::io;
use crate::convert::*;
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
impl Time {
    /// Create a `Time` that is exactly midnight.
    ///
    /// ```rust
    /// # use time::Time;
    /// # use time_macros::time;
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
    pub(crate) const MAX: Self = Self::__from_hms_nanos_unchecked(
        23,
        59,
        59,
        999_999_999,
    );
    /// Create a `Time` from its components.
    #[doc(hidden)]
    pub const fn __from_hms_nanos_unchecked(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Self {
        debug_assert!(hour < Hour.per(Day));
        debug_assert!(minute < Minute.per(Hour));
        debug_assert!(second < Second.per(Minute));
        debug_assert!(nanosecond < Nanosecond.per(Second));
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
    pub const fn from_hms(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => Hour.per(Day) - 1);
        ensure_value_in_range!(minute in 0 => Minute.per(Hour) - 1);
        ensure_value_in_range!(second in 0 => Second.per(Minute) - 1);
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
        ensure_value_in_range!(hour in 0 => Hour.per(Day) - 1);
        ensure_value_in_range!(minute in 0 => Minute.per(Hour) - 1);
        ensure_value_in_range!(second in 0 => Second.per(Minute) - 1);
        ensure_value_in_range!(millisecond in 0 => Millisecond.per(Second) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                hour,
                minute,
                second,
                millisecond as u32 * Nanosecond.per(Millisecond),
            ),
        )
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
        ensure_value_in_range!(hour in 0 => Hour.per(Day) - 1);
        ensure_value_in_range!(minute in 0 => Minute.per(Hour) - 1);
        ensure_value_in_range!(second in 0 => Second.per(Minute) - 1);
        ensure_value_in_range!(microsecond in 0 => Microsecond.per(Second) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                hour,
                minute,
                second,
                microsecond * Nanosecond.per(Microsecond) as u32,
            ),
        )
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
        ensure_value_in_range!(hour in 0 => Hour.per(Day) - 1);
        ensure_value_in_range!(minute in 0 => Minute.per(Hour) - 1);
        ensure_value_in_range!(second in 0 => Second.per(Minute) - 1);
        ensure_value_in_range!(nanosecond in 0 => Nanosecond.per(Second) - 1);
        Ok(Self::__from_hms_nanos_unchecked(hour, minute, second, nanosecond))
    }
    /// Get the clock hour, minute, and second.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00:00).as_hms(), (0, 0, 0));
    /// assert_eq!(time!(23:59:59).as_hms(), (23, 59, 59));
    /// ```
    pub const fn as_hms(self) -> (u8, u8, u8) {
        (self.hour, self.minute, self.second)
    }
    /// Get the clock hour, minute, second, and millisecond.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00:00).as_hms_milli(), (0, 0, 0, 0));
    /// assert_eq!(time!(23:59:59.999).as_hms_milli(), (23, 59, 59, 999));
    /// ```
    pub const fn as_hms_milli(self) -> (u8, u8, u8, u16) {
        (
            self.hour,
            self.minute,
            self.second,
            (self.nanosecond / Nanosecond.per(Millisecond)) as u16,
        )
    }
    /// Get the clock hour, minute, second, and microsecond.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00:00).as_hms_micro(), (0, 0, 0, 0));
    /// assert_eq!(
    ///     time!(23:59:59.999_999).as_hms_micro(),
    ///     (23, 59, 59, 999_999)
    /// );
    /// ```
    pub const fn as_hms_micro(self) -> (u8, u8, u8, u32) {
        (
            self.hour,
            self.minute,
            self.second,
            self.nanosecond / Nanosecond.per(Microsecond) as u32,
        )
    }
    /// Get the clock hour, minute, second, and nanosecond.
    ///
    /// ```rust
    /// # use time_macros::time;
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
    /// # use time_macros::time;
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
    /// # use time_macros::time;
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
    /// # use time_macros::time;
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
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00).millisecond(), 0);
    /// assert_eq!(time!(23:59:59.999).millisecond(), 999);
    /// ```
    pub const fn millisecond(self) -> u16 {
        (self.nanosecond / Nanosecond.per(Millisecond)) as _
    }
    /// Get the microseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00).microsecond(), 0);
    /// assert_eq!(time!(23:59:59.999_999).microsecond(), 999_999);
    /// ```
    pub const fn microsecond(self) -> u32 {
        self.nanosecond / Nanosecond.per(Microsecond) as u32
    }
    /// Get the nanoseconds within the second.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00).nanosecond(), 0);
    /// assert_eq!(time!(23:59:59.999_999_999).nanosecond(), 999_999_999);
    /// ```
    pub const fn nanosecond(self) -> u32 {
        self.nanosecond
    }
    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow, returning whether
    /// the date is different.
    pub(crate) const fn adjusting_add(
        self,
        duration: Duration,
    ) -> (DateAdjustment, Self) {
        let mut nanoseconds = self.nanosecond as i32 + duration.subsec_nanoseconds();
        let mut seconds = self.second as i8
            + (duration.whole_seconds() % Second.per(Minute) as i64) as i8;
        let mut minutes = self.minute as i8
            + (duration.whole_minutes() % Minute.per(Hour) as i64) as i8;
        let mut hours = self.hour as i8
            + (duration.whole_hours() % Hour.per(Day) as i64) as i8;
        let mut date_adjustment = DateAdjustment::None;
        cascade!(nanoseconds in 0..Nanosecond.per(Second) as _ => seconds);
        cascade!(seconds in 0..Second.per(Minute) as _ => minutes);
        cascade!(minutes in 0..Minute.per(Hour) as _ => hours);
        if hours >= Hour.per(Day) as _ {
            hours -= Hour.per(Day) as i8;
            date_adjustment = DateAdjustment::Next;
        } else if hours < 0 {
            hours += Hour.per(Day) as i8;
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
    pub(crate) const fn adjusting_sub(
        self,
        duration: Duration,
    ) -> (DateAdjustment, Self) {
        let mut nanoseconds = self.nanosecond as i32 - duration.subsec_nanoseconds();
        let mut seconds = self.second as i8
            - (duration.whole_seconds() % Second.per(Minute) as i64) as i8;
        let mut minutes = self.minute as i8
            - (duration.whole_minutes() % Minute.per(Hour) as i64) as i8;
        let mut hours = self.hour as i8
            - (duration.whole_hours() % Hour.per(Day) as i64) as i8;
        let mut date_adjustment = DateAdjustment::None;
        cascade!(nanoseconds in 0..Nanosecond.per(Second) as _ => seconds);
        cascade!(seconds in 0..Second.per(Minute) as _ => minutes);
        cascade!(minutes in 0..Minute.per(Hour) as _ => hours);
        if hours >= Hour.per(Day) as _ {
            hours -= Hour.per(Day) as i8;
            date_adjustment = DateAdjustment::Next;
        } else if hours < 0 {
            hours += Hour.per(Day) as i8;
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
        let mut second = self.second
            + (duration.as_secs() % Second.per(Minute) as u64) as u8;
        let mut minute = self.minute
            + ((duration.as_secs() / Second.per(Minute) as u64)
                % Minute.per(Hour) as u64) as u8;
        let mut hour = self.hour
            + ((duration.as_secs() / Second.per(Hour) as u64) % Hour.per(Day) as u64)
                as u8;
        let mut is_next_day = false;
        cascade!(nanosecond in 0..Nanosecond.per(Second) => second);
        cascade!(second in 0..Second.per(Minute) => minute);
        cascade!(minute in 0..Minute.per(Hour) => hour);
        if hour >= Hour.per(Day) {
            hour -= Hour.per(Day);
            is_next_day = true;
        }
        (is_next_day, Self::__from_hms_nanos_unchecked(hour, minute, second, nanosecond))
    }
    /// Subtract the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow,
    /// returning whether the date is the previous date as the first element of the tuple.
    pub(crate) const fn adjusting_sub_std(self, duration: StdDuration) -> (bool, Self) {
        let mut nanosecond = self.nanosecond as i32 - duration.subsec_nanos() as i32;
        let mut second = self.second as i8
            - (duration.as_secs() % Second.per(Minute) as u64) as i8;
        let mut minute = self.minute as i8
            - ((duration.as_secs() / Second.per(Minute) as u64)
                % Minute.per(Hour) as u64) as i8;
        let mut hour = self.hour as i8
            - ((duration.as_secs() / Second.per(Hour) as u64) % Hour.per(Day) as u64)
                as i8;
        let mut is_previous_day = false;
        cascade!(nanosecond in 0..Nanosecond.per(Second) as _ => second);
        cascade!(second in 0..Second.per(Minute) as _ => minute);
        cascade!(minute in 0..Minute.per(Hour) as _ => hour);
        if hour < 0 {
            hour += Hour.per(Day) as i8;
            is_previous_day = true;
        }
        (
            is_previous_day,
            Self::__from_hms_nanos_unchecked(
                hour as _,
                minute as _,
                second as _,
                nanosecond as _,
            ),
        )
    }
    /// Replace the clock hour.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_hour(7),
    ///     Ok(time!(07:02:03.004_005_006))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_hour(24).is_err()); // 24 isn't a valid hour
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_hour(self, hour: u8) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(hour in 0 => Hour.per(Day) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                hour,
                self.minute,
                self.second,
                self.nanosecond,
            ),
        )
    }
    /// Replace the minutes within the hour.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_minute(7),
    ///     Ok(time!(01:07:03.004_005_006))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_minute(60).is_err()); // 60 isn't a valid minute
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_minute(
        self,
        minute: u8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(minute in 0 => Minute.per(Hour) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                self.hour,
                minute,
                self.second,
                self.nanosecond,
            ),
        )
    }
    /// Replace the seconds within the minute.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_second(7),
    ///     Ok(time!(01:02:07.004_005_006))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_second(60).is_err()); // 60 isn't a valid second
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_second(
        self,
        second: u8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(second in 0 => Second.per(Minute) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                self.hour,
                self.minute,
                second,
                self.nanosecond,
            ),
        )
    }
    /// Replace the milliseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_millisecond(7),
    ///     Ok(time!(01:02:03.007))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_millisecond(1_000).is_err()); // 1_000 isn't a valid millisecond
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_millisecond(
        self,
        millisecond: u16,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(millisecond in 0 => Millisecond.per(Second) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                self.hour,
                self.minute,
                self.second,
                millisecond as u32 * Nanosecond.per(Millisecond),
            ),
        )
    }
    /// Replace the microseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_microsecond(7_008),
    ///     Ok(time!(01:02:03.007_008))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_microsecond(1_000_000).is_err()); // 1_000_000 isn't a valid microsecond
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_microsecond(
        self,
        microsecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(microsecond in 0 => Microsecond.per(Second) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                self.hour,
                self.minute,
                self.second,
                microsecond * Nanosecond.per(Microsecond) as u32,
            ),
        )
    }
    /// Replace the nanoseconds within the second.
    ///
    /// ```rust
    /// # use time_macros::time;
    /// assert_eq!(
    ///     time!(01:02:03.004_005_006).replace_nanosecond(7_008_009),
    ///     Ok(time!(01:02:03.007_008_009))
    /// );
    /// assert!(time!(01:02:03.004_005_006).replace_nanosecond(1_000_000_000).is_err()); // 1_000_000_000 isn't a valid nanosecond
    /// ```
    #[must_use = "This method does not mutate the original `Time`."]
    pub const fn replace_nanosecond(
        self,
        nanosecond: u32,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(nanosecond in 0 => Nanosecond.per(Second) - 1);
        Ok(
            Self::__from_hms_nanos_unchecked(
                self.hour,
                self.minute,
                self.second,
                nanosecond,
            ),
        )
    }
}
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
    /// # use time::format_description;
    /// # use time_macros::time;
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
    /// # use time::Time;
    /// # use time_macros::{time, format_description};
    /// let format = format_description!("[hour]:[minute]:[second]");
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
        write!(f, "{}:{:02}:{:02}.{value:0width$}", self.hour, self.minute, self.second,)
    }
}
impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl Add<Duration> for Time {
    type Output = Self;
    /// Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
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
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!(12:00) + 2.std_hours(), time!(14:00));
    /// assert_eq!(time!(23:59:59) + 2.std_seconds(), time!(0:00:01));
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        self.adjusting_add_std(duration).1
    }
}
impl_add_assign!(Time : Duration, StdDuration);
impl Sub<Duration> for Time {
    type Output = Self;
    /// Subtract the sub-day time of the [`Duration`] from the `Time`. Wraps on overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
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
    /// # use time::ext::NumericalStdDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!(14:00) - 2.std_hours(), time!(12:00));
    /// assert_eq!(time!(0:00:01) - 2.std_seconds(), time!(23:59:59));
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        self.adjusting_sub_std(duration).1
    }
}
impl_sub_assign!(Time : Duration, StdDuration);
impl Sub for Time {
    type Output = Duration;
    /// Subtract two `Time`s, returning the [`Duration`] between. This assumes both `Time`s are in
    /// the same calendar day.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// # use time_macros::time;
    /// assert_eq!(time!(0:00) - time!(0:00), 0.seconds());
    /// assert_eq!(time!(1:00) - time!(0:00), 1.hours());
    /// assert_eq!(time!(0:00) - time!(1:00), (-1).hours());
    /// assert_eq!(time!(0:00) - time!(23:00), (-23).hours());
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let hour_diff = (self.hour as i8) - (rhs.hour as i8);
        let minute_diff = (self.minute as i8) - (rhs.minute as i8);
        let second_diff = (self.second as i8) - (rhs.second as i8);
        let nanosecond_diff = (self.nanosecond as i32) - (rhs.nanosecond as i32);
        let seconds = hour_diff as i64 * Second.per(Hour) as i64
            + minute_diff as i64 * Second.per(Minute) as i64 + second_diff as i64;
        let (seconds, nanoseconds) = if seconds > 0 && nanosecond_diff < 0 {
            (seconds - 1, nanosecond_diff + Nanosecond.per(Second) as i32)
        } else if seconds < 0 && nanosecond_diff > 0 {
            (seconds + 1, nanosecond_diff - Nanosecond.per(Second) as i32)
        } else {
            (seconds, nanosecond_diff)
        };
        Duration::new_unchecked(seconds, nanoseconds)
    }
}
#[cfg(test)]
mod tests_llm_16_445 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_hms_nanos_unchecked_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.hour, 12);
        debug_assert_eq!(time.minute, 34);
        debug_assert_eq!(time.second, 56);
        debug_assert_eq!(time.nanosecond, 789_456_123);
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_hour_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_minute_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_second_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
             }
});    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_nanos_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_448 {
    use crate::{util::DateAdjustment, Duration, Time};
    #[derive(Debug, PartialEq)]
    enum Adjustment {
        Previous,
        Next,
        None,
    }
    impl From<DateAdjustment> for Adjustment {
        fn from(adj: DateAdjustment) -> Self {
            match adj {
                DateAdjustment::Previous => Self::Previous,
                DateAdjustment::None => Self::None,
                DateAdjustment::Next => Self::Next,
            }
        }
    }
    #[test]
    fn adjusting_sub_no_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::seconds(rug_fuzz_4);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::None);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(11, 59, 59, 0));
             }
});    }
    #[test]
    fn adjusting_sub_with_date_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::seconds(rug_fuzz_4);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::Previous);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(23, 59, 59, 0));
             }
});    }
    #[test]
    fn adjusting_sub_with_overflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::new(rug_fuzz_4, -rug_fuzz_5);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::None);
        debug_assert_eq!(
            new_time, Time::__from_hms_nanos_unchecked(23, 59, 59, 999_999_999)
        );
             }
});    }
    #[test]
    fn adjusting_sub_with_underflow() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::seconds(rug_fuzz_4);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::None);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(23, 58, 59, 500));
             }
});    }
    #[test]
    fn adjusting_sub_with_day_change() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::seconds(rug_fuzz_4);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::Previous);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(23, 59, 59, 0));
             }
});    }
    #[test]
    fn adjusting_sub_with_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::new(rug_fuzz_4, -rug_fuzz_5);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::None);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(12, 34, 57, 0));
             }
});    }
    #[test]
    fn adjusting_sub_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u8, u8, u8, u32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let duration = Duration::new(rug_fuzz_4, rug_fuzz_5);
        let (adjustment, new_time) = time.adjusting_sub(duration);
        debug_assert_eq!(Adjustment::from(adjustment), Adjustment::None);
        debug_assert_eq!(new_time, Time::__from_hms_nanos_unchecked(12, 34, 56, 0));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_451_llm_16_451 {
    use crate::Time;
    use time_macros::time;
    #[test]
    fn test_as_hms_micro() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_micro(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .as_hms_micro(), (0, 0, 0, 0)
        );
        debug_assert_eq!(
            Time::from_hms_micro(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7).unwrap()
            .as_hms_micro(), (12, 34, 56, 0)
        );
        debug_assert_eq!(
            Time::from_hms_micro(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10, rug_fuzz_11)
            .unwrap().as_hms_micro(), (23, 59, 59, 1)
        );
        debug_assert_eq!(
            Time::from_hms_micro(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14, rug_fuzz_15)
            .unwrap().as_hms_micro(), (23, 59, 59, 999_999)
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_452 {
    use crate::{Time, error};
    #[test]
    fn as_hms_milli_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let midnight = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(midnight.as_hms_milli(), (0, 0, 0, 0));
             }
});    }
    #[test]
    fn as_hms_milli_before_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (9, 41, 16, 345));
             }
});    }
    #[test]
    fn as_hms_milli_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let noon = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(noon.as_hms_milli(), (12, 0, 0, 0));
             }
});    }
    #[test]
    fn as_hms_milli_after_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (13, 59, 59, 999));
             }
});    }
    #[test]
    fn as_hms_milli_before_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (23, 59, 59, 999));
             }
});    }
    #[test]
    fn as_hms_milli_invalid_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_ne!(time.as_hms_milli(), (12, 30, 30, 1_000));
             }
});    }
    #[test]
    fn as_hms_milli_edge_case_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (23, 59, 59, 999));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_453 {
    use crate::{Time, error::ComponentRange};
    #[test]
    fn test_as_hms_nano_midnight() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_midnight = 0;
        let time = Time::MIDNIGHT;
        debug_assert_eq!(time.as_hms_nano(), (0, 0, 0, 0));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_midnight = 0;
    }
    #[test]
    fn test_as_hms_nano_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_nano(), (12, 0, 0, 0));
             }
});    }
    #[test]
    fn test_as_hms_nano_max_time() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_max_time = 0;
        let time = Time::MAX;
        debug_assert_eq!(time.as_hms_nano(), (23, 59, 59, 999_999_999));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_max_time = 0;
    }
    #[test]
    fn test_as_hms_nano_random_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_nano(), (13, 29, 31, 123_456_789));
             }
});    }
    #[test]
    fn test_as_hms_nano_error_hour() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
             }
});    }
    #[test]
    fn test_as_hms_nano_error_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
             }
});    }
    #[test]
    fn test_as_hms_nano_error_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
             }
});    }
    #[test]
    fn test_as_hms_nano_error_nanosecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_454 {
    use super::*;
    use crate::*;
    use crate::{Time, error::ComponentRange};
    #[test]
    fn test_from_hms_valid_times() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_ok());
        debug_assert!(Time::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_ok());
        debug_assert!(Time::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).is_ok());
             }
});    }
    #[test]
    fn test_from_hms_invalid_hours() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
             }
});    }
    #[test]
    fn test_from_hms_invalid_minutes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
             }
});    }
    #[test]
    fn test_from_hms_invalid_seconds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;
    use crate::*;
    use crate::Time;
    #[test]
    fn hour_works_correctly() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).hour(), 0
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6,
            rug_fuzz_7).hour(), 1
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).hour(), 12
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).hour(), 23
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_459_llm_16_459 {
    use super::*;
    use crate::*;
    use crate::Time;
    use time_macros::time;
    #[test]
    fn microsecond_at_midnight() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 0
        );
             }
});    }
    #[test]
    fn microsecond_within_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 789_012
        );
             }
});    }
    #[test]
    fn microsecond_at_last_microsecond() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 999_999
        );
             }
});    }
    #[test]
    fn microsecond_just_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 0
        );
             }
});    }
    #[test]
    fn microsecond_just_before_next_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 999_999
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_460 {
    use super::*;
    use crate::*;
    #[test]
    fn millisecond_at_midnight() {
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_at_midnight = 0;
        debug_assert_eq!(Time::MIDNIGHT.millisecond(), 0);
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_at_midnight = 0;
    }
    #[test]
    fn millisecond_at_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).millisecond(), 0
        );
             }
});    }
    #[test]
    fn millisecond_at_specific_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).millisecond(), 999
        );
             }
});    }
    #[test]
    fn millisecond_max_value() {
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_max_value = 0;
        debug_assert_eq!(Time::MAX.millisecond(), 999);
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_max_value = 0;
    }
    #[test]
    fn millisecond_min_value() {
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_min_value = 0;
        debug_assert_eq!(Time::MIN.millisecond(), 0);
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_min_value = 0;
    }
    #[test]
    fn millisecond_with_various_nanos() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).millisecond(), 500
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6,
            rug_fuzz_7).millisecond(), 1
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).millisecond(), 999
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;
    use crate::*;
    #[test]
    fn test_minute() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).minute(), 0
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6,
            rug_fuzz_7).minute(), 59
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).minute(), 34
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).minute(), 15
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_462 {
    use super::*;
    use crate::*;
    use crate::Time;
    #[test]
    fn nanosecond_at_midnight() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_at_midnight = 0;
        debug_assert_eq!(Time::MIDNIGHT.nanosecond(), 0);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_at_midnight = 0;
    }
    #[test]
    fn nanosecond_at_noon() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap().nanosecond(), 0
        );
             }
});    }
    #[test]
    fn nanosecond_at_random_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .nanosecond(), 123_456_789
        );
             }
});    }
    #[test]
    fn nanosecond_at_last_nano_of_day() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_at_last_nano_of_day = 0;
        debug_assert_eq!(Time::MAX.nanosecond(), 999_999_999);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_at_last_nano_of_day = 0;
    }
    #[test]
    #[should_panic]
    fn nanosecond_invalid_time() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _ = Time::__from_hms_nanos_unchecked(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .nanosecond();
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_463 {
    use crate::Time;
    use crate::error::ComponentRange;
    use time_macros::time;
    #[test]
    fn replace_hour_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_time = Time::from_hms_nano(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        let expected_time = Time::from_hms_nano(
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
            )
            .unwrap();
        debug_assert_eq!(initial_time.replace_hour(rug_fuzz_8), Ok(expected_time));
             }
});    }
    #[test]
    fn replace_hour_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let initial_time = Time::from_hms_nano(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .unwrap();
        debug_assert!(
            matches!(initial_time.replace_hour(rug_fuzz_4), Err(ComponentRange { .. }))
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_466 {
    use super::*;
    use crate::*;
    #[test]
    fn test_replace_minute_valid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let result = time.replace_minute(rug_fuzz_4);
        debug_assert_eq!(
            result, Ok(Time::__from_hms_nanos_unchecked(01, 07, 03, 004_005_006))
        );
             }
});    }
    #[test]
    fn test_replace_minute_invalid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u32, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let result = time.replace_minute(rug_fuzz_4);
        debug_assert!(result.is_err());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_469_llm_16_469 {
    use crate::Time;
    #[test]
    fn test_second() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32, u8, u8, u8, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).second(), 0
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6,
            rug_fuzz_7).second(), 59
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_8, rug_fuzz_9, rug_fuzz_10,
            rug_fuzz_11).second(), 15
        );
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_12, rug_fuzz_13, rug_fuzz_14,
            rug_fuzz_15).second(), 45
        );
             }
});    }
}
