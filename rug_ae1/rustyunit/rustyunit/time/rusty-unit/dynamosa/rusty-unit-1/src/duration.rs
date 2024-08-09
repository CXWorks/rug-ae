//! The [`Duration`] struct and its associated `impl`s.

use core::cmp::Ordering;
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, Div, Mul, Neg, Sub, SubAssign};
use core::time::Duration as StdDuration;

use crate::error;
#[cfg(feature = "std")]
use crate::Instant;

/// By explicitly inserting this enum where padding is expected, the compiler is able to better
/// perform niche value optimization.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Padding {
    #[allow(clippy::missing_docs_in_private_items)]
    Optimize,
}

impl Default for Padding {
    fn default() -> Self {
        Self::Optimize
    }
}

/// A span of time with nanosecond precision.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional part represented in
/// nanoseconds.
///
/// This implementation allows for negative durations, unlike [`core::time::Duration`].
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    /// Number of whole seconds.
    seconds: i64,
    /// Number of nanoseconds within the second. The sign always matches the `seconds` field.
    nanoseconds: i32, // always -10^9 < nanoseconds < 10^9
    #[allow(clippy::missing_docs_in_private_items)]
    padding: Padding,
}

impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Duration")
            .field("seconds", &self.seconds)
            .field("nanoseconds", &self.nanoseconds)
            .finish()
    }
}

impl Duration {
    // region: constants
    /// Equivalent to `0.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::ZERO, 0.seconds());
    /// ```
    pub const ZERO: Self = Self::seconds(0);

    /// Equivalent to `1.nanoseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::NANOSECOND, 1.nanoseconds());
    /// ```
    pub const NANOSECOND: Self = Self::nanoseconds(1);

    /// Equivalent to `1.microseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MICROSECOND, 1.microseconds());
    /// ```
    pub const MICROSECOND: Self = Self::microseconds(1);

    /// Equivalent to `1.milliseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MILLISECOND, 1.milliseconds());
    /// ```
    pub const MILLISECOND: Self = Self::milliseconds(1);

    /// Equivalent to `1.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::SECOND, 1.seconds());
    /// ```
    pub const SECOND: Self = Self::seconds(1);

    /// Equivalent to `1.minutes()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::MINUTE, 1.minutes());
    /// ```
    pub const MINUTE: Self = Self::minutes(1);

    /// Equivalent to `1.hours()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::HOUR, 1.hours());
    /// ```
    pub const HOUR: Self = Self::hours(1);

    /// Equivalent to `1.days()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::DAY, 1.days());
    /// ```
    pub const DAY: Self = Self::days(1);

    /// Equivalent to `1.weeks()`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::WEEK, 1.weeks());
    /// ```
    pub const WEEK: Self = Self::weeks(1);

    /// The minimum possible duration. Adding any negative duration to this will cause an overflow.
    pub const MIN: Self = Self::new_unchecked(i64::MIN, -999_999_999);

    /// The maximum possible duration. Adding any positive duration to this will cause an overflow.
    pub const MAX: Self = Self::new_unchecked(i64::MAX, 999_999_999);
    // endregion constants

