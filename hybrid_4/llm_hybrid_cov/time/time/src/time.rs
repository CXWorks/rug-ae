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
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_valid = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789_456_123;
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
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_valid = 0;
    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_hour_overflow() {
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_hour_overflow = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_hour_overflow = 0;
    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_minute_overflow() {
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_minute_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_minute_overflow = 0;
    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_second_overflow() {
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_second_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 0;
        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_second_overflow = 0;
    }
    #[test]
    #[should_panic]
    fn test_from_hms_nanos_unchecked_nanos_overflow() {
        let _rug_st_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_nanos_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1_000_000_000;
        let _ = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let _rug_ed_tests_llm_16_445_rrrruuuugggg_test_from_hms_nanos_unchecked_nanos_overflow = 0;
    }
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
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_no_change = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_no_change = 0;
    }
    #[test]
    fn adjusting_sub_with_date_change() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_date_change = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_date_change = 0;
    }
    #[test]
    fn adjusting_sub_with_overflow() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_overflow = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 500;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 501;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_overflow = 0;
    }
    #[test]
    fn adjusting_sub_with_underflow() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_underflow = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 500;
        let rug_fuzz_4 = 60;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_underflow = 0;
    }
    #[test]
    fn adjusting_sub_with_day_change() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_day_change = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 2;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_day_change = 0;
    }
    #[test]
    fn adjusting_sub_with_nanos() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_nanos = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789_123_456;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 789_123_456;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_nanos = 0;
    }
    #[test]
    fn adjusting_sub_with_zero() {
        let _rug_st_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_zero = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
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
        let _rug_ed_tests_llm_16_448_rrrruuuugggg_adjusting_sub_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_451_llm_16_451 {
    use crate::Time;
    use time_macros::time;
    #[test]
    fn test_as_hms_micro() {
        let _rug_st_tests_llm_16_451_llm_16_451_rrrruuuugggg_test_as_hms_micro = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 12;
        let rug_fuzz_5 = 34;
        let rug_fuzz_6 = 56;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 23;
        let rug_fuzz_9 = 59;
        let rug_fuzz_10 = 59;
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = 23;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 59;
        let rug_fuzz_15 = 999_999;
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
        let _rug_ed_tests_llm_16_451_llm_16_451_rrrruuuugggg_test_as_hms_micro = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_452 {
    use crate::{Time, error};
    #[test]
    fn as_hms_milli_midnight() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_midnight = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let midnight = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(midnight.as_hms_milli(), (0, 0, 0, 0));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_midnight = 0;
    }
    #[test]
    fn as_hms_milli_before_noon() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_before_noon = 0;
        let rug_fuzz_0 = 9;
        let rug_fuzz_1 = 41;
        let rug_fuzz_2 = 16;
        let rug_fuzz_3 = 345_000_000;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (9, 41, 16, 345));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_before_noon = 0;
    }
    #[test]
    fn as_hms_milli_noon() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_noon = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let noon = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(noon.as_hms_milli(), (12, 0, 0, 0));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_noon = 0;
    }
    #[test]
    fn as_hms_milli_after_noon() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_after_noon = 0;
        let rug_fuzz_0 = 13;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_000_000;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (13, 59, 59, 999));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_after_noon = 0;
    }
    #[test]
    fn as_hms_milli_before_midnight() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_before_midnight = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_000_000;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (23, 59, 59, 999));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_before_midnight = 0;
    }
    #[test]
    fn as_hms_milli_invalid_nanos() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_invalid_nanos = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 30;
        let rug_fuzz_3 = 1_000_000_001;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_ne!(time.as_hms_milli(), (12, 30, 30, 1_000));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_invalid_nanos = 0;
    }
    #[test]
    fn as_hms_milli_edge_case_nanos() {
        let _rug_st_tests_llm_16_452_rrrruuuugggg_as_hms_milli_edge_case_nanos = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_999_999;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_milli(), (23, 59, 59, 999));
        let _rug_ed_tests_llm_16_452_rrrruuuugggg_as_hms_milli_edge_case_nanos = 0;
    }
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
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_noon = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_nano(), (12, 0, 0, 0));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_noon = 0;
    }
    #[test]
    fn test_as_hms_nano_max_time() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_max_time = 0;
        let time = Time::MAX;
        debug_assert_eq!(time.as_hms_nano(), (23, 59, 59, 999_999_999));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_max_time = 0;
    }
    #[test]
    fn test_as_hms_nano_random_time() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_random_time = 0;
        let rug_fuzz_0 = 13;
        let rug_fuzz_1 = 29;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 123_456_789;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        debug_assert_eq!(time.as_hms_nano(), (13, 29, 31, 123_456_789));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_random_time = 0;
    }
    #[test]
    fn test_as_hms_nano_error_hour() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_hour = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_hour = 0;
    }
    #[test]
    fn test_as_hms_nano_error_minute() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_minute = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_minute = 0;
    }
    #[test]
    fn test_as_hms_nano_error_second() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 60;
        let rug_fuzz_3 = 0;
        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_second = 0;
    }
    #[test]
    fn test_as_hms_nano_error_nanosecond() {
        let _rug_st_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_nanosecond = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1_000_000_000;
        let time = Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3);
        debug_assert!(time.is_err());
        debug_assert!(matches!(time, Err(ComponentRange)));
        let _rug_ed_tests_llm_16_453_rrrruuuugggg_test_as_hms_nano_error_nanosecond = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_454 {
    use super::*;
    use crate::*;
    use crate::{Time, error::ComponentRange};
    #[test]
    fn test_from_hms_valid_times() {
        let _rug_st_tests_llm_16_454_rrrruuuugggg_test_from_hms_valid_times = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 23;
        let rug_fuzz_4 = 59;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 12;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 45;
        debug_assert!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).is_ok());
        debug_assert!(Time::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).is_ok());
        debug_assert!(Time::from_hms(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8).is_ok());
        let _rug_ed_tests_llm_16_454_rrrruuuugggg_test_from_hms_valid_times = 0;
    }
    #[test]
    fn test_from_hms_invalid_hours() {
        let _rug_st_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_hours = 0;
        let rug_fuzz_0 = 24;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        let _rug_ed_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_hours = 0;
    }
    #[test]
    fn test_from_hms_invalid_minutes() {
        let _rug_st_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_minutes = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 60;
        let rug_fuzz_2 = 0;
        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        let _rug_ed_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_minutes = 0;
    }
    #[test]
    fn test_from_hms_invalid_seconds() {
        let _rug_st_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_seconds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 60;
        debug_assert!(
            matches!(Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2),
            Err(ComponentRange))
        );
        let _rug_ed_tests_llm_16_454_rrrruuuugggg_test_from_hms_invalid_seconds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;
    use crate::*;
    use crate::Time;
    #[test]
    fn hour_works_correctly() {
        let _rug_st_tests_llm_16_458_rrrruuuugggg_hour_works_correctly = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 12;
        let rug_fuzz_9 = 30;
        let rug_fuzz_10 = 30;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 23;
        let rug_fuzz_13 = 59;
        let rug_fuzz_14 = 59;
        let rug_fuzz_15 = 0;
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
        let _rug_ed_tests_llm_16_458_rrrruuuugggg_hour_works_correctly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_459_llm_16_459 {
    use super::*;
    use crate::*;
    use crate::Time;
    use time_macros::time;
    #[test]
    fn microsecond_at_midnight() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_at_midnight = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 0
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_at_midnight = 0;
    }
    #[test]
    fn microsecond_within_second() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_within_second = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 789_012_000;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 789_012
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_within_second = 0;
    }
    #[test]
    fn microsecond_at_last_microsecond() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_at_last_microsecond = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_999_000;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 999_999
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_at_last_microsecond = 0;
    }
    #[test]
    fn microsecond_just_nanos() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_just_nanos = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 0
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_just_nanos = 0;
    }
    #[test]
    fn microsecond_just_before_next_second() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_just_before_next_second = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_999_999;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .microsecond(), 999_999
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_microsecond_just_before_next_second = 0;
    }
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
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_at_noon = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).millisecond(), 0
        );
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_at_noon = 0;
    }
    #[test]
    fn millisecond_at_specific_time() {
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_at_specific_time = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_000_000;
        debug_assert_eq!(
            Time::__from_hms_nanos_unchecked(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2,
            rug_fuzz_3).millisecond(), 999
        );
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_at_specific_time = 0;
    }
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
        let _rug_st_tests_llm_16_460_rrrruuuugggg_millisecond_with_various_nanos = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 500_000_000;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 1_000_000;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 999_999_999;
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
        let _rug_ed_tests_llm_16_460_rrrruuuugggg_millisecond_with_various_nanos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_461 {
    use super::*;
    use crate::*;
    #[test]
    fn test_minute() {
        let _rug_st_tests_llm_16_461_rrrruuuugggg_test_minute = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 999_999_999;
        let rug_fuzz_8 = 12;
        let rug_fuzz_9 = 34;
        let rug_fuzz_10 = 56;
        let rug_fuzz_11 = 789_012_345;
        let rug_fuzz_12 = 6;
        let rug_fuzz_13 = 15;
        let rug_fuzz_14 = 30;
        let rug_fuzz_15 = 123_456_789;
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
        let _rug_ed_tests_llm_16_461_rrrruuuugggg_test_minute = 0;
    }
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
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_at_noon = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        debug_assert_eq!(
            Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap().nanosecond(), 0
        );
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_at_noon = 0;
    }
    #[test]
    fn nanosecond_at_random_time() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_at_random_time = 0;
        let rug_fuzz_0 = 13;
        let rug_fuzz_1 = 46;
        let rug_fuzz_2 = 28;
        let rug_fuzz_3 = 123_456_789;
        debug_assert_eq!(
            Time::from_hms_nano(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3).unwrap()
            .nanosecond(), 123_456_789
        );
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_at_random_time = 0;
    }
    #[test]
    fn nanosecond_at_last_nano_of_day() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_at_last_nano_of_day = 0;
        debug_assert_eq!(Time::MAX.nanosecond(), 999_999_999);
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_at_last_nano_of_day = 0;
    }
    #[test]
    #[should_panic]
    fn nanosecond_invalid_time() {
        let _rug_st_tests_llm_16_462_rrrruuuugggg_nanosecond_invalid_time = 0;
        let rug_fuzz_0 = 25;
        let rug_fuzz_1 = 61;
        let rug_fuzz_2 = 61;
        let rug_fuzz_3 = 1_000_000_000;
        let _ = Time::__from_hms_nanos_unchecked(
                rug_fuzz_0,
                rug_fuzz_1,
                rug_fuzz_2,
                rug_fuzz_3,
            )
            .nanosecond();
        let _rug_ed_tests_llm_16_462_rrrruuuugggg_nanosecond_invalid_time = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_463 {
    use crate::Time;
    use crate::error::ComponentRange;
    use time_macros::time;
    #[test]
    fn replace_hour_valid() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_replace_hour_valid = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4_005_006;
        let rug_fuzz_4 = 7;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 3;
        let rug_fuzz_7 = 4_005_006;
        let rug_fuzz_8 = 7;
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
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_replace_hour_valid = 0;
    }
    #[test]
    fn replace_hour_invalid() {
        let _rug_st_tests_llm_16_463_rrrruuuugggg_replace_hour_invalid = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4_005_006;
        let rug_fuzz_4 = 24;
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
        let _rug_ed_tests_llm_16_463_rrrruuuugggg_replace_hour_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_466 {
    use super::*;
    use crate::*;
    #[test]
    fn test_replace_minute_valid() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_replace_minute_valid = 0;
        let rug_fuzz_0 = 01;
        let rug_fuzz_1 = 02;
        let rug_fuzz_2 = 03;
        let rug_fuzz_3 = 004_005_006;
        let rug_fuzz_4 = 7;
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
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_replace_minute_valid = 0;
    }
    #[test]
    fn test_replace_minute_invalid() {
        let _rug_st_tests_llm_16_466_rrrruuuugggg_test_replace_minute_invalid = 0;
        let rug_fuzz_0 = 01;
        let rug_fuzz_1 = 02;
        let rug_fuzz_2 = 03;
        let rug_fuzz_3 = 004_005_006;
        let rug_fuzz_4 = 60;
        let time = Time::__from_hms_nanos_unchecked(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
        );
        let result = time.replace_minute(rug_fuzz_4);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_466_rrrruuuugggg_test_replace_minute_invalid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_469_llm_16_469 {
    use crate::Time;
    #[test]
    fn test_second() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_second = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 23;
        let rug_fuzz_5 = 59;
        let rug_fuzz_6 = 59;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 12;
        let rug_fuzz_9 = 30;
        let rug_fuzz_10 = 15;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 1;
        let rug_fuzz_13 = 23;
        let rug_fuzz_14 = 45;
        let rug_fuzz_15 = 0;
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
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_test_second = 0;
    }
}
#[cfg(test)]
mod tests_rug_303 {
    use super::*;
    #[test]
    fn test_from_hms_milli() {
        let _rug_st_tests_rug_303_rrrruuuugggg_test_from_hms_milli = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999;
        let mut p0: u8 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        let mut p2: u8 = rug_fuzz_2;
        let mut p3: u16 = rug_fuzz_3;
        debug_assert!(Time::from_hms_milli(p0, p1, p2, p3).is_ok());
        let _rug_ed_tests_rug_303_rrrruuuugggg_test_from_hms_milli = 0;
    }
}
#[cfg(test)]
mod tests_rug_304 {
    use super::*;
    #[test]
    fn test_from_hms_micro() {
        let _rug_st_tests_rug_304_rrrruuuugggg_test_from_hms_micro = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 30;
        let rug_fuzz_2 = 45;
        let rug_fuzz_3 = 123456;
        let mut p0: u8 = rug_fuzz_0;
        let mut p1: u8 = rug_fuzz_1;
        let mut p2: u8 = rug_fuzz_2;
        let mut p3: u32 = rug_fuzz_3;
        debug_assert_eq!(
            Time::from_hms_micro(p0, p1, p2, p3), Ok(Time::__from_hms_nanos_unchecked(p0,
            p1, p2, p3 * 1_000))
        );
        let _rug_ed_tests_rug_304_rrrruuuugggg_test_from_hms_micro = 0;
    }
}
#[cfg(test)]
mod tests_rug_305 {
    use super::*;
    #[test]
    fn test_from_hms_nano() {
        let _rug_st_tests_rug_305_rrrruuuugggg_test_from_hms_nano = 0;
        let rug_fuzz_0 = 23;
        let rug_fuzz_1 = 59;
        let rug_fuzz_2 = 59;
        let rug_fuzz_3 = 999_999_999;
        let p0: u8 = rug_fuzz_0;
        let p1: u8 = rug_fuzz_1;
        let p2: u8 = rug_fuzz_2;
        let p3: u32 = rug_fuzz_3;
        debug_assert!(Time::from_hms_nano(p0, p1, p2, p3).is_ok());
        let _rug_ed_tests_rug_305_rrrruuuugggg_test_from_hms_nano = 0;
    }
}
#[cfg(test)]
mod tests_rug_306 {
    use crate::Time;
    #[test]
    fn test_as_hms() {
        let _rug_st_tests_rug_306_rrrruuuugggg_test_as_hms = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        debug_assert_eq!(Time::as_hms(p0), (12, 34, 56));
        let _rug_ed_tests_rug_306_rrrruuuugggg_test_as_hms = 0;
    }
}
#[cfg(test)]
mod tests_rug_307 {
    use super::*;
    use crate::{Duration, Time};
    #[test]
    fn test_adjusting_add() {
        let _rug_st_tests_rug_307_rrrruuuugggg_test_adjusting_add = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1 = Duration::minutes(rug_fuzz_3);
        <Time>::adjusting_add(p0, p1);
        let _rug_ed_tests_rug_307_rrrruuuugggg_test_adjusting_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_308 {
    use super::*;
    use std::time::Duration as StdDuration;
    use crate::Time;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_308_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1 = StdDuration::new(rug_fuzz_3, rug_fuzz_4);
        Time::adjusting_add_std(p0, p1);
        let _rug_ed_tests_rug_308_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_309 {
    use super::*;
    use std::time::Duration as StdDuration;
    use crate::Time;
    #[test]
    fn test_adjusting_sub_std() {
        let _rug_st_tests_rug_309_rrrruuuugggg_test_adjusting_sub_std = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1 = StdDuration::new(rug_fuzz_3, rug_fuzz_4);
        let _result = Time::adjusting_sub_std(p0, p1);
        let _rug_ed_tests_rug_309_rrrruuuugggg_test_adjusting_sub_std = 0;
    }
}
#[cfg(test)]
mod tests_rug_310 {
    use super::*;
    use crate::Time;
    #[test]
    fn test_replace_second() {
        let _rug_st_tests_rug_310_rrrruuuugggg_test_replace_second = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 45;
        let rug_fuzz_4 = 60;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: u8 = rug_fuzz_3;
        debug_assert_eq!(
            < Time > ::replace_second(p0, p1).unwrap(), Time::from_hms(12, 34, p1)
            .unwrap()
        );
        let mutated_p1: u8 = rug_fuzz_4;
        debug_assert!(< Time > ::replace_second(p0, mutated_p1).is_err());
        let _rug_ed_tests_rug_310_rrrruuuugggg_test_replace_second = 0;
    }
}
#[cfg(test)]
mod tests_rug_311 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_311_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 123;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: u16 = rug_fuzz_3;
        let _ = <Time>::replace_millisecond(p0, p1);
        let _rug_ed_tests_rug_311_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_312 {
    use super::*;
    use crate::Time;
    #[test]
    fn test_replace_microsecond() {
        let _rug_st_tests_rug_312_rrrruuuugggg_test_replace_microsecond = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 123_456;
        let rug_fuzz_4 = 1_000_000;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: u32 = rug_fuzz_3;
        debug_assert_eq!(
            p0.replace_microsecond(p1), Ok(Time::from_hms_micro(12, 34, 56, p1).unwrap())
        );
        let mut p1_invalid: u32 = rug_fuzz_4;
        debug_assert!(p0.replace_microsecond(p1_invalid).is_err());
        let _rug_ed_tests_rug_312_rrrruuuugggg_test_replace_microsecond = 0;
    }
}
#[cfg(test)]
mod tests_rug_313 {
    use super::*;
    use crate::Time;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_313_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 123_456_789;
        let rug_fuzz_4 = 1_000_000_000;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: u32 = rug_fuzz_3;
        debug_assert_eq!(
            p0.replace_nanosecond(p1), Ok(Time::__from_hms_nanos_unchecked(12, 34, 56,
            p1))
        );
        debug_assert!(p0.replace_nanosecond(rug_fuzz_4).is_err());
        let _rug_ed_tests_rug_313_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_314 {
    use super::*;
    use std::ops::Add;
    use crate::{Duration, Time};
    #[test]
    fn test_add() {
        let _rug_st_tests_rug_314_rrrruuuugggg_test_add = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let mut p0: Time = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: Duration = Duration::minutes(rug_fuzz_3);
        <Time as Add<Duration>>::add(p0, p1);
        let _rug_ed_tests_rug_314_rrrruuuugggg_test_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_315 {
    use super::*;
    use std::time::Duration;
    #[test]
    fn test_add() {
        let _rug_st_tests_rug_315_rrrruuuugggg_test_add = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1 = Duration::new(rug_fuzz_3, rug_fuzz_4);
        <Time as std::ops::Add<Duration>>::add(p0, p1);
        let _rug_ed_tests_rug_315_rrrruuuugggg_test_add = 0;
    }
}
#[cfg(test)]
mod tests_rug_316 {
    use super::*;
    use std::ops::Sub;
    use crate::{Duration, Time};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_316_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let mut p0: Time = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: Duration = Duration::minutes(rug_fuzz_3);
        <Time as Sub<Duration>>::sub(p0, p1);
        let _rug_ed_tests_rug_316_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_317 {
    use super::*;
    use std::ops::Sub;
    use std::time::Duration;
    use crate::Time;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_317_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 0;
        let mut p0: Time = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1: Duration = Duration::new(rug_fuzz_3, rug_fuzz_4);
        <Time as Sub<Duration>>::sub(p0, p1);
        let _rug_ed_tests_rug_317_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_318 {
    use super::*;
    use crate::ext::NumericalDuration;
    use crate::Duration;
    use time_macros::time;
    use std::ops::Sub;
    #[test]
    fn test_sub() {
        let _rug_st_tests_rug_318_rrrruuuugggg_test_sub = 0;
        let rug_fuzz_0 = 12;
        let rug_fuzz_1 = 34;
        let rug_fuzz_2 = 56;
        let rug_fuzz_3 = 12;
        let rug_fuzz_4 = 34;
        let rug_fuzz_5 = 56;
        let mut p0 = Time::from_hms(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let mut p1 = Time::from_hms(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5).unwrap();
        p0.sub(p1);
        let _rug_ed_tests_rug_318_rrrruuuugggg_test_sub = 0;
    }
}
