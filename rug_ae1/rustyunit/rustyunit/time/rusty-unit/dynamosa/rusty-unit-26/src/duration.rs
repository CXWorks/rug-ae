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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::ops::Mul;
	use std::ops::Add;
	use std::ops::Div;
	use std::cmp::PartialOrd;
	use std::ops::Sub;
	use std::cmp::Eq;
	use std::ops::Neg;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6350() {
    rusty_monitor::set_test_id(6350);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -63i32;
    let mut i64_0: i64 = 8i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut i32_1: i32 = 93i32;
    let mut i32_2: i32 = -7i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 91u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_5: i64 = -84i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_1);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut month_3: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7550() {
    rusty_monitor::set_test_id(7550);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut i32_0: i32 = -53i32;
    let mut u16_0: u16 = 5u16;
    let mut i32_1: i32 = 36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_0: i64 = -27i64;
    let mut i64_1: i64 = -78i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 30u16;
    let mut i32_2: i32 = 22i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_2: i64 = -14i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_3: i32 = -31i32;
    let mut i64_3: i64 = 55i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_3, i32_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_518() {
    rusty_monitor::set_test_id(518);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i64_0: i64 = 94i64;
    let mut i64_1: i64 = 53i64;
    let mut u16_0: u16 = 48u16;
    let mut i32_0: i32 = 169i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = 43i8;
    let mut i8_1: i8 = -88i8;
    let mut i8_2: i8 = -16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_2: i64 = -117i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_3: i64 = -81i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_4: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_1, i32_1);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_2);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2019() {
    rusty_monitor::set_test_id(2019);
    let mut i32_0: i32 = -138i32;
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_1: i32 = -100i32;
    let mut i64_1: i64 = -151i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_1);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i64_2: i64 = -133i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 66u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut u16_0: u16 = 20u16;
    let mut i32_3: i32 = -103i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i64_3: i64 = 109i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut i32_4: i32 = 69i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_7);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8325() {
    rusty_monitor::set_test_id(8325);
    let mut i32_0: i32 = 197i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 153i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = 3i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_3: i64 = -84i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut i64_8: i64 = crate::duration::Duration::whole_minutes(duration_0);
    let mut u8_0: u8 = crate::date::Date::iso_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5985() {
    rusty_monitor::set_test_id(5985);
    let mut i64_0: i64 = -135i64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = -101i32;
    let mut i64_1: i64 = -44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 73u16;
    let mut i32_1: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i8_0: i8 = -21i8;
    let mut i64_2: i64 = -105i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut duration_4_ref_0: &std::time::Duration = &mut duration_4;
    let mut i16_0: i16 = -7i16;
    let mut i32_2: i32 = 61i32;
    let mut i64_3: i64 = -20i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i64_4: i64 = 117i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = -154i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut i64_6: i64 = -170i64;
    let mut i32_3: i32 = 3i32;
    let mut i64_7: i64 = -6i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_8: i64 = -81i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut i64_9: i64 = -7i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_10: i64 = -26i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_11: i64 = crate::duration::Duration::whole_days(duration_12);
    let mut f64_0: f64 = std::ops::Div::div(duration_11, duration_10);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_4_ref_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7034() {
    rusty_monitor::set_test_id(7034);
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_8);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_104() {
    rusty_monitor::set_test_id(104);
    let mut i64_0: i64 = 23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -49i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i8_0: i8 = -75i8;
    let mut i8_1: i8 = -14i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = -34i32;
    let mut i64_1: i64 = 15i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut f32_0: f32 = -63.534104f32;
    let mut f32_1: f32 = -36.059265f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut i8_3: i8 = 46i8;
    let mut i8_4: i8 = -1i8;
    let mut i8_5: i8 = 12i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i64_2: i64 = -77i64;
    let mut i32_2: i32 = -29i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_2);
    let mut u16_0: u16 = 51u16;
    let mut i32_3: i32 = 44i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i32_4: i32 = 65i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut f32_2: f32 = 14.002301f32;
    let mut i64_3: i64 = 25i64;
    let mut i32_5: i32 = 102i32;
    let mut u16_1: u16 = 84u16;
    let mut i64_4: i64 = 106i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u16_1);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, i32_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_1);
    let mut u8_1: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1275() {
    rusty_monitor::set_test_id(1275);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7148() {
    rusty_monitor::set_test_id(7148);
    let mut u8_0: u8 = 5u8;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_0: i64 = -239i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i32_0: i32 = 93i32;
    let mut i32_1: i32 = -7i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut u32_0: u32 = 50u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 27u8;
    let mut u8_3: u8 = 91u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i64_5: i64 = -84i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut f64_0: f64 = std::ops::Div::div(duration_9, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_3, u8_2, u8_1, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_2_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6720() {
    rusty_monitor::set_test_id(6720);
    let mut f64_0: f64 = 62.770298f64;
    let mut i64_0: i64 = -3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 101i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -58i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = -40i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 86u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 6i8;
    let mut i8_10: i8 = -34i8;
    let mut i8_11: i8 = -8i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3874() {
    rusty_monitor::set_test_id(3874);
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5432() {
    rusty_monitor::set_test_id(5432);
    let mut i64_0: i64 = 324i64;
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 101i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 11i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_4: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 43i32;
    let mut i32_1: i32 = 126i32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_4, i32_0);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_6);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_1);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2319() {
    rusty_monitor::set_test_id(2319);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i128_0: i128 = 253i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = 9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 71.225837f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -124i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i32_1: i32 = -190i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = -66i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u32_0: u32 = 99u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 68i8;
    let mut i8_4: i8 = 67i8;
    let mut i8_5: i8 = -1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::January;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4007() {
    rusty_monitor::set_test_id(4007);
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -170i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_3: i64 = -81i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut f64_0: f64 = std::ops::Div::div(duration_4, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6016() {
    rusty_monitor::set_test_id(6016);
    let mut i32_0: i32 = -147i32;
    let mut i32_1: i32 = -63i32;
    let mut i64_0: i64 = 125i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut i32_2: i32 = -48i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_1: i64 = 118i64;
    let mut month_0: month::Month = crate::month::Month::February;
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 9u16;
    let mut i32_3: i32 = -127i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut month_1: month::Month = crate::month::Month::April;
    let mut i32_4: i32 = -20i32;
    let mut i8_0: i8 = -107i8;
    let mut i8_1: i8 = -31i8;
    let mut i8_2: i8 = -39i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_5: i32 = -24i32;
    let mut i64_2: i64 = -16i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_2, i32_4);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3352() {
    rusty_monitor::set_test_id(3352);
    let mut i8_0: i8 = -12i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 61i8;
    let mut i8_4: i8 = 79i8;
    let mut i8_5: i8 = 78i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_6: i8 = 65i8;
    let mut i64_0: i64 = -108i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_6);
    let mut i32_0: i32 = 27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i32_1: i32 = 93i32;
    let mut i32_2: i32 = -7i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = -84i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_1);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_960() {
    rusty_monitor::set_test_id(960);
    let mut i32_0: i32 = 31i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut i32_1: i32 = -108i32;
    let mut u16_0: u16 = 7u16;
    let mut i64_0: i64 = 87i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut i128_0: i128 = 75i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = -181i64;
    let mut i64_2: i64 = -78i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_1: u16 = 30u16;
    let mut i32_2: i32 = 22i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = -14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = -31i32;
    let mut i64_4: i64 = 55i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut i8_0: i8 = -80i8;
    let mut i8_1: i8 = 18i8;
    let mut i8_2: i8 = 83i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -95.779601f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_4: i32 = -50i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_7);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_1, i32_1);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_8);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_976() {
    rusty_monitor::set_test_id(976);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut u32_0: u32 = 95u32;
    let mut f64_0: f64 = -114.360566f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut u16_0: u16 = 90u16;
    let mut i32_0: i32 = -16i32;
    let mut i64_0: i64 = 6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut i32_1: i32 = 117i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut i64_1: i64 = 44i64;
    let mut u32_1: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut i64_5: i64 = -81i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut f64_1: f64 = std::ops::Div::div(duration_9, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_2, u8_1, u8_0, u32_1);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_1, duration_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3129() {
    rusty_monitor::set_test_id(3129);
    let mut i64_0: i64 = -223i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_0: i32 = -41i32;
    let mut i64_1: i64 = -38i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = 174i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i32_2: i32 = 93i32;
    let mut i32_3: i32 = -7i32;
    let mut i64_2: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 91u8;
    let mut i64_3: i64 = -154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -170i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_4: i32 = 3i32;
    let mut i64_5: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_7);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut i64_6: i64 = -84i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut i64_8: i64 = -34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_5: i32 = 43i32;
    let mut i32_6: i32 = 126i32;
    let mut i64_9: i64 = -26i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i32_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_5);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut f64_0: f64 = std::ops::Div::div(duration_9, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_1);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3163() {
    rusty_monitor::set_test_id(3163);
    let mut f32_0: f32 = -110.299323f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 129i32;
    let mut i64_0: i64 = 95i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -97i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i32_1: i32 = 63i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_2: i32 = 63i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_2: i64 = 52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -8i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut u8_0: u8 = 25u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i32_3: i32 = 37i32;
    let mut f64_0: f64 = -88.867846f64;
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_0);
    let mut i64_4: i64 = 65i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_0);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3580() {
    rusty_monitor::set_test_id(3580);
    let mut u8_0: u8 = 25u8;
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 27u8;
    let mut u8_3: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -8i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_3, u8_2, u8_1, u32_0);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, u8_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_minutes(duration_9);
    let mut month_2: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2356() {
    rusty_monitor::set_test_id(2356);
    let mut u32_0: u32 = 64u32;
    let mut i32_0: i32 = 8i32;
    let mut i64_0: i64 = -187i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -88i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut u16_0: u16 = 29u16;
    let mut i32_1: i32 = 170i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_2: i64 = 44i64;
    let mut i64_3: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_5: i64 = -81i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_7);
    let mut f64_0: f64 = std::ops::Div::div(duration_6, duration_5);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5429() {
    rusty_monitor::set_test_id(5429);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = 206i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 55i8;
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 68u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 27u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -70i32;
    let mut i64_1: i64 = 8i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut i32_1: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_2: i64 = 44i64;
    let mut u32_1: u32 = 50u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 7u8;
    let mut i64_3: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -170i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 3i32;
    let mut i64_5: i64 = -6i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i64_6: i64 = -81i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = -7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_8);
    let mut f64_0: f64 = std::ops::Div::div(duration_7, duration_6);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5877() {
    rusty_monitor::set_test_id(5877);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 32u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 11u8;
    let mut f64_0: f64 = 10.395355f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f64_1: f64 = 90.693752f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_1: u32 = 69u32;
    let mut u8_3: u8 = 61u8;
    let mut u8_4: u8 = 56u8;
    let mut u8_5: u8 = 19u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = 114i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -8i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = -48i32;
    let mut i64_0: i64 = -71i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_0);
    let mut i8_3: i8 = -92i8;
    let mut i8_4: i8 = -33i8;
    let mut i8_5: i8 = 43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -99i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i8_6: i8 = 81i8;
    let mut i8_7: i8 = 37i8;
    let mut i8_8: i8 = -41i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i128_0: i128 = -6i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_2: u32 = 91u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 89u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 59u16;
    let mut i64_2: i64 = 36i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u16_0);
    let mut i8_9: i8 = -77i8;
    let mut i8_10: i8 = 24i8;
    let mut i8_11: i8 = 85i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 43i8;
    let mut i8_13: i8 = -75i8;
    let mut i8_14: i8 = 87i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u8_9: u8 = 97u8;
    let mut i64_3: i64 = 44i64;
    let mut u32_3: u32 = 50u32;
    let mut u8_10: u8 = 52u8;
    let mut u8_11: u8 = 27u8;
    let mut u8_12: u8 = 7u8;
    let mut i64_4: i64 = -154i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -170i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = 3i32;
    let mut i64_6: i64 = -6i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut duration_11: crate::duration::Duration = std::ops::Add::add(duration_10, duration_9);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_11);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_8);
    let mut i64_7: i64 = -81i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -7i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_9: i64 = -26i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_16: crate::duration::Duration = std::ops::Div::div(duration_15, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_14);
    let mut f64_2: f64 = std::ops::Div::div(duration_13, duration_12);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_12, u8_11, u8_10, u32_3);
    let mut duration_17: crate::duration::Duration = std::ops::Div::div(duration_16, u8_9);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1791() {
    rusty_monitor::set_test_id(1791);
    let mut f64_0: f64 = -72.237158f64;
    let mut f32_0: f32 = 45.611574f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut f32_1: f32 = 3.187158f32;
    let mut i32_0: i32 = -202i32;
    let mut i64_0: i64 = -69i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 105i32;
    let mut i64_1: i64 = -119i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut i32_2: i32 = 5i32;
    let mut i64_2: i64 = -252i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_2);
    let mut i64_3: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_4: i64 = -154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -170i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 3i32;
    let mut i64_6: i64 = -6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_3);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_9);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_6);
    let mut i64_7: i64 = -81i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -7i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_9: i64 = -26i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_5, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_14);
    let mut f64_1: f64 = std::ops::Div::div(duration_11, duration_10);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_3);
    let mut duration_15: crate::duration::Duration = std::ops::Div::div(duration_2, f32_1);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_874() {
    rusty_monitor::set_test_id(874);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 165i64;
    let mut i64_1: i64 = -189i64;
    let mut i64_2: i64 = 86i64;
    let mut str_0: &str = "Uxr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i16_0: i16 = -130i16;
    let mut i128_0: i128 = -81i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 4i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = -63i32;
    let mut i64_3: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_1, padding: padding_0};
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_0);
    let mut u16_0: u16 = 21u16;
    let mut i32_2: i32 = 153i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut u16_1: u16 = 21u16;
    let mut i32_3: i32 = -92i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_3, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1320() {
    rusty_monitor::set_test_id(1320);
    let mut i32_0: i32 = 90i32;
    let mut i64_0: i64 = -54i64;
    let mut i8_0: i8 = 79i8;
    let mut i8_1: i8 = -25i8;
    let mut i8_2: i8 = 44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i8_3: i8 = -107i8;
    let mut i8_4: i8 = -31i8;
    let mut i8_5: i8 = -39i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = -24i32;
    let mut i64_1: i64 = -16i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i128_0: i128 = 105i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_2: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_3: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_4: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_2, i32_2);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_7, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2513() {
    rusty_monitor::set_test_id(2513);
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -170i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_3: i64 = -81i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut f64_0: f64 = std::ops::Div::div(duration_4, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1849() {
    rusty_monitor::set_test_id(1849);
    let mut i64_0: i64 = 68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = -78i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_0: u16 = 30u16;
    let mut i32_0: i32 = 22i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_2: i64 = -14i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_1: i32 = -31i32;
    let mut i64_3: i64 = 55i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut i32_2: i32 = -50i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i32_3: i32 = 5i32;
    let mut i64_4: i64 = 25i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 69u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_4_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1519() {
    rusty_monitor::set_test_id(1519);
    let mut i32_0: i32 = 23i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut i32_1: i32 = 79i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_2: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_1);
    let mut i64_4: i64 = -81i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8283() {
    rusty_monitor::set_test_id(8283);
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2018() {
    rusty_monitor::set_test_id(2018);
    let mut i32_0: i32 = -57i32;
    let mut i64_0: i64 = -8i64;
    let mut i64_1: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_4: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_5: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5582() {
    rusty_monitor::set_test_id(5582);
    let mut i64_0: i64 = -127i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_0: i32 = -113i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 54i32;
    let mut i64_1: i64 = 41i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_2: i64 = 44i64;
    let mut i64_3: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 3i32;
    let mut i64_5: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_6: i64 = -7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4752() {
    rusty_monitor::set_test_id(4752);
    let mut u16_0: u16 = 58u16;
    let mut i8_0: i8 = -51i8;
    let mut i128_0: i128 = 5i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i64_0: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_5: i64 = -81i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_1, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5880() {
    rusty_monitor::set_test_id(5880);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i64_0: i64 = -107i64;
    let mut i64_1: i64 = -133i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 66u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut u16_0: u16 = 20u16;
    let mut i32_1: i32 = -103i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_2: i64 = 109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = -84i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_3, i32_2);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_5);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_4, i32_3);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4207() {
    rusty_monitor::set_test_id(4207);
    let mut i64_0: i64 = 3i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i64_5: i64 = -81i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_7);
    let mut f64_0: f64 = std::ops::Div::div(duration_6, duration_5);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_177() {
    rusty_monitor::set_test_id(177);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 9i8;
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut i32_0: i32 = 44i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 40i8;
    let mut i8_3: i8 = 72i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_1);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 110i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3786() {
    rusty_monitor::set_test_id(3786);
    let mut i64_0: i64 = 23i64;
    let mut i32_0: i32 = 98i32;
    let mut i64_1: i64 = -93i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut i64_2: i64 = 102i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i64_3: i64 = 18i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut i32_1: i32 = 49i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i64_4: i64 = -154i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -170i64;
    let mut i32_2: i32 = 3i32;
    let mut i64_6: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_7: i64 = -81i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i64_9: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_1, duration_1);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_898() {
    rusty_monitor::set_test_id(898);
    let mut u32_0: u32 = 48u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 73u8;
    let mut i32_0: i32 = 51i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u8_3: u8 = 81u8;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 115i32;
    let mut i64_0: i64 = -51i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut i32_2: i32 = -73i32;
    let mut i64_1: i64 = -191i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_0: i8 = -83i8;
    let mut i8_1: i8 = -71i8;
    let mut i8_2: i8 = 45i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut u16_0: u16 = 19u16;
    let mut i32_3: i32 = 83i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = -5.403479f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -222.412122f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut u8_4: u8 = 98u8;
    let mut i64_2: i64 = -27i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_8, u8_4);
    let mut i32_4: i32 = -92i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_9);
    let mut i32_5: i32 = -44i32;
    let mut i64_3: i64 = -87i64;
    let mut bool_0: bool = true;
    let mut i64_4: i64 = -85i64;
    let mut i64_5: i64 = -68i64;
    let mut i64_6: i64 = -11i64;
    let mut str_0: &str = "zmv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut u32_1: u32 = 28u32;
    let mut u8_5: u8 = 65u8;
    let mut u8_6: u8 = 91u8;
    let mut u8_7: u8 = 50u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_7, u8_6, u8_5, u32_1);
    let mut i64_7: i64 = -55i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i8_3: i8 = -37i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_8: i64 = -10i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_2, u8_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_982() {
    rusty_monitor::set_test_id(982);
    let mut i64_0: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 3i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut i64_3: i64 = -84i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 133i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_313() {
    rusty_monitor::set_test_id(313);
    let mut i32_0: i32 = 18i32;
    let mut i64_0: i64 = 232i64;
    let mut i64_1: i64 = 44i64;
    let mut i64_2: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_0, duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_225() {
    rusty_monitor::set_test_id(225);
    let mut i32_0: i32 = -72i32;
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 165i64;
    let mut i64_1: i64 = -189i64;
    let mut i64_2: i64 = 86i64;
    let mut str_0: &str = "Uxr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i16_0: i16 = -130i16;
    let mut i128_0: i128 = -81i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 20u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 4i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i32_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3193() {
    rusty_monitor::set_test_id(3193);
    let mut u16_0: u16 = 99u16;
    let mut u8_0: u8 = 65u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 46u8;
    let mut i64_0: i64 = -62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut i128_0: i128 = -97i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = 100i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i32_1: i32 = 93i32;
    let mut i32_2: i32 = -7i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u32_0: u32 = 2u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 10u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_4);
    let mut i64_5: i64 = -84i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut f64_0: f64 = std::ops::Div::div(duration_9, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_1);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2102() {
    rusty_monitor::set_test_id(2102);
    let mut i64_0: i64 = 105i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 16u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 145i32;
    let mut i64_1: i64 = 86i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = -83i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut f32_0: f32 = -38.513756f32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i64_3: i64 = -135i64;
    let mut u16_0: u16 = 5u16;
    let mut i32_1: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i64_4: i64 = -27i64;
    let mut i64_5: i64 = -78i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut u16_1: u16 = 30u16;
    let mut i32_2: i32 = 22i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_3);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i64_6: i64 = -14i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_3: i32 = -31i32;
    let mut i64_7: i64 = 55i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_3);
    let mut i64_8: i64 = -5i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut f64_0: f64 = -98.130897f64;
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_6, f64_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_12: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8415() {
    rusty_monitor::set_test_id(8415);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 28u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_1: i64 = 44i64;
    let mut u32_1: u32 = 50u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut i64_5: i64 = -81i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_7);
    let mut f64_0: f64 = std::ops::Div::div(duration_6, duration_5);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2525() {
    rusty_monitor::set_test_id(2525);
    let mut i64_0: i64 = -165i64;
    let mut i64_1: i64 = 47i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i32_0: i32 = 11i32;
    let mut i64_2: i64 = -38i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i32_1: i32 = 93i32;
    let mut i32_2: i32 = -7i32;
    let mut i64_3: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 91u8;
    let mut i64_4: i64 = -154i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -170i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_3: i32 = 3i32;
    let mut i64_6: i64 = -6i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_3);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_7);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i64_7: i64 = -84i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut i64_9: i64 = -34i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_10: i64 = -26i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::weeks(i64_10);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_11: i64 = crate::duration::Duration::whole_days(duration_10);
    let mut f64_0: f64 = std::ops::Div::div(duration_9, duration_8);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_1);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_2_ref_0, duration_0_ref_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1470() {
    rusty_monitor::set_test_id(1470);
    let mut i16_0: i16 = -124i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut f64_0: f64 = 20.735932f64;
    let mut i32_0: i32 = 93i32;
    let mut i32_1: i32 = -7i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut i64_4: i64 = -84i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut i64_6: i64 = -34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_1: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_0);
    let mut duration_12: crate::duration::Duration = std::ops::Mul::mul(duration_6, f64_0);
    let mut month_3: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1179() {
    rusty_monitor::set_test_id(1179);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_8);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut f64_0: f64 = -101.659248f64;
    let mut i64_0: i64 = -77i64;
    let mut i32_0: i32 = -29i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut u16_0: u16 = 51u16;
    let mut i32_1: i32 = 44i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i32_2: i32 = 65i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut f32_0: f32 = 12.216725f32;
    let mut i64_1: i64 = 25i64;
    let mut i32_3: i32 = 102i32;
    let mut u16_1: u16 = 84u16;
    let mut i64_2: i64 = 106i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_1);
    let mut u8_1: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_444() {
    rusty_monitor::set_test_id(444);
    let mut f64_0: f64 = 42.118157f64;
    let mut i64_0: i64 = 213i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u8_0: u8 = 27u8;
    let mut f64_1: f64 = -103.537332f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u8_0);
    let mut i8_0: i8 = 75i8;
    let mut i8_1: i8 = -66i8;
    let mut i8_2: i8 = -4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut str_0: &str = "Uxr";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i16_0: i16 = -130i16;
    let mut i128_0: i128 = -81i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    let mut u32_0: u32 = 66u32;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 79u8;
    let mut u8_3: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = 4i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = -63i32;
    let mut i64_1: i64 = -101i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_0);
    let mut u16_0: u16 = 21u16;
    let mut i32_2: i32 = 153i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut f64_2: f64 = std::ops::Div::div(duration_4, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_130() {
    rusty_monitor::set_test_id(130);
    let mut i16_0: i16 = -83i16;
    let mut i64_0: i64 = 138i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i8_0: i8 = -36i8;
    let mut i8_1: i8 = -7i8;
    let mut i8_2: i8 = 25i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u16_0: u16 = 18u16;
    let mut i32_1: i32 = -88i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i8_3: i8 = 45i8;
    let mut i8_4: i8 = 44i8;
    let mut i8_5: i8 = -5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_1: u16 = 40u16;
    let mut i32_2: i32 = -61i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut i8_6: i8 = 1i8;
    let mut i8_7: i8 = -53i8;
    let mut i8_8: i8 = 122i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_3: i32 = 137i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_4, utcoffset_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_1, primitivedatetime_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_325() {
    rusty_monitor::set_test_id(325);
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -81i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 43i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5175() {
    rusty_monitor::set_test_id(5175);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_1: i32 = 43i32;
    let mut i32_2: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7820() {
    rusty_monitor::set_test_id(7820);
    let mut i32_0: i32 = 93i32;
    let mut i32_1: i32 = -7i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut f64_0: f64 = std::ops::Div::div(duration_4, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_0, i32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5984() {
    rusty_monitor::set_test_id(5984);
    let mut i32_0: i32 = 93i32;
    let mut i32_1: i32 = -7i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_2: i32 = 3i32;
    let mut i64_2: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -84i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = -7i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut i64_5: i64 = -34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 122i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut f64_0: f64 = std::ops::Div::div(duration_4, duration_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_0, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8050() {
    rusty_monitor::set_test_id(8050);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -69i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = 88i8;
    let mut i8_1: i8 = -63i8;
    let mut i8_2: i8 = -94i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 29i8;
    let mut i8_4: i8 = 8i8;
    let mut i8_5: i8 = -1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 1u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 19u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -216i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = 14i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i32_0);
    let mut i32_2: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i64_2: i64 = 45i64;
    let mut u32_1: u32 = 50u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 7u8;
    let mut i64_3: i64 = -154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -170i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_3: i32 = 3i32;
    let mut i64_5: i64 = -6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_3);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_9);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_6);
    let mut i64_6: i64 = -81i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i64_7: i64 = -7i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut i32_5: i32 = 126i32;
    let mut i64_8: i64 = -26i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_8);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, i32_5);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_9: i64 = crate::duration::Duration::whole_days(duration_12);
    let mut f64_0: f64 = std::ops::Div::div(duration_11, duration_10);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_3, u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    let mut duration_15: crate::duration::Duration = std::ops::Add::add(duration_14, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_292() {
    rusty_monitor::set_test_id(292);
    let mut i8_0: i8 = -17i8;
    let mut i8_1: i8 = -45i8;
    let mut i8_2: i8 = -70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_0: i32 = 212i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_3: i8 = 11i8;
    let mut i64_1: i64 = -208i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_3);
    let mut u16_0: u16 = 48u16;
    let mut i32_1: i32 = -6i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_2: i32 = 35i32;
    let mut i128_0: i128 = 75i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_2);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 11u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = 34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i32_3: i32 = 35i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    let mut tuple_1: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_0);
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_131() {
    rusty_monitor::set_test_id(131);
    let mut i64_0: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_0: i8 = 11i8;
    let mut i64_1: i64 = -208i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_0);
    let mut u16_0: u16 = 48u16;
    let mut i32_0: i32 = -6i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_1: i32 = 35i32;
    let mut i128_0: i128 = 75i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_1);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 11u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = 34i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_5);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_2: i32 = 35i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1258() {
    rusty_monitor::set_test_id(1258);
    let mut i32_0: i32 = -88i32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -220i32;
    let mut i64_0: i64 = 88i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_2: i32 = 34i32;
    let mut i64_1: i64 = -73i64;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut i64_2: i64 = 152i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = -6i8;
    let mut i8_2: i8 = -121i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 62i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 69i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 29u32;
    let mut u8_0: u8 = 71u8;
    let mut u8_1: u8 = 51u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -20.425789f32;
    let mut i64_3: i64 = 20i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_3, i32_2);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3424() {
    rusty_monitor::set_test_id(3424);
    let mut u16_0: u16 = 38u16;
    let mut i32_0: i32 = 91i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6738() {
    rusty_monitor::set_test_id(6738);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut u32_0: u32 = 31u32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 76i32;
    let mut i64_0: i64 = -100i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i32_1: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u32_1: u32 = 57u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut i32_2: i32 = -236i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i32_3: i32 = -53i32;
    let mut i64_1: i64 = -135i64;
    let mut u16_0: u16 = 5u16;
    let mut i32_4: i32 = 36i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut i64_2: i64 = -78i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_1: u16 = 30u16;
    let mut i32_5: i32 = 22i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut i64_3: i64 = -14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_1, i32_3);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1691() {
    rusty_monitor::set_test_id(1691);
    let mut u8_0: u8 = 71u8;
    let mut i64_0: i64 = 90i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_0: i32 = 93i32;
    let mut i32_1: i32 = -7i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut u32_0: u32 = 50u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 27u8;
    let mut u8_3: u8 = 91u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_5: i64 = -84i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut i64_7: i64 = -34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_3, u8_2, u8_1, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_3: month::Month = crate::month::Month::May;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1300() {
    rusty_monitor::set_test_id(1300);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i16_0: i16 = -155i16;
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 43i32;
    let mut i32_1: i32 = 126i32;
    let mut i64_3: i64 = -26i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i32_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6482() {
    rusty_monitor::set_test_id(6482);
    let mut i8_0: i8 = 76i8;
    let mut i8_1: i8 = -48i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_0: i64 = -86i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut f32_0: f32 = 170.888308f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i32_0: i32 = -62i32;
    let mut i8_3: i8 = -37i8;
    let mut i8_4: i8 = -102i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_6: i8 = 21i8;
    let mut i8_7: i8 = 51i8;
    let mut i8_8: i8 = 101i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -58i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = -40i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut i8_9: i8 = -100i8;
    let mut i8_10: i8 = 5i8;
    let mut i8_11: i8 = -104i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 22u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 86u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i32_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_2);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4775() {
    rusty_monitor::set_test_id(4775);
    let mut i64_0: i64 = -205i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -34i32;
    let mut i64_1: i64 = 14i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 39i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_4: i64 = -154i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = -170i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_1: i32 = 3i32;
    let mut i64_6: i64 = -6i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut duration_8: crate::duration::Duration = std::ops::Add::add(duration_7, duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut i64_7: i64 = -81i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i64_8: i64 = -7i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_9: i64 = -26i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_13: crate::duration::Duration = std::ops::Div::div(duration_12, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_11);
    let mut f64_0: f64 = std::ops::Div::div(duration_10, duration_9);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut duration_14: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1842() {
    rusty_monitor::set_test_id(1842);
    let mut i8_0: i8 = -107i8;
    let mut i8_1: i8 = 123i8;
    let mut i8_2: i8 = -28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -78i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut i64_4: i64 = -81i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = -7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_6: i64 = -26i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_2, u8_1, u8_0, u32_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5605() {
    rusty_monitor::set_test_id(5605);
    let mut i32_0: i32 = -37i32;
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut i64_1: i64 = 44i64;
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i64_3: i64 = -170i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = 3i32;
    let mut i64_4: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_5: i64 = -81i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = -7i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_2: i32 = 43i32;
    let mut i32_3: i32 = 126i32;
    let mut i64_7: i64 = -26i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, i32_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_8: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4173() {
    rusty_monitor::set_test_id(4173);
    let mut i32_0: i32 = 69i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = -41i64;
    let mut i32_1: i32 = 71i32;
    let mut i64_1: i64 = -276i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut i8_0: i8 = -8i8;
    let mut i8_1: i8 = 14i8;
    let mut i8_2: i8 = -3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_2: i64 = 39i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u32_0: u32 = 82u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 62u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 11i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = 29i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i64_4: i64 = 119i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_4, i32_3);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_6, i32_4);
    let mut u8_3: u8 = crate::date::Date::day(date_0);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6057() {
    rusty_monitor::set_test_id(6057);
    let mut i64_0: i64 = 44i64;
    let mut i64_1: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 43i32;
    let mut i32_1: i32 = 126i32;
    let mut i64_3: i64 = -26i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3592() {
    rusty_monitor::set_test_id(3592);
    let mut u16_0: u16 = 24u16;
    let mut i64_0: i64 = -56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::April;
    let mut i64_1: i64 = 64i64;
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_2: i64 = -58i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut i64_3: i64 = -40i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i32_0: i32 = 126i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_3, i32_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_4);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3995() {
    rusty_monitor::set_test_id(3995);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut i64_0: i64 = 4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f32_0: f32 = 72.937272f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Neg::neg(duration_1);
    let mut i8_0: i8 = 53i8;
    let mut i8_1: i8 = 40i8;
    let mut i8_2: i8 = 38i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut f64_0: f64 = -25.435065f64;
    let mut i32_0: i32 = 220i32;
    let mut i64_1: i64 = 113i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut u16_0: u16 = 6u16;
    let mut i32_1: i32 = -44i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut u8_0: u8 = 86u8;
    let mut i64_2: i64 = 129i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u8_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_6);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut i64_3: i64 = -107i64;
    let mut i64_4: i64 = 142i64;
    let mut i32_2: i32 = 64i32;
    let mut i64_5: i64 = 14i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut u32_0: u32 = 58u32;
    let mut u8_1: u8 = 56u8;
    let mut u8_2: u8 = 63u8;
    let mut u8_3: u8 = 80u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u32_1: u32 = 64u32;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 34u8;
    let mut u8_6: u8 = 99u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_3: i32 = 100i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_7);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i8_3: i8 = 58i8;
    let mut i8_4: i8 = 20i8;
    let mut i8_5: i8 = -31i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_4: i32 = 43i32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_8, i32_4);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_9);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_10, i32_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8165() {
    rusty_monitor::set_test_id(8165);
    let mut i128_0: i128 = -41i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = 62.672953f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_0: i32 = 35i32;
    let mut i64_0: i64 = 149i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_1: i64 = -84i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i32_1: i32 = -37i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_2: i64 = -199i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_3);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_3: i64 = -192i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut i8_0: i8 = -85i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -77i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_6);
    let mut f64_0: f64 = std::ops::Div::div(duration_10, duration_5);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2852() {
    rusty_monitor::set_test_id(2852);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut u32_0: u32 = 62u32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_0);
    let mut u16_0: u16 = 22u16;
    let mut f64_0: f64 = 3.446858f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u16_0);
    let mut i32_0: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 42i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 24i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 44i64;
    let mut u32_1: u32 = 50u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 7u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -170i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_2: i32 = 3i32;
    let mut i64_3: i64 = -6i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_9);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_6);
    let mut i64_4: i64 = -7i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_3: i32 = 43i32;
    let mut i32_4: i32 = 126i32;
    let mut i64_5: i64 = -26i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_13: crate::duration::Duration = std::ops::Div::div(duration_12, i32_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut i64_6: i64 = crate::duration::Duration::whole_days(duration_11);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_4, u8_2, u8_1, u8_0, u32_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_0);
    let mut duration_10_ref_0: &crate::duration::Duration = &mut duration_10;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_10_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut f32_0: f32 = -83.961833f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u16_0: u16 = 61u16;
    let mut i64_0: i64 = 92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u16_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 199i32;
    let mut i64_1: i64 = -31i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i32_0);
    let mut i32_1: i32 = 0i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut f64_0: f64 = -167.171500f64;
    let mut u32_0: u32 = 86u32;
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, u32_0);
    let mut i32_2: i32 = -43i32;
    let mut i64_2: i64 = 108i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut i32_3: i32 = 66i32;
    let mut i8_0: i8 = 121i8;
    let mut u8_0: u8 = 29u8;
    let mut i64_3: i64 = 63i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_8);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_7, f64_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_2, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7064() {
    rusty_monitor::set_test_id(7064);
    let mut i64_0: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i16_0: i16 = 45i16;
    let mut i64_1: i64 = -90i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = -190i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 53i8;
    let mut i8_1: i8 = 91i8;
    let mut i8_2: i8 = -54i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -36i8;
    let mut i8_4: i8 = 28i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 37i8;
    let mut i8_8: i8 = -112i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -5i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_4: i64 = -52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i64_5: i64 = 154i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i32_0: i32 = -196i32;
    let mut i64_6: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_0);
    let mut i64_7: i64 = -81i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_2_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1906() {
    rusty_monitor::set_test_id(1906);
    let mut i64_0: i64 = 177i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -94i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 83u8;
    let mut i128_0: i128 = 142i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 8u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 62u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i32_0: i32 = 113i32;
    let mut month_0: month::Month = crate::month::Month::July;
    let mut i32_1: i32 = -60i32;
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -85i64;
    let mut i64_3: i64 = -68i64;
    let mut i64_4: i64 = -11i64;
    let mut str_0: &str = "zmv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut u32_2: u32 = 28u32;
    let mut u8_6: u8 = 65u8;
    let mut u8_7: u8 = 91u8;
    let mut u8_8: u8 = 50u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_5: i64 = -55i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_6: i64 = -10i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i8_3: i8 = -98i8;
    let mut i8_4: i8 = -80i8;
    let mut i8_5: i8 = 80i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_6: i8 = -107i8;
    let mut i8_7: i8 = -35i8;
    let mut i8_8: i8 = 30i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_7: i64 = crate::duration::Duration::whole_days(duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_5, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_477() {
    rusty_monitor::set_test_id(477);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut u32_0: u32 = 67u32;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = -54i32;
    let mut i64_0: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_1};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i32_1: i32 = 129i32;
    let mut i64_1: i64 = 95i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = -97i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut i32_2: i32 = 63i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut f32_0: f32 = -140.284896f32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_3: i64 = 52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i64_4: i64 = -8i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u8_0: u8 = 25u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i32_3: i32 = 37i32;
    let mut f64_0: f64 = -98.130897f64;
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f64_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_0);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_0, duration_1);
    let mut padding_2: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8322() {
    rusty_monitor::set_test_id(8322);
    let mut i64_0: i64 = 43i64;
    let mut i8_0: i8 = -37i8;
    let mut i8_1: i8 = -102i8;
    let mut i8_2: i8 = 17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 21i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 101i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = -40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 5i8;
    let mut i8_8: i8 = -104i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_4: i64 = -34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 43i32;
    let mut i32_1: i32 = 126i32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_4, i32_1);
    let mut i64_5: i64 = crate::duration::Duration::whole_days(duration_3);
    let mut f64_0: f64 = std::ops::Div::div(duration_5, duration_6);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_0);
    panic!("From RustyUnit with love");
}
}