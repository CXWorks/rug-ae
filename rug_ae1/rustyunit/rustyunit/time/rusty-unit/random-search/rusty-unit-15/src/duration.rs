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
fn rusty_test_3375() {
    rusty_monitor::set_test_id(3375);
    let mut i32_0: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_0: u32 = 64u32;
    let mut u8_0: u8 = 18u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 39u8;
    let mut u32_1: u32 = 35u32;
    let mut i64_0: i64 = -19i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_1);
    let mut i32_1: i32 = 89i32;
    let mut i64_1: i64 = -110i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3394() {
    rusty_monitor::set_test_id(3394);
    let mut i32_0: i32 = -138i32;
    let mut i64_0: i64 = 72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 47i8;
    let mut i8_1: i8 = 16i8;
    let mut i8_2: i8 = -32i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = 87i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = 106i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i128_1: i128 = -4i128;
    let mut i32_2: i32 = -154i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut u8_0: u8 = 9u8;
    let mut f32_0: f32 = 7.445471f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_4);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_209() {
    rusty_monitor::set_test_id(209);
    let mut i32_0: i32 = -95i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i16_0: i16 = -88i16;
    let mut i64_0: i64 = 83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i32_1: i32 = -172i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut i16_1: i16 = -80i16;
    let mut u8_0: u8 = 2u8;
    let mut i64_1: i64 = 126i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = 13i64;
    let mut i32_2: i32 = -28i32;
    let mut i64_3: i64 = -28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = -119i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_5, i16_1);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_7_ref_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut u8_1: u8 = crate::date::Date::iso_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4918() {
    rusty_monitor::set_test_id(4918);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 79u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i32_1: i32 = -59i32;
    let mut i64_0: i64 = -93i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut f32_0: f32 = 143.718601f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_1_ref_0, duration_0_ref_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1346() {
    rusty_monitor::set_test_id(1346);
    let mut i32_0: i32 = -94i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_1: i32 = -10i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_3);
    let mut i64_0: i64 = -35i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_2);
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_add(time_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1688() {
    rusty_monitor::set_test_id(1688);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 53u8;
    let mut u8_2: u8 = 22u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 16i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -108.222374f32;
    let mut i32_0: i32 = -130i32;
    let mut i64_0: i64 = -175i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_0);
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_944() {
    rusty_monitor::set_test_id(944);
    let mut i8_0: i8 = 9i8;
    let mut i8_1: i8 = -61i8;
    let mut i8_2: i8 = 26i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -153.325138f32;
    let mut i64_0: i64 = -8i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u32_0: u32 = 75u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 46u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 239i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3023() {
    rusty_monitor::set_test_id(3023);
    let mut i8_0: i8 = -118i8;
    let mut f32_0: f32 = 40.257102f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u16_0: u16 = 72u16;
    let mut i32_0: i32 = 3i32;
    let mut i64_0: i64 = 61i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 126i32;
    let mut i64_1: i64 = 42i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_1);
    let mut f64_0: f64 = -167.968424f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_4_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut i64_0: i64 = 221i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 22i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -60i32;
    let mut i64_2: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_0, padding: padding_0};
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i8_0: i8 = -36i8;
    let mut i8_1: i8 = 54i8;
    let mut i8_2: i8 = -49i8;
    let mut f64_0: f64 = 116.959072f64;
    let mut i32_1: i32 = -105i32;
    let mut i64_3: i64 = -190i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, f64_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_3_ref_0, duration_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut month_2: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2148() {
    rusty_monitor::set_test_id(2148);
    let mut i64_0: i64 = 37i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 87u16;
    let mut i32_0: i32 = 29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u8_0: u8 = 94u8;
    let mut i64_1: i64 = 89i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    let mut i8_0: i8 = 17i8;
    let mut i8_1: i8 = -80i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 45u32;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 5u8;
    let mut u8_3: u8 = 61u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut u32_1: u32 = 34u32;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 99u8;
    let mut u8_6: u8 = 69u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_1: i32 = -21i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut u32_2: u32 = 58u32;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 44u8;
    let mut u8_9: u8 = 25u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i32_2: i32 = -116i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut u32_3: u32 = 40u32;
    let mut u8_10: u8 = 95u8;
    let mut u8_11: u8 = 70u8;
    let mut u8_12: u8 = 96u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut time_3_ref_0: &mut crate::time::Time = &mut time_3;
    let mut i64_2: i64 = 21i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_3: i32 = 31i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_3);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut u8_13: u8 = crate::date::Date::day(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3079() {
    rusty_monitor::set_test_id(3079);
    let mut i8_0: i8 = -25i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut u32_0: u32 = 60u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = -4.998806f64;
    let mut i64_0: i64 = 10i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i16_0: i16 = 62i16;
    let mut i64_1: i64 = -227i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut f64_1: f64 = -26.026680f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut padding_2: duration::Padding = std::clone::Clone::clone(padding_1_ref_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut padding_2_ref_0: &duration::Padding = &mut padding_2;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_2_ref_0, padding_0_ref_0);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, i16_0);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_3, f64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4887() {
    rusty_monitor::set_test_id(4887);
    let mut i8_0: i8 = 11i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = -92i8;
    let mut i8_3: i8 = 111i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i32_0: i32 = 239i32;
    let mut i64_0: i64 = 21i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_4: i8 = -128i8;
    let mut i8_5: i8 = -35i8;
    let mut i8_6: i8 = -21i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_351() {
    rusty_monitor::set_test_id(351);
    let mut i64_0: i64 = -21i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i8_0: i8 = 48i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 45u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -18i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut f64_0: f64 = -44.701391f64;
    let mut i64_1: i64 = 15i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i64_2: i64 = -142i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_3_ref_0);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3995() {
    rusty_monitor::set_test_id(3995);
    let mut i8_0: i8 = 53i8;
    let mut i8_1: i8 = 101i8;
    let mut i8_2: i8 = -28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 73i8;
    let mut i8_4: i8 = -12i8;
    let mut i8_5: i8 = -18i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 124i8;
    let mut i8_7: i8 = -63i8;
    let mut i8_8: i8 = 115i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 14u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 56u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -1i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_1: u32 = 72u32;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_1);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -59i32;
    let mut i64_1: i64 = 110i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut i64_2: i64 = -47i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut i32_2: i32 = 73i32;
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_2, month_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3973() {
    rusty_monitor::set_test_id(3973);
    let mut u8_0: u8 = 11u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 35u8;
    let mut u32_0: u32 = 38u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut u16_0: u16 = 18u16;
    let mut i32_0: i32 = -154i32;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut padding_2: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    let mut u8_6: u8 = crate::time::Time::hour(time_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3573() {
    rusty_monitor::set_test_id(3573);
    let mut i8_0: i8 = 56i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i8_1: i8 = 126i8;
    let mut i8_2: i8 = -78i8;
    let mut i8_3: i8 = -64i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i32_0: i32 = -43i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_1: i32 = 18i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i64_1: i64 = -44i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut i16_0: i16 = 58i16;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_4);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::day(offsetdatetime_1);
    let mut f64_0: f64 = std::ops::Div::div(duration_7, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2639() {
    rusty_monitor::set_test_id(2639);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut u16_0: u16 = 50u16;
    let mut i32_0: i32 = -17i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 44u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 60u8;
    let mut f32_0: f32 = 17.236662f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Neg::neg(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = crate::date::Date::to_julian_day(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut i32_0: i32 = -96i32;
    let mut i64_0: i64 = 30i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut i128_0: i128 = 35i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_1: i32 = -15i32;
    let mut i64_1: i64 = 28i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 41i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_2);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut bool_1: bool = std::cmp::PartialEq::eq(duration_3_ref_0, duration_1_ref_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3134() {
    rusty_monitor::set_test_id(3134);
    let mut i64_0: i64 = -111i64;
    let mut u32_0: u32 = 59u32;
    let mut u8_0: u8 = 39u8;
    let mut u8_1: u8 = 46u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 76i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut f32_0: f32 = -28.252138f32;
    let mut i64_2: i64 = 150i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4509() {
    rusty_monitor::set_test_id(4509);
    let mut i32_0: i32 = 44i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut f32_0: f32 = -74.804487f32;
    let mut i64_0: i64 = 39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u16_0: u16 = 91u16;
    let mut i32_1: i32 = -16i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i8_0: i8 = -75i8;
    let mut i128_0: i128 = 76i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i8_0);
    let mut i64_1: i64 = 105i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_0: u8 = crate::date::Date::day(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3935() {
    rusty_monitor::set_test_id(3935);
    let mut i64_0: i64 = 127i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut f32_0: f32 = -94.400995f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 64u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut f32_1: f32 = 250.212936f32;
    let mut i128_0: i128 = 55i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, f32_1);
    let mut u16_1: u16 = 72u16;
    let mut i32_1: i32 = 116i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut u8_0: u8 = crate::date::Date::day(date_2);
    let mut u8_1: u8 = crate::primitive_date_time::PrimitiveDateTime::sunday_based_week(primitivedatetime_0);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3420() {
    rusty_monitor::set_test_id(3420);
    let mut i16_0: i16 = 18i16;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 91i32;
    let mut i64_0: i64 = -8i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 44u16;
    let mut i32_1: i32 = -54i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1607() {
    rusty_monitor::set_test_id(1607);
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 36u8;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut f32_0: f32 = 40.885394f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_0: i64 = 31i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut i128_0: i128 = 46i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = 65i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_8);
    let mut u32_0: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_1);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_3);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_632() {
    rusty_monitor::set_test_id(632);
    let mut i32_0: i32 = -155i32;
    let mut f32_0: f32 = 27.487323f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut i32_1: i32 = -70i32;
    let mut i64_0: i64 = 38i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = -49i8;
    let mut i8_1: i8 = -1i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 89i8;
    let mut i8_4: i8 = 82i8;
    let mut i8_5: i8 = 32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = -86i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2449() {
    rusty_monitor::set_test_id(2449);
    let mut i8_0: i8 = -39i8;
    let mut i8_1: i8 = -73i8;
    let mut i8_2: i8 = -17i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 15i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut u16_0: u16 = 43u16;
    let mut i16_0: i16 = -61i16;
    let mut f32_0: f32 = -9.507970f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u16_0);
    let mut u32_0: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3452() {
    rusty_monitor::set_test_id(3452);
    let mut i64_0: i64 = 7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i128_0: i128 = -53i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u32_0: u32 = 59u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 129.134130f64;
    let mut i128_1: i128 = -94i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut i8_0: i8 = -20i8;
    let mut i8_1: i8 = 62i8;
    let mut i8_2: i8 = 122i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = -180i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_1: u32 = 61u32;
    let mut u8_3: u8 = 83u8;
    let mut u8_4: u8 = 45u8;
    let mut u8_5: u8 = 98u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = -172i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Neg::neg(duration_5);
    let mut i32_0: i32 = 6i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut i8_3: i8 = 46i8;
    let mut i32_1: i32 = 43i32;
    let mut i64_3: i64 = -7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, i8_3);
    let mut f64_1: f64 = -11.677051f64;
    let mut i32_2: i32 = -44i32;
    let mut i64_4: i64 = 96i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, f64_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_10, duration_8);
    let mut duration_11: crate::duration::Duration = std::option::Option::unwrap(option_0);
    let mut i64_5: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_1);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_1_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2795() {
    rusty_monitor::set_test_id(2795);
    let mut f64_0: f64 = -192.702872f64;
    let mut f64_1: f64 = -177.161428f64;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -75i32;
    let mut i64_0: i64 = -98i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_1);
    let mut i64_1: i64 = -56i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut u8_0: u8 = 43u8;
    let mut i32_1: i32 = 46i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_2);
    let mut u8_1: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut date_0: crate::date::Date = std::result::Result::unwrap(result_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4558() {
    rusty_monitor::set_test_id(4558);
    let mut i8_0: i8 = 10i8;
    let mut i8_1: i8 = 66i8;
    let mut i8_2: i8 = -37i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = 36i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i8_3: i8 = 92i8;
    let mut i64_1: i64 = 10i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_3);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_2);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_442() {
    rusty_monitor::set_test_id(442);
    let mut u32_0: u32 = 64u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 26u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -58i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 170i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u16_0: u16 = 32u16;
    let mut i32_0: i32 = 190i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut u8_3: u8 = 79u8;
    let mut i8_0: i8 = -73i8;
    let mut i64_2: i64 = -29i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut i64_3: i64 = 77i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_1: i32 = -52i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_3, u8_3);
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_817() {
    rusty_monitor::set_test_id(817);
    let mut u16_0: u16 = 97u16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut i8_0: i8 = 34i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = -57i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 46i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut primitivedatetime_1_ref_0: &mut crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3203() {
    rusty_monitor::set_test_id(3203);
    let mut u16_0: u16 = 38u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 99i32;
    let mut i64_0: i64 = 78i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut i8_0: i8 = -10i8;
    let mut i8_1: i8 = -111i8;
    let mut i8_2: i8 = -10i8;
    let mut i32_1: i32 = 45i32;
    let mut f64_0: f64 = 94.725693f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i8_3: i8 = 118i8;
    let mut i8_4: i8 = -45i8;
    let mut i8_5: i8 = 8i8;
    let mut u16_1: u16 = 86u16;
    let mut i32_2: i32 = -31i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_350() {
    rusty_monitor::set_test_id(350);
    let mut i32_0: i32 = 80i32;
    let mut i64_0: i64 = -39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut u16_0: u16 = 47u16;
    let mut i64_1: i64 = -17i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = 60i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = -1i8;
    let mut i8_1: i8 = 56i8;
    let mut i8_2: i8 = 73i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = std::result::Result::unwrap(result_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_3_ref_0, duration_1_ref_0);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_648() {
    rusty_monitor::set_test_id(648);
    let mut f64_0: f64 = -76.012980f64;
    let mut i128_0: i128 = -41i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut f32_0: f32 = 142.490869f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i128_1: i128 = -90i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut u8_0: u8 = 21u8;
    let mut i64_0: i64 = -149i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, u8_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_5);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_8, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1919() {
    rusty_monitor::set_test_id(1919);
    let mut u16_0: u16 = 56u16;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 32u8;
    let mut i64_0: i64 = 230i64;
    let mut f64_0: f64 = 66.381438f64;
    let mut i128_0: i128 = 119i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i128_1: i128 = 66i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_0: i32 = 85i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3352() {
    rusty_monitor::set_test_id(3352);
    let mut f64_0: f64 = 31.104805f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = -234i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut f64_1: f64 = 27.562650f64;
    let mut i64_0: i64 = -8i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i64_1: i64 = 65i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1_ref_0: &mut crate::instant::Instant = &mut instant_1;
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3433() {
    rusty_monitor::set_test_id(3433);
    let mut i64_0: i64 = 101i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i128_0: i128 = 20i128;
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut bool_0: bool = false;
    let mut i64_2: i64 = 178i64;
    let mut i64_3: i64 = 70i64;
    let mut i64_4: i64 = -38i64;
    let mut str_0: &str = "Fs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4911() {
    rusty_monitor::set_test_id(4911);
    let mut u16_0: u16 = 99u16;
    let mut i32_0: i32 = -170i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 49u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = 77i8;
    let mut i8_2: i8 = -104i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i16_0: i16 = 201i16;
    let mut i64_0: i64 = 188i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut u8_3: u8 = 24u8;
    let mut i64_1: i64 = 14i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_4);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i16_1: i16 = -40i16;
    let mut i128_0: i128 = 202i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_3: i8 = 86i8;
    let mut i32_1: i32 = -33i32;
    let mut i64_2: i64 = 19i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i8_3);
    let mut i32_2: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_4, date_2);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i64_3: i64 = -140i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_3: i32 = 51i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_2};
    let mut i8_4: i8 = -36i8;
    let mut i64_4: i64 = -98i64;
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut u32_1: u32 = 49u32;
    let mut u8_4: u8 = 41u8;
    let mut u8_5: u8 = 33u8;
    let mut u8_6: u8 = 83u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_4: i32 = -69i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_2);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut duration_10: crate::duration::Duration = std::default::Default::default();
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut duration_11: crate::duration::Duration = std::ops::Div::div(duration_5, i16_1);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_9);
    let mut u8_7: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2242() {
    rusty_monitor::set_test_id(2242);
    let mut i32_0: i32 = 78i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 97u32;
    let mut u8_0: u8 = 84u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -31i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut i16_0: i16 = -31i16;
    let mut i64_0: i64 = 65i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i16_1: i16 = crate::duration::Duration::subsec_milliseconds(duration_1);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3901() {
    rusty_monitor::set_test_id(3901);
    let mut u16_0: u16 = 17u16;
    let mut i32_0: i32 = 62i32;
    let mut i32_1: i32 = -42i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut f32_0: f32 = -58.926298f32;
    let mut i64_0: i64 = -20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 105i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 77u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 77u16;
    let mut i32_2: i32 = -83i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut duration_2: crate::duration::Duration = std::clone::Clone::clone(duration_0_ref_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4011() {
    rusty_monitor::set_test_id(4011);
    let mut u16_0: u16 = 62u16;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 88u8;
    let mut u8_2: u8 = 68u8;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut u16_1: u16 = 36u16;
    let mut u32_0: u32 = 44u32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut u32_1: u32 = 50u32;
    let mut u8_3: u8 = 64u8;
    let mut u8_4: u8 = 94u8;
    let mut u8_5: u8 = 25u8;
    let mut u32_2: u32 = 78u32;
    let mut u8_6: u8 = 71u8;
    let mut u8_7: u8 = 1u8;
    let mut u8_8: u8 = 14u8;
    let mut i32_0: i32 = -190i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 45i32;
    let mut i32_2: i32 = 88i32;
    let mut i64_0: i64 = 35i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_1);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_8, u8_7, u8_6, u32_2);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut padding_2: duration::Padding = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_1, u16_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3325() {
    rusty_monitor::set_test_id(3325);
    let mut i64_0: i64 = 150i64;
    let mut f32_0: f32 = -51.503337f32;
    let mut i32_0: i32 = -69i32;
    let mut i64_1: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u32_0: u32 = 21u32;
    let mut u8_0: u8 = 14u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 47u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 82u16;
    let mut i32_1: i32 = -40i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_1);
    let mut tuple_1: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_hours(duration_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1342() {
    rusty_monitor::set_test_id(1342);
    let mut f32_0: f32 = -69.971688f32;
    let mut f32_1: f32 = 148.021977f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut i128_0: i128 = 256i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i16_0: i16 = -63i16;
    let mut i64_0: i64 = 85i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i16_0);
    let mut i32_0: i32 = -147i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_3_ref_0, duration_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4989() {
    rusty_monitor::set_test_id(4989);
    let mut f64_0: f64 = 30.221479f64;
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 33i8;
    let mut i64_0: i64 = -23i64;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = -32i64;
    let mut i64_2: i64 = -49i64;
    let mut i64_3: i64 = -81i64;
    let mut str_0: &str = "QrA215";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2703() {
    rusty_monitor::set_test_id(2703);
    let mut i8_0: i8 = -25i8;
    let mut i8_1: i8 = -37i8;
    let mut i8_2: i8 = 18i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -103i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut i8_3: i8 = -70i8;
    let mut i128_0: i128 = -93i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_3);
    let mut f32_0: f32 = 17.679361f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = 47i64;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_hours(duration_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = std::result::Result::unwrap(result_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_nanoseconds(duration_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut i32_0: i32 = 64i32;
    let mut i64_0: i64 = -124i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = -92i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut f64_0: f64 = 150.631292f64;
    let mut i32_1: i32 = 67i32;
    let mut i64_2: i64 = -116i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut u8_0: u8 = 12u8;
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_0);
    let mut u32_0: u32 = 11u32;
    let mut i64_3: i64 = 68i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u32_0);
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1516() {
    rusty_monitor::set_test_id(1516);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut u32_0: u32 = 5u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 40u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 68u16;
    let mut i32_0: i32 = -110i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_1: u32 = 50u32;
    let mut i64_0: i64 = -197i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut f32_0: f32 = 273.750215f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1255() {
    rusty_monitor::set_test_id(1255);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -47i32;
    let mut i64_0: i64 = 192i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut u16_0: u16 = 8u16;
    let mut i32_1: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i8_0: i8 = 98i8;
    let mut i8_1: i8 = -23i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_2: i32 = 151i32;
    let mut i8_3: i8 = 88i8;
    let mut i8_4: i8 = 94i8;
    let mut i8_5: i8 = 71i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u8_0: u8 = 25u8;
    let mut i64_1: i64 = -154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u8_0);
    let mut u16_1: u16 = 8u16;
    let mut i32_3: i32 = -71i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_4: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_402() {
    rusty_monitor::set_test_id(402);
    let mut i8_0: i8 = -35i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -69.394099f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 61i32;
    let mut i64_0: i64 = 119i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i128_0: i128 = 3i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i128_1: i128 = 47i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_6, duration_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1973() {
    rusty_monitor::set_test_id(1973);
    let mut i8_0: i8 = 39i8;
    let mut i8_1: i8 = 84i8;
    let mut i8_2: i8 = -56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 7u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 71u32;
    let mut u8_3: u8 = 34u8;
    let mut u8_4: u8 = 62u8;
    let mut u8_5: u8 = 37u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -80i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut f64_0: f64 = 170.439596f64;
    let mut i64_0: i64 = 72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_3: i8 = 40i8;
    let mut i64_1: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i8_3);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_3_ref_0, duration_1_ref_0);
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3666() {
    rusty_monitor::set_test_id(3666);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i32_0: i32 = 46i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u16_0: u16 = 48u16;
    let mut i32_1: i32 = 31i32;
    let mut i32_2: i32 = 45i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = -80i32;
    let mut i64_0: i64 = -115i64;
    let mut u16_1: u16 = 42u16;
    let mut i32_4: i32 = 39i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_2);
    let mut date_1_ref_0: &mut crate::date::Date = &mut date_1;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3095() {
    rusty_monitor::set_test_id(3095);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i64_0: i64 = -69i64;
    let mut i64_1: i64 = 46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i32_0: i32 = -121i32;
    let mut u32_0: u32 = 42u32;
    let mut f32_0: f32 = -86.092798f32;
    let mut i64_2: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_836() {
    rusty_monitor::set_test_id(836);
    let mut f32_0: f32 = -116.080402f32;
    let mut i64_0: i64 = -44i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 99i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i8_0: i8 = 80i8;
    let mut i64_2: i64 = 142i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_5);
    let mut u16_0: u16 = 33u16;
    let mut i32_1: i32 = -23i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_1);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2936() {
    rusty_monitor::set_test_id(2936);
    let mut f32_0: f32 = 127.135933f32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 56i32;
    let mut i64_0: i64 = 4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut i32_1: i32 = 147i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i32_2: i32 = -115i32;
    let mut i16_0: i16 = 109i16;
    let mut i128_0: i128 = -59i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut f64_0: f64 = 39.708143f64;
    let mut i128_1: i128 = 91i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_2, month_0);
    let mut month_1: month::Month = crate::date::Date::month(date_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_4_ref_0, duration_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4205() {
    rusty_monitor::set_test_id(4205);
    let mut i8_0: i8 = 56i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = -2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_0: u32 = 80u32;
    let mut u8_0: u8 = 91u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 6u8;
    let mut i32_0: i32 = -112i32;
    let mut i32_1: i32 = 92i32;
    let mut i64_0: i64 = -116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u8_3: u8 = 3u8;
    let mut i64_1: i64 = -118i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_3);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_2);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_0, i32_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1890() {
    rusty_monitor::set_test_id(1890);
    let mut u16_0: u16 = 68u16;
    let mut i64_0: i64 = 41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut i8_0: i8 = 13i8;
    let mut i8_1: i8 = -58i8;
    let mut i8_2: i8 = 100i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -73i8;
    let mut i8_4: i8 = 16i8;
    let mut i8_5: i8 = -63i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 17i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 27u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 70u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_1: u16 = 0u16;
    let mut i32_0: i32 = -29i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_6: i8 = -104i8;
    let mut i8_7: i8 = -104i8;
    let mut i8_8: i8 = 20i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut utcoffset_2_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_2;
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_sub(offsetdatetime_1, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3333() {
    rusty_monitor::set_test_id(3333);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 96u16;
    let mut i32_0: i32 = -12i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -14i32;
    let mut i64_1: i64 = 87i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut f64_0: f64 = -82.834405f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    let mut i32_2: i32 = -133i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_0);
    let mut u32_0: u32 = crate::primitive_date_time::PrimitiveDateTime::nanosecond(primitivedatetime_2);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_517() {
    rusty_monitor::set_test_id(517);
    let mut i16_0: i16 = -35i16;
    let mut u16_0: u16 = 75u16;
    let mut i128_0: i128 = 25i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = -243i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -113i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Neg::neg(duration_3);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 21i32;
    let mut i64_2: i64 = -154i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_0, padding: padding_0};
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_5);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_2);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_monday(weekday_1);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_844() {
    rusty_monitor::set_test_id(844);
    let mut i32_0: i32 = -20i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 58u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 40u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 100i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut u32_1: u32 = 51u32;
    let mut i64_0: i64 = 36i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = -20i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i16_0: i16 = -69i16;
    let mut f64_0: f64 = 54.967246f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i16_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_3_ref_0, duration_1_ref_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_0, u32_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::primitive_date_time::PrimitiveDateTime::to_iso_week_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3286() {
    rusty_monitor::set_test_id(3286);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -91i64;
    let mut i64_1: i64 = 19i64;
    let mut i64_2: i64 = -94i64;
    let mut str_0: &str = "gYmJe4nm4FA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i32_0: i32 = 70i32;
    let mut i64_3: i64 = 83i64;
    let mut i64_4: i64 = 55i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut f64_0: f64 = 164.186768f64;
    let mut i64_5: i64 = -115i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f64_0);
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    let mut i64_6: i64 = crate::duration::Duration::whole_minutes(duration_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_0);
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4529() {
    rusty_monitor::set_test_id(4529);
    let mut u16_0: u16 = 70u16;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 52u8;
    let mut i32_0: i32 = -139i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_3: u8 = 76u8;
    let mut u8_4: u8 = 13u8;
    let mut u8_5: u8 = 3u8;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut i32_1: i32 = 21i32;
    let mut i32_2: i32 = 29i32;
    let mut i64_0: i64 = -162i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i32_2);
    let mut i64_1: i64 = 261i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut f32_0: f32 = -360.557848f32;
    let mut i64_2: i64 = -78i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, f32_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_6, duration_4);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_2, i32_1);
    let mut i128_0: i128 = crate::duration::Duration::whole_milliseconds(duration_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2995() {
    rusty_monitor::set_test_id(2995);
    let mut i64_0: i64 = -93i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -89i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut i64_2: i64 = -254i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = -72i32;
    let mut i64_3: i64 = -50i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i64_4: i64 = 136i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_6, i32_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    let mut f64_0: f64 = std::ops::Div::div(duration_3, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4389() {
    rusty_monitor::set_test_id(4389);
    let mut i64_0: i64 = -32i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut f32_0: f32 = -57.457711f32;
    let mut f32_1: f32 = 95.172796f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f32_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut u16_0: u16 = 49u16;
    let mut i32_0: i32 = -5i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_3_ref_0, duration_1_ref_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3242() {
    rusty_monitor::set_test_id(3242);
    let mut f32_0: f32 = -28.235860f32;
    let mut i16_0: i16 = 100i16;
    let mut i64_0: i64 = -133i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 9i32;
    let mut i64_1: i64 = -33i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut i128_0: i128 = -74i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_0: i8 = -33i8;
    let mut i64_2: i64 = -118i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_4);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4958() {
    rusty_monitor::set_test_id(4958);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 32u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 20i8;
    let mut i8_1: i8 = 103i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 96i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = -57i8;
    let mut i8_4: i8 = 16i8;
    let mut i8_5: i8 = -41i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 37u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 48u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 66i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i8_6: i8 = 111i8;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = 72i32;
    let mut i64_0: i64 = 41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_6);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_days(duration_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_887() {
    rusty_monitor::set_test_id(887);
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 26u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 84u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i16_0: i16 = 52i16;
    let mut i64_0: i64 = 62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1763() {
    rusty_monitor::set_test_id(1763);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_5: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_6: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_7: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = 62i32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3292() {
    rusty_monitor::set_test_id(3292);
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u8_0: u8 = 15u8;
    let mut i32_0: i32 = 130i32;
    let mut i64_1: i64 = -173i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
    let mut u8_1: u8 = 27u8;
    let mut i64_2: i64 = 191i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_2);
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3591() {
    rusty_monitor::set_test_id(3591);
    let mut i32_0: i32 = -45i32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f64_0: f64 = -125.599502f64;
    let mut i64_0: i64 = -1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 14u16;
    let mut i32_1: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_add(primitivedatetime_1, duration_3);
    let mut option_1: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1778() {
    rusty_monitor::set_test_id(1778);
    let mut u16_0: u16 = 16u16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i16_0: i16 = -39i16;
    let mut i64_0: i64 = 163i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i16_0);
    let mut u16_1: u16 = 25u16;
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut i64_1: i64 = 98i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_762() {
    rusty_monitor::set_test_id(762);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -108i32;
    let mut i64_0: i64 = 7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut i32_1: i32 = 195i32;
    let mut i64_1: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_2: i32 = 67i32;
    let mut i64_2: i64 = -114i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i16_0: i16 = -81i16;
    let mut i64_3: i64 = -74i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i16_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_days(duration_5);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2255() {
    rusty_monitor::set_test_id(2255);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -114i32;
    let mut i64_0: i64 = -46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut i64_1: i64 = 33i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i128_0: i128 = 8i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 16u16;
    let mut i32_1: i32 = 87i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_0: u32 = 41u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 82u8;
    let mut u8_2: u8 = 68u8;
    let mut i32_2: i32 = 122i32;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut u16_1: u16 = crate::util::days_in_year(i32_2);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4602() {
    rusty_monitor::set_test_id(4602);
    let mut u16_0: u16 = 57u16;
    let mut i32_0: i32 = 161i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = -27i32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 55i32;
    let mut i64_0: i64 = -83i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_2, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_1);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 62u8;
    let mut u8_1: u8 = 34u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_nano(time_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_1);
    let mut tuple_1: (i32, u8) = crate::date::Date::iso_year_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4473() {
    rusty_monitor::set_test_id(4473);
    let mut i64_0: i64 = -9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 145i32;
    let mut i64_1: i64 = 57i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_2: i64 = 35i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut i128_0: i128 = 100i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 67i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 46u16;
    let mut i32_1: i32 = -95i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_0);
    let mut i64_4: i64 = crate::duration::Duration::whole_seconds(duration_3);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2165() {
    rusty_monitor::set_test_id(2165);
    let mut i32_0: i32 = 172i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i16_0: i16 = -28i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i32_1: i32 = -110i32;
    let mut f64_0: f64 = -41.772232f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_1);
    let mut i64_0: i64 = -37i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 60u8;
    let mut i32_2: i32 = 4i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_5);
    let mut i16_1: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_1_ref_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = std::result::Result::unwrap(result_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3521() {
    rusty_monitor::set_test_id(3521);
    let mut f32_0: f32 = 2.373944f32;
    let mut u32_0: u32 = 56u32;
    let mut i64_0: i64 = -132i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i8_0: i8 = 41i8;
    let mut i64_1: i64 = -63i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = 115i64;
    let mut i64_3: i64 = -81i64;
    let mut i64_4: i64 = 73i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_98() {
    rusty_monitor::set_test_id(98);
    let mut i8_0: i8 = -83i8;
    let mut i8_1: i8 = 58i8;
    let mut i8_2: i8 = -34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -72i8;
    let mut i8_4: i8 = -12i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 42u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 66u32;
    let mut u8_3: u8 = 71u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 39u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -34i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_1: i32 = -9i32;
    let mut i64_0: i64 = -70i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut u32_2: u32 = 37u32;
    let mut u8_6: u8 = 67u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 50u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_9: u8 = 26u8;
    let mut i32_2: i32 = 195i32;
    let mut i32_3: i32 = -25i32;
    let mut i64_1: i64 = 127i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i32_3);
    let mut i8_6: i8 = 57i8;
    let mut f64_0: f64 = 43.860811f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i8_6);
    let mut u32_3: u32 = 48u32;
    let mut u8_10: u8 = 22u8;
    let mut u8_11: u8 = 8u8;
    let mut u8_12: u8 = 85u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i16_0: i16 = -9i16;
    let mut i64_2: i64 = -77i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
    let mut i32_4: i32 = 191i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_6);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut u8_13: u8 = 48u8;
    let mut i128_0: i128 = 37i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, u8_13);
    let mut u16_0: u16 = 56u16;
    let mut i32_5: i32 = 132i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut u16_1: u16 = crate::date::Date::ordinal(date_3);
    let mut i64_3: i64 = crate::duration::Duration::whole_days(duration_8);
    let mut tuple_0: (i32, month::Month, u8) = crate::offset_date_time::OffsetDateTime::to_calendar_date(offsetdatetime_3);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_9, weekday_0);
    let mut tuple_1: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_2, duration_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2964() {
    rusty_monitor::set_test_id(2964);
    let mut f32_0: f32 = 149.262381f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_0: i64 = -74i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = -215i32;
    let mut u8_0: u8 = 91u8;
    let mut i64_1: i64 = -29i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u8_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = -64i32;
    let mut i64_2: i64 = 98i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut u32_0: u32 = 31u32;
    let mut f32_1: f32 = -119.732556f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u32_0);
    let mut f32_2: f32 = -172.392049f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_8);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_5);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut bool_1: bool = std::cmp::PartialEq::ne(duration_9_ref_0, duration_4_ref_0);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut month_0: month::Month = crate::month::Month::September;
    let mut duration_10: crate::duration::Duration = std::ops::Sub::sub(duration_2, duration_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3606() {
    rusty_monitor::set_test_id(3606);
    let mut i64_0: i64 = 60i64;
    let mut i128_0: i128 = -154i128;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 127i64;
    let mut i64_2: i64 = 31i64;
    let mut i64_3: i64 = -94i64;
    let mut str_0: &str = "BbeV2bLEMEcy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = -137i32;
    let mut i64_4: i64 = -154i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_0, padding: padding_0};
    let mut i128_1: i128 = crate::duration::Duration::whole_milliseconds(duration_0);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3835() {
    rusty_monitor::set_test_id(3835);
    let mut i32_0: i32 = -104i32;
    let mut i64_0: i64 = -62i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut f32_0: f32 = -166.661057f32;
    let mut i8_0: i8 = -39i8;
    let mut f64_0: f64 = 43.407706f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut i64_1: i64 = -35i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_4_ref_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_3, f32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_1, i32_0);
    panic!("From RustyUnit with love");
}
}