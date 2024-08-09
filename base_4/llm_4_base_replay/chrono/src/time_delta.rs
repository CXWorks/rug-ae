//! Temporal quantification
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::time::Duration as StdDuration;
use core::{fmt, i64};
#[cfg(any(feature = "std", test))]
use std::error::Error;
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};
/// The number of nanoseconds in a microsecond.
const NANOS_PER_MICRO: i32 = 1000;
/// The number of nanoseconds in a millisecond.
const NANOS_PER_MILLI: i32 = 1_000_000;
/// The number of nanoseconds in seconds.
const NANOS_PER_SEC: i32 = 1_000_000_000;
/// The number of microseconds per second.
const MICROS_PER_SEC: i64 = 1_000_000;
/// The number of milliseconds per second.
const MILLIS_PER_SEC: i64 = 1000;
/// The number of seconds in a minute.
const SECS_PER_MINUTE: i64 = 60;
/// The number of seconds in an hour.
const SECS_PER_HOUR: i64 = 3600;
/// The number of (non-leap) seconds in days.
const SECS_PER_DAY: i64 = 86400;
/// The number of (non-leap) seconds in a week.
const SECS_PER_WEEK: i64 = 604800;
macro_rules! try_opt {
    ($e:expr) => {
        match $e { Some(v) => v, None => return None, }
    };
}
/// ISO 8601 time duration with nanosecond precision.
///
/// This also allows for the negative duration; see individual methods for details.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, Deserialize, Serialize))]
pub struct TimeDelta {
    secs: i64,
    nanos: i32,
}
/// The minimum possible `Duration`: `i64::MIN` milliseconds.
pub(crate) const MIN: TimeDelta = TimeDelta {
    secs: i64::MIN / MILLIS_PER_SEC - 1,
    nanos: NANOS_PER_SEC + (i64::MIN % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI,
};
/// The maximum possible `Duration`: `i64::MAX` milliseconds.
pub(crate) const MAX: TimeDelta = TimeDelta {
    secs: i64::MAX / MILLIS_PER_SEC,
    nanos: (i64::MAX % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI,
};
impl TimeDelta {
    /// Makes a new `Duration` with given number of weeks.
    /// Equivalent to `Duration::seconds(weeks * 7 * 24 * 60 * 60)` with overflow checks.
    /// Panics when the duration is out of bounds.
    #[inline]
    #[must_use]
    pub fn weeks(weeks: i64) -> TimeDelta {
        let secs = weeks
            .checked_mul(SECS_PER_WEEK)
            .expect("Duration::weeks out of bounds");
        TimeDelta::seconds(secs)
    }
    /// Makes a new `Duration` with given number of days.
    /// Equivalent to `Duration::seconds(days * 24 * 60 * 60)` with overflow checks.
    /// Panics when the duration is out of bounds.
    #[inline]
    #[must_use]
    pub fn days(days: i64) -> TimeDelta {
        let secs = days.checked_mul(SECS_PER_DAY).expect("Duration::days out of bounds");
        TimeDelta::seconds(secs)
    }
    /// Makes a new `Duration` with given number of hours.
    /// Equivalent to `Duration::seconds(hours * 60 * 60)` with overflow checks.
    /// Panics when the duration is out of bounds.
    #[inline]
    #[must_use]
    pub fn hours(hours: i64) -> TimeDelta {
        let secs = hours
            .checked_mul(SECS_PER_HOUR)
            .expect("Duration::hours ouf of bounds");
        TimeDelta::seconds(secs)
    }
    /// Makes a new `Duration` with given number of minutes.
    /// Equivalent to `Duration::seconds(minutes * 60)` with overflow checks.
    /// Panics when the duration is out of bounds.
    #[inline]
    #[must_use]
    pub fn minutes(minutes: i64) -> TimeDelta {
        let secs = minutes
            .checked_mul(SECS_PER_MINUTE)
            .expect("Duration::minutes out of bounds");
        TimeDelta::seconds(secs)
    }
    /// Makes a new `Duration` with given number of seconds.
    /// Panics when the duration is more than `i64::MAX` seconds
    /// or less than `i64::MIN` seconds.
    #[inline]
    #[must_use]
    pub fn seconds(seconds: i64) -> TimeDelta {
        let d = TimeDelta {
            secs: seconds,
            nanos: 0,
        };
        if d < MIN || d > MAX {
            panic!("Duration::seconds out of bounds");
        }
        d
    }
    /// Makes a new `TimeDelta` with given number of milliseconds.
    #[inline]
    pub const fn milliseconds(milliseconds: i64) -> TimeDelta {
        let (secs, millis) = div_mod_floor_64(milliseconds, MILLIS_PER_SEC);
        let nanos = millis as i32 * NANOS_PER_MILLI;
        TimeDelta { secs, nanos }
    }
    /// Makes a new `TimeDelta` with given number of microseconds.
    #[inline]
    pub const fn microseconds(microseconds: i64) -> TimeDelta {
        let (secs, micros) = div_mod_floor_64(microseconds, MICROS_PER_SEC);
        let nanos = micros as i32 * NANOS_PER_MICRO;
        TimeDelta { secs, nanos }
    }
    /// Makes a new `TimeDelta` with given number of nanoseconds.
    #[inline]
    pub const fn nanoseconds(nanos: i64) -> TimeDelta {
        let (secs, nanos) = div_mod_floor_64(nanos, NANOS_PER_SEC as i64);
        TimeDelta {
            secs,
            nanos: nanos as i32,
        }
    }
    /// Returns the total number of whole weeks in the duration.
    #[inline]
    pub const fn num_weeks(&self) -> i64 {
        self.num_days() / 7
    }
    /// Returns the total number of whole days in the duration.
    pub const fn num_days(&self) -> i64 {
        self.num_seconds() / SECS_PER_DAY
    }
    /// Returns the total number of whole hours in the duration.
    #[inline]
    pub const fn num_hours(&self) -> i64 {
        self.num_seconds() / SECS_PER_HOUR
    }
    /// Returns the total number of whole minutes in the duration.
    #[inline]
    pub const fn num_minutes(&self) -> i64 {
        self.num_seconds() / SECS_PER_MINUTE
    }
    /// Returns the total number of whole seconds in the duration.
    pub const fn num_seconds(&self) -> i64 {
        if self.secs < 0 && self.nanos > 0 { self.secs + 1 } else { self.secs }
    }
    /// Returns the number of nanoseconds such that
    /// `nanos_mod_sec() + num_seconds() * NANOS_PER_SEC` is the total number of
    /// nanoseconds in the duration.
    const fn nanos_mod_sec(&self) -> i32 {
        if self.secs < 0 && self.nanos > 0 {
            self.nanos - NANOS_PER_SEC
        } else {
            self.nanos
        }
    }
    /// Returns the total number of whole milliseconds in the duration,
    pub const fn num_milliseconds(&self) -> i64 {
        let secs_part = self.num_seconds() * MILLIS_PER_SEC;
        let nanos_part = self.nanos_mod_sec() / NANOS_PER_MILLI;
        secs_part + nanos_part as i64
    }
    /// Returns the total number of whole microseconds in the duration,
    /// or `None` on overflow (exceeding 2^63 microseconds in either direction).
    pub const fn num_microseconds(&self) -> Option<i64> {
        let secs_part = try_opt!(self.num_seconds().checked_mul(MICROS_PER_SEC));
        let nanos_part = self.nanos_mod_sec() / NANOS_PER_MICRO;
        secs_part.checked_add(nanos_part as i64)
    }
    /// Returns the total number of whole nanoseconds in the duration,
    /// or `None` on overflow (exceeding 2^63 nanoseconds in either direction).
    pub const fn num_nanoseconds(&self) -> Option<i64> {
        let secs_part = try_opt!(self.num_seconds().checked_mul(NANOS_PER_SEC as i64));
        let nanos_part = self.nanos_mod_sec();
        secs_part.checked_add(nanos_part as i64)
    }
    /// Add two durations, returning `None` if overflow occurred.
    #[must_use]
    pub fn checked_add(&self, rhs: &TimeDelta) -> Option<TimeDelta> {
        let mut secs = try_opt!(self.secs.checked_add(rhs.secs));
        let mut nanos = self.nanos + rhs.nanos;
        if nanos >= NANOS_PER_SEC {
            nanos -= NANOS_PER_SEC;
            secs = try_opt!(secs.checked_add(1));
        }
        let d = TimeDelta { secs, nanos };
        if d < MIN || d > MAX { None } else { Some(d) }
    }
    /// Subtract two durations, returning `None` if overflow occurred.
    #[must_use]
    pub fn checked_sub(&self, rhs: &TimeDelta) -> Option<TimeDelta> {
        let mut secs = try_opt!(self.secs.checked_sub(rhs.secs));
        let mut nanos = self.nanos - rhs.nanos;
        if nanos < 0 {
            nanos += NANOS_PER_SEC;
            secs = try_opt!(secs.checked_sub(1));
        }
        let d = TimeDelta { secs, nanos };
        if d < MIN || d > MAX { None } else { Some(d) }
    }
    /// Returns the duration as an absolute (non-negative) value.
    #[inline]
    pub const fn abs(&self) -> TimeDelta {
        if self.secs < 0 && self.nanos != 0 {
            TimeDelta {
                secs: (self.secs + 1).abs(),
                nanos: NANOS_PER_SEC - self.nanos,
            }
        } else {
            TimeDelta {
                secs: self.secs.abs(),
                nanos: self.nanos,
            }
        }
    }
    /// The minimum possible `Duration`: `i64::MIN` milliseconds.
    #[inline]
    pub const fn min_value() -> TimeDelta {
        MIN
    }
    /// The maximum possible `Duration`: `i64::MAX` milliseconds.
    #[inline]
    pub const fn max_value() -> TimeDelta {
        MAX
    }
    /// A duration where the stored seconds and nanoseconds are equal to zero.
    #[inline]
    pub const fn zero() -> TimeDelta {
        TimeDelta { secs: 0, nanos: 0 }
    }
    /// Returns `true` if the duration equals `Duration::zero()`.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.secs == 0 && self.nanos == 0
    }
    /// Creates a `time::Duration` object from `std::time::Duration`
    ///
    /// This function errors when original duration is larger than the maximum
    /// value supported for this type.
    pub fn from_std(duration: StdDuration) -> Result<TimeDelta, OutOfRangeError> {
        if duration.as_secs() > MAX.secs as u64 {
            return Err(OutOfRangeError(()));
        }
        let d = TimeDelta {
            secs: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        };
        if d > MAX {
            return Err(OutOfRangeError(()));
        }
        Ok(d)
    }
    /// Creates a `std::time::Duration` object from `time::Duration`
    ///
    /// This function errors when duration is less than zero. As standard
    /// library implementation is limited to non-negative values.
    pub fn to_std(&self) -> Result<StdDuration, OutOfRangeError> {
        if self.secs < 0 {
            return Err(OutOfRangeError(()));
        }
        Ok(StdDuration::new(self.secs as u64, self.nanos as u32))
    }
}
impl Neg for TimeDelta {
    type Output = TimeDelta;
    #[inline]
    fn neg(self) -> TimeDelta {
        if self.nanos == 0 {
            TimeDelta {
                secs: -self.secs,
                nanos: 0,
            }
        } else {
            TimeDelta {
                secs: -self.secs - 1,
                nanos: NANOS_PER_SEC - self.nanos,
            }
        }
    }
}
impl Add for TimeDelta {
    type Output = TimeDelta;
    fn add(self, rhs: TimeDelta) -> TimeDelta {
        let mut secs = self.secs + rhs.secs;
        let mut nanos = self.nanos + rhs.nanos;
        if nanos >= NANOS_PER_SEC {
            nanos -= NANOS_PER_SEC;
            secs += 1;
        }
        TimeDelta { secs, nanos }
    }
}
impl Sub for TimeDelta {
    type Output = TimeDelta;
    fn sub(self, rhs: TimeDelta) -> TimeDelta {
        let mut secs = self.secs - rhs.secs;
        let mut nanos = self.nanos - rhs.nanos;
        if nanos < 0 {
            nanos += NANOS_PER_SEC;
            secs -= 1;
        }
        TimeDelta { secs, nanos }
    }
}
impl Mul<i32> for TimeDelta {
    type Output = TimeDelta;
    fn mul(self, rhs: i32) -> TimeDelta {
        let total_nanos = self.nanos as i64 * rhs as i64;
        let (extra_secs, nanos) = div_mod_floor_64(total_nanos, NANOS_PER_SEC as i64);
        let secs = self.secs * rhs as i64 + extra_secs;
        TimeDelta {
            secs,
            nanos: nanos as i32,
        }
    }
}
impl Div<i32> for TimeDelta {
    type Output = TimeDelta;
    fn div(self, rhs: i32) -> TimeDelta {
        let mut secs = self.secs / rhs as i64;
        let carry = self.secs - secs * rhs as i64;
        let extra_nanos = carry * NANOS_PER_SEC as i64 / rhs as i64;
        let mut nanos = self.nanos / rhs + extra_nanos as i32;
        if nanos >= NANOS_PER_SEC {
            nanos -= NANOS_PER_SEC;
            secs += 1;
        }
        if nanos < 0 {
            nanos += NANOS_PER_SEC;
            secs -= 1;
        }
        TimeDelta { secs, nanos }
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<'a> std::iter::Sum<&'a TimeDelta> for TimeDelta {
    fn sum<I: Iterator<Item = &'a TimeDelta>>(iter: I) -> TimeDelta {
        iter.fold(TimeDelta::zero(), |acc, x| acc + *x)
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::iter::Sum<TimeDelta> for TimeDelta {
    fn sum<I: Iterator<Item = TimeDelta>>(iter: I) -> TimeDelta {
        iter.fold(TimeDelta::zero(), |acc, x| acc + x)
    }
}
impl fmt::Display for TimeDelta {
    /// Format a duration using the [ISO 8601] format
    ///
    /// [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601#Durations
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (abs, sign) = if self.secs < 0 { (-*self, "-") } else { (*self, "") };
        let days = abs.secs / SECS_PER_DAY;
        let secs = abs.secs - days * SECS_PER_DAY;
        let hasdate = days != 0;
        let hastime = (secs != 0 || abs.nanos != 0) || !hasdate;
        write!(f, "{}P", sign)?;
        if hasdate {
            write!(f, "{}D", days)?;
        }
        if hastime {
            if abs.nanos == 0 {
                write!(f, "T{}S", secs)?;
            } else if abs.nanos % NANOS_PER_MILLI == 0 {
                write!(f, "T{}.{:03}S", secs, abs.nanos / NANOS_PER_MILLI)?;
            } else if abs.nanos % NANOS_PER_MICRO == 0 {
                write!(f, "T{}.{:06}S", secs, abs.nanos / NANOS_PER_MICRO)?;
            } else {
                write!(f, "T{}.{:09}S", secs, abs.nanos)?;
            }
        }
        Ok(())
    }
}
/// Represents error when converting `Duration` to/from a standard library
/// implementation
///
/// The `std::time::Duration` supports a range from zero to `u64::MAX`
/// *seconds*, while this module supports signed range of up to
/// `i64::MAX` of *milliseconds*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError(());
impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source duration value is out of range for the target type")
    }
}
#[cfg(any(feature = "std", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Error for OutOfRangeError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "out of range error"
    }
}
#[inline]
const fn div_mod_floor_64(this: i64, other: i64) -> (i64, i64) {
    (this.div_euclid(other), this.rem_euclid(other))
}
#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for TimeDelta {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<TimeDelta> {
        const MIN_SECS: i64 = i64::MIN / MILLIS_PER_SEC - 1;
        const MAX_SECS: i64 = i64::MAX / MILLIS_PER_SEC;
        let secs: i64 = u.int_in_range(MIN_SECS..=MAX_SECS)?;
        let nanos: i32 = u.int_in_range(0..=(NANOS_PER_SEC - 1))?;
        let duration = TimeDelta { secs, nanos };
        if duration < MIN || duration > MAX {
            Err(arbitrary::Error::IncorrectFormat)
        } else {
            Ok(duration)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{OutOfRangeError, TimeDelta, MAX, MIN};
    use std::time::Duration as StdDuration;
    use std::{i32, i64};
    #[test]
    fn test_duration() {
        assert!(TimeDelta::seconds(1) != TimeDelta::zero());
        assert_eq!(TimeDelta::seconds(1) + TimeDelta::seconds(2), TimeDelta::seconds(3));
        assert_eq!(
            TimeDelta::seconds(86399) + TimeDelta::seconds(4), TimeDelta::days(1) +
            TimeDelta::seconds(3)
        );
        assert_eq!(
            TimeDelta::days(10) - TimeDelta::seconds(1000), TimeDelta::seconds(863000)
        );
        assert_eq!(
            TimeDelta::days(10) - TimeDelta::seconds(1000000), TimeDelta::seconds(-
            136000)
        );
        assert_eq!(
            TimeDelta::days(2) + TimeDelta::seconds(86399) +
            TimeDelta::nanoseconds(1234567890), TimeDelta::days(3) +
            TimeDelta::nanoseconds(234567890)
        );
        assert_eq!(- TimeDelta::days(3), TimeDelta::days(- 3));
        assert_eq!(
            - (TimeDelta::days(3) + TimeDelta::seconds(70)), TimeDelta::days(- 4) +
            TimeDelta::seconds(86400 - 70)
        );
    }
    #[test]
    fn test_duration_num_days() {
        assert_eq!(TimeDelta::zero().num_days(), 0);
        assert_eq!(TimeDelta::days(1).num_days(), 1);
        assert_eq!(TimeDelta::days(- 1).num_days(), - 1);
        assert_eq!(TimeDelta::seconds(86399).num_days(), 0);
        assert_eq!(TimeDelta::seconds(86401).num_days(), 1);
        assert_eq!(TimeDelta::seconds(- 86399).num_days(), 0);
        assert_eq!(TimeDelta::seconds(- 86401).num_days(), - 1);
        assert_eq!(TimeDelta::days(i32::MAX as i64).num_days(), i32::MAX as i64);
        assert_eq!(TimeDelta::days(i32::MIN as i64).num_days(), i32::MIN as i64);
    }
    #[test]
    fn test_duration_num_seconds() {
        assert_eq!(TimeDelta::zero().num_seconds(), 0);
        assert_eq!(TimeDelta::seconds(1).num_seconds(), 1);
        assert_eq!(TimeDelta::seconds(- 1).num_seconds(), - 1);
        assert_eq!(TimeDelta::milliseconds(999).num_seconds(), 0);
        assert_eq!(TimeDelta::milliseconds(1001).num_seconds(), 1);
        assert_eq!(TimeDelta::milliseconds(- 999).num_seconds(), 0);
        assert_eq!(TimeDelta::milliseconds(- 1001).num_seconds(), - 1);
    }
    #[test]
    fn test_duration_num_milliseconds() {
        assert_eq!(TimeDelta::zero().num_milliseconds(), 0);
        assert_eq!(TimeDelta::milliseconds(1).num_milliseconds(), 1);
        assert_eq!(TimeDelta::milliseconds(- 1).num_milliseconds(), - 1);
        assert_eq!(TimeDelta::microseconds(999).num_milliseconds(), 0);
        assert_eq!(TimeDelta::microseconds(1001).num_milliseconds(), 1);
        assert_eq!(TimeDelta::microseconds(- 999).num_milliseconds(), 0);
        assert_eq!(TimeDelta::microseconds(- 1001).num_milliseconds(), - 1);
        assert_eq!(TimeDelta::milliseconds(i64::MAX).num_milliseconds(), i64::MAX);
        assert_eq!(TimeDelta::milliseconds(i64::MIN).num_milliseconds(), i64::MIN);
        assert_eq!(MAX.num_milliseconds(), i64::MAX);
        assert_eq!(MIN.num_milliseconds(), i64::MIN);
    }
    #[test]
    fn test_duration_num_microseconds() {
        assert_eq!(TimeDelta::zero().num_microseconds(), Some(0));
        assert_eq!(TimeDelta::microseconds(1).num_microseconds(), Some(1));
        assert_eq!(TimeDelta::microseconds(- 1).num_microseconds(), Some(- 1));
        assert_eq!(TimeDelta::nanoseconds(999).num_microseconds(), Some(0));
        assert_eq!(TimeDelta::nanoseconds(1001).num_microseconds(), Some(1));
        assert_eq!(TimeDelta::nanoseconds(- 999).num_microseconds(), Some(0));
        assert_eq!(TimeDelta::nanoseconds(- 1001).num_microseconds(), Some(- 1));
        assert_eq!(TimeDelta::microseconds(i64::MAX).num_microseconds(), Some(i64::MAX));
        assert_eq!(TimeDelta::microseconds(i64::MIN).num_microseconds(), Some(i64::MIN));
        assert_eq!(MAX.num_microseconds(), None);
        assert_eq!(MIN.num_microseconds(), None);
        const MICROS_PER_DAY: i64 = 86_400_000_000;
        assert_eq!(
            TimeDelta::days(i64::MAX / MICROS_PER_DAY).num_microseconds(), Some(i64::MAX
            / MICROS_PER_DAY * MICROS_PER_DAY)
        );
        assert_eq!(
            TimeDelta::days(i64::MIN / MICROS_PER_DAY).num_microseconds(), Some(i64::MIN
            / MICROS_PER_DAY * MICROS_PER_DAY)
        );
        assert_eq!(
            TimeDelta::days(i64::MAX / MICROS_PER_DAY + 1).num_microseconds(), None
        );
        assert_eq!(
            TimeDelta::days(i64::MIN / MICROS_PER_DAY - 1).num_microseconds(), None
        );
    }
    #[test]
    fn test_duration_num_nanoseconds() {
        assert_eq!(TimeDelta::zero().num_nanoseconds(), Some(0));
        assert_eq!(TimeDelta::nanoseconds(1).num_nanoseconds(), Some(1));
        assert_eq!(TimeDelta::nanoseconds(- 1).num_nanoseconds(), Some(- 1));
        assert_eq!(TimeDelta::nanoseconds(i64::MAX).num_nanoseconds(), Some(i64::MAX));
        assert_eq!(TimeDelta::nanoseconds(i64::MIN).num_nanoseconds(), Some(i64::MIN));
        assert_eq!(MAX.num_nanoseconds(), None);
        assert_eq!(MIN.num_nanoseconds(), None);
        const NANOS_PER_DAY: i64 = 86_400_000_000_000;
        assert_eq!(
            TimeDelta::days(i64::MAX / NANOS_PER_DAY).num_nanoseconds(), Some(i64::MAX /
            NANOS_PER_DAY * NANOS_PER_DAY)
        );
        assert_eq!(
            TimeDelta::days(i64::MIN / NANOS_PER_DAY).num_nanoseconds(), Some(i64::MIN /
            NANOS_PER_DAY * NANOS_PER_DAY)
        );
        assert_eq!(
            TimeDelta::days(i64::MAX / NANOS_PER_DAY + 1).num_nanoseconds(), None
        );
        assert_eq!(
            TimeDelta::days(i64::MIN / NANOS_PER_DAY - 1).num_nanoseconds(), None
        );
    }
    #[test]
    fn test_duration_checked_ops() {
        assert_eq!(
            TimeDelta::milliseconds(i64::MAX - 1).checked_add(&
            TimeDelta::microseconds(999)), Some(TimeDelta::milliseconds(i64::MAX - 2) +
            TimeDelta::microseconds(1999))
        );
        assert!(
            TimeDelta::milliseconds(i64::MAX).checked_add(&
            TimeDelta::microseconds(1000)).is_none()
        );
        assert_eq!(
            TimeDelta::milliseconds(i64::MIN).checked_sub(& TimeDelta::milliseconds(0)),
            Some(TimeDelta::milliseconds(i64::MIN))
        );
        assert!(
            TimeDelta::milliseconds(i64::MIN).checked_sub(& TimeDelta::milliseconds(1))
            .is_none()
        );
    }
    #[test]
    fn test_duration_abs() {
        assert_eq!(TimeDelta::milliseconds(1300).abs(), TimeDelta::milliseconds(1300));
        assert_eq!(TimeDelta::milliseconds(1000).abs(), TimeDelta::milliseconds(1000));
        assert_eq!(TimeDelta::milliseconds(300).abs(), TimeDelta::milliseconds(300));
        assert_eq!(TimeDelta::milliseconds(0).abs(), TimeDelta::milliseconds(0));
        assert_eq!(TimeDelta::milliseconds(- 300).abs(), TimeDelta::milliseconds(300));
        assert_eq!(TimeDelta::milliseconds(- 700).abs(), TimeDelta::milliseconds(700));
        assert_eq!(TimeDelta::milliseconds(- 1000).abs(), TimeDelta::milliseconds(1000));
        assert_eq!(TimeDelta::milliseconds(- 1300).abs(), TimeDelta::milliseconds(1300));
        assert_eq!(TimeDelta::milliseconds(- 1700).abs(), TimeDelta::milliseconds(1700));
    }
    #[test]
    #[allow(clippy::erasing_op)]
    fn test_duration_mul() {
        assert_eq!(TimeDelta::zero() * i32::MAX, TimeDelta::zero());
        assert_eq!(TimeDelta::zero() * i32::MIN, TimeDelta::zero());
        assert_eq!(TimeDelta::nanoseconds(1) * 0, TimeDelta::zero());
        assert_eq!(TimeDelta::nanoseconds(1) * 1, TimeDelta::nanoseconds(1));
        assert_eq!(TimeDelta::nanoseconds(1) * 1_000_000_000, TimeDelta::seconds(1));
        assert_eq!(TimeDelta::nanoseconds(1) * - 1_000_000_000, - TimeDelta::seconds(1));
        assert_eq!(- TimeDelta::nanoseconds(1) * 1_000_000_000, - TimeDelta::seconds(1));
        assert_eq!(
            TimeDelta::nanoseconds(30) * 333_333_333, TimeDelta::seconds(10) -
            TimeDelta::nanoseconds(10)
        );
        assert_eq!(
            (TimeDelta::nanoseconds(1) + TimeDelta::seconds(1) + TimeDelta::days(1)) * 3,
            TimeDelta::nanoseconds(3) + TimeDelta::seconds(3) + TimeDelta::days(3)
        );
        assert_eq!(TimeDelta::milliseconds(1500) * - 2, TimeDelta::seconds(- 3));
        assert_eq!(TimeDelta::milliseconds(- 1500) * 2, TimeDelta::seconds(- 3));
    }
    #[test]
    fn test_duration_div() {
        assert_eq!(TimeDelta::zero() / i32::MAX, TimeDelta::zero());
        assert_eq!(TimeDelta::zero() / i32::MIN, TimeDelta::zero());
        assert_eq!(
            TimeDelta::nanoseconds(123_456_789) / 1, TimeDelta::nanoseconds(123_456_789)
        );
        assert_eq!(
            TimeDelta::nanoseconds(123_456_789) / - 1, -
            TimeDelta::nanoseconds(123_456_789)
        );
        assert_eq!(
            - TimeDelta::nanoseconds(123_456_789) / - 1,
            TimeDelta::nanoseconds(123_456_789)
        );
        assert_eq!(
            - TimeDelta::nanoseconds(123_456_789) / 1, -
            TimeDelta::nanoseconds(123_456_789)
        );
        assert_eq!(TimeDelta::seconds(1) / 3, TimeDelta::nanoseconds(333_333_333));
        assert_eq!(TimeDelta::seconds(4) / 3, TimeDelta::nanoseconds(1_333_333_333));
        assert_eq!(TimeDelta::seconds(- 1) / 2, TimeDelta::milliseconds(- 500));
        assert_eq!(TimeDelta::seconds(1) / - 2, TimeDelta::milliseconds(- 500));
        assert_eq!(TimeDelta::seconds(- 1) / - 2, TimeDelta::milliseconds(500));
        assert_eq!(TimeDelta::seconds(- 4) / 3, TimeDelta::nanoseconds(- 1_333_333_333));
        assert_eq!(TimeDelta::seconds(- 4) / - 3, TimeDelta::nanoseconds(1_333_333_333));
    }
    #[test]
    fn test_duration_sum() {
        let duration_list_1 = [TimeDelta::zero(), TimeDelta::seconds(1)];
        let sum_1: TimeDelta = duration_list_1.iter().sum();
        assert_eq!(sum_1, TimeDelta::seconds(1));
        let duration_list_2 = [
            TimeDelta::zero(),
            TimeDelta::seconds(1),
            TimeDelta::seconds(6),
            TimeDelta::seconds(10),
        ];
        let sum_2: TimeDelta = duration_list_2.iter().sum();
        assert_eq!(sum_2, TimeDelta::seconds(17));
        let duration_vec = vec![
            TimeDelta::zero(), TimeDelta::seconds(1), TimeDelta::seconds(6),
            TimeDelta::seconds(10),
        ];
        let sum_3: TimeDelta = duration_vec.into_iter().sum();
        assert_eq!(sum_3, TimeDelta::seconds(17));
    }
    #[test]
    fn test_duration_fmt() {
        assert_eq!(TimeDelta::zero().to_string(), "PT0S");
        assert_eq!(TimeDelta::days(42).to_string(), "P42D");
        assert_eq!(TimeDelta::days(- 42).to_string(), "-P42D");
        assert_eq!(TimeDelta::seconds(42).to_string(), "PT42S");
        assert_eq!(TimeDelta::milliseconds(42).to_string(), "PT0.042S");
        assert_eq!(TimeDelta::microseconds(42).to_string(), "PT0.000042S");
        assert_eq!(TimeDelta::nanoseconds(42).to_string(), "PT0.000000042S");
        assert_eq!(
            (TimeDelta::days(7) + TimeDelta::milliseconds(6543)).to_string(),
            "P7DT6.543S"
        );
        assert_eq!(TimeDelta::seconds(- 86401).to_string(), "-P1DT1S");
        assert_eq!(TimeDelta::nanoseconds(- 1).to_string(), "-PT0.000000001S");
        assert_eq!(
            format!("{:30}", TimeDelta::days(1) + TimeDelta::milliseconds(2345)),
            "P1DT2.345S"
        );
    }
    #[test]
    fn test_to_std() {
        assert_eq!(TimeDelta::seconds(1).to_std(), Ok(StdDuration::new(1, 0)));
        assert_eq!(TimeDelta::seconds(86401).to_std(), Ok(StdDuration::new(86401, 0)));
        assert_eq!(
            TimeDelta::milliseconds(123).to_std(), Ok(StdDuration::new(0, 123000000))
        );
        assert_eq!(
            TimeDelta::milliseconds(123765).to_std(), Ok(StdDuration::new(123,
            765000000))
        );
        assert_eq!(TimeDelta::nanoseconds(777).to_std(), Ok(StdDuration::new(0, 777)));
        assert_eq!(MAX.to_std(), Ok(StdDuration::new(9223372036854775, 807000000)));
        assert_eq!(TimeDelta::seconds(- 1).to_std(), Err(OutOfRangeError(())));
        assert_eq!(TimeDelta::milliseconds(- 1).to_std(), Err(OutOfRangeError(())));
    }
    #[test]
    fn test_from_std() {
        assert_eq!(
            Ok(TimeDelta::seconds(1)), TimeDelta::from_std(StdDuration::new(1, 0))
        );
        assert_eq!(
            Ok(TimeDelta::seconds(86401)), TimeDelta::from_std(StdDuration::new(86401,
            0))
        );
        assert_eq!(
            Ok(TimeDelta::milliseconds(123)), TimeDelta::from_std(StdDuration::new(0,
            123000000))
        );
        assert_eq!(
            Ok(TimeDelta::milliseconds(123765)),
            TimeDelta::from_std(StdDuration::new(123, 765000000))
        );
        assert_eq!(
            Ok(TimeDelta::nanoseconds(777)), TimeDelta::from_std(StdDuration::new(0,
            777))
        );
        assert_eq!(
            Ok(MAX), TimeDelta::from_std(StdDuration::new(9223372036854775, 807000000))
        );
        assert_eq!(
            TimeDelta::from_std(StdDuration::new(9223372036854776, 0)),
            Err(OutOfRangeError(()))
        );
        assert_eq!(
            TimeDelta::from_std(StdDuration::new(9223372036854775, 807000001)),
            Err(OutOfRangeError(()))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_200_llm_16_200 {
    use crate::time_delta::OutOfRangeError;
    use std::error::Error;
    #[test]
    fn out_of_range_error_description_test() {
        let _rug_st_tests_llm_16_200_llm_16_200_rrrruuuugggg_out_of_range_error_description_test = 0;
        let error = OutOfRangeError(());
        debug_assert_eq!(error.description(), "out of range error");
        let _rug_ed_tests_llm_16_200_llm_16_200_rrrruuuugggg_out_of_range_error_description_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_201 {
    use super::*;
    use crate::*;
    use std::iter::Sum;
    #[test]
    fn test_sum_empty() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_sum_empty = 0;
        let deltas: Vec<TimeDelta> = Vec::new();
        let sum = deltas.iter().sum::<TimeDelta>();
        debug_assert_eq!(sum, TimeDelta::zero());
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_sum_empty = 0;
    }
    #[test]
    fn test_sum_single() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let single_delta = TimeDelta::seconds(rug_fuzz_0);
        let deltas = vec![single_delta];
        let sum = deltas.iter().sum::<TimeDelta>();
        debug_assert_eq!(sum, single_delta);
             }
}
}
}    }
    #[test]
    fn test_sum_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![
            TimeDelta::seconds(rug_fuzz_0), TimeDelta::seconds(3),
            TimeDelta::milliseconds(2000)
        ];
        let sum = deltas.iter().sum::<TimeDelta>();
        debug_assert_eq!(sum, TimeDelta::seconds(5 + 3 + 2));
             }
}
}
}    }
    #[test]
    fn test_sum_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![TimeDelta::seconds(rug_fuzz_0), TimeDelta::seconds(- 3)];
        let sum = deltas.iter().sum::<TimeDelta>();
        debug_assert_eq!(sum, TimeDelta::seconds(2));
             }
}
}
}    }
    #[test]
    fn test_sum_overflow() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_sum_overflow = 0;
        let large_delta = TimeDelta::seconds(i64::MAX);
        let deltas = vec![large_delta, TimeDelta::milliseconds(1)];
        let sum = deltas.iter().sum::<TimeDelta>();
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_sum_overflow = 0;
    }
    #[test]
    fn test_sum_underflow() {
        let _rug_st_tests_llm_16_201_rrrruuuugggg_test_sum_underflow = 0;
        let large_negative_delta = TimeDelta::seconds(i64::MIN);
        let deltas = vec![large_negative_delta, TimeDelta::milliseconds(- 1)];
        let sum = deltas.iter().sum::<TimeDelta>();
        let _rug_ed_tests_llm_16_201_rrrruuuugggg_test_sum_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_202 {
    use super::*;
    use crate::*;
    use std::iter::Sum;
    use time_delta::TimeDelta;
    #[test]
    fn test_sum_empty() {
        let _rug_st_tests_llm_16_202_rrrruuuugggg_test_sum_empty = 0;
        let deltas: Vec<TimeDelta> = vec![];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, TimeDelta::zero());
        let _rug_ed_tests_llm_16_202_rrrruuuugggg_test_sum_empty = 0;
    }
    #[test]
    fn test_sum_single() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let single = TimeDelta::seconds(rug_fuzz_0);
        let deltas = vec![single];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, single);
             }
}
}
}    }
    #[test]
    fn test_sum_multiple() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![
            TimeDelta::seconds(rug_fuzz_0), TimeDelta::seconds(20),
            TimeDelta::seconds(30)
        ];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, TimeDelta::seconds(60));
             }
}
}
}    }
    #[test]
    fn test_sum_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![TimeDelta::seconds(rug_fuzz_0), TimeDelta::seconds(- 20)];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, TimeDelta::seconds(- 10));
             }
}
}
}    }
    #[test]
    fn test_sum_mixed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![
            TimeDelta::seconds(rug_fuzz_0), TimeDelta::minutes(2), TimeDelta::hours(- 1)
        ];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, TimeDelta::seconds(- 3470));
             }
}
}
}    }
    #[test]
    fn test_sum_with_milliseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let deltas = vec![
            TimeDelta::milliseconds(rug_fuzz_0), TimeDelta::milliseconds(450)
        ];
        let combined = TimeDelta::sum(deltas.iter());
        debug_assert_eq!(combined, TimeDelta::seconds(1));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_203 {
    use super::*;
    use crate::*;
    #[test]
    fn test_add_positive_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0)
            + TimeDelta::milliseconds(rug_fuzz_1);
        let delta2 = TimeDelta::seconds(rug_fuzz_2) + TimeDelta::nanoseconds(rug_fuzz_3);
        let sum = delta1 + delta2;
        debug_assert_eq!(sum, TimeDelta::seconds(3) + TimeDelta::milliseconds(1000));
             }
}
}
}    }
    #[test]
    fn test_add_negative_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(-rug_fuzz_0)
            + TimeDelta::milliseconds(-rug_fuzz_1);
        let delta2 = TimeDelta::seconds(-rug_fuzz_2)
            + TimeDelta::nanoseconds(-rug_fuzz_3);
        let sum = delta1 + delta2;
        debug_assert_eq!(sum, TimeDelta::seconds(- 3) + TimeDelta::milliseconds(- 1000));
             }
}
}
}    }
    #[test]
    fn test_add_mixed_durations() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0) + TimeDelta::nanoseconds(rug_fuzz_1);
        let delta2 = TimeDelta::seconds(-rug_fuzz_2)
            + TimeDelta::milliseconds(-rug_fuzz_3);
        let sum = delta1 + delta2;
        debug_assert_eq!(sum, TimeDelta::seconds(1) + TimeDelta::nanoseconds(0));
             }
}
}
}    }
    #[test]
    fn test_add_nanos_in_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(NANOS_PER_SEC - rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let sum = delta1 + delta2;
        debug_assert_eq!(sum, TimeDelta::seconds(1));
             }
}
}
}    }
    #[test]
    fn test_add_nanos_carry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0)
            + TimeDelta::nanoseconds(NANOS_PER_SEC - rug_fuzz_1);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_2);
        let sum = delta1 + delta2;
        debug_assert_eq!(sum, TimeDelta::seconds(2) + TimeDelta::nanoseconds(1));
             }
}
}
}    }
    #[test]
    fn test_add_negative_carry() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(-rug_fuzz_0)
            + TimeDelta::nanoseconds(rug_fuzz_1);
        let delta2 = TimeDelta::nanoseconds(-rug_fuzz_2);
        let sum = delta1 + delta2;
        debug_assert_eq!(
            sum, TimeDelta::seconds(- 1) + TimeDelta::nanoseconds(NANOS_PER_SEC - 1)
        );
             }
}
}
}    }
    const NANOS_PER_SEC: i64 = 1_000_000_000;
    const NANOS_PER_MILLI: i32 = 1_000_000;
    const NANOS_PER_MICRO: i32 = 1_000;
    const SECS_PER_DAY: i64 = 86_400;
}
#[cfg(test)]
mod tests_llm_16_204 {
    use crate::TimeDelta;
    use std::ops::Div;
    #[test]
    fn div_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td.div(rug_fuzz_1);
        debug_assert_eq!(result, TimeDelta::seconds(5));
             }
}
}
}    }
    #[test]
    fn div_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(-rug_fuzz_0);
        let result = td.div(rug_fuzz_1);
        debug_assert_eq!(result, TimeDelta::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn div_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td.div(rug_fuzz_1);
        debug_assert_eq!(result, TimeDelta::seconds(10));
             }
}
}
}    }
    #[test]
    fn div_fractional() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::milliseconds(rug_fuzz_0);
        let result = td.div(rug_fuzz_1);
        debug_assert_eq!(result, TimeDelta::milliseconds(750));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn div_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        td.div(rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn div_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(rug_fuzz_0) + TimeDelta::nanoseconds(rug_fuzz_1);
        let result = td.div(rug_fuzz_2);
        debug_assert_eq!(result, TimeDelta::seconds(5) + TimeDelta::nanoseconds(2));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_205 {
    use super::*;
    use crate::*;
    #[test]
    fn multiply_by_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td * rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::seconds(0));
             }
}
}
}    }
    #[test]
    fn multiply_by_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td * rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::seconds(10));
             }
}
}
}    }
    #[test]
    fn multiply_by_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td * rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::seconds(15));
             }
}
}
}    }
    #[test]
    fn multiply_by_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(rug_fuzz_0);
        let result = td * -rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::seconds(- 20));
             }
}
}
}    }
    #[test]
    fn multiply_with_nanoseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::milliseconds(rug_fuzz_0);
        let result = td * rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::milliseconds(1500));
             }
}
}
}    }
    #[test]
    fn multiply_large_number() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(i64::MAX / rug_fuzz_0);
        let result = td * rug_fuzz_1;
        debug_assert_eq!(result, TimeDelta::seconds(i64::MAX - 1));
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn multiply_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::seconds(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
        let _result = td * rug_fuzz_2;
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_206 {
    use crate::TimeDelta;
    use std::ops::Neg;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    #[test]
    fn test_neg_zero() {
        let _rug_st_tests_llm_16_206_rrrruuuugggg_test_neg_zero = 0;
        let zero = TimeDelta::zero();
        let neg_zero = zero.neg();
        debug_assert_eq!(neg_zero, zero);
        let _rug_ed_tests_llm_16_206_rrrruuuugggg_test_neg_zero = 0;
    }
    #[test]
    fn test_neg_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let pos = TimeDelta::seconds(rug_fuzz_0);
        let neg = pos.neg();
        debug_assert_eq!(neg, TimeDelta::seconds(- 5));
             }
}
}
}    }
    #[test]
    fn test_neg_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let neg = TimeDelta::seconds(-rug_fuzz_0);
        let pos = neg.neg();
        debug_assert_eq!(pos, TimeDelta::seconds(5));
             }
}
}
}    }
    #[test]
    fn test_neg_positive_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let pos = TimeDelta {
            secs: rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        let neg = pos.neg();
        debug_assert_eq!(neg, TimeDelta { secs : - 5, nanos : 500_000_000 });
             }
}
}
}    }
    #[test]
    fn test_neg_negative_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let neg = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        let pos = neg.neg();
        debug_assert_eq!(pos, TimeDelta { secs : 3, nanos : 500_000_000 });
             }
}
}
}    }
    #[test]
    fn test_neg_positive_one_nano() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let pos = TimeDelta {
            secs: rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        let neg = pos.neg();
        debug_assert_eq!(neg, TimeDelta { secs : - 1, nanos : NANOS_PER_SEC - 1 });
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_207 {
    use super::*;
    use crate::*;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    #[test]
    fn test_sub_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(TimeDelta::seconds(rug_fuzz_2), result);
             }
}
}
}    }
    #[test]
    fn test_sub_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(TimeDelta { secs : - rug_fuzz_2, nanos : rug_fuzz_3 }, result);
             }
}
}
}    }
    #[test]
    fn test_sub_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(TimeDelta { secs : rug_fuzz_2, nanos : rug_fuzz_3 }, result);
             }
}
}
}    }
    #[test]
    fn test_sub_to_negative_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(
            TimeDelta { secs : - rug_fuzz_2, nanos : NANOS_PER_SEC - rug_fuzz_3 }, result
        );
             }
}
}
}    }
    #[test]
    fn test_sub_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(TimeDelta::seconds(rug_fuzz_2), result);
             }
}
}
}    }
    #[test]
    fn test_sub_boundary_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i64, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1 - delta2;
        debug_assert_eq!(TimeDelta { secs : rug_fuzz_2, nanos : rug_fuzz_3 }, result);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_596 {
    use crate::TimeDelta;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    #[test]
    fn test_abs_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(delta.abs(), delta);
             }
}
}
}    }
    #[test]
    fn test_abs_negative_seconds_positive_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        let expected = TimeDelta {
            secs: rug_fuzz_2,
            nanos: NANOS_PER_SEC - rug_fuzz_3,
        };
        debug_assert_eq!(delta.abs(), expected);
             }
}
}
}    }
    #[test]
    fn test_abs_negative_seconds_zero_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::seconds(-rug_fuzz_0);
        debug_assert_eq!(delta.abs(), TimeDelta::seconds(10));
             }
}
}
}    }
    #[test]
    fn test_abs_negative_seconds_negative_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: -rug_fuzz_1,
        };
        let expected = TimeDelta {
            secs: -delta.secs,
            nanos: -delta.nanos,
        };
        debug_assert_eq!(delta.abs(), expected);
             }
}
}
}    }
    #[test]
    fn test_abs_zero() {
        let _rug_st_tests_llm_16_596_rrrruuuugggg_test_abs_zero = 0;
        let delta = TimeDelta::zero();
        debug_assert_eq!(delta.abs(), TimeDelta::zero());
        let _rug_ed_tests_llm_16_596_rrrruuuugggg_test_abs_zero = 0;
    }
    #[test]
    fn test_abs_edge_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: i64::MIN,
            nanos: rug_fuzz_0,
        };
        let expected = TimeDelta {
            secs: i64::MIN.abs(),
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.abs(), expected);
             }
}
}
}    }
    #[test]
    fn test_abs_edge_case_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: i64::MIN,
            nanos: rug_fuzz_0,
        };
        let expected = TimeDelta {
            secs: (i64::MIN + rug_fuzz_1).abs(),
            nanos: NANOS_PER_SEC - rug_fuzz_2,
        };
        debug_assert_eq!(delta.abs(), expected);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_597 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_add_no_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::milliseconds(rug_fuzz_1);
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::milliseconds(6500)));
             }
}
}
}    }
    #[test]
    fn test_checked_add_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(i64::MAX);
        let delta2 = TimeDelta::seconds(rug_fuzz_0);
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(result, None);
             }
}
}
}    }
    #[test]
    fn test_checked_add_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(-rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::seconds(- 2)));
             }
}
}
}    }
    #[test]
    fn test_checked_add_edge_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(i64::MAX);
        let delta2 = TimeDelta::nanoseconds(-rug_fuzz_0);
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(
            result, Some(TimeDelta { secs : i64::MAX, nanos : (NANOS_PER_SEC - 1) as i32,
            })
        );
             }
}
}
}    }
    #[test]
    fn test_checked_add_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::milliseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::nanoseconds(1_000_500)));
             }
}
}
}    }
    #[test]
    fn test_checked_add_zero() {
        let _rug_st_tests_llm_16_597_rrrruuuugggg_test_checked_add_zero = 0;
        let delta1 = TimeDelta::zero();
        let delta2 = TimeDelta::zero();
        let result = delta1.checked_add(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::zero()));
        let _rug_ed_tests_llm_16_597_rrrruuuugggg_test_checked_add_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_598 {
    use super::*;
    use crate::*;
    #[test]
    fn test_checked_sub_non_overflowing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1.checked_sub(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::seconds(5)));
             }
}
}
}    }
    #[test]
    fn test_checked_sub_underflowing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(rug_fuzz_0);
        let delta2 = TimeDelta::seconds(rug_fuzz_1);
        let result = delta1.checked_sub(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::seconds(- 5)));
             }
}
}
}    }
    #[test]
    fn test_checked_sub_nanosecond_adjustment() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1.checked_sub(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::nanoseconds(1_000_000_000)));
             }
}
}
}    }
    #[test]
    fn test_checked_sub_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::seconds(i64::MIN);
        let delta2 = TimeDelta::seconds(-rug_fuzz_0);
        let result = delta1.checked_sub(&delta2);
        debug_assert!(result.is_none());
             }
}
}
}    }
    #[test]
    fn test_checked_sub_with_max_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::max_value();
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_0);
        let result = delta1.checked_sub(&delta2);
        debug_assert_eq!(
            result, Some(TimeDelta::max_value() - TimeDelta::nanoseconds(1))
        );
             }
}
}
}    }
    #[test]
    fn test_checked_sub_with_min_value() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::min_value();
        let delta2 = TimeDelta::nanoseconds(-rug_fuzz_0);
        let result = delta1.checked_sub(&delta2);
        debug_assert!(result.is_none());
             }
}
}
}    }
    #[test]
    fn test_checked_sub_nanoseconds_underflowing() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta1 = TimeDelta::nanoseconds(rug_fuzz_0);
        let delta2 = TimeDelta::nanoseconds(rug_fuzz_1);
        let result = delta1.checked_sub(&delta2);
        debug_assert_eq!(result, Some(TimeDelta::nanoseconds(- 1_000)));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_599 {
    use crate::TimeDelta;
    const SECS_PER_DAY: i64 = 86_400;
    #[test]
    fn test_days_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = rug_fuzz_0;
        let duration = TimeDelta::days(days);
        debug_assert_eq!(duration.num_seconds(), days * SECS_PER_DAY);
             }
}
}
}    }
    #[test]
    fn test_days_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = -rug_fuzz_0;
        let duration = TimeDelta::days(days);
        debug_assert_eq!(duration.num_seconds(), days * SECS_PER_DAY);
             }
}
}
}    }
    #[test]
    fn test_days_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::days(rug_fuzz_0);
        debug_assert!(duration.is_zero());
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::days out of bounds")]
    fn test_days_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = i64::MAX / SECS_PER_DAY + rug_fuzz_0;
        TimeDelta::days(days);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::days out of bounds")]
    fn test_days_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let days = i64::MIN / SECS_PER_DAY - rug_fuzz_0;
        TimeDelta::days(days);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_600 {
    use super::*;
    use crate::*;
    use std::time::Duration as StdDuration;
    use crate::time_delta::OutOfRangeError;
    use crate::time_delta::TimeDelta;
    const MAX_SECS: i64 = TimeDelta::max_value().num_seconds();
    #[test]
    fn test_from_std_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = StdDuration::new(MAX_SECS as u64, rug_fuzz_0);
        debug_assert!(TimeDelta::from_std(duration).is_ok());
             }
}
}
}    }
    #[test]
    fn test_from_std_with_max_nanos_within_range() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = StdDuration::new(MAX_SECS as u64, rug_fuzz_0);
        debug_assert!(TimeDelta::from_std(duration).is_ok());
             }
}
}
}    }
    #[test]
    fn test_from_std_with_seconds_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = StdDuration::new(
            (MAX_SECS as u64).saturating_add(rug_fuzz_0),
            rug_fuzz_1,
        );
        debug_assert!(TimeDelta::from_std(duration).is_err());
             }
}
}
}    }
    #[test]
    fn test_from_std_with_nanos_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = StdDuration::new(MAX_SECS as u64, rug_fuzz_0);
        debug_assert!(TimeDelta::from_std(duration).is_err());
             }
}
}
}    }
    #[test]
    fn test_from_std_with_max_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u64, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = StdDuration::new(
            (MAX_SECS as u64).saturating_add(rug_fuzz_0),
            rug_fuzz_1,
        );
        debug_assert!(TimeDelta::from_std(duration).is_err());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_601 {
    use super::*;
    use crate::*;
    #[test]
    fn hours_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(rug_fuzz_0);
        debug_assert_eq!(duration, TimeDelta::seconds(5 * 60 * 60));
             }
}
}
}    }
    #[test]
    fn hours_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(-rug_fuzz_0);
        debug_assert_eq!(duration, TimeDelta::seconds(- 5 * 60 * 60));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::hours ouf of bounds")]
    fn hours_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::hours(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::hours ouf of bounds")]
    fn hours_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::hours(i64::MIN / rug_fuzz_0 - rug_fuzz_1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_602 {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert!(TimeDelta::zero().is_zero());
        debug_assert!(! TimeDelta::seconds(rug_fuzz_0).is_zero());
        debug_assert!(! TimeDelta::milliseconds(rug_fuzz_1).is_zero());
        debug_assert!(! TimeDelta::nanoseconds(rug_fuzz_2).is_zero());
        debug_assert!(! TimeDelta::microseconds(- rug_fuzz_3).is_zero());
        debug_assert!(! (- TimeDelta::seconds(rug_fuzz_4)).is_zero());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_603 {
    use crate::TimeDelta;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_603_rrrruuuugggg_test_max_value = 0;
        let max_duration = TimeDelta::max_value();
        debug_assert_eq!(max_duration.secs, i64::MAX / 1_000);
        debug_assert_eq!(
            max_duration.nanos, (i64::MAX % 1_000) as i32 * (NANOS_PER_SEC / 1_000)
        );
        let _rug_ed_tests_llm_16_603_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_604 {
    use crate::TimeDelta;
    use std::time::Duration as StdDuration;
    use std::num::Wrapping;
    #[test]
    fn microseconds_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::microseconds(rug_fuzz_0);
        debug_assert_eq!(delta, TimeDelta::seconds(1));
        let delta = TimeDelta::microseconds(rug_fuzz_1);
        debug_assert_eq!(delta, TimeDelta::milliseconds(1500));
        let delta = TimeDelta::microseconds(-rug_fuzz_2);
        debug_assert_eq!(delta, TimeDelta::seconds(- 1));
        let delta = TimeDelta::microseconds(-rug_fuzz_3);
        debug_assert_eq!(delta, TimeDelta::milliseconds(- 1500));
        let delta = TimeDelta::microseconds(rug_fuzz_4);
        debug_assert_eq!(delta, TimeDelta::seconds(0));
             }
}
}
}    }
    #[test]
    fn microseconds_bounds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let max_micros = i64::MAX / rug_fuzz_0;
        let delta = TimeDelta::microseconds(max_micros);
        debug_assert_eq!(delta, TimeDelta::seconds(max_micros / 1_000_000));
        let min_micros = i64::MIN / rug_fuzz_1;
        let delta = TimeDelta::microseconds(min_micros);
        debug_assert_eq!(delta, TimeDelta::seconds(min_micros / 1_000_000));
             }
}
}
}    }
    #[test]
    fn microseconds_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let over_max_micros = Wrapping(i64::MAX) + Wrapping(rug_fuzz_0);
        let delta = TimeDelta::microseconds(over_max_micros.0);
        debug_assert_eq!(delta, TimeDelta::seconds(over_max_micros.0 / 1_000_000));
        let under_min_micros = Wrapping(i64::MIN) - Wrapping(rug_fuzz_1);
        let delta = TimeDelta::microseconds(under_min_micros.0);
        debug_assert_eq!(delta, TimeDelta::seconds(under_min_micros.0 / 1_000_000));
             }
}
}
}    }
    #[test]
    fn microseconds_std_duration_conversion() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(u64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let std_duration = StdDuration::from_micros(rug_fuzz_0);
        let delta = TimeDelta::from_std(std_duration).unwrap();
        let std_converted = delta.to_std().unwrap();
        debug_assert_eq!(delta, TimeDelta::seconds(1));
        debug_assert_eq!(std_converted, std_duration);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_605 {
    use super::*;
    use crate::*;
    #[test]
    fn milliseconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(TimeDelta::milliseconds(rug_fuzz_0), TimeDelta::zero());
             }
}
}
}    }
    #[test]
    fn milliseconds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            TimeDelta::milliseconds(rug_fuzz_0), TimeDelta { secs : 1, nanos :
            500_000_000 }
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            TimeDelta::milliseconds(- rug_fuzz_0), TimeDelta { secs : - 2, nanos :
            500_000_000 }
        );
             }
}
}
}    }
    #[test]
    fn milliseconds_edge_case() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        debug_assert_eq!(
            TimeDelta::milliseconds(rug_fuzz_0), TimeDelta { secs : 1, nanos : 0 }
        );
        debug_assert_eq!(
            TimeDelta::milliseconds(- rug_fuzz_1), TimeDelta { secs : - 1, nanos : 0 }
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_606 {
    use crate::TimeDelta;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_606_rrrruuuugggg_test_min_value = 0;
        let min_value = TimeDelta::min_value();
        let expected = TimeDelta::milliseconds(i64::MIN);
        debug_assert_eq!(min_value, expected);
        let _rug_ed_tests_llm_16_606_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_607 {
    use super::*;
    use crate::*;
    #[test]
    fn test_minutes_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(rug_fuzz_0);
        debug_assert_eq!(duration.num_seconds(), 15 * 60);
             }
}
}
}    }
    #[test]
    fn test_minutes_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(-rug_fuzz_0);
        debug_assert_eq!(duration.num_seconds(), - 15 * 60);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::minutes out of bounds")]
    fn test_minutes_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::minutes(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::minutes out of bounds")]
    fn test_minutes_underflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::minutes(i64::MIN / rug_fuzz_0 - rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_minutes_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(rug_fuzz_0);
        debug_assert!(duration.is_zero());
             }
}
}
}    }
    #[test]
    fn test_minutes_boundaries() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let max = TimeDelta::minutes(i64::MAX / rug_fuzz_0);
        let min = TimeDelta::minutes(i64::MIN / rug_fuzz_1);
        debug_assert_eq!(max.num_seconds(), i64::MAX / 60 * 60);
        debug_assert_eq!(min.num_seconds(), i64::MIN / 60 * 60);
             }
}
}
}    }
    #[test]
    fn test_minutes_one() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(rug_fuzz_0);
        debug_assert_eq!(duration.num_seconds(), 60);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_608 {
    use super::*;
    use crate::*;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    #[test]
    fn test_nanos_mod_sec_positive_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.nanos_mod_sec(), 123_456_789);
             }
}
}
}    }
    #[test]
    fn test_nanos_mod_sec_negative_duration_positive_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.nanos_mod_sec(), 123_456_789 - NANOS_PER_SEC);
             }
}
}
}    }
    #[test]
    fn test_nanos_mod_sec_negative_duration_zero_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.nanos_mod_sec(), 0);
             }
}
}
}    }
    #[test]
    fn test_nanos_mod_sec_zero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.nanos_mod_sec(), 0);
             }
}
}
}    }
    #[test]
    fn test_nanos_mod_sec_positive_duration_negative_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: rug_fuzz_0,
            nanos: -rug_fuzz_1,
        };
        debug_assert_eq!(delta.nanos_mod_sec(), - 123_456_789);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_609 {
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_nanoseconds_within_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(rug_fuzz_0);
        debug_assert_eq!(td.secs, 0);
        debug_assert_eq!(td.nanos, 999_999_999);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_exactly_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(rug_fuzz_0);
        debug_assert_eq!(td.secs, 1);
        debug_assert_eq!(td.nanos, 0);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_more_than_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(rug_fuzz_0);
        debug_assert_eq!(td.secs, 1);
        debug_assert_eq!(td.nanos, 1);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_negative_less_than_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(-rug_fuzz_0);
        debug_assert_eq!(td.secs, - 1);
        debug_assert_eq!(td.nanos, 1);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_negative_exactly_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(-rug_fuzz_0);
        debug_assert_eq!(td.secs, - 1);
        debug_assert_eq!(td.nanos, 0);
             }
}
}
}    }
    #[test]
    fn test_nanoseconds_negative_more_than_one_second() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let td = TimeDelta::nanoseconds(-rug_fuzz_0);
        debug_assert_eq!(td.secs, - 2);
        debug_assert_eq!(td.nanos, 999_999_999);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_610 {
    use super::*;
    use crate::*;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    const SECS_PER_MINUTE: i64 = 60;
    const SECS_PER_HOUR: i64 = SECS_PER_MINUTE * 60;
    const SECS_PER_DAY: i64 = SECS_PER_HOUR * 24;
    const SECS_PER_WEEK: i64 = SECS_PER_DAY * 7;
    const MIN: TimeDelta = TimeDelta {
        secs: i64::MIN / SECS_PER_DAY * SECS_PER_DAY,
        nanos: 0,
    };
    const MAX: TimeDelta = TimeDelta {
        secs: i64::MAX / SECS_PER_DAY * SECS_PER_DAY,
        nanos: 0,
    };
    #[test]
    fn test_num_days_with_no_days() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_days(), 0);
             }
}
}
}    }
    #[test]
    fn test_num_days_with_single_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::days(rug_fuzz_0);
        debug_assert_eq!(duration.num_days(), 1);
             }
}
}
}    }
    #[test]
    fn test_num_days_with_multiple_days() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::days(rug_fuzz_0);
        debug_assert_eq!(duration.num_days(), 10);
             }
}
}
}    }
    #[test]
    fn test_num_days_with_negative_days() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::days(-rug_fuzz_0);
        debug_assert_eq!(duration.num_days(), - 5);
             }
}
}
}    }
    #[test]
    fn test_num_days_with_partial_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(rug_fuzz_0);
        debug_assert_eq!(duration.num_days(), 1);
             }
}
}
}    }
    #[test]
    fn test_num_days_max() {
        let _rug_st_tests_llm_16_610_rrrruuuugggg_test_num_days_max = 0;
        let duration = MAX;
        debug_assert_eq!(duration.num_days(), i64::MAX / SECS_PER_DAY);
        let _rug_ed_tests_llm_16_610_rrrruuuugggg_test_num_days_max = 0;
    }
    #[test]
    fn test_num_days_min() {
        let _rug_st_tests_llm_16_610_rrrruuuugggg_test_num_days_min = 0;
        let duration = MIN;
        debug_assert_eq!(duration.num_days(), i64::MIN / SECS_PER_DAY);
        let _rug_ed_tests_llm_16_610_rrrruuuugggg_test_num_days_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_611 {
    use super::*;
    use crate::*;
    const SECS_PER_HOUR: i64 = 3600;
    #[test]
    fn num_hours_zero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::zero();
        debug_assert_eq!(rug_fuzz_0, duration.num_hours());
             }
}
}
}    }
    #[test]
    fn num_hours_pos_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(rug_fuzz_0);
        debug_assert_eq!(rug_fuzz_1, duration.num_hours());
             }
}
}
}    }
    #[test]
    fn num_hours_neg_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(-rug_fuzz_0);
        debug_assert_eq!(- rug_fuzz_1, duration.num_hours());
             }
}
}
}    }
    #[test]
    fn num_hours_part_hour() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(SECS_PER_HOUR / rug_fuzz_0);
        debug_assert_eq!(rug_fuzz_1, duration.num_hours());
             }
}
}
}    }
    #[test]
    fn num_hours_more_than_day() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hours = rug_fuzz_0;
        let duration = TimeDelta::hours(hours);
        debug_assert_eq!(hours, duration.num_hours());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_612 {
    use super::*;
    use crate::*;
    const NANOS_PER_MICRO: i32 = 1_000;
    const MICROS_PER_SEC: i64 = 1_000_000;
    #[test]
    fn test_num_microseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i64, i64, i64, i64, i64, i32, i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let one_micro = TimeDelta::microseconds(rug_fuzz_0);
        debug_assert_eq!(one_micro.num_microseconds(), Some(1));
        let one_sec = TimeDelta::seconds(rug_fuzz_1);
        debug_assert_eq!(one_sec.num_microseconds(), Some(MICROS_PER_SEC));
        let one_micro_neg = TimeDelta::microseconds(-rug_fuzz_2);
        debug_assert_eq!(one_micro_neg.num_microseconds(), Some(- 1));
        let one_sec_neg = TimeDelta::seconds(-rug_fuzz_3);
        debug_assert_eq!(one_sec_neg.num_microseconds(), Some(- MICROS_PER_SEC));
        let max_value = TimeDelta {
            secs: i64::MAX / MICROS_PER_SEC,
            nanos: (i64::MAX % MICROS_PER_SEC) as i32 * NANOS_PER_MICRO,
        };
        debug_assert_eq!(max_value.num_microseconds(), Some(i64::MAX));
        let overflow = TimeDelta {
            secs: i64::MAX / MICROS_PER_SEC + rug_fuzz_4,
            nanos: rug_fuzz_5,
        };
        debug_assert_eq!(overflow.num_microseconds(), None);
        let underflow = TimeDelta {
            secs: i64::MIN / MICROS_PER_SEC - rug_fuzz_6,
            nanos: rug_fuzz_7,
        };
        debug_assert_eq!(underflow.num_microseconds(), None);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;
    use crate::*;
    const NANOS_PER_SEC: i32 = 1_000_000_000;
    const MILLIS_PER_SEC: i64 = 1_000;
    const NANOS_PER_MILLI: i32 = 1_000_000;
    const SECS_PER_MINUTE: i64 = 60;
    const SECS_PER_HOUR: i64 = SECS_PER_MINUTE * 60;
    const SECS_PER_DAY: i64 = SECS_PER_HOUR * 24;
    const SECS_PER_WEEK: i64 = SECS_PER_DAY * 7;
    const MIN: TimeDelta = TimeDelta {
        secs: i64::MIN,
        nanos: 0,
    };
    const MAX: TimeDelta = TimeDelta {
        secs: i64::MAX,
        nanos: NANOS_PER_SEC - 1,
    };
    #[test]
    fn num_milliseconds_works() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9)) = <(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::zero();
        debug_assert_eq!(duration.num_milliseconds(), 0);
        let duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_milliseconds(), 1_000);
        let duration = TimeDelta::seconds(-rug_fuzz_1);
        debug_assert_eq!(duration.num_milliseconds(), - 1_000);
        let duration = TimeDelta::milliseconds(rug_fuzz_2);
        debug_assert_eq!(duration.num_milliseconds(), 1);
        let duration = TimeDelta::milliseconds(-rug_fuzz_3);
        debug_assert_eq!(duration.num_milliseconds(), - 1);
        let duration = TimeDelta::nanoseconds(rug_fuzz_4);
        debug_assert_eq!(duration.num_milliseconds(), 0);
        let duration = TimeDelta::microseconds(rug_fuzz_5);
        debug_assert_eq!(duration.num_milliseconds(), 0);
        let duration = TimeDelta::minutes(rug_fuzz_6);
        debug_assert_eq!(duration.num_milliseconds(), 60_000);
        let duration = TimeDelta::hours(rug_fuzz_7);
        debug_assert_eq!(duration.num_milliseconds(), 3_600_000);
        let duration = TimeDelta::days(rug_fuzz_8);
        debug_assert_eq!(duration.num_milliseconds(), 86_400_000);
        let duration = TimeDelta::weeks(rug_fuzz_9);
        debug_assert_eq!(duration.num_milliseconds(), 604_800_000);
        let duration = TimeDelta::max_value();
        debug_assert_eq!(duration.num_milliseconds(), i64::MAX);
        let duration = TimeDelta::min_value();
        debug_assert_eq!(duration.num_milliseconds(), i64::MIN);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_614 {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_minutes_positive_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), 10);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(-rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), - 10);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_positive_with_overflow() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(i64::MAX / rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), i64::MAX / 60);
             }
}
}
}    }
    #[test]
    #[should_panic]
    fn test_num_minutes_overflow_panic() {
        let _rug_st_tests_llm_16_614_rrrruuuugggg_test_num_minutes_overflow_panic = 0;
        let _ = TimeDelta::minutes(i64::MAX);
        let _rug_ed_tests_llm_16_614_rrrruuuugggg_test_num_minutes_overflow_panic = 0;
    }
    #[test]
    fn test_num_minutes_zero_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), 0);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_hour_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::hours(rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), 60);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_subseconds() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_minutes(), 0);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(rug_fuzz_0)
            + TimeDelta::nanoseconds(rug_fuzz_1);
        debug_assert_eq!(duration.num_minutes(), 1);
             }
}
}
}    }
    #[test]
    fn test_num_minutes_with_negative_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::minutes(-rug_fuzz_0)
            + TimeDelta::nanoseconds(-rug_fuzz_1);
        debug_assert_eq!(duration.num_minutes(), - 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_615 {
    use crate::TimeDelta;
    const NANOS_PER_SEC: i64 = 1_000_000_000;
    #[test]
    fn test_num_nanoseconds_positive_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_nanoseconds(), Some(NANOS_PER_SEC));
             }
}
}
}    }
    #[test]
    fn test_num_nanoseconds_negative_duration() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(-rug_fuzz_0);
        debug_assert_eq!(duration.num_nanoseconds(), Some(- NANOS_PER_SEC));
             }
}
}
}    }
    #[test]
    fn test_num_nanoseconds_subsecond() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::nanoseconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_nanoseconds(), Some(500));
             }
}
}
}    }
    #[test]
    fn test_num_nanoseconds_subsecond_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::nanoseconds(-rug_fuzz_0);
        debug_assert_eq!(duration.num_nanoseconds(), Some(- 500));
             }
}
}
}    }
    #[test]
    fn test_num_nanoseconds_overflow() {
        let _rug_st_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_overflow = 0;
        let duration = TimeDelta::seconds(i64::MAX);
        debug_assert_eq!(duration.num_nanoseconds(), None);
        let _rug_ed_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_overflow = 0;
    }
    #[test]
    fn test_num_nanoseconds_underflow() {
        let _rug_st_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_underflow = 0;
        let duration = TimeDelta::seconds(i64::MIN);
        debug_assert_eq!(duration.num_nanoseconds(), None);
        let _rug_ed_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_underflow = 0;
    }
    #[test]
    fn test_num_nanoseconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(duration.num_nanoseconds(), Some(0));
             }
}
}
}    }
    #[test]
    fn test_num_nanoseconds_max_value() {
        let _rug_st_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_max_value = 0;
        let max_value = TimeDelta::max_value();
        debug_assert!(max_value.num_nanoseconds().is_some());
        let _rug_ed_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_max_value = 0;
    }
    #[test]
    fn test_num_nanoseconds_min_value() {
        let _rug_st_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_min_value = 0;
        let min_value = TimeDelta::min_value();
        debug_assert!(min_value.num_nanoseconds().is_some());
        let _rug_ed_tests_llm_16_615_rrrruuuugggg_test_num_nanoseconds_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_616 {
    use super::*;
    use crate::*;
    #[test]
    fn num_seconds_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(delta.num_seconds(), 0);
             }
}
}
}    }
    #[test]
    fn num_seconds_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(delta.num_seconds(), 123);
             }
}
}
}    }
    #[test]
    fn num_seconds_negative_no_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta::seconds(-rug_fuzz_0);
        debug_assert_eq!(delta.num_seconds(), - 123);
             }
}
}
}    }
    #[test]
    fn num_seconds_negative_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: -rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.num_seconds(), - 122);
             }
}
}
}    }
    #[test]
    fn num_seconds_positive_with_nanos() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let delta = TimeDelta {
            secs: rug_fuzz_0,
            nanos: rug_fuzz_1,
        };
        debug_assert_eq!(delta.num_seconds(), 123);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_617 {
    use super::*;
    use crate::*;
    use crate::time_delta::{SECS_PER_DAY, NANOS_PER_SEC};
    #[test]
    fn test_num_weeks() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i64, i64, i64, i64, i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let zero_duration = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(zero_duration.num_weeks(), 0);
        let one_week_duration = TimeDelta::weeks(rug_fuzz_1);
        debug_assert_eq!(one_week_duration.num_weeks(), 1);
        let multiple_weeks_duration = TimeDelta::weeks(rug_fuzz_2);
        debug_assert_eq!(multiple_weeks_duration.num_weeks(), 5);
        let not_full_week_duration = TimeDelta::seconds(rug_fuzz_3 * SECS_PER_DAY);
        debug_assert_eq!(not_full_week_duration.num_weeks(), 1);
        let negative_duration = TimeDelta::seconds(
            -(rug_fuzz_4 * SECS_PER_DAY * rug_fuzz_5),
        );
        debug_assert_eq!(negative_duration.num_weeks(), - 1);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_618 {
    use crate::TimeDelta;
    #[test]
    fn test_seconds_normal() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let dur = TimeDelta::seconds(rug_fuzz_0);
        debug_assert_eq!(dur.secs, 42);
        debug_assert_eq!(dur.nanos, 0);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::seconds out of bounds")]
    fn test_seconds_panic_on_overflow() {
        let _rug_st_tests_llm_16_618_rrrruuuugggg_test_seconds_panic_on_overflow = 0;
        let _ = TimeDelta::seconds(i64::MAX);
        let _rug_ed_tests_llm_16_618_rrrruuuugggg_test_seconds_panic_on_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "Duration::seconds out of bounds")]
    fn test_seconds_panic_on_underflow() {
        let _rug_st_tests_llm_16_618_rrrruuuugggg_test_seconds_panic_on_underflow = 0;
        let _ = TimeDelta::seconds(i64::MIN);
        let _rug_ed_tests_llm_16_618_rrrruuuugggg_test_seconds_panic_on_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_619 {
    use super::*;
    use crate::*;
    use std::time::Duration as StdDuration;
    fn time_delta(secs: i64, nanos: i32) -> TimeDelta {
        TimeDelta { secs, nanos }
    }
    #[test]
    fn to_std_zero_duration() {
        let zero_delta = time_delta(0, 0);
        assert_eq!(zero_delta.to_std().unwrap(), StdDuration::new(0, 0));
    }
    #[test]
    fn to_std_positive_duration() {
        let positive_delta = time_delta(10, 50);
        assert_eq!(positive_delta.to_std().unwrap(), StdDuration::new(10, 50));
    }
    #[test]
    fn to_std_negative_seconds() {
        let negative_seconds_delta = time_delta(-10, 50);
        assert!(negative_seconds_delta.to_std().is_err());
    }
    #[test]
    fn to_std_negative_nanos() {
        let negative_nanos_delta = time_delta(-1, -1);
        assert!(negative_nanos_delta.to_std().is_err());
    }
    #[test]
    fn to_std_max_positive_duration() {
        let max_positive_delta = time_delta(i64::MAX, (NANOS_PER_SEC - 1) as i32);
        assert_eq!(
            max_positive_delta.to_std().unwrap(), StdDuration::new(i64::MAX as u64,
            (NANOS_PER_SEC - 1) as u32)
        );
    }
    #[test]
    fn to_std_positive_seconds_negative_nanos() {
        let negative_nanos_positive_seconds_delta = time_delta(1, -1);
        assert!(negative_nanos_positive_seconds_delta.to_std().is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_620 {
    use crate::TimeDelta;
    use std::i64;
    #[test]
    fn test_weeks_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::weeks(rug_fuzz_0);
        debug_assert_eq!(duration, TimeDelta::days(5 * 7));
             }
}
}
}    }
    #[test]
    fn test_weeks_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::weeks(-rug_fuzz_0);
        debug_assert_eq!(duration, TimeDelta::days(- 5 * 7));
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::weeks out of bounds")]
    fn test_weeks_overflow_positive() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::weeks(i64::MAX / rug_fuzz_0 + rug_fuzz_1);
             }
}
}
}    }
    #[test]
    #[should_panic(expected = "Duration::weeks out of bounds")]
    fn test_weeks_overflow_negative() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(i64, i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let _duration = TimeDelta::weeks(i64::MIN / rug_fuzz_0 - rug_fuzz_1);
             }
}
}
}    }
    #[test]
    fn test_weeks_zero() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i64) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let duration = TimeDelta::weeks(rug_fuzz_0);
        debug_assert_eq!(duration, TimeDelta::zero());
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_621 {
    use crate::time_delta::TimeDelta;
    #[test]
    fn test_zero() {
        let _rug_st_tests_llm_16_621_rrrruuuugggg_test_zero = 0;
        let zero_duration = TimeDelta::zero();
        debug_assert_eq!(zero_duration.secs, 0);
        debug_assert_eq!(zero_duration.nanos, 0);
        debug_assert_eq!(zero_duration, TimeDelta::seconds(0));
        debug_assert_eq!(zero_duration, TimeDelta::milliseconds(0));
        debug_assert_eq!(zero_duration, TimeDelta::microseconds(0));
        debug_assert_eq!(zero_duration, TimeDelta::nanoseconds(0));
        debug_assert!(zero_duration.is_zero());
        let _rug_ed_tests_llm_16_621_rrrruuuugggg_test_zero = 0;
    }
}
