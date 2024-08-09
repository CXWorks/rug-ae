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
	use std::convert::TryFrom;
	use std::ops::Sub;
	use std::ops::Neg;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4227() {
    rusty_monitor::set_test_id(4227);
    let mut i16_0: i16 = -91i16;
    let mut i32_0: i32 = 30i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -130i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = 98i8;
    let mut i8_2: i8 = 50i8;
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i32_1: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i32_2: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = 17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_3: i8 = -36i8;
    let mut i8_4: i8 = -63i8;
    let mut i8_5: i8 = -1i8;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6620() {
    rusty_monitor::set_test_id(6620);
    let mut i32_0: i32 = 17i32;
    let mut i64_0: i64 = 46i64;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_2: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut i64_3: i64 = -88i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_6);
    let mut i32_2: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_2);
    let mut i64_5: i64 = -101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_10: crate::duration::Duration = std::default::Default::default();
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_13);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_12);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_9);
    let mut i16_0: i16 = -76i16;
    let mut duration_14: crate::duration::Duration = std::default::Default::default();
    let mut duration_15: crate::duration::Duration = std::ops::Div::div(duration_14, i16_0);
    let mut duration_15_ref_0: &crate::duration::Duration = &mut duration_15;
    let mut i64_6: i64 = -87i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_16_ref_0: &crate::duration::Duration = &mut duration_16;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_16_ref_0, duration_15_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_8);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6982() {
    rusty_monitor::set_test_id(6982);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i32_0: i32 = 46i32;
    let mut i64_0: i64 = 43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_2: i64 = -2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut i64_4: i64 = -88i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_7);
    let mut i32_2: i32 = 79i32;
    let mut i64_5: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_2);
    let mut i64_6: i64 = -101i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_14);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_13);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_10);
    let mut i16_0: i16 = -76i16;
    let mut duration_15: crate::duration::Duration = std::default::Default::default();
    let mut duration_16: crate::duration::Duration = std::ops::Div::div(duration_15, i16_0);
    let mut duration_16_ref_0: &crate::duration::Duration = &mut duration_16;
    let mut i64_7: i64 = -87i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_17_ref_0: &crate::duration::Duration = &mut duration_17;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_17_ref_0, duration_16_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_9);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_5);
    let mut padding_1: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4656() {
    rusty_monitor::set_test_id(4656);
    let mut i8_0: i8 = -83i8;
    let mut i8_1: i8 = 67i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -80.335765f32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i64_1: i64 = 2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 54u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 83u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 31u32;
    let mut i64_2: i64 = 131i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u32_1);
    let mut f64_0: f64 = -90.990095f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_6: u8 = 34u8;
    let mut i64_3: i64 = -36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u8_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_4: i64 = -70i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_9);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_2: i32 = -34i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i64_5: i64 = 56i64;
    let mut u8_7: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_10, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_6: i64 = 6i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration {seconds: i64_6, nanoseconds: i32_7, padding: padding_0};
    let mut i64_7: i64 = -44i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_14: crate::duration::Duration = std::ops::Mul::mul(duration_11, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_7);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_5, u8_4, u8_3);
    let mut u8_8: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_1);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_183() {
    rusty_monitor::set_test_id(183);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 114i64;
    let mut u8_0: u8 = 29u8;
    let mut i32_0: i32 = 37i32;
    let mut i64_1: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u16_0: u16 = 15u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = -125i32;
    let mut i64_2: i64 = -28i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u16_0);
    let mut i16_0: i16 = 110i16;
    let mut i64_3: i64 = 40i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i16_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut u32_0: u32 = 2u32;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 47u8;
    let mut u8_3: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i8_0: i8 = 84i8;
    let mut i8_1: i8 = 36i8;
    let mut i8_2: i8 = 21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = -188i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6066() {
    rusty_monitor::set_test_id(6066);
    let mut i64_0: i64 = 183i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 59i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 87u8;
    let mut u8_1: u8 = 65u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 99u16;
    let mut i32_1: i32 = 14i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut i32_2: i32 = 234i32;
    let mut i128_0: i128 = -45i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u16_1: u16 = 17u16;
    let mut i32_3: i32 = -107i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i32_4: i32 = -94i32;
    let mut i64_1: i64 = 233i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_5: i32 = -145i32;
    let mut i64_2: i64 = 22i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut i128_1: i128 = -193i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_6);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i32_6: i32 = -10i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut i128_2: i128 = -102i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut i64_3: i64 = -51i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_8);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut i64_4: i64 = 51i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_7: i32 = -240i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_7};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_9);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_2);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_5_ref_0, duration_0_ref_0);
    let mut u8_4: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7883() {
    rusty_monitor::set_test_id(7883);
    let mut i32_0: i32 = -104i32;
    let mut i64_0: i64 = 128i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 48i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_1: i32 = -90i32;
    let mut i64_2: i64 = -102i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_2: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_3: i64 = 92i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_4: i64 = -2i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = -94i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_10: crate::duration::Duration = std::ops::Sub::sub(duration_9, duration_8);
    let mut i64_6: i64 = -88i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_11);
    let mut i32_3: i32 = 79i32;
    let mut i64_7: i64 = 36i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_12, i32_3);
    let mut i64_8: i64 = -101i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_15: crate::duration::Duration = std::default::Default::default();
    let mut duration_16: crate::duration::Duration = std::default::Default::default();
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_16, duration_15);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_18: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_18);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_17);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_14);
    let mut i16_0: i16 = -76i16;
    let mut duration_19: crate::duration::Duration = std::default::Default::default();
    let mut duration_20: crate::duration::Duration = std::ops::Div::div(duration_19, i16_0);
    let mut duration_20_ref_0: &crate::duration::Duration = &mut duration_20;
    let mut i64_9: i64 = -87i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::microseconds(i64_9);
    let mut duration_21_ref_0: &crate::duration::Duration = &mut duration_21;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_21_ref_0, duration_20_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_13);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_10);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut duration_22: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8152() {
    rusty_monitor::set_test_id(8152);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_0: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_1: i64 = -2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut i64_3: i64 = -88i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_6);
    let mut i64_4: i64 = -101i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_11);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_10);
    let mut i16_0: i16 = -76i16;
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = std::ops::Div::div(duration_12, i16_0);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut i64_5: i64 = -87i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_14_ref_0, duration_13_ref_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6664() {
    rusty_monitor::set_test_id(6664);
    let mut u16_0: u16 = 1u16;
    let mut i32_0: i32 = 23i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u16_1: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_1: u32 = 2u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8416() {
    rusty_monitor::set_test_id(8416);
    let mut i128_0: i128 = -183i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut i64_0: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_0: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_2: i64 = -2i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -94i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut i64_4: i64 = -88i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_9);
    let mut i32_1: i32 = 79i32;
    let mut i64_5: i64 = 36i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_1);
    let mut i64_6: i64 = -101i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut duration_14: crate::duration::Duration = std::default::Default::default();
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_14, duration_13);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_16: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_16);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_15);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_12);
    let mut i16_0: i16 = -76i16;
    let mut duration_17: crate::duration::Duration = std::default::Default::default();
    let mut duration_18: crate::duration::Duration = std::ops::Div::div(duration_17, i16_0);
    let mut duration_18_ref_0: &crate::duration::Duration = &mut duration_18;
    let mut i64_7: i64 = -87i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_19_ref_0: &crate::duration::Duration = &mut duration_19;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_19_ref_0, duration_18_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_11);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_8);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_2_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2104() {
    rusty_monitor::set_test_id(2104);
    let mut i32_0: i32 = -1i32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -131i32;
    let mut i64_0: i64 = 62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut i64_1: i64 = -28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut u16_0: u16 = 64u16;
    let mut i32_2: i32 = -78i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut str_0: &str = "fD7S4rW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -13i64;
    let mut i64_3: i64 = -58i64;
    let mut i64_4: i64 = -64i64;
    let mut str_1: &str = "3hpgD7X1jMs";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i64_5: i64 = crate::duration::Duration::whole_weeks(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_114() {
    rusty_monitor::set_test_id(114);
    let mut u32_0: u32 = 24u32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 5i32;
    let mut i64_0: i64 = 77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -40i8;
    let mut i8_2: i8 = -49i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = -106i32;
    let mut i32_2: i32 = -54i32;
    let mut i64_1: i64 = -18i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut u32_1: u32 = 59u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 25u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i64_2: i64 = 16i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 56u16;
    let mut i32_3: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i64_3: i64 = 11i64;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2764() {
    rusty_monitor::set_test_id(2764);
    let mut i8_0: i8 = -80i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_0: i32 = -98i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = -61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut f64_0: f64 = 79.932828f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = -7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut f64_1: f64 = 160.210218f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1121() {
    rusty_monitor::set_test_id(1121);
    let mut i32_0: i32 = 276i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut i16_0: i16 = -76i16;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i16_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3195() {
    rusty_monitor::set_test_id(3195);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 55u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 81u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 36u8;
    let mut u8_5: u8 = 82u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = 107i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut u8_6: u8 = 60u8;
    let mut u8_7: u8 = 18u8;
    let mut u8_8: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_9: u8 = 34u8;
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_9);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -70i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_2: i32 = -34i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i64_2: i64 = 56i64;
    let mut u8_10: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_3: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_7, padding: padding_0};
    let mut i64_4: i64 = -44i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_5, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_10);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_2, duration_2);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_8, u8_7, u8_6);
    let mut u8_11: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3929() {
    rusty_monitor::set_test_id(3929);
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_0: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u8_3: u8 = 34u8;
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_1: i32 = -34i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i64_1: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_2: i32 = -137i32;
    let mut i32_3: i32 = -28i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u16_0: u16 = 80u16;
    let mut i32_4: i32 = 34i32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_4);
    let mut u16_1: u16 = 86u16;
    let mut i32_5: i32 = 36i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_6: i32 = -274i32;
    let mut i64_2: i64 = 6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_6, padding: padding_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, u16_0);
    let mut i32_7: i32 = crate::date::Date::to_julian_day(date_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8376() {
    rusty_monitor::set_test_id(8376);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -8i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_1: i32 = 23i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_1: u16 = 74u16;
    let mut i32_2: i32 = -137i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -13i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_1: i64 = -2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut i64_3: i64 = -88i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_6);
    let mut i32_3: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_3);
    let mut i64_5: i64 = -101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_10: crate::duration::Duration = std::default::Default::default();
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_13);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_12);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_9);
    let mut i16_0: i16 = -76i16;
    let mut duration_14: crate::duration::Duration = std::default::Default::default();
    let mut duration_15: crate::duration::Duration = std::ops::Div::div(duration_14, i16_0);
    let mut duration_15_ref_0: &crate::duration::Duration = &mut duration_15;
    let mut i64_6: i64 = -87i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_16_ref_0: &crate::duration::Duration = &mut duration_16;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_16_ref_0, duration_15_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_8);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
    let mut u8_9: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6184() {
    rusty_monitor::set_test_id(6184);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = -24.408858f64;
    let mut f32_0: f32 = 106.912439f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut i32_0: i32 = 109i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i32_1: i32 = -8i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_2: i32 = 23i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_1: u32 = 8u32;
    let mut u8_3: u8 = 72u8;
    let mut u8_4: u8 = 81u8;
    let mut u8_5: u8 = 5u8;
    let mut u16_1: u16 = 74u16;
    let mut i32_3: i32 = -137i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut i64_0: i64 = 92i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_2: u32 = 1u32;
    let mut f32_1: f32 = 28.986262f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_3: u32 = 2u32;
    let mut u8_6: u8 = 41u8;
    let mut u8_7: u8 = 61u8;
    let mut u8_8: u8 = 58u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut i64_1: i64 = -2i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_6, duration_5);
    let mut i64_3: i64 = -88i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_8);
    let mut i32_4: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_4);
    let mut i64_5: i64 = -101i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_4: u32 = 11u32;
    let mut u8_9: u8 = 89u8;
    let mut u8_10: u8 = 51u8;
    let mut u8_11: u8 = 96u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_4);
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_15);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_14);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_11);
    let mut i16_0: i16 = -76i16;
    let mut duration_16: crate::duration::Duration = std::default::Default::default();
    let mut duration_17: crate::duration::Duration = std::ops::Div::div(duration_16, i16_0);
    let mut duration_17_ref_0: &crate::duration::Duration = &mut duration_17;
    let mut i64_6: i64 = -87i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_18_ref_0: &crate::duration::Duration = &mut duration_18;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_18_ref_0, duration_17_ref_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_10);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_7);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_2);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_4, i32_1);
    let mut u8_12: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8130() {
    rusty_monitor::set_test_id(8130);
    let mut i32_0: i32 = -8i32;
    let mut u16_0: u16 = 1u16;
    let mut i32_1: i32 = 23i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_1: u32 = 2u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i64_1: i64 = -94i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i64_2: i64 = -88i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_4);
    let mut i32_2: i32 = 79i32;
    let mut i64_3: i64 = 36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_2);
    let mut i64_4: i64 = 101i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_2: u32 = 11u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_11);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_10);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_7);
    let mut i16_0: i16 = -76i16;
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = std::ops::Div::div(duration_12, i16_0);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut i64_5: i64 = -87i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_14_ref_0, duration_13_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_6);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_104() {
    rusty_monitor::set_test_id(104);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_0_ref_0: &crate::instant::Instant = &mut instant_0;
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut i32_0: i32 = -92i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = -61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut padding_3: time::Padding = crate::time::Padding::Optimize;
    let mut f64_0: f64 = 79.932828f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_1: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut f64_1: f64 = 160.210218f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_714() {
    rusty_monitor::set_test_id(714);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -4i64;
    let mut i64_1: i64 = 44i64;
    let mut i64_2: i64 = -43i64;
    let mut str_0: &str = "j0PxGG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i64_3: i64 = -56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = 122i8;
    let mut i8_2: i8 = 52i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 4u32;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -93i32;
    let mut i64_4: i64 = 118i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_3: i8 = 38i8;
    let mut i8_4: i8 = 89i8;
    let mut i8_5: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i32_1: i32 = -28i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut f64_0: f64 = -188.632218f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 35i32;
    let mut i64_0: i64 = 90i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut u8_0: u8 = 29u8;
    let mut i64_1: i64 = 56i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    let mut u16_0: u16 = 53u16;
    let mut i32_1: i32 = -45i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_2: i32 = 67i32;
    let mut u16_1: u16 = 26u16;
    let mut i32_3: i32 = 122i32;
    let mut i64_2: i64 = -27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u16_1);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i128_0: i128 = 106i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = -64.276705f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u32_0: u32 = 29u32;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 2u8;
    let mut u8_3: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u8_4: u8 = crate::time::Time::second(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2944() {
    rusty_monitor::set_test_id(2944);
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = -94i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i64_2: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut i32_0: i32 = 79i32;
    let mut i64_3: i64 = 36i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_0);
    let mut i64_4: i64 = -101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_10);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_9);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_6);
    let mut i16_0: i16 = -76i16;
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i16_0);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut i64_5: i64 = -87i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_13_ref_0, duration_12_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6350() {
    rusty_monitor::set_test_id(6350);
    let mut i32_0: i32 = -22i32;
    let mut i64_0: i64 = -166i64;
    let mut i64_1: i64 = 29i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i64_2: i64 = 69i64;
    let mut i32_1: i32 = -64i32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i128_0: i128 = -24i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 9i64;
    let mut i64_4: i64 = 114i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u32_1: u32 = 21u32;
    let mut u8_3: u8 = 86u8;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 9u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 34i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_1, i32_2);
    let mut u16_0: u16 = 86u16;
    let mut i32_3: i32 = 36i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_5: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_3, padding: padding_0};
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_5, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7882() {
    rusty_monitor::set_test_id(7882);
    let mut i128_0: i128 = -52i128;
    let mut f64_0: f64 = 115.951475f64;
    let mut i32_0: i32 = 101i32;
    let mut i64_0: i64 = 49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_2: i64 = -2i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -94i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut i64_4: i64 = -88i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_9);
    let mut i32_2: i32 = 79i32;
    let mut i64_5: i64 = 36i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_10, i32_2);
    let mut i64_6: i64 = -101i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_2: u32 = 11u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut duration_14: crate::duration::Duration = std::default::Default::default();
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_14, duration_13);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_16: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_16);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_15);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_12);
    let mut i16_0: i16 = -76i16;
    let mut duration_17: crate::duration::Duration = std::default::Default::default();
    let mut duration_18: crate::duration::Duration = std::ops::Div::div(duration_17, i16_0);
    let mut duration_18_ref_0: &crate::duration::Duration = &mut duration_18;
    let mut i64_7: i64 = -87i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_19_ref_0: &crate::duration::Duration = &mut duration_19;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_19_ref_0, duration_18_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_11);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_8);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_2);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_124() {
    rusty_monitor::set_test_id(124);
    let mut u16_0: u16 = 3u16;
    let mut i32_0: i32 = 57i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i32_1: i32 = -87i32;
    let mut i64_0: i64 = -71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i32_2: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i16_0: i16 = 52i16;
    let mut f32_0: f32 = 170.931974f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 61u8;
    let mut i32_3: i32 = 36i32;
    let mut i32_4: i32 = -36i32;
    let mut i64_1: i64 = -133i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut f64_0: f64 = 82.750485f64;
    let mut u16_1: u16 = 66u16;
    let mut i64_2: i64 = -57i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u16_1);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, f64_0);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_0, weekday_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4762() {
    rusty_monitor::set_test_id(4762);
    let mut i64_0: i64 = -24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_0: i32 = 8i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut u16_0: u16 = 57u16;
    let mut i64_1: i64 = 93i64;
    let mut i128_0: i128 = 119i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_0: i8 = -104i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = 119i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -89i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 137i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i8_3: i8 = -30i8;
    let mut i8_4: i8 = -50i8;
    let mut i8_5: i8 = 78i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -90i8;
    let mut i8_7: i8 = -55i8;
    let mut i8_8: i8 = 36i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -30i8;
    let mut i8_10: i8 = 25i8;
    let mut i8_11: i8 = 102i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_1: u32 = 61u32;
    let mut u8_3: u8 = 30u8;
    let mut u8_4: u8 = 20u8;
    let mut u8_5: u8 = 40u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_1: i128 = 60i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_3, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3930() {
    rusty_monitor::set_test_id(3930);
    let mut f64_0: f64 = 155.367645f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 119i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_1: i64 = -2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut i64_3: i64 = -88i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_7);
    let mut i32_2: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_2);
    let mut i64_5: i64 = -101i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_14);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_13);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_10);
    let mut i16_0: i16 = -76i16;
    let mut duration_15: crate::duration::Duration = std::default::Default::default();
    let mut duration_16: crate::duration::Duration = std::ops::Div::div(duration_15, i16_0);
    let mut duration_16_ref_0: &crate::duration::Duration = &mut duration_16;
    let mut i64_6: i64 = -87i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_17_ref_0: &crate::duration::Duration = &mut duration_17;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_17_ref_0, duration_16_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_9);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut duration_18: crate::duration::Duration = std::clone::Clone::clone(duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8262() {
    rusty_monitor::set_test_id(8262);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = 13i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_0: i32 = 86i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i64_1: i64 = -134i64;
    let mut i32_1: i32 = 31i32;
    let mut f64_0: f64 = -151.543846f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 0i64;
    let mut i32_2: i32 = 219i32;
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 34u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 69u16;
    let mut i32_3: i32 = -6i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut u32_1: u32 = 22u32;
    let mut u8_3: u8 = 15u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 24u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = -52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i32_4: i32 = 10i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_2);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u16_1: u16 = 80u16;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_1);
    let mut i32_5: i32 = 36i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_4: i64 = 6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_5, padding: padding_0};
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_7, u16_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut i16_0: i16 = -235i16;
    let mut f64_0: f64 = 53.591483f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut u32_0: u32 = 83u32;
    let mut u32_1: u32 = 21u32;
    let mut i128_0: i128 = -3i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut u32_2: u32 = 68u32;
    let mut f32_0: f32 = 120.909601f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_2);
    let mut u16_0: u16 = 19u16;
    let mut i32_0: i32 = -128i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = -83i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut f64_1: f64 = 36.807305f64;
    let mut i128_1: i128 = -77i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, f64_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_8, duration_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_minutes(duration_9);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut duration_10: crate::duration::Duration = std::ops::Neg::neg(duration_5);
    let mut i16_1: i16 = crate::duration::Duration::subsec_milliseconds(duration_3);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, u32_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4182() {
    rusty_monitor::set_test_id(4182);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_0: u8 = 47u8;
    let mut i32_0: i32 = -55i32;
    let mut u32_0: u32 = 8u32;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 81u8;
    let mut u8_3: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = 92i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_1: u32 = 2u32;
    let mut u8_4: u8 = 41u8;
    let mut u8_5: u8 = 61u8;
    let mut u8_6: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i64_1: i64 = -2i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut i64_3: i64 = -88i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_5);
    let mut i32_2: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_2);
    let mut i64_5: i64 = -101i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_2: u32 = 11u32;
    let mut u8_7: u8 = 89u8;
    let mut u8_8: u8 = 51u8;
    let mut u8_9: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = std::default::Default::default();
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i16_0: i16 = -76i16;
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, i16_0);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut i64_6: i64 = -87i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_15_ref_0: &crate::duration::Duration = &mut duration_15;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_15_ref_0, duration_14_ref_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_3, u8_2, u8_1, u32_0);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3188() {
    rusty_monitor::set_test_id(3188);
    let mut i32_0: i32 = -64i32;
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut u32_0: u32 = 65u32;
    let mut i32_1: i32 = -84i32;
    let mut i64_2: i64 = -41i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u32_0);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i64_3: i64 = -84i64;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_2: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u8_3: u8 = 34u8;
    let mut i64_4: i64 = -36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u8_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = -70i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_9);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_3: i32 = -34i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_6: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_4: i32 = -137i32;
    let mut i32_5: i32 = -28i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut u16_0: u16 = 80u16;
    let mut i32_6: i32 = 34i32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_10, i32_6);
    let mut u16_1: u16 = 86u16;
    let mut i32_7: i32 = 36i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_8: i32 = -274i32;
    let mut i64_7: i64 = 6i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration {seconds: i64_7, nanoseconds: i32_8, padding: padding_0};
    let mut i64_8: i64 = -44i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_14: crate::duration::Duration = std::ops::Mul::mul(duration_11, u16_0);
    let mut i32_9: i32 = crate::date::Date::to_julian_day(date_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_4, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_5_ref_0, duration_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7003() {
    rusty_monitor::set_test_id(7003);
    let mut i64_0: i64 = -28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 64u16;
    let mut i32_0: i32 = -78i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = true;
    let mut i64_1: i64 = -102i64;
    let mut i64_2: i64 = -39i64;
    let mut i64_3: i64 = 46i64;
    let mut str_0: &str = "fD7S4rW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_509() {
    rusty_monitor::set_test_id(509);
    let mut i64_0: i64 = -101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i16_0: i16 = -76i16;
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i16_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i64_1: i64 = -87i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_7_ref_0, duration_6_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1310() {
    rusty_monitor::set_test_id(1310);
    let mut i32_0: i32 = 79i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut i16_0: i16 = -76i16;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i16_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i64_2: i64 = -87i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_9_ref_0, duration_8_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4631() {
    rusty_monitor::set_test_id(4631);
    let mut i32_0: i32 = 220i32;
    let mut i32_1: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_0: u8 = 34u8;
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -70i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut u8_1: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = -137i32;
    let mut i32_3: i32 = -28i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u16_0: u16 = 86u16;
    let mut i32_4: i32 = 36i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_5: i32 = -274i32;
    let mut i64_2: i64 = 6i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_5, padding: padding_0};
    let mut i64_3: i64 = -44i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_6: i32 = crate::date::Date::to_julian_day(date_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_1);
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1712() {
    rusty_monitor::set_test_id(1712);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -148i32;
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_1: i32 = -77i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 99.456951f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 173i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut i32_3: i32 = 28i32;
    let mut i32_4: i32 = -63i32;
    let mut i64_1: i64 = 78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_5: i32 = 51i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut i64_2: i64 = crate::duration::Duration::whole_days(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_336() {
    rusty_monitor::set_test_id(336);
    let mut u8_0: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -137i32;
    let mut i32_1: i32 = -28i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u16_0: u16 = 86u16;
    let mut i32_2: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = -274i32;
    let mut i64_0: i64 = 6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_3, padding: padding_0};
    let mut i64_1: i64 = -44i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -81i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut u8_1: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_1);
    let mut i32_4: i32 = crate::date::Date::to_julian_day(date_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6455() {
    rusty_monitor::set_test_id(6455);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut month_1: month::Month = crate::month::Month::November;
    let mut i64_0: i64 = -59i64;
    let mut i64_1: i64 = -30i64;
    let mut i32_0: i32 = 55i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = -59i32;
    let mut i32_2: i32 = -18i32;
    let mut i64_2: i64 = 73i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_1);
    let mut i32_3: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i64_3: i64 = -8i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i8_0: i8 = 29i8;
    let mut i8_1: i8 = -112i8;
    let mut i8_2: i8 = -76i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_4: i32 = 61i32;
    let mut i64_4: i64 = 32i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut i64_5: i64 = -107i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i32_5: i32 = -28i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 80u16;
    let mut i32_6: i32 = 34i32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_6, i32_5);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_6: i64 = 6i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_6, padding: padding_0};
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_7, u16_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5617() {
    rusty_monitor::set_test_id(5617);
    let mut u32_0: u32 = 34u32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut f32_0: f32 = 53.828438f32;
    let mut i64_0: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut u32_1: u32 = 68u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 36u8;
    let mut u8_2: u8 = 9u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut u32_2: u32 = 82u32;
    let mut u8_3: u8 = 96u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 19u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut u32_3: u32 = 71u32;
    let mut u8_6: u8 = 81u8;
    let mut u8_7: u8 = 34u8;
    let mut u8_8: u8 = 96u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut u16_0: u16 = 23u16;
    let mut i32_0: i32 = 101i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i32_1: i32 = 87i32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -125i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_4: u32 = 25u32;
    let mut u8_9: u8 = 30u8;
    let mut u8_10: u8 = 39u8;
    let mut u8_11: u8 = 57u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_4);
    let mut i8_0: i8 = -11i8;
    let mut i8_1: i8 = -20i8;
    let mut i8_2: i8 = -17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_5: u32 = 93u32;
    let mut u8_12: u8 = 77u8;
    let mut u8_13: u8 = 25u8;
    let mut u8_14: u8 = 66u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_4);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u32_6: u32 = 28u32;
    let mut u8_15: u8 = 6u8;
    let mut u8_16: u8 = 63u8;
    let mut u8_17: u8 = 18u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_6: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut i8_3: i8 = 28i8;
    let mut i8_4: i8 = -99i8;
    let mut i8_5: i8 = -10i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5416() {
    rusty_monitor::set_test_id(5416);
    let mut i8_0: i8 = 66i8;
    let mut i8_1: i8 = -75i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 70u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_0: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_6: u8 = 34u8;
    let mut i64_1: i64 = -36i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u8_6);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -70i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_4);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i32_2: i32 = -34i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i64_3: i64 = 56i64;
    let mut u8_7: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_4: i64 = 6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_7, padding: padding_0};
    let mut i64_5: i64 = -44i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_6, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_7);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_3);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_5, u8_4, u8_3);
    let mut tuple_1: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_739() {
    rusty_monitor::set_test_id(739);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut u16_0: u16 = 15u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -125i32;
    let mut i64_0: i64 = -28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut i16_0: i16 = 110i16;
    let mut i64_1: i64 = 40i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 84i8;
    let mut i8_1: i8 = 36i8;
    let mut i8_2: i8 = 21i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -188i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_3: i8 = -5i8;
    let mut i8_4: i8 = 68i8;
    let mut i8_5: i8 = -44i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7317() {
    rusty_monitor::set_test_id(7317);
    let mut i64_0: i64 = 137i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_0: u16 = 85u16;
    let mut i32_0: i32 = -107i32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_3: u8 = 34u8;
    let mut i64_1: i64 = -36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u8_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -70i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_2: i32 = -34i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_1: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i32_5);
    let mut u16_2: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_4: i64 = -44i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_7, u16_1);
    let mut i32_7: i32 = crate::date::Date::to_julian_day(date_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut result_3: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_1);
    let mut u8_5: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6932() {
    rusty_monitor::set_test_id(6932);
    let mut i64_0: i64 = 58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut i64_1: i64 = -17i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i64_2: i64 = 135i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut i64_3: i64 = -114i64;
    let mut i64_4: i64 = -2i64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = -61i32;
    let mut i32_1: i32 = -18i32;
    let mut i64_5: i64 = 73i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_0);
    let mut i32_2: i32 = 0i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut i64_6: i64 = -8i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i8_0: i8 = 29i8;
    let mut i8_1: i8 = -112i8;
    let mut i8_2: i8 = -76i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_3: i32 = 61i32;
    let mut i64_7: i64 = 32i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_3);
    let mut i32_4: i32 = -137i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i32_5: i32 = 34i32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_8, i32_5);
    let mut u16_0: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i64_8: i64 = 6i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_6, padding: padding_0};
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut duration_13: crate::duration::Duration = std::ops::Mul::mul(duration_10, u16_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_2_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1412() {
    rusty_monitor::set_test_id(1412);
    let mut u32_0: u32 = 92u32;
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut i32_0: i32 = 79i32;
    let mut i64_1: i64 = 36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut i64_2: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    let mut i16_0: i16 = -76i16;
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i16_0);
    let mut duration_10_ref_0: &crate::duration::Duration = &mut duration_10;
    let mut i64_3: i64 = -87i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_11_ref_0: &crate::duration::Duration = &mut duration_11;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_11_ref_0, duration_10_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6418() {
    rusty_monitor::set_test_id(6418);
    let mut i8_0: i8 = -82i8;
    let mut i8_1: i8 = 126i8;
    let mut i8_2: i8 = 71i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = -102i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_3: u8 = 34u8;
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = -70i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_2: i32 = -34i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i64_2: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_3: i64 = 6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_7, padding: padding_0};
    let mut i64_4: i64 = -44i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_5, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_2);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_645() {
    rusty_monitor::set_test_id(645);
    let mut f64_0: f64 = -177.249530f64;
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_1: f64 = 99.456951f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_0: i32 = 173i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_1: i32 = 28i32;
    let mut i32_2: i32 = -63i32;
    let mut i64_1: i64 = 78i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_3: i32 = 51i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6426() {
    rusty_monitor::set_test_id(6426);
    let mut i8_0: i8 = -88i8;
    let mut i64_0: i64 = 221i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i32_0: i32 = 208i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut date_1_ref_0: &crate::date::Date = &mut date_1;
    let mut i64_1: i64 = -120i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = -64i32;
    let mut i64_2: i64 = 231i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut f32_0: f32 = 91.691153f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = -88i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_3: i64 = 79i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_3: i32 = -71i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_5);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_3);
    let mut i64_4: i64 = -10i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i128_0: i128 = -24i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 58i8;
    let mut i8_3: i8 = -4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i64_5: i64 = 114i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2145() {
    rusty_monitor::set_test_id(2145);
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = -94i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i64_2: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut i32_0: i32 = 79i32;
    let mut i64_3: i64 = 36i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_0);
    let mut i64_4: i64 = -101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i16_0: i16 = -76i16;
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i16_0);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut i64_5: i64 = -87i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_13_ref_0, duration_12_ref_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8274() {
    rusty_monitor::set_test_id(8274);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i16_0: i16 = 93i16;
    let mut f64_0: f64 = 24.525907f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_0: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 92i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i64_1: i64 = -2i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -94i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_6, duration_5);
    let mut i64_3: i64 = -88i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_8);
    let mut i32_1: i32 = 79i32;
    let mut i64_4: i64 = 36i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_1);
    let mut i64_5: i64 = -101i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_2: u32 = 11u32;
    let mut u8_3: u8 = 89u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_13, duration_12);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_15: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_15);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_14);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_11);
    let mut i16_1: i16 = -76i16;
    let mut duration_16: crate::duration::Duration = std::default::Default::default();
    let mut duration_17: crate::duration::Duration = std::ops::Div::div(duration_16, i16_1);
    let mut duration_17_ref_0: &crate::duration::Duration = &mut duration_17;
    let mut i64_6: i64 = -87i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_18_ref_0: &crate::duration::Duration = &mut duration_18;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_18_ref_0, duration_17_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_10);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_7);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7626() {
    rusty_monitor::set_test_id(7626);
    let mut i8_0: i8 = -24i8;
    let mut i64_0: i64 = -107i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut i64_1: i64 = 117i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_0: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u8_3: u8 = 34u8;
    let mut i64_2: i64 = -36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u8_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_3: i64 = -70i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_1: i32 = -34i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_4: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_2: i32 = -137i32;
    let mut i32_3: i32 = -28i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut u16_0: u16 = 80u16;
    let mut i32_4: i32 = 34i32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, i32_4);
    let mut u16_1: u16 = 86u16;
    let mut i32_5: i32 = 36i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_6: i32 = -274i32;
    let mut i64_5: i64 = 6i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration {seconds: i64_5, nanoseconds: i32_6, padding: padding_0};
    let mut i64_6: i64 = -44i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_13: crate::duration::Duration = std::ops::Mul::mul(duration_10, u16_0);
    let mut i32_7: i32 = crate::date::Date::to_julian_day(date_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4746() {
    rusty_monitor::set_test_id(4746);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i16_0: i16 = 68i16;
    let mut f64_0: f64 = -76.482461f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 262i32;
    let mut i64_0: i64 = -68i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_1};
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_3: u8 = 34u8;
    let mut i64_1: i64 = -36i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u8_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = -70i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_6);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_2: i32 = -34i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_4: i64 = 6i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_7, padding: padding_2};
    let mut i64_5: i64 = -44i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_8, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_5);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut i32_9: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7633() {
    rusty_monitor::set_test_id(7633);
    let mut i32_0: i32 = -71i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = -34i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 37u8;
    let mut i32_1: i32 = 105i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u8_3: u8 = 34u8;
    let mut i64_1: i64 = -36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u8_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -70i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i32_2: i32 = -34i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = 56i64;
    let mut u8_4: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_3: i32 = -137i32;
    let mut i32_4: i32 = -28i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u16_0: u16 = 80u16;
    let mut i32_5: i32 = 34i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i32_5);
    let mut u16_1: u16 = 86u16;
    let mut i32_6: i32 = 36i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_7: i32 = -274i32;
    let mut i64_4: i64 = 6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_7, padding: padding_0};
    let mut i64_5: i64 = -44i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_7, u16_0);
    let mut i32_8: i32 = crate::date::Date::to_julian_day(date_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_2, u8_1, u8_0);
    let mut month_1: month::Month = crate::month::Month::December;
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_1);
    let mut u8_5: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_525() {
    rusty_monitor::set_test_id(525);
    let mut i64_0: i64 = -80i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_0: f64 = 18.874058f64;
    let mut i64_1: i64 = -97i64;
    let mut i8_0: i8 = -48i8;
    let mut i8_1: i8 = 92i8;
    let mut i8_2: i8 = 15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = -61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut f64_1: f64 = 79.932828f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_5_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3414() {
    rusty_monitor::set_test_id(3414);
    let mut u16_0: u16 = 31u16;
    let mut i32_0: i32 = -86i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 82u8;
    let mut i32_1: i32 = -94i32;
    let mut i32_2: i32 = 79i32;
    let mut i64_0: i64 = -2i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = -94i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i64_2: i64 = -88i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut i32_3: i32 = 79i32;
    let mut i64_3: i64 = 36i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_3);
    let mut i64_4: i64 = -101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_0: u32 = 11u32;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 51u8;
    let mut u8_3: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_8, duration_7);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_10);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_9);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_6);
    let mut i16_0: i16 = -76i16;
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i16_0);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut i64_5: i64 = -87i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut duration_13_ref_0: &crate::duration::Duration = &mut duration_13;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_13_ref_0, duration_12_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_5);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_644() {
    rusty_monitor::set_test_id(644);
    let mut i32_0: i32 = 79i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut i16_0: i16 = -76i16;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i16_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i64_2: i64 = -87i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_9_ref_0, duration_8_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5114() {
    rusty_monitor::set_test_id(5114);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 30i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -130i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = 132i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut i32_1: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i32_2: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_2: i64 = 17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut utcoffset_0_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_0;
    let mut i8_0: i8 = -36i8;
    let mut i8_1: i8 = -63i8;
    let mut i8_2: i8 = -1i8;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8247() {
    rusty_monitor::set_test_id(8247);
    let mut i32_0: i32 = 168i32;
    let mut i64_0: i64 = 168i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut u16_0: u16 = 52u16;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 47i32;
    let mut i64_1: i64 = -42i64;
    let mut i128_0: i128 = 119i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = -104i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = 119i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = -89i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 90u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 137i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i8_3: i8 = -30i8;
    let mut i8_4: i8 = -50i8;
    let mut i8_5: i8 = 78i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -90i8;
    let mut i8_7: i8 = -55i8;
    let mut i8_8: i8 = 36i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -30i8;
    let mut i8_10: i8 = 25i8;
    let mut i8_11: i8 = 102i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_4: i64 = 6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_5, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1821() {
    rusty_monitor::set_test_id(1821);
    let mut u32_0: u32 = 55u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 88u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 39i32;
    let mut i64_0: i64 = 47i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut f32_0: f32 = 33.301773f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_1: i32 = 63i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_1: i64 = -45i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_4);
    let mut f32_1: f32 = -7.143486f32;
    let mut i64_2: i64 = -187i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, f32_1);
    let mut i32_2: i32 = -200i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_3: i32 = -92i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i64_3: i64 = -61i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut f64_0: f64 = 79.932828f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_4: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut f64_1: f64 = 160.210218f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_9);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_11);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::to_julian_day(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5987() {
    rusty_monitor::set_test_id(5987);
    let mut f64_0: f64 = 47.905621f64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 43i32;
    let mut i64_0: i64 = 27i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut i32_1: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i32_2: i32 = -239i32;
    let mut i64_1: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_3: i32 = -71i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut i64_2: i64 = -10i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i128_0: i128 = -24i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 9i64;
    let mut i8_0: i8 = -2i8;
    let mut i8_1: i8 = 58i8;
    let mut i8_2: i8 = -4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 114i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut u16_0: u16 = 16u16;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5970() {
    rusty_monitor::set_test_id(5970);
    let mut i64_0: i64 = 168i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = -107i32;
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 81u8;
    let mut u8_2: u8 = 5u8;
    let mut u16_0: u16 = 74u16;
    let mut i32_1: i32 = -137i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_1: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = -60i8;
    let mut i8_2: i8 = -12i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 1u32;
    let mut f32_0: f32 = 28.986262f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_2: u32 = 2u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 58u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut i64_2: i64 = -2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut i64_4: i64 = -88i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_7);
    let mut i32_2: i32 = 79i32;
    let mut i64_5: i64 = 36i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_2);
    let mut i64_6: i64 = -101i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_3: u32 = 11u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 51u8;
    let mut u8_8: u8 = 96u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut duration_11: crate::duration::Duration = std::default::Default::default();
    let mut duration_12: crate::duration::Duration = std::default::Default::default();
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_14);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_13);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_10);
    let mut i16_0: i16 = -76i16;
    let mut duration_15: crate::duration::Duration = std::default::Default::default();
    let mut duration_16: crate::duration::Duration = std::ops::Div::div(duration_15, i16_0);
    let mut duration_16_ref_0: &crate::duration::Duration = &mut duration_16;
    let mut i64_7: i64 = -87i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut duration_17_ref_0: &crate::duration::Duration = &mut duration_17;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_17_ref_0, duration_16_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_9);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = crate::util::days_in_year(i32_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut bool_1: bool = crate::duration::Duration::is_zero(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_738() {
    rusty_monitor::set_test_id(738);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -28i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 80u16;
    let mut i32_1: i32 = 34i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_1);
    let mut u16_1: u16 = 86u16;
    let mut i32_2: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_3: i32 = -274i32;
    let mut i64_1: i64 = 6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_3, padding: padding_0};
    let mut i64_2: i64 = -44i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_1, u16_0);
    let mut i32_4: i32 = crate::date::Date::to_julian_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_368() {
    rusty_monitor::set_test_id(368);
    let mut i32_0: i32 = 79i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut i16_0: i16 = -76i16;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i16_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i64_2: i64 = -87i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_9_ref_0, duration_8_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2602() {
    rusty_monitor::set_test_id(2602);
    let mut u32_0: u32 = 33u32;
    let mut i32_0: i32 = 120i32;
    let mut i64_0: i64 = -9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i32_1: i32 = 118i32;
    let mut i64_1: i64 = 16i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut u16_0: u16 = 22u16;
    let mut i32_2: i32 = -128i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_1: u32 = 99u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 80u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = -53i8;
    let mut i8_2: i8 = 34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_3: i8 = 89i8;
    let mut i64_2: i64 = 52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i8_3);
    let mut i32_3: i32 = 31i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 18u8;
    let mut u8_5: u8 = 37u8;
    let mut i32_4: i32 = 105i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u8_6: u8 = 34u8;
    let mut i64_3: i64 = -36i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u8_6);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_4: i64 = -70i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_8);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i32_5: i32 = -34i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i64_5: i64 = 56i64;
    let mut u8_7: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_6: i32 = -137i32;
    let mut i32_7: i32 = -28i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_7);
    let mut u16_1: u16 = 80u16;
    let mut i32_8: i32 = 34i32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, i32_8);
    let mut u16_2: u16 = 86u16;
    let mut i32_9: i32 = 36i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_9, u16_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_10: i32 = -274i32;
    let mut i64_6: i64 = 6i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration {seconds: i64_6, nanoseconds: i32_10, padding: padding_0};
    let mut i64_7: i64 = -44i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_13: crate::duration::Duration = std::ops::Mul::mul(duration_10, u16_1);
    let mut i32_11: i32 = crate::date::Date::to_julian_day(date_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_6, month_0, u8_7);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_2, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_3, u8_5, u8_4, u8_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1814() {
    rusty_monitor::set_test_id(1814);
    let mut i64_0: i64 = -108i64;
    let mut i32_0: i32 = 12i32;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_1: i32 = -97i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_1: i64 = -61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut f64_0: f64 = 79.932828f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut f64_1: f64 = 160.210218f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_1, i32_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1077() {
    rusty_monitor::set_test_id(1077);
    let mut i32_0: i32 = 79i32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i64_1: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 96u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut i16_0: i16 = -76i16;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i16_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i64_2: i64 = -87i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_9_ref_0, duration_8_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_2);
    let mut month_0: month::Month = crate::month::Month::January;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u32_0: u32 = 5u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -147i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i16_0: i16 = 220i16;
    let mut i64_0: i64 = -126i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut i64_1: i64 = 75i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = -19i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 211i32;
    let mut i64_3: i64 = -48i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_1, padding: padding_0};
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = -44i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u16_0: u16 = crate::time::Time::millisecond(time_1);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_2);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_1);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::nanosecond(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_530() {
    rusty_monitor::set_test_id(530);
    let mut f32_0: f32 = 214.067119f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u8_0: u8 = 16u8;
    let mut i64_0: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u8_0);
    let mut i32_0: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i32_1: i32 = -89i32;
    let mut i64_1: i64 = 2i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut u8_1: u8 = 85u8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_2: i32 = -137i32;
    let mut i32_3: i32 = -28i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_2: i64 = 0i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 86u16;
    let mut i32_4: i32 = 36i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_5: i32 = -274i32;
    let mut i64_3: i64 = 6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_5, padding: padding_0};
    let mut i64_4: i64 = -44i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = -81i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_10);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_7);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut u8_2: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_2);
    let mut i32_6: i32 = crate::date::Date::to_julian_day(date_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_2, month_0, u8_1);
    let mut i32_7: i32 = crate::duration::Duration::subsec_nanoseconds(duration_5);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_0);
    panic!("From RustyUnit with love");
}
}