    // region: is_{sign}
    /// Check if a duration is exactly zero.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!(0.seconds().is_zero());
    /// assert!(!1.nanoseconds().is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        self.seconds == 0 && self.nanoseconds == 0
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!((-1).seconds().is_negative());
    /// assert!(!0.seconds().is_negative());
    /// assert!(!1.seconds().is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.seconds < 0 || self.nanoseconds < 0
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert!(1.seconds().is_positive());
    /// assert!(!0.seconds().is_positive());
    /// assert!(!(-1).seconds().is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.seconds > 0 || self.nanoseconds > 0
    }
    // endregion is_{sign}

    // region: abs
    /// Get the absolute value of the duration.
    ///
    /// This method saturates the returned value if it would otherwise overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().abs(), 1.seconds());
    /// assert_eq!(0.seconds().abs(), 0.seconds());
    /// assert_eq!((-1).seconds().abs(), 1.seconds());
    /// ```
    pub const fn abs(self) -> Self {
        Self::new_unchecked(self.seconds.saturating_abs(), self.nanoseconds.abs())
    }

    /// Convert the existing `Duration` to a `std::time::Duration` and its sign. This doesn't
    /// actually require the standard library, but is currently only used when it's enabled.
    #[allow(clippy::missing_const_for_fn)] // false positive
    #[cfg(feature = "std")]
    pub(crate) fn abs_std(self) -> StdDuration {
        StdDuration::new(self.seconds.unsigned_abs(), self.nanoseconds.unsigned_abs())
    }
    // endregion abs

    // region: constructors
    /// Create a new `Duration` without checking the validity of the components.
    pub(crate) const fn new_unchecked(seconds: i64, nanoseconds: i32) -> Self {
        Self {
            seconds,
            nanoseconds,
            padding: Padding::Optimize,
        }
    }

    /// Create a new `Duration` with the provided seconds and nanoseconds. If nanoseconds is at
    /// least ±10<sup>9</sup>, it will wrap to the number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::new(1, 0), 1.seconds());
    /// assert_eq!(Duration::new(-1, 0), (-1).seconds());
    /// assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());
    /// ```
    pub const fn new(mut seconds: i64, mut nanoseconds: i32) -> Self {
        seconds += nanoseconds as i64 / 1_000_000_000;
        nanoseconds %= 1_000_000_000;

        if seconds > 0 && nanoseconds < 0 {
            seconds -= 1;
            nanoseconds += 1_000_000_000;
        } else if seconds < 0 && nanoseconds > 0 {
            seconds += 1;
            nanoseconds -= 1_000_000_000;
        }

        Self::new_unchecked(seconds, nanoseconds)
    }

    /// Create a new `Duration` with the given number of weeks. Equivalent to
    /// `Duration::seconds(weeks * 604_800)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::weeks(1), 604_800.seconds());
    /// ```
    pub const fn weeks(weeks: i64) -> Self {
        Self::seconds(weeks * 604_800)
    }

    /// Create a new `Duration` with the given number of days. Equivalent to
    /// `Duration::seconds(days * 86_400)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::days(1), 86_400.seconds());
    /// ```
    pub const fn days(days: i64) -> Self {
        Self::seconds(days * 86_400)
    }

    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::hours(1), 3_600.seconds());
    /// ```
    pub const fn hours(hours: i64) -> Self {
        Self::seconds(hours * 3_600)
    }

    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60)`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::minutes(1), 60.seconds());
    /// ```
    pub const fn minutes(minutes: i64) -> Self {
        Self::seconds(minutes * 60)
    }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds(1), 1_000.milliseconds());
    /// ```
    pub const fn seconds(seconds: i64) -> Self {
        Self::new_unchecked(seconds, 0)
    }

    /// Creates a new `Duration` from the specified number of seconds represented as `f64`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f64(-0.5), -0.5.seconds());
    /// ```
    pub fn seconds_f64(seconds: f64) -> Self {
        Self::new_unchecked(seconds as _, ((seconds % 1.) * 1_000_000_000.) as _)
    }

    /// Creates a new `Duration` from the specified number of seconds represented as `f32`.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
    /// ```
    pub fn seconds_f32(seconds: f32) -> Self {
        Self::new_unchecked(seconds as _, ((seconds % 1.) * 1_000_000_000.) as _)
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
    /// assert_eq!(Duration::milliseconds(-1), (-1_000).microseconds());
    /// ```
    pub const fn milliseconds(milliseconds: i64) -> Self {
        Self::new_unchecked(
            milliseconds / 1_000,
            ((milliseconds % 1_000) * 1_000_000) as _,
        )
    }

    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
    /// assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
    /// ```
    pub const fn microseconds(microseconds: i64) -> Self {
        Self::new_unchecked(
            microseconds / 1_000_000,
            ((microseconds % 1_000_000) * 1_000) as _,
        )
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
    /// assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
    /// ```
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        Self::new_unchecked(
            nanoseconds / 1_000_000_000,
            (nanoseconds % 1_000_000_000) as _,
        )
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// As the input range cannot be fully mapped to the output, this should only be used where it's
    /// known to result in a valid value.
    pub(crate) const fn nanoseconds_i128(nanoseconds: i128) -> Self {
        Self::new_unchecked(
            (nanoseconds / 1_000_000_000) as _,
            (nanoseconds % 1_000_000_000) as _,
        )
    }
    // endregion constructors

    // region: getters
    /// Get the number of whole weeks in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.weeks().whole_weeks(), 1);
    /// assert_eq!((-1).weeks().whole_weeks(), -1);
    /// assert_eq!(6.days().whole_weeks(), 0);
    /// assert_eq!((-6).days().whole_weeks(), 0);
    /// ```
    pub const fn whole_weeks(self) -> i64 {
        self.whole_seconds() / 604_800
    }

    /// Get the number of whole days in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.days().whole_days(), 1);
    /// assert_eq!((-1).days().whole_days(), -1);
    /// assert_eq!(23.hours().whole_days(), 0);
    /// assert_eq!((-23).hours().whole_days(), 0);
    /// ```
    pub const fn whole_days(self) -> i64 {
        self.whole_seconds() / 86_400
    }

    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.hours().whole_hours(), 1);
    /// assert_eq!((-1).hours().whole_hours(), -1);
    /// assert_eq!(59.minutes().whole_hours(), 0);
    /// assert_eq!((-59).minutes().whole_hours(), 0);
    /// ```
    pub const fn whole_hours(self) -> i64 {
        self.whole_seconds() / 3_600
    }

    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.minutes().whole_minutes(), 1);
    /// assert_eq!((-1).minutes().whole_minutes(), -1);
    /// assert_eq!(59.seconds().whole_minutes(), 0);
    /// assert_eq!((-59).seconds().whole_minutes(), 0);
    /// ```
    pub const fn whole_minutes(self) -> i64 {
        self.whole_seconds() / 60
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().whole_seconds(), 1);
    /// assert_eq!((-1).seconds().whole_seconds(), -1);
    /// assert_eq!(1.minutes().whole_seconds(), 60);
    /// assert_eq!((-1).minutes().whole_seconds(), -60);
    /// ```
    pub const fn whole_seconds(self) -> i64 {
        self.seconds
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
    /// ```
    pub fn as_seconds_f64(self) -> f64 {
        self.seconds as f64 + self.nanoseconds as f64 / 1_000_000_000.
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
    /// ```
    pub fn as_seconds_f32(self) -> f32 {
        self.seconds as f32 + self.nanoseconds as f32 / 1_000_000_000.
    }

    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.seconds().whole_milliseconds(), 1_000);
    /// assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
    /// assert_eq!(1.milliseconds().whole_milliseconds(), 1);
    /// assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
    /// ```
    pub const fn whole_milliseconds(self) -> i128 {
        self.seconds as i128 * 1_000 + self.nanoseconds as i128 / 1_000_000
    }

    /// Get the number of milliseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000..1_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
    /// assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
    /// ```
    // Allow the lint, as the value is guaranteed to be less than 1000.
    pub const fn subsec_milliseconds(self) -> i16 {
        (self.nanoseconds / 1_000_000) as _
    }

    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
    /// assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
    /// assert_eq!(1.microseconds().whole_microseconds(), 1);
    /// assert_eq!((-1).microseconds().whole_microseconds(), -1);
    /// ```
    pub const fn whole_microseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000 + self.nanoseconds as i128 / 1_000
    }

    /// Get the number of microseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000_000..1_000_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
    /// assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
    /// ```
    pub const fn subsec_microseconds(self) -> i32 {
        self.nanoseconds / 1_000
    }

    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
    /// assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
    /// assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
    /// assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
    /// ```
    pub const fn whole_nanoseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000_000 + self.nanoseconds as i128
    }

    /// Get the number of nanoseconds past the number of whole seconds.
    ///
    /// The returned value will always be in the range `-1_000_000_000..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(1.000_000_400.seconds().subsec_nanoseconds(), 400);
    /// assert_eq!((-1.000_000_400).seconds().subsec_nanoseconds(), -400);
    /// ```
    pub const fn subsec_nanoseconds(self) -> i32 {
        self.nanoseconds
    }
    // endregion getters

    // region: checked arithmetic
    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::MAX.checked_add(1.nanoseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let mut seconds = const_try_opt!(self.seconds.checked_add(rhs.seconds));
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = const_try_opt!(seconds.checked_add(1));
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = const_try_opt!(seconds.checked_sub(1));
        }

        Some(Self::new_unchecked(seconds, nanoseconds))
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::ZERO));
    /// assert_eq!(Duration::MIN.checked_sub(1.nanoseconds()), None);
    /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    /// ```
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let mut seconds = const_try_opt!(self.seconds.checked_sub(rhs.seconds));
        let mut nanoseconds = self.nanoseconds - rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = const_try_opt!(seconds.checked_add(1));
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = const_try_opt!(seconds.checked_sub(1));
        }

        Some(Self::new_unchecked(seconds, nanoseconds))
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    /// assert_eq!(Duration::MAX.checked_mul(2), None);
    /// assert_eq!(Duration::MIN.checked_mul(2), None);
    /// ```
    pub const fn checked_mul(self, rhs: i32) -> Option<Self> {
        // Multiply nanoseconds as i64, because it cannot overflow that way.
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / 1_000_000_000;
        let nanoseconds = (total_nanos % 1_000_000_000) as _;
        let seconds = const_try_opt!(
            const_try_opt!(self.seconds.checked_mul(rhs as _)).checked_add(extra_secs)
        );

        Some(Self::new_unchecked(seconds, nanoseconds))
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0` or if the result would overflow.
    ///
    /// ```rust
    /// # use time::ext::NumericalDuration;
    /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    /// assert_eq!(1.seconds().checked_div(0), None);
    /// ```
    pub const fn checked_div(self, rhs: i32) -> Option<Self> {
        let seconds = const_try_opt!(self.seconds.checked_div(rhs as i64));
        let carry = self.seconds - seconds * (rhs as i64);
        let extra_nanos = const_try_opt!((carry * 1_000_000_000).checked_div(rhs as i64));
        let nanoseconds = const_try_opt!(self.nanoseconds.checked_div(rhs)) + (extra_nanos as i32);

        Some(Self::new_unchecked(seconds, nanoseconds))
    }
    // endregion checked arithmetic

    // region: saturating arithmetic
    /// Computes `self + rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_add(5.seconds()), 10.seconds());
    /// assert_eq!(Duration::MAX.saturating_add(1.nanoseconds()), Duration::MAX);
    /// assert_eq!(
    ///     Duration::MIN.saturating_add((-1).nanoseconds()),
    ///     Duration::MIN
    /// );
    /// assert_eq!((-5).seconds().saturating_add(5.seconds()), Duration::ZERO);
    /// ```
    pub const fn saturating_add(self, rhs: Self) -> Self {
        let (mut seconds, overflow) = self.seconds.overflowing_add(rhs.seconds);
        if overflow {
            if self.seconds > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = match seconds.checked_add(1) {
                Some(seconds) => seconds,
                None => return Self::MAX,
            };
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = match seconds.checked_sub(1) {
                Some(seconds) => seconds,
                None => return Self::MIN,
            };
        }

        Self::new_unchecked(seconds, nanoseconds)
    }

    /// Computes `self - rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_sub(5.seconds()), Duration::ZERO);
    /// assert_eq!(Duration::MIN.saturating_sub(1.nanoseconds()), Duration::MIN);
    /// assert_eq!(
    ///     Duration::MAX.saturating_sub((-1).nanoseconds()),
    ///     Duration::MAX
    /// );
    /// assert_eq!(5.seconds().saturating_sub(10.seconds()), (-5).seconds());
    /// ```
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        let (mut seconds, overflow) = self.seconds.overflowing_sub(rhs.seconds);
        if overflow {
            if self.seconds > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let mut nanoseconds = self.nanoseconds - rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = match seconds.checked_add(1) {
                Some(seconds) => seconds,
                None => return Self::MAX,
            };
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = match seconds.checked_sub(1) {
                Some(seconds) => seconds,
                None => return Self::MIN,
            };
        }

        Self::new_unchecked(seconds, nanoseconds)
    }

    /// Computes `self * rhs`, saturating if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, ext::NumericalDuration};
    /// assert_eq!(5.seconds().saturating_mul(2), 10.seconds());
    /// assert_eq!(5.seconds().saturating_mul(-2), (-10).seconds());
    /// assert_eq!(5.seconds().saturating_mul(0), Duration::ZERO);
    /// assert_eq!(Duration::MAX.saturating_mul(2), Duration::MAX);
    /// assert_eq!(Duration::MIN.saturating_mul(2), Duration::MIN);
    /// assert_eq!(Duration::MAX.saturating_mul(-2), Duration::MIN);
    /// assert_eq!(Duration::MIN.saturating_mul(-2), Duration::MAX);
    /// ```
    pub const fn saturating_mul(self, rhs: i32) -> Self {
        // Multiply nanoseconds as i64, because it cannot overflow that way.
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / 1_000_000_000;
        let nanoseconds = (total_nanos % 1_000_000_000) as _;
        let (seconds, overflow1) = self.seconds.overflowing_mul(rhs as _);
        if overflow1 {
            if self.seconds > 0 && rhs > 0 || self.seconds < 0 && rhs < 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }
        let (seconds, overflow2) = seconds.overflowing_add(extra_secs);
        if overflow2 {
            if self.seconds > 0 && rhs > 0 {
                return Self::MAX;
            }
            return Self::MIN;
        }

        Self::new_unchecked(seconds, nanoseconds)
    }
    // endregion saturating arithmetic

    /// Runs a closure, returning the duration of time it took to run. The return value of the
    /// closure is provided in the second part of the tuple.
    #[cfg(feature = "std")]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (Self, T) {
        let start = Instant::now();
        let return_value = f();
        let end = Instant::now();

        (end - start, return_value)
    }
}

