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
	use std::ops::Div;
	use std::ops::Mul;
	use std::ops::Add;
	use std::cmp::PartialOrd;
	use std::convert::TryFrom;
	use std::ops::Sub;
	use std::cmp::Eq;
	use std::ops::Neg;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_285() {
    rusty_monitor::set_test_id(285);
    let mut f64_0: f64 = 223.143946f64;
    let mut i32_0: i32 = 17i32;
    let mut i64_0: i64 = 63i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut u16_0: u16 = 14u16;
    let mut i32_1: i32 = -65i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut u32_0: u32 = 49u32;
    let mut u32_1: u32 = 54u32;
    let mut i128_0: i128 = -1i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_0);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut duration_4: crate::duration::Duration = std::clone::Clone::clone(duration_3_ref_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_hours(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_523() {
    rusty_monitor::set_test_id(523);
    let mut i64_0: i64 = -220i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 16i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut u16_0: u16 = 3u16;
    let mut i32_0: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i8_0: i8 = -82i8;
    let mut i128_0: i128 = 60i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_3);
    let mut option_1: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2562() {
    rusty_monitor::set_test_id(2562);
    let mut i64_0: i64 = -65i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = -87i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut u32_0: u32 = 28u32;
    let mut u8_0: u8 = 98u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 45u8;
    let mut i64_2: i64 = -86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = 2i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_0: i32 = -39i32;
    let mut i64_4: i64 = -126i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut u16_0: u16 = 23u16;
    let mut i32_1: i32 = 27i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_2: i32 = -12i32;
    let mut i32_3: i32 = 151i32;
    let mut f64_0: f64 = 89.029942f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i32_3);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut i32_4: i32 = 21i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut month_0: month::Month = crate::date::Date::month(date_2);
    let mut u32_1: u32 = crate::time::Time::microsecond(time_0);
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_1_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2518() {
    rusty_monitor::set_test_id(2518);
    let mut i64_0: i64 = -123i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i32_0: i32 = 134i32;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 21u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 74u8;
    let mut i32_1: i32 = 60i32;
    let mut i64_1: i64 = 189i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut bool_0: bool = crate::util::is_leap_year(i32_1);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2561() {
    rusty_monitor::set_test_id(2561);
    let mut i32_0: i32 = 25i32;
    let mut f32_0: f32 = -78.926949f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -131i32;
    let mut i64_0: i64 = 137i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_2: i32 = -90i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 19u8;
    let mut i32_3: i32 = 23i32;
    let mut i32_4: i32 = 12i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut u8_1: u8 = crate::date::Date::sunday_based_week(date_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_0, weekday_1);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1850() {
    rusty_monitor::set_test_id(1850);
    let mut i32_0: i32 = -78i32;
    let mut i64_0: i64 = -71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u32_0: u32 = 76u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -13i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut u8_3: u8 = 13u8;
    let mut i64_1: i64 = -148i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_3);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut i64_2: i64 = -55i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = -161i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut duration_7: crate::duration::Duration = std::default::Default::default();
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_6, duration_2);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_616() {
    rusty_monitor::set_test_id(616);
    let mut u8_0: u8 = 10u8;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut u16_0: u16 = 61u16;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 48u8;
    let mut u8_3: u8 = 50u8;
    let mut u8_4: u8 = 99u8;
    let mut i8_0: i8 = 11i8;
    let mut f32_0: f32 = -118.669055f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u16_1: u16 = 3u16;
    let mut i64_0: i64 = -37i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_4);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_3, u8_2, u8_1, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_960() {
    rusty_monitor::set_test_id(960);
    let mut f64_0: f64 = -22.943446f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = -146i64;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 80u8;
    let mut u8_1: u8 = 80u8;
    let mut u8_2: u8 = 92u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -2i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut i32_0: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2908() {
    rusty_monitor::set_test_id(2908);
    let mut i128_0: i128 = -70i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i32_0: i32 = 2i32;
    let mut i64_0: i64 = -20i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i8_0: i8 = 9i8;
    let mut i8_1: i8 = 69i8;
    let mut i8_2: i8 = -110i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut u8_0: u8 = 21u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 96u8;
    let mut u16_0: u16 = 23u16;
    let mut i32_1: i32 = 11i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i32_2: i32 = -56i32;
    let mut i64_1: i64 = -63i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_2);
    let mut f64_0: f64 = 92.963971f64;
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = std::ops::Sub::sub(duration_7, duration_6);
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_4);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_2, u8_2, u8_1, u8_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_8_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1875() {
    rusty_monitor::set_test_id(1875);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = -203i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_1: i64 = 59i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f64_0: f64 = 146.620348f64;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -197i32;
    let mut i64_2: i64 = -25i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_0, padding: padding_0};
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, f64_0);
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_1_ref_0, duration_0_ref_0);
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut month_0: month::Month = crate::month::Month::November;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_874() {
    rusty_monitor::set_test_id(874);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i16_0: i16 = 47i16;
    let mut i64_0: i64 = -77i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut u16_0: u16 = 6u16;
    let mut i32_0: i32 = -146i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut f64_0: f64 = -44.277198f64;
    let mut i64_1: i64 = 14i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_3);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(padding_1_ref_0, padding_0_ref_0);
    let mut ordering_0: std::cmp::Ordering = std::option::Option::unwrap(option_0);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_572() {
    rusty_monitor::set_test_id(572);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i64_0: i64 = 205i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut month_1: month::Month = crate::month::Month::July;
    let mut i32_0: i32 = -101i32;
    let mut u32_0: u32 = 47u32;
    let mut i128_0: i128 = 152i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_0);
    let mut u16_0: u16 = 28u16;
    let mut i32_1: i32 = -188i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut tuple_0: (i32, u8) = crate::date::Date::iso_year_week(date_1);
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_0, month_1);
    let mut i64_1: i64 = crate::duration::Duration::whole_days(duration_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1805() {
    rusty_monitor::set_test_id(1805);
    let mut i16_0: i16 = 48i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = -32i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = -55i32;
    let mut i32_1: i32 = 30i32;
    let mut i64_1: i64 = 73i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_0);
    let mut f32_0: f32 = -13.291636f32;
    let mut i128_0: i128 = -113i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, f32_0);
    let mut i32_2: i32 = 124i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_6);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_1, duration_4);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_2, i16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_369() {
    rusty_monitor::set_test_id(369);
    let mut i16_0: i16 = 148i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = 5i32;
    let mut f64_0: f64 = -13.183505f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_1: i32 = 170i32;
    let mut i64_0: i64 = 111i64;
    let mut u8_0: u8 = 94u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 57u8;
    let mut u32_0: u32 = 77u32;
    let mut u8_3: u8 = 34u8;
    let mut u8_4: u8 = 61u8;
    let mut u8_5: u8 = 93u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_5, u8_4, u8_3, u32_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_2);
    let mut u8_6: u8 = crate::util::days_in_year_month(i32_0, month_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_3_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1555() {
    rusty_monitor::set_test_id(1555);
    let mut i8_0: i8 = -8i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -36i8;
    let mut i8_3: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u16_0: u16 = 14u16;
    let mut i32_0: i32 = 116i32;
    let mut f64_0: f64 = 57.144785f64;
    let mut u16_1: u16 = 13u16;
    let mut i64_0: i64 = 11i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_1);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_1, f64_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_208() {
    rusty_monitor::set_test_id(208);
    let mut i8_0: i8 = 92i8;
    let mut i8_1: i8 = -67i8;
    let mut i8_2: i8 = -15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 26u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 63u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -15i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_0: i64 = -111i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -104i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut f64_0: f64 = -186.981530f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4238() {
    rusty_monitor::set_test_id(4238);
    let mut i8_0: i8 = -110i8;
    let mut i8_1: i8 = -77i8;
    let mut i8_2: i8 = 80i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_0: i32 = -13i32;
    let mut i32_1: i32 = 70i32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 96i32;
    let mut i64_0: i64 = -43i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_2, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_1);
    let mut bool_0: bool = crate::util::is_leap_year(i32_0);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::December;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4075() {
    rusty_monitor::set_test_id(4075);
    let mut i16_0: i16 = -219i16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i32_0: i32 = 42i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut u32_0: u32 = 79u32;
    let mut f32_0: f32 = -24.999396f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u32_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i64_0: i64 = 0i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_monday(weekday_2);
    let mut i32_1: i32 = crate::duration::Duration::subsec_microseconds(duration_4);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    let mut bool_0: bool = crate::duration::Duration::is_positive(duration_5);
    let mut month_0: month::Month = crate::month::Month::July;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3998() {
    rusty_monitor::set_test_id(3998);
    let mut i64_0: i64 = 114i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut f64_0: f64 = 12.668942f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 20u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 99u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 21i32;
    let mut i64_1: i64 = 31i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2864() {
    rusty_monitor::set_test_id(2864);
    let mut u32_0: u32 = 84u32;
    let mut u8_0: u8 = 78u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 72u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 222i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i8_0: i8 = 44i8;
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = 72i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 16i8;
    let mut i8_4: i8 = -14i8;
    let mut i8_5: i8 = -75i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 94u32;
    let mut u8_3: u8 = 54u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 45u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_0: i64 = -216i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_1: i32 = 96i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut i32_2: i32 = -111i32;
    let mut i64_1: i64 = -110i64;
    let mut u32_2: u32 = 31u32;
    let mut i64_2: i64 = 17i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, u32_2);
    let mut i32_3: i32 = crate::duration::Duration::subsec_microseconds(duration_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1575() {
    rusty_monitor::set_test_id(1575);
    let mut u16_0: u16 = 53u16;
    let mut i32_0: i32 = -120i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 46i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = 11i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i16_0: i16 = 52i16;
    let mut i64_2: i64 = 92i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i16_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_6_ref_0, duration_4_ref_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_1, duration_2);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3656() {
    rusty_monitor::set_test_id(3656);
    let mut i32_0: i32 = 20i32;
    let mut f64_0: f64 = 16.234315f64;
    let mut i64_0: i64 = 40i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut u32_0: u32 = 21u32;
    let mut u8_0: u8 = 79u8;
    let mut u8_1: u8 = 43u8;
    let mut u8_2: u8 = 83u8;
    let mut i64_1: i64 = 74i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut u16_0: u16 = 68u16;
    let mut i32_1: i32 = 104i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4351() {
    rusty_monitor::set_test_id(4351);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut f32_0: f32 = -173.809299f32;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 21i32;
    let mut i64_0: i64 = -18i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_1};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i64_1: i64 = 41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = 83i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i32_1: i32 = 42i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3196() {
    rusty_monitor::set_test_id(3196);
    let mut i8_0: i8 = 22i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut f64_0: f64 = -14.237056f64;
    let mut i64_0: i64 = 145i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut i32_0: i32 = 48i32;
    let mut i64_1: i64 = 150i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i32_1: i32 = 149i32;
    let mut f32_0: f32 = 52.607663f32;
    let mut i32_2: i32 = -19i32;
    let mut i64_2: i64 = -50i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, f32_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_1);
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_8, duration_5);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3180() {
    rusty_monitor::set_test_id(3180);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 84i64;
    let mut i64_1: i64 = 36i64;
    let mut i64_2: i64 = -11i64;
    let mut str_0: &str = "OhUsoVjILYyTy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i32_0: i32 = -38i32;
    let mut i64_3: i64 = 13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut u16_0: u16 = 67u16;
    let mut i64_4: i64 = 97i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u16_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_4_ref_0, duration_2_ref_0);
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3232() {
    rusty_monitor::set_test_id(3232);
    let mut i64_0: i64 = 131i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -87i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut u16_0: u16 = 17u16;
    let mut i32_0: i32 = 167i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut f32_0: f32 = 111.764969f32;
    let mut u16_1: u16 = 36u16;
    let mut f32_1: f32 = 128.332254f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, u16_1);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, f32_0);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_789() {
    rusty_monitor::set_test_id(789);
    let mut i64_0: i64 = 39i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i32_0: i32 = 89i32;
    let mut i64_2: i64 = -56i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut f32_0: f32 = 91.721505f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut duration_5_ref_0: &std::time::Duration = &mut duration_5;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 118i32;
    let mut i64_3: i64 = 158i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_1, padding: padding_0};
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_5_ref_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i32_2: i32 = crate::duration::Duration::subsec_microseconds(duration_3);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3903() {
    rusty_monitor::set_test_id(3903);
    let mut i32_0: i32 = -231i32;
    let mut i64_0: i64 = 146i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut i16_0: i16 = -70i16;
    let mut i16_1: i16 = 155i16;
    let mut i64_1: i64 = 19i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i16_1);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut f64_0: f64 = -120.441938f64;
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 94u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 67u32;
    let mut u8_3: u8 = 14u8;
    let mut u8_4: u8 = 54u8;
    let mut u8_5: u8 = 55u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_0: f32 = -181.327852f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_3, i16_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4008() {
    rusty_monitor::set_test_id(4008);
    let mut u32_0: u32 = 24u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 36u8;
    let mut u8_2: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 30u16;
    let mut i32_0: i32 = -25i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i16_0: i16 = -110i16;
    let mut i64_0: i64 = -72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i64_1: i64 = -108i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -193i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut f64_0: f64 = std::ops::Div::div(duration_2, duration_1);
    let mut tuple_0: (month::Month, u8) = crate::date::Date::month_day(date_0);
    let mut u8_3: u8 = crate::time::Time::minute(time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1364() {
    rusty_monitor::set_test_id(1364);
    let mut u16_0: u16 = 22u16;
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut f64_0: f64 = -42.812364f64;
    let mut f64_1: f64 = -54.767109f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_1: i64 = 118i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = -77i32;
    let mut i64_2: i64 = -54i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_0, padding: padding_0};
    let mut duration_5: crate::duration::Duration = std::ops::Add::add(duration_4, duration_3);
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_2);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_633() {
    rusty_monitor::set_test_id(633);
    let mut u32_0: u32 = 58u32;
    let mut i64_0: i64 = -110i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f64_0: f64 = 0.245974f64;
    let mut i64_1: i64 = 0i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i8_0: i8 = -34i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i16_0: i16 = 22i16;
    let mut i64_2: i64 = -36i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, i16_0);
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::day(primitivedatetime_0);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_3_ref_0, duration_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4206() {
    rusty_monitor::set_test_id(4206);
    let mut f64_0: f64 = -84.427367f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_0: i8 = -119i8;
    let mut i8_1: i8 = 26i8;
    let mut i8_2: i8 = 7i8;
    let mut f64_1: f64 = -87.449624f64;
    let mut u32_0: u32 = 59u32;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_0);
    let mut u16_0: u16 = 97u16;
    let mut i32_0: i32 = -73i32;
    let mut i64_0: i64 = -129i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u16_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_3, f64_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_1_ref_0, duration_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4225() {
    rusty_monitor::set_test_id(4225);
    let mut i8_0: i8 = -25i8;
    let mut i8_1: i8 = 46i8;
    let mut i8_2: i8 = 58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 43u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 49u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 91u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i16_0: i16 = 50i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut u16_0: u16 = 48u16;
    let mut i32_0: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut u32_2: u32 = 7u32;
    let mut i64_0: i64 = 54i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_2);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_3);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::hour(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_664() {
    rusty_monitor::set_test_id(664);
    let mut i8_0: i8 = -100i8;
    let mut i64_0: i64 = 14i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i32_0: i32 = 17i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut i8_1: i8 = -91i8;
    let mut i8_2: i8 = -49i8;
    let mut i8_3: i8 = -90i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_0: u32 = 17u32;
    let mut u8_0: u8 = 49u8;
    let mut u8_1: u8 = 83u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 90i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = 6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut i32_1: i32 = 10i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut i8_4: i8 = -50i8;
    let mut i8_5: i8 = 38i8;
    let mut i8_6: i8 = -69i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = 77i8;
    let mut i64_3: i64 = -94i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, i8_7);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_6);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3980() {
    rusty_monitor::set_test_id(3980);
    let mut u16_0: u16 = 49u16;
    let mut i64_0: i64 = -117i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_0);
    let mut i32_0: i32 = 6i32;
    let mut i64_1: i64 = -68i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 47u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 34u8;
    let mut i64_2: i64 = 139i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut i32_1: i32 = -128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4437() {
    rusty_monitor::set_test_id(4437);
    let mut u32_0: u32 = 55u32;
    let mut u8_0: u8 = 67u8;
    let mut u8_1: u8 = 34u8;
    let mut u8_2: u8 = 47u8;
    let mut i128_0: i128 = 7i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f64_0: f64 = -83.319880f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 99u32;
    let mut i16_0: i16 = -37i16;
    let mut f32_0: f32 = 19.662907f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i16_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u32_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4860() {
    rusty_monitor::set_test_id(4860);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 41u8;
    let mut u8_1: u8 = 96u8;
    let mut u8_2: u8 = 66u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 19u16;
    let mut i32_0: i32 = -120i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u16_1: u16 = 24u16;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = 112i32;
    let mut i64_0: i64 = -127i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u16_1);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut bool_0: bool = crate::duration::Duration::is_negative(duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3258() {
    rusty_monitor::set_test_id(3258);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut u32_0: u32 = 78u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 18u8;
    let mut i8_0: i8 = -46i8;
    let mut i8_1: i8 = -11i8;
    let mut i8_2: i8 = 101i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f32_0: f32 = -34.875258f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = -152i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Neg::neg(duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut u32_1: u32 = 74u32;
    let mut u8_3: u8 = 87u8;
    let mut u8_4: u8 = 70u8;
    let mut u8_5: u8 = 79u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = -161i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = -43i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4941() {
    rusty_monitor::set_test_id(4941);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 10i32;
    let mut i64_0: i64 = 190i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut i32_1: i32 = -182i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i32_2: i32 = 95i32;
    let mut i64_1: i64 = 110i64;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_2_ref_0: &duration::Padding = &mut padding_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_2_ref_0, padding_1_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4202() {
    rusty_monitor::set_test_id(4202);
    let mut i8_0: i8 = 72i8;
    let mut i8_1: i8 = 69i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -53i8;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_3);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 13u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 15u16;
    let mut i32_0: i32 = 21i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_1);
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_667() {
    rusty_monitor::set_test_id(667);
    let mut f32_0: f32 = -24.523656f32;
    let mut i64_0: i64 = 75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u16_0: u16 = 78u16;
    let mut i32_0: i32 = 6i32;
    let mut i32_1: i32 = 142i32;
    let mut i64_1: i64 = 44i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i128_0: i128 = -102i128;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1492() {
    rusty_monitor::set_test_id(1492);
    let mut i8_0: i8 = -49i8;
    let mut f64_0: f64 = -13.539319f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i64_0: i64 = 47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = 108i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i64_2: i64 = -69i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut i32_0: i32 = 27i32;
    let mut i64_3: i64 = -5i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Add::add(duration_5, duration_4);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_6_ref_0, duration_3_ref_0);
    let mut bool_1: bool = crate::duration::Duration::is_positive(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1373() {
    rusty_monitor::set_test_id(1373);
    let mut f64_0: f64 = 96.949099f64;
    let mut i64_0: i64 = -22i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut i64_1: i64 = 59i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 99u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 49u8;
    let mut u8_2: u8 = 1u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_679() {
    rusty_monitor::set_test_id(679);
    let mut u32_0: u32 = 6u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 44u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -8i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_0: i64 = 10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = -110i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut i32_1: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(padding_0_ref_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1607() {
    rusty_monitor::set_test_id(1607);
    let mut u8_0: u8 = 83u8;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut i32_0: i32 = 53i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut i32_1: i32 = -19i32;
    let mut u32_0: u32 = 24u32;
    let mut i32_2: i32 = -85i32;
    let mut i64_0: i64 = -37i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_3, i32_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2618() {
    rusty_monitor::set_test_id(2618);
    let mut u8_0: u8 = 67u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i8_0: i8 = -71i8;
    let mut i64_0: i64 = -41i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i8_0);
    let mut duration_4: crate::duration::Duration = std::default::Default::default();
    let mut i32_0: i32 = 166i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut u8_1: u8 = 3u8;
    let mut i128_0: i128 = 55i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u8_1);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_6_ref_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_1);
    let mut duration_7: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1867() {
    rusty_monitor::set_test_id(1867);
    let mut u8_0: u8 = 78u8;
    let mut i64_0: i64 = -195i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut i32_0: i32 = -106i32;
    let mut u16_0: u16 = 64u16;
    let mut i64_1: i64 = 81i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = -33i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_1: u16 = 10u16;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u16_1);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_2, u16_0);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2017() {
    rusty_monitor::set_test_id(2017);
    let mut u8_0: u8 = 84u8;
    let mut u16_0: u16 = 39u16;
    let mut i32_0: i32 = -44i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_1: i32 = 27i32;
    let mut i64_0: i64 = -85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut u32_0: u32 = 60u32;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 6u8;
    let mut u8_3: u8 = 2u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_1: u32 = 32u32;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 10u8;
    let mut u8_6: u8 = 27u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_2: i32 = 33i32;
    let mut i64_1: i64 = -148i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = -41i32;
    let mut i8_0: i8 = -19i8;
    let mut i8_1: i8 = -111i8;
    let mut i8_2: i8 = 11i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_3, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3963() {
    rusty_monitor::set_test_id(3963);
    let mut u32_0: u32 = 24u32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 99u16;
    let mut i32_0: i32 = 63i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = 96i8;
    let mut i8_1: i8 = 36i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = 60i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_1: u16 = 88u16;
    let mut i32_1: i32 = 9i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2253() {
    rusty_monitor::set_test_id(2253);
    let mut i64_0: i64 = 82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 40u16;
    let mut i32_0: i32 = 60i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_0: i8 = -16i8;
    let mut i8_1: i8 = 57i8;
    let mut i8_2: i8 = -14i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i16_0: i16 = 15i16;
    let mut i64_1: i64 = 19i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i16_0);
    let mut u16_1: u16 = 8u16;
    let mut i32_1: i32 = 22i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_764() {
    rusty_monitor::set_test_id(764);
    let mut u32_0: u32 = 64u32;
    let mut u8_0: u8 = 19u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 31u8;
    let mut u16_0: u16 = 17u16;
    let mut f32_0: f32 = 180.355951f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut i32_0: i32 = 61i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = 169i64;
    let mut i64_1: i64 = -68i64;
    let mut i64_2: i64 = 175i64;
    let mut str_0: &str = "4I";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4796() {
    rusty_monitor::set_test_id(4796);
    let mut u32_0: u32 = 94u32;
    let mut i64_0: i64 = 48i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut i64_1: i64 = 113i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i128_0: i128 = 176i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 114i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3860() {
    rusty_monitor::set_test_id(3860);
    let mut f64_0: f64 = -168.787810f64;
    let mut i64_0: i64 = 73i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -138i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_0: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut f32_0: f32 = -96.357448f32;
    let mut i16_0: i16 = -129i16;
    let mut i32_1: i32 = 49i32;
    let mut i64_2: i64 = -103i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4420() {
    rusty_monitor::set_test_id(4420);
    let mut i16_0: i16 = 251i16;
    let mut i64_0: i64 = -66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut f32_0: f32 = 88.734926f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_1: i64 = 136i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut f32_1: f32 = -174.313736f32;
    let mut i64_2: i64 = -14i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f32_1);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_7);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_4, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3653() {
    rusty_monitor::set_test_id(3653);
    let mut u32_0: u32 = 19u32;
    let mut f32_0: f32 = 16.246055f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_0);
    let mut u16_0: u16 = 91u16;
    let mut i64_0: i64 = -50i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut u32_1: u32 = 86u32;
    let mut u8_0: u8 = 37u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut u16_1: u16 = 96u16;
    let mut i32_0: i32 = 208i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2299() {
    rusty_monitor::set_test_id(2299);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 76u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 40u32;
    let mut u8_3: u8 = 76u8;
    let mut u8_4: u8 = 43u8;
    let mut u8_5: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 15u16;
    let mut i32_0: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut f32_0: f32 = -63.812982f32;
    let mut i64_0: i64 = -163i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1914() {
    rusty_monitor::set_test_id(1914);
    let mut f64_0: f64 = -105.261791f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = -4i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut i64_1: i64 = -95i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i64_2: i64 = 80i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = 210i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_6);
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_5_ref_0, duration_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3315() {
    rusty_monitor::set_test_id(3315);
    let mut u32_0: u32 = 41u32;
    let mut u8_0: u8 = 85u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 74u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 99u32;
    let mut u8_3: u8 = 9u8;
    let mut u8_4: u8 = 39u8;
    let mut u8_5: u8 = 99u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -30i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut u32_2: u32 = 3u32;
    let mut u8_6: u8 = 47u8;
    let mut u8_7: u8 = 2u8;
    let mut u8_8: u8 = 0u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i8_0: i8 = -118i8;
    let mut i8_1: i8 = 39i8;
    let mut i8_2: i8 = -106i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i128_0: i128 = -8i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f64_0: f64 = -143.724301f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut tuple_0: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_1);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u32_3: u32 = crate::time::Time::nanosecond(time_2);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i32_1: i32 = crate::utc_offset::UtcOffset::whole_seconds(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2057() {
    rusty_monitor::set_test_id(2057);
    let mut i32_0: i32 = 103i32;
    let mut i32_1: i32 = 75i32;
    let mut i64_0: i64 = -9i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut i32_2: i32 = 70i32;
    let mut i64_1: i64 = -64i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = 65i32;
    let mut i64_2: i64 = -73i64;
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 67u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_2, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_695() {
    rusty_monitor::set_test_id(695);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = -83i8;
    let mut i8_2: i8 = -45i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 195i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = -51i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut i64_2: i64 = 153i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_5);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3954() {
    rusty_monitor::set_test_id(3954);
    let mut u8_0: u8 = 27u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut u32_0: u32 = 79u32;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 36u8;
    let mut u8_3: u8 = 43u8;
    let mut i64_0: i64 = -118i64;
    let mut i32_0: i32 = -132i32;
    let mut i64_1: i64 = 65i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i32_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_weeks(duration_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_3, u8_2, u8_1, u32_0);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4137() {
    rusty_monitor::set_test_id(4137);
    let mut i64_0: i64 = 132i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f64_0: f64 = -93.010173f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_0: u16 = 58u16;
    let mut i16_0: i16 = -52i16;
    let mut i64_1: i64 = -29i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i16_0);
    let mut month_0: month::Month = crate::month::Month::September;
    let mut i32_0: i32 = -26i32;
    let mut i64_2: i64 = -56i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut i64_3: i64 = 85i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 18u8;
    let mut u8_2: u8 = 92u8;
    let mut i64_4: i64 = 30i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i64_5: i64 = crate::duration::Duration::whole_hours(duration_8);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut i64_6: i64 = crate::duration::Duration::whole_hours(duration_7);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut duration_9: crate::duration::Duration = std::ops::Div::div(duration_4, u16_0);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_sub(offsetdatetime_0, duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_61() {
    rusty_monitor::set_test_id(61);
    let mut f64_0: f64 = 162.265392f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 8i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i128_0: i128 = 34i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = 66.513885f32;
    let mut f32_1: f32 = 69.766114f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut i64_0: i64 = -67i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 54u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = crate::duration::Duration::whole_seconds(duration_1);
    let mut option_0: std::option::Option<crate::primitive_date_time::PrimitiveDateTime> = crate::primitive_date_time::PrimitiveDateTime::checked_sub(primitivedatetime_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut f64_0: f64 = 78.244371f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 57i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 44u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 278i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i64_0: i64 = -65i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i64_1: i64 = -95i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_1_ref_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4858() {
    rusty_monitor::set_test_id(4858);
    let mut i32_0: i32 = -29i32;
    let mut i64_0: i64 = 160i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i8_0: i8 = -115i8;
    let mut i8_1: i8 = 10i8;
    let mut i8_2: i8 = -10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 22u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -88i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut i32_2: i32 = -8i32;
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_2, month_0);
    let mut i64_2: i64 = crate::duration::Duration::whole_days(duration_1);
    let mut duration_2: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3961() {
    rusty_monitor::set_test_id(3961);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 73u16;
    let mut i32_0: i32 = -20i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i8_0: i8 = -67i8;
    let mut i8_1: i8 = 63i8;
    let mut i8_2: i8 = -24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 9u32;
    let mut u8_3: u8 = 18u8;
    let mut u8_4: u8 = 58u8;
    let mut u8_5: u8 = 56u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -130i32;
    let mut i64_0: i64 = 197i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut u16_1: u16 = 81u16;
    let mut i32_2: i32 = -186i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut i64_1: i64 = 54i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i16_0: i16 = 42i16;
    let mut f64_0: f64 = -53.784859f64;
    let mut i128_0: i128 = 18i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_1_ref_0);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4485() {
    rusty_monitor::set_test_id(4485);
    let mut i64_0: i64 = -30i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 40i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Add::add(duration_1, duration_0);
    let mut i64_2: i64 = -27i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i128_0: i128 = 137i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 20u16;
    let mut i32_0: i32 = -3i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_5);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut i32_1: i32 = -136i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4259() {
    rusty_monitor::set_test_id(4259);
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 40u8;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut u32_0: u32 = 77u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 2u8;
    let mut u8_5: u8 = 18u8;
    let mut i8_0: i8 = -48i8;
    let mut i128_0: i128 = -82i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut i32_0: i32 = -140i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut i64_0: i64 = 255i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i32_1: i32 = -77i32;
    let mut duration_3: crate::duration::Duration = std::default::Default::default();
    let mut u32_1: u32 = 8u32;
    let mut u8_6: u8 = 78u8;
    let mut i64_1: i64 = 34i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u8_6);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_3, i32_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_2, u8_5, u8_4, u8_3, u32_0);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4022() {
    rusty_monitor::set_test_id(4022);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i32_0: i32 = 4i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_0: u8 = 31u8;
    let mut i64_0: i64 = 72i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i32_1: i32 = 34i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut date_2_ref_0: &crate::date::Date = &mut date_2;
    let mut i32_2: i32 = 20i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3_ref_0: &crate::date::Date = &mut date_3;
    let mut u8_1: u8 = crate::date::Date::day(date_0);
    let mut u8_2: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3832() {
    rusty_monitor::set_test_id(3832);
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u8_0: u8 = 20u8;
    let mut i32_0: i32 = 10i32;
    let mut i64_1: i64 = -67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = 85i32;
    let mut i64_2: i64 = 38i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut f64_0: f64 = 138.185596f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_1, u8_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4539() {
    rusty_monitor::set_test_id(4539);
    let mut f64_0: f64 = 6.246101f64;
    let mut f64_1: f64 = -226.496832f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut i64_0: i64 = -95i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut f32_0: f32 = 99.511062f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_4_ref_0, duration_3_ref_0);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_65() {
    rusty_monitor::set_test_id(65);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 98i8;
    let mut i8_2: i8 = 94i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -72i8;
    let mut i8_4: i8 = -80i8;
    let mut i8_5: i8 = -21i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut tuple_0: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4079() {
    rusty_monitor::set_test_id(4079);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i64_0: i64 = -31i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = -56i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i16_0: i16 = -39i16;
    let mut i32_1: i32 = -111i32;
    let mut i64_1: i64 = 16i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_3);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::primitive_date_time::PrimitiveDateTime::as_hms_milli(primitivedatetime_1);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    panic!("From RustyUnit with love");
}
}