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
	use std::ops::Neg;
	use std::ops::Sub;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1081() {
//    rusty_monitor::set_test_id(1081);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut i64_0: i64 = 1000000i64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_4);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_2, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut duration_7_ref_0: &crate::duration::Duration = &mut duration_7;
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_7_ref_0, duration_0_ref_0);
    let mut i32_2: i32 = crate::duration::Duration::subsec_nanoseconds(duration_6);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7631() {
//    rusty_monitor::set_test_id(7631);
    let mut i64_0: i64 = -119i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_0: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i8_3: i8 = 63i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 25i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 6u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut i8_6: i8 = 1i8;
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i8_6);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f64_0);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_1);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4820() {
//    rusty_monitor::set_test_id(4820);
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 3u8;
    let mut i32_0: i32 = -23i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = 127i8;
    let mut i64_0: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i8_0);
    let mut i32_1: i32 = 93i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut i64_1: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 2i8;
    let mut i8_3: i8 = -41i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i32_2: i32 = 48i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i32_3: i32 = 178i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut u8_3: u8 = crate::date::Date::monday_based_week(date_2);
    let mut i64_2: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1051() {
//    rusty_monitor::set_test_id(1051);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 32i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut i64_0: i64 = 24i64;
    let mut i8_0: i8 = 127i8;
    let mut i8_1: i8 = 96i8;
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_2, i8_1);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = std::ops::Neg::neg(duration_0);
    let mut i32_1: i32 = 100i32;
    let mut i32_2: i32 = 86399i32;
    let mut i64_1: i64 = 86400i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_1, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_3: i32 = 65i32;
    let mut i32_4: i32 = 291i32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_3);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = crate::duration::Duration::whole_seconds(duration_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6195() {
//    rusty_monitor::set_test_id(6195);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut i32_0: i32 = -89i32;
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -27i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 6i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 511i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut i32_2: i32 = 2i32;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_3: i32 = 18i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_3, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_0_ref_0);
    let mut u8_3: u8 = crate::util::days_in_year_month(i32_2, month_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_2);
    let mut month_2: month::Month = crate::month::Month::previous(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8891() {
//    rusty_monitor::set_test_id(8891);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_0: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_3);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4125() {
//    rusty_monitor::set_test_id(4125);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_0: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_3);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7512() {
//    rusty_monitor::set_test_id(7512);
    let mut u16_0: u16 = 999u16;
    let mut i64_0: i64 = 0i64;
    let mut u8_0: u8 = 23u8;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut i32_0: i32 = 13i32;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 109i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 4696837146684686335.421177f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_1: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 100000u32;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 30u8;
    let mut u8_3: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_2: i32 = 156i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 366u16;
    let mut i8_9: i8 = 127i8;
    let mut i64_1: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_9);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_1);
    let mut u8_4: u8 = crate::time::Time::minute(time_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_548() {
//    rusty_monitor::set_test_id(548);
    let mut i8_0: i8 = 5i8;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i32_0: i32 = 139i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_1: i32 = 235i32;
    let mut i64_1: i64 = -20i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i8_1: i8 = 4i8;
    let mut i8_2: i8 = 82i8;
    let mut i8_3: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_580() {
//    rusty_monitor::set_test_id(580);
    let mut u32_0: u32 = 100000000u32;
    let mut i32_0: i32 = 167i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_1);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 1721425i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut u32_2: u32 = 100000u32;
    let mut i64_1: i64 = 26i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u32_2);
    let mut i32_2: i32 = 314i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut i64_2: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut u16_1: u16 = 365u16;
    let mut i32_3: i32 = 82i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_3);
    let mut u8_4: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5765() {
//    rusty_monitor::set_test_id(5765);
    let mut i32_0: i32 = 99i32;
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = std::default::Default::default();
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 93i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_1, padding: padding_0};
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 325i32;
    let mut i64_2: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_2, padding: padding_1};
    let mut duration_5: crate::duration::Duration = std::ops::Sub::sub(duration_4, duration_3);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_347() {
//    rusty_monitor::set_test_id(347);
    let mut i32_0: i32 = 274i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut i32_1: i32 = 93i32;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i32_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_2: i64 = 604800i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut duration_7_ref_0: &std::time::Duration = &mut duration_7;
    let mut i32_2: i32 = 280i32;
    let mut duration_8: crate::duration::Duration = std::default::Default::default();
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_8, i32_2);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut i64_3: i64 = 2440588i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut duration_11_ref_0: &std::time::Duration = &mut duration_11;
    let mut i32_3: i32 = 105i32;
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_3);
    let mut duration_12_ref_0: &crate::duration::Duration = &mut duration_12;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_12_ref_0, duration_11_ref_0);
    let mut option_1: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_9_ref_0, duration_7_ref_0);
    let mut option_2: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_4_ref_0, duration_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_457() {
//    rusty_monitor::set_test_id(457);
    let mut i16_0: i16 = 4i16;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i16_0);
    let mut i8_0: i8 = -34i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 7i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_1: month::Month = crate::month::Month::December;
    let mut month_2: month::Month = crate::month::Month::January;
    let mut month_3: month::Month = crate::month::Month::September;
    let mut month_4: month::Month = crate::month::Month::November;
    let mut month_5: month::Month = crate::month::Month::November;
    let mut month_6: month::Month = crate::month::Month::November;
    let mut month_7: month::Month = crate::month::Month::May;
    let mut month_8: month::Month = crate::month::Month::August;
    let mut month_9: month::Month = crate::month::Month::August;
    let mut month_10: month::Month = crate::month::Month::September;
    let mut month_11: month::Month = crate::month::Month::October;
    let mut month_12: month::Month = crate::month::Month::April;
    let mut month_13: month::Month = crate::month::Month::next(month_12);
    let mut month_14: month::Month = crate::month::Month::next(month_11);
    let mut month_15: month::Month = crate::month::Month::next(month_10);
    let mut month_16: month::Month = crate::month::Month::next(month_9);
    let mut month_17: month::Month = crate::month::Month::next(month_8);
    let mut month_18: month::Month = crate::month::Month::next(month_7);
    let mut month_19: month::Month = crate::month::Month::next(month_6);
    let mut month_20: month::Month = crate::month::Month::next(month_5);
    let mut month_21: month::Month = crate::month::Month::next(month_4);
    let mut month_22: month::Month = crate::month::Month::next(month_3);
    let mut month_23: month::Month = crate::month::Month::next(month_2);
    let mut month_24: month::Month = crate::month::Month::next(month_1);
    let mut month_25: month::Month = crate::month::Month::next(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_635() {
//    rusty_monitor::set_test_id(635);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i32_0: i32 = 291i32;
    let mut i64_0: i64 = -9i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i32_1: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_4);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_300() {
//    rusty_monitor::set_test_id(300);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3348() {
//    rusty_monitor::set_test_id(3348);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = -108i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 51i8;
    let mut i8_4: i8 = 4i8;
    let mut i8_5: i8 = 6i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = 1i32;
    let mut i64_0: i64 = -82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = 4i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = -54i8;
    let mut i8_10: i8 = 60i8;
    let mut i8_11: i8 = 109i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 144i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut i8_12: i8 = 4i8;
    let mut i8_13: i8 = 0i8;
    let mut i8_14: i8 = 60i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i32_3: i32 = 156i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_1);
    let mut u16_0: u16 = 366u16;
    let mut i8_15: i8 = 127i8;
    let mut i64_1: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_15);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_2);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_449() {
//    rusty_monitor::set_test_id(449);
    let mut i8_0: i8 = 6i8;
    let mut f64_0: f64 = 113.890873f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 90i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_428() {
//    rusty_monitor::set_test_id(428);
    let mut i32_0: i32 = -105i32;
    let mut i64_0: i64 = 23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 178i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = std::ops::Sub::sub(duration_3, duration_2);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 64i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 28u8;
    let mut u8_5: u8 = 24u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_4);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4556() {
//    rusty_monitor::set_test_id(4556);
    let mut i8_0: i8 = 69i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = -72i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 23i8;
    let mut i8_7: i8 = -53i8;
    let mut i8_8: i8 = 0i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 88u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 83u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 353i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 2i32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut i32_2: i32 = 1721119i32;
    let mut i64_1: i64 = 1i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = 398i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_3: u8 = 53u8;
    let mut i32_4: i32 = 212i32;
    let mut i8_9: i8 = 28i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i8_9);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut duration_5: crate::duration::Duration = std::clone::Clone::clone(duration_4_ref_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_3, weekday_0);
    let mut i32_5: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut f64_0: f64 = std::ops::Div::div(duration_1, duration_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3713() {
//    rusty_monitor::set_test_id(3713);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_0: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_3);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_168() {
//    rusty_monitor::set_test_id(168);
    let mut i32_0: i32 = 144i32;
    let mut i128_0: i128 = 0i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i32_0);
    let mut i64_0: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 61u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 11u8;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_559() {
//    rusty_monitor::set_test_id(559);
    let mut f32_0: f32 = 169.430846f32;
    let mut i128_0: i128 = 0i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut i32_0: i32 = -92i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_1: i64 = 60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 69u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 31u8;
    let mut i64_2: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_9);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8806() {
//    rusty_monitor::set_test_id(8806);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 53i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_0: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_0);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_1);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6754() {
//    rusty_monitor::set_test_id(6754);
    let mut i32_0: i32 = 189i32;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 25i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 2147483647i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut i8_6: i8 = 1i8;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_6);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_0);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::hour(primitivedatetime_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_2, i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_367() {
//    rusty_monitor::set_test_id(367);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut padding_2: duration::Padding = std::default::Default::default();
    let mut padding_2_ref_0: &duration::Padding = &mut padding_2;
    let mut padding_3: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_3_ref_0: &duration::Padding = &mut padding_3;
    let mut padding_4: duration::Padding = std::default::Default::default();
    let mut padding_4_ref_0: &duration::Padding = &mut padding_4;
    let mut padding_5: duration::Padding = std::default::Default::default();
    let mut padding_5_ref_0: &duration::Padding = &mut padding_5;
    let mut padding_6: duration::Padding = std::default::Default::default();
    let mut padding_6_ref_0: &duration::Padding = &mut padding_6;
    let mut padding_7: duration::Padding = std::default::Default::default();
    let mut padding_7_ref_0: &duration::Padding = &mut padding_7;
    let mut padding_8: duration::Padding = std::default::Default::default();
    let mut padding_8_ref_0: &duration::Padding = &mut padding_8;
    let mut padding_9: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_9_ref_0: &duration::Padding = &mut padding_9;
    let mut padding_10: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_10_ref_0: &duration::Padding = &mut padding_10;
    let mut padding_11: duration::Padding = std::default::Default::default();
    let mut padding_11_ref_0: &duration::Padding = &mut padding_11;
    let mut padding_12: duration::Padding = std::default::Default::default();
    let mut padding_12_ref_0: &duration::Padding = &mut padding_12;
    let mut padding_13: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_13_ref_0: &duration::Padding = &mut padding_13;
    let mut bool_0: bool = std::cmp::PartialEq::eq(padding_13_ref_0, padding_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(padding_11_ref_0, padding_10_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(padding_9_ref_0, padding_8_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(padding_7_ref_0, padding_6_ref_0);
    let mut bool_4: bool = std::cmp::PartialEq::eq(padding_5_ref_0, padding_4_ref_0);
    let mut bool_5: bool = std::cmp::PartialEq::eq(padding_3_ref_0, padding_2_ref_0);
    let mut bool_6: bool = std::cmp::PartialEq::eq(padding_1_ref_0, padding_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_288() {
//    rusty_monitor::set_test_id(288);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_2_ref_0: &duration::Padding = &mut padding_2;
    let mut padding_3: duration::Padding = std::default::Default::default();
    let mut padding_3_ref_0: &duration::Padding = &mut padding_3;
    let mut padding_4: duration::Padding = std::default::Default::default();
    let mut padding_4_ref_0: &duration::Padding = &mut padding_4;
    let mut padding_5: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_5_ref_0: &duration::Padding = &mut padding_5;
    let mut padding_6: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_6_ref_0: &duration::Padding = &mut padding_6;
    let mut padding_7: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_7_ref_0: &duration::Padding = &mut padding_7;
    let mut padding_8: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_8_ref_0: &duration::Padding = &mut padding_8;
    let mut padding_9: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_9_ref_0: &duration::Padding = &mut padding_9;
    let mut padding_10: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_10_ref_0: &duration::Padding = &mut padding_10;
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
#[timeout(30000)]fn rusty_test_4063() {
//    rusty_monitor::set_test_id(4063);
    let mut i8_0: i8 = -61i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 263i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i8_3: i8 = 4i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -54i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 109i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 144i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_2};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_1);
    let mut u32_2: u32 = 100000u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 30u8;
    let mut u8_8: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_9: i8 = 4i8;
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_2: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_3, time_3);
    let mut i8_12: i8 = 127i8;
    let mut i64_0: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_12);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_4);
    let mut u8_9: u8 = crate::time::Time::minute(time_2);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_387() {
//    rusty_monitor::set_test_id(387);
    let mut i64_0: i64 = 0i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = -119i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_0: i32 = 364i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut time_0_ref_0: &mut crate::time::Time = &mut time_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1404() {
//    rusty_monitor::set_test_id(1404);
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -55i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_0: i8 = 20i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut i32_1: i32 = 100i32;
    let mut i32_2: i32 = 86399i32;
    let mut i64_2: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_1);
    let mut i8_6: i8 = -33i8;
    let mut i8_7: i8 = -76i8;
    let mut i8_8: i8 = 18i8;
    let mut i32_3: i32 = 235i32;
    let mut i32_4: i32 = 291i32;
    let mut i32_5: i32 = 43i32;
    let mut i64_3: i64 = 0i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_5);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_6, i32_4);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut bool_0: bool = crate::util::is_leap_year(i32_3);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_8, i8_7, i8_6);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7932() {
//    rusty_monitor::set_test_id(7932);
    let mut f64_0: f64 = 4815374002031689728.000000f64;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut i32_0: i32 = 1000i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 207i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 3i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i32_2);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 4u8;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 46u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_3: i32 = -10i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut i64_1: i64 = -81i64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i32_4: i32 = 16i32;
    let mut i32_5: i32 = 99i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_5);
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut u32_1: u32 = 999999999u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 24u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_6);
    let mut padding_1: duration::Padding = std::default::Default::default();
    let mut i32_6: i32 = 93i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_6, padding: padding_1};
    let mut padding_2: duration::Padding = std::default::Default::default();
    let mut i32_7: i32 = 325i32;
    let mut i64_3: i64 = 12i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_7, padding: padding_2};
    let mut duration_9: crate::duration::Duration = std::ops::Sub::sub(duration_8, duration_7);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_9, i32_4);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_nano(offsetdatetime_1);
    let mut option_1: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5066() {
//    rusty_monitor::set_test_id(5066);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i64_0: i64 = 60i64;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_1: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, f32_1);
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_7_ref_0: &mut crate::duration::Duration = &mut duration_7;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_3);
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_187() {
//    rusty_monitor::set_test_id(187);
    let mut i16_0: i16 = 102i16;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i16_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut f64_0: f64 = 15.494724f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f32_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut u16_0: u16 = 367u16;
    let mut i32_0: i32 = 43i32;
    let mut i64_1: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, u16_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut u8_0: u8 = 12u8;
    let mut i64_2: i64 = -52i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, u8_0);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut i64_3: i64 = -67i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_9_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_8_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_6_ref_0);
    let mut tuple_3: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_4_ref_0);
    let mut tuple_4: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_3_ref_0);
    let mut tuple_5: () = std::cmp::Eq::assert_receiver_is_total_eq(duration_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_616() {
//    rusty_monitor::set_test_id(616);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 59i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 8u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = -186i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut i64_0: i64 = -4i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut u8_3: u8 = 12u8;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, u8_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_2);
    let mut u32_2: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_306() {
//    rusty_monitor::set_test_id(306);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut padding_2: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_2_ref_0: &duration::Padding = &mut padding_2;
    let mut padding_3: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_3_ref_0: &duration::Padding = &mut padding_3;
    let mut padding_4: duration::Padding = std::default::Default::default();
    let mut padding_4_ref_0: &duration::Padding = &mut padding_4;
    let mut padding_5: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_5_ref_0: &duration::Padding = &mut padding_5;
    let mut padding_6: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_6_ref_0: &duration::Padding = &mut padding_6;
    let mut padding_7: duration::Padding = std::default::Default::default();
    let mut padding_7_ref_0: &duration::Padding = &mut padding_7;
    let mut padding_8: duration::Padding = std::default::Default::default();
    let mut padding_8_ref_0: &duration::Padding = &mut padding_8;
    let mut padding_9: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_9_ref_0: &duration::Padding = &mut padding_9;
    let mut padding_10: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_10_ref_0: &duration::Padding = &mut padding_10;
    let mut padding_11: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_11_ref_0: &duration::Padding = &mut padding_11;
    let mut padding_12: duration::Padding = std::default::Default::default();
    let mut padding_12_ref_0: &duration::Padding = &mut padding_12;
    let mut padding_13: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_13_ref_0: &duration::Padding = &mut padding_13;
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
#[timeout(30000)]fn rusty_test_674() {
//    rusty_monitor::set_test_id(674);
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i32_0: i32 = 280i32;
    let mut i64_1: i64 = -39i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut u8_0: u8 = 3u8;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u8_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut u16_0: u16 = 365u16;
    let mut i32_1: i32 = 511i32;
    let mut i64_2: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u16_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i32_2: i32 = 82i32;
    let mut f64_1: f64 = -21.422431f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_8: crate::duration::Duration = std::ops::Div::div(duration_7, i32_2);
    let mut duration_8_ref_0: &crate::duration::Duration = &mut duration_8;
    let mut duration_9: crate::duration::Duration = std::default::Default::default();
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut f64_2: f64 = 0.000000f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_10_ref_0: &crate::duration::Duration = &mut duration_10;
    let mut i64_3: i64 = 1000000000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_11_ref_0: &crate::duration::Duration = &mut duration_11;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_11_ref_0, duration_10_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(duration_9_ref_0, duration_8_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(duration_2_ref_0, duration_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7852() {
//    rusty_monitor::set_test_id(7852);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_0: i64 = 604800i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i32_0: i32 = 1721119i32;
    let mut i64_2: i64 = -123i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i64_4: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut i64_5: i64 = 253402300799i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i32_1: i32 = 184i32;
    let mut i64_6: i64 = 11i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_9);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = 314i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1142() {
//    rusty_monitor::set_test_id(1142);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut i64_0: i64 = 74i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = -1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 109i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_1: i32 = 156i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_2);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut u16_0: u16 = 366u16;
    let mut i8_9: i8 = 127i8;
    let mut i64_1: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_9);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, u16_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_3);
    let mut u8_6: u8 = crate::time::Time::minute(time_1);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5918() {
//    rusty_monitor::set_test_id(5918);
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_0: i32 = 511i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut i32_1: i32 = 2i32;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 18i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_2, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_0_ref_0);
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_1, month_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_312() {
//    rusty_monitor::set_test_id(312);
    let mut u8_0: u8 = 59u8;
    let mut i64_0: i64 = 1000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 100000000u32;
    let mut u8_1: u8 = 66u8;
    let mut u8_2: u8 = 22u8;
    let mut u8_3: u8 = 7u8;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8719() {
//    rusty_monitor::set_test_id(8719);
    let mut u32_0: u32 = 1000000u32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_0: i8 = 3i8;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_1: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_0: i32 = 178i32;
    let mut i64_2: i64 = -70i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_8);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -195i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u16_0: u16 = 367u16;
    let mut i32_1: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_10);
    let mut u16_1: u16 = 3u16;
    let mut i32_2: i32 = 246i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i32_3: i32 = 70i32;
    let mut i32_4: i32 = 6i32;
    let mut i64_4: i64 = 24i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_4);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_3);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_1_ref_0, duration_3_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_465() {
//    rusty_monitor::set_test_id(465);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = -52i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 229i32;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i32_1: i32 = 280i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_1: std::time::Instant = crate::instant::Instant::into_inner(instant_0);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_2: i32 = 2i32;
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_2, i32_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 59i8;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_3: i32 = 150i32;
    let mut i64_2: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_3, padding: padding_0};
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, i8_3);
    let mut u16_0: u16 = 41u16;
    let mut i32_4: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_6);
    let mut date_4: crate::date::Date = crate::primitive_date_time::PrimitiveDateTime::date(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3177() {
//    rusty_monitor::set_test_id(3177);
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 3u8;
    let mut i32_0: i32 = -23i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_0: i8 = 127i8;
    let mut i64_0: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i8_0);
    let mut i32_1: i32 = 93i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut u8_3: u8 = 74u8;
    let mut i32_2: i32 = 257i32;
    let mut i64_1: i64 = 12i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i32_2);
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 2i8;
    let mut i8_3: i8 = -41i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i32_3: i32 = 48i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut i32_4: i32 = 178i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u8_3);
    let mut u8_4: u8 = crate::date::Date::monday_based_week(date_2);
    let mut i64_2: i64 = crate::duration::Duration::whole_weeks(duration_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms(date_0, u8_2, u8_1, u8_0);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut i32_5: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8829() {
//    rusty_monitor::set_test_id(8829);
    let mut i32_0: i32 = 3i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut bool_0: bool = true;
    let mut i64_1: i64 = 2147483647i64;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut i64_3: i64 = 60i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_3, maximum: i64_2, value: i64_1, conditional_range: bool_0};
    let mut i32_1: i32 = 268i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i32_2: i32 = 257i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_3: i32 = 376i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_2);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_4: i32 = 240i32;
    let mut i64_4: i64 = -154i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_4);
    let mut f64_1: f64 = 4768169126130614272.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Add::add(duration_3, duration_2);
    let mut i32_5: i32 = 16i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut i8_0: i8 = 69i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::January;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_106() {
//    rusty_monitor::set_test_id(106);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 6i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 30u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 78u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 80u32;
    let mut u8_3: u8 = 53u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 53u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 41u16;
    let mut i32_0: i32 = 296i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_3: i8 = 127i8;
    let mut f64_0: f64 = 226.746297f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_3);
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    let mut u8_6: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6821() {
//    rusty_monitor::set_test_id(6821);
    let mut i32_0: i32 = 110i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i16_0: i16 = 42i16;
    let mut i64_1: i64 = 3600i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i16_0);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_2);
    let mut i8_3: i8 = 4i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -54i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 109i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 144i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_2};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut u32_2: u32 = 100000u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 30u8;
    let mut u8_8: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_9: i8 = 4i8;
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_3: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_3);
    let mut u16_0: u16 = 366u16;
    let mut i8_12: i8 = 127i8;
    let mut i64_2: i64 = 86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i8_12);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u16_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_5);
    let mut u8_9: u8 = crate::time::Time::minute(time_2);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_3);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    let mut duration_7: crate::duration::Duration = std::ops::Add::add(duration_6, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_651() {
//    rusty_monitor::set_test_id(651);
    let mut i32_0: i32 = 1721425i32;
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut duration_1_ref_0: &std::time::Duration = &mut duration_1;
    let mut i8_0: i8 = 59i8;
    let mut i64_1: i64 = 1000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_0);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = -111i32;
    let mut i64_2: i64 = 1000000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration {seconds: i64_2, nanoseconds: i32_1, padding: padding_0};
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut duration_5_ref_0: &std::time::Duration = &mut duration_5;
    let mut i64_3: i64 = 3600i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i64_4: i64 = 12i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut duration_8_ref_0: &std::time::Duration = &mut duration_8;
    let mut i64_5: i64 = 2440588i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_9_ref_0: &crate::duration::Duration = &mut duration_9;
    let mut i32_2: i32 = 48i32;
    let mut i64_6: i64 = 60i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_10, i32_2);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut duration_12_ref_0: &std::time::Duration = &mut duration_12;
    let mut u8_0: u8 = 12u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_14: crate::duration::Duration = std::ops::Mul::mul(duration_13, u8_0);
    let mut duration_14_ref_0: &crate::duration::Duration = &mut duration_14;
    let mut bool_0: bool = std::cmp::PartialEq::eq(duration_14_ref_0, duration_12_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(duration_9_ref_0, duration_8_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(duration_6_ref_0, duration_5_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(duration_3_ref_0, duration_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_640() {
//    rusty_monitor::set_test_id(640);
    let mut i64_0: i64 = 24i64;
    let mut u8_0: u8 = 10u8;
    let mut f64_0: f64 = 4815374002031689728.000000f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f64_0);
    let mut u8_1: u8 = 31u8;
    let mut u32_0: u32 = 100u32;
    let mut i64_1: i64 = 1000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u32_0);
    let mut u8_2: u8 = 6u8;
    let mut i32_0: i32 = 161i32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut duration_6: crate::duration::Duration = std::ops::Sub::sub(duration_5, duration_4);
    let mut u8_3: u8 = 52u8;
    let mut i64_2: i64 = 604800i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u8_4: u8 = 9u8;
    let mut i16_0: i16 = 0i16;
    let mut i64_3: i64 = 1i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut duration_9: crate::duration::Duration = std::ops::Mul::mul(duration_8, i16_0);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, u8_4);
    let mut duration_11: crate::duration::Duration = std::ops::Mul::mul(duration_7, u8_3);
    let mut duration_12: crate::duration::Duration = std::ops::Mul::mul(duration_6, u8_2);
    let mut duration_13: crate::duration::Duration = std::ops::Mul::mul(duration_3, u8_1);
    let mut duration_14: crate::duration::Duration = std::ops::Mul::mul(duration_1, u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4001() {
//    rusty_monitor::set_test_id(4001);
    let mut i8_0: i8 = 73i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 144i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i32_1: i32 = -153i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, f32_1);
    let mut i64_0: i64 = 253402300799i64;
    let mut i64_1: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_2);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut u16_0: u16 = crate::date::Date::ordinal(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_446() {
//    rusty_monitor::set_test_id(446);
    let mut f32_0: f32 = 116.033905f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = -61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i16_0: i16 = 57i16;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, i16_0);
    let mut f64_1: f64 = 4696837146684686336.000000f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = std::ops::Mul::mul(duration_5, f64_1);
    let mut i32_0: i32 = 25i32;
    let mut i64_1: i64 = 3600i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_0);
    let mut i16_1: i16 = 4i16;
    let mut i128_0: i128 = 1000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_10: crate::duration::Duration = std::ops::Div::div(duration_9, i16_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::abs(duration_11);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_10);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_4);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1175() {
//    rusty_monitor::set_test_id(1175);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_0: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = -8i8;
    let mut i8_5: i8 = 1i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 3u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 2147483647i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_1);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    let mut padding_1: duration::Padding = std::clone::Clone::clone(padding_0_ref_0);
    let mut month_0: month::Month = crate::month::Month::October;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_469() {
//    rusty_monitor::set_test_id(469);
    let mut u8_0: u8 = 12u8;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_0_ref_0: &mut crate::offset_date_time::OffsetDateTime = &mut offsetdatetime_0;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8819() {
//    rusty_monitor::set_test_id(8819);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut i8_0: i8 = 3i8;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_0: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 178i32;
    let mut i64_1: i64 = -70i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_6);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = -195i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 367u16;
    let mut i32_1: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_8);
    let mut u16_1: u16 = 3u16;
    let mut i32_2: i32 = 246i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i32_3: i32 = 70i32;
    let mut i32_4: i32 = 6i32;
    let mut i64_3: i64 = 24i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_3);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_2_ref_0, duration_1_ref_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_2);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8537() {
//    rusty_monitor::set_test_id(8537);
    let mut i8_0: i8 = 1i8;
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_1: i8 = 44i8;
    let mut i8_2: i8 = 1i8;
    let mut i8_3: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 0i8;
    let mut i8_6: i8 = -34i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_5, i8_4);
    let mut i8_7: i8 = 1i8;
    let mut i64_1: i64 = 60i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_7);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 23u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 201i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i128_0: i128 = -66i128;
    let mut u32_1: u32 = 76u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 6u8;
    let mut u8_5: u8 = 10u8;
    let mut i8_8: i8 = 3i8;
    let mut i128_1: i128 = 1000000i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i8_8);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_6_ref_0: &crate::duration::Duration = &mut duration_6;
    let mut i64_2: i64 = 24i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_2: i32 = 178i32;
    let mut i64_3: i64 = -70i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_9: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_10);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i128_2: i128 = 1000000000i128;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_4: i64 = -195i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u16_0: u16 = 367u16;
    let mut i32_3: i32 = 76i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_12);
    let mut u16_1: u16 = 3u16;
    let mut i32_4: i32 = 246i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut i32_5: i32 = 70i32;
    let mut i32_6: i32 = 6i32;
    let mut i64_5: i64 = 24i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_6);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_13, i32_5);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_6_ref_0, duration_5_ref_0);
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_503() {
//    rusty_monitor::set_test_id(503);
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 100u32;
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, u32_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1992() {
//    rusty_monitor::set_test_id(1992);
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 5u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = -72i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = -53i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 88u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_0: u16 = 999u16;
    let mut i32_0: i32 = 353i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 1721119i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i32_2: i32 = 398i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_3, duration_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i8_6: i8 = 28i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i8_6);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut duration_4: crate::duration::Duration = std::clone::Clone::clone(duration_3_ref_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_100() {
//    rusty_monitor::set_test_id(100);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = 33i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 81i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 65u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 233i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 30u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 11u8;
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut u16_0: u16 = 59u16;
    let mut i32_2: i32 = 172i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut i32_3: i32 = 36525i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut tuple_0: (i32, month::Month, u8) = crate::date::Date::to_calendar_date(date_4);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_60() {
//    rusty_monitor::set_test_id(60);
    let mut i16_0: i16 = 32767i16;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Sub::sub(duration_1, duration_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = 604800i64;
    let mut i64_3: i64 = 3600i64;
    let mut i64_4: i64 = 9223372036854775807i64;
    let mut str_0: &str = "nanoseconds";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut i32_0: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u16_0: u16 = 367u16;
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut i64_5: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut duration_4: crate::duration::Duration = std::ops::Div::div(duration_3, f64_0);
    let mut i32_1: i32 = 3652425i32;
    let mut i64_6: i64 = -102i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_1);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_4, u16_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_6);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    let mut duration_7: crate::duration::Duration = std::ops::Mul::mul(duration_5, i16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_511() {
//    rusty_monitor::set_test_id(511);
    let mut i32_0: i32 = 336i32;
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i128_0: i128 = 67i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i128_1: i128 = 0i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut duration_5: crate::duration::Duration = std::default::Default::default();
    let mut u32_0: u32 = 67u32;
    let mut i128_2: i128 = 1i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_7: crate::duration::Duration = std::ops::Div::div(duration_6, u32_0);
    let mut i64_2: i64 = 0i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut f64_0: f64 = -103.131875f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_10: crate::duration::Duration = std::ops::Mul::mul(duration_9, f32_0);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_1: i32 = 116i32;
    let mut i64_3: i64 = 3600i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration {seconds: i64_3, nanoseconds: i32_1, padding: padding_0};
    let mut i64_4: i64 = 83i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u16_0: u16 = 60u16;
    let mut f64_1: f64 = 4828193600913801216.000000f64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_14: crate::duration::Duration = std::ops::Div::div(duration_13, u16_0);
    let mut f64_2: f64 = std::ops::Div::div(duration_14, duration_12);
    let mut f64_3: f64 = std::ops::Div::div(duration_11, duration_10);
    let mut f64_4: f64 = std::ops::Div::div(duration_8, duration_7);
    let mut f64_5: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut f64_6: f64 = std::ops::Div::div(duration_3, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_676() {
//    rusty_monitor::set_test_id(676);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut i128_0: i128 = 1000000000i128;
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i32_0: i32 = 320i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 23i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 2i8;
    let mut i8_6: i8 = 54i8;
    let mut i8_7: i8 = 24i8;
    let mut i8_8: i8 = 2i8;
    let mut i8_9: i8 = crate::utc_offset::UtcOffset::whole_hours(utcoffset_0);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_107() {
//    rusty_monitor::set_test_id(107);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 59i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4652007308841189376.000000f64;
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f64_0);
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 79u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 36525i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut u16_0: u16 = 366u16;
    let mut f64_1: f64 = 4794699203894837248.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, u16_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u16_1: u16 = 60u16;
    let mut i32_1: i32 = 178i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut tuple_0: (i32, u16) = crate::date::Date::to_ordinal_date(date_1);
    let mut f64_2: f64 = std::ops::Div::div(duration_5, duration_4);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8126() {
//    rusty_monitor::set_test_id(8126);
    let mut i64_0: i64 = 0i64;
    let mut month_0: month::Month = crate::month::Month::September;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 109i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 156i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = 127i8;
    let mut i64_1: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_6);
    let mut u8_6: u8 = crate::time::Time::minute(time_1);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1162() {
//    rusty_monitor::set_test_id(1162);
    let mut i32_0: i32 = 1000i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_1: i32 = 100i32;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_1, padding: padding_0};
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, f32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 109i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 52u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 60u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 144i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut u32_1: u32 = 100000u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 30u8;
    let mut u8_5: u8 = 10u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 0i8;
    let mut i8_8: i8 = 60i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_3: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_2);
    let mut u16_0: u16 = 366u16;
    let mut i8_9: i8 = 127i8;
    let mut i64_1: i64 = 86i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, i8_9);
    let mut duration_5: crate::duration::Duration = std::ops::Div::div(duration_4, u16_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut u8_6: u8 = crate::time::Time::minute(time_1);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut option_0: std::option::Option<std::cmp::Ordering> = std::cmp::PartialOrd::partial_cmp(duration_5_ref_0, duration_1_ref_0);
    let mut u8_7: u8 = crate::date::Date::monday_based_week(date_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1578() {
//    rusty_monitor::set_test_id(1578);
    let mut i8_0: i8 = 20i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i64_0: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = -55i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_1, nanoseconds: i32_0, padding: padding_0};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_3: i8 = 20i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut duration_2: crate::duration::Duration = std::default::Default::default();
    let mut duration_3: crate::duration::Duration = std::ops::Neg::neg(duration_2);
    let mut i32_1: i32 = 100i32;
    let mut i32_2: i32 = 86399i32;
    let mut i64_2: i64 = 86400i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i32_1);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8021() {
//    rusty_monitor::set_test_id(8021);
    let mut i32_0: i32 = 161i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = -79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut i8_3: i8 = 3i8;
    let mut i128_0: i128 = 1000002i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_3: crate::duration::Duration = std::ops::Mul::mul(duration_2, i8_3);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i64_1: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = 178i32;
    let mut i64_2: i64 = -70i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_8);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -195i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u16_0: u16 = 367u16;
    let mut i32_2: i32 = 76i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_10);
    let mut u16_1: u16 = 3u16;
    let mut i32_3: i32 = 246i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut i32_4: i32 = 70i32;
    let mut i32_5: i32 = 6i32;
    let mut i64_4: i64 = 24i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_4);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_4_ref_0, duration_1_ref_0);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_450() {
//    rusty_monitor::set_test_id(450);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f64_0: f64 = 4741671816366391296.000000f64;
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, f64_0);
    let mut u32_0: u32 = 64u32;
    let mut f64_1: f64 = 4768169126130614272.000000f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u32_0);
    let mut f64_2: f64 = 4696837146684686336.000000f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut i64_2: i64 = 1000000000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f32_0);
    let mut i64_3: i64 = 2147483647i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_0: i32 = 511i32;
    let mut i64_4: i64 = 12i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration {seconds: i64_4, nanoseconds: i32_0, padding: padding_0};
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_10);
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_9);
    let mut i128_2: i128 = crate::duration::Duration::whole_microseconds(duration_8);
    let mut i128_3: i128 = crate::duration::Duration::whole_microseconds(duration_5);
    let mut i128_4: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    let mut i128_5: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i128_6: i128 = crate::duration::Duration::whole_microseconds(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2821() {
//    rusty_monitor::set_test_id(2821);
    let mut i8_0: i8 = 69i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = -72i8;
    let mut i8_5: i8 = 3i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_0: i32 = 2i32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_0, padding: padding_0};
    let mut i32_1: i32 = 398i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_0: u8 = 30u8;
    let mut i32_2: i32 = 212i32;
    let mut i8_6: i8 = 28i8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_2, i8_6);
    let mut duration_3_ref_0: &crate::duration::Duration = &mut duration_3;
    let mut duration_4: crate::duration::Duration = std::clone::Clone::clone(duration_3_ref_0);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_0, weekday_0);
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::July;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6248() {
//    rusty_monitor::set_test_id(6248);
    let mut u16_0: u16 = 7u16;
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -54i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 109i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 144i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 30u8;
    let mut u8_2: u8 = 10u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 156i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_6: i8 = 127i8;
    let mut i64_0: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, i8_6);
    let mut duration_3: crate::duration::Duration = std::ops::Div::div(duration_0, u16_0);
    let mut u8_3: u8 = crate::time::Time::minute(time_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_362() {
//    rusty_monitor::set_test_id(362);
    let mut u16_0: u16 = 60u16;
    let mut i8_0: i8 = 1i8;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_302() {
//    rusty_monitor::set_test_id(302);
    let mut u32_0: u32 = 10u32;
    let mut u32_1: u32 = 1000000u32;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u32_1);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8867() {
//    rusty_monitor::set_test_id(8867);
    let mut i8_0: i8 = 3i8;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, i8_0);
    let mut duration_1_ref_0: &crate::duration::Duration = &mut duration_1;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_0: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 178i32;
    let mut i64_1: i64 = -70i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i128_1: i128 = 1000000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_2: i64 = -195i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u16_0: u16 = 367u16;
    let mut i32_1: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_7);
    let mut u16_1: u16 = 3u16;
    let mut i32_2: i32 = 246i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i32_3: i32 = 70i32;
    let mut i32_4: i32 = 6i32;
    let mut i64_3: i64 = 24i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_3);
    let mut bool_0: bool = std::cmp::PartialEq::ne(duration_2_ref_0, duration_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6336() {
//    rusty_monitor::set_test_id(6336);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_0: i32 = 235i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 25i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = -8i8;
    let mut i8_8: i8 = 1i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 61u8;
    let mut u8_1: u8 = 4u8;
    let mut u8_2: u8 = 3u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 2147483647i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_1};
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut i8_9: i8 = 1i8;
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, i8_9);
    let mut duration_2: crate::duration::Duration = std::ops::Mul::mul(duration_1, f64_0);
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut u8_3: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7595() {
//    rusty_monitor::set_test_id(7595);
    let mut i32_0: i32 = -89i32;
    let mut i8_0: i8 = -27i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 999999999u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 6u8;
    let mut u8_2: u8 = 0u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 7u16;
    let mut i32_1: i32 = 511i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_0_ref_0: &crate::duration::Duration = &mut duration_0;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut i32_2: i32 = 18i32;
    let mut i64_0: i64 = 12i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration {seconds: i64_0, nanoseconds: i32_2, padding: padding_0};
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, f32_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(duration_2_ref_0, duration_0_ref_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3399() {
//    rusty_monitor::set_test_id(3399);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_0: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 60u16;
    let mut i32_0: i32 = 167i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_1: i32 = 144i32;
    let mut f32_1: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f32_1);
    let mut i32_2: i32 = -153i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i64_1: i64 = 53i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_2: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8: crate::duration::Duration = std::ops::Mul::mul(duration_7, f32_2);
    let mut i64_2: i64 = 253402300799i64;
    let mut i64_3: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_2);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut result_0: std::result::Result<crate::duration::Duration, crate::error::conversion_range::ConversionRange> = std::convert::TryFrom::try_from(duration_6);
    let mut u16_1: u16 = crate::date::Date::ordinal(date_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_4, i32_1);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut tuple_1: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_327() {
//    rusty_monitor::set_test_id(327);
    let mut f32_0: f32 = 142.388097f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 60u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 246i32;
    let mut f32_1: f32 = 39.962152f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: crate::duration::Duration = std::ops::Div::div(duration_1, i32_0);
    let mut i64_0: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_3);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut u32_1: u32 = 999999999u32;
    let mut i32_1: i32 = 342i32;
    let mut i64_1: i64 = 2147483647i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, u32_1);
    let mut i64_2: i64 = 1i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_2: u32 = 999999u32;
    let mut u8_3: u8 = 53u8;
    let mut u8_4: u8 = 24u8;
    let mut u8_5: u8 = 28u8;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut u32_3: u32 = 10000u32;
    let mut u8_6: u8 = 10u8;
    let mut u8_7: u8 = 11u8;
    let mut u8_8: u8 = 7u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_3);
    let mut u16_0: u16 = 79u16;
    let mut i32_2: i32 = 14i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_2);
    let mut tuple_1: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6532() {
//    rusty_monitor::set_test_id(6532);
    let mut u8_0: u8 = 4u8;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, u8_0);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = -94i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 30u8;
    let mut u8_3: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i32_0: i32 = 1000000000i32;
    let mut i64_1: i64 = 1000000000i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 69i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut i8_3: i8 = 4i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -54i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 109i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_1: u32 = 0u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 0u8;
    let mut u8_6: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut i32_2: i32 = 144i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_2};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_2, utcoffset_1);
    let mut u32_2: u32 = 100000u32;
    let mut u8_7: u8 = 3u8;
    let mut u8_8: u8 = 30u8;
    let mut u8_9: u8 = 10u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i8_9: i8 = 4i8;
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 60i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i32_3: i32 = 156i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_3, utcoffset_3);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_3);
    let mut u16_0: u16 = 366u16;
    let mut i8_12: i8 = 127i8;
    let mut i64_2: i64 = 86i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut duration_5: crate::duration::Duration = std::ops::Mul::mul(duration_4, i8_12);
    let mut duration_6: crate::duration::Duration = std::ops::Div::div(duration_5, u16_0);
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_5);
    let mut u8_10: u8 = crate::time::Time::minute(time_2);
    let mut u16_1: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1036() {
//    rusty_monitor::set_test_id(1036);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 10u8;
    let mut u16_0: u16 = 366u16;
    let mut i32_0: i32 = -38i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 23u8;
    let mut u8_4: u8 = 1u8;
    let mut u8_5: u8 = 93u8;
    let mut u32_2: u32 = 100u32;
    let mut duration_0: crate::duration::Duration = std::default::Default::default();
    let mut duration_1: crate::duration::Duration = std::ops::Div::div(duration_0, u32_2);
    let mut i32_1: i32 = 1000000i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u16_1: u16 = 60u16;
    let mut i32_2: i32 = 30i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_2, u8_5, u8_4, u8_3, u32_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6089() {
//    rusty_monitor::set_test_id(6089);
    let mut padding_0: duration::Padding = std::default::Default::default();
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut i32_0: i32 = 131i32;
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_1: crate::duration::Duration = std::ops::Mul::mul(duration_0, f32_0);
    let mut i64_0: i64 = 53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -126.856216f32;
    let mut f64_0: f64 = 4768169126130614272.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = std::ops::Mul::mul(duration_3, f32_1);
    let mut i64_1: i64 = 253402300799i64;
    let mut i64_2: i64 = 2440588i64;
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::offset_date_time::OffsetDateTime::to_iso_week_date(offsetdatetime_0);
    let mut padding_2: time::Padding = crate::time::Padding::Optimize;
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_mul(duration_1, i32_0);
    let mut ordering_0: std::cmp::Ordering = std::cmp::Ord::cmp(padding_1_ref_0, padding_0_ref_0);
//    panic!("From RustyUnit with love");
}
}