// region: trait impls
impl TryFrom<StdDuration> for Duration {
    type Error = error::ConversionRange;

    fn try_from(original: StdDuration) -> Result<Self, error::ConversionRange> {
        Ok(Self::new(
            original
                .as_secs()
                .try_into()
                .map_err(|_| error::ConversionRange)?,
            original.subsec_nanos() as _,
        ))
    }
}

impl TryFrom<Duration> for StdDuration {
    type Error = error::ConversionRange;

    fn try_from(duration: Duration) -> Result<Self, error::ConversionRange> {
        Ok(Self::new(
            duration
                .seconds
                .try_into()
                .map_err(|_| error::ConversionRange)?,
            duration
                .nanoseconds
                .try_into()
                .map_err(|_| error::ConversionRange)?,
        ))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl Add<StdDuration> for Duration {
    type Output = Self;

    fn add(self, std_duration: StdDuration) -> Self::Output {
        self + Self::try_from(std_duration)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Add<Duration> for StdDuration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        rhs + self
    }
}

impl_add_assign!(Duration: Duration, StdDuration);

impl Neg for Duration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_unchecked(-self.seconds, -self.nanoseconds)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl Sub<StdDuration> for Duration {
    type Output = Self;

    fn sub(self, rhs: StdDuration) -> Self::Output {
        self - Self::try_from(rhs)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Sub<Duration> for StdDuration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration::try_from(self)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
            - rhs
    }
}

impl_sub_assign!(Duration: Duration, StdDuration);

impl SubAssign<Duration> for StdDuration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = (*self - rhs).try_into().expect(
            "Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will \
             change the type.",
        );
    }
}

/// Implement `Mul` (reflexively) and `Div` for `Duration` for various types.
macro_rules! duration_mul_div_int {
    ($($type:ty),+) => {$(
        impl Mul<$type> for Duration {
            type Output = Self;

            fn mul(self, rhs: $type) -> Self::Output {
                Self::nanoseconds_i128(
                    self.whole_nanoseconds()
                        .checked_mul(rhs as _)
                        .expect("overflow when multiplying duration")
                )
            }
        }

        impl Mul<Duration> for $type {
            type Output = Duration;

            fn mul(self, rhs: Duration) -> Self::Output {
                rhs * self
            }
        }

        impl Div<$type> for Duration {
            type Output = Self;

            fn div(self, rhs: $type) -> Self::Output {
                Self::nanoseconds_i128(self.whole_nanoseconds() / rhs as i128)
            }
        }
    )+};
}
duration_mul_div_int![i8, i16, i32, u8, u16, u32];

