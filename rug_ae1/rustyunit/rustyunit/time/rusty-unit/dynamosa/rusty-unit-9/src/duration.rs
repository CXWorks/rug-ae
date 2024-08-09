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
	use std::ops::Add;
	use std::ops::Mul;
	use std::ops::Div;
	use std::cmp::PartialOrd;
	use std::ops::Neg;
	use std::ops::Sub;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7452() {
    rusty_monitor::set_test_id(7452);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut i64_2: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_4: i32 = -44i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_5);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3182() {
    rusty_monitor::set_test_id(3182);
    let mut i8_0: i8 = 50i8;
    let mut i8_1: i8 = 99i8;
    let mut i8_2: i8 = 31i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_2);
    let mut i64_2: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_4: i32 = -44i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_5);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_1);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3757() {
    rusty_monitor::set_test_id(3757);
    let mut u16_0: u16 = 42u16;
    let mut i32_0: i32 = -51i32;
    let mut f32_0: f32 = -42.982609f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i8_0: i8 = -6i8;
    let mut i8_1: i8 = -13i8;
    let mut i8_2: i8 = 102i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 82u32;
    let mut u8_0: u8 = 44u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 91u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -155i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i128_0: i128 = -90i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_1: i32 = -127i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_1_ref_0, duration_0_ref_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7961() {
    rusty_monitor::set_test_id(7961);
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = -119i32;
    let mut i64_1: i64 = -174i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3899() {
    rusty_monitor::set_test_id(3899);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i64_0: i64 = 64i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i64_1: i64 = 219i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_0: u32 = 91u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u8_3: u8 = 89u8;
    let mut i64_2: i64 = 93i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u8_3);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut f32_0: f32 = -90.330575f32;
    let mut i32_0: i32 = -121i32;
    let mut i64_3: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut u16_0: u16 = 3u16;
    let mut i32_1: i32 = -13i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_7);
    let mut i32_2: i32 = -89i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i32_3: i32 = -117i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_3);
    let mut i64_4: i64 = 15i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut f32_1: f32 = 150.380696f32;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_5: i64 = -186i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_9, f32_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_8, duration_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_12);
    let mut bool_1: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_138() {
    rusty_monitor::set_test_id(138);
    let mut i64_0: i64 = -70i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut i32_0: i32 = 61i32;
    let mut i64_1: i64 = -36i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut i32_1: i32 = -16i32;
    let mut i64_2: i64 = 83i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = 121i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u16_0: u16 = 12u16;
    let mut i32_2: i32 = 44i32;
    let mut i64_4: i64 = 28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut i8_0: i8 = -25i8;
    let mut i8_1: i8 = 25i8;
    let mut i8_2: i8 = 83i8;
    let mut i8_3: i8 = 119i8;
    let mut i64_5: i64 = -54i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, i8_3);
    let mut i64_6: i64 = -33i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_6);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_8);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_6, u16_0);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_5);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut duration_11: crate::duration::Duration = std::ops::Add::add(duration_3, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5993() {
    rusty_monitor::set_test_id(5993);
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_3: i32 = -44i32;
    let mut i64_2: i64 = 25i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_5);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_130() {
    rusty_monitor::set_test_id(130);
    let mut i64_0: i64 = -42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut f64_0: f64 = 223.342353f64;
    let mut i64_1: i64 = -207i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f64_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut u16_0: u16 = 92u16;
    let mut i32_0: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u16_2: u16 = 44u16;
    let mut i32_2: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 33u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 41u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut i16_0: i16 = -21i16;
    let mut i64_2: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i16_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_weeks(duration_4);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut u8_9: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1572() {
    rusty_monitor::set_test_id(1572);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 34u8;
    let mut u8_1: u8 = 57u8;
    let mut u8_2: u8 = 67u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -49i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut u16_0: u16 = 92u16;
    let mut i32_1: i32 = -197i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_1: u32 = 49u32;
    let mut u8_3: u8 = 98u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 75u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 98u16;
    let mut i32_2: i32 = 99i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut u16_2: u16 = 44u16;
    let mut i32_3: i32 = 34i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut u32_2: u32 = 86u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 22u8;
    let mut u8_8: u8 = 17u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u32_3: u32 = 33u32;
    let mut u8_9: u8 = 3u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 41u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_3);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_2);
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_3);
    let mut u8_12: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_3);
    let mut u8_13: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1703() {
    rusty_monitor::set_test_id(1703);
    let mut i16_0: i16 = 9i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut u32_0: u32 = 29u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 48u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 51u16;
    let mut i32_0: i32 = -145i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i32_1: i32 = -101i32;
    let mut i64_0: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut i32_2: i32 = 114i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut u16_1: u16 = 35u16;
    let mut i32_3: i32 = -10i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 67i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_1: u32 = 32u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 59u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 79i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_2: u16 = 75u16;
    let mut i32_4: i32 = -21i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3827() {
    rusty_monitor::set_test_id(3827);
    let mut i64_0: i64 = 38i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_0: i8 = 42i8;
    let mut i8_1: i8 = -33i8;
    let mut i8_2: i8 = 39i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 87u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 79u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -135i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut u32_1: u32 = 10u32;
    let mut i128_0: i128 = -157i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_2: u32 = 73u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 90u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i32_1: i32 = 76i32;
    let mut i64_1: i64 = -73i64;
    let mut u8_6: u8 = 49u8;
    let mut u8_7: u8 = 44u8;
    let mut u8_8: u8 = 1u8;
    let mut i32_2: i32 = 217i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u8_9: u8 = 44u8;
    let mut i64_2: i64 = 17i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, u8_9);
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut i64_3: i64 = 222i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_10: crate::duration::Duration = std::ops::Sub::sub(duration_9, duration_8);
    let mut duration_11: crate::duration::Duration = std::ops::Sub::sub(duration_10, duration_7);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_1, u8_8, u8_7, u8_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_3);
    let mut tuple_1: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1325() {
    rusty_monitor::set_test_id(1325);
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_1: i32 = 38i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 7i32;
    let mut i64_2: i64 = 5i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 65u32;
    let mut i64_3: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_0);
    let mut i64_4: i64 = -38i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_6, duration_5);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_7, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8660() {
    rusty_monitor::set_test_id(8660);
    let mut u8_0: u8 = 29u8;
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = -99i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = 101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut u8_1: u8 = 34u8;
    let mut i64_3: i64 = -257i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u8_1);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut f32_0: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, f32_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_9, duration_7);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_769() {
    rusty_monitor::set_test_id(769);
    let mut i32_0: i32 = 90i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 74u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -116i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut i64_0: i64 = 196i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = -75i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_3: i32 = -59i32;
    let mut i64_1: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_3, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_4: i32 = 168i32;
    let mut i64_2: i64 = 83i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_4, padding: padding_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut f32_1: f32 = 82.675518f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut month_1: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_3_ref_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut bool_1: bool = crate::duration::Duration::is_zero(duration_7);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5517() {
    rusty_monitor::set_test_id(5517);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_0: i32 = -128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i64_0: i64 = -120i64;
    let mut i32_1: i32 = 101i32;
    let mut u32_1: u32 = 56u32;
    let mut i32_2: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_1);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2970() {
    rusty_monitor::set_test_id(2970);
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut i64_2: i64 = -28i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_4: i32 = -44i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_4, duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1966() {
    rusty_monitor::set_test_id(1966);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = -243i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut u32_1: u32 = 10u32;
    let mut i128_0: i128 = -157i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_2: u32 = 73u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 32u8;
    let mut u8_5: u8 = 90u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_2);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut f64_0: f64 = -102.928376f64;
    let mut i64_0: i64 = 42i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i32_1: i32 = 76i32;
    let mut i64_1: i64 = -73i64;
    let mut u8_6: u8 = 44u8;
    let mut i64_2: i64 = 17i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, u8_6);
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut i64_3: i64 = 222i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_11: crate::duration::Duration = std::ops::Sub::sub(duration_10, duration_9);
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(duration_11, duration_8);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_6_ref_0, duration_4_ref_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_2);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8222() {
    rusty_monitor::set_test_id(8222);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = -172i64;
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 79i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f64_0: f64 = -193.549273f64;
    let mut i64_1: i64 = -85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 29i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_1: i32 = -119i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5077() {
    rusty_monitor::set_test_id(5077);
    let mut f64_0: f64 = 49.196665f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = -121i64;
    let mut u16_0: u16 = 15u16;
    let mut i32_0: i32 = -106i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 61i32;
    let mut i128_0: i128 = -69i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = -112i8;
    let mut i8_1: i8 = 10i8;
    let mut i8_2: i8 = 20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -35i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_3: i64 = 116i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = -119i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2335() {
    rusty_monitor::set_test_id(2335);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 10u32;
    let mut i128_0: i128 = -157i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_1: u32 = 73u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut f64_0: f64 = -102.928376f64;
    let mut i64_0: i64 = 46i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, f64_0);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut i32_0: i32 = 76i32;
    let mut i64_1: i64 = -73i64;
    let mut u8_3: u8 = 49u8;
    let mut u8_4: u8 = 44u8;
    let mut u8_5: u8 = 1u8;
    let mut i32_1: i32 = 217i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_5, u8_4, u8_3);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_7_ref_0, duration_5_ref_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_3);
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1137() {
    rusty_monitor::set_test_id(1137);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 10u32;
    let mut i128_0: i128 = -157i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 73u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 90u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut f64_0: f64 = -102.928376f64;
    let mut i64_0: i64 = 46i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut u8_3: u8 = 49u8;
    let mut u8_4: u8 = 44u8;
    let mut u8_5: u8 = 1u8;
    let mut i32_0: i32 = 217i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_6: u8 = 44u8;
    let mut i64_1: i64 = 17i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, u8_6);
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut i64_2: i64 = 222i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_11: crate::duration::Duration = std::ops::Sub::sub(duration_10, duration_9);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_5, u8_4, u8_3);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_6_ref_0, duration_4_ref_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    let mut u8_7: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2019() {
    rusty_monitor::set_test_id(2019);
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = -83i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -223.591359f64;
    let mut i64_0: i64 = -191i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 73u8;
    let mut u8_1: u8 = 70u8;
    let mut u8_2: u8 = 89u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = -11i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = 202i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u32_1: u32 = 48u32;
    let mut u8_3: u8 = 70u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 14u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 81u16;
    let mut i32_0: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_2};
    let mut i64_3: i64 = crate::duration::Duration::whole_weeks(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5667() {
    rusty_monitor::set_test_id(5667);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 77i64;
    let mut i64_1: i64 = -138i64;
    let mut i64_2: i64 = 167i64;
    let mut str_0: &str = "hplB";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_3: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut i32_2: i32 = 41i32;
    let mut i64_4: i64 = -37i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut i64_5: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i32_4: i32 = -44i32;
    let mut i64_6: i64 = 25i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_5);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    let mut duration_10: crate::duration::Duration = std::option::Option::unwrap(option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_143() {
    rusty_monitor::set_test_id(143);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 16u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = -78i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i16_0: i16 = -58i16;
    let mut f64_0: f64 = -5.519969f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut u16_0: u16 = 61u16;
    let mut i32_1: i32 = 82i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6461() {
    rusty_monitor::set_test_id(6461);
    let mut f32_0: f32 = -123.637925f32;
    let mut f32_1: f32 = -82.470980f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = -9i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 24i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = -50i64;
    let mut u16_0: u16 = 52u16;
    let mut i32_2: i32 = 79i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut i64_1: i64 = -85i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 29i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i8_3: i8 = -111i8;
    let mut f32_2: f32 = 25.847436f32;
    let mut i8_4: i8 = -42i8;
    let mut i8_5: i8 = -4i8;
    let mut i8_6: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i32_3: i32 = 6i32;
    let mut i64_3: i64 = -46i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_4: i32 = 52i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = -40i8;
    let mut i8_9: i8 = -46i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i8_10: i8 = 124i8;
    let mut i8_11: i8 = -23i8;
    let mut i8_12: i8 = 62i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut i64_4: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i32_5: i32 = -219i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_2};
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut i64_5: i64 = 66i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_8);
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_sub(date_7, duration_7);
    let mut u32_0: u32 = 82u32;
    let mut u8_0: u8 = 70u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 33u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_6: i64 = 100i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_3);
    let mut i64_7: i64 = 157i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i32_6: i32 = 149i32;
    let mut i64_8: i64 = -46i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_14: crate::duration::Duration = std::ops::Mul::mul(duration_11, i32_6);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i32_7: i32 = 33i32;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_7);
    let mut f64_0: f64 = std::ops::Div::div(duration_13, duration_15);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5593() {
    rusty_monitor::set_test_id(5593);
    let mut i8_0: i8 = -94i8;
    let mut i8_1: i8 = -6i8;
    let mut i8_2: i8 = -81i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 52u32;
    let mut i64_0: i64 = 130i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut u32_1: u32 = 87u32;
    let mut i32_0: i32 = -24i32;
    let mut i64_1: i64 = -28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut u16_0: u16 = 41u16;
    let mut i32_1: i32 = 139i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut f32_0: f32 = 100.895422f32;
    let mut i128_0: i128 = 57i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_2: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut i8_3: i8 = 50i8;
    let mut i64_3: i64 = 1i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i8_3);
    let mut i8_4: i8 = -32i8;
    let mut i8_5: i8 = 15i8;
    let mut i8_6: i8 = -65i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut u32_2: u32 = 3u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 95u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_2);
    let mut i32_2: i32 = 13i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_3, offset: utcoffset_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_8);
    let mut u16_1: u16 = 36u16;
    let mut i32_3: i32 = -36i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut u32_3: u32 = 83u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 33u8;
    let mut u8_5: u8 = 69u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_3);
    let mut f32_1: f32 = -58.454827f32;
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, f32_1);
    let mut i32_4: i32 = -128i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_10);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_5, time_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_6);
    let mut i64_4: i64 = -120i64;
    let mut i32_5: i32 = 101i32;
    let mut u32_4: u32 = 56u32;
    let mut i32_6: i32 = -109i32;
    let mut i64_5: i64 = -67i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_6);
    let mut duration_12: crate::duration::Duration = std::ops::Mul::mul(duration_11, u32_4);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut i64_6: i64 = -135i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_15: crate::duration::Duration = std::ops::Add::add(duration_14, duration_13);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_15, i32_5);
    let mut duration_16: crate::duration::Duration = std::default::Default::default();
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_3);
    let mut i32_7: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_4);
    let mut i32_8: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_2);
    let mut duration_18: crate::duration::Duration = std::ops::Div::div(duration_6, f32_0);
    let mut i32_9: i32 = crate::offset_date_time::OffsetDateTime::to_julian_day(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1918() {
    rusty_monitor::set_test_id(1918);
    let mut i64_0: i64 = 138i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut f32_0: f32 = 72.553630f32;
    let mut i32_0: i32 = -46i32;
    let mut i64_1: i64 = 66i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u16_0: u16 = 92u16;
    let mut i32_1: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_2: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u16_2: u16 = 44u16;
    let mut i32_3: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_2);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::month::Month::December;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5988() {
    rusty_monitor::set_test_id(5988);
    let mut i64_0: i64 = -46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -66i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i64_2: i64 = 10i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 26i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut i64_4: i64 = 48i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_5: i64 = 74i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, i16_0);
    let mut i32_1: i32 = 7i32;
    let mut i64_6: i64 = 5i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_7: i64 = -101i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_7);
    let mut duration_12: crate::duration::Duration = std::ops::Div::div(duration_11, u32_1);
    let mut i64_8: i64 = -38i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_2: i32 = 184i32;
    let mut i64_9: i64 = -4i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_9, i32_2);
    let mut duration_15: crate::duration::Duration = std::ops::Div::div(duration_14, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_15);
    let mut duration_16: crate::duration::Duration = std::ops::Sub::sub(duration_13, duration_12);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_16, duration_10);
    let mut duration_17: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2183() {
    rusty_monitor::set_test_id(2183);
    let mut i32_0: i32 = 68i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i64_0: i64 = -217i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i32_1: i32 = 138i32;
    let mut i64_1: i64 = -133i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u16_0: u16 = 92u16;
    let mut i32_2: i32 = -197i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_3: i32 = 99i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut u16_2: u16 = 44u16;
    let mut i32_4: i32 = 34i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 33u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 69u8;
    let mut u8_8: u8 = 41u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_1);
    let mut i16_0: i16 = -21i16;
    let mut i64_2: i64 = 116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_weeks(duration_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_3);
    let mut u8_9: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_1_ref_0, duration_0_ref_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1856() {
    rusty_monitor::set_test_id(1856);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u32_0: u32 = 5u32;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 85u8;
    let mut u16_0: u16 = 13u16;
    let mut i32_0: i32 = -21i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_1: u32 = 45u32;
    let mut u8_3: u8 = 4u8;
    let mut u8_4: u8 = 98u8;
    let mut u8_5: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = -76i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -223.591359f64;
    let mut i64_0: i64 = -191i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut u32_2: u32 = 80u32;
    let mut u8_6: u8 = 73u8;
    let mut u8_7: u8 = 70u8;
    let mut u8_8: u8 = 89u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = -11i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut i64_2: i64 = 202i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u32_3: u32 = 48u32;
    let mut u8_9: u8 = 70u8;
    let mut u8_10: u8 = 97u8;
    let mut u8_11: u8 = 14u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut u16_1: u16 = 81u16;
    let mut i32_1: i32 = -71i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5992() {
    rusty_monitor::set_test_id(5992);
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 38i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_1: i64 = -101i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_1);
    let mut i64_2: i64 = -38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_2: i32 = 184i32;
    let mut i64_3: i64 = -4i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_5);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_249() {
    rusty_monitor::set_test_id(249);
    let mut f32_0: f32 = 19.974467f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = 29i8;
    let mut i8_1: i8 = -69i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = -120i64;
    let mut i32_2: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i64_2: i64 = -135i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_701() {
    rusty_monitor::set_test_id(701);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i64_0: i64 = 143i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = 8i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut i64_2: i64 = 182i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i64_3: i64 = 26i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_2: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -59i32;
    let mut i64_4: i64 = 61i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_1, padding: padding_2};
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f32_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut duration_9_ref_0: &std::time::Duration = &mut duration_9;
    let mut padding_3: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 168i32;
    let mut i64_5: i64 = 83i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration {seconds: i64_5, nanoseconds: i32_2, padding: padding_3};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_11, duration_10);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut i8_0: i8 = -63i8;
    let mut f32_1: f32 = 82.675518f32;
    let mut i32_3: i32 = 111i32;
    let mut i64_6: i64 = 34i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_3);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_15: crate::duration::Duration = std::ops::Div::div(duration_13, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_12_ref_0, duration_9_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_6_ref_0, duration_3_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6397() {
    rusty_monitor::set_test_id(6397);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_0: u8 = 29u8;
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 69u8;
    let mut u8_3: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = -25i32;
    let mut i64_1: i64 = -30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut f32_0: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, f32_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1970() {
    rusty_monitor::set_test_id(1970);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i32_0: i32 = -112i32;
    let mut f64_0: f64 = -66.367420f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut f64_1: f64 = 61.217661f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_0: u32 = 79u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 25u8;
    let mut u8_2: u8 = 48u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 55u16;
    let mut i32_1: i32 = -11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u8_3: u8 = 29u8;
    let mut i64_0: i64 = -7i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_1: u32 = 58u32;
    let mut u8_4: u8 = 37u8;
    let mut u8_5: u8 = 69u8;
    let mut u8_6: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_2: i32 = -99i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -25i32;
    let mut i64_2: i64 = -30i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_8: crate::duration::Duration = std::ops::Add::add(duration_7, duration_6);
    let mut i64_3: i64 = 101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut u8_7: u8 = 34u8;
    let mut i64_4: i64 = -257i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, u8_7);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut f32_0: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, f32_0);
    let mut f64_2: f64 = std::ops::Div::div(duration_14, duration_12);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_8);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_5);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6754() {
    rusty_monitor::set_test_id(6754);
    let mut i128_0: i128 = -157i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -102.928376f64;
    let mut i64_0: i64 = 46i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f64_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i32_0: i32 = 76i32;
    let mut i64_1: i64 = -73i64;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 1u8;
    let mut i32_1: i32 = 217i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_3: u8 = 44u8;
    let mut i64_2: i64 = 17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u8_3);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut i64_3: i64 = 222i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_8, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8281() {
    rusty_monitor::set_test_id(8281);
    let mut u16_0: u16 = 66u16;
    let mut i32_0: i32 = 53i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u8_0: u8 = 12u8;
    let mut i32_1: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_2: i32 = -82i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut month_0: month::Month = crate::date::Date::month(date_2);
    let mut f64_0: f64 = 109.043940f64;
    let mut u16_1: u16 = 5u16;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 97u8;
    let mut u8_3: u8 = 1u8;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_3: i32 = 28i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_2);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_4: i32 = -119i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_4, u8_3, u8_2, u8_1, u16_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_121() {
    rusty_monitor::set_test_id(121);
    let mut i8_0: i8 = 117i8;
    let mut i8_1: i8 = -28i8;
    let mut i8_2: i8 = 72i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 68u8;
    let mut i64_0: i64 = -36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_1);
    let mut i64_1: i64 = -71i64;
    let mut i32_0: i32 = -84i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u16_0: u16 = 60u16;
    let mut i64_2: i64 = 111i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut u8_2: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_1, u8_0);
    let mut month_1: month::Month = crate::month::Month::September;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_784() {
    rusty_monitor::set_test_id(784);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i16_0: i16 = 72i16;
    let mut f64_0: f64 = 120.038099f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -59i32;
    let mut i64_0: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_1};
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_3_ref_0: &std::time::Duration = &mut duration_3;
    let mut padding_2: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 168i32;
    let mut i64_1: i64 = 83i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_2, padding: padding_2};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_5, duration_4);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i8_0: i8 = -63i8;
    let mut f32_1: f32 = 82.675518f32;
    let mut i32_3: i32 = 111i32;
    let mut i64_2: i64 = 34i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_7, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_3_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut padding_3: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2405() {
    rusty_monitor::set_test_id(2405);
    let mut u32_0: u32 = 23u32;
    let mut u8_0: u8 = 44u8;
    let mut u8_1: u8 = 82u8;
    let mut u8_2: u8 = 53u8;
    let mut u16_0: u16 = 52u16;
    let mut i32_0: i32 = 79i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut f64_0: f64 = -193.549273f64;
    let mut i64_0: i64 = -85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 29i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i8_0: i8 = -111i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 38i8;
    let mut f32_0: f32 = 25.847436f32;
    let mut i8_3: i8 = -42i8;
    let mut i8_4: i8 = -4i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_1: i32 = 52i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i8_6: i8 = 3i8;
    let mut i8_7: i8 = -40i8;
    let mut i8_8: i8 = -46i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_2: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_2: i32 = -219i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_3: i64 = 66i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 82u32;
    let mut u8_3: u8 = 70u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 33u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_4: i64 = -34i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i32_3: i32 = 19i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i64_5: i64 = 70i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i64_6: i64 = 66i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_4: i32 = 37i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1566() {
    rusty_monitor::set_test_id(1566);
    let mut f32_0: f32 = -77.678962f32;
    let mut i64_0: i64 = 62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut u8_0: u8 = 40u8;
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i32_0: i32 = -159i32;
    let mut u8_1: u8 = 29u8;
    let mut i64_1: i64 = -7i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u8_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_0: u32 = 58u32;
    let mut u8_2: u8 = 37u8;
    let mut u8_3: u8 = 69u8;
    let mut u8_4: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_4, u8_3, u8_2, u32_0);
    let mut i32_1: i32 = -99i32;
    let mut i64_2: i64 = 120i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i32_2: i32 = -25i32;
    let mut i64_3: i64 = -30i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut i64_4: i64 = 101i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut u8_5: u8 = 34u8;
    let mut i64_5: i64 = -257i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, u8_5);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut f32_1: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_13: crate::duration::Duration = std::ops::Div::div(duration_12, f32_1);
    let mut f64_0: f64 = std::ops::Div::div(duration_13, duration_11);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut duration_15: std::time::Duration = crate::duration::Duration::abs_std(duration_14);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_7);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    let mut duration_16: crate::duration::Duration = std::ops::Neg::neg(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_736() {
    rusty_monitor::set_test_id(736);
    let mut u16_0: u16 = 97u16;
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 5u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 81u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 42u16;
    let mut i32_1: i32 = -148i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u16_2: u16 = 36u16;
    let mut i32_2: i32 = -36i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 33u8;
    let mut u8_5: u8 = 69u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_3: i32 = -128i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_3, time_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_4);
    let mut i64_0: i64 = -120i64;
    let mut u32_2: u32 = 56u32;
    let mut i32_4: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_4);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_2);
    let mut i32_5: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_2);
    let mut i32_6: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4141() {
    rusty_monitor::set_test_id(4141);
    let mut u8_0: u8 = 23u8;
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_0, u8_2, u8_1, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_0: i64 = -120i64;
    let mut i32_2: i32 = 101i32;
    let mut u32_1: u32 = 56u32;
    let mut i32_3: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_2);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut i32_4: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut i32_0: i32 = -44i32;
    let mut i64_0: i64 = -81i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i128_0: i128 = 61i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 67i32;
    let mut i64_1: i64 = -183i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_2: i64 = 15i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 116i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut u32_0: u32 = 47u32;
    let mut u8_0: u8 = 18u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 26u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_4: i64 = -34i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i32_2: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8045() {
    rusty_monitor::set_test_id(8045);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_0: u8 = 89u8;
    let mut i32_0: i32 = -3i32;
    let mut u16_0: u16 = 36u16;
    let mut i32_1: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 33u8;
    let mut u8_3: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_2: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_0: i64 = -120i64;
    let mut u32_1: u32 = 56u32;
    let mut i32_3: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut i32_4: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2644() {
    rusty_monitor::set_test_id(2644);
    let mut u8_0: u8 = 50u8;
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = -27i32;
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 51u32;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 96u8;
    let mut u8_3: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    let mut i32_2: i32 = 38i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut i32_3: i32 = 7i32;
    let mut i64_2: i64 = 5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_3: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u32_1);
    let mut i64_4: i64 = -38i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_4: i32 = 184i32;
    let mut i64_5: i64 = -4i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_4);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_9);
    let mut duration_10: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_10, duration_4);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2614() {
    rusty_monitor::set_test_id(2614);
    let mut i32_0: i32 = -92i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 33i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 4u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 90u8;
    let mut u8_2: u8 = 14u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 23i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i8_0: i8 = -34i8;
    let mut i8_1: i8 = -57i8;
    let mut i8_2: i8 = -2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -46i8;
    let mut i8_4: i8 = -15i8;
    let mut i8_5: i8 = 10i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = -87.841322f32;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut duration_4_ref_0: &std::time::Duration = &mut duration_4;
    let mut i32_1: i32 = -44i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i32_1);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i8_6: i8 = 56i8;
    let mut f32_1: f32 = -20.982231f32;
    let mut i32_2: i32 = 0i32;
    let mut i64_2: i64 = -2i64;
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut u16_0: u16 = 46u16;
    let mut i32_3: i32 = -29i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_10);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_7);
    let mut i32_4: i32 = -74i32;
    let mut i64_3: i64 = -101i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_12: crate::duration::Duration = std::ops::Neg::neg(duration_11);
    let mut i32_5: i32 = 41i32;
    let mut i64_4: i64 = -37i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_5);
    let mut i32_6: i32 = -43i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_6};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_3);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_16: crate::duration::Duration = std::ops::Div::div(duration_14, i8_6);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2727() {
    rusty_monitor::set_test_id(2727);
    let mut u32_0: u32 = 94u32;
    let mut u8_0: u8 = 50u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 53u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_1: u32 = 83u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 33u8;
    let mut u8_5: u8 = 69u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_0: i64 = -120i64;
    let mut i32_2: i32 = 101i32;
    let mut u32_2: u32 = 56u32;
    let mut i32_3: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_2);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut i32_4: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut tuple_1: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2396() {
    rusty_monitor::set_test_id(2396);
    let mut i64_0: i64 = 68i64;
    let mut u16_0: u16 = 66u16;
    let mut u16_1: u16 = 92u16;
    let mut i32_0: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut u16_2: u16 = 98u16;
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_2);
    let mut i32_2: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 33u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 69u8;
    let mut u8_5: u8 = 41u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut i16_0: i16 = -21i16;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut u8_6: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7389() {
    rusty_monitor::set_test_id(7389);
    let mut f64_0: f64 = 20.937479f64;
    let mut i64_0: i64 = 140i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut u16_0: u16 = 48u16;
    let mut i32_0: i32 = -52i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut u16_1: u16 = 92u16;
    let mut i32_1: i32 = -197i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_2: u16 = 98u16;
    let mut i32_2: i32 = 99i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut u16_3: u16 = 44u16;
    let mut i32_3: i32 = 34i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_3);
    let mut u32_1: u32 = 33u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 69u8;
    let mut u8_5: u8 = 41u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut i16_0: i16 = -21i16;
    let mut i64_1: i64 = 116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_weeks(duration_3);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_4);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_3);
    let mut u8_6: u8 = crate::date::Date::sunday_based_week(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4397() {
    rusty_monitor::set_test_id(4397);
    let mut f32_0: f32 = 13.837070f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f64_0: f64 = -102.928376f64;
    let mut i64_0: i64 = 46i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f64_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i32_0: i32 = 76i32;
    let mut i64_1: i64 = -73i64;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 1u8;
    let mut i32_1: i32 = 217i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_3: u8 = 44u8;
    let mut i64_2: i64 = 17i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u8_3);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut i64_3: i64 = 222i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_8, duration_5);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_3_ref_0, duration_1_ref_0);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_9);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4001() {
    rusty_monitor::set_test_id(4001);
    let mut i8_0: i8 = -18i8;
    let mut i64_0: i64 = 170i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_1: i8 = 64i8;
    let mut i8_2: i8 = 41i8;
    let mut i8_3: i8 = 126i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u16_0: u16 = 14u16;
    let mut u8_0: u8 = 42u8;
    let mut u8_1: u8 = 76u8;
    let mut u8_2: u8 = 40u8;
    let mut i8_4: i8 = -32i8;
    let mut i8_5: i8 = 42i8;
    let mut i8_6: i8 = 122i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = 71i8;
    let mut i8_8: i8 = -110i8;
    let mut i8_9: i8 = -17i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i32_0: i32 = 75i32;
    let mut i64_1: i64 = -142i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = -202i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_1: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut i32_2: i32 = -53i32;
    let mut i64_3: i64 = -38i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_2);
    let mut i32_3: i32 = -62i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    let mut duration_5: crate::duration::Duration = std::clone::Clone::clone(duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4842() {
    rusty_monitor::set_test_id(4842);
    let mut i128_0: i128 = 117i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut i64_2: i64 = -28i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_5, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4271() {
    rusty_monitor::set_test_id(4271);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Neg::neg(duration_4);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut i64_2: i64 = -28i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_4: i32 = -44i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_9: crate::duration::Duration = std::ops::Add::add(duration_8, duration_7);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_6, duration_5);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2648() {
    rusty_monitor::set_test_id(2648);
    let mut u32_0: u32 = 43u32;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 91u8;
    let mut u8_2: u8 = 43u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = -211i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u32_1: u32 = 73u32;
    let mut u8_3: u8 = 56u8;
    let mut u8_4: u8 = 17u8;
    let mut u8_5: u8 = 42u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 58i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = -20i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut u32_2: u32 = 58u32;
    let mut u8_6: u8 = 8u8;
    let mut u8_7: u8 = 5u8;
    let mut u8_8: u8 = 15u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 30u16;
    let mut i32_0: i32 = -11i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_4_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7821() {
    rusty_monitor::set_test_id(7821);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 99i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u32_0: u32 = 38u32;
    let mut f32_0: f32 = -79.400052f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_0);
    let mut u16_0: u16 = 58u16;
    let mut i32_0: i32 = -6i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i8_0: i8 = -116i8;
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 50i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -29i8;
    let mut i8_4: i8 = -62i8;
    let mut i8_5: i8 = 17i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_1: f32 = -37.594879f32;
    let mut i64_2: i64 = -122i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_1);
    let mut f32_2: f32 = -39.371380f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut i8_6: i8 = -4i8;
    let mut i8_7: i8 = -19i8;
    let mut i8_8: i8 = 92i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f32_3: f32 = 25.697432f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_3);
    let mut duration_8: crate::duration::Duration = std::ops::Neg::neg(duration_7);
    let mut i8_9: i8 = -97i8;
    let mut i8_10: i8 = 79i8;
    let mut i8_11: i8 = -15i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_1: i32 = 89i32;
    let mut i64_3: i64 = 12i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 150i32;
    let mut i64_4: i64 = 67i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_2, padding: padding_0};
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut duration_12_ref_0: &std::time::Duration = &mut duration_12;
    let mut i64_5: i64 = 79i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut i64_6: i64 = 6i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut duration_15_ref_0: &crate::duration::Duration = &mut duration_15;
    let mut i8_12: i8 = -58i8;
    let mut i32_3: i32 = -121i32;
    let mut i64_7: i64 = 24i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_3);
    let mut u16_1: u16 = 3u16;
    let mut i32_4: i32 = -13i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_16);
    let mut i32_5: i32 = -89i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut i32_6: i32 = -117i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_6};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_5);
    let mut f32_4: f32 = 150.380696f32;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_4);
    let mut duration_18: crate::duration::Duration = std::ops::Div::div(duration_17, i8_12);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_15_ref_0, duration_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::ne(duration_1_ref_0, duration_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7778() {
    rusty_monitor::set_test_id(7778);
    let mut i8_0: i8 = -95i8;
    let mut i8_1: i8 = 47i8;
    let mut i8_2: i8 = -111i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i16_0: i16 = -60i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut f32_0: f32 = 91.382285f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_2, u8_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut u16_0: u16 = 5u16;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 97u8;
    let mut u8_3: u8 = 1u8;
    let mut f32_1: f32 = -46.329670f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_2: i32 = 28i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_5);
    let mut i64_1: i64 = 116i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_3: i32 = -119i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_3, u8_3, u8_2, u8_1, u16_0);
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_7, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut i64_3: i64 = crate::duration::Duration::whole_seconds(duration_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_1);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2083() {
    rusty_monitor::set_test_id(2083);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 34u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_3: u8 = 29u8;
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_3);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_1: u32 = 58u32;
    let mut u8_4: u8 = 37u8;
    let mut u8_5: u8 = 69u8;
    let mut u8_6: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_0: i32 = -99i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = -25i32;
    let mut i64_2: i64 = -30i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut i64_3: i64 = 101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut u8_7: u8 = 34u8;
    let mut i64_4: i64 = -257i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, u8_7);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut f32_0: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, f32_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_11, duration_9);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_13: std::time::Duration = crate::duration::Duration::abs_std(duration_12);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_5);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_1, duration_2);
    let mut u8_8: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2962() {
    rusty_monitor::set_test_id(2962);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut u16_1: u16 = 92u16;
    let mut i32_1: i32 = -197i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_2: u16 = 98u16;
    let mut i32_2: i32 = 99i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut u16_3: u16 = 44u16;
    let mut i32_3: i32 = 34i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_3);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_1);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_3);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_2);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut tuple_1: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1424() {
    rusty_monitor::set_test_id(1424);
    let mut i64_0: i64 = -76i64;
    let mut u16_0: u16 = 83u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u32_0: u32 = 56u32;
    let mut i32_2: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5095() {
    rusty_monitor::set_test_id(5095);
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = 37i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -119i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_539() {
    rusty_monitor::set_test_id(539);
    let mut i8_0: i8 = -11i8;
    let mut i8_1: i8 = -54i8;
    let mut i8_2: i8 = -86i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 108i8;
    let mut i8_4: i8 = -47i8;
    let mut i8_5: i8 = -26i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 98u16;
    let mut i32_0: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i64_0: i64 = 77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i32_1: i32 = 247i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u8_0: u8 = 97u8;
    let mut i64_1: i64 = -25i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    let mut u32_0: u32 = 81u32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 100i32;
    let mut i64_2: i64 = 85i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_2, padding: padding_0};
    let mut i64_3: i64 = -102i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u32_0);
    let mut i32_3: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_6);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8672() {
    rusty_monitor::set_test_id(8672);
    let mut f32_0: f32 = -139.127213f32;
    let mut i32_0: i32 = -158i32;
    let mut i64_0: i64 = -128i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_1: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_1: i64 = -120i64;
    let mut i64_2: i64 = -135i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3143() {
    rusty_monitor::set_test_id(3143);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -59i32;
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 168i32;
    let mut i64_1: i64 = 83i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_2, padding: padding_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i8_0: i8 = -63i8;
    let mut f32_1: f32 = 82.675518f32;
    let mut i32_3: i32 = 111i32;
    let mut i64_2: i64 = 34i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_6, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_5_ref_0, duration_2_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_394() {
    rusty_monitor::set_test_id(394);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 168i32;
    let mut i64_0: i64 = 83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i8_0: i8 = -63i8;
    let mut f32_0: f32 = 82.675518f32;
    let mut i32_2: i32 = 111i32;
    let mut i64_1: i64 = 34i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_3, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5873() {
    rusty_monitor::set_test_id(5873);
    let mut i32_0: i32 = -1i32;
    let mut u32_0: u32 = 41u32;
    let mut u8_0: u8 = 45u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 73u8;
    let mut i32_1: i32 = 42i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_2: i32 = -29i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_3: i32 = 41i32;
    let mut i64_0: i64 = -37i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_3);
    let mut i32_4: i32 = -43i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_3);
    let mut i64_1: i64 = -28i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_5: i32 = -44i32;
    let mut i64_2: i64 = 25i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_5);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut u8_4: u8 = crate::util::weeks_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3006() {
    rusty_monitor::set_test_id(3006);
    let mut u16_0: u16 = 92u16;
    let mut i32_0: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u16_2: u16 = 44u16;
    let mut i32_2: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3660() {
    rusty_monitor::set_test_id(3660);
    let mut i128_0: i128 = 171i128;
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i64_0: i64 = -120i64;
    let mut i32_1: i32 = 101i32;
    let mut u32_1: u32 = 56u32;
    let mut i32_2: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_1);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6738() {
    rusty_monitor::set_test_id(6738);
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut u16_0: u16 = 5u16;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 97u8;
    let mut u8_3: u8 = 1u8;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 28i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_2: i32 = -119i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_1, u8_3, u8_2, u8_1, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7682() {
    rusty_monitor::set_test_id(7682);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut f64_0: f64 = -0.245811f64;
    let mut i64_0: i64 = -84i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut u16_0: u16 = 33u16;
    let mut i32_0: i32 = 89i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = -70i8;
    let mut i8_2: i8 = -22i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = -1i8;
    let mut i8_5: i8 = -18i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = -192i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = 48i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = -152i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_2);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_2: i64 = 74i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut i32_3: i32 = 38i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_4);
    let mut i32_4: i32 = 7i32;
    let mut i64_3: i64 = 5i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i32_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_4: i64 = -101i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, u32_1);
    let mut i64_5: i64 = -38i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_5: i32 = 184i32;
    let mut i64_6: i64 = -4i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_5);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_11);
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(duration_9, duration_8);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_1: f64 = std::ops::Div::div(duration_12, duration_6);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_3);
    let mut duration_13: crate::duration::Duration = std::default::Default::default();
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut month_0: month::Month = crate::month::Month::March;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5210() {
    rusty_monitor::set_test_id(5210);
    let mut i64_0: i64 = -203i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i64_1: i64 = -120i64;
    let mut i32_2: i32 = 101i32;
    let mut u32_0: u32 = 56u32;
    let mut i64_2: i64 = -67i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u32_0);
    let mut i64_3: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_4, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut f64_0: f64 = 30.076607f64;
    let mut i64_0: i64 = -38i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 16u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 38u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 72u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = -78i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -47i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i16_0: i16 = -58i16;
    let mut f64_1: f64 = -5.519969f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i16_0);
    let mut u16_0: u16 = 61u16;
    let mut i32_1: i32 = 82i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3064() {
    rusty_monitor::set_test_id(3064);
    let mut i32_0: i32 = 32i32;
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_1: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = -59i32;
    let mut i64_1: i64 = 61i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_2, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut duration_5_ref_0: &std::time::Duration = &mut duration_5;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_3: i32 = 165i32;
    let mut i64_2: i64 = 83i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_3, padding: padding_1};
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_6);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i8_0: i8 = -63i8;
    let mut i32_4: i32 = 111i32;
    let mut i64_3: i64 = 34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_8_ref_0, duration_5_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_0);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4828() {
    rusty_monitor::set_test_id(4828);
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    let mut i32_1: i32 = 38i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut i64_2: i64 = 5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 65u32;
    let mut i64_3: i64 = -101i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_0);
    let mut i64_4: i64 = -38i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_2: i32 = 184i32;
    let mut i64_5: i64 = -4i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_6, duration_5);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8205() {
    rusty_monitor::set_test_id(8205);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3_ref_0: &crate::instant::Instant = &mut instant_3;
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 84i8;
    let mut i8_1: i8 = -55i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_3: i8 = 44i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 70u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 25.827337f64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = -110i32;
    let mut i64_0: i64 = -9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut instant_4_ref_0: &crate::instant::Instant = &mut instant_4;
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1895() {
    rusty_monitor::set_test_id(1895);
    let mut i64_0: i64 = 79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 26i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -75i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut f32_0: f32 = -6.602946f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -59i32;
    let mut i64_2: i64 = 61i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut duration_5_ref_0: &std::time::Duration = &mut duration_5;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 168i32;
    let mut i64_3: i64 = 83i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_2, padding: padding_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i8_0: i8 = -63i8;
    let mut f32_1: f32 = 82.675518f32;
    let mut i32_3: i32 = 111i32;
    let mut i64_4: i64 = 34i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_9, i8_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_8_ref_0, duration_5_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_926() {
    rusty_monitor::set_test_id(926);
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    let mut i32_1: i32 = 38i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i32_2: i32 = 7i32;
    let mut i64_2: i64 = 5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_3: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_3: i32 = 184i32;
    let mut i64_4: i64 = -4i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_7);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5698() {
    rusty_monitor::set_test_id(5698);
    let mut i32_0: i32 = 130i32;
    let mut i32_1: i32 = 48i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u16_0: u16 = 36u16;
    let mut i32_2: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f32_0);
    let mut i32_3: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_1: i64 = -120i64;
    let mut i32_4: i32 = 101i32;
    let mut u32_1: u32 = 56u32;
    let mut i32_5: i32 = -109i32;
    let mut i64_2: i64 = -67i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_5);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u32_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_3: i64 = -135i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_8: crate::duration::Duration = std::ops::Add::add(duration_7, duration_6);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_8, i32_4);
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_2);
    let mut i32_6: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut i8_0: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5267() {
    rusty_monitor::set_test_id(5267);
    let mut i32_0: i32 = 22i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 88u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -82i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut u16_0: u16 = 92u16;
    let mut i32_2: i32 = -197i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_1: u32 = 49u32;
    let mut u8_3: u8 = 98u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 75u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 98u16;
    let mut i32_3: i32 = 99i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_2);
    let mut u16_2: u16 = 44u16;
    let mut i32_4: i32 = 34i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut u32_2: u32 = 86u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 22u8;
    let mut u8_8: u8 = 17u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u32_3: u32 = 33u32;
    let mut u8_9: u8 = 3u8;
    let mut u8_10: u8 = 69u8;
    let mut u8_11: u8 = 41u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_3);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_2};
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_3);
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_4);
    let mut u8_12: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    let mut u8_13: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1338() {
    rusty_monitor::set_test_id(1338);
    let mut f64_0: f64 = 197.045309f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -45i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i32_1: i32 = -58i32;
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = -83i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_1: f64 = -223.591359f64;
    let mut i64_0: i64 = -191i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_1);
    let mut u32_1: u32 = 80u32;
    let mut u8_3: u8 = 73u8;
    let mut u8_4: u8 = 70u8;
    let mut u8_5: u8 = 89u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = -11i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut i64_2: i64 = 202i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u32_2: u32 = 48u32;
    let mut u8_6: u8 = 70u8;
    let mut u8_7: u8 = 97u8;
    let mut u8_8: u8 = 14u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut u16_0: u16 = 81u16;
    let mut i32_2: i32 = -71i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_3};
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_2, i32_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3525() {
    rusty_monitor::set_test_id(3525);
    let mut u16_0: u16 = 92u16;
    let mut i32_0: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut u16_2: u16 = 44u16;
    let mut i32_2: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut u32_0: u32 = 86u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 33u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 69u8;
    let mut u8_5: u8 = 41u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_1);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1947() {
    rusty_monitor::set_test_id(1947);
    let mut u8_0: u8 = 94u8;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -128i32;
    let mut i64_0: i64 = -125i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut i8_0: i8 = 19i8;
    let mut i8_1: i8 = 31i8;
    let mut i8_2: i8 = -57i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = -12i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut u16_0: u16 = 99u16;
    let mut i32_2: i32 = 89i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_1: i64 = 64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u32_0: u32 = 24u32;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 92u8;
    let mut u8_3: u8 = 28u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_0);
    let mut u32_1: u32 = 5u32;
    let mut u8_4: u8 = 49u8;
    let mut u8_5: u8 = 86u8;
    let mut u8_6: u8 = 85u8;
    let mut u16_1: u16 = 13u16;
    let mut i32_3: i32 = -21i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut u32_2: u32 = 45u32;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 98u8;
    let mut u8_9: u8 = 97u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i8_3: i8 = -39i8;
    let mut i8_4: i8 = -83i8;
    let mut i8_5: i8 = 62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = -223.591359f64;
    let mut i64_2: i64 = -191i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f64_0);
    let mut u32_3: u32 = 80u32;
    let mut u8_10: u8 = 73u8;
    let mut u8_11: u8 = 70u8;
    let mut u8_12: u8 = 89u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_6, time_3);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i64_3: i64 = -11i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut i64_4: i64 = 202i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_8, duration_7);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_6);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut u32_4: u32 = 48u32;
    let mut u8_13: u8 = 70u8;
    let mut u8_14: u8 = 97u8;
    let mut u8_15: u8 = 14u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_15, u8_14, u8_13, u32_4);
    let mut u16_2: u16 = 81u16;
    let mut i32_4: i32 = -71i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_6, time: time_5};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_4, u8_6, u8_5, u8_4, u32_1);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_570() {
    rusty_monitor::set_test_id(570);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i8_0: i8 = 16i8;
    let mut i8_1: i8 = -4i8;
    let mut i8_2: i8 = -70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = -46i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut f64_0: f64 = 117.095041f64;
    let mut i64_1: i64 = -37i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f64_0);
    let mut i32_1: i32 = -14i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut month_0: month::Month = crate::month::Month::January;
    let mut u16_0: u16 = 13u16;
    let mut i32_2: i32 = -21i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = -39i8;
    let mut i8_4: i8 = -83i8;
    let mut i8_5: i8 = 62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = -223.591359f64;
    let mut i64_2: i64 = -191i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f64_1);
    let mut u32_1: u32 = 80u32;
    let mut u8_3: u8 = 73u8;
    let mut u8_4: u8 = 70u8;
    let mut u8_5: u8 = 89u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_1);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i64_3: i64 = -11i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut i64_4: i64 = 202i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_544() {
    rusty_monitor::set_test_id(544);
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7559() {
    rusty_monitor::set_test_id(7559);
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut i32_2: i32 = 28i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i64_1: i64 = 116i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::January;
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4076() {
    rusty_monitor::set_test_id(4076);
    let mut i32_0: i32 = -148i32;
    let mut i64_0: i64 = -115i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i32_2: i32 = 126i32;
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 48i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_3: i32 = -152i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_3: i64 = 74i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut i32_4: i32 = 38i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut i64_4: i64 = 7i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_1, i32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_5: i64 = -101i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, u32_1);
    let mut i64_6: i64 = -38i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_5: i32 = 184i32;
    let mut i64_7: i64 = -4i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_5);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_10, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_11);
    let mut duration_12: crate::duration::Duration = std::ops::Sub::sub(duration_9, duration_8);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_12, duration_6);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_3);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7268() {
    rusty_monitor::set_test_id(7268);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut u8_0: u8 = 12u8;
    let mut i32_0: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_1: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut u16_0: u16 = 5u16;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 97u8;
    let mut u8_3: u8 = 1u8;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_2: i32 = 28i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_3: i32 = -119i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_3, u8_3, u8_2, u8_1, u16_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8084() {
    rusty_monitor::set_test_id(8084);
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 64i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -106i8;
    let mut i8_4: i8 = -126i8;
    let mut i8_5: i8 = 70i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i16_0: i16 = 29i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut i64_0: i64 = 112i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i64_1: i64 = -113i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i8_6: i8 = 76i8;
    let mut i8_7: i8 = 65i8;
    let mut i8_8: i8 = -74i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 43u32;
    let mut u8_3: u8 = 99u8;
    let mut u8_4: u8 = 38u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_9: i8 = -102i8;
    let mut i8_10: i8 = -2i8;
    let mut i8_11: i8 = 97i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_0: i32 = 148i32;
    let mut i32_1: i32 = -40i32;
    let mut i64_2: i64 = -96i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3448() {
    rusty_monitor::set_test_id(3448);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 57u8;
    let mut u8_2: u8 = 49u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut u16_0: u16 = 46u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut i32_1: i32 = -74i32;
    let mut i64_0: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Neg::neg(duration_5);
    let mut i32_2: i32 = 41i32;
    let mut i64_1: i64 = -37i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut i32_3: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_2);
    let mut i64_2: i64 = -28i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i32_4: i32 = -44i32;
    let mut i64_3: i64 = 25i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_10: crate::duration::Duration = std::ops::Add::add(duration_9, duration_8);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_3);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_7, duration_6);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4937() {
    rusty_monitor::set_test_id(4937);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_1: i32 = -131i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i64_0: i64 = -120i64;
    let mut i32_2: i32 = 101i32;
    let mut u32_0: u32 = 56u32;
    let mut i32_3: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_3);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_6, i32_2);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_4: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2361() {
    rusty_monitor::set_test_id(2361);
    let mut f32_0: f32 = 53.031445f32;
    let mut i64_0: i64 = -131i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut f64_0: f64 = -2.471452f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = -100i128;
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_1: f32 = -58.454827f32;
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, f32_1);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i64_1: i64 = -120i64;
    let mut i32_2: i32 = -109i32;
    let mut i64_2: i64 = -67i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut i32_3: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_5, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3437() {
    rusty_monitor::set_test_id(3437);
    let mut i128_0: i128 = -159i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = 116i8;
    let mut i8_2: i8 = -109i8;
    let mut u16_0: u16 = 92u16;
    let mut i32_0: i32 = -197i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 75u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 98u16;
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u16_2: u16 = 44u16;
    let mut i32_2: i32 = 34i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut u32_1: u32 = 86u32;
    let mut u8_3: u8 = 74u8;
    let mut u8_4: u8 = 22u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i16_0: i16 = -21i16;
    let mut i64_0: i64 = 116i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_weeks(duration_2);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_2);
    let mut i128_1: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5866() {
    rusty_monitor::set_test_id(5866);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i64_0: i64 = -120i64;
    let mut u32_1: u32 = 56u32;
    let mut i32_1: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_2: i64 = -135i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::default::Default::default();
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_2: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3909() {
    rusty_monitor::set_test_id(3909);
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut i32_0: i32 = 215i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut i64_1: i64 = 96i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 16u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -11.042182f32;
    let mut i64_2: i64 = 45i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut u16_0: u16 = 95u16;
    let mut i32_1: i32 = 40i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut u8_3: u8 = 12u8;
    let mut i32_2: i32 = 84i32;
    let mut i64_3: i64 = 94i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u8_3);
    let mut i32_3: i32 = -82i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_5);
    let mut month_0: month::Month = crate::date::Date::month(date_5);
    let mut f64_0: f64 = 109.043940f64;
    let mut u16_1: u16 = 5u16;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 97u8;
    let mut u8_6: u8 = 1u8;
    let mut f32_1: f32 = -46.329670f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i32_4: i32 = 28i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_7: crate::date::Date = crate::date::Date::saturating_sub(date_6, duration_6);
    let mut i64_4: i64 = 116i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_5: i32 = -119i32;
    let mut i64_5: i64 = -174i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_5);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_7, u8_6, u8_5, u8_4, u16_1);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_8, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_2);
    let mut tuple_1: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3553() {
    rusty_monitor::set_test_id(3553);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = -152i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i16_0: i16 = 168i16;
    let mut i64_1: i64 = 74i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    let mut i32_1: i32 = 38i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i32_2: i32 = 7i32;
    let mut i64_2: i64 = 5i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 65u32;
    let mut i64_3: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u32_1);
    let mut i64_4: i64 = -38i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut f32_0: f32 = 30.112788f32;
    let mut i32_3: i32 = 184i32;
    let mut i64_5: i64 = -4i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_3);
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_8, f32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_9);
    let mut duration_10: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut instant_2: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_10, duration_4);
    let mut u16_0: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_1);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3146() {
    rusty_monitor::set_test_id(3146);
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = -120i64;
    let mut i64_1: i64 = -135i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_2: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2098() {
    rusty_monitor::set_test_id(2098);
    let mut f64_0: f64 = -98.854238f64;
    let mut i64_0: i64 = -165i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut padding_0_ref_0: &time::Padding = &mut padding_0;
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    let mut u16_0: u16 = 36u16;
    let mut i32_0: i32 = -36i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut u32_0: u32 = 83u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 69u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = -58.454827f32;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f32_0);
    let mut i32_1: i32 = -128i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut u32_1: u32 = 56u32;
    let mut i32_2: i32 = -109i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u32_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut i64_2: i64 = -135i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_8: crate::duration::Duration = std::ops::Add::add(duration_7, duration_6);
    let mut padding_1_ref_0: &time::Padding = &mut padding_1;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_8);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5537() {
    rusty_monitor::set_test_id(5537);
    let mut i32_0: i32 = 8i32;
    let mut u8_0: u8 = 12u8;
    let mut i32_1: i32 = 84i32;
    let mut i64_0: i64 = 94i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut f64_0: f64 = 109.043940f64;
    let mut f32_0: f32 = -46.329670f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_1: i64 = 116i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = -106i32;
    let mut i64_2: i64 = -174i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::January;
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6937() {
    rusty_monitor::set_test_id(6937);
    let mut i64_0: i64 = -78i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 26u16;
    let mut i32_0: i32 = 69i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut f64_0: f64 = -71.892653f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = 117i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut u16_1: u16 = 46u16;
    let mut i32_1: i32 = -29i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut i32_2: i32 = -74i32;
    let mut i64_1: i64 = -101i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_6: crate::duration::Duration = std::ops::Neg::neg(duration_5);
    let mut i32_3: i32 = 41i32;
    let mut i64_2: i64 = -37i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut i32_4: i32 = -43i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_4);
    let mut i64_3: i64 = -28i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_5: i32 = -44i32;
    let mut i64_4: i64 = 25i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut duration_10: crate::duration::Duration = std::ops::Add::add(duration_9, duration_8);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_7, duration_6);
    let mut u16_2: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_2);
    let mut u8_1: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7579() {
    rusty_monitor::set_test_id(7579);
    let mut u8_0: u8 = 29u8;
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_1: u8 = 37u8;
    let mut u8_2: u8 = 69u8;
    let mut u8_3: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = -99i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = -25i32;
    let mut i64_2: i64 = -30i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut i64_3: i64 = 101i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut f32_0: f32 = 208.936735f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, f32_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_5);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_648() {
    rusty_monitor::set_test_id(648);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut f64_0: f64 = -136.451634f64;
    let mut i32_0: i32 = 26i32;
    let mut i64_0: i64 = -126i64;
    let mut u32_0: u32 = 45u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 40u8;
    let mut u8_2: u8 = 97u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 64i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i16_0: i16 = 29i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut i64_1: i64 = 112i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -113i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_4, f64_0);
    panic!("From RustyUnit with love");
}
}