impl Mul<f32> for Duration {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() * rhs)
    }
}

impl Mul<Duration> for f32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Duration {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() * rhs)
    }
}

impl Mul<Duration> for f64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl_mul_assign!(Duration: i8, i16, i32, u8, u16, u32, f32, f64);

impl Div<f32> for Duration {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() / rhs)
    }
}

impl Div<f64> for Duration {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() / rhs)
    }
}

impl_div_assign!(Duration: i8, i16, i32, u8, u16, u32, f32, f64);

impl Div for Duration {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.as_seconds_f64() / rhs.as_seconds_f64()
    }
}

impl Div<StdDuration> for Duration {
    type Output = f64;

    fn div(self, rhs: StdDuration) -> Self::Output {
        self.as_seconds_f64() / rhs.as_secs_f64()
    }
}

impl Div<Duration> for StdDuration {
    type Output = f64;

    fn div(self, rhs: Duration) -> Self::Output {
        self.as_secs_f64() / rhs.as_seconds_f64()
    }
}

impl PartialEq<StdDuration> for Duration {
    fn eq(&self, rhs: &StdDuration) -> bool {
        Ok(*self) == Self::try_from(*rhs)
    }
}

impl PartialEq<Duration> for StdDuration {
    fn eq(&self, rhs: &Duration) -> bool {
        rhs == self
    }
}

impl PartialOrd<StdDuration> for Duration {
    fn partial_cmp(&self, rhs: &StdDuration) -> Option<Ordering> {
        if rhs.as_secs() > i64::MAX as _ {
            return Some(Ordering::Less);
        }

        Some(
            self.seconds
                .cmp(&(rhs.as_secs() as _))
                .then_with(|| self.nanoseconds.cmp(&(rhs.subsec_nanos() as _))),
        )
    }
}

impl PartialOrd<Duration> for StdDuration {
    fn partial_cmp(&self, rhs: &Duration) -> Option<Ordering> {
        rhs.partial_cmp(self).map(Ordering::reverse)
    }
}

impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl<'a> Sum<&'a Self> for Duration {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
// endregion trait impls


#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::ops::Mul;
	use std::ops::Div;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::cmp::Eq;
	use std::ops::Neg;
	use std::ops::Sub;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3342() {
    rusty_monitor::set_test_id(3342);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 59i16;
    let mut i32_1: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_2: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut padding_1: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4736() {
    rusty_monitor::set_test_id(4736);
    let mut u8_0: u8 = 6u8;
    let mut i64_0: i64 = 252i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_0: i32 = -50i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_2: i64 = 132i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i32_2: i32 = -43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = -5i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 96i32;
    let mut i64_3: i64 = 185i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_4, padding: padding_0};
    let mut month_0: month::Month = crate::month::Month::September;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::August;
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1956() {
    rusty_monitor::set_test_id(1956);
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_0: u16 = 23u16;
    let mut i128_0: i128 = -61i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u16_0);
    let mut i64_1: i64 = -3i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u16_1: u16 = 50u16;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 3u8;
    let mut u16_2: u16 = 68u16;
    let mut i32_0: i32 = -182i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_2);
    let mut i32_1: i32 = 21i32;
    let mut i64_2: i64 = -98i64;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_2: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 59i16;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_3: i32 = 47i32;
    let mut i64_3: i64 = 4i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut u32_1: u32 = 18u32;
    let mut u8_6: u8 = 62u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i32_4: i32 = -152i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_7);
    let mut i64_4: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_4);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_1);
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3288() {
    rusty_monitor::set_test_id(3288);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i16_0: i16 = 59i16;
    let mut i32_1: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut i64_1: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_1);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4951() {
    rusty_monitor::set_test_id(4951);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i16_0: i16 = 59i16;
    let mut i32_1: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_1;
    let mut i32_2: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4299() {
    rusty_monitor::set_test_id(4299);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = -5i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3587() {
    rusty_monitor::set_test_id(3587);
    let mut i64_0: i64 = -79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -130i32;
    let mut i64_1: i64 = 121i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut i8_0: i8 = -125i8;
    let mut i32_1: i32 = -50i32;
    let mut i64_2: i64 = -109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_2: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i64_3: i64 = 132i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_5);
    let mut i32_4: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_5: i32 = 96i32;
    let mut i64_4: i64 = 185i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_5, padding: padding_1};
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_7);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut month_5: month::Month = crate::month::Month::December;
    let mut i32_6: i32 = crate::duration::Duration::subsec_nanoseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7029() {
    rusty_monitor::set_test_id(7029);
    let mut i64_0: i64 = 139i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_0: i32 = 188i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_1: i64 = -54i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut f64_0: f64 = -154.279545f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 74i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i8_3: i8 = -118i8;
    let mut i8_4: i8 = 34i8;
    let mut i8_5: i8 = 21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = 28.858244f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i128_0: i128 = 146i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 137i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u8_0: u8 = 61u8;
    let mut f32_1: f32 = 71.953591f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, u8_0);
    let mut i128_1: i128 = 51i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_3: i64 = -27i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_10_ref_0: &crate::duration::Duration = &mut duration_10;
    let mut i64_4: i64 = 50i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_11_ref_0: &crate::duration::Duration = &mut duration_11;
    let mut i64_5: i64 = 51i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut i8_6: i8 = -125i8;
    let mut i32_1: i32 = -50i32;
    let mut i64_6: i64 = -109i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, i8_6);
    let mut u16_0: u16 = 86u16;
    let mut i32_2: i32 = 62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_14);
    let mut i64_7: i64 = 132i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i32_3: i32 = -43i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_12);
    let mut i32_4: i32 = -5i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_5);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_5: i32 = 96i32;
    let mut i64_8: i64 = 185i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration {seconds: i64_8, nanoseconds: i32_5, padding: padding_0};
    let mut duration_17: crate::duration::Duration = std::ops::Mul::mul(duration_16, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_17);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_1: u8 = crate::time::Time::second(time_1);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_3);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_11_ref_0, duration_10_ref_0);
    let mut f64_1: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_4_ref_0);
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_183() {
    rusty_monitor::set_test_id(183);
    let mut f64_0: f64 = -86.225821f64;
    let mut i64_0: i64 = -134i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut i32_0: i32 = 80i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_1: i32 = -75i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_1: i64 = -161i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = 8i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut f32_0: f32 = -235.364164f32;
    let mut f32_1: f32 = 20.274554f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f32_0);
    let mut i32_2: i32 = -23i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_0};
    let mut i32_3: i32 = -94i32;
    let mut f64_1: f64 = -50.231717f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f64_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_8);
    let mut u16_0: u16 = crate::util::days_in_year(i32_3);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_0);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4058() {
    rusty_monitor::set_test_id(4058);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -87i32;
    let mut i64_0: i64 = 54i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i16_0: i16 = 49i16;
    let mut i32_1: i32 = 19i32;
    let mut i64_1: i64 = 17i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_1: i16 = 59i16;
    let mut i32_3: i32 = 63i32;
    let mut i64_2: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i16_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_3;
    let mut i32_4: i32 = 47i32;
    let mut i64_3: i64 = 4i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_5: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_6);
    let mut i64_4: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_4);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut duration_8: crate::duration::Duration = std::ops::Add::add(duration_7, duration_1);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1434() {
    rusty_monitor::set_test_id(1434);
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 4u8;
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = 33i32;
    let mut i64_0: i64 = 80i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 38i32;
    let mut i64_1: i64 = 189i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 19i32;
    let mut i64_2: i64 = -52i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_2, padding: padding_1};
    let mut i64_3: i64 = 7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = 72i8;
    let mut i8_5: i8 = -29i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 14i8;
    let mut i8_7: i8 = -68i8;
    let mut i8_8: i8 = 73i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 37i8;
    let mut i128_0: i128 = -173i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i8_9);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_4: i64 = -29i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = -178i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut u32_1: u32 = 47u32;
    let mut u8_3: u8 = 75u8;
    let mut u8_4: u8 = 82u8;
    let mut u8_5: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = -15i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i64_7: i64 = -59i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut i32_3: i32 = 176i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_1, duration_14);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut month_0: month::Month = crate::month::Month::September;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5992() {
    rusty_monitor::set_test_id(5992);
    let mut i64_0: i64 = -101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut f32_0: f32 = -11.845997f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = -23i8;
    let mut i8_1: i8 = 17i8;
    let mut i8_2: i8 = 126i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut u16_0: u16 = 68u16;
    let mut i32_0: i32 = -182i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 21i32;
    let mut i64_1: i64 = -98i64;
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = -88i8;
    let mut i8_5: i8 = 43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 9i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_2);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut u8_3: u8 = crate::time::Time::second(time_0);
    let mut month_3: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_949() {
    rusty_monitor::set_test_id(949);
    let mut i8_0: i8 = 22i8;
    let mut i8_1: i8 = -115i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = -88i8;
    let mut i8_5: i8 = 43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 59i16;
    let mut i32_1: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_2: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_395() {
    rusty_monitor::set_test_id(395);
    let mut i32_0: i32 = -80i32;
    let mut i8_0: i8 = -125i8;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut i32_3: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 96i32;
    let mut i64_2: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_4, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut month_1: month::Month = crate::month::Month::November;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_173() {
    rusty_monitor::set_test_id(173);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i16_0: i16 = -75i16;
    let mut i64_0: i64 = 49i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    let mut i8_0: i8 = 37i8;
    let mut i128_0: i128 = -173i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i8_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_1: i64 = -29i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = -178i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut u32_0: u32 = 47u32;
    let mut u8_0: u8 = 75u8;
    let mut u8_1: u8 = 82u8;
    let mut u8_2: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = -15i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_10);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i64_4: i64 = -59i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut i32_1: i32 = 176i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_1, duration_12);
    let mut duration_13: crate::duration::Duration = std::ops::Add::add(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_90() {
    rusty_monitor::set_test_id(90);
    let mut u32_0: u32 = 82u32;
    let mut i64_0: i64 = 86i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut u16_0: u16 = 8u16;
    let mut i32_0: i32 = 8i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut i128_0: i128 = 151i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_1: u16 = 91u16;
    let mut i32_1: i32 = 20i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut i32_2: i32 = -142i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i8_0: i8 = 9i8;
    let mut i32_3: i32 = -40i32;
    let mut i64_1: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i8_0);
    let mut i32_4: i32 = 37i32;
    let mut i64_2: i64 = 69i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i32_4);
    let mut i32_5: i32 = 57i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut i128_0: i128 = 151i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 91u16;
    let mut i32_0: i32 = 20i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i32_1: i32 = -142i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 9i8;
    let mut i32_2: i32 = -40i32;
    let mut i64_0: i64 = 185i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_0);
    let mut i32_3: i32 = 37i32;
    let mut i64_1: i64 = 69i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_3);
    let mut i32_4: i32 = 57i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4616() {
    rusty_monitor::set_test_id(4616);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 97u16;
    let mut u8_0: u8 = 65u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 99u8;
    let mut i32_0: i32 = 107i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 81u32;
    let mut i64_0: i64 = -23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 94u8;
    let mut u8_4: u8 = 26u8;
    let mut u8_5: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 41i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut u16_1: u16 = 50u16;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 15u8;
    let mut u8_8: u8 = 3u8;
    let mut u16_2: u16 = 68u16;
    let mut i32_2: i32 = -182i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut i32_3: i32 = 21i32;
    let mut i64_1: i64 = -98i64;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u32_2: u32 = 6u32;
    let mut u8_9: u8 = 10u8;
    let mut u8_10: u8 = 98u8;
    let mut u8_11: u8 = 58u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_2);
    let mut i32_4: i32 = 9i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut i16_0: i16 = 59i16;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i16_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_3;
    let mut i32_5: i32 = 47i32;
    let mut i64_2: i64 = 4i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut u32_3: u32 = 18u32;
    let mut u8_12: u8 = 62u8;
    let mut u8_13: u8 = 36u8;
    let mut u8_14: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_3);
    let mut i32_6: i32 = -152i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_3);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_4);
    let mut i64_3: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_3);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_5);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_2, u8_8, u8_7, u8_6, u16_1);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5958() {
    rusty_monitor::set_test_id(5958);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -199i64;
    let mut i64_1: i64 = -42i64;
    let mut i64_2: i64 = 60i64;
    let mut str_0: &str = "Wxl5fQNn6NOd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_3: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_4: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut i32_3: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 96i32;
    let mut i64_5: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_5, nanoseconds: i32_4, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_1: bool = crate::duration::Duration::is_zero(duration_4);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_2: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4645() {
    rusty_monitor::set_test_id(4645);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f64_0: f64 = -170.810424f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -110i8;
    let mut i8_1: i8 = -77i8;
    let mut i8_2: i8 = -35i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 92i8;
    let mut i8_4: i8 = 20i8;
    let mut i8_5: i8 = 47i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 29u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_6: i8 = -40i8;
    let mut i8_7: i8 = 112i8;
    let mut i8_8: i8 = 127i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_0: f32 = 48.855187f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1307() {
    rusty_monitor::set_test_id(1307);
    let mut u32_0: u32 = 17u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 15u16;
    let mut i32_0: i32 = 55i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_1: u32 = 19u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 58u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut i16_0: i16 = 59i16;
    let mut i32_2: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_3: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut u32_2: u32 = 18u32;
    let mut u8_6: u8 = 62u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_4: i32 = -152i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_3, duration_2);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_4);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4221() {
    rusty_monitor::set_test_id(4221);
    let mut i8_0: i8 = -6i8;
    let mut i8_1: i8 = -100i8;
    let mut i8_2: i8 = -21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -53i8;
    let mut i8_4: i8 = 63i8;
    let mut i8_5: i8 = -12i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -48i8;
    let mut i8_7: i8 = 28i8;
    let mut i8_8: i8 = 34i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 68u32;
    let mut u8_0: u8 = 32u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 73u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -177i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 4u16;
    let mut i32_0: i32 = -54i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut u32_1: u32 = 36u32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 23i32;
    let mut i64_1: i64 = -125i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_1);
    let mut i64_2: i64 = -187i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i8_9: i8 = 42i8;
    let mut i8_10: i8 = 74i8;
    let mut i8_11: i8 = 29i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i8_12: i8 = 73i8;
    let mut i8_13: i8 = -88i8;
    let mut i8_14: i8 = 43i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_3, utcoffset_4);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut u32_2: u32 = 19u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 58u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i32_2: i32 = 9i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_1);
    let mut i16_0: i16 = 59i16;
    let mut i32_3: i32 = 63i32;
    let mut i64_3: i64 = 82i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i16_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_5;
    let mut i32_4: i32 = 47i32;
    let mut i64_4: i64 = 4i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut u32_3: u32 = 18u32;
    let mut u8_6: u8 = 62u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 9u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut i32_5: i32 = -152i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_3);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_5, duration_6);
    let mut i64_5: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_5);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_2);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_2);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    let mut i8_15: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_195() {
    rusty_monitor::set_test_id(195);
    let mut i8_0: i8 = 37i8;
    let mut i128_0: i128 = -173i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_0: i64 = -29i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = -178i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i64_2: i64 = -15i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_6);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i32_1: i32 = 176i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1881() {
    rusty_monitor::set_test_id(1881);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut f32_0: f32 = 54.248031f32;
    let mut i64_0: i64 = -122i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i8_0);
    let mut i64_2: i64 = 132i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i32_1: i32 = -43i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_2: i32 = -5i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 96i32;
    let mut i64_3: i64 = 185i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_3, padding: padding_2};
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_5);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1729() {
    rusty_monitor::set_test_id(1729);
    let mut i8_0: i8 = 51i8;
    let mut i8_1: i8 = 13i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -73i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = -95i8;
    let mut i8_4: i8 = 52i8;
    let mut i8_5: i8 = -27i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -45i8;
    let mut i8_7: i8 = 124i8;
    let mut i8_8: i8 = 32i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 77u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -156i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i8_9: i8 = -125i8;
    let mut i32_2: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_9);
    let mut u16_0: u16 = 86u16;
    let mut i32_3: i32 = 62i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_4: i32 = -43i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_2);
    let mut i32_5: i32 = -5i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_4, date_5);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_5);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_6: i32 = 96i32;
    let mut i64_2: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_6, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_3: u8 = crate::time::Time::second(time_2);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_3);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1835() {
    rusty_monitor::set_test_id(1835);
    let mut u8_0: u8 = 38u8;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 98u8;
    let mut u8_3: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i32_1: i32 = 63i32;
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_2: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u32_1: u32 = 18u32;
    let mut u8_4: u8 = 62u8;
    let mut u8_5: u8 = 36u8;
    let mut u8_6: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_1);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_3_ref_0: &crate::time::Time = &mut time_3;
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_826() {
    rusty_monitor::set_test_id(826);
    let mut i64_0: i64 = 115i64;
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = 33i32;
    let mut i64_1: i64 = 80i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 38i32;
    let mut i64_2: i64 = 189i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_3: i64 = 146i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 19i32;
    let mut i64_4: i64 = -52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_2, padding: padding_1};
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3153() {
    rusty_monitor::set_test_id(3153);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4214() {
    rusty_monitor::set_test_id(4214);
    let mut i16_0: i16 = 0i16;
    let mut f32_0: f32 = 59.003566f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i128_0: i128 = -3i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut i128_1: i128 = 118i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i16_1: i16 = 6i16;
    let mut u16_0: u16 = 50u16;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 3u8;
    let mut u16_1: u16 = 68u16;
    let mut i32_0: i32 = -182i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut i32_1: i32 = 21i32;
    let mut i64_0: i64 = -98i64;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_2: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_3: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut u32_1: u32 = 18u32;
    let mut u8_6: u8 = 62u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i32_4: i32 = -152i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_7);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_4_ref_0, duration_3_ref_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut bool_1: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3576() {
    rusty_monitor::set_test_id(3576);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 96i32;
    let mut i64_2: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_3, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut month_4: month::Month = crate::month::Month::August;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5920() {
    rusty_monitor::set_test_id(5920);
    let mut i64_0: i64 = 107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = -57i8;
    let mut i64_1: i64 = 64i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut month_0: month::Month = crate::month::Month::August;
    let mut f64_0: f64 = -170.810424f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_1: i8 = -110i8;
    let mut i8_2: i8 = -77i8;
    let mut i8_3: i8 = -35i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_0: u32 = 29u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_4: i8 = -40i8;
    let mut i8_5: i8 = 112i8;
    let mut i8_6: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut f32_0: f32 = 48.855187f32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f32_0);
    let mut f32_1: f32 = -33.034974f32;
    let mut i64_2: i64 = 15i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f32_1);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_6_ref_0, duration_3_ref_0);
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1357() {
    rusty_monitor::set_test_id(1357);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_0: i64 = -53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 16i8;
    let mut i8_1: i8 = 70i8;
    let mut i8_2: i8 = -115i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 38i8;
    let mut i8_5: i8 = -75i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 83u16;
    let mut i32_0: i32 = -119i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 79u8;
    let mut f64_0: f64 = -15.226145f64;
    let mut i128_0: i128 = 4i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u16_1: u16 = 7u16;
    let mut i32_1: i32 = 90i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_2);
    let mut i64_1: i64 = crate::duration::Duration::whole_seconds(duration_5);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut bool_1: bool = std::cmp::PartialEq::eq(duration_2_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7756() {
    rusty_monitor::set_test_id(7756);
    let mut i64_0: i64 = 47i64;
    let mut u8_0: u8 = 61u8;
    let mut f32_0: f32 = 71.953591f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i128_0: i128 = 51i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = -27i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 50i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_0: i8 = -125i8;
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_3, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_0: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_7);
    let mut i64_3: i64 = 132i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i32_1: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut i32_2: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = 96i32;
    let mut i64_4: i64 = 185i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_3, padding: padding_0};
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_10);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_1: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_4_ref_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1629() {
    rusty_monitor::set_test_id(1629);
    let mut i32_0: i32 = 2i32;
    let mut i64_0: i64 = -88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 32u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 48u8;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_1: u32 = 19u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 59i16;
    let mut i32_2: i32 = 63i32;
    let mut i64_1: i64 = 82i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_3: i32 = 47i32;
    let mut i64_2: i64 = 4i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut u32_2: u32 = 18u32;
    let mut u8_6: u8 = 62u8;
    let mut u8_7: u8 = 36u8;
    let mut u8_8: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_4: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_4);
    let mut i64_3: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_3);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_3_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut u16_0: u16 = 39u16;
    let mut i32_0: i32 = -7i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = -100i8;
    let mut i8_1: i8 = -81i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut i8_3: i8 = 108i8;
    let mut i32_1: i32 = -44i32;
    let mut i64_0: i64 = -62i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_3);
    let mut i32_2: i32 = 75i32;
    let mut i32_3: i32 = -90i32;
    let mut i64_1: i64 = -141i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_4: i32 = 1i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut f64_0: f64 = 70.756397f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, f64_0);
    let mut i32_5: i32 = -16i32;
    let mut i64_2: i64 = 51i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_4: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_2);
    let mut i32_6: i32 = crate::date::Date::to_julian_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_820() {
    rusty_monitor::set_test_id(820);
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = -8i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 4u8;
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = 33i32;
    let mut i64_2: i64 = 80i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u16_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 38i32;
    let mut i64_3: i64 = 189i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_1, padding: padding_0};
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 146i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 19i32;
    let mut i64_5: i64 = -52i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_5, nanoseconds: i32_2, padding: padding_1};
    let mut u32_1: u32 = 70u32;
    let mut u8_3: u8 = 86u8;
    let mut u8_4: u8 = 84u8;
    let mut u8_5: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = 7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = 72i8;
    let mut i8_5: i8 = -29i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 14i8;
    let mut i8_7: i8 = -68i8;
    let mut i8_8: i8 = 73i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 37i8;
    let mut i128_0: i128 = -173i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i8_9);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_7: i64 = -29i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i64_8: i64 = -178i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut u32_2: u32 = 47u32;
    let mut u8_6: u8 = 75u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 64u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_9: i64 = -15i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut i32_3: i32 = 7i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_15);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut i64_10: i64 = -59i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_17: std::time::Duration = crate::duration::Duration::abs_std(duration_16);
    let mut i32_4: i32 = 176i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_2, duration_17);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_1_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5413() {
    rusty_monitor::set_test_id(5413);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = -92i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut i32_3: i32 = -5i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::August;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = -68i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 76u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 20u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 153i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_0: i64 = 140i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_1: i32 = 189i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut f32_0: f32 = 17.673460f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_2: i32 = -55i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_1);
    let mut u32_1: u32 = 51u32;
    let mut i64_1: i64 = 49i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_1);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i64_2: i64 = -206i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: crate::duration::Duration = std::clone::Clone::clone(duration_3_ref_0);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_2);
    let mut i64_3: i64 = crate::duration::Duration::whole_hours(duration_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut i32_0: i32 = 70i32;
    let mut i8_0: i8 = 87i8;
    let mut i8_1: i8 = 100i8;
    let mut i8_2: i8 = -34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -8i8;
    let mut i8_4: i8 = 123i8;
    let mut i8_5: i8 = -17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 67u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 73i16;
    let mut i64_0: i64 = -3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut i32_1: i32 = 81i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i8_6: i8 = -12i8;
    let mut i64_1: i64 = -56i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_6);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut u16_0: u16 = crate::util::days_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1081() {
    rusty_monitor::set_test_id(1081);
    let mut u16_0: u16 = 54u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 86i32;
    let mut i64_0: i64 = 43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut f64_0: f64 = 0.548859f64;
    let mut i64_1: i64 = -138i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_0: i8 = 46i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut i8_3: i8 = 10i8;
    let mut i8_4: i8 = -7i8;
    let mut i8_5: i8 = -29i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i128_0: i128 = -48i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_6: i8 = -110i8;
    let mut i8_7: i8 = -63i8;
    let mut i8_8: i8 = -63i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 67u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 57u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 16i8;
    let mut i8_10: i8 = -60i8;
    let mut i8_11: i8 = -46i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 12i8;
    let mut i8_13: i8 = -27i8;
    let mut i8_14: i8 = 22i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 21u32;
    let mut u8_3: u8 = 91u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 68u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_4: i64 = 35i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i8_15: i8 = -127i8;
    let mut i8_16: i8 = -30i8;
    let mut i8_17: i8 = -102i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i16_0: i16 = -114i16;
    let mut i64_5: i64 = 27i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, i16_0);
    let mut f64_1: f64 = -108.259305f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i8_18: i8 = -52i8;
    let mut i8_19: i8 = -123i8;
    let mut i8_20: i8 = -3i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 35u32;
    let mut u8_6: u8 = 91u8;
    let mut u8_7: u8 = 91u8;
    let mut u8_8: u8 = 62u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_6: i64 = -81i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut i8_21: i8 = 58i8;
    let mut i8_22: i8 = -57i8;
    let mut i8_23: i8 = 16i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_1: i32 = -34i32;
    let mut i64_7: i64 = -112i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_1);
    let mut duration_15: crate::duration::Duration = std::ops::Neg::neg(duration_14);
    let mut i32_2: i32 = -182i32;
    let mut i64_8: i64 = -21i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_2);
    let mut u32_3: u32 = 7u32;
    let mut u8_9: u8 = 55u8;
    let mut u8_10: u8 = 23u8;
    let mut u8_11: u8 = 35u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = -83i8;
    let mut i8_25: i8 = 21i8;
    let mut i8_26: i8 = -73i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = -60i8;
    let mut i8_28: i8 = 3i8;
    let mut i8_29: i8 = -56i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_4: u32 = 37u32;
    let mut u8_12: u8 = 40u8;
    let mut u8_13: u8 = 32u8;
    let mut u8_14: u8 = 4u8;
    let mut u16_1: u16 = 72u16;
    let mut i32_3: i32 = 33i32;
    let mut i64_9: i64 = 80i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_3);
    let mut duration_18: crate::duration::Duration = std::ops::Mul::mul(duration_17, u16_1);
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_4: i32 = 38i32;
    let mut i64_10: i64 = 189i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration {seconds: i64_10, nanoseconds: i32_4, padding: padding_1};
    let mut duration_20: crate::duration::Duration = std::ops::Neg::neg(duration_19);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_20);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_18);
    let mut i64_11: i64 = 146i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_5: i32 = 19i32;
    let mut i64_12: i64 = -52i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration {seconds: i64_12, nanoseconds: i32_5, padding: padding_2};
    let mut u32_5: u32 = 70u32;
    let mut u8_15: u8 = 86u8;
    let mut u8_16: u8 = 84u8;
    let mut u8_17: u8 = 85u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i64_13: i64 = 7i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::days(i64_13);
    let mut i8_30: i8 = -1i8;
    let mut i8_31: i8 = 72i8;
    let mut i8_32: i8 = -29i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 14i8;
    let mut i8_34: i8 = -68i8;
    let mut i8_35: i8 = 73i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i8_36: i8 = 37i8;
    let mut i128_1: i128 = -173i128;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_25: crate::duration::Duration = std::ops::Div::div(duration_24, i8_36);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_25);
    let mut i64_14: i64 = -29i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::days(i64_14);
    let mut i64_15: i64 = -178i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_15);
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_28, duration_27);
    let mut u32_6: u32 = 47u32;
    let mut u8_18: u8 = 75u8;
    let mut u8_19: u8 = 82u8;
    let mut u8_20: u8 = 64u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i64_16: i64 = -15i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::minutes(i64_16);
    let mut i32_6: i32 = 7i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_30);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_5);
    let mut i64_17: i64 = -59i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::weeks(i64_17);
    let mut duration_32: std::time::Duration = crate::duration::Duration::abs_std(duration_31);
    let mut i32_7: i32 = 176i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_7};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut time_6: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_6, duration_32);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_14, u8_13, u8_12, u32_4);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_85() {
    rusty_monitor::set_test_id(85);
    let mut i64_0: i64 = -4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 48i8;
    let mut i8_1: i8 = 82i8;
    let mut i8_2: i8 = -67i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u8_0: u8 = 42u8;
    let mut i64_1: i64 = -136i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    let mut i8_3: i8 = 69i8;
    let mut i8_4: i8 = -49i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut u8_1: u8 = 18u8;
    let mut i64_2: i64 = -50i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7369() {
    rusty_monitor::set_test_id(7369);
    let mut u16_0: u16 = 70u16;
    let mut u8_0: u8 = 61u8;
    let mut f32_0: f32 = 71.953591f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i128_0: i128 = 51i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -27i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_1: i64 = 50i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i64_2: i64 = 51i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i8_0: i8 = -125i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_3: i64 = -109i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i8_0);
    let mut u16_1: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_8);
    let mut i64_4: i64 = 132i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut i32_3: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 96i32;
    let mut i64_5: i64 = 185i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration {seconds: i64_5, nanoseconds: i32_4, padding: padding_0};
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_10, i16_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_11);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_1: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_5_ref_0, duration_4_ref_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_9, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6527() {
    rusty_monitor::set_test_id(6527);
    let mut f32_0: f32 = -35.290039f32;
    let mut f32_1: f32 = 152.233346f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i32_0: i32 = 21i32;
    let mut i64_0: i64 = -98i64;
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = 43i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i16_0: i16 = 59i16;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_2;
    let mut i32_2: i32 = 47i32;
    let mut i64_1: i64 = 4i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut u32_1: u32 = 18u32;
    let mut u8_3: u8 = 62u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 9u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_8);
    let mut i64_2: i64 = 189i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_2);
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7716() {
    rusty_monitor::set_test_id(7716);
    let mut u8_0: u8 = 76u8;
    let mut u8_1: u8 = 13u8;
    let mut u8_2: u8 = 72u8;
    let mut i16_0: i16 = -119i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut i32_0: i32 = 90i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i32_1: i32 = -75i32;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut f32_0: f32 = 77.505261f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i8_0: i8 = -51i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 17i8;
    let mut i8_5: i8 = 43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = 12i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut f64_0: f64 = 19.036654f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = -22i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_6: i8 = 78i8;
    let mut i8_7: i8 = -59i8;
    let mut i8_8: i8 = 107i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -84i8;
    let mut i8_10: i8 = 90i8;
    let mut i8_11: i8 = -53i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -101i8;
    let mut i8_13: i8 = -19i8;
    let mut i8_14: i8 = 19i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_0: u32 = 55u32;
    let mut u8_3: u8 = 14u8;
    let mut u8_4: u8 = 73u8;
    let mut u8_5: u8 = 44u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_4: i32 = 90i32;
    let mut f64_1: f64 = 48.299972f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_4);
    let mut i128_0: i128 = 0i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_8: crate::duration::Duration = std::ops::Neg::neg(duration_7);
    let mut f64_2: f64 = std::ops::Div::div(duration_8, duration_6);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4777() {
    rusty_monitor::set_test_id(4777);
    let mut i64_0: i64 = 3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 112i32;
    let mut i64_1: i64 = 155i64;
    let mut u32_0: u32 = 37u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 1u8;
    let mut u16_0: u16 = 72u16;
    let mut i32_1: i32 = 33i32;
    let mut i64_2: i64 = 80i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u16_0);
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 38i32;
    let mut i64_3: i64 = 189i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_2, padding: padding_1};
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i8_0: i8 = 112i8;
    let mut i8_1: i8 = -5i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 146i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut u32_1: u32 = 70u32;
    let mut u8_3: u8 = 86u8;
    let mut u8_4: u8 = 84u8;
    let mut u8_5: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_3: i8 = -1i8;
    let mut i8_4: i8 = 72i8;
    let mut i8_5: i8 = -29i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 14i8;
    let mut i8_7: i8 = -68i8;
    let mut i8_8: i8 = 73i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 37i8;
    let mut i128_0: i128 = -173i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i8_9);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = -29i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i64_6: i64 = -178i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut u32_2: u32 = 47u32;
    let mut u8_6: u8 = 75u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 64u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_7: i64 = -15i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i32_3: i32 = 7i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_12);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut i64_8: i64 = -59i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut i32_4: i32 = 176i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_2, duration_14);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_1: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut duration_16: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4686() {
    rusty_monitor::set_test_id(4686);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_0: i32 = -33i32;
    let mut u16_0: u16 = 28u16;
    let mut u32_0: u32 = 32u32;
    let mut i32_1: i32 = 34i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_1: u32 = 53u32;
    let mut u8_0: u8 = 14u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i32_2: i32 = 27i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut str_0: &str = "emXoi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_1, u16_0);
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_0, month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5013() {
    rusty_monitor::set_test_id(5013);
    let mut i8_0: i8 = -127i8;
    let mut i32_0: i32 = -50i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut u16_0: u16 = 86u16;
    let mut i32_1: i32 = 62i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_2: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut i32_3: i32 = -5i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i16_0: i16 = -84i16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = 96i32;
    let mut i64_2: i64 = 185i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_4, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_4);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    let mut u8_0: u8 = crate::time::Time::second(time_0);
    let mut month_4: month::Month = crate::month::Month::August;
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    panic!("From RustyUnit with love");
